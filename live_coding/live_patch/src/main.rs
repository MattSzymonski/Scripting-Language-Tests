// REQUIREMENTS
//   Rust stable (edition 2024).  Windows only.  Run from the live_coding/
//   directory:
//     cargo build
//     cargo run -p live_patch -- loop
//
// DESCRIPTION
//   Live Coding host — a Live++ / UE-style hot reload loop.  The base DLL
//   (patch_dll.dll) is loaded ONCE and its exported `update`/`compute` are
//   called with PLAIN direct function pointers — no wrapper, no jump table.
//   When you edit a function and save:
//
//     - Leaf functions (compute) are recompiled to a tiny DLL, then their
//       PROLOGUE in the loaded base DLL is patched with a `jmp rel32` to the
//       new code.  Existing callers are redirected in place (Live Coding!).
//     - Functions with internal dependencies (update) trigger a full cargo
//       rebuild + reload, then leaf prologues are re-applied.
//     - A brand-new function can be compiled, loaded, and REGISTERED by name
//       at runtime (the "add a new function live" capability).
//
// USAGE
//   cargo run -p live_patch -- loop        # hot-reload loop
//   cargo run -p live_patch                 # static demos
//
// --- SCRIPT ---

use live_patch as live_coding;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

// ---------------------------------------------------------------------------
// Direct function pointers to the loaded base DLL — no indirection.
// ---------------------------------------------------------------------------

type UpdateFn = unsafe extern "C" fn(tick: i32);
type ComputeFn = unsafe extern "C" fn(x: i32, y: i32) -> i32;
type ExtraFn = unsafe extern "C" fn(tick: i32) -> i32;

// ---------------------------------------------------------------------------
// main — CLI dispatch
// ---------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "loop" {
        run_hot_reload_loop();
    } else {
        println!("=== Live Coding (Live++-style) ===\n");
        println!("  Run with 'cargo run -p live_patch -- loop' for hot-reload mode.\n");

        demo_prologue_patch();
        demo_new_function();

        println!("\n=== Demos complete ===");
    }
}

// ---------------------------------------------------------------------------
// Demo 1 — prologue patching an existing function in place
// ---------------------------------------------------------------------------

fn demo_prologue_patch() {
    println!("--- Demo: prologue-patch an existing function in place ---");

    // Compile a replacement for a "compute" that returns 9999 instead of the
    // normal formula, load it, and patch the ORIGINAL function's prologue.
    let original_address: u64 = demo_compute_original as *const () as usize as u64;
    let params = "x: i32, y: i32";
    let body = "9999";
    let return_type = "i32";

    let start = Instant::now();
    let result = unsafe {
        live_coding::compile_and_patch_prologue(
            "demo_compute",
            params,
            body,
            return_type,
            original_address,
        )
    };
    match result {
        Ok(_) => {
            println!(
                "  Patched demo_compute prologue in {:?}.  Original address now jumps to new code.",
                start.elapsed()
            );
            let before = demo_compute_original(1, 2);
            // NOTE: after prologue patching, even the ORIGINAL pointer returns 9999!
            println!(
                "  Calling the ORIGINAL pointer: demo_compute(1,2) = {before}  (should be 9999!)"
            );
        }
        Err(e) => println!("  Failed: {e}"),
    }

    println!();
}

/// A stand-in "existing" function that will be prologue-patched.
fn demo_compute_original(_x: i32, _y: i32) -> i32 {
    42
}

// ---------------------------------------------------------------------------
// Demo 2 — compile + register a NEW function at runtime
// ---------------------------------------------------------------------------

fn demo_new_function() {
    println!("--- Demo: add a NEW function at runtime ---");

    let start = Instant::now();
    let address = unsafe {
        live_coding::compile_and_register_new("extra_new", "tick: i32", "tick * 100 + 7", "i32")
    };
    match address {
        Ok(addr) => {
            println!(
                "  Registered new function 'extra_new' at {addr:#x} in {:?}",
                start.elapsed()
            );
            // Resolve by name and call it — this function did not exist at startup.
            if let Some(resolved) = live_coding::resolve_symbol("extra_new") {
                let extra_fn: ExtraFn = unsafe { std::mem::transmute(resolved) };
                let value = unsafe { extra_fn(3) };
                println!("  Called registered 'extra_new'(3) = {value}  (expected 307)");
            } else {
                println!("  Could not resolve 'extra_new' by name");
            }
        }
        Err(e) => println!("  Failed: {e}"),
    }

    println!();
}

// ---------------------------------------------------------------------------
// Build helpers — capture compiler output so errors can be shown and handled.
// ---------------------------------------------------------------------------

/// Run `cargo build -p patch_dll --target-dir <dir>` and capture its output.
///
/// Returns `Ok(())` on success, or `Err` with the captured stderr (which
/// contains the rustc diagnostics) when the build fails.
fn build_patch_dll(workspace_root: &std::path::Path) -> Result<(), String> {
    let output = std::process::Command::new("cargo")
        .args(["build", "-p", "patch_dll", "--target-dir", "target_reload"])
        .current_dir(workspace_root)
        .output()
        .map_err(|e| format!("failed to run cargo build: {e}"))?;

    if output.status.success() {
        Ok(())
    } else {
        // stderr holds the rustc diagnostics; strip trailing whitespace.
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        Err(format!("{stdout}{stderr}").trim().to_string())
    }
}

/// Reload the base DLL from `base_dll` and return (handle, update_addr,
/// compute_addr).  Used both after a successful rebuild and to recover the
/// previous DLL when a rebuild fails (the old file is never overwritten by a
/// failed build, so it is still valid on disk).
unsafe fn load_base_dll(
    base_dll: &std::path::Path,
) -> Result<(*mut std::ffi::c_void, u64, u64), String> {
    let handle = unsafe { live_coding::load_library(base_dll) }?;
    let update_address = unsafe { live_coding::resolve_export(handle, "update") }?;
    let compute_address = unsafe { live_coding::resolve_export(handle, "compute") }?;
    Ok((handle, update_address, compute_address))
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

    // 1. Cold build + initial load of the BASE DLL.
    println!("  Building patch_dll.dll (cold)...");
    let build_start = Instant::now();
    match build_patch_dll(&workspace_root) {
        Ok(()) => println!("  Cold build: {:?}", build_start.elapsed()),
        Err(errors) => {
            println!("  ? cold build failed.  Compiler output:\n{errors}");
            return;
        }
    }

    let base_dll = workspace_root
        .join("target_reload")
        .join("debug")
        .join("patch_dll.dll");

    // Load the base DLL and resolve the exported entry points to PLAIN
    // function pointers.  There is no wrapper and no jump table: after a
    // prologue patch these same pointers transparently call the new code.
    // `base_handle` is kept mutable so a full reload can re-capture the
    // freshly loaded DLL's handle for the next unload.
    print!("  Loading base DLL... ");
    let mut base_handle = match unsafe { live_coding::load_library(&base_dll) } {
        Ok(handle) => handle,
        Err(e) => {
            println!("failed: {e}");
            return;
        }
    };
    let update_address = match unsafe { live_coding::resolve_export(base_handle, "update") } {
        Ok(addr) => addr,
        Err(e) => {
            println!("failed to resolve 'update': {e}");
            return;
        }
    };
    let compute_address = match unsafe { live_coding::resolve_export(base_handle, "compute") } {
        Ok(addr) => addr,
        Err(e) => {
            println!("failed to resolve 'compute': {e}");
            return;
        }
    };
    println!("OK (update @ {update_address:#x}, compute @ {compute_address:#x})");

    // Direct callable pointers — the ONLY way we call the game functions.
    // `compute_address` is kept mutable because a full rebuild reloads the
    // base DLL and re-resolves the prologue-patch target; `update_address` is
    // only used once to build the callable pointer.
    let mut compute_address = compute_address;
    let mut update_fn: UpdateFn = unsafe { std::mem::transmute(update_address) };
    let mut compute_fn: ComputeFn = unsafe { std::mem::transmute(compute_address) };

    // 2. Watch state — snapshot EVERY .rs file under patch_dll/src/ so any
    //    change (structs, helpers, test.rs, new modules, ...) is detected,
    //    not just edits to extern function bodies in lib.rs.
    let patch_src_root = workspace_root.join("patch_dll").join("src");
    let mut source_snapshots: HashMap<PathBuf, Vec<u8>> = HashMap::new();
    for source_file in collect_source_files(&patch_src_root) {
        let content = std::fs::read(&source_file).unwrap_or_default();
        source_snapshots.insert(source_file, content);
    }
    // lib.rs with all extern bodies stripped — used to tell "only a function
    // body changed" (fast prologue-patch) from "structs/helpers/other files
    // changed" (full rebuild).
    let mut stripped_lib_rs =
        strip_extern_bodies(&std::fs::read_to_string(&patch_src).unwrap_or_default());

    // Cache of previous function bodies: (name, params, body, ret_type).
    let mut cached_funcs: Vec<(String, String, String, String)> = Vec::new();
    let mut iteration: u64 = 0;

    println!();
    println!("  +--------------------------------------------------------------+");
    println!("  |   LIVE CODING LOOP -- edit any .rs under patch_dll/src, save |");
    println!("  |                                                              |");
    println!("  |   Leaf functions (compute) -> prologue-patched in place      |");
    println!("  |   Dep functions  (update)  -> cargo build + reload           |");
    println!("  |   NEW functions            -> compiled + registered by name  |");
    println!("  |   Structs / helpers /      -> full cargo build + reload      |");
    println!("  |   test.rs / new modules                                       |");
    println!("  |                                                              |");
    println!("  |   Base DLL stays loaded.  Direct calls are redirected.       |");
    println!("  |   Press Ctrl+C to exit.                                      |");
    println!("  +--------------------------------------------------------------+");
    println!();

    loop {
        iteration += 1;
        game_tick(iteration, update_fn, compute_fn);
        std::thread::sleep(std::time::Duration::from_millis(800));

        // Diff the on-disk source against the last snapshot — ANY file change.
        let source_files = collect_source_files(&patch_src_root);
        let mut changed_files: Vec<PathBuf> = Vec::new();
        for source_file in &source_files {
            let current = std::fs::read(source_file).unwrap_or_default();
            let previous = source_snapshots.get(source_file);
            if previous != Some(&current) {
                changed_files.push(source_file.clone());
            }
            source_snapshots.insert(source_file.clone(), current);
        }

        if changed_files.is_empty() {
            continue;
        }

        println!("\n  -- source changed (iter {iteration}) --");
        for source_file in &changed_files {
            let relative = source_file
                .strip_prefix(&workspace_root)
                .unwrap_or(source_file);
            println!("      {}", relative.display());
        }

        let source = match std::fs::read_to_string(&patch_src) {
            Ok(s) => s,
            Err(e) => {
                println!("  ? Cannot read {patch_src:?}: {e}");
                continue;
            }
        };

        let funcs = extract_function_bodies(&source);

        // A full rebuild is needed when ANYTHING outside the extern function
        // bodies changed — structs, internal helpers, attributes, `mod` decls
        // — or when another file (test.rs, a new module) changed.  Compare
        // lib.rs with its extern bodies stripped: differing stripped text
        // means non-body content changed.
        let new_stripped_lib_rs = strip_extern_bodies(&source);
        let lib_rs_changed = changed_files
            .iter()
            .any(|source_file| *source_file == patch_src);
        let mut full_rebuild = !lib_rs_changed || new_stripped_lib_rs != stripped_lib_rs;
        stripped_lib_rs = new_stripped_lib_rs;

        // If `update` (a function with internal deps) changed, a full rebuild
        // is needed too — detect it up front so leaf patches are skipped.
        if funcs.iter().any(|(name, _, body, _)| {
            name == "update"
                && cached_funcs
                    .iter()
                    .find(|(cached_name, _, cached_body, _)| {
                        cached_name == name && cached_body == body
                    })
                    .is_none()
        }) {
            full_rebuild = true;
        }

        let total_start = Instant::now();
        let mut patched = 0usize;
        let mut skipped = 0usize;
        let mut absorbed_by_rebuild = 0usize;
        // Functions whose changes were applied successfully this round.
        // Only these get committed to the cache (so failures retry later).
        let mut applied_funcs: Vec<(String, String, String, String)> = Vec::new();

        for (name, params, body, ret_type) in &funcs {
            // Check if this function's body actually changed.
            let changed = cached_funcs
                .iter()
                .find(|(n, _, b, _)| n == name && b == body)
                .is_none();

            if !changed {
                skipped += 1;
                continue;
            }

            if full_rebuild {
                // The full rebuild recompiles everything from source, so any
                // per-function patching here would be discarded — skip it.
                absorbed_by_rebuild += 1;
                continue;
            }

            match name.as_str() {
                // Leaf function — prologue-patch in place (Live Coding!).
                "compute" => {
                    print!("    {name} (leaf, prologue-patch)... ");
                    let func_start = Instant::now();
                    let result = unsafe {
                        live_coding::compile_and_patch_prologue(
                            name,
                            params,
                            body,
                            ret_type,
                            compute_address,
                        )
                    };
                    match result {
                        Ok(_) => {
                            println!("? patched in {:?}", func_start.elapsed());
                            patched += 1;
                            applied_funcs.push((
                                name.clone(),
                                params.clone(),
                                body.clone(),
                                ret_type.clone(),
                            ));
                        }
                        Err(e) => println!("? {e}"),
                    }
                }
                // Function with internal deps — full cargo rebuild + reload.
                "update" => {
                    full_rebuild = true;
                }
                // Any OTHER name is treated as a NEW function: compile,
                // load, and register it by name at runtime.
                other => {
                    print!("    {other} (NEW, register by name)... ");
                    let func_start = Instant::now();
                    let result = unsafe {
                        live_coding::compile_and_register_new(other, params, body, ret_type)
                    };
                    match result {
                        Ok(addr) => {
                            println!("? registered @ {addr:#x} in {:?}", func_start.elapsed());
                            patched += 1;
                            applied_funcs.push((
                                name.clone(),
                                params.clone(),
                                body.clone(),
                                ret_type.clone(),
                            ));
                        }
                        Err(e) => println!("? {e}"),
                    }
                }
            }
        }

        // Merge the newly applied changes into the cache, keeping entries
        // for functions that were unchanged this round and replacing the
        // entries for functions whose changes were applied successfully.
        // Failed changes are NOT committed, so the next save retries them.
        for applied in &applied_funcs {
            upsert_cached_func(&mut cached_funcs, applied.clone());
        }

        // A full DLL reload is needed for non-extern changes or when a
        // dependent function (update) changed.
        if full_rebuild {
            println!("    full cargo build + reload...");

            // Unload the BASE DLL (by its captured handle) BEFORE running
            // cargo, so the linker can overwrite the file.  Note: we must
            // use the base handle explicitly — the runtime's global handle
            // may have been overwritten by intermediate patch-DLL loads.
            unsafe { live_coding::unload_library(base_handle) };

            let build_start = Instant::now();
            let build_result = build_patch_dll(&workspace_root);

            match build_result {
                Ok(()) => {
                    // Load the fresh DLL and re-resolve.
                    let new_handle = match unsafe { live_coding::load_library(&base_dll) } {
                        Ok(handle) => handle,
                        Err(e) => {
                            println!("    ? reload failed: {e}");
                            continue;
                        }
                    };
                    // Remember the new base handle for the next unload.
                    base_handle = new_handle;
                    let new_update =
                        match unsafe { live_coding::resolve_export(new_handle, "update") } {
                            Ok(addr) => addr,
                            Err(e) => {
                                println!("    ? resolve update failed: {e}");
                                continue;
                            }
                        };
                    let new_compute =
                        match unsafe { live_coding::resolve_export(new_handle, "compute") } {
                            Ok(addr) => addr,
                            Err(e) => {
                                println!("    ? resolve compute failed: {e}");
                                continue;
                            }
                        };

                    // Point the direct callables and the prologue-patch target
                    // at the freshly rebuilt DLL.
                    compute_address = new_compute;
                    update_fn = unsafe { std::mem::transmute(new_update) };
                    compute_fn = unsafe { std::mem::transmute(new_compute) };
                    println!(
                        "    Reloaded base DLL + re-resolved in {:?}",
                        build_start.elapsed()
                    );
                    patched += 1;

                    // The rebuilt DLL now matches the current source for
                    // EVERY extracted function — commit them all to the
                    // cache so a re-save of the same bodies is treated as
                    // unchanged.
                    for entry in &funcs {
                        upsert_cached_func(&mut cached_funcs, entry.clone());
                    }
                }
                Err(errors) => {
                    // The build failed — PRINT the compiler diagnostics, then
                    // recover by reloading the PREVIOUS DLL (still intact on
                    // disk, since a failed build never overwrites it) so the
                    // game keeps running the last good code.
                    println!("    ? cargo build FAILED.  Compiler output:\n{errors}");
                    println!("    Reloading the previous DLL to keep running...");
                    match unsafe { load_base_dll(&base_dll) } {
                        Ok((old_handle, old_update, old_compute)) => {
                            base_handle = old_handle;
                            compute_address = old_compute;
                            update_fn = unsafe { std::mem::transmute(old_update) };
                            compute_fn = unsafe { std::mem::transmute(old_compute) };
                            println!("    Recovered previous DLL (still running old code).");
                        }
                        Err(e) => {
                            println!("    ? recovery failed: {e}");
                            continue;
                        }
                    }
                }
            }
        }

        println!(
                "  -- {patched} patched, {skipped} unchanged, {absorbed_by_rebuild} handled by rebuild -- total {:?} --\n",
                total_start.elapsed()
            );
    }
}

/// One iteration of game logic — calls update and compute through PLAIN
/// function pointers.  No wrapper: if a prologue was patched, the same
/// pointers transparently execute the new code.
fn game_tick(iteration: u64, update_fn: UpdateFn, compute_fn: ComputeFn) {
    let tick_val = iteration as i32;

    unsafe { update_fn(tick_val) };

    let result = unsafe { compute_fn(tick_val, 3) };
    println!("    compute({tick_val}, 3) = {result}");
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Recursively collect every `.rs` file under `root` (e.g. patch_dll/src/).
fn collect_source_files(root: &std::path::Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(directory) = stack.pop() {
        if let Ok(entries) = std::fs::read_dir(&directory) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    files.push(path);
                }
            }
        }
    }
    files.sort();
    files
}

/// Return `source` with the BODY of every `pub extern "C" fn` removed (the
/// signatures are kept).  Diffing two versions of this stripped text tells us
/// whether only function bodies changed (fast prologue-patch) or whether
/// non-body content — structs, internal helpers, attributes, `mod` decls —
/// also changed (which requires a full rebuild).
fn strip_extern_bodies(source: &str) -> String {
    let mut result = String::new();
    let mut rest = source;

    while let Some(start) = rest.find("pub extern \"C\" fn ") {
        // Keep everything before the function.
        result.push_str(&rest[..start]);
        rest = &rest[start..];

        // Find the opening brace of the body and keep the signature up to it.
        let Some(open) = rest.find('{') else {
            // No body found — keep the remainder as-is.
            result.push_str(rest);
            break;
        };
        result.push_str(&rest[..=open]);
        rest = &rest[open + 1..];

        // Drop the body up to the matching closing brace.
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
        rest = &rest[body_end + 1..];
    }

    result.push_str(rest);
    result
}

/// Insert or replace a function entry in the body cache.
///
/// The cache maps function name → (params, body, ret_type).  Only entries
/// whose changes were applied successfully are committed; this ensures a
/// failed change is re-attempted on the next save.
fn upsert_cached_func(
    cache: &mut Vec<(String, String, String, String)>,
    entry: (String, String, String, String),
) {
    if let Some(slot) = cache.iter_mut().find(|(name, _, _, _)| *name == entry.0) {
        *slot = entry;
    } else {
        cache.push(entry);
    }
}

/// Crude parser: extract `pub extern "C" fn NAME(params) [-> RetType] { body }`
/// Returns `Vec<(function_name, params, body_text, return_type)>`.
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
