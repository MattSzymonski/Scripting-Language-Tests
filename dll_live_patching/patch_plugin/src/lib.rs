// patch_plugin/src/lib.rs — patch cdylib for DLL live-patching MVP
//
// This DLL is loaded SEPARATELY from the original plugin.dll and contains
// replacement function implementations. The host redirects calls from the
// original DLL's add_v1 to this DLL's add_v2/add_v3 via jump-table swap.
//
// Stub Trampolines (Path A):
//   add_v2 and add_v3 call get_call_count() in the ORIGINAL plugin.dll via
//   a function-pointer trampoline. The host injects the original function's
//   address at startup. This demonstrates cross-module state access — the
//   same thing Subsecond's create_undefined_symbol_stub() achieves at link
//   time with generated assembly stubs.

use std::sync::atomic::{AtomicPtr, Ordering};

/// Sentinel function — exported so the host can compute this DLL's ASLR slide.
#[unsafe(no_mangle)]
pub extern "C" fn __subsecond_anchor() {}

// ── Trampoline: function pointer to the original DLL's get_call_count ────
//
// The host injects the correct address after loading both DLLs. This is a
// runtime equivalent of Subsecond's link-time stub trampolines.

static TRAMPOLINE_GET_CALL_COUNT: AtomicPtr<()> = AtomicPtr::new(std::ptr::null_mut());

/// Called by the host to wire up the trampoline. After this, add_v2 and
/// add_v3 can call into the original DLL's get_call_count.
#[unsafe(no_mangle)]
pub extern "C" fn set_get_call_count_trampoline(ptr: *const ()) {
    TRAMPOLINE_GET_CALL_COUNT.store(ptr as *mut (), Ordering::Relaxed);
}

fn call_original_increment() -> u32 {
    let ptr = TRAMPOLINE_GET_CALL_COUNT.load(Ordering::Relaxed);
    if ptr.is_null() {
        return 0; // trampoline not yet wired — graceful fallback
    }
    let func: extern "C" fn() -> u32 = unsafe { std::mem::transmute(ptr) };
    func()
}

// ── Patch implementations ───────────────────────────────────────────────

/// First replacement — returns sum + 1, and calls back into the original DLL
/// to increment the shared call counter via the trampoline.
#[unsafe(no_mangle)]
pub extern "C" fn add_v2(left: i32, right: i32) -> i32 {
    call_original_increment(); // increments original DLL's counter via trampoline
    left + right + 1
}

/// Second replacement — returns sum + 2, also calls back via trampoline.
#[unsafe(no_mangle)]
pub extern "C" fn add_v3(left: i32, right: i32) -> i32 {
    call_original_increment(); // increments original DLL's counter via trampoline
    left + right + 2
}

/// Patch for the greet function — returns length * 10 instead of length.
/// Demonstrates multi-function jump-table support (Path C).
#[unsafe(no_mangle)]
pub extern "C" fn greet_v2(name: *const std::ffi::c_char) -> i32 {
    call_original_increment(); // trampoline to original DLL
    if name.is_null() {
        return 6;
    }
    let cstr = unsafe { std::ffi::CStr::from_ptr(name) };
    (cstr.to_bytes().len() * 10) as i32
}
