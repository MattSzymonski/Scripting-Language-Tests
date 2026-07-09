//! Plugin manager: loads, tracks, watches, and reloads bird plugin DLLs.
//!
//! Supports multiple load strategies for benchmarking:
//! - **Sequential**: one DLL at a time (baseline, like Unreal's default)
//! - **Parallel**: rayon thread pool for concurrent LoadLibrary calls
//! - **Lazy**: defer loading until first access

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use rayon::prelude::*;

use crate::ffi::{BirdDefinition, CreateBirdFn, PluginInfo};

// ── Load strategy ───────────────────────────────────────

/// How plugins are loaded.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadMode {
    /// One DLL at a time — baseline measurement.
    Sequential,
    /// Load multiple DLLs concurrently via rayon thread pool.
    /// Windows loader has internal locks, but I/O and static init can overlap.
    Parallel,
    /// Only load a DLL when its bird is first requested (not implemented).
    Lazy,
}

// ── Metrics ─────────────────────────────────────────────

/// Load-time and runtime metrics collected by the plugin manager.
#[derive(Debug, Clone)]
pub struct LoadMetrics {
    /// Load strategy used.
    pub mode: &'static str,
    /// Total wall-clock time to load all plugins (ms).
    pub total_load_ms: f64,
    /// Time spent in file I/O (copy to versioned path) — sum across all plugins.
    pub total_io_ms: f64,
    /// Time spent in libloading::Library::new() — sum across all plugins.
    pub total_symbol_ms: f64,
    /// Time spent in create_bird() calls — sum across all plugins.
    pub total_init_ms: f64,
    /// Per-plugin load times, indexed by plugin index.
    pub per_plugin_ms: Vec<f64>,
    /// Number of plugins successfully loaded.
    pub loaded_count: usize,
    /// Total number of plugins attempted.
    pub total_plugins: usize,
    /// Number of times a plugin was reloaded (hot-reload events).
    pub reload_count: usize,
    /// Current frames per second.
    pub fps: f64,
}

// ── Plugin handle ───────────────────────────────────────

/// Holds a loaded DLL and its extracted function pointer.
#[derive(Clone)]
struct LoadedPlugin {
    /// Keeps the DLL mapped in memory. Dropping this unloads the library.
    #[allow(dead_code)]
    library: Arc<libloading::Library>,
    /// Pointer to the `create_bird` function exported by the DLL.
    create_bird: CreateBirdFn,
    /// Cached bird definition (call once on load, reuse each frame).
    bird: BirdDefinition,
    /// Path to the versioned DLL copy on disk.
    #[allow(dead_code)]
    versioned_path: PathBuf,
    /// Version counter for this plugin (increments on each reload).
    version: usize,
}

// ── Plugin manager ──────────────────────────────────────

/// Manages all plugin DLLs: loading, querying, watching, and reloading.
pub struct PluginManager {
    /// All loaded plugins, keyed by index.
    plugins: Vec<Option<LoadedPlugin>>,
    /// Load strategy.
    mode: LoadMode,
    /// Per-plugin load times for metrics.
    load_times_ms: Vec<f64>,
    /// Total time spent loading all plugins.
    total_load_ms: f64,
    /// Time breakdown.
    total_io_ms: f64,
    total_symbol_ms: f64,
    total_init_ms: f64,
    /// Number of reloads performed.
    reload_count: Arc<AtomicUsize>,
    /// Plugin source directory (watched for changes).
    plugin_source_dir: PathBuf,
    /// Plugin build output directory (where DLLs are).
    plugin_target_dir: PathBuf,
    /// FPS tracking.
    last_frame_time: Instant,
    frame_count: u64,
    current_fps: f64,
    /// Whether an initial full load has completed.
    initialized: bool,
    /// Reload queue: plugin indices that need reloading (from watcher).
    reload_queue: Arc<Mutex<Vec<usize>>>,
    /// Whether the watcher has been started.
    watcher_active: bool,
}

impl PluginManager {
    // ── Construction ────────────────────────────────────

    /// Create a new plugin manager.
    ///
    /// * `plugin_source_dir` — directory containing `plugin_XXX/` source crates.
    /// * `plugin_target_dir` — directory containing built `plugin_XXX.dll` files
    ///   (typically `plugins/target/release/`).
    pub fn new(
        mode: LoadMode,
        plugin_source_dir: impl Into<PathBuf>,
        plugin_target_dir: impl Into<PathBuf>,
    ) -> Self {
        Self {
            plugins: Vec::new(),
            mode,
            load_times_ms: Vec::new(),
            total_load_ms: 0.0,
            total_io_ms: 0.0,
            total_symbol_ms: 0.0,
            total_init_ms: 0.0,
            reload_count: Arc::new(AtomicUsize::new(0)),
            plugin_source_dir: plugin_source_dir.into(),
            plugin_target_dir: plugin_target_dir.into(),
            last_frame_time: Instant::now(),
            frame_count: 0,
            current_fps: 0.0,
            initialized: false,
            reload_queue: Arc::new(Mutex::new(Vec::new())),
            watcher_active: false,
        }
    }

    // ── Loading ─────────────────────────────────────────

    /// Load all plugin DLLs from the target directory.
    pub fn load_all(&mut self) -> Result<(), String> {
        let start = Instant::now();

        // Scan for plugin DLLs
        let mut dll_entries: Vec<(usize, PathBuf)> = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&self.plugin_target_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                    if name.starts_with("plugin_") && path.extension().map_or(false, |e| e == "dll")
                    {
                        if let Ok(idx) = name.strip_prefix("plugin_").unwrap_or("").parse::<usize>()
                        {
                            dll_entries.push((idx, path));
                        }
                    }
                }
            }
        }

        dll_entries.sort_by_key(|(idx, _)| *idx);
        let plugin_count = dll_entries.len();

        self.plugins = vec![None; plugin_count];
        self.load_times_ms = vec![0.0; plugin_count];

        match self.mode {
            LoadMode::Sequential => {
                println!("Loading mode: SEQUENTIAL (one DLL at a time)");
                self.load_sequential(&dll_entries);
            }
            LoadMode::Parallel => {
                println!("Loading mode: PARALLEL (rayon thread pool)");
                self.load_parallel(&dll_entries);
            }
            LoadMode::Lazy => {
                println!("Loading mode: LAZY (deferred — nothing loaded yet)");
            }
        }

        self.total_load_ms = start.elapsed().as_secs_f64() * 1000.0;
        self.initialized = true;

        let loaded = self.loaded_count();
        println!(
            "PluginManager: loaded {}/{} plugins in {:.1} ms (IO: {:.1} ms, symbol: {:.1} ms, init: {:.1} ms)",
            loaded, plugin_count, self.total_load_ms,
            self.total_io_ms, self.total_symbol_ms, self.total_init_ms
        );

        Ok(())
    }

    /// Sequential loading: one DLL after another.
    fn load_sequential(&mut self, entries: &[(usize, PathBuf)]) {
        for (index, dll_path) in entries {
            let per_start = Instant::now();

            match self.load_single(*index, dll_path) {
                Ok((io_ms, sym_ms, init_ms)) => {
                    self.total_io_ms += io_ms;
                    self.total_symbol_ms += sym_ms;
                    self.total_init_ms += init_ms;
                }
                Err(err) => {
                    eprintln!("  WARNING: failed to load plugin_{:03}: {}", index, err);
                }
            }

            self.load_times_ms[*index] = per_start.elapsed().as_secs_f64() * 1000.0;
        }
    }

    /// Parallel loading: use rayon to load multiple DLLs concurrently.
    fn load_parallel(&mut self, entries: &[(usize, PathBuf)]) {
        // Pre-compute versioned paths (can't borrow self in par_iter)
        let target_dir = self.plugin_target_dir.clone();

        let results: Vec<(usize, Result<(f64, f64, f64, LoadedPlugin), String>)> = entries
            .par_iter()
            .map(|(index, dll_path)| {
                let per_start = Instant::now();

                let result = load_single_dll(*index, dll_path, &target_dir);

                let elapsed = per_start.elapsed().as_secs_f64() * 1000.0;
                match &result {
                    Ok((_, _, _, _)) => (*index, Ok(result.unwrap())),
                    Err(e) => (*index, Err(e.clone())),
                }
                // Note: we lose per-plugin timing in parallel mode due to rayon
                // but total wall clock is what matters for stress testing
            })
            .collect();

        for (index, result) in results {
            match result {
                Ok((io_ms, sym_ms, init_ms, plugin)) => {
                    self.total_io_ms += io_ms;
                    self.total_symbol_ms += sym_ms;
                    self.total_init_ms += init_ms;
                    self.plugins[index] = Some(plugin);
                }
                Err(err) => {
                    eprintln!("  WARNING: failed to load plugin_{:03}: {}", index, err);
                }
            }
        }
    }

    /// Load a single plugin DLL (sequential mode).
    fn load_single(&mut self, index: usize, dll_path: &Path) -> Result<(f64, f64, f64), String> {
        let (io_ms, sym_ms, init_ms, plugin) =
            load_single_dll(index, dll_path, &self.plugin_target_dir)?;
        self.plugins[index] = Some(plugin);
        Ok((io_ms, sym_ms, init_ms))
    }

    /// Reload a specific plugin (called by watcher or manually).
    pub fn reload_plugin(&mut self, index: usize) -> Result<(), String> {
        let current_version = self.plugins[index].as_ref().map(|p| p.version).unwrap_or(0);
        let new_version = current_version + 1;

        let dll_path = self
            .plugin_target_dir
            .join(format!("plugin_{:03}.dll", index));
        if !dll_path.exists() {
            return Err(format!("DLL not found: {}", dll_path.display()));
        }

        // Copy to new versioned path
        let versioned_path = versioned_dll_path(&self.plugin_target_dir, index, new_version);
        std::fs::copy(&dll_path, &versioned_path).map_err(|e| format!("copy failed: {}", e))?;

        // Load new version
        let library = unsafe {
            libloading::Library::new(&versioned_path)
                .map_err(|e| format!("libloading failed: {}", e))?
        };
        let library = Arc::new(library);

        let create_bird: CreateBirdFn = {
            let symbol: libloading::Symbol<CreateBirdFn> = unsafe {
                library
                    .get(b"create_bird")
                    .map_err(|e| format!("symbol lookup failed: {}", e))?
            };
            *symbol
        };

        let bird = unsafe { (create_bird)() };

        // Swap in the new plugin (old one drops, freeing the old DLL)
        self.plugins[index] = Some(LoadedPlugin {
            library,
            create_bird,
            bird,
            versioned_path,
            version: new_version,
        });

        self.reload_count.fetch_add(1, Ordering::Relaxed);

        println!(
            "  Reloaded plugin_{:03} (v{}) - bird: {}",
            index,
            new_version,
            self.plugins[index].as_ref().unwrap().bird.name_string()
        );

        Ok(())
    }

    // ── Queries ─────────────────────────────────────────

    /// Get the BirdDefinition for a specific plugin.
    pub fn get_bird(&self, index: usize) -> Option<&BirdDefinition> {
        self.plugins
            .get(index)
            .and_then(|p| p.as_ref().map(|p| &p.bird))
    }

    /// Get PluginInfo for all loaded plugins.
    pub fn all_plugin_infos(&self) -> Vec<PluginInfo> {
        self.plugins
            .iter()
            .enumerate()
            .map(|(i, opt)| {
                if let Some(p) = opt {
                    PluginInfo {
                        index: i as u32,
                        name: format!("plugin_{:03}", i),
                        bird: p.bird,
                        loaded: true,
                    }
                } else {
                    PluginInfo {
                        index: i as u32,
                        name: format!("plugin_{:03}", i),
                        bird: BirdDefinition {
                            name: [0u8; 32],
                            color_r: 128,
                            color_g: 128,
                            color_b: 128,
                            size: 1.0,
                        },
                        loaded: false,
                    }
                }
            })
            .collect()
    }

    /// Total number of plugins (loaded + failed).
    pub fn total_plugins(&self) -> usize {
        self.plugins.len()
    }

    /// Number of successfully loaded plugins.
    pub fn loaded_count(&self) -> usize {
        self.plugins.iter().filter(|p| p.is_some()).count()
    }

    // ── Metrics ─────────────────────────────────────────

    /// Build the current metrics snapshot.
    pub fn metrics(&self) -> LoadMetrics {
        let mode_str = match self.mode {
            LoadMode::Sequential => "sequential",
            LoadMode::Parallel => "parallel",
            LoadMode::Lazy => "lazy",
        };
        LoadMetrics {
            mode: mode_str,
            total_load_ms: self.total_load_ms,
            total_io_ms: self.total_io_ms,
            total_symbol_ms: self.total_symbol_ms,
            total_init_ms: self.total_init_ms,
            per_plugin_ms: self.load_times_ms.clone(),
            loaded_count: self.loaded_count(),
            total_plugins: self.total_plugins(),
            reload_count: self.reload_count.load(Ordering::Relaxed),
            fps: self.current_fps,
        }
    }

    /// Update frame counter (call once per frame from update()).
    pub fn update_frame(&mut self) {
        self.frame_count += 1;
        let elapsed = self.last_frame_time.elapsed();
        if elapsed.as_secs_f64() >= 0.5 {
            self.current_fps = self.frame_count as f64 / elapsed.as_secs_f64();
            self.frame_count = 0;
            self.last_frame_time = Instant::now();
        }
    }

    // ── File watching ───────────────────────────────────

    /// Start watching plugin source directories for changes.
    ///
    /// When a `.rs` file in a plugin crate changes, the watcher will:
    /// 1. Rebuild that single plugin via `cargo build`.
    /// 2. Enqueue it for reloading on the next frame.
    pub fn start_watching(&mut self) {
        if self.watcher_active {
            return;
        }
        self.watcher_active = true;

        let plugin_source_dir = self.plugin_source_dir.clone();
        let reload_queue = self.reload_queue.clone();

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

            if let Err(e) = watcher.watch(&plugin_source_dir, RecursiveMode::Recursive) {
                eprintln!("Watcher failed to watch: {}", e);
                return;
            }

            println!("Watching for changes in: {}", plugin_source_dir.display());

            // Debounce: wait for a quiet period before triggering rebuild
            let mut pending_indices: Vec<usize> = Vec::new();
            let debounce_ms = 500;

            loop {
                match rx.recv_timeout(std::time::Duration::from_millis(debounce_ms)) {
                    Ok(event) => {
                        if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                            for path in &event.paths {
                                if let Some(index) = extract_plugin_index(path) {
                                    if !pending_indices.contains(&index) {
                                        pending_indices.push(index);
                                    }
                                }
                            }
                        }
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        // Debounce period elapsed — rebuild & enqueue
                        if !pending_indices.is_empty() {
                            pending_indices.sort();
                            pending_indices.dedup();

                            for &index in &pending_indices {
                                println!("  Rebuilding plugin_{:03}...", index);
                                rebuild_plugin(index, &plugin_source_dir);
                            }

                            // Enqueue for reload on next frame
                            if let Ok(mut queue) = reload_queue.lock() {
                                queue.extend(&pending_indices);
                            }

                            pending_indices.clear();
                        }
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }
        });
    }

    /// Process any queued reloads (call from update() each frame).
    pub fn process_reloads(&mut self) {
        let mut queue = self.reload_queue.lock().unwrap();
        let indices: Vec<usize> = queue.drain(..).collect();
        drop(queue);

        for index in indices {
            if let Err(e) = self.reload_plugin(index) {
                eprintln!("  Failed to reload plugin_{:03}: {}", index, e);
            }
        }
    }

    /// Whether the initial load has completed.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

// ── Core loading logic (used by both sequential and parallel paths) ──

/// Load a single DLL and return timing breakdown + the loaded plugin.
///
/// Returns: (io_ms, symbol_ms, init_ms, LoadedPlugin)
fn load_single_dll(
    index: usize,
    dll_path: &Path,
    target_dir: &Path,
) -> Result<(f64, f64, f64, LoadedPlugin), String> {
    let version = 1usize;
    let versioned_path = versioned_dll_path(target_dir, index, version);

    // ── Phase 1: I/O (file copy) ──
    let io_start = Instant::now();
    std::fs::copy(dll_path, &versioned_path).map_err(|e| format!("copy failed: {}", e))?;
    let io_ms = io_start.elapsed().as_secs_f64() * 1000.0;

    // ── Phase 2: Symbol resolution (LoadLibrary + GetProcAddress) ──
    let sym_start = Instant::now();
    let library = unsafe {
        libloading::Library::new(&versioned_path)
            .map_err(|e| format!("libloading failed: {}", e))?
    };
    let library = Arc::new(library);

    let create_bird: CreateBirdFn = {
        let symbol: libloading::Symbol<CreateBirdFn> = unsafe {
            library
                .get(b"create_bird")
                .map_err(|e| format!("symbol lookup failed: {}", e))?
        };
        *symbol
    };
    let sym_ms = sym_start.elapsed().as_secs_f64() * 1000.0;

    // ── Phase 3: Init (create_bird call — simulates module startup) ──
    let init_start = Instant::now();
    let bird = unsafe { (create_bird)() };
    let init_ms = init_start.elapsed().as_secs_f64() * 1000.0;

    Ok((
        io_ms,
        sym_ms,
        init_ms,
        LoadedPlugin {
            library,
            create_bird,
            bird,
            versioned_path,
            version,
        },
    ))
}

// ── Helpers ─────────────────────────────────────────────

/// Build a versioned DLL path: `plugins/target/release/plugin_042_v3.dll`
fn versioned_dll_path(base_dir: &Path, index: usize, version: usize) -> PathBuf {
    base_dir.join(format!("plugin_{:03}_v{}.dll", index, version))
}

/// Extract plugin index from a file path like `plugins/plugin_042/src/lib.rs`.
fn extract_plugin_index(path: &Path) -> Option<usize> {
    let path_str = path.to_string_lossy();
    for component in path_str.split(&['/', '\\']) {
        if let Some(rest) = component.strip_prefix("plugin_") {
            if let Ok(idx) = rest.parse::<usize>() {
                return Some(idx);
            }
        }
    }
    None
}

/// Rebuild a single plugin via `cargo build`.
fn rebuild_plugin(index: usize, plugin_source_dir: &Path) {
    let package = format!("plugin_{:03}", index);

    let output = std::process::Command::new("cargo")
        .args(["build", "--release", "-p", &package])
        .current_dir(plugin_source_dir)
        .output();

    match output {
        Ok(out) => {
            if out.status.success() {
                println!("    Build OK");
            } else {
                let stderr = String::from_utf8_lossy(&out.stderr);
                eprintln!("    Build FAILED:\n{}", stderr);
            }
        }
        Err(e) => {
            eprintln!("    Failed to run cargo: {}", e);
        }
    }
}
