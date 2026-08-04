// types.rs — Shared data structures for the DLL live-patching system

use std::collections::HashMap;

/// Maps old function addresses to new function addresses.
/// Keys and values are RVAs (relative to their respective module bases)
/// until rebased by apply_dll_patch().
pub type AddressMap = HashMap<u64, u64>;

/// A jump table — the data structure that describes a patch.
/// Mirrors Subsecond's `subsecond_types::JumpTable` but with DLL-aware sentinels.
#[derive(Debug, Clone)] #[allow(dead_code)]
pub struct LocalJumpTable {
    pub map: AddressMap,
    pub sentinel_rva_original: u64,
    pub sentinel_rva_patch: u64,
}

/// Information about a loaded module, stored at registration time.
#[derive(Clone, Debug)] #[allow(dead_code)]
pub struct RegisteredModule {
    pub name: String,
    pub base_address: usize,
    pub sentinel_address: usize,
    pub image_size: usize,
}

/// Errors that can occur during library registration or patching.
#[derive(Debug)]
pub enum PatchingError {
    LibraryNotLoaded(String),
    SentinelNotInLibrary { sentinel: usize, name: String },
    AlreadyRegistered(String),
    ModuleInfoFailed(String),
    SymbolNotFound(String),
    SentinelNotExported(String),
    #[allow(dead_code)]
    PdbError(String),
}

impl std::fmt::Display for PatchingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LibraryNotLoaded(n) => write!(f, "library '{n}' is not loaded"),
            Self::SentinelNotInLibrary { sentinel, name } =>
                write!(f, "sentinel {sentinel:#x} not in library '{name}'"),
            Self::AlreadyRegistered(n) => write!(f, "library '{n}' already registered"),
            Self::ModuleInfoFailed(n) => write!(f, "failed to query module info for '{n}'"),
            Self::SymbolNotFound(n) => write!(f, "symbol '{n}' not found"),
            Self::SentinelNotExported(n) =>
                write!(f, "sentinel '{n}' not exported — add #[unsafe(no_mangle)] pub extern \"C\" fn {n}() {{}}"),
            Self::PdbError(msg) => write!(f, "PDB error: {msg}"),
        }
    }
}
