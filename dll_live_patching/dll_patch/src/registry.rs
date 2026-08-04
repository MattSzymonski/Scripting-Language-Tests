// registry.rs — Module registry for tracking loaded DLLs
//
// Stores base address, sentinel address, and image size for each registered
// library. Used by the patching engine to compute per-module ASLR slides.

use crate::types::{PatchError, RegisteredModule};
use std::collections::HashMap;
use std::ffi::CString;
use std::sync::{LazyLock, Mutex};

// ── Windows FFI ──────────────────────────────────────────────────────────

pub(crate) mod windows_ffi {
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
            hProcess: *mut c_void,
            hModule: *mut c_void,
            lpmodinfo: *mut MODULEINFO,
            cb: u32,
        ) -> i32;
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    #[allow(non_snake_case)]
    pub struct MODULEINFO {
        pub lpBaseOfDll: *mut c_void,
        pub SizeOfImage: u32,
        pub EntryPoint: *mut c_void,
    }
}

// ── Global Registry ──────────────────────────────────────────────────────

static MODULE_REGISTRY: LazyLock<Mutex<HashMap<String, RegisteredModule>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Register a dynamically loaded library for hot-patching.
///
/// The sentinel must be an exported `extern "C"` function whose runtime
/// address is stable across the library's lifetime. It serves as the
/// ASLR reference point — the same role `main` plays for executables
/// in Subsecond's existing design.
///
/// # Safety
/// `sentinel_ptr` must point to a valid function in the named library.
/// The library must remain loaded for the lifetime of all patches.
pub unsafe fn register_library(
    library_name: &str,
    sentinel_ptr: *const std::ffi::c_void,
) -> Result<RegisteredModule, PatchError> {
    let name_cstr = CString::new(library_name)
        .map_err(|_| PatchError::LibraryNotLoaded(library_name.to_string()))?;
    let hmod = unsafe { windows_ffi::GetModuleHandleA(name_cstr.as_ptr() as *const u8) };
    if hmod.is_null() {
        return Err(PatchError::LibraryNotLoaded(library_name.to_string()));
    }

    let mut modinfo: windows_ffi::MODULEINFO = unsafe { std::mem::zeroed() };
    let size = std::mem::size_of::<windows_ffi::MODULEINFO>() as u32;
    if unsafe {
        windows_ffi::GetModuleInformation(
            windows_ffi::GetCurrentProcess(),
            hmod,
            &mut modinfo,
            size,
        )
    } == 0
    {
        return Err(PatchError::ModuleInfoFailed(library_name.to_string()));
    }

    let base_address = modinfo.lpBaseOfDll as usize;
    let image_size = modinfo.SizeOfImage as usize;
    let sentinel_address = sentinel_ptr as usize;

    if sentinel_address < base_address || sentinel_address >= base_address + image_size {
        return Err(PatchError::SentinelNotInLibrary {
            sentinel: sentinel_address,
            name: library_name.to_string(),
        });
    }

    let mut registry = MODULE_REGISTRY.lock().unwrap();
    if registry.contains_key(library_name) {
        return Err(PatchError::AlreadyRegistered(library_name.to_string()));
    }

    let module = RegisteredModule {
        name: library_name.to_string(),
        base_address,
        sentinel_address,
        image_size,
    };
    registry.insert(library_name.to_string(), module.clone());
    Ok(module)
}

/// Register a library by looking up its sentinel by name via GetProcAddress.
///
/// # Safety
/// Same requirements as `register_library`.
pub unsafe fn register_library_by_name(
    library_name: &str,
    sentinel_name: &str,
) -> Result<RegisteredModule, PatchError> {
    let name_cstr = CString::new(library_name)
        .map_err(|_| PatchError::LibraryNotLoaded(library_name.to_string()))?;
    let hmod = unsafe { windows_ffi::GetModuleHandleA(name_cstr.as_ptr() as *const u8) };
    if hmod.is_null() {
        return Err(PatchError::LibraryNotLoaded(library_name.to_string()));
    }
    let sentinel_cstr = CString::new(sentinel_name)
        .map_err(|_| PatchError::SentinelNotExported(sentinel_name.to_string()))?;
    let sentinel_ptr =
        unsafe { windows_ffi::GetProcAddress(hmod, sentinel_cstr.as_ptr() as *const u8) };
    if sentinel_ptr.is_null() {
        return Err(PatchError::SentinelNotExported(sentinel_name.to_string()));
    }
    unsafe { register_library(library_name, sentinel_ptr) }
}

/// Compute the ASLR slide: runtime_sentinel - link_time_sentinel_rva.
pub fn compute_aslr_slide(module: &RegisteredModule, link_time_sentinel_rva: u64) -> usize {
    module
        .sentinel_address
        .wrapping_sub(link_time_sentinel_rva as usize)
}

/// Resolve a symbol's RVA (offset from module base) via GetProcAddress.
pub fn resolve_rva(
    library_name: &str,
    symbol_name: &str,
    base_address: usize,
) -> Result<u64, PatchError> {
    let name_cstr = CString::new(library_name)
        .map_err(|_| PatchError::LibraryNotLoaded(library_name.to_string()))?;
    let hmod = unsafe { windows_ffi::GetModuleHandleA(name_cstr.as_ptr() as *const u8) };
    if hmod.is_null() {
        return Err(PatchError::LibraryNotLoaded(library_name.to_string()));
    }
    let sym_cstr = CString::new(symbol_name)
        .map_err(|_| PatchError::SymbolNotFound(symbol_name.to_string()))?;
    let ptr = unsafe { windows_ffi::GetProcAddress(hmod, sym_cstr.as_ptr() as *const u8) };
    if ptr.is_null() {
        return Err(PatchError::SymbolNotFound(symbol_name.to_string()));
    }
    Ok((ptr as usize - base_address) as u64)
}

/// Get a handle to a loaded module for use with GetProcAddress etc.
pub fn get_module_handle(library_name: &str) -> Result<*mut std::ffi::c_void, PatchError> {
    let name_cstr = CString::new(library_name)
        .map_err(|_| PatchError::LibraryNotLoaded(library_name.to_string()))?;
    let hmod = unsafe { windows_ffi::GetModuleHandleA(name_cstr.as_ptr() as *const u8) };
    if hmod.is_null() {
        return Err(PatchError::LibraryNotLoaded(library_name.to_string()));
    }
    Ok(hmod)
}
