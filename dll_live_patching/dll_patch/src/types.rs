// types.rs — Core data structures for the DLL live-patching framework

use std::collections::HashMap;

/// Maps old function addresses (runtime values after rebasing, or RVAs before)
/// to new function addresses. The meaning of keys/values depends on context:
/// - Before rebasing: RVAs relative to their respective module bases.
/// - After rebasing (in APP_JUMP_TABLE): runtime virtual addresses.
pub type AddressMap = HashMap<u64, u64>;

/// A jump table describing a patch — the data structure shipped from the
/// build system to the runtime. Maps original function addresses to their
/// replacement implementations.
#[derive(Debug, Clone)]
pub struct JumpTable {
    /// Old → new address mappings (RVAs before rebasing, runtime after).
    pub map: AddressMap,
    /// Link-time RVA of the sentinel in the original binary.
    pub sentinel_rva_original: u64,
    /// Link-time RVA of the sentinel in the patch binary.
    pub sentinel_rva_patch: u64,
}

/// Information about a loaded module, stored in the module registry.
#[derive(Clone, Debug)]
pub struct RegisteredModule {
    /// Filesystem name, e.g. "plugin.dll".
    pub name: String,
    /// Base address (HMODULE on Windows).
    pub base_address: usize,
    /// Runtime address of the sentinel function (post-ASLR).
    pub sentinel_address: usize,
    /// Total size of the module image in bytes.
    pub image_size: usize,
}

/// Errors that can occur during library registration or patching.
#[derive(Debug)]
pub enum PatchError {
    /// The named library is not loaded in the current process.
    LibraryNotLoaded(String),
    /// The sentinel address does not fall within the library's image bounds.
    SentinelNotInLibrary {
        sentinel: usize,
        name: String,
    },
    /// The library has already been registered.
    AlreadyRegistered(String),
    /// Failed to query module information (GetModuleInformation).
    ModuleInfoFailed(String),
    /// The requested symbol was not found in the library.
    SymbolNotFound(String),
    /// The sentinel symbol is not exported from the library.
    SentinelNotExported(String),
    /// A PDB-related error occurred.
    PdbError(String),
}

impl std::fmt::Display for PatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LibraryNotLoaded(n) => write!(f, "library '{n}' is not loaded"),
            Self::SentinelNotInLibrary { sentinel, name } => {
                write!(f, "sentinel {sentinel:#x} not in library '{name}'")
            }
            Self::AlreadyRegistered(n) => write!(f, "library '{n}' already registered"),
            Self::ModuleInfoFailed(n) => write!(f, "failed to query module info for '{n}'"),
            Self::SymbolNotFound(n) => write!(f, "symbol '{n}' not found"),
            Self::SentinelNotExported(n) => write!(
                f,
                "sentinel '{n}' not exported — add #[unsafe(no_mangle)] pub extern \"C\" fn {n}() {{}}"
            ),
            Self::PdbError(msg) => write!(f, "PDB error: {msg}"),
        }
    }
}

impl std::error::Error for PatchError {}
