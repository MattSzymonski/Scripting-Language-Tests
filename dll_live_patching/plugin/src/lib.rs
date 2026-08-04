// plugin/src/lib.rs — cdylib for DLL live-patching MVP
//
// Exports:
//   __subsecond_anchor  — empty sentinel function for ASLR slide computation
//   add_v1              — original implementation: a + b
//   add_v2              — "patched" implementation: a + b + 1
//   add_v3              — second "patch": a + b + 2
//
// In a production system, add_v2/add_v3 would live in a separate patch DLL
// built by the CLI. For the MVP, all versions coexist in the same module so
// we can test redirection without a second compilation step.

use std::sync::atomic::{AtomicU32, Ordering};

/// Sentinel function — exists solely to provide a stable, exported address
/// that the host can use to compute the DLL's ASLR slide at runtime.
/// The function body is empty; it is never called.
#[unsafe(no_mangle)]
pub extern "C" fn __subsecond_anchor() {}

/// A counter preserved across "patches" to verify that DLL state survives
/// function-pointer redirection.
static CALL_COUNT: AtomicU32 = AtomicU32::new(0);

/// Original implementation — returns the sum of two integers.
#[unsafe(no_mangle)]
pub extern "C" fn add_v1(left: i32, right: i32) -> i32 {
    CALL_COUNT.fetch_add(1, Ordering::Relaxed);
    left + right
}

/// First "patch" — returns sum + 1.
/// In the MVP this lives in the same DLL. In production, this would be
/// compiled into a separate patch DLL and loaded alongside the original.
#[unsafe(no_mangle)]
pub extern "C" fn add_v2(left: i32, right: i32) -> i32 {
    CALL_COUNT.fetch_add(1, Ordering::Relaxed);
    left + right + 1
}

/// Second "patch" — returns sum + 2.
/// Demonstrates that patches can be applied sequentially.
#[unsafe(no_mangle)]
pub extern "C" fn add_v3(left: i32, right: i32) -> i32 {
    CALL_COUNT.fetch_add(1, Ordering::Relaxed);
    left + right + 2
}

/// Returns the current call count, so the host can verify state is preserved
/// across function-pointer redirections.
#[unsafe(no_mangle)]
pub extern "C" fn get_call_count() -> u32 {
    CALL_COUNT.load(Ordering::Relaxed)
}
