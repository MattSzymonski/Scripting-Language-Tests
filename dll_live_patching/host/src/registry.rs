// registry.rs — Module registry for tracking loaded DLLs
//
// Stores base address, sentinel address, and image size for each registered
// library. Used by apply_dll_patch() to compute per-module ASLR slides.

use crate::types::{PatchingError, RegisteredModule};
use crate::windows_ffi;
use std::collections::HashMap;
use std::ffi::CString;
use std::sync::{LazyLock, Mutex};

static MODULE_REGISTRY: LazyLock<Mutex<HashMap<String, RegisteredModule>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Register a dynamically loaded library for hot-patching.
pub fn register_library(
    library_name: &str,
    sentinel_symbol_name: &str,
) -> Result<RegisteredModule, PatchingError> {
    let name_cstr = CString::new(library_name)
        .map_err(|_| PatchingError::LibraryNotLoaded(library_name.to_string()))?;
    let hmod = unsafe { windows_ffi::GetModuleHandleA(name_cstr.as_ptr() as *const u8) };
    if hmod.is_null() {
        return Err(PatchingError::LibraryNotLoaded(library_name.to_string()));
    }

    let mut modinfo: windows_ffi::MODULEINFO = unsafe { std::mem::zeroed() };
    let size = std::mem::size_of::<windows_ffi::MODULEINFO>() as u32;
    if unsafe { windows_ffi::GetModuleInformation(windows_ffi::GetCurrentProcess(), hmod, &mut modinfo, size) } == 0 {
        return Err(PatchingError::ModuleInfoFailed(library_name.to_string()));
    }

    let base_address = modinfo.lpBaseOfDll as usize;
    let image_size = modinfo.SizeOfImage as usize;

    let sentinel_cstr = CString::new(sentinel_symbol_name)
        .map_err(|_| PatchingError::SentinelNotExported(sentinel_symbol_name.to_string()))?;
    let sentinel_ptr = unsafe { windows_ffi::GetProcAddress(hmod, sentinel_cstr.as_ptr() as *const u8) };
    if sentinel_ptr.is_null() {
        return Err(PatchingError::SentinelNotExported(sentinel_symbol_name.to_string()));
    }
    let sentinel_address = sentinel_ptr as usize;

    if sentinel_address < base_address || sentinel_address >= base_address + image_size {
        return Err(PatchingError::SentinelNotInLibrary {
            sentinel: sentinel_address,
            name: library_name.to_string(),
        });
    }

    let mut registry = MODULE_REGISTRY.lock().unwrap();
    if registry.contains_key(library_name) {
        return Err(PatchingError::AlreadyRegistered(library_name.to_string()));
    }

    let module = RegisteredModule { name: library_name.to_string(), base_address, sentinel_address, image_size };
    registry.insert(library_name.to_string(), module.clone());
    Ok(module)
}

/// Compute the ASLR slide: runtime_sentinel - link_time_sentinel_rva.
pub fn compute_aslr_slide(module: &RegisteredModule, link_time_sentinel_rva: u64) -> usize {
    module.sentinel_address.wrapping_sub(link_time_sentinel_rva as usize)
}

/// Resolve a symbol's RVA (offset from module base) via GetProcAddress.
pub fn resolve_rva(
    library_name: &str,
    symbol_name: &str,
    base_address: usize,
) -> Result<u64, PatchingError> {
    let name_cstr = CString::new(library_name)
        .map_err(|_| PatchingError::LibraryNotLoaded(library_name.to_string()))?;
    let hmod = unsafe { windows_ffi::GetModuleHandleA(name_cstr.as_ptr() as *const u8) };
    if hmod.is_null() {
        return Err(PatchingError::LibraryNotLoaded(library_name.to_string()));
    }
    let sym_cstr = CString::new(symbol_name)
        .map_err(|_| PatchingError::SymbolNotFound(symbol_name.to_string()))?;
    let ptr = unsafe { windows_ffi::GetProcAddress(hmod, sym_cstr.as_ptr() as *const u8) };
    if ptr.is_null() {
        return Err(PatchingError::SymbolNotFound(symbol_name.to_string()));
    }
    Ok((ptr as usize - base_address) as u64)
}
