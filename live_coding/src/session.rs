//! The live-coding session: owns the loaded project libraries, caches,
//! change detection, per-function prologue patching and the full-rebuild
//! fallback.
//!
//! The session is engine-agnostic.  The consumer:
//!
//! 1. builds a [`LiveCodeSessionConfig`] (paths, build command, the
//!    exported-symbol contract, the API-table pointer handed to the DLLs,
//!    the file→scope mapping),
//! 2. calls [`LiveCodeSession::new`] (validates the config), then
//!    [`LiveCodeSession::load_initial`] and points its engine at the
//!    returned update address,
//! 3. watches the project directory (any file watcher - or the built-in
//!    [`LiveCodeSession::watch`] under the `watch` feature) and calls
//!    [`LiveCodeSession::handle_change`] with the changed paths; on
//!    [`ChangeOutcome::FullReloadNeedsPointerSwap`] it must repoint the
//!    engine, everything else leaves the engine pointer untouched.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;

use libloading::{Library, Symbol};

use crate::error::{LiveCodeError, Result};
use crate::style::{
    hot_prefix, paint_bright_cyan, paint_bright_green, paint_bright_red, paint_bright_yellow,
    paint_cyan, paint_dim, paint_green, paint_magenta, paint_red, paint_yellow, rule,
};
use crate::symbols::{symbol_path, HotSymbol, SourceCache};
use crate::LogLevel;

/// Resolve a hot function's address in a loaded DLL by key, through the
/// DLL's single exported resolver entry point (returns 0 for unknown names,
/// which we treat as `None`).
fn resolve_symbol_address(library: &Library, resolver_name: &str, name: &str) -> Option<usize> {
    let resolver: Symbol<extern "C" fn(*const std::os::raw::c_char) -> usize> =
        unsafe { library.get(resolver_name.as_bytes()).ok()? };
    let c_name = std::ffi::CString::new(name).ok()?;
    // Calling a function pointer is a safe operation; only obtaining it from
    // the DLL is unsafe.
    let address = resolver(c_name.as_ptr());
    if address == 0 {
        None
    } else {
        Some(address)
    }
}

/// Monotonic counter used to give every loaded copy of the DLL a unique name.
static LIB_VERSION: AtomicU32 = AtomicU32::new(1);

/// Copy a freshly built DLL to a unique versioned name so the OS loader (and
/// Windows' file lock on the previous copy) never conflates versions.
fn versioned_lib_path(lib_dir: &Path, stem: &str, extension: &str) -> PathBuf {
    let version = LIB_VERSION.fetch_add(1, Ordering::Relaxed);
    lib_dir.join(format!("{stem}_v{version}.{extension}"))
}

/// The exported entry-point names the session expects on the project DLLs
/// (base, full reloads and patch modules).  Defaults match the mini-engine
/// layout; a consumer with different symbol names should override them.
#[derive(Clone, Debug)]
pub struct LiveCodingContract {
    /// Receives the API-table pointer (`project_set_api`).
    pub set_api: String,
    /// Runs once per load (`project_init`).
    pub init: String,
    /// Resolves a hot symbol's address by qualified path
    /// (`project_resolve_symbol`).
    pub resolve_symbol: String,
    /// Receives the base dependency-address table on patch DLLs
    /// (`project_set_dependencies`).
    pub set_dependencies: String,
    /// The engine's per-frame entry point (`project_update`).
    pub update: String,
}

impl Default for LiveCodingContract {
    fn default() -> Self {
        Self {
            set_api: "project_set_api".to_string(),
            init: "project_init".to_string(),
            resolve_symbol: "project_resolve_symbol".to_string(),
            set_dependencies: "project_set_dependencies".to_string(),
            update: "project_update".to_string(),
        }
    }
}

/// Everything the session needs to know about the consumer's project.
///
/// Note: not `Clone`/`Debug` because it carries an optional logger callback.
pub struct LiveCodeSessionConfig {
    /// Workspace root: `cargo`'s working directory for project builds.
    pub workspace_dir: PathBuf,
    /// The project crate directory (watched by the consumer / `watch`).
    pub project_dir: PathBuf,
    /// The directory containing the project's `.rs` sources (e.g.
    /// `<project>/src`), read by the change detector.
    pub project_src_dir: PathBuf,
    /// `cargo` arguments that build the base DLL (run in `workspace_dir`).
    pub build_args: Vec<String>,
    /// Full path of the freshly built DLL.
    pub dll_path: PathBuf,
    /// The exported entry-point names (see [`LiveCodingContract`]).
    pub contract: LiveCodingContract,
    /// The API-table pointer handed to the DLLs' `set_api`.
    pub api_pointer: *const (),
    /// Maps each source file to its Rust scope (`None` skips the file).
    pub file_scope: fn(&Path) -> Option<String>,
    /// Function names that are never hot (plumbing / infrastructure).
    pub infra_names: Vec<String>,
    /// Rust edition used for the standalone `rustc` leaf compiles.
    pub rustc_edition: String,
    /// Shared-library suffix for leaf compiles (`dll` / `so` / `dylib`).
    pub artifact_extension: String,
    /// How many versioned DLL copies to keep on disk (older ones are pruned
    /// best-effort; loaded DLLs are locked by Windows and retained in memory
    /// forever for in-flight-call safety, so pruning helps across runs).
    pub keep_versioned_libraries: usize,
    /// Minimum log verbosity shown on stdout (see [`LogLevel`]).
    pub log_level: LogLevel,
    /// Optional logger callback.  When set, ALL output goes through it
    /// instead of stdout/stderr.
    pub logger: Option<Box<dyn Fn(LogLevel, String) + Send + Sync>>,
}

impl LiveCodeSessionConfig {
    /// Convenience for the standard mini-engine layout:
    ///
    /// - watched project at `<workspace>/project`,
    /// - sources at `<workspace>/project/src`,
    /// - build command `cargo build -p project --target-dir target_reload`,
    /// - DLL at `<workspace>/target_reload/debug/project.dll`,
    /// - the default [`LiveCodingContract`],
    /// - the mini-engine file→scope mapping and infrastructure blocklist.
    pub fn mini_engine(workspace: &Path, api_pointer: *const ()) -> Self {
        let workspace = workspace.to_path_buf();
        Self {
            project_dir: workspace.join("project"),
            project_src_dir: workspace.join("project").join("src"),
            build_args: vec![
                "build".to_string(),
                "-p".to_string(),
                "project".to_string(),
                "--target-dir".to_string(),
                "target_reload".to_string(),
            ],
            dll_path: workspace
                .join("target_reload")
                .join("debug")
                .join("project.dll"),
            contract: LiveCodingContract::default(),
            api_pointer,
            file_scope: crate::symbols::default_file_scope,
            // The mini-engine's project-specific infrastructure blocklist:
            // plumbing exports, the generated registry helper, the state
            // accessor and the verification tooling.  (The plumbing names are
            // also unioned from the contract in `LiveCodeSession::new`.)
            infra_names: [
                "project_set_api",
                "project_init",
                "project_resolve_symbol",
                "project_set_dependencies",
                "resolve_hot_symbol",
                "state",
                "spawn_default_quads",
                "verification_checksum",
                "print_verification_checksum",
            ]
            .iter()
            .map(|name| name.to_string())
            .collect(),
            rustc_edition: "2021".to_string(),
            artifact_extension: "dll".to_string(),
            keep_versioned_libraries: 8,
            log_level: LogLevel::Normal,
            logger: None,
            workspace_dir: workspace,
        }
    }

    /// Validate the configuration, returning [`LiveCodeError::InvalidConfig`]
    /// on the first problem found.  Called by [`LiveCodeSession::new`].
    pub fn validate(&self) -> Result<()> {
        if self.build_args.is_empty() {
            return Err(LiveCodeError::InvalidConfig(
                "build_args must not be empty".to_string(),
            ));
        }
        if !self.workspace_dir.exists() {
            return Err(LiveCodeError::InvalidConfig(format!(
                "workspace_dir does not exist: {}",
                self.workspace_dir.display()
            )));
        }
        if !self.project_src_dir.exists() {
            return Err(LiveCodeError::InvalidConfig(format!(
                "project_src_dir does not exist: {}",
                self.project_src_dir.display()
            )));
        }
        if self.dll_path.file_name().is_none() {
            return Err(LiveCodeError::InvalidConfig(format!(
                "dll_path has no file name: {}",
                self.dll_path.display()
            )));
        }
        if self.artifact_extension.is_empty() {
            return Err(LiveCodeError::InvalidConfig(
                "artifact_extension must not be empty".to_string(),
            ));
        }
        Ok(())
    }
}

/// What a change event did, so the consumer knows whether its engine pointer
/// must change (it normally does not - the base prologue is re-patched).
#[derive(Debug)]
pub enum ChangeOutcome {
    /// One or more hot functions were recompiled standalone and their
    /// prologues patched in place - base DLL untouched, pointer unchanged.
    Patched {
        /// Bare names of the patched functions.
        functions: Vec<String>,
    },
    /// A full rebuild was loaded alongside and the base prologue re-patched;
    /// the engine pointer stays valid.
    FullReload {
        /// The fresh project update address (for logging).
        update_address: usize,
    },
    /// A full rebuild was loaded but the base prologue could not be
    /// re-patched, so the consumer MUST point its engine at `update_address`.
    FullReloadNeedsPointerSwap {
        /// The fresh project update address to install.
        update_address: usize,
    },
    /// The save produced identical content - nothing to do.
    Nothing,
}

/// Owns the loaded project and applies live-coding changes.
pub struct LiveCodeSession {
    config: LiveCodeSessionConfig,
    /// Never-hot function names (plumbing / infrastructure).
    infra_names: HashSet<String>,
    /// Incremental per-file analysis cache (contents + extraction + strip).
    source_cache: SourceCache,
    /// Address of the update entry point in the very first base DLL - the
    /// engine pointer target.  Its prologue is what every full reload
    /// re-patches.
    base_update_address: Option<usize>,
    /// Canonical hot-symbol qualified paths (order == dependency-table
    /// order), from the most recently fully-built DLL.
    hot_keys: Vec<String>,
    /// Base-DLL addresses of the hot symbols (same order).  These are the
    /// prologue-patch targets AND the dependency table handed to every patch
    /// DLL, so a patched function calls back into the base (single source of
    /// truth).
    base_symbols: Vec<usize>,
    /// How many change events have been processed (for logging).
    change_count: u64,
    /// Cache of the last applied function bodies keyed by qualified path:
    /// (key, params, body, ret).
    cached_funcs: Vec<(String, String, String, String)>,
    /// All source with hot bodies stripped, to detect non-body changes.
    stripped_source: String,
    /// Every loaded library (base copy, full builds, patch DLLs), retained so
    /// an in-flight call never hits unmapped memory.
    retained_libraries: Vec<Library>,
}

impl LiveCodeSession {
    /// Create a session, validating the configuration.
    pub fn new(config: LiveCodeSessionConfig) -> Result<Self> {
        config.validate()?;
        // The contract's plumbing exports are never hot - union them with the
        // config's project-specific infrastructure names (state accessor,
        // verification helpers, ...).  The update entry point is deliberately
        // NOT added: it is a hot function.
        let mut infra_names: HashSet<String> = config.infra_names.iter().cloned().collect();
        infra_names.insert(config.contract.set_api.clone());
        infra_names.insert(config.contract.init.clone());
        infra_names.insert(config.contract.resolve_symbol.clone());
        infra_names.insert(config.contract.set_dependencies.clone());
        Ok(Self {
            config,
            infra_names,
            source_cache: SourceCache::new(),
            base_update_address: None,
            hot_keys: Vec::new(),
            base_symbols: Vec::new(),
            change_count: 0,
            cached_funcs: Vec::new(),
            stripped_source: String::new(),
            retained_libraries: Vec::new(),
        })
    }

    /// The configuration this session was created with.
    pub fn config(&self) -> &LiveCodeSessionConfig {
        &self.config
    }

    /// Emit one log line: through the configured logger callback if set,
    /// otherwise to stdout (or stderr for errors), filtered by `log_level`.
    fn emit(&self, level: LogLevel, message: &str) {
        if let Some(callback) = &self.config.logger {
            (callback)(level, message.to_string());
        } else if level <= self.config.log_level {
            if level == LogLevel::Error {
                eprint!("{message}");
            } else {
                print!("{message}");
            }
        }
    }

    fn is_cached(&self, key: &str, body: &str) -> bool {
        self.cached_funcs
            .iter()
            .any(|(cached_key, _, cached_body, _)| cached_key == key && cached_body == body)
    }

    fn upsert_cached(&mut self, entry: (String, String, String, String)) {
        if let Some(slot) = self
            .cached_funcs
            .iter_mut()
            .find(|cached| cached.0 == entry.0)
        {
            *slot = entry;
        } else {
            self.cached_funcs.push(entry);
        }
    }

    // -- build + load ------------------------------------------------------

    /// Run the consumer's cargo build command and return the DLL path.
    fn build_project(&self) -> Result<PathBuf> {
        let output = Command::new("cargo")
            .args(&self.config.build_args)
            .current_dir(&self.config.workspace_dir)
            .output()
            .map_err(|source| LiveCodeError::SourceIo {
                path: self.config.workspace_dir.clone(),
                source,
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(LiveCodeError::BuildFailed {
                stderr: format!("{stdout}{stderr}").trim().to_string(),
            });
        }

        Ok(self.config.dll_path.clone())
    }

    /// Best-effort prune of old versioned DLL copies, keeping the newest
    /// `keep_versioned_libraries`.  Loaded DLLs are locked by Windows, so
    /// files from the CURRENT process can't be removed - this mainly cleans
    /// up copies from previous runs.
    fn prune_old_versioned_libraries(&self) {
        let keep = self.config.keep_versioned_libraries.max(1);
        let Some(parent) = self.config.dll_path.parent() else {
            return;
        };
        let stem = self
            .config
            .dll_path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        let prefix = format!("{stem}_v");
        let suffix = format!(".{}", self.config.artifact_extension);

        let Ok(entries) = std::fs::read_dir(parent) else {
            return;
        };
        let mut versions: Vec<(u64, PathBuf)> = entries
            .flatten()
            .filter_map(|entry| {
                let path = entry.path();
                let name = path.file_name()?.to_str()?;
                if !name.starts_with(&prefix) || !name.ends_with(&suffix) {
                    return None;
                }
                let middle = &name[prefix.len()..name.len() - suffix.len()];
                let number: u64 = middle.parse().ok()?;
                Some((number, path))
            })
            .collect();
        versions.sort_by_key(|(number, _)| *number);
        while versions.len() > keep {
            let (_, oldest) = versions.remove(0);
            // Best-effort: ignore failures (the file may be locked).
            let _ = std::fs::remove_file(oldest);
        }
    }

    /// Build the whole project crate, load a fresh copy, hand it the API
    /// table, run its init, and return (library, update address).
    fn build_and_load_full(&self) -> Result<(Library, usize)> {
        let lib_path = self.build_project()?;
        self.prune_old_versioned_libraries();
        let stem = lib_path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| {
                lib_path
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "lib".to_string())
            });
        let versioned = versioned_lib_path(
            lib_path.parent().unwrap(),
            &stem,
            &self.config.artifact_extension,
        );
        std::fs::copy(&lib_path, &versioned).map_err(|source| LiveCodeError::CopyFailed {
            from: lib_path.clone(),
            to: versioned.clone(),
            source,
        })?;

        // SAFETY: Library::new maps the freshly copied DLL; the returned
        // handle is the only way to reach its symbols afterwards.
        let library =
            unsafe { Library::new(&versioned) }.map_err(|message| LiveCodeError::LoadFailed {
                path: versioned.clone(),
                message: message.to_string(),
            })?;

        // Hand the API table to the DLL, run its init, and copy out the
        // update address.  The Symbols borrow `library`, so they must be
        // dropped before `library` is moved into the result.
        // SAFETY: the exported functions are plain extern "C" entry points
        // with the expected signatures; the API pointer is a thin pointer the
        // DLL casts to its own mirror type.
        let update_address: usize = unsafe {
            let set_api: Symbol<extern "C" fn(*const ())> = library
                .get(self.config.contract.set_api.as_bytes())
                .map_err(|_| LiveCodeError::SymbolMissing {
                    name: self.config.contract.set_api.clone(),
                })?;
            set_api(self.config.api_pointer);

            let init: Symbol<extern "C" fn()> = library
                .get(self.config.contract.init.as_bytes())
                .map_err(|_| LiveCodeError::SymbolMissing {
                name: self.config.contract.init.clone(),
            })?;
            init();

            let update: Symbol<crate::ProjectUpdateFn> = library
                .get(self.config.contract.update.as_bytes())
                .map_err(|_| LiveCodeError::SymbolMissing {
                    name: self.config.contract.update.clone(),
                })?;
            *update as usize
        };

        Ok((library, update_address))
    }

    /// Initial build + load; returns the update address the consumer should
    /// point its engine at.
    pub fn load_initial(&mut self) -> Result<usize> {
        let dll_name = self
            .config
            .dll_path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "project.dll".to_string());
        self.emit(
            LogLevel::Normal,
            &format!(
                "\n  {}\n",
                paint_bright_cyan(&format!("▸ Building {dll_name} (cold)..."))
            ),
        );
        let build_start = Instant::now();
        let (library, update_address) = self.build_and_load_full()?;

        // Resolve every hot symbol's address in this freshly loaded DLL -
        // these are the prologue-patch targets AND the dependency table
        // handed to future patch DLLs.
        let project_sources = self
            .source_cache
            .refresh(&self.config.project_src_dir, &[])?;
        let hot_symbols = crate::symbols::collect_hot_symbols_cached(
            &project_sources,
            &self.infra_names,
            self.config.file_scope,
            &mut self.source_cache,
        );
        self.hot_keys = hot_symbols
            .iter()
            .map(|symbol| symbol_path(symbol))
            .collect();
        self.base_symbols = hot_symbols
            .iter()
            .map(|symbol| {
                resolve_symbol_address(
                    &library,
                    &self.config.contract.resolve_symbol,
                    &symbol_path(symbol),
                )
                .unwrap_or(0)
            })
            .collect();

        self.base_update_address = Some(update_address);
        self.retained_libraries.push(library);

        // Seed the change cache from the current sources so the first edit is
        // diffed against the initial state.  The combined source covers
        // lib.rs + bloat + the module graph; we cache both the hot symbols
        // and the plumbing exports.
        let lib_content = project_sources
            .iter()
            .find(|(path, _)| path.file_name().map_or(false, |name| name == "lib.rs"))
            .map(|(_, content)| content.clone())
            .unwrap_or_default();
        let root_hot_names: HashSet<String> = hot_symbols
            .iter()
            .filter(|symbol| symbol.scope == "top")
            .map(|symbol| symbol.name.clone())
            .collect();
        let stripped = project_sources
            .iter()
            .map(|(path, content)| self.source_cache.stripped(path, content, &root_hot_names))
            .collect::<Vec<_>>()
            .join("\n");
        self.stripped_source = stripped;
        for entry in &crate::analysis::extract_function_bodies(&lib_content) {
            self.upsert_cached(entry.clone());
        }
        for symbol in &hot_symbols {
            self.upsert_cached((
                symbol_path(symbol),
                symbol.params.clone(),
                symbol.body.clone(),
                symbol.ret.clone(),
            ));
        }
        self.emit(
            LogLevel::Normal,
            &format!(
                "\n{} {}\n",
                hot_prefix(),
                paint_green(&format!(
                    "cold build + load in {} - {} source file(s), {} hot symbol(s) cached",
                    paint_bright_cyan(&format!("{:?}", build_start.elapsed())),
                    paint_bright_cyan(&project_sources.len().to_string()),
                    paint_bright_cyan(&hot_symbols.len().to_string())
                ))
            ),
        );
        self.emit(
            LogLevel::Normal,
            &format!(
                "{} {}\n",
                hot_prefix(),
                paint_bright_green(&format!(
                    "base {dll_name} loaded - jumping quads are live ({} @ {})",
                    self.config.contract.update,
                    paint_cyan(&format!("{update_address:#x}"))
                ))
            ),
        );

        Ok(update_address)
    }

    // -- per-function patch ------------------------------------------------

    /// Compile one changed hot function standalone and patch its prologue in
    /// the loaded base DLL - true Live Coding, for leaves AND functions in
    /// the middle of the call graph.
    fn try_patch_function(
        &mut self,
        lib_source: &str,
        project_sources: &[(PathBuf, String)],
        changed: &HotSymbol,
    ) -> Result<()> {
        let consts = crate::analysis::extract_top_level_consts(lib_source);
        self.emit(
            LogLevel::Normal,
            &format!(
                "    {}\n",
                paint_magenta("── per-function patch (true Live Coding) ──")
            ),
        );
        self.emit(
            LogLevel::Normal,
            &format!(
                "    injecting {} project const(s) into the self-contained module\n",
                paint_bright_cyan(&consts.len().to_string())
            ),
        );
        self.emit(
            LogLevel::Normal,
            &format!(
                "    patching {}   params=({})  body={} chars\n",
                paint_bright_cyan(&changed.name),
                paint_dim(&changed.params),
                paint_cyan(&changed.body.len().to_string())
            ),
        );

        // Rebuild the canonical symbol list from the current source so the
        // patch module's dependency-table indices match the host's table.
        let symbols = crate::symbols::collect_hot_symbols_cached(
            project_sources,
            &self.infra_names,
            self.config.file_scope,
            &mut self.source_cache,
        );
        let module_names = crate::module::PatchModuleNames {
            resolver: &self.config.contract.resolve_symbol,
            set_api: &self.config.contract.set_api,
            set_dependencies: &self.config.contract.set_dependencies,
        };
        let module_source = crate::module::build_patch_module_source(
            lib_source,
            &symbols,
            &symbol_path(changed),
            &module_names,
        );
        let compile_start = Instant::now();
        self.emit(
            LogLevel::Normal,
            &format!("    rustc compiling standalone module...\n"),
        );
        let lib_path = crate::patch::compile_rust_source(
            &module_source,
            "project_patch",
            &self.config.rustc_edition,
            &self.config.artifact_extension,
        )?;
        self.emit(
            LogLevel::Normal,
            &format!(
                "    rustc OK in {} -> {}\n",
                paint_green(&format!("{:?}", compile_start.elapsed())),
                paint_cyan(&lib_path.display().to_string())
            ),
        );

        // SAFETY: Library::new maps the freshly compiled patch DLL.
        let library =
            unsafe { Library::new(&lib_path) }.map_err(|message| LiveCodeError::LoadFailed {
                path: lib_path.clone(),
                message: message.to_string(),
            })?;

        // SAFETY: the patch module exports plain extern "C" entry points with
        // the expected signatures; the API pointer and dependency table are
        // thin pointers the module casts to its own mirror types.
        unsafe {
            let set_api: Symbol<extern "C" fn(*const ())> = library
                .get(self.config.contract.set_api.as_bytes())
                .map_err(|_| LiveCodeError::SymbolMissing {
                    name: self.config.contract.set_api.clone(),
                })?;
            set_api(self.config.api_pointer);

            let set_dependencies: Symbol<extern "C" fn(*const usize)> = library
                .get(self.config.contract.set_dependencies.as_bytes())
                .map_err(|_| LiveCodeError::SymbolMissing {
                    name: self.config.contract.set_dependencies.clone(),
                })?;
            set_dependencies(self.base_symbols.as_ptr());
        };

        let new_address = resolve_symbol_address(
            &library,
            &self.config.contract.resolve_symbol,
            &symbol_path(changed),
        )
        .ok_or_else(|| LiveCodeError::SymbolMissing {
            name: symbol_path(changed),
        })?;
        let old_address = self
            .base_symbols
            .get(changed.index)
            .copied()
            .ok_or_else(|| LiveCodeError::SymbolMissing {
                name: changed.name.clone(),
            })?;

        self.emit(
            LogLevel::Normal,
            &format!(
                "    base {} @ {}  (pointer stays unchanged)\n",
                paint_dim(&changed.name),
                paint_cyan(&format!("{old_address:#x}"))
            ),
        );
        self.emit(
            LogLevel::Normal,
            &format!(
                "    new  {} @ {}  (patch DLL)\n",
                paint_bright_cyan(&changed.name),
                paint_cyan(&format!("{new_address:#x}"))
            ),
        );
        self.emit(
            LogLevel::Normal,
            &format!("    patching base prologue...\n"),
        );
        // SAFETY: single-threaded patching contract (see crate docs): the
        // function is not executing on another thread while its prologue is
        // rewritten.
        unsafe { crate::patch::patch_prologue(old_address, new_address)? };

        self.retained_libraries.push(library);
        self.emit(
            LogLevel::Normal,
            &format!(
                "{} {}\n",
                hot_prefix(),
                paint_bright_green(&format!(
                    "PATCHED {} IN PLACE in {:?} - base DLL untouched, pointer unchanged",
                    changed.name,
                    compile_start.elapsed()
                ))
            ),
        );
        Ok(())
    }

    // -- full rebuild ------------------------------------------------------

    /// Full rebuild + load alongside; re-patch the base prologue to the fresh
    /// code.  Returns `(update_address, base_re_patched)`.
    fn full_reload(&mut self, reason: &str) -> Result<(usize, bool)> {
        self.emit(
            LogLevel::Normal,
            &format!(
                "    {}\n",
                paint_yellow("── full rebuild + load-alongside ──")
            ),
        );
        if !reason.is_empty() {
            self.emit(
                LogLevel::Normal,
                &format!("    {}\n", paint_dim(&format!("reason: {reason}"))),
            );
        }
        let build_start = Instant::now();
        self.emit(
            LogLevel::Normal,
            &format!("    cargo {} ...\n", self.config.build_args.join(" ")),
        );
        let (library, update_address) = self.build_and_load_full()?;

        // The fresh DLL is the new base: re-resolve every hot symbol's
        // address in it (prologue-patch targets + dependency table).
        let project_sources = self.source_cache.sources();
        let hot_symbols = crate::symbols::collect_hot_symbols_cached(
            &project_sources,
            &self.infra_names,
            self.config.file_scope,
            &mut self.source_cache,
        );
        self.hot_keys = hot_symbols
            .iter()
            .map(|symbol| symbol_path(symbol))
            .collect();
        self.base_symbols = hot_symbols
            .iter()
            .map(|symbol| {
                resolve_symbol_address(
                    &library,
                    &self.config.contract.resolve_symbol,
                    &symbol_path(symbol),
                )
                .unwrap_or(0)
            })
            .collect();

        self.emit(
            LogLevel::Normal,
            &format!(
                "    build + load OK in {} ({} hot symbols)\n",
                paint_green(&format!("{:?}", build_start.elapsed())),
                paint_bright_cyan(&hot_symbols.len().to_string())
            ),
        );
        self.emit(
            LogLevel::Normal,
            &format!(
                "    fresh {} @ {}\n",
                self.config.contract.update,
                paint_cyan(&format!("{update_address:#x}"))
            ),
        );

        let base_re_patched = match self.base_update_address {
            Some(base) => {
                self.emit(
                    LogLevel::Normal,
                    &format!(
                        "    re-patching base prologue ({}) -> ({})...\n",
                        paint_cyan(&format!("{base:#x}")),
                        paint_cyan(&format!("{update_address:#x}"))
                    ),
                );
                // SAFETY: single-threaded patching contract (see crate docs).
                match unsafe { crate::patch::patch_prologue(base, update_address) } {
                    Ok(()) => {
                        self.emit(
                            LogLevel::Normal,
                            &format!(
                                "    {}\n",
                                paint_green("base prologue re-patched - pointer unchanged")
                            ),
                        );
                        true
                    }
                    Err(e) => {
                        self.emit(
                            LogLevel::Normal,
                            &format!(
                                "    {} - swapping engine pointer instead\n",
                                paint_yellow(&format!("prologue patch failed ({e})"))
                            ),
                        );
                        false
                    }
                }
            }
            None => {
                self.emit(
                    LogLevel::Normal,
                    &format!(
                        "    {}\n",
                        paint_dim("no base yet - pointing the engine directly at the fresh DLL")
                    ),
                );
                self.base_update_address = Some(update_address);
                false
            }
        };

        self.retained_libraries.push(library);
        Ok((update_address, base_re_patched))
    }

    // -- change detection --------------------------------------------------

    /// Analyse a file-change event and apply it: hot-patch the changed
    /// functions in place when possible, otherwise full-rebuild.  Returns
    /// what happened so the consumer can react (see [`ChangeOutcome`]).
    pub fn handle_change(&mut self, changed_paths: &[PathBuf]) -> Result<ChangeOutcome> {
        self.change_count += 1;
        self.emit(
            LogLevel::Normal,
            &format!(
                "\n{} {}\n",
                hot_prefix(),
                paint_bright_yellow(&format!(
                    "========== change #{} detected ==========",
                    self.change_count
                ))
            ),
        );
        for path in changed_paths {
            self.emit(
                LogLevel::Normal,
                &format!(
                    "  {} {}\n",
                    paint_cyan("▸ file changed:"),
                    paint_dim(&path.display().to_string())
                ),
            );
        }

        // Read the changed files (and only them) and merge with the cache;
        // unchanged files are never re-read or re-parsed.
        let project_sources = self
            .source_cache
            .refresh(&self.config.project_src_dir, changed_paths)?;
        if project_sources.is_empty() {
            return Err(LiveCodeError::InvalidConfig(format!(
                "no project sources found under {}",
                self.config.project_src_dir.display()
            )));
        }
        let lib_source = project_sources
            .iter()
            .find(|(path, _)| path.file_name().map_or(false, |name| name == "lib.rs"))
            .map(|(_, content)| content.clone())
            .unwrap_or_default();
        let extern_funcs = crate::analysis::extract_function_bodies(&lib_source);

        // Canonical hot symbols (update entry + module graph + lib helpers).
        let symbols = crate::symbols::collect_hot_symbols_cached(
            &project_sources,
            &self.infra_names,
            self.config.file_scope,
            &mut self.source_cache,
        );

        // Strip hot-function bodies (private lib helpers only when hot) so a
        // hot body edit counts as a body-only change.  ROOT-scope names only:
        // with qualified keys, every unique-name lib helper is hot (scope
        // "top"), so its body is stripped and edits are hot-patchable.
        let root_hot_names: HashSet<String> = symbols
            .iter()
            .filter(|symbol| symbol.scope == "top")
            .map(|symbol| symbol.name.clone())
            .collect();
        let stripped = project_sources
            .iter()
            .map(|(path, content)| self.source_cache.stripped(path, content, &root_hot_names))
            .collect::<Vec<_>>()
            .join("\n");
        let stripped_changed = stripped != self.stripped_source;
        self.stripped_source = stripped;

        // Per-function diff against the cache.  Tracked = the NON-hot plumbing
        // exports (set_api/init/resolve_symbol) + all hot symbols.  Hot
        // symbols like the update entry are also `pub extern` and would be
        // picked up by the extern extractor too - filtering them out here
        // keeps each function listed exactly once.
        let plumbing_only: Vec<(String, String, String, String)> = extern_funcs
            .iter()
            .filter(|entry| !symbols.iter().any(|symbol| symbol.name == entry.0))
            .cloned()
            .collect();
        // (qualified key, display name, params, body, ret) - the KEY is the
        // fully-qualified path, so a lib helper and a module function that
        // share a bare name are tracked as two distinct entries.
        let mut tracked: Vec<(String, String, String, String, String)> = plumbing_only
            .iter()
            .map(|(name, params, body, ret)| {
                (
                    name.clone(),
                    name.clone(),
                    params.clone(),
                    body.clone(),
                    ret.clone(),
                )
            })
            .collect();
        for symbol in &symbols {
            tracked.push((
                symbol_path(symbol),
                symbol.name.clone(),
                symbol.params.clone(),
                symbol.body.clone(),
                symbol.ret.clone(),
            ));
        }
        self.emit(
            LogLevel::Normal,
            &format!("  {}\n", rule("source analysis")),
        );
        self.emit(
            LogLevel::Normal,
            &format!(
                "  {} source file(s), {} hot function(s) + {} plumbing export(s) tracked:\n",
                paint_bright_cyan(&project_sources.len().to_string()),
                paint_bright_cyan(&symbols.len().to_string()),
                paint_bright_cyan(&plumbing_only.len().to_string())
            ),
        );
        for entry in &tracked {
            let status_text = if self.is_cached(&entry.0, &entry.3) {
                "unchanged"
            } else {
                "CHANGED"
            };
            let padded = format!("{status_text:<9}");
            let status = if status_text == "CHANGED" {
                paint_bright_red(&padded)
            } else {
                paint_dim(&padded)
            };
            let signature = if entry.4.is_empty() {
                format!("({})", entry.2)
            } else {
                format!("({}) -> {}", entry.2, entry.4)
            };
            self.emit(
                LogLevel::Normal,
                &format!(
                    "    {:<44} {} body={:>3} chars  sig={}\n",
                    entry.0,
                    status,
                    entry.3.len(),
                    signature
                ),
            );
        }
        let non_body_text = if stripped_changed {
            "CHANGED"
        } else {
            "unchanged"
        };
        self.emit(
            LogLevel::Normal,
            &format!(
                "  non-body content (structs / consts / static): {}\n",
                if stripped_changed {
                    paint_bright_red(non_body_text)
                } else {
                    paint_green(non_body_text)
                }
            ),
        );

        // Skip entirely when the save produced identical content (a no-op
        // touch / editor re-save) - nothing to patch or rebuild.  Tracked is
        // (key, name, params, body, ret), so the body lives at index 3.
        let anything_changed = tracked
            .iter()
            .any(|entry| !self.is_cached(&entry.0, &entry.3))
            || stripped_changed;
        if !anything_changed {
            self.emit(LogLevel::Normal, &format!("  {}\n", rule("decision")));
            self.emit(
                LogLevel::Normal,
                &format!(
                    "  {}\n",
                    paint_yellow("nothing actually changed (identical to cache) - skipping")
                ),
            );
            return Ok(ChangeOutcome::Nothing);
        }

        // Decide: which hot functions changed, and is ANYTHING non-hot-patchable?
        let changed_symbols: Vec<&HotSymbol> = symbols
            .iter()
            .filter(|symbol| !self.is_cached(&symbol_path(symbol), &symbol.body))
            .collect();

        // Anything below forces a full rebuild:
        //  - non-body content (consts / structs / attrs / module decls / bloat)
        //  - plumbing functions (set_api / init / resolver, whatever the
        //    contract names them)
        //  - the hot function set changed shape (new/removed module functions)
        let plumbing_changed = extern_funcs.iter().any(|entry| {
            (entry.0 == self.config.contract.set_api
                || entry.0 == self.config.contract.init
                || entry.0 == self.config.contract.resolve_symbol)
                && !self.is_cached(&entry.0, &entry.2)
        });
        let structure_changed = symbols.len() != self.hot_keys.len()
            || symbols
                .iter()
                .any(|symbol| !self.hot_keys.contains(&symbol_path(symbol)));

        let full_rebuild_needed = stripped_changed || plumbing_changed || structure_changed;

        self.emit(LogLevel::Normal, &format!("  {}\n", rule("decision")));
        if !full_rebuild_needed && !changed_symbols.is_empty() {
            let names = changed_symbols
                .iter()
                .map(|symbol| symbol.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            self.emit(
                LogLevel::Normal,
                &format!(
                    "  {} ({}) → per-function prologue patch\n",
                    paint_bright_green("hot-patchable = TRUE"),
                    paint_bright_cyan(&names)
                ),
            );
        } else {
            let mut reasons = Vec::new();
            if changed_symbols.is_empty() {
                reasons.push("no hot function body changed".to_string());
            }
            if stripped_changed {
                reasons.push("non-body content changed (structs/consts/static/bloat)".to_string());
            }
            if plumbing_changed {
                reasons.push(format!(
                    "plumbing function changed ({} / {})",
                    self.config.contract.set_api, self.config.contract.init
                ));
            }
            if structure_changed {
                reasons.push("hot function set changed shape (new/removed)".to_string());
            }
            let reasons = if reasons.is_empty() {
                "no actionable change".to_string()
            } else {
                reasons.join("; ")
            };
            self.emit(
                LogLevel::Normal,
                &format!(
                    "  {} ({reasons}) → {}\n",
                    paint_bright_yellow("hot-patchable = FALSE"),
                    paint_bright_cyan("full rebuild + load-alongside")
                ),
            );
        }

        // Fast path: every changed function is hot - patch each prologue in
        // place (composing naturally: each patch calls back into the base).
        if !full_rebuild_needed && !changed_symbols.is_empty() {
            let mut patched_all = true;
            for changed in &changed_symbols {
                match self.try_patch_function(&lib_source, &project_sources, changed) {
                    Ok(()) => {}
                    Err(errors) => {
                        self.emit(
                            LogLevel::Error,
                            &format!(
                                "{} per-function patch failed for {} - falling back to a full rebuild:\n{}\n",
                                hot_prefix(),
                                changed.name,
                                paint_red(&errors.to_string())
                            ),
                        );
                        patched_all = false;
                        break;
                    }
                }
            }
            if patched_all {
                for changed in &changed_symbols {
                    self.upsert_cached((
                        symbol_path(changed),
                        changed.params.clone(),
                        changed.body.clone(),
                        changed.ret.clone(),
                    ));
                }
                return Ok(ChangeOutcome::Patched {
                    functions: changed_symbols
                        .iter()
                        .map(|symbol| symbol.name.clone())
                        .collect(),
                });
            }
        }

        // Full-rebuild fallback (or the change wasn't hot-patchable).
        match self.full_reload("hot-ineligible or per-function patch failed") {
            Ok((update_address, base_re_patched)) => {
                for entry in &tracked {
                    self.upsert_cached((
                        entry.0.clone(),
                        entry.2.clone(),
                        entry.3.clone(),
                        entry.4.clone(),
                    ));
                }
                if base_re_patched {
                    Ok(ChangeOutcome::FullReload { update_address })
                } else {
                    Ok(ChangeOutcome::FullReloadNeedsPointerSwap { update_address })
                }
            }
            Err(errors) => {
                self.emit(
                    LogLevel::Error,
                    &format!(
                        "{} rebuild FAILED - keeping previous code running:\n{}\n",
                        hot_prefix(),
                        paint_red(&errors.to_string())
                    ),
                );
                Err(errors)
            }
        }
    }

    /// Watch `project_dir` for changes and apply them, blocking forever.
    ///
    /// This is the batteries-included loop for console hosts (available with
    /// the `watch` feature).  It calls `on_outcome` on the main thread after
    /// every change is applied, so the consumer can react (e.g. repoint its
    /// engine on [`ChangeOutcome::FullReloadNeedsPointerSwap`]).
    ///
    /// # Panics
    ///
    /// Never returns on its own - the watcher channel closing is treated as a
    /// fatal error and propagated.
    #[cfg(feature = "watch")]
    pub fn watch<F>(&mut self, mut on_outcome: F) -> Result<()>
    where
        F: FnMut(ChangeOutcome),
    {
        use notify::{Config, RecursiveMode, Watcher};
        use std::time::Duration;

        let (change_tx, change_rx) = std::sync::mpsc::channel::<Vec<PathBuf>>();
        let watch_dir = self.config.project_dir.clone();
        let mut watcher = notify::recommended_watcher(
            move |result: std::result::Result<notify::Event, notify::Error>| {
                if let Ok(event) = result {
                    let changed_paths: Vec<PathBuf> = event
                        .paths
                        .iter()
                        .filter(|path| {
                            path.extension().map_or(false, |ext| ext == "rs")
                                || path.file_name().map_or(false, |name| name == "Cargo.toml")
                        })
                        .cloned()
                        .collect();
                    if !changed_paths.is_empty() {
                        let _ = change_tx.send(changed_paths);
                    }
                }
            },
        )
        .map_err(|e| LiveCodeError::InvalidConfig(format!("cannot create watcher: {e}")))?;
        watcher
            .configure(Config::default().with_poll_interval(Duration::from_millis(500)))
            .map_err(|e| LiveCodeError::InvalidConfig(format!("cannot configure watcher: {e}")))?;
        watcher
            .watch(&watch_dir, RecursiveMode::Recursive)
            .map_err(|e| {
                LiveCodeError::InvalidConfig(format!("cannot watch {}: {e}", watch_dir.display()))
            })?;

        self.emit(
            LogLevel::Normal,
            &format!(
                "{} watching {} for source changes...\n",
                crate::style::paint_bright_cyan("[watch]"),
                watch_dir.display()
            ),
        );

        loop {
            // Block until the next change event arrives.
            let changed_paths = change_rx.recv().map_err(|e| {
                LiveCodeError::InvalidConfig(format!("watcher channel closed: {e}"))
            })?;
            // Let the editor finish writing before reading the file.
            std::thread::sleep(Duration::from_millis(150));
            // Merge any extra events from the same save burst.
            let mut all_paths = changed_paths;
            while let Ok(more_paths) = change_rx.try_recv() {
                for path in more_paths {
                    if !all_paths.contains(&path) {
                        all_paths.push(path);
                    }
                }
            }
            match self.handle_change(&all_paths) {
                Ok(outcome) => on_outcome(outcome),
                Err(errors) => self.emit(
                    LogLevel::Error,
                    &format!(
                        "{} change handling failed - keeping previous code running:\n{}\n",
                        hot_prefix(),
                        errors
                    ),
                ),
            }
        }
    }
}
