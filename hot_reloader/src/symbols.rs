//! The hot-symbol model: which functions are hot-reloadable, how they are
//! keyed (by fully-qualified path) and how a source file maps to a scope.
//!
//! Also hosts the [`SourceCache`], the incremental per-file analysis cache
//! that keeps the change detector from re-reading and re-parsing unchanged
//! files on every save.

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use crate::analysis;

/// A single hot-reloadable function, with the module scope it lives in (so
/// the patch module can rebuild the same call-graph shape as stubs).
#[derive(Clone)]
pub struct HotSymbol {
    /// Bare function name (may collide across scopes - the key is the path).
    pub name: String,
    /// The parameter list text, e.g. `x: i32, y: i32`.
    pub params: String,
    /// The function body (inner content, braces excluded), trimmed.
    pub body: String,
    /// The return type text, empty when the function returns `()`.
    pub ret: String,
    /// Rust scope: "top" for lib.rs, "modules" for modules.rs,
    /// "modules::module_003" for a module file.
    pub scope: String,
    /// Position in the canonical symbol list == index into the dependency
    /// address table the host hands every patch DLL.
    pub index: usize,
}

/// Function names that are never hot regardless of the project.  This is
/// deliberately tiny: the plumbing exports themselves (`project_set_api` &
/// co.) are excluded via the [`crate::session::LiveCodingContract`] at
/// session build time, and project-specific infrastructure helpers (e.g. the
/// mini-engine's `state` / `spawn_default_quads`) belong in
/// [`crate::session::LiveCodeSessionConfig::infra_names`] instead.  The one
/// entry here, `resolve_hot_symbol`, is the registry helper the crate's
/// resolver pattern generates (kept defensively - it never appears in
/// scanned sources).
pub const DEFAULT_INFRA_NAMES: &[&str] = &["resolve_hot_symbol"];

/// Default source-file -> Rust-scope mapping (the mini-engine layout):
/// `lib.rs` is the crate root ("top"), `modules.rs` is "modules", and
/// `modules/module_*.rs` are "modules::module_*".  Return `None` for files
/// that are not part of the hot module graph.
pub fn default_file_scope(path: &Path) -> Option<String> {
    let file_name = path.file_name()?.to_str()?;
    match file_name {
        "lib.rs" => Some("top".to_string()),
        "modules.rs" => Some("modules".to_string()),
        name if name.starts_with("module_") && name.ends_with(".rs") => {
            let stem = &name[..name.len() - 3]; // strip ".rs"
            Some(format!("modules::{stem}"))
        }
        _ => None,
    }
}

/// Incremental per-file analysis cache.
///
/// Every entry is keyed by the content hash it was computed from, so a
/// changed file simply misses and only that file is re-read / re-parsed.
pub struct SourceCache {
    /// Raw file contents, keyed by absolute path.
    contents: HashMap<PathBuf, String>,
    /// Per-file hot-function extraction `(content_hash, tuples)`.
    extracted: HashMap<PathBuf, (u64, Vec<(String, String, String, String)>)>,
    /// Per-file hot-body stripping `(content_hash, root_names_hash, text)`.
    stripped: HashMap<PathBuf, (u64, u64, String)>,
}

/// A cheap content hash (not cryptographic - only used for equality checks).
fn content_hash(content: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content.hash(&mut hasher);
    hasher.finish()
}

/// A hash over the root-scope hot names (used to invalidate the strip cache
/// when the set of hot private helpers changes).
fn names_hash(names: &HashSet<String>) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for name in names.iter() {
        name.hash(&mut hasher);
    }
    hasher.finish()
}

impl SourceCache {
    /// An empty cache.
    pub fn new() -> Self {
        Self {
            contents: HashMap::new(),
            extracted: HashMap::new(),
            stripped: HashMap::new(),
        }
    }

    /// Re-read the changed `.rs` files and ensure every `.rs` file under
    /// `src_dir` is present in the cache, then return the current sources in
    /// canonical (path-sorted) order.
    ///
    /// Returns an error if a *changed* file cannot be read (a missing file is
    /// the user deleting it - ignore).
    pub fn refresh(
        &mut self,
        src_dir: &Path,
        changed_paths: &[PathBuf],
    ) -> crate::error::Result<Vec<(PathBuf, String)>> {
        for path in changed_paths {
            if path.extension().map_or(false, |ext| ext == "rs") {
                let content = std::fs::read_to_string(path).map_err(|source| {
                    crate::error::LiveCodeError::SourceIo {
                        path: path.clone(),
                        source,
                    }
                })?;
                self.contents.insert(path.clone(), content);
            }
        }

        // Ensure the whole tree is present (new files, first run, editor
        // renames).  Missing/unreadable files are left out of the cache.
        let mut stack = vec![src_dir.to_path_buf()];
        while let Some(directory) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&directory) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().map_or(false, |ext| ext == "rs") {
                    self.contents
                        .entry(path.clone())
                        .or_insert_with(|| std::fs::read_to_string(&path).unwrap_or_default());
                }
            }
        }

        let mut sources: Vec<(PathBuf, String)> = self
            .contents
            .iter()
            .map(|(path, content)| (path.clone(), content.clone()))
            .collect();
        sources.sort_by(|(path_a, _), (path_b, _)| path_a.cmp(path_b));
        Ok(sources)
    }

    /// The currently cached sources in canonical (path-sorted) order, without
    /// touching disk.
    pub fn sources(&self) -> Vec<(PathBuf, String)> {
        let mut sources: Vec<(PathBuf, String)> = self
            .contents
            .iter()
            .map(|(path, content)| (path.clone(), content.clone()))
            .collect();
        sources.sort_by(|(path_a, _), (path_b, _)| path_a.cmp(path_b));
        sources
    }

    /// Cached scope-aware extraction of a single file's hot functions.
    pub fn extracted(
        &mut self,
        path: &Path,
        content: &str,
        scope: &str,
    ) -> Vec<(String, String, String, String)> {
        let hash = content_hash(content);
        if let Some((cached_hash, tuples)) = self.extracted.get(path) {
            if *cached_hash == hash {
                return tuples.clone();
            }
        }
        let tuples = if scope == "top" {
            let mut list = analysis::extract_function_bodies(content);
            list.extend(analysis::extract_private_functions(content));
            list
        } else {
            analysis::extract_public_functions(content)
        };
        self.extracted
            .insert(path.to_path_buf(), (hash, tuples.clone()));
        tuples
    }

    /// Cached hot-body stripping of a single file.
    pub fn stripped(
        &mut self,
        path: &Path,
        content: &str,
        root_hot_names: &HashSet<String>,
    ) -> String {
        let content_hash = content_hash(content);
        let root_hash = names_hash(root_hot_names);
        if let Some((cached_content, cached_root, text)) = self.stripped.get(path) {
            if *cached_content == content_hash && *cached_root == root_hash {
                return text.clone();
            }
        }
        let text = analysis::strip_hot_bodies(content, root_hot_names);
        self.stripped
            .insert(path.to_path_buf(), (content_hash, root_hash, text.clone()));
        text
    }
}

impl Default for SourceCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Collect every hot-reloadable function in canonical order: the module graph
/// first (run_update/run_compute + each module file's tick/compute/sample/
/// scale/offset as plain `pub fn`), then lib.rs (the exported hot function
/// plus private helpers).  Plumbing and infrastructure (see `infra_names`)
/// are not hot.  Symbols are keyed by their fully-qualified path (see
/// [`symbol_path`]), so a lib helper and a module function sharing a bare
/// name (lib `scale_value_003` vs `modules::module_003::scale_value_003`) are
/// BOTH hot.
///
/// This is the incremental variant: per-file extraction results are reused
/// from `cache` whenever a file's content has not changed.
///
/// `file_scope` maps each source file to its Rust scope (`None` skips the
/// file); `infra_names` lists the never-hot function names.
pub fn collect_hot_symbols_cached(
    project_sources: &[(PathBuf, String)],
    infra_names: &HashSet<String>,
    file_scope: fn(&Path) -> Option<String>,
    cache: &mut SourceCache,
) -> Vec<HotSymbol> {
    // Process non-lib files first (module graph before lib.rs), stable order.
    let mut order: Vec<&(PathBuf, String)> = project_sources.iter().collect();
    order.sort_by_key(|(path, _)| {
        let is_lib = path.file_name().map_or(false, |name| name == "lib.rs");
        (is_lib, path.clone())
    });

    let mut symbols: Vec<HotSymbol> = Vec::new();
    for (path, content) in order {
        let Some(scope) = file_scope(path) else {
            continue;
        };
        let extracted = cache.extracted(path, content, &scope);
        for (name, params, body, ret) in extracted {
            // Skip infrastructure only.  No name-collision check: symbols are
            // keyed by qualified path, so duplicate bare names across scopes
            // (lib `scale_value_003` vs `modules::module_003::scale_value_003`)
            // are both hot.
            if infra_names.contains(&name) {
                continue;
            }
            let index = symbols.len();
            symbols.push(HotSymbol {
                name,
                params,
                body,
                ret,
                scope: scope.clone(),
                index,
            });
        }
    }
    symbols
}

/// Non-cached convenience wrapper (creates a throwaway cache).
pub fn collect_hot_symbols(
    project_sources: &[(PathBuf, String)],
    infra_names: &HashSet<String>,
    file_scope: fn(&Path) -> Option<String>,
) -> Vec<HotSymbol> {
    let mut cache = SourceCache::new();
    collect_hot_symbols_cached(project_sources, infra_names, file_scope, &mut cache)
}

/// The resolver key AND Rust path of a hot symbol: bare name for lib.rs
/// top-level functions, fully-qualified path otherwise.  Because keys are
/// qualified, a lib helper and a module function can share a bare name (lib
/// `scale_value_003` vs `modules::module_003::scale_value_003`) and both stay
/// hot - each is resolved by its distinct qualified key.
pub fn symbol_path(symbol: &HotSymbol) -> String {
    match symbol.scope.as_str() {
        "top" => symbol.name.clone(),
        "modules" => format!("modules::{}", symbol.name),
        _ => format!("{}::{}", symbol.scope, symbol.name),
    }
}

/// Split `x: i32, y: i32` into ([types], [names]).
pub fn parse_params(params: &str) -> (Vec<String>, Vec<String>) {
    let trimmed = params.trim();
    if trimmed.is_empty() {
        return (Vec::new(), Vec::new());
    }
    let mut types = Vec::new();
    let mut names = Vec::new();
    for part in trimmed.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some(colon) = part.find(':') {
            names.push(part[..colon].trim().to_string());
            types.push(part[colon + 1..].trim().to_string());
        }
    }
    (types, names)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sources(pairs: &[(&str, &str)]) -> Vec<(PathBuf, String)> {
        pairs
            .iter()
            .map(|(name, content)| (PathBuf::from(name), content.to_string()))
            .collect()
    }

    fn infra() -> HashSet<String> {
        DEFAULT_INFRA_NAMES
            .iter()
            .map(|s| s.to_string())
            .chain(
                [
                    "project_set_api",
                    "project_init",
                    "project_resolve_symbol",
                    "state",
                    "spawn_default_quads",
                    "verification_checksum",
                    "print_verification_checksum",
                ]
                .iter()
                .map(|s| s.to_string()),
            )
            .collect()
    }

    #[test]
    fn default_file_scope_maps_mini_engine_layout() {
        assert_eq!(
            default_file_scope(Path::new("lib.rs")).as_deref(),
            Some("top")
        );
        assert_eq!(
            default_file_scope(Path::new("modules.rs")).as_deref(),
            Some("modules")
        );
        assert_eq!(
            default_file_scope(Path::new("module_003.rs")).as_deref(),
            Some("modules::module_003")
        );
        assert_eq!(
            default_file_scope(Path::new("bloat_gen_no_export.rs")),
            None
        );
        assert_eq!(default_file_scope(Path::new("notes.txt")), None);
    }

    #[test]
    fn collect_symbols_skips_infrastructure_and_keeps_colliding_twins() {
        let project_sources = sources(&[
            (
                "lib.rs",
                "pub extern \"C\" fn project_set_api() {}\n\
                 fn get_aaa() -> i32 { 3 }\n\
                 fn scale_value_003() -> i32 { 4 }\n\
                 fn state() {}\n",
            ),
            ("modules.rs", "pub fn run_update() -> i32 { 1 }\n"),
            (
                "module_003.rs",
                "pub fn scale_value_003() -> i32 { 2 }\n\
                 pub fn tick_003() -> i32 { 1 }\n",
            ),
        ]);
        let symbols = collect_hot_symbols(&project_sources, &infra(), default_file_scope);
        let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
        // `state` and `project_set_api` are infra; the lib `scale_value_003`
        // collides with the module one but BOTH are hot (qualified keys).
        assert!(names.contains(&"get_aaa"));
        assert!(names.contains(&"run_update"));
        assert!(names.contains(&"tick_003"));
        assert!(!names.contains(&"state"));
        assert!(!names.contains(&"project_set_api"));
        assert_eq!(names.iter().filter(|n| **n == "scale_value_003").count(), 2);
    }

    #[test]
    fn symbol_path_is_qualified_per_scope() {
        let project_sources = sources(&[
            ("lib.rs", "fn get_aaa() -> i32 { 3 }\n"),
            ("module_003.rs", "pub fn scale_value_003() -> i32 { 2 }\n"),
        ]);
        let symbols = collect_hot_symbols(&project_sources, &infra(), default_file_scope);
        let paths: Vec<String> = symbols.iter().map(symbol_path).collect();
        assert!(paths.contains(&"get_aaa".to_string()));
        assert!(paths.contains(&"modules::module_003::scale_value_003".to_string()));
    }

    #[test]
    fn source_cache_reuses_unchanged_extraction() {
        let project_sources = sources(&[("lib.rs", "fn get_aaa() -> i32 { 3 }\n")]);
        let mut cache = SourceCache::new();
        let first =
            collect_hot_symbols_cached(&project_sources, &infra(), default_file_scope, &mut cache);
        let second =
            collect_hot_symbols_cached(&project_sources, &infra(), default_file_scope, &mut cache);
        assert_eq!(first.len(), second.len());
        assert_eq!(first[0].name, second[0].name);
        // Unchanged content must hit the cache (same result, no re-parse cost).
        assert_eq!(cache.extracted.len(), 1);
    }

    #[test]
    fn parse_params_splits_types_and_names() {
        let (types, names) = parse_params("x: i32, y: f32");
        assert_eq!(types, vec!["i32", "f32"]);
        assert_eq!(names, vec!["x", "y"]);
        let (types, names) = parse_params("");
        assert!(types.is_empty() && names.is_empty());
    }
}
