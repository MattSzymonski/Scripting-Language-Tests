//! Module manager: loads, tracks, watches, and reloads the AAA-scenario
//! module DLLs described by `aaa_scenario/manifest.json`.
//!
//! Every module (core, subsystem, and leaf tier alike) goes through the same
//! four timed phases per module, mirroring `AAA-Scenario.csv`'s columns:
//!
//!   1. **LoadLibrary**  — `libloading::Library::new()`. Because dependency
//!      DLLs are real PE imports (see each crate's generated `build.rs`),
//!      this genuinely triggers Windows' own recursive loader for any
//!      not-yet-resident dependency — including that dependency's `#[ctor]`
//!      static-initializer cost. Nothing here is simulated.
//!   2. **GetSymbol**    — resolving the `create_module_instance` and
//!      `module_startup` exports.
//!   3. **CreateInstance** — calling `create_module_instance()`.
//!   4. **StartupModule** — calling `module_startup()`, the analog of
//!      Unreal's `StartupModule`: additive, and squarely each module's own
//!      responsibility rather than the OS loader's.
//!
//! Only the leaf tier is hot-reloadable (core/subsystem model "engine" code,
//! not hot-reloadable content) — reload copies to a versioned file name so
//! the original stays loadable/rebuildable while the old version is still
//! mapped, same trick the original single-plugin harness used.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use rayon::prelude::*;

use crate::ffi::{CreateInstanceFn, ModuleIdentity, ModuleMetrics, ModuleStartupFn, ModuleTier};
use crate::manifest::Manifest;
use crate::win32;

/// Empirically-observed baseline of OS/CRT DLLs a Rust cdylib on this
/// toolchain always imports (KERNEL32, ntdll, VCRUNTIME140, three
/// api-ms-win-crt-* forwarders — see analyze_dll.py output on any generated
/// module). Added to a module's own declared dependency count to approximate
/// `NumImportedDlls` without embedding a full PE parser in this binary.
const BASELINE_OS_IMPORTS: u32 = 6;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadMode {
    /// One DLL at a time, in the manifest's fixed (dependency-oblivious) boot order.
    Sequential,
    /// Load multiple DLLs concurrently via a rayon thread pool.
    Parallel,
    /// Defer loading until first access (not implemented — reserved).
    Lazy,
}

#[derive(Debug, Clone)]
pub struct LoadMetrics {
    pub mode: &'static str,
    pub total_load_ms: f64,
    pub total_load_library_ms: f64,
    pub total_get_symbol_ms: f64,
    pub total_create_instance_ms: f64,
    pub total_startup_module_ms: f64,
    pub loaded_count: usize,
    pub total_modules: usize,
    pub core_count: usize,
    pub subsystem_count: usize,
    pub leaf_count: usize,
    pub reload_count: usize,
    pub fps: f64,
    /// Slowest modules by Total_ms, paired with any dependency they were the
    /// first to pull in transitively — the concrete, per-run evidence of
    /// dll_loading_performance_101.md section 6's "unlucky first loader".
    pub top_outliers: Vec<(String, f64, Vec<String>)>,
}

struct LoadedModule {
    #[allow(dead_code)]
    library: Arc<libloading::Library>,
    identity: ModuleIdentity,
    version: usize,
}

pub struct PluginManager {
    manifest: Manifest,
    release_dir: PathBuf,
    /// aaa_scenario/ root — parent of `core/`, `subsystem/`, `leaf/`, `target/`.
    scenario_root: PathBuf,
    mode: LoadMode,
    dep_closures: HashMap<String, Vec<String>>,
    loaded: HashMap<String, LoadedModule>,
    metrics: Vec<ModuleMetrics>,
    total_load_ms: f64,
    reload_count: Arc<AtomicUsize>,
    last_frame_time: Instant,
    frame_count: u64,
    current_fps: f64,
    reload_queue: Arc<Mutex<Vec<String>>>,
    watcher_active: bool,
}

impl PluginManager {
    pub fn new(mode: LoadMode, manifest: Manifest, manifest_path: &Path) -> Self {
        let release_dir = manifest.release_dir(manifest_path);
        let scenario_root = manifest_path.parent().unwrap_or(Path::new(".")).to_path_buf();

        let dep_closures = manifest
            .modules
            .keys()
            .map(|name| (name.clone(), manifest.transitive_deps(name)))
            .collect();

        Self {
            manifest,
            release_dir,
            scenario_root,
            mode,
            dep_closures,
            loaded: HashMap::new(),
            metrics: Vec::new(),
            total_load_ms: 0.0,
            reload_count: Arc::new(AtomicUsize::new(0)),
            last_frame_time: Instant::now(),
            frame_count: 0,
            current_fps: 0.0,
            reload_queue: Arc::new(Mutex::new(Vec::new())),
            watcher_active: false,
        }
    }

    // ── Loading ─────────────────────────────────────────

    pub fn load_all(&mut self) -> Result<(), String> {
        let start = Instant::now();
        let boot_order = self.manifest.boot_order.clone();

        match self.mode {
            LoadMode::Sequential => {
                println!("Loading mode: SEQUENTIAL (boot order, one DLL at a time)");
                for name in &boot_order {
                    let (metrics, loaded) = load_one(&self.manifest, &self.release_dir, &self.dep_closures, name, 1);
                    if let Some(l) = loaded {
                        self.loaded.insert(name.clone(), l);
                    }
                    self.metrics.push(metrics);
                }
            }
            LoadMode::Parallel => {
                println!("Loading mode: PARALLEL (rayon thread pool over boot order)");
                let manifest = &self.manifest;
                let release_dir = &self.release_dir;
                let dep_closures = &self.dep_closures;
                let results: Vec<(ModuleMetrics, Option<LoadedModule>)> = boot_order
                    .par_iter()
                    .map(|name| load_one(manifest, release_dir, dep_closures, name, 1))
                    .collect();
                for (metrics, loaded) in results {
                    let name = metrics.name.clone();
                    if let Some(l) = loaded {
                        self.loaded.insert(name, l);
                    }
                    self.metrics.push(metrics);
                }
            }
            LoadMode::Lazy => {
                println!("Loading mode: LAZY (deferred — nothing loaded yet)");
            }
        }

        self.total_load_ms = start.elapsed().as_secs_f64() * 1000.0;

        let loaded = self.loaded.len();
        println!(
            "PluginManager: loaded {}/{} modules in {:.1} ms",
            loaded,
            boot_order.len(),
            self.total_load_ms
        );
        Ok(())
    }

    /// Reload a specific leaf module (called by the watcher).
    pub fn reload_module(&mut self, name: &str) -> Result<(), String> {
        let entry = self
            .manifest
            .modules
            .get(name)
            .ok_or_else(|| format!("unknown module: {}", name))?;
        if !entry.hot_reloadable {
            return Err(format!("{} is not hot-reloadable (tier: {})", name, entry.tier));
        }

        let current_version = self.loaded.get(name).map(|m| m.version).unwrap_or(0);
        let new_version = current_version + 1;

        let original = self.release_dir.join(&entry.dll);
        if !original.exists() {
            return Err(format!("DLL not found: {}", original.display()));
        }
        let versioned = versioned_dll_path(&self.release_dir, name, new_version);
        std::fs::copy(&original, &versioned).map_err(|e| format!("copy failed: {}", e))?;

        let library = unsafe {
            libloading::Library::new(&versioned).map_err(|e| format!("libloading failed: {}", e))?
        };
        let library = Arc::new(library);
        let identity = unsafe {
            let sym: libloading::Symbol<CreateInstanceFn> = library
                .get(b"create_module_instance")
                .map_err(|e| format!("symbol lookup failed: {}", e))?;
            (*sym)()
        };
        unsafe {
            let sym: libloading::Symbol<ModuleStartupFn> = library
                .get(b"module_startup")
                .map_err(|e| format!("symbol lookup failed: {}", e))?;
            (*sym)();
        }

        self.loaded.insert(
            name.to_string(),
            LoadedModule { library, identity, version: new_version },
        );
        self.reload_count.fetch_add(1, Ordering::Relaxed);

        println!(
            "  Reloaded {} (v{}) - identity: {}",
            name,
            new_version,
            self.loaded[name].identity.name_string()
        );
        Ok(())
    }

    // ── Queries ─────────────────────────────────────────

    pub fn all_identities(&self) -> Vec<(String, ModuleIdentity, bool)> {
        self.manifest
            .boot_order
            .iter()
            .map(|name| {
                if let Some(loaded) = self.loaded.get(name) {
                    (name.clone(), loaded.identity, true)
                } else {
                    (
                        name.clone(),
                        ModuleIdentity {
                            name: [0u8; 32],
                            color_r: 80,
                            color_g: 80,
                            color_b: 80,
                            tier: tier_str_to_u8(&self.manifest.modules[name].tier),
                            size: 1.0,
                        },
                        false,
                    )
                }
            })
            .collect()
    }

    pub fn total_modules(&self) -> usize {
        self.manifest.modules.len()
    }

    pub fn loaded_count(&self) -> usize {
        self.loaded.len()
    }

    // ── Metrics ─────────────────────────────────────────

    pub fn metrics(&self) -> LoadMetrics {
        let mode_str = match self.mode {
            LoadMode::Sequential => "sequential",
            LoadMode::Parallel => "parallel",
            LoadMode::Lazy => "lazy",
        };

        let mut sorted: Vec<&ModuleMetrics> = self.metrics.iter().collect();
        sorted.sort_by(|a, b| b.total_ms.partial_cmp(&a.total_ms).unwrap());
        let top_outliers = sorted
            .iter()
            .take(8)
            .map(|m| (m.name.clone(), m.total_ms, m.newly_loaded_deps.clone()))
            .collect();

        LoadMetrics {
            mode: mode_str,
            total_load_ms: self.total_load_ms,
            total_load_library_ms: self.metrics.iter().map(|m| m.load_library_ms).sum(),
            total_get_symbol_ms: self.metrics.iter().map(|m| m.get_symbol_ms).sum(),
            total_create_instance_ms: self.metrics.iter().map(|m| m.create_instance_ms).sum(),
            total_startup_module_ms: self.metrics.iter().map(|m| m.startup_module_ms).sum(),
            loaded_count: self.loaded_count(),
            total_modules: self.total_modules(),
            core_count: self.manifest.counts.core,
            subsystem_count: self.manifest.counts.subsystem,
            leaf_count: self.manifest.counts.leaf,
            reload_count: self.reload_count.load(Ordering::Relaxed),
            fps: self.current_fps,
            top_outliers,
        }
    }

    pub fn update_frame(&mut self) {
        self.frame_count += 1;
        let elapsed = self.last_frame_time.elapsed();
        if elapsed.as_secs_f64() >= 0.5 {
            self.current_fps = self.frame_count as f64 / elapsed.as_secs_f64();
            self.frame_count = 0;
            self.last_frame_time = Instant::now();
        }
    }

    /// Write a CSV report. The first 12 columns exactly match
    /// `AAA-Scenario.csv`'s layout, so a run of this harness can be diffed
    /// directly against the reference Unreal data already in the repo; a
    /// trailing `Tier` column (not present in that reference file) is added
    /// for this harness's own core/subsystem/leaf breakdown.
    pub fn write_csv(&self, path: &Path) -> std::io::Result<()> {
        use std::io::Write;
        let mut f = std::fs::File::create(path)?;
        writeln!(
            f,
            "ModuleName,Total_ms,LoadLibrary_ms,GetSymbol_ms,CreateInstance_ms,StartupModule_ms,Status,FileSize_bytes,MemoryUsed_bytes,NumExportedFunctions,NumExportedNames,NumImportedDlls,Tier"
        )?;
        for m in &self.metrics {
            writeln!(
                f,
                "{},{:.3},{:.3},{:.3},{:.3},{:.3},{},{},{},{},{},{},{}",
                m.name,
                m.total_ms,
                m.load_library_ms,
                m.get_symbol_ms,
                m.create_instance_ms,
                m.startup_module_ms,
                m.status,
                m.file_size_bytes,
                m.memory_used_bytes,
                m.num_exported_functions,
                m.num_exported_functions,
                m.num_imported_dlls,
                m.tier,
            )?;
        }
        Ok(())
    }

    // ── File watching (leaf tier only) ──────────────────

    pub fn start_watching(&mut self) {
        if self.watcher_active {
            return;
        }
        self.watcher_active = true;

        let leaf_source_dir = self.scenario_root.join("leaf");
        let leaf_manifest = self.scenario_root.join("leaf").join("Cargo.toml");
        let target_dir = self.scenario_root.join("target");
        let reload_queue = self.reload_queue.clone();
        let known_leaf_names: HashSet<String> = self
            .manifest
            .modules
            .iter()
            .filter(|(_, e)| e.hot_reloadable)
            .map(|(n, _)| n.clone())
            .collect();

        if !leaf_source_dir.exists() {
            return;
        }

        std::thread::spawn(move || {
            use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

            let (tx, rx) = std::sync::mpsc::channel();
            let mut watcher = match RecommendedWatcher::new(
                move |res: Result<Event, notify::Error>| {
                    if let Ok(event) = res {
                        let _ = tx.send(event);
                    }
                },
                Config::default(),
            ) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Watcher failed to start: {}", e);
                    return;
                }
            };

            if let Err(e) = watcher.watch(&leaf_source_dir, RecursiveMode::Recursive) {
                eprintln!("Watcher failed to watch: {}", e);
                return;
            }
            println!("Watching for changes in: {}", leaf_source_dir.display());

            let mut pending: Vec<String> = Vec::new();
            let debounce_ms = 500;

            loop {
                match rx.recv_timeout(std::time::Duration::from_millis(debounce_ms)) {
                    Ok(event) => {
                        if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                            for path in &event.paths {
                                if let Some(name) = extract_leaf_name(path, &known_leaf_names) {
                                    if !pending.contains(&name) {
                                        pending.push(name);
                                    }
                                }
                            }
                        }
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        if !pending.is_empty() {
                            pending.sort();
                            pending.dedup();
                            for name in &pending {
                                println!("  Rebuilding {}...", name);
                                rebuild_module(name, &leaf_manifest, &target_dir);
                            }
                            if let Ok(mut queue) = reload_queue.lock() {
                                queue.extend(pending.drain(..));
                            }
                        }
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }
        });
    }

    pub fn process_reloads(&mut self) {
        let indices: Vec<String> = {
            let mut queue = self.reload_queue.lock().unwrap();
            queue.drain(..).collect()
        };
        for name in indices {
            if let Err(e) = self.reload_module(&name) {
                eprintln!("  Failed to reload {}: {}", name, e);
            }
        }
    }

}

fn tier_str_to_u8(tier: &str) -> u8 {
    match tier {
        "core" => 0,
        "subsystem" => 1,
        _ => 2,
    }
}

// ── Core loading logic (shared by sequential and parallel paths) ────────

fn load_one(
    manifest: &Manifest,
    release_dir: &Path,
    dep_closures: &HashMap<String, Vec<String>>,
    name: &str,
    version: usize,
) -> (ModuleMetrics, Option<LoadedModule>) {
    let entry = &manifest.modules[name];
    let original_path = release_dir.join(&entry.dll);

    let dll_path = if entry.hot_reloadable {
        let versioned = versioned_dll_path(release_dir, name, version);
        if let Err(e) = std::fs::copy(&original_path, &versioned) {
            return (fail_metrics(name, entry.tier.as_str(), entry, &format!("copy failed: {}", e)), None);
        }
        versioned
    } else {
        original_path
    };

    let file_size = std::fs::metadata(&dll_path).map(|m| m.len()).unwrap_or(entry.payload_bytes);

    let closure = dep_closures.get(name).cloned().unwrap_or_default();
    let residents_before: HashSet<String> = closure
        .iter()
        .filter(|d| win32::is_module_resident(&format!("{}.dll", d)))
        .cloned()
        .collect();

    let mem_before = win32::working_set_bytes() as i64;

    // Retries here are not papering over a bug: on a machine with an active
    // EDR/AV agent (verified present on this one — see AAA_SCENARIO.md),
    // `LoadLibrary` on a just-written, never-seen-before file can transiently
    // fail while the agent's real-time scan holds the file — precisely
    // dll_loading_performance_101.md section 7, item 3. Retrying (and
    // counting the wait toward LoadLibrary_ms, since that's genuinely how
    // long the caller waited to get the module loaded) is both the practical
    // fix and the honest one.
    let t_load = Instant::now();
    let library = load_library_with_retry(&dll_path);
    let load_library_ms = t_load.elapsed().as_secs_f64() * 1000.0;

    let library = match library {
        Ok(l) => Arc::new(l),
        Err(e) => {
            return (
                fail_metrics(name, entry.tier.as_str(), entry, &format!("libloading failed: {}", e)),
                None,
            )
        }
    };

    let newly_loaded_deps: Vec<String> = closure
        .into_iter()
        .filter(|d| !residents_before.contains(d) && win32::is_module_resident(&format!("{}.dll", d)))
        .collect();

    let t_sym = Instant::now();
    let create_instance: Result<CreateInstanceFn, String> = unsafe {
        library
            .get::<CreateInstanceFn>(b"create_module_instance")
            .map(|s| *s)
            .map_err(|e| e.to_string())
    };
    let module_startup: Result<ModuleStartupFn, String> = unsafe {
        library
            .get::<ModuleStartupFn>(b"module_startup")
            .map(|s| *s)
            .map_err(|e| e.to_string())
    };
    let get_symbol_ms = t_sym.elapsed().as_secs_f64() * 1000.0;

    let (create_instance, module_startup) = match (create_instance, module_startup) {
        (Ok(c), Ok(s)) => (c, s),
        (Err(e), _) | (_, Err(e)) => {
            return (fail_metrics(name, entry.tier.as_str(), entry, &format!("symbol lookup failed: {}", e)), None)
        }
    };

    let t_create = Instant::now();
    let identity = unsafe { create_instance() };
    let create_instance_ms = t_create.elapsed().as_secs_f64() * 1000.0;

    let t_startup = Instant::now();
    let _ = unsafe { module_startup() };
    let startup_module_ms = t_startup.elapsed().as_secs_f64() * 1000.0;

    let mem_after = win32::working_set_bytes() as i64;

    let metrics = ModuleMetrics {
        name: name.to_string(),
        tier: ModuleTier::from_str(&entry.tier),
        total_ms: load_library_ms + get_symbol_ms + create_instance_ms + startup_module_ms,
        load_library_ms,
        get_symbol_ms,
        create_instance_ms,
        startup_module_ms,
        status: "OK",
        file_size_bytes: file_size,
        memory_used_bytes: (mem_after - mem_before).max(0),
        num_exported_functions: entry.export_count,
        num_imported_dlls: entry.deps.len() as u32 + BASELINE_OS_IMPORTS,
        newly_loaded_deps,
    };

    (metrics, Some(LoadedModule { library, identity, version }))
}

fn fail_metrics(name: &str, tier: &str, entry: &crate::manifest::ModuleEntry, reason: &str) -> ModuleMetrics {
    eprintln!("  WARNING: failed to load {}: {}", name, reason);
    ModuleMetrics {
        name: name.to_string(),
        tier: ModuleTier::from_str(tier),
        total_ms: 0.0,
        load_library_ms: 0.0,
        get_symbol_ms: 0.0,
        create_instance_ms: 0.0,
        startup_module_ms: 0.0,
        status: "FAIL",
        file_size_bytes: entry.payload_bytes,
        memory_used_bytes: 0,
        num_exported_functions: entry.export_count,
        num_imported_dlls: entry.deps.len() as u32 + BASELINE_OS_IMPORTS,
        newly_loaded_deps: Vec::new(),
    }
}

// ── Helpers ─────────────────────────────────────────────

/// Retries `LoadLibrary` a bounded number of times with short backoff. See
/// the comment at the call site: this compensates for transient EDR/AV
/// real-time-scan lock contention on freshly-written files, not a bug in the
/// loading logic itself.
const LOAD_RETRY_ATTEMPTS: u32 = 8;

fn load_library_with_retry(path: &Path) -> Result<libloading::Library, libloading::Error> {
    let mut last_err = None;
    for attempt in 0..LOAD_RETRY_ATTEMPTS {
        match unsafe { libloading::Library::new(path) } {
            Ok(lib) => return Ok(lib),
            Err(e) => {
                last_err = Some(e);
                std::thread::sleep(std::time::Duration::from_millis(50 * (attempt as u64 + 1)));
            }
        }
    }
    Err(last_err.expect("loop runs at least once"))
}

fn versioned_dll_path(release_dir: &Path, name: &str, version: usize) -> PathBuf {
    release_dir.join(format!("{}_v{}.dll", name, version))
}

fn extract_leaf_name(path: &Path, known: &HashSet<String>) -> Option<String> {
    for component in path.components() {
        if let Some(s) = component.as_os_str().to_str() {
            if known.contains(s) {
                return Some(s.to_string());
            }
        }
    }
    None
}

fn rebuild_module(name: &str, leaf_manifest: &Path, target_dir: &Path) {
    let output = std::process::Command::new("cargo")
        .args([
            "build",
            "--release",
            "--manifest-path",
            leaf_manifest.to_str().unwrap_or("leaf/Cargo.toml"),
            "--target-dir",
            target_dir.to_str().unwrap_or("target"),
            "-p",
            name,
        ])
        .output();

    match output {
        Ok(out) => {
            if out.status.success() {
                println!("    Build OK");
            } else {
                eprintln!("    Build FAILED:\n{}", String::from_utf8_lossy(&out.stderr));
            }
        }
        Err(e) => eprintln!("    Failed to run cargo: {}", e),
    }
}
