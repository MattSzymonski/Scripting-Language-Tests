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

use sss::{self as subsecond, HotFn, JumpTable};
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
// main
// ---------------------------------------------------------------------------

fn main() {
    println!("=== subsecond MVP Demo ===\n");

    demo_basic_patching();
    demo_hotfn_api();
    demo_ptr_address_change_detection();
    demo_register_handler();
    demo_nested_call_with_unwind();
    demo_multi_arg_function();
    demo_bloat_hotpatch();

    println!("\n=== All demos complete ===");
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
