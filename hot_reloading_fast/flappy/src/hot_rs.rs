//! Hot-reload support for the pure-Rust game logic (`game_rs`).
//!
//! Trimmed adaptation of `live_patching/orchestrator`'s build/load/patch loop
//! and `live_engine::hot_table::HotFnTable`, aimed at `game_rs` instead of
//! `game_hot`. See that crate for the fuller write-up of the technique.
//!
//! Only compiled for the default (non-`cs`) path.

use std::ffi::c_void;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicPtr, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Current wall-clock time as `HH:MM:SS.mmm`, for timestamping reload log lines.
fn now() -> String {
    chrono::Local::now().format("%H:%M:%S%.3f").to_string()
}

use libloading::{Library, Symbol};

use crate::ffi::EngineApi;

pub type GameWorldSizeFn = extern "C" fn() -> usize;
pub type GameInitFn = extern "C" fn(*const EngineApi, *mut c_void);
pub type GameUpdateFn = extern "C" fn(*const EngineApi, *mut c_void, f32);
pub type GameDrawFn = extern "C" fn(*const EngineApi, *mut c_void);

// ---------------------------------------------------------------------------
// Hot function table — the engine calls through this exclusively.
// ---------------------------------------------------------------------------

pub struct HotFnTable {
    world_size: AtomicPtr<()>,
    init: AtomicPtr<()>,
    update: AtomicPtr<()>,
    draw: AtomicPtr<()>,
}

impl HotFnTable {
    fn new() -> Self {
        Self {
            world_size: AtomicPtr::new(std::ptr::null_mut()),
            init: AtomicPtr::new(std::ptr::null_mut()),
            update: AtomicPtr::new(std::ptr::null_mut()),
            draw: AtomicPtr::new(std::ptr::null_mut()),
        }
    }

    fn patch(&self, game: &LoadedGame) {
        self.world_size.store(game.world_size as *mut (), Ordering::Release);
        self.init.store(game.init as *mut (), Ordering::Release);
        self.update.store(game.update as *mut (), Ordering::Release);
        self.draw.store(game.draw as *mut (), Ordering::Release);
    }

    pub fn read_world_size(&self) -> GameWorldSizeFn {
        let ptr = self.world_size.load(Ordering::Acquire);
        if ptr.is_null() {
            stub_world_size
        } else {
            unsafe { std::mem::transmute(ptr) }
        }
    }

    pub fn read_init(&self) -> GameInitFn {
        let ptr = self.init.load(Ordering::Acquire);
        if ptr.is_null() {
            stub_init
        } else {
            unsafe { std::mem::transmute(ptr) }
        }
    }

    pub fn read_update(&self) -> GameUpdateFn {
        let ptr = self.update.load(Ordering::Acquire);
        if ptr.is_null() {
            stub_update
        } else {
            unsafe { std::mem::transmute(ptr) }
        }
    }

    pub fn read_draw(&self) -> GameDrawFn {
        let ptr = self.draw.load(Ordering::Acquire);
        if ptr.is_null() {
            stub_draw
        } else {
            unsafe { std::mem::transmute(ptr) }
        }
    }
}

extern "C" fn stub_world_size() -> usize {
    0
}
extern "C" fn stub_init(_api: *const EngineApi, _world: *mut c_void) {}
extern "C" fn stub_update(_api: *const EngineApi, _world: *mut c_void, _dt: f32) {}
extern "C" fn stub_draw(_api: *const EngineApi, _world: *mut c_void) {}

// ---------------------------------------------------------------------------
// Build + load
// ---------------------------------------------------------------------------

static PATCH_VERSION: AtomicU32 = AtomicU32::new(1);

/// A unique library filename per patch, so the OS loader (and Windows' file
/// lock on the previous copy) never gets confused about which version is
/// which.
fn versioned_lib_name(base_dir: &Path) -> PathBuf {
    let ver = PATCH_VERSION.fetch_add(1, Ordering::Relaxed);
    base_dir.join(format!("game_rs_v{ver}.dll"))
}

fn workspace_dir() -> PathBuf {
    // flappy's manifest dir is `hot_reloading/flappy`; the workspace root
    // (where `cargo build -p game_rs` should run) is its parent.
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn build_game_lib(release: bool) -> Result<PathBuf, String> {
    let workspace = workspace_dir();
    let target_dir = workspace.join("target");

    let mut cmd = Command::new("cargo");
    cmd.args(["build", "-p", "game_rs"])
        .arg("--target-dir")
        .arg(&target_dir)
        .current_dir(&workspace);
    if release {
        cmd.arg("--release");
    }

    let output = cmd.output().map_err(|e| format!("cargo build failed: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "cargo build -p game_rs failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let profile = if release { "release" } else { "debug" };
    Ok(target_dir.join(profile).join("game_rs.dll"))
}

struct LoadedGame {
    lib: Library,
    world_size: GameWorldSizeFn,
    init: GameInitFn,
    update: GameUpdateFn,
    draw: GameDrawFn,
}

fn load_game(lib_path: &Path) -> Result<LoadedGame, Box<dyn std::error::Error>> {
    // Copy to a versioned name to avoid Windows DLL file-locking/caching.
    let versioned = versioned_lib_name(lib_path.parent().unwrap());
    std::fs::copy(lib_path, &versioned)?;

    // SAFETY: we trust the game_rs library (it's our own code, built
    // moments ago from the same workspace).
    let lib = unsafe { Library::new(&versioned)? };

    // NOTE: these names must match the `#[no_mangle]` names in
    // `game_rs/src/lib.rs` exactly.
    let (world_size, init, update, draw) = unsafe {
        let world_size: Symbol<GameWorldSizeFn> = lib.get(b"game_world_size")?;
        let init: Symbol<GameInitFn> = lib.get(b"game_init")?;
        let update: Symbol<GameUpdateFn> = lib.get(b"game_update")?;
        let draw: Symbol<GameDrawFn> = lib.get(b"game_draw")?;
        (*world_size, *init, *update, *draw)
    };

    Ok(LoadedGame { lib, world_size, init, update, draw })
}

fn build_and_load() -> Result<LoadedGame, String> {
    let lib_path = build_game_lib(false)?;
    load_game(&lib_path).map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Keeps the hot table alive, plus everything that must outlive the running
/// game (the file watcher and every loaded library, so in-flight calls into
/// an "old" version never jump into unmapped memory).
pub struct HotGame {
    pub table: Arc<HotFnTable>,
    _old_libraries: Arc<Mutex<Vec<Library>>>,
    _watcher: notify::RecommendedWatcher,
}

/// Build `game_rs` once (blocking), start watching its sources, and
/// return the shared table the running scene reads from every frame.
pub fn start() -> HotGame {
    let table = Arc::new(HotFnTable::new());
    let old_libraries: Arc<Mutex<Vec<Library>>> = Arc::new(Mutex::new(Vec::new()));

    let start = Instant::now();
    match build_and_load() {
        Ok(game) => {
            table.patch(&game);
            old_libraries.lock().unwrap().push(game.lib);
            println!(
                "[hot] game_rs loaded (v{}) in {:.2?}",
                PATCH_VERSION.load(Ordering::Relaxed),
                start.elapsed()
            );
        }
        Err(e) => eprintln!("[hot] initial build of game_rs failed:\n{e}"),
    }

    let watch_dir = workspace_dir().join("game_rs").join("src");
    let table_for_watch = table.clone();
    let old_libraries_for_watch = old_libraries.clone();

    let watcher = crate::watch::spawn(watch_dir, "rs", move || {
        println!("[hot] [{}] change detected — rebuilding game_rs...", now());
        let start = Instant::now();
        match build_and_load() {
            Ok(game) => {
                table_for_watch.patch(&game);
                old_libraries_for_watch.lock().unwrap().push(game.lib);
                println!(
                    "[hot] [{}] PATCHED (v{}) in {:.2?}",
                    now(),
                    PATCH_VERSION.load(Ordering::Relaxed),
                    start.elapsed()
                );
            }
            Err(e) => eprintln!(
                "[hot] [{}] rebuild failed (after {:.2?}):\n{e}",
                now(),
                start.elapsed()
            ),
        }
    })
    .expect("failed to start game_rs watcher");

    HotGame {
        table,
        _old_libraries: old_libraries,
        _watcher: watcher,
    }
}
