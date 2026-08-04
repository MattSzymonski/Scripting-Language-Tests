// windows_ffi.rs — Windows FFI declarations used by registry and jump_table

use std::ffi::c_void;

#[link(name = "kernel32")]
unsafe extern "system" {
    pub fn GetModuleHandleA(lpModuleName: *const u8) -> *mut c_void;
    pub fn GetProcAddress(hModule: *mut c_void, lpProcName: *const u8) -> *mut c_void;
    pub fn GetCurrentProcess() -> *mut c_void;
}

#[link(name = "psapi")]
unsafe extern "system" {
    pub fn GetModuleInformation(
        hProcess: *mut c_void, hModule: *mut c_void,
        lpmodinfo: *mut MODULEINFO, cb: u32,
    ) -> i32;
}

#[repr(C)] #[derive(Clone, Copy, Debug, Default)] #[allow(non_snake_case)]
pub struct MODULEINFO {
    pub lpBaseOfDll: *mut c_void,
    pub SizeOfImage: u32,
    pub EntryPoint: *mut c_void,
}
