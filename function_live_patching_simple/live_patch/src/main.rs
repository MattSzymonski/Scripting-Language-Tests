// REQUIREMENTS
//   Rust stable (edition 2024).  Run from the function_live_patching_simple/
//   directory:
//     cargo build -p patch_dll
//     cargo run -p live_patch -- loop
//
// DESCRIPTION
//   Game hot-reload host using the subsecond-style jump-table live-patching
//   runtime.  Calls `update(tick)` and `compute(x, y)` every 800 ms through
//   `HotFn::current`, which transparently dispatches to the latest DLL export
//   when a patch has been loaded.
//
//   Watches patch_dll/src/ for changes.  When you edit and save:
//   - Leaf functions (compute) ? compiled individually via `rustc` (~160 ms)
//   - Functions with internal dependencies (update) ? `cargo build` reload
//     (~300 ms without bloat, ~900 ms with 5 000-function bloat module)
//
// USAGE
//   cargo run -p live_patch -- loop        # hot-reload loop
//   cargo run -p live_patch                 # one-shot static demo
//
// --- SCRIPT ---

use live_patch::{self as subsecond, HotFn, JumpTable};
use std::collections::HashMap;
use std::time::Instant;

mod bloat_gen;

// ---------------------------------------------------------------------------
// Stub game functions -- placeholders that exist in the host binary.
// At runtime, HotFn::current redirects to the DLL exports via the jump table.
// These are never actually called when a DLL patch is loaded.
// ---------------------------------------------------------------------------

fn update_stub(_tick: i32) {
    println!("[stub] update called -- patch_dll.dll not loaded?");
}

fn compute_stub(_x: i32, _y: i32) -> i32 {
    println!("[stub] compute called -- patch_dll.dll not loaded?");
    0
}

// ---------------------------------------------------------------------------
// main -- CLI dispatch
// ---------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "loop" {
        run_hot_reload_loop();
    } else {
        println!("=== Live Patch Game Host ===\n");
        println!("  Run with 'cargo run -p live_patch -- loop' for hot-reload mode.\n");

        demo_static_patch();
        demo_bloat_hotpatch();
        demo_dll_hotpatch();

        println!("\n=== Demos complete ===");
    }
}

// ---------------------------------------------------------------------------
// Live-reload loop
// ---------------------------------------------------------------------------

fn run_hot_reload_loop() {
    let exe_dir = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let workspace_root = exe_dir.parent().unwrap().parent().unwrap();
    let patch_src = workspace_root.join("patch_dll").join("src").join("lib.rs");

    let symbol_map: &[(&str, u64)] = &[
        ("update", update_stub as *const () as usize as u64),
        ("compute", compute_stub as *const () as usize as u64),
    ];

    // Cold build + initial load
    println!("  Building patch_dll.dll (cold)...");
    let build_start = Instant::now();
    let build_ok = std::process::Command::new("cargo")
        .args(["build", "-p", "patch_dll", "--target-dir", "target_reload"])
        .current_dir(&workspace_root)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !build_ok {
        println!("  ? cargo build failed -- check patch_dll/src/lib.rs for errors.");
        return;
    }
    println!("  Cold build: {:?}", build_start.elapsed());

    let base_dll = workspace_root
        .join("target_reload")
        .join("debug")
        .join("patch_dll.dll");
    print!("  Loading base DLL... ");
    match unsafe { subsecond::reload_patch_dll(&base_dll, symbol_map) } {
        Ok(n) => println!("{n} functions linked.  Base DLL stays loaded."),
        Err(e) => {
            println!("failed: {e}");
            return;
        }
    }

    // Watch state
    let mut last_modified = file_modified_time(&patch_src);
    let mut cached_funcs: Vec<(String, String, String, String)> = Vec::new();
    let mut iteration: u64 = 0;

    println!();
    println!("  +--------------------------------------------------------------+");
    println!("  |   GAME LIVE-PATCH LOOP -- edit patch_dll/src/, save, watch   |");
    println!("  |                                                              |");
    println!("  |   Leaf functions (compute) -> rustc single-fn DLL  (~0.16s)  |");
    println!("  |   Dep functions  (update)  -> cargo build reload   (~0.3s)   |");
    println!("  |                                                              |");
    println!("  |   Base DLL stays loaded.  Only changed pointers swap.        |");
    println!("  |   Press Ctrl+C to exit.                                      |");
    println!("  +--------------------------------------------------------------+");
    println!();

    loop {
        iteration += 1;
        game_tick(iteration);
        std::thread::sleep(std::time::Duration::from_millis(800));

        let current_modified = file_modified_time(&patch_src);
        if current_modified != last_modified {
            last_modified = current_modified;
            println!("\n  -- patch_dll source changed (iter {iteration}) --");

            let source = match std::fs::read_to_string(&patch_src) {
                Ok(s) => s,
                Err(e) => {
                    println!("  ? Cannot read source: {e}");
                    continue;
                }
            };

            let funcs = extract_function_bodies(&source);

            let total_start = Instant::now();
            let mut patched = 0usize;
            let mut skipped = 0usize;
            let mut full_rebuild = false;

            for (name, params, body, ret_type) in &funcs {
                let (orig_addr, is_leaf): (u64, bool) = match name.as_str() {
                    "compute" => (compute_stub as *const () as usize as u64, true),
                    "update" => (update_stub as *const () as usize as u64, false),
                    _ => continue,
                };

                let changed = cached_funcs
                    .iter()
                    .find(|(n, _, b, _)| n == name && b == body)
                    .is_none();

                if !changed {
                    skipped += 1;
                    continue;
                }

                if is_leaf {
                    print!("    {name} (leaf, per-fn rustc)... ");
                    let func_start = Instant::now();
                    match unsafe {
                        subsecond::compile_and_patch(name, params, body, ret_type, orig_addr)
                    } {
                        Ok(_) => {
                            println!("? patched in {:?}", func_start.elapsed());
                            patched += 1;
                        }
                        Err(e) => println!("? {e}"),
                    }
                } else {
                    full_rebuild = true;
                }
            }

            cached_funcs = funcs;

            if full_rebuild {
                println!("    update has internal deps -> full cargo build...");

                // Unload the current DLL so cargo can overwrite the file.
                // Windows locks loaded DLLs; FreeLibrary releases the lock.
                unsafe { subsecond::unload_patch_dll() };

                let build_start = Instant::now();
                let build_ok = std::process::Command::new("cargo")
                    .args(["build", "-p", "patch_dll", "--target-dir", "target_reload"])
                    .current_dir(&workspace_root)
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false);

                if build_ok {
                    print!("    Reloading patch_dll.dll... ");
                    match unsafe { subsecond::reload_patch_dll(&base_dll, symbol_map) } {
                        Ok(n) => {
                            println!("{n} symbols re-linked in {:?}", build_start.elapsed());
                            patched += 1;
                        }
                        Err(e) => println!("failed: {e}"),
                    }
                } else {
                    println!("    ? cargo build failed.");
                }
            }

            println!(
                "  -- {patched} changed, {skipped} unchanged -- total {:?} --\n",
                total_start.elapsed()
            );
        }
    }
}

fn game_tick(iteration: u64) {
    let tick_val = iteration as i32;

    let update_ptr: fn(i32) = update_stub as fn(i32);
    let mut hot_update = HotFn::current(update_ptr);
    hot_update.call((tick_val,));

    let compute_ptr: fn(i32, i32) -> i32 = compute_stub;
    let mut hot_compute = HotFn::current(compute_ptr);
    let result = hot_compute.call((tick_val, 3));
    println!("    compute({tick_val}, 3) = {result}");
}

// ---------------------------------------------------------------------------
// Static demos
// ---------------------------------------------------------------------------

fn demo_static_patch() {
    println!("--- Demo: In-process jump-table patch ---");

    let update_ptr: fn(i32) = update_stub as fn(i32);
    let mut hot_update = HotFn::current(update_ptr);

    println!("  Before patch:");
    hot_update.call((42,));

    let mut map = HashMap::new();
    map.insert(
        update_stub as *const () as usize as u64,
        bloat_gen::bloat_42 as *const () as usize as u64,
    );
    unsafe { subsecond::apply_patch(JumpTable { map }).unwrap() };

    println!("  After patch (redirected to bloat_42):");
    hot_update.call((42,));

    println!();
}

fn demo_bloat_hotpatch() {
    println!("--- Demo: Bloat-file hot-patching (5 000 functions) ---");

    let function_pointer: fn() -> i32 =
        unsafe { std::mem::transmute::<extern "C" fn() -> i32, fn() -> i32>(bloat_gen::bloat_42) };
    let mut hot_bloat = HotFn::current(function_pointer);

    let before = hot_bloat.call(());
    println!("  bloat_42 before patch: {before}");

    let start = Instant::now();
    let mut map = HashMap::new();
    map.insert(
        bloat_gen::bloat_42 as *const () as usize as u64,
        bloat_42_v2 as *const () as usize as u64,
    );
    unsafe { subsecond::apply_patch(JumpTable { map }).unwrap() };

    let after = hot_bloat.call(());
    println!(
        "  bloat_42 after  patch: {after}  (patched in {:?})",
        start.elapsed()
    );

    let direct_4999 = bloat_gen::bloat_4999();
    println!("  bloat_4999 (untouched): {direct_4999}");

    println!();
}

fn bloat_42_v2() -> i32 {
    9999
}

fn demo_dll_hotpatch() {
    println!("--- Demo: DLL-based hot-patching ---");

    let dll_path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("patch_dll.dll");

    if !dll_path.exists() {
        println!("  patch_dll.dll not found.  Build with: cargo build -p patch_dll");
        println!("  Then copy: cp target/debug/patch_dll.dll target/debug/");
        println!("  (Skipping.)\n");
        return;
    }

    let symbol_map: &[(&str, u64)] = &[
        ("update", update_stub as *const () as usize as u64),
        ("compute", compute_stub as *const () as usize as u64),
    ];

    let start = Instant::now();
    match unsafe { subsecond::load_patch_dll(&dll_path, symbol_map) } {
        Ok(table) => {
            println!(
                "  DLL loaded, {} symbols resolved in {:?}",
                table.map.len(),
                start.elapsed()
            );
            unsafe { subsecond::apply_patch(table).unwrap() };

            let update_ptr: fn(i32) = update_stub as fn(i32);
            let mut hot_update = HotFn::current(update_ptr);
            println!("  Calling update(99) through jump table:");
            hot_update.call((99,));

            let compute_ptr: fn(i32, i32) -> i32 = compute_stub;
            let mut hot_compute = HotFn::current(compute_ptr);
            let result = hot_compute.call((6, 7));
            println!("  compute(6, 7) = {result}  (expect 6*7*3 = 126)");
        }
        Err(e) => println!("  Failed: {e}"),
    }

    println!();
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn file_modified_time(path: &std::path::Path) -> Option<std::time::SystemTime> {
    std::fs::metadata(path).ok().and_then(|m| m.modified().ok())
}

fn extract_function_bodies(source: &str) -> Vec<(String, String, String, String)> {
    let mut results = Vec::new();
    let mut rest = source;

    while let Some(start) = rest.find("pub extern \"C\" fn ") {
        rest = &rest[start + 18..];

        let name_end = rest.find('(').unwrap_or(rest.len());
        let name = rest[..name_end].trim().to_string();
        rest = &rest[name_end + 1..];

        let mut paren_depth = 1u32;
        let mut params_end = 0usize;
        for (i, ch) in rest.char_indices() {
            match ch {
                '(' => paren_depth += 1,
                ')' => {
                    paren_depth -= 1;
                    if paren_depth == 0 {
                        params_end = i;
                        break;
                    }
                }
                _ => {}
            }
        }
        let params = rest[..params_end].trim().to_string();
        rest = &rest[params_end + 1..];

        let body_start = rest.find('{').unwrap_or(rest.len());
        let ret_section = &rest[..body_start];
        let ret_type = if let Some(arrow) = ret_section.find("->") {
            ret_section[arrow + 2..]
                .trim()
                .split(|c: char| c.is_whitespace() || c == '{')
                .next()
                .unwrap_or("")
                .to_string()
        } else {
            String::new()
        };

        rest = &rest[body_start + 1..];
        let mut depth = 1u32;
        let mut body_end = 0usize;
        for (i, ch) in rest.char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        body_end = i;
                        break;
                    }
                }
                _ => {}
            }
        }
        let body = rest[..body_end].trim().to_string();
        rest = &rest[body_end + 1..];

        results.push((name, params, body, ret_type));
    }

    results
}
