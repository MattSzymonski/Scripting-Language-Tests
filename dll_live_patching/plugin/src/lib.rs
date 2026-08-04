// plugin/src/lib.rs — original cdylib for DLL live-patching MVP Step 1
//
// Exports:
//   __subsecond_anchor  — empty sentinel function for ASLR slide computation
//   add_v1              — original implementation: a + b
//   get_call_count      — returns the call counter (proves state survives patches)
//
// The replacement functions (add_v2, add_v3) now live in a SEPARATE DLL
// (patch_plugin/). This is the real Subsecond architecture: the original
// DLL is never replaced; patched code lives in a separate patch dylib.

use std::sync::atomic::{AtomicU32, Ordering};

/// Sentinel function — exists solely to provide a stable, exported address
/// that the host can use to compute the DLL's ASLR slide at runtime.
#[unsafe(no_mangle)]
pub extern "C" fn __subsecond_anchor() {}

/// A counter preserved across "patches" to verify that DLL state survives
/// function-pointer redirection (even when the new implementation lives
/// in a different DLL).
static CALL_COUNT: AtomicU32 = AtomicU32::new(0);

/// Original implementation — returns the sum of two integers.
#[unsafe(no_mangle)]
pub extern "C" fn add_v1(left: i32, right: i32) -> i32 {
    CALL_COUNT.fetch_add(1, Ordering::Relaxed);
    left + right
}

/// Returns the current call count, so the host can verify state is preserved
/// across function-pointer redirections.
#[unsafe(no_mangle)]
pub extern "C" fn get_call_count() -> u32 {
    CALL_COUNT.load(Ordering::Relaxed)
}

/// Increment and return the call count. Called by patch DLLs via trampoline
/// so they can update the original DLL's shared state.
#[unsafe(no_mangle)]
pub extern "C" fn increment_call_count() -> u32 {
    CALL_COUNT.fetch_add(1, Ordering::Relaxed) + 1
}

/// Second hot-reloadable function — returns a greeting string length.
/// Demonstrates multi-function jump-table support (Path C).
#[unsafe(no_mangle)]
pub extern "C" fn greet(name: *const std::ffi::c_char) -> i32 {
    CALL_COUNT.fetch_add(1, Ordering::Relaxed);
    if name.is_null() {
        return 6;
    } // "world!"
    let cstr = unsafe { std::ffi::CStr::from_ptr(name) };
    cstr.to_bytes().len() as i32
}
