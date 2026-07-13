//! Loads `aaa_scenario/manifest.json`, written by `generate_aaa_modules.py`.
//!
//! The manifest is the single source of truth for the module graph: which
//! tier each module belongs to, its declared dependencies (real PE imports —
//! see that module's generated `build.rs`), and a fixed "boot order" (a
//! shuffled, dependency-oblivious module list, matching how a real engine's
//! module manager iterates modules roughly by registration order, not by
//! topologically sorting the dependency graph first).

use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Mirrors the JSON fields `generate_aaa_modules.py` writes per module.
/// Deliberately does NOT deserialize every field the generator writes (e.g.
/// `payload_bucket`, `ctor_iterations`, `identity`) — those exist in
/// `manifest.json` for humans/external tooling inspecting the generated
/// graph, but this harness gets a module's actual identity and cost from the
/// DLL itself at load time (`create_module_instance`/`module_startup`),
/// which is the authoritative, load-bearing source. `serde` ignores JSON
/// keys with no matching field, so this stays forward-compatible with a
/// richer manifest.
#[derive(Debug, Deserialize)]
pub struct ModuleEntry {
    pub tier: String,
    pub dll: String,
    pub deps: Vec<String>,
    pub hot_reloadable: bool,
    pub payload_bytes: u64,
    pub export_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct Counts {
    pub core: usize,
    pub subsystem: usize,
    pub leaf: usize,
    pub total: usize,
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub counts: Counts,
    /// Relative to the manifest's own directory (i.e. `aaa_scenario/`).
    pub target_release_dir: String,
    pub boot_order: Vec<String>,
    pub modules: HashMap<String, ModuleEntry>,
}

impl Manifest {
    pub fn load(manifest_path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(manifest_path)
            .map_err(|e| format!("failed to read {}: {}", manifest_path.display(), e))?;
        serde_json::from_str(&text).map_err(|e| format!("failed to parse manifest: {}", e))
    }

    /// Directory containing the built `.dll` / `.dll.lib` files for every module.
    pub fn release_dir(&self, manifest_path: &Path) -> PathBuf {
        manifest_path
            .parent()
            .unwrap_or(Path::new("."))
            .join(&self.target_release_dir)
    }

    /// Full transitive dependency closure of `name` (BFS over `deps`), used to
    /// detect which shared modules a given load pulls in as a side effect.
    pub fn transitive_deps(&self, name: &str) -> Vec<String> {
        let mut seen = HashSet::new();
        let mut queue: Vec<String> = self
            .modules
            .get(name)
            .map(|m| m.deps.clone())
            .unwrap_or_default();
        let mut out = Vec::new();
        while let Some(dep) = queue.pop() {
            if !seen.insert(dep.clone()) {
                continue;
            }
            out.push(dep.clone());
            if let Some(entry) = self.modules.get(&dep) {
                queue.extend(entry.deps.iter().cloned());
            }
        }
        out
    }
}
