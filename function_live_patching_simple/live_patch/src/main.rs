// REQUIREMENTS
//   Rust stable (edition 2021).  Run from the `sss/` directory:
//     cargo run
//
// DESCRIPTION
//   Interactive demo of the `sss` (subsecond-simple) hot-patching runtime.
//   Demonstrates: basic jump-table patching with fn pointers, `HotFn::current`,
//   `ptr_address` change detection, `register_handler` callbacks, and nested
//   `call()` with automatic unwind / retry.
//
//   IMPORTANT: Subsecond's fast path works with `fn(...)` *pointer* types.
//   Rust function *items* (e.g. bare `greet_v1`) are zero-sized types with a
//   unique compiler-generated `call_it`.  The slow path (used for closures)
//   requires the compiler toolchain (ThinLink) to populate the jump table
//   correctly.  This demo therefore uses explicit `fn()` casts for the
//   function-pointer fast path.
//
// USAGE
//   cargo run
//
// --- SCRIPT ---

use live_patch::{self as subsecond, HotFn, JumpTable};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

mod bloat_gen;

// ---------------------------------------------------------------------------
// Demo "handler" functions — these simulate code that could be hot-reloaded
// ---------------------------------------------------------------------------

fn greet_v1() {
    println!("  [v1] Hello, world!");
}

fn greet_v2() {
    println!("  [v2] Hello, hot-reloaded world!");
}

fn greet_v3() {
    println!("  [v3] Welcome to subsecond-powered hot-patching!");
}

fn compute_v1(x: i32, y: i32) -> i32 {
    x + y
}

fn compute_v2(x: i32, y: i32) -> i32 {
    x * y
}

// ---------------------------------------------------------------------------
// main — CLI dispatch
// ---------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "loop" {
        run_hot_reload_loop();
    } else {
        println!("=== subsecond MVP Demo ===\n");

        demo_basic_patching();
        demo_hotfn_api();
        demo_ptr_address_change_detection();
        demo_register_handler();
        demo_nested_call_with_unwind();
        demo_multi_arg_function();
        demo_bloat_hotpatch();
        demo_dll_hotpatch();

        println!("\n=== All demos complete ===");
        println!("  (run with 'cargo run -p live_patch -- loop' for live-reload mode)");
    }
}

// ---------------------------------------------------------------------------
// Live-reload loop — polls patch_dll.dll for changes and hot-patches in
// real time.  Edit patch_dll/src/lib.rs, run `cargo build -p patch_dll`,
// and watch the running process pick up the new code without restarting.
// ---------------------------------------------------------------------------

fn run_hot_reload_loop() {
    let exe_dir = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let base_dll = exe_dir.join("patch_dll.dll");
    let workspace_root = exe_dir.parent().unwrap().parent().unwrap();

    let reload_source = workspace_root.join("patch_dll").join("src").join("lib.rs");

    let symbol_map: &[(&str, u64)] = &[
        ("greet_dll", greet_v1 as *const () as usize as u64),
        ("compute_dll", compute_v1 as *const () as usize as u64),
        (
            "bloat_42_dll",
            bloat_gen::bloat_42 as *const () as usize as u64,
        ),
    ];

    // ── Load the BASE DLL once ──
    print!("  Loading base DLL (patch_dll.dll)... ");
    match unsafe { subsecond::reload_patch_dll(&base_dll, symbol_map) } {
        Ok(n) => println!("{n} functions linked.  Base DLL stays loaded."),
        Err(e) => println!("not found ({e}) — build with: cargo build -p patch_dll"),
    }

    // ── Watch state ──
    let mut last_reload_source = file_modified_time(&reload_source);
    // Cache of previous function bodies: (name, params, body, ret_type).
    let mut cached_funcs: Vec<(String, String, String, String)> = Vec::new();

    println!();
    println!("  ╔══════════════════════════════════════════════════════════════╗");
    println!("  ║   LIVE PATCH LOOP — edit a function, save, watch it reload  ║");
    println!("  ║                                                              ║");
    println!("  ║   Edit:   patch_dll/src/lib.rs                            ║");
    println!("  ║   Effect: only changed functions recompiled (~0.16s)   ║");
    println!("  ║                                                              ║");
    println!("  ║   Base DLL stays loaded.  Only changed pointers swap.       ║");
    println!("  ║   Exit: press Ctrl+C                                         ║");
    println!("  ╚══════════════════════════════════════════════════════════════╝");
    println!();

    let mut iteration: u64 = 0;

    loop {
        iteration += 1;
        tick(iteration);
        std::thread::sleep(std::time::Duration::from_millis(800));

        let cur_src = file_modified_time(&reload_source);
        if cur_src != last_reload_source {
            last_reload_source = cur_src;
            println!("\n  ── patch_dll source edited (iter {iteration}) ──");
            println!("  ── Compiling each function individually with rustc (no full relink)... ──");

            // Read the current source to extract function bodies.
            let source = match std::fs::read_to_string(&reload_source) {
                Ok(s) => s,
                Err(e) => {
                    println!("  ✗ Cannot read source: {e}");
                    continue;
                }
            };

            // Extract current function bodies from the source.
            let funcs = extract_function_bodies(&source);

            let total_start = Instant::now();
            let mut patched = 0usize;
            let mut skipped = 0usize;

            for (name, params, body, ret_type) in &funcs {
                let orig_addr = match name.as_str() {
                    "greet_dll" => greet_v1 as *const () as usize as u64,
                    "compute_dll" => compute_v1 as *const () as usize as u64,
                    "bloat_42_dll" => bloat_gen::bloat_42 as *const () as usize as u64,
                    _ => continue,
                };

                // Check if this function's body actually changed.
                let changed = cached_funcs
                    .iter()
                    .find(|(n, _, b, _)| n == name && b == body)
                    .is_none();

                if !changed {
                    skipped += 1;
                    continue;
                }

                print!("    {name}... ");
                let func_start = Instant::now();
                match unsafe {
                    subsecond::compile_and_patch(name, params, body, ret_type, orig_addr)
                } {
                    Ok(_) => {
                        println!("✓ compiled + patched in {:?}", func_start.elapsed());
                        patched += 1;
                    }
                    Err(e) => println!("✗ {e}"),
                }
            }

            // Update the cache for next time.
            cached_funcs = funcs;

            println!(
                "  ── {patched} changed, {skipped} unchanged — patched in {:?} (no 5000-function relink!) ──\n",
                total_start.elapsed()
            );
        }
    }
}

/// One iteration of "business logic" — calls functions that may be patched.
fn tick(iteration: u64) {
    // greet_v1 → potentially DLL-patched
    let fp: fn() = greet_v1 as fn();
    let mut hot_greet = HotFn::current(fp);

    print!("  [{iteration:>4}] ");
    hot_greet.call(());

    // compute_v1 → potentially DLL-patched
    let fp2: fn(i32, i32) -> i32 = compute_v1;
    let mut hot_compute = HotFn::current(fp2);
    let result = hot_compute.call((iteration as i32, 3));
    print!("  compute({}, 3) = {:>4}", iteration, result);

    // bloat_42 → potentially DLL-patched
    let fp3: fn() -> i32 =
        unsafe { std::mem::transmute::<extern "C" fn() -> i32, fn() -> i32>(bloat_gen::bloat_42) };
    let mut hot_bloat = HotFn::current(fp3);
    let bloat_val = hot_bloat.call(());
    println!("  bloat_42() = {}", bloat_val);
}

/// Return the last-modified time of `path`, or `None` if it doesn't exist.
fn file_modified_time(path: &std::path::Path) -> Option<std::time::SystemTime> {
    std::fs::metadata(path).ok().and_then(|m| m.modified().ok())
}

// ---------------------------------------------------------------------------
// Demo 1 — basic jump-table patching with fn pointers
// ---------------------------------------------------------------------------

fn demo_basic_patching() {
    println!("─── Demo 1: Basic patching via HotFn::current with fn pointers ───");

    // Cast to fn() so we get a pointer-sized type (fast path).
    // Rust function *items* (bare `greet_v1`) are ZSTs and use the slow path.
    let function_pointer: fn() = greet_v1;
    let mut hot = HotFn::current(function_pointer);

    println!("  Before patch:");
    hot.call(());

    // Build a jump table that redirects greet_v1 → greet_v2.
    let mut map = HashMap::new();
    map.insert(function_to_addr(greet_v1), function_to_addr(greet_v2));
    unsafe { subsecond::apply_patch(JumpTable { map }).unwrap() };

    println!("  After patch (greet_v1 → greet_v2):");
    hot.call(());

    // Patch again: greet_v1 → greet_v3
    let mut map = HashMap::new();
    map.insert(function_to_addr(greet_v1), function_to_addr(greet_v3));
    unsafe { subsecond::apply_patch(JumpTable { map }).unwrap() };

    println!("  After second patch (greet_v1 → greet_v3):");
    hot.call(());

    println!();
}

// ---------------------------------------------------------------------------
// Demo 2 — HotFn::current API (the library-author interface)
// ---------------------------------------------------------------------------

fn demo_hotfn_api() {
    println!("─── Demo 2: HotFn::current() API ───");

    let function_pointer: fn() = greet_v1;
    let mut hot_greet = HotFn::current(function_pointer);

    println!("  Calling via HotFn::call():");
    hot_greet.call(());

    let mut map = HashMap::new();
    map.insert(function_to_addr(greet_v1), function_to_addr(greet_v2));
    unsafe { subsecond::apply_patch(JumpTable { map }).unwrap() };

    println!("  After patch:");
    hot_greet.call(());

    println!();
}

// ---------------------------------------------------------------------------
// Demo 3 — ptr_address() change detection
// ---------------------------------------------------------------------------

fn demo_ptr_address_change_detection() {
    println!("─── Demo 3: ptr_address() change detection ───");

    let function_pointer: fn() = greet_v1;
    let hot = HotFn::current(function_pointer);
    let address_before = hot.ptr_address();
    println!("  ptr_address before patch: 0x{:016x}", address_before.0);

    // Map to a *different* target than what's currently in the jump table
    // (Demo 2 left greet_v1→greet_v2, so we map to greet_v3 instead).
    let mut map = HashMap::new();
    map.insert(function_to_addr(greet_v1), function_to_addr(greet_v3));
    unsafe { subsecond::apply_patch(JumpTable { map }).unwrap() };

    let address_after = hot.ptr_address();
    println!("  ptr_address after  patch: 0x{:016x}", address_after.0);
    println!("  Address changed: {}", address_before != address_after);

    println!();
}

// ---------------------------------------------------------------------------
// Demo 4 — register_handler() callback
// ---------------------------------------------------------------------------

fn demo_register_handler() {
    println!("─── Demo 4: register_handler() callback ───");

    subsecond::register_handler(Arc::new(|| {
        println!("  >>> Handler notified: a patch was just applied!");
    }));

    let mut map = HashMap::new();
    map.insert(function_to_addr(greet_v1), function_to_addr(greet_v2));
    unsafe { subsecond::apply_patch(JumpTable { map }).unwrap() };

    let mut map = HashMap::new();
    map.insert(function_to_addr(greet_v1), function_to_addr(greet_v3));
    unsafe { subsecond::apply_patch(JumpTable { map }).unwrap() };

    println!();
}

// ---------------------------------------------------------------------------
// Demo 5 — nested subsecond::call() with HotFnPanic unwind / retry
// ---------------------------------------------------------------------------

fn demo_nested_call_with_unwind() {
    println!("─── Demo 5: Nested call() unwind / retry ───");

    // Subsecond's `call()` catches HotFnPanic and retries automatically.
    // This means edits to code *inside* the closure take effect on retry,
    // while edits *above* the call site cause a safe unwind.

    let mut counter: u32 = 0;

    subsecond::call(|| {
        // Outer call — if code above us changes we unwind to here.
        println!("  Outer call (iteration {})", counter);

        subsecond::call(|| {
            // Inner call — isolated hot-reload boundary.
            counter += 1;
            println!("    Inner call — counter is now {}", counter);
        });

        if counter >= 3 {
            println!("  Counter reached {}, demo complete.", counter);
        } else {
            // Simulate a "stale caller" scenario: in a real app this happens
            // automatically when the compiler rebuilds changed code above the
            // call site.  The enclosing `subsecond::call` loop catches
            // HotFnPanic, unwinds to the outer call boundary, and retries.
            println!("    (simulating stale caller unwind...)");
            std::panic::resume_unwind(Box::new(subsecond::HotFnPanic::new()));
        }
    });

    println!();
}

// ---------------------------------------------------------------------------
// Demo 6 — multi-argument function patching
// ---------------------------------------------------------------------------

fn demo_multi_arg_function() {
    println!("─── Demo 6: Multi-argument function patching ───");

    let function_pointer: fn(i32, i32) -> i32 = compute_v1;
    let mut hot_compute = HotFn::current(function_pointer);

    let result = hot_compute.call((3, 4));
    println!("  compute_v1(3, 4) = {}  (expected: 3 + 4 = 7)", result);
    assert_eq!(result, 7);

    // Patch compute_v1 → compute_v2
    let mut map = HashMap::new();
    map.insert(
        function_to_addr_2(compute_v1),
        function_to_addr_2(compute_v2),
    );
    unsafe { subsecond::apply_patch(JumpTable { map }).unwrap() };

    let result = hot_compute.call((3, 4));
    println!("  compute_v2(3, 4) = {}  (expected: 3 * 4 = 12)", result);
    assert_eq!(result, 12);

    println!();
}

// ---------------------------------------------------------------------------
// Helpers — safe conversions from function pointers to u64
// ---------------------------------------------------------------------------

/// Convert a zero-argument function pointer to a u64 address.
fn function_to_addr(function: fn()) -> u64 {
    function as usize as u64
}

/// Convert a two-argument function pointer to a u64 address.
fn function_to_addr_2<T1, T2, R>(function: fn(T1, T2) -> R) -> u64 {
    function as usize as u64
}

// ---------------------------------------------------------------------------
// Alternative implementations for bloat functions (simulate "recompiled" code)
// ---------------------------------------------------------------------------

/// Replacement for bloat_42 — would be the result of recompiling just this
/// one function after a developer changed its body.
fn bloat_42_v2() -> i32 {
    // New business logic: returns a sentinel value instead of computing.
    9999
}

/// Replacement for bloat_0 — another patched variant.
fn bloat_0_v2() -> i32 {
    // Returns a fixed value.
    7777
}

// ---------------------------------------------------------------------------
// Demo 7 — bloat-file hot-patching (the "skip recompilation" demo)
// ---------------------------------------------------------------------------

fn demo_bloat_hotpatch() {
    println!("─── Demo 7: Bloat-file hot-patching (5 000 functions, ~50 K lines) ───");
    println!("  Concept: bloat_gen.rs contains 5 000 auto-generated functions.");
    println!("  In a traditional workflow, changing ONE function means waiting");
    println!("  for rustc to recompile ALL 5 000 functions (seconds).");
    println!("  With Subsecond, only the changed function is recompiled and");
    println!("  a single jump-table pointer is updated (microseconds).\n");

    // Snapshot a few bloat functions to show they are alive.
    let direct_42 = bloat_gen::bloat_42();
    let direct_0 = bloat_gen::bloat_0();
    let direct_1000 = bloat_gen::bloat_1000();
    let direct_4999 = bloat_gen::bloat_4999();
    println!("  Direct calls (bypass jump table):");
    println!("    bloat_0()    = {}", direct_0);
    println!("    bloat_42()   = {}", direct_42);
    println!("    bloat_1000() = {}", direct_1000);
    println!("    bloat_4999() = {}", direct_4999);

    // --- Patch bloat_42 ---
    // transmute extern "C" fn → Rust fn (sound on x64 Windows — same ABI).
    // Only Rust-abi fn pointers implement FnMut, which HotFn requires.
    let function_pointer: fn() -> i32 =
        unsafe { std::mem::transmute::<extern "C" fn() -> i32, fn() -> i32>(bloat_gen::bloat_42) };
    let mut hot_bloat_42 = HotFn::current(function_pointer);

    let before = hot_bloat_42.call(());
    println!(
        "\n  HotFn call to bloat_42 before patch: {} (original logic)",
        before
    );

    // Measure the patch time.
    let start = Instant::now();
    let mut map = HashMap::new();
    map.insert(
        bloat_gen::bloat_42 as *const () as usize as u64,
        bloat_42_v2 as *const () as usize as u64,
    );
    unsafe { subsecond::apply_patch(JumpTable { map }).unwrap() };
    let elapsed = start.elapsed();

    let after = hot_bloat_42.call(());
    println!(
        "  HotFn call to bloat_42 after  patch: {} (patched → sentinel 9999)",
        after
    );
    println!(
        "  Patch applied in {:?} — no recompilation of the other 4 999 functions!",
        elapsed
    );

    // Show that other bloat functions are completely unaffected.
    let zero_after = bloat_gen::bloat_0();
    let k_after = bloat_gen::bloat_1000();
    let end_after = bloat_gen::bloat_4999();
    println!("\n  Other bloat functions unaffected (still original logic):");
    println!(
        "    bloat_0()    = {} (same as before: {})",
        zero_after, direct_0
    );
    println!(
        "    bloat_1000() = {} (same as before: {})",
        k_after, direct_1000
    );
    println!(
        "    bloat_4999() = {} (same as before: {})",
        end_after, direct_4999
    );
    assert_eq!(zero_after, direct_0);
    assert_eq!(k_after, direct_1000);
    assert_eq!(end_after, direct_4999);

    // --- Patch bloat_0 as well (two independent patches, same speed) ---
    let fptr: fn() -> i32 =
        unsafe { std::mem::transmute::<extern "C" fn() -> i32, fn() -> i32>(bloat_gen::bloat_0) };
    let mut hot_bloat_0 = HotFn::current(fptr);
    let before_0 = hot_bloat_0.call(());

    let start2 = Instant::now();
    let mut map2 = HashMap::new();
    map2.insert(
        bloat_gen::bloat_0 as *const () as usize as u64,
        bloat_0_v2 as *const () as usize as u64,
    );
    unsafe { subsecond::apply_patch(JumpTable { map: map2 }).unwrap() };
    let elapsed2 = start2.elapsed();

    let after_0 = hot_bloat_0.call(());
    println!(
        "\n  bloat_0 before patch: {}  →  after patch: {} (sentinel 7777)",
        before_0, after_0
    );
    println!("  Second patch applied in {:?}.", elapsed2);

    // --- Summary ---
    println!("\n  ═══════════════════════════════════════════════");
    println!("  Takeaway:");
    println!("    • 5 000 functions, ~50 K lines of Rust");
    println!(
        "    • Patching one function: {:?} (jump-table update)",
        elapsed
    );
    println!("    • Recompiling all 5 000: typically 2–10 seconds");
    println!("    • Speedup: orders of magnitude");
    println!("    • Other functions: completely unaffected");
    println!("  ═══════════════════════════════════════════════");

    println!();
}

// ---------------------------------------------------------------------------
// Demo 8 — DLL-based hot-patching (load patch_dll.dll at runtime)
// ---------------------------------------------------------------------------

fn demo_dll_hotpatch() {
    println!("─── Demo 8: DLL-based hot-patching (patch_dll.dll) ───");
    println!("  Concept: instead of manually constructing a JumpTable,");
    println!("  we load a pre-compiled DLL (patch_dll.dll) that exports");
    println!("  replacement functions.  This is the same flow the real");
    println!("  Subsecond uses: compile changed functions → DLL → load → patch.\n");

    // Locate patch_dll.dll next to the executable.
    let dll_path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("patch_dll.dll");

    if !dll_path.exists() {
        println!("  patch_dll.dll not found at '{}'", dll_path.display());
        println!("  Build it first:  cargo build -p patch_dll");
        println!("  Then copy:       cp target/debug/patch_dll.dll target/debug/");
        println!("  (Skipping DLL demo.)\n");
        return;
    }

    println!("  Found DLL: {}", dll_path.display());

    // Build a symbol map: (DLL export name, original function address).
    // The original addresses come from functions defined in THIS binary.
    let symbol_map: &[(&str, u64)] = &[
        ("greet_dll", greet_v1 as *const () as usize as u64),
        ("compute_dll", compute_v1 as *const () as usize as u64),
        (
            "bloat_42_dll",
            bloat_gen::bloat_42 as *const () as usize as u64,
        ),
        (
            "bloat_0_dll",
            bloat_gen::bloat_0 as *const () as usize as u64,
        ),
    ];

    // Load the DLL and resolve symbols.
    let start = Instant::now();
    let jump_table = unsafe { subsecond::load_patch_dll(&dll_path, symbol_map) };
    let load_elapsed = start.elapsed();

    match jump_table {
        Ok(table) => {
            println!(
                "  DLL loaded + {} symbols resolved in {:?}",
                table.map.len(),
                load_elapsed
            );

            // Apply the jump table.
            let start2 = Instant::now();
            unsafe { subsecond::apply_patch(table).unwrap() };
            let apply_elapsed = start2.elapsed();
            println!("  Jump table applied in {:?}", apply_elapsed);

            // --- Verify: greet_v1 now dispatches to DLL ---
            let fp: fn() = greet_v1 as fn();
            let mut hot_greet = HotFn::current(fp);
            println!("\n  Calling greet_v1 through HotFn (should hit DLL):");
            hot_greet.call(());

            // --- Verify: compute_v1 now dispatches to DLL ---
            let fp2: fn(i32, i32) -> i32 = compute_v1;
            let mut hot_compute = HotFn::current(fp2);
            let result = hot_compute.call((6, 7));
            println!(
                "  compute_v1(6, 7) = {}  (DLL multiplies: 6*7=42, original adds: 13)",
                result
            );
            assert_eq!(result, 42, "DLL compute_dll should multiply");

            // --- Verify: bloat_42 now dispatches to DLL ---
            let fp3: fn() -> i32 = unsafe {
                std::mem::transmute::<extern "C" fn() -> i32, fn() -> i32>(bloat_gen::bloat_42)
            };
            let mut hot_bloat = HotFn::current(fp3);
            let bloat_result = hot_bloat.call(());
            println!(
                "  bloat_42 through HotFn = {}  (DLL sentinel: 8888)",
                bloat_result
            );
            assert_eq!(bloat_result, 8888, "DLL bloat_42_dll should return 8888");

            // --- Verify: direct call still hits original ---
            let direct = bloat_gen::bloat_42();
            println!(
                "  bloat_42 direct call      = {}  (original, bypasses jump table)",
                direct
            );
            // Direct call does NOT go through the jump table!
            assert_ne!(direct, 8888);

            // --- Verify: bloat_0 redirected too ---
            let fp4: fn() -> i32 = unsafe {
                std::mem::transmute::<extern "C" fn() -> i32, fn() -> i32>(bloat_gen::bloat_0)
            };
            let mut hot_bloat0 = HotFn::current(fp4);
            let b0 = hot_bloat0.call(());
            println!("  bloat_0 through HotFn  = {}  (DLL sentinel: 6666)", b0);
            assert_eq!(b0, 6666, "DLL bloat_0_dll should return 6666");

            // --- Summary ---
            println!("\n  ═══════════════════════════════════════════════");
            println!("  DLL hot-patching summary:");
            println!("    • DLL load + resolve: {:?}", load_elapsed);
            println!("    • Jump table apply:   {:?}", apply_elapsed);
            println!(
                "    • Total patch time:   {:?}",
                load_elapsed + apply_elapsed
            );
            println!("    • 4 functions patched from a pre-compiled DLL");
            println!("    • Direct calls still hit original code (safe)");
            println!("  ═══════════════════════════════════════════════");
        }
        Err(error) => {
            println!("  Failed to load DLL: {error}");
        }
    }

    println!();
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Crude parser: extract `pub extern "C" fn NAME(params) [-> RetType] { body }`
/// Returns `Vec<(function_name, params, body_text, return_type)>`.
fn extract_function_bodies(source: &str) -> Vec<(String, String, String, String)> {
    let mut results = Vec::new();
    let mut rest = source;

    while let Some(start) = rest.find("pub extern \"C\" fn ") {
        rest = &rest[start + 18..]; // skip prefix

        // Function name.
        let name_end = rest.find('(').unwrap_or(rest.len());
        let name = rest[..name_end].trim().to_string();
        rest = &rest[name_end + 1..]; // skip '('

        // Parameter list — find matching ')'.
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
        rest = &rest[params_end + 1..]; // skip ')'

        // Return type (optional).
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

        // Body — find matching '}'.
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
