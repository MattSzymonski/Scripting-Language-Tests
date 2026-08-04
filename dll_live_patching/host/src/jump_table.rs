// jump_table.rs — Global jump table, DLL-aware patch application, and hot-function dispatch
//
// This is the core patching engine. It mirrors Subsecond's apply_patch() +
// APP_JUMP_TABLE but uses per-DLL sentinels instead of the EXE's hardcoded
// 'main' symbol.

use crate::types::{AddressMap, LocalJumpTable, PatchingError, RegisteredModule};
use crate::windows_ffi;
use std::ffi::CString;
use std::sync::atomic::{AtomicPtr, Ordering};

// ── Global Jump Table ─────────────────────────────────────────────────────

pub static APP_JUMP_TABLE: AtomicPtr<LocalJumpTable> = AtomicPtr::new(std::ptr::null_mut());

pub fn commit_jump_table(table: LocalJumpTable) {
    let old = APP_JUMP_TABLE.swap(Box::into_raw(Box::new(table)), Ordering::Relaxed);
    if !old.is_null() {
        let _leaked = unsafe { Box::from_raw(old) };
    }
}

pub fn lookup_in_jump_table(original_runtime_addr: u64) -> Option<u64> {
    let table_ptr = APP_JUMP_TABLE.load(Ordering::Relaxed);
    if table_ptr.is_null() { return None; }
    let table = unsafe { &*table_ptr };
    table.map.get(&original_runtime_addr).copied()
}

// ── DLL-Aware Patch Application ──────────────────────────────────────────

pub fn apply_dll_patch(
    original_module: &RegisteredModule,
    table: &mut LocalJumpTable,
) -> Result<(), PatchingError> {
    // 1. Get the patch DLL handle
    let patch_lib_name = CString::new("patch_plugin.dll")
        .map_err(|_| PatchingError::LibraryNotLoaded("patch_plugin.dll".into()))?;
    let patch_hmod = unsafe { windows_ffi::GetModuleHandleA(patch_lib_name.as_ptr() as *const u8) };
    if patch_hmod.is_null() {
        return Err(PatchingError::LibraryNotLoaded("patch_plugin.dll".into()));
    }

    // 2. Compute original DLL's ASLR slide
    let old_offset = original_module.sentinel_address.wrapping_sub(table.sentinel_rva_original as usize);

    // 3. Compute patch DLL's ASLR slide from its sentinel
    let sentinel_cstr = CString::new("__subsecond_anchor")
        .map_err(|_| PatchingError::SentinelNotExported("__subsecond_anchor".into()))?;
    let patch_sentinel = unsafe { windows_ffi::GetProcAddress(patch_hmod, sentinel_cstr.as_ptr() as *const u8) };
    if patch_sentinel.is_null() {
        return Err(PatchingError::SentinelNotExported("__subsecond_anchor in patch DLL".into()));
    }
    let new_offset = patch_sentinel as usize - table.sentinel_rva_patch as usize;

    println!("[apply_dll_patch] old_offset = {:#x}, new_offset = {:#x}", old_offset, new_offset);

    // 4. Rebase: RVAs → runtime addresses
    let rebased_map: AddressMap = table.map.iter().map(|(old_rva, new_rva)| {
        let old_runtime = (*old_rva as usize).wrapping_add(old_offset) as u64;
        let new_runtime = (*new_rva as usize).wrapping_add(new_offset) as u64;
        (old_runtime, new_runtime)
    }).collect();

    println!("[apply_dll_patch] rebased {} entries", rebased_map.len());

    // 5. Commit
    table.map = rebased_map;
    commit_jump_table(table.clone());
    Ok(())
}

// ── Hot-Function Dispatch ─────────────────────────────────────────────────

static ORIGINAL_ADD_V1_ADDR: AtomicPtr<std::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());

pub fn init_hot_function(original_ptr: *const std::ffi::c_void) {
    ORIGINAL_ADD_V1_ADDR.store(original_ptr as *mut std::ffi::c_void, Ordering::Relaxed);
}

fn get_original_runtime_addr() -> u64 {
    ORIGINAL_ADD_V1_ADDR.load(Ordering::Relaxed) as u64
}

/// Call the currently-active version of the hot-reloadable add function.
pub unsafe fn call_hot_add(left: i32, right: i32) -> i32 {
    let original_addr = get_original_runtime_addr();
    if let Some(new_runtime) = lookup_in_jump_table(original_addr) {
        let func: extern "C" fn(i32, i32) -> i32 = unsafe { std::mem::transmute(new_runtime as *const ()) };
        return func(left, right);
    }
    let func: extern "C" fn(i32, i32) -> i32 = unsafe { std::mem::transmute(original_addr as *const ()) };
    func(left, right)
}
