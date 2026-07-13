//! Small, self-contained Win32 FFI helpers — deliberately raw `extern`
//! declarations rather than a `windows`/`winapi` dependency, since only two
//! calls are needed:
//!
//! - `GetModuleHandleW` — cheap, no-refcount-bump way to ask "is this DLL
//!   (by bare file name) already resident in this process?" Used to detect
//!   which of a module's dependencies got pulled in transitively by a single
//!   `LoadLibrary` call (dll_loading_performance_101.md section 3 step 4 /
//!   section 6's recursive-load discussion) — that set is the direct,
//!   observable evidence of "this module happened to be first to touch a
//!   shared dependency," rather than something we have to infer indirectly.
//! - `K32GetProcessMemoryInfo` — process working-set size, sampled just
//!   before/after a module's load+startup sequence, mirroring the
//!   `MemoryUsed_bytes` column already present in `AAA-Scenario.csv`.

#[cfg(windows)]
mod raw {
    use std::os::raw::c_void;

    #[link(name = "kernel32")]
    extern "system" {
        pub fn GetModuleHandleW(lp_module_name: *const u16) -> *mut c_void;
        pub fn GetCurrentProcess() -> *mut c_void;
    }

    #[repr(C)]
    #[derive(Default)]
    pub struct ProcessMemoryCounters {
        pub cb: u32,
        pub page_fault_count: u32,
        pub peak_working_set_size: usize,
        pub working_set_size: usize,
        pub quota_peak_paged_pool_usage: usize,
        pub quota_paged_pool_usage: usize,
        pub quota_peak_non_paged_pool_usage: usize,
        pub quota_non_paged_pool_usage: usize,
        pub pagefile_usage: usize,
        pub peak_pagefile_usage: usize,
    }

    #[link(name = "psapi")]
    extern "system" {
        pub fn K32GetProcessMemoryInfo(
            process: *mut c_void,
            counters: *mut ProcessMemoryCounters,
            cb: u32,
        ) -> i32;
    }
}

/// Whether a module (matched by bare file name, e.g. `"aaa_core_02.dll"`) is
/// currently resident in this process, regardless of what path it was
/// actually loaded from.
#[cfg(windows)]
pub fn is_module_resident(dll_file_name: &str) -> bool {
    let wide: Vec<u16> = dll_file_name.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe { !raw::GetModuleHandleW(wide.as_ptr()).is_null() }
}

#[cfg(not(windows))]
pub fn is_module_resident(_dll_file_name: &str) -> bool {
    false
}

/// Current process working-set size, in bytes. Best-effort: returns 0 on
/// failure (e.g. non-Windows) rather than propagating an error, since this
/// is a diagnostic metric, not something the load itself depends on.
#[cfg(windows)]
pub fn working_set_bytes() -> u64 {
    use raw::ProcessMemoryCounters;
    let mut counters = ProcessMemoryCounters {
        cb: std::mem::size_of::<ProcessMemoryCounters>() as u32,
        ..Default::default()
    };
    unsafe {
        let process = raw::GetCurrentProcess();
        if raw::K32GetProcessMemoryInfo(process, &mut counters, counters.cb) != 0 {
            counters.working_set_size as u64
        } else {
            0
        }
    }
}

#[cfg(not(windows))]
pub fn working_set_bytes() -> u64 {
    0
}
