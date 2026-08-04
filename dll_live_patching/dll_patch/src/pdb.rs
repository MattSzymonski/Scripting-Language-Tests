// pdb.rs — PDB-based symbol discovery (optional feature)
//
// Parses Windows .pdb files to enumerate Public symbols and build address
// maps. Mirrors Subsecond's HotpatchModuleCache::new() for Windows.

use crate::types::{AddressMap, JumpTable, PatchError};
use pdb::FallibleIterator;
use std::collections::HashMap;
use std::path::Path;

/// Parse a DLL's .pdb file and return all Public symbols as name → RVA.
pub fn collect_public_symbols(dll_path: &Path) -> Result<HashMap<String, u64>, PatchError> {
    let pdb_path = dll_path.with_extension("pdb");
    let pdb_file = std::fs::File::open(&pdb_path).map_err(|e| {
        PatchError::PdbError(format!("cannot open {}: {e}", pdb_path.display()))
    })?;
    let mut pdb = pdb::PDB::open(pdb_file).map_err(|e| PatchError::PdbError(format!("{e}")))?;
    let global_symbols = pdb
        .global_symbols()
        .map_err(|e| PatchError::PdbError(format!("{e}")))?;
    let address_map = pdb
        .address_map()
        .map_err(|e| PatchError::PdbError(format!("{e}")))?;
    let mut symbols = global_symbols.iter();
    let mut result = HashMap::new();

    while let Ok(Some(symbol)) = symbols.next() {
        if let Ok(pdb::SymbolData::Public(data)) = symbol.parse() {
            if let Some(rva) = data.offset.to_rva(&address_map) {
                result.insert(data.name.to_string().to_string(), rva.0 as u64);
            }
        }
    }
    Ok(result)
}

/// Build a JumpTable by matching symbol names between original and patch PDBs.
/// For every Public symbol present in BOTH, maps original_RVA → patch_RVA.
pub fn build_jump_table_from_pdbs(
    original_dll: &Path,
    patch_dll: &Path,
) -> Result<JumpTable, PatchError> {
    let original_symbols = collect_public_symbols(original_dll)?;
    let patch_symbols = collect_public_symbols(patch_dll)?;

    let mut map = AddressMap::default();
    for (name, original_rva) in &original_symbols {
        if let Some(patch_rva) = patch_symbols.get(name) {
            if original_rva != patch_rva {
                map.insert(*original_rva, *patch_rva);
            }
        }
    }

    let sentinel_original = original_symbols
        .get("__subsecond_anchor")
        .copied()
        .ok_or_else(|| PatchError::PdbError("sentinel not found in original PDB".into()))?;
    let sentinel_patch = patch_symbols
        .get("__subsecond_anchor")
        .copied()
        .ok_or_else(|| PatchError::PdbError("sentinel not found in patch PDB".into()))?;

    Ok(JumpTable {
        map,
        sentinel_rva_original: sentinel_original,
        sentinel_rva_patch: sentinel_patch,
    })
}
