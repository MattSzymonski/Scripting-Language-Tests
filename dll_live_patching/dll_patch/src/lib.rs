// dll_patch/src/lib.rs — Reusable DLL live-patching framework
//
// Provides jump-table-based function redirection for dynamically loaded
// libraries (DLLs) on Windows. Supports cross-module patching with per-DLL
// ASLR sentinel resolution, PDB-based symbol discovery, and live rebuild.
//
// # Quick Start
// ```rust,ignore
// use dll_patch::{register_library, init_function, call_hot, apply_dll_patch};
// use dll_patch::types::{JumpTable, AddressMap};
//
// // 1. Load the DLL and register it
// let lib = unsafe { libloading::Library::new("plugin.dll")? };
// let sentinel: libloading::Symbol<unsafe extern "C" fn()> =
//     unsafe { lib.get(b"__subsecond_anchor")? };
// let module = unsafe { register_library("plugin.dll", *sentinel as *const _)? };
//
// // 2. Register hot functions
// let add_v1: libloading::Symbol<unsafe extern "C" fn(i32, i32) -> i32> =
//     unsafe { lib.get(b"add_v1")? };
// let add_v1_ptr = *add_v1 as *const ();
// let add_v1_rva = (*add_v1 as *const () as usize) - module.base_address;
// init_function(add_v1_rva as u64, add_v1_ptr);
//
// // 3. Call (goes to original)
// let result = unsafe { call_hot::<(i32, i32), i32>(add_v1_rva as u64, (2, 3)) };
//
// // 4. Apply a patch
// let mut table = JumpTable {
//     map: AddressMap::from([(add_v1_rva as u64, add_v2_rva)]),
//     sentinel_rva_original,
//     sentinel_rva_patch,
// };
// apply_dll_patch(&module, &mut table)?;
//
// // 5. Call again (now goes to patched version)
// let result = unsafe { call_hot::<(i32, i32), i32>(add_v1_rva as u64, (2, 3)) };
// ```

mod types;
mod registry;
mod engine;

#[cfg(feature = "pdb")]
pub mod pdb;

#[cfg(feature = "rebuild")]
pub mod rebuild;

// Re-export core types
pub use types::{AddressMap, JumpTable, PatchError, RegisteredModule};

// Re-export registry functions
pub use registry::{
    compute_aslr_slide, get_module_handle, register_library, register_library_by_name, resolve_rva,
};

// Re-export engine functions
pub use engine::{
    apply_dll_patch, apply_dll_patch_named, call_hot, commit_jump_table, init_function, lookup,
};
