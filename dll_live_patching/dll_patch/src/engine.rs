// engine.rs — Jump table engine: commit, apply, and dispatch
//
// This is the core patching engine. It maintains a global AtomicPtr<JumpTable>
// and provides DLL-aware patch application with per-module ASLR sentinels.

use crate::registry::{compute_aslr_slide, get_module_handle};
use crate::types::{AddressMap, JumpTable, PatchError, RegisteredModule};
use std::ffi::CString;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::LazyLock;

// ── Global Jump Table ─────────────────────────────────────────────────────

/// The global jump table — an atomic pointer to a leaked JumpTable.
/// Call sites read this pointer to find current function implementations.
/// On each patch, a new table is built, leaked, and atomically swapped.
pub static APP_JUMP_TABLE: AtomicPtr<JumpTable> = AtomicPtr::new(std::ptr::null_mut());

/// Commit a jump table — leak it and atomically swap the global pointer.
pub fn commit_jump_table(table: JumpTable) {
    let old = APP_JUMP_TABLE.swap(Box::into_raw(Box::new(table)), Ordering::Relaxed);
    if !old.is_null() {
        // Previous table is intentionally leaked — its memory may still be
        // referenced by threads that loaded the old pointer before the swap.
        let _leaked = unsafe { Box::from_raw(old) };
    }
}

/// Look up a function in the jump table by its original runtime address.
/// Returns the replacement address if one exists, or None.
pub fn lookup(original_runtime_addr: u64) -> Option<u64> {
    let table_ptr = APP_JUMP_TABLE.load(Ordering::Relaxed);
    if table_ptr.is_null() {
        return None;
    }
    let table = unsafe { &*table_ptr };
    table.map.get(&original_runtime_addr).copied()
}

// ── DLL-Aware Patch Application ──────────────────────────────────────────

/// Apply a patch to a registered DLL.
///
/// 1. Locates the patch DLL (hardcoded to "patch_plugin.dll" for now — in
///    production this comes from the JumpTable or a configurable name).
/// 2. Computes ASLR slides for both the original DLL and the patch DLL
///    using their respective sentinels.
/// 3. Rebases all RVAs in the jump table to runtime addresses.
/// 4. Atomically commits the rebased table.
pub fn apply_dll_patch(
    original_module: &RegisteredModule,
    table: &mut JumpTable,
) -> Result<(), PatchError> {
    apply_dll_patch_named(original_module, table, "patch_plugin.dll")
}

/// Apply a patch from a named patch DLL.
pub fn apply_dll_patch_named(
    original_module: &RegisteredModule,
    table: &mut JumpTable,
    patch_lib_name: &str,
) -> Result<(), PatchError> {
    // 1. Get the patch DLL handle
    let patch_hmod = get_module_handle(patch_lib_name)?;

    // 2. Compute original DLL's ASLR slide from its sentinel
    let old_offset = compute_aslr_slide(original_module, table.sentinel_rva_original);

    // 3. Compute patch DLL's ASLR slide from its sentinel
    let sentinel_cstr = CString::new("__subsecond_anchor")
        .map_err(|_| PatchError::SentinelNotExported("__subsecond_anchor".into()))?;
    let patch_sentinel = unsafe {
        crate::registry::windows_ffi::GetProcAddress(
            patch_hmod,
            sentinel_cstr.as_ptr() as *const u8,
        )
    };
    if patch_sentinel.is_null() {
        return Err(PatchError::SentinelNotExported(
            "__subsecond_anchor in patch DLL".into(),
        ));
    }
    let new_offset = patch_sentinel as usize - table.sentinel_rva_patch as usize;

    // 4. Rebase: RVAs → runtime addresses
    let rebased_map: AddressMap = table
        .map
        .iter()
        .map(|(old_rva, new_rva)| {
            let old_runtime = (*old_rva as usize).wrapping_add(old_offset) as u64;
            let new_runtime = (*new_rva as usize).wrapping_add(new_offset) as u64;
            (old_runtime, new_runtime)
        })
        .collect();

    // 5. Commit
    table.map = rebased_map;
    commit_jump_table(table.clone());
    Ok(())
}

// ── Hot-Function Dispatch ─────────────────────────────────────────────────
//
// Each hot-reloadable function registers its original runtime address at init
// time. On each call, the dispatch looks up that address in the jump table
// and calls the replacement if found, or the original if not.

use std::collections::HashMap;
use std::sync::Mutex;

static FUNCTION_REGISTRY: LazyLock<Mutex<HashMap<u64, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Register a hot-reloadable function. `function_id` should be the link-time
/// RVA of the function in its original DLL. `original_ptr` is the runtime
/// address of the original implementation (used as fallback and as the lookup
/// key into the jump table).
pub fn init_function(function_id: u64, original_ptr: *const ()) {
    FUNCTION_REGISTRY
        .lock()
        .unwrap()
        .insert(function_id, original_ptr as usize);
}

/// Call a hot-reloadable function. Looks up `function_id` in the registry,
/// then checks the jump table for a replacement. If found, calls the
/// replacement; otherwise calls the original.
///
/// # Safety
/// The function signature must match the registered function. The caller
/// is responsible for passing correct argument types.
///
/// # Type Parameters
/// * `A` — argument tuple type, e.g. `(i32, i32)` for a two-arg function.
///   Use `()` for a zero-argument function.
/// * `R` — return type.
pub unsafe fn call_hot<A, R>(function_id: u64, args: A) -> R {
    let registry = FUNCTION_REGISTRY.lock().unwrap();
    let original_addr = registry.get(&function_id).copied().unwrap_or(0);
    drop(registry);

    if original_addr == 0 {
        // Unregistered function — undefined behavior if called.
        return unsafe { std::mem::zeroed() };
    }

    let target_addr = if let Some(new_addr) = lookup(original_addr as u64) {
        new_addr as *const ()
    } else {
        original_addr as *const ()
    };

    // transmute_copy: copies the pointer bits into a function pointer of the
    // desired signature. Safe because *const () and fn(A) -> R are both
    // pointer-sized on all supported platforms.
    let func: extern "C" fn(A) -> R = unsafe { std::mem::transmute_copy(&target_addr) };
    func(args)
}
