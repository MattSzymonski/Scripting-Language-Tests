// patch_dll — Base DLL for the live_patch runtime.
//
// This crate compiles to a Windows DLL (cdylib).  It exports `extern "C"`
// replacement functions that the main `sss` binary loads at runtime via
// `LoadLibraryW` / `GetProcAddress`.  Each exported symbol is a "patched"
// version of a function that lives in `sss`.
//
// In a full Subsecond pipeline the CLI tool (ThinLink) would compile *only*
// the changed functions into this DLL.  Here we manually export known-good
// replacements for demonstration purposes.
mod bloat_gen;
// ---------------------------------------------------------------------------
// Greeting replacements — called in place of sss::greet_v1
// ---------------------------------------------------------------------------

/// Only greet changed again — compute & bloat should be SKIPPED now.
#[unsafe(no_mangle)]
pub extern "C" fn greet_dll() {
    println!("  [DLL v12] Second edit — only THIS function recompiled!xxx");
}

/// Another variant — demonstrates swapping to a different DLL export.
#[unsafe(no_mangle)]
pub extern "C" fn greet_dll_v2() {
    println!("  [DLL v2] Hot-patched via patch_dll.dll!");
}

// ---------------------------------------------------------------------------
// Compute replacement — called in place of sss::compute_v1
// ---------------------------------------------------------------------------

/// Replacement for compute_v1: returns x * 2 + y (v8 — final test!).
#[unsafe(no_mangle)]
pub extern "C" fn compute_dll(x: i32, y: i32) -> i32 {
    x * 3 + y
}

// ---------------------------------------------------------------------------
// Bloat replacements — called in place of bloat_gen::bloat_42 / bloat_0
// ---------------------------------------------------------------------------

/// Replacement for bloat_42: new sentinel (v5 — per-function rustc!).
#[unsafe(no_mangle)]
pub extern "C" fn bloat_42_dll() -> i32 {
    1234
}

/// Replacement for bloat_0: returns a DLL-specific sentinel.
#[unsafe(no_mangle)]
pub extern "C" fn bloat_0_dll() -> i32 {
    6666
}
