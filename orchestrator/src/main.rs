//! # Orchestrator — the live-patching controller
//!
//! This binary is the "main program". It:
//!
//! 1. Compiles `game_hot` as a cdylib
//! 2. Loads the library and resolves hot function symbols
//! 3. Injects them into the engine's [`HotFnTable`]
//! 4. Starts the engine loop (which calls through the hot table)
//! 5. Watches `game_hot/src/` for file changes
//! 6. On change: recompiles, reloads, repatches — all while the engine runs
//!
//! ## Architecture overview
//!
//! ```text
//!                    ┌──────────────────┐
//!                    │   Orchestrator   │
//!                    │                  │
//!   game_hot/src/ ──▶│ file watcher ────┤ (notify)
//!   (change)         │     │            │
//!                    │     ▼            │
//!                    │ cargo build ─────┤ (new .dll)
//!                    │     │            │
//!                    │     ▼            │
//!                    │ libloading ──────┤ (resolve symbols)
//!                    │     │            │
//!                    │     ▼            │
//!                    │ HotFnTable ──────┤ (atomic swap)
//!                    │   .patch_*()     │
//!                    └──────┬───────────┘
//!                           │ shared Arc<HotFnTable>
//!                    ┌──────▼───────────┐
//!                    │  Engine::run()   │
//!                    │  reads hot table │
//!                    │  every frame     │
//!                    └──────────────────┘
//! ```

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use libloading::Library;
use live_engine::api::{GameInitFn, GameRenderFn, GameUpdateFn, GameWorldSizeFn};
use live_engine::engine;
use live_engine::hot_table::HotFnTable;
use macroquad::prelude::*;
use notify::{Config, Event, EventKind, RecursiveMode, Watcher};

// ---------------------------------------------------------------------------
// Patch version counter — increments each time we build & reload
// ---------------------------------------------------------------------------

static PATCH_VERSION: AtomicU32 = AtomicU32::new(1);

/// Generate a unique library name for each patch so the OS loader doesn't
/// cache the old one.
fn versioned_lib_name(base_dir: &Path) -> PathBuf {
    let ver = PATCH_VERSION.fetch_add(1, Ordering::Relaxed);
    base_dir.join(format!("game_hot_v{}.dll", ver))
}

// ---------------------------------------------------------------------------
// Build step — compile game_hot as a cdylib
// ---------------------------------------------------------------------------

fn build_game_lib(release: bool) -> Result<PathBuf, String> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let target_dir = manifest_dir.join("..").join("target");

    let mut cmd = Command::new("cargo");
    cmd.args(["build", "-p", "game_hot"])
        .arg("--target-dir")
        .arg(&target_dir)
        .current_dir(&manifest_dir.join(".."));

    if release {
        cmd.arg("--release");
    }

    let output = cmd.output().map_err(|e| format!("cargo build failed: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("cargo build failed:\n{stderr}"));
    }

    let profile = if release { "release" } else { "debug" };
    let lib_path = target_dir.join(profile).join("game_hot.dll");

    Ok(lib_path)
}

// ---------------------------------------------------------------------------
// Load step — open the cdylib and resolve hot function symbols
// ---------------------------------------------------------------------------

struct LoadedGame {
    /// We keep the `Library` alive so the function pointers remain valid.
    /// Old libraries are kept in `old_libraries` for safety.
    lib: Library,
    init: GameInitFn,
    update: GameUpdateFn,
    render: GameRenderFn,
    world_size: GameWorldSizeFn,
}

/// We hold old libraries here so their code stays mapped while in-flight
/// calls may still be using them.
type OldLibrary = Library;

fn load_game(lib_path: &Path) -> Result<LoadedGame, Box<dyn std::error::Error>> {
    // Copy to a versioned name to avoid Windows DLL caching issues.
    let versioned = versioned_lib_name(lib_path.parent().unwrap());
    std::fs::copy(lib_path, &versioned)?;

    // SAFETY: we trust the game_hot library (it's our own code).
    let lib = unsafe { Library::new(&versioned)? };

    // Resolve symbols. We extract raw function pointers (Copy types) before
    // the Symbol borrows drop, so we can move `lib` into LoadedGame.
    //
    // NOTE: These names must match the `#[no_mangle] pub extern "C" fn`
    // names in game_hot/src/lib.rs exactly.
    let init_ptr: GameInitFn = {
        let sym: libloading::Symbol<GameInitFn> = unsafe { lib.get(b"game_init")? };
        *sym  // Copy the function pointer value
    };
    let update_ptr: GameUpdateFn = {
        let sym: libloading::Symbol<GameUpdateFn> = unsafe { lib.get(b"game_update")? };
        *sym
    };
    let render_ptr: GameRenderFn = {
        let sym: libloading::Symbol<GameRenderFn> = unsafe { lib.get(b"game_render")? };
        *sym
    };
    let world_size_ptr: GameWorldSizeFn = {
        let sym: libloading::Symbol<GameWorldSizeFn> = unsafe { lib.get(b"game_world_size")? };
        *sym
    };

    Ok(LoadedGame {
        lib,
        init: init_ptr,
        update: update_ptr,
        render: render_ptr,
        world_size: world_size_ptr,
    })
}

// ---------------------------------------------------------------------------
// Patch step — atomically swap function pointers in the hot table
// ---------------------------------------------------------------------------

unsafe fn patch_game(hot_table: &HotFnTable, game: &LoadedGame) {
    // Each swap returns the old function pointer. We don't need it here
    // because the old library is kept in `old_libraries` (the pointers
    // into it remain valid as long as the Library lives).

    let _old_init = hot_table.patch_init(game.init);
    let _old_update = hot_table.patch_update(game.update);
    let _old_render = hot_table.patch_render(game.render);
    let _old_world_size = hot_table.patch_world_size(game.world_size);

    println!(
        "[orchestrator] PATCHED — hot table updated (v{})",
        PATCH_VERSION.load(Ordering::Relaxed)
    );
}

// ---------------------------------------------------------------------------
// File watcher — detect changes to game_hot source files
// ---------------------------------------------------------------------------

fn watch_game_sources(
    tx: std::sync::mpsc::Sender<()>,
) -> Result<notify::RecommendedWatcher, notify::Error> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let game_src = manifest_dir.join("..").join("game_hot").join("src");

    let mut watcher = notify::recommended_watcher(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                match event.kind {
                    EventKind::Modify(_) | EventKind::Create(_) => {
                        // Only trigger on .rs files.
                        if event.paths.iter().any(|p| {
                            p.extension().map_or(false, |e| e == "rs")
                        }) {
                            let _ = tx.send(());
                        }
                    }
                    _ => {}
                }
            }
        },
    )?;

    watcher.configure(Config::default().with_poll_interval(Duration::from_millis(500)))?;
    watcher.watch(&game_src, RecursiveMode::NonRecursive)?;

    println!("[orchestrator] Watching {} for changes...", game_src.display());
    Ok(watcher)
}

// ---------------------------------------------------------------------------
// Hot-reload loop — runs in a background thread while the engine spins
// ---------------------------------------------------------------------------

fn hot_reload_loop(
    hot_table: Arc<HotFnTable>,
    reload_rx: std::sync::mpsc::Receiver<()>,
    old_libraries: Arc<std::sync::Mutex<Vec<OldLibrary>>>,
) {
    // Use a simple debounce: wait for a burst of events to settle.
    loop {
        // Wait for the first change event.
        if reload_rx.recv().is_err() {
            break;
        }

        // Drain any additional events that arrived in quick succession.
        while reload_rx.try_recv().is_ok() {}

        // Small cooldown to let the editor finish writing.
        std::thread::sleep(Duration::from_millis(300));

        println!("[orchestrator] Change detected — rebuilding game_hot...");

        match build_game_lib(false) {
            Ok(lib_path) => match load_game(&lib_path) {
                Ok(game) => {
                    // SAFETY: the hot table is safe to patch from any thread;
                    // the engine reads it with Acquire, we write with Release.
                    unsafe { patch_game(&hot_table, &game); }

                    // Keep the old library alive (code may still be executing).
                    // The newly loaded library will also be kept alive by the
                    // `old_libraries` list on the next patch cycle.
                    let mut old = old_libraries.lock().unwrap();
                    old.push(game.lib); // This is the *new* lib; it will become "old" next patch.

                    println!(
                        "[orchestrator] {} old libraries retained",
                        old.len()
                    );
                }
                Err(e) => {
                    eprintln!("[orchestrator] Failed to load game lib: {e}");
                }
            },
            Err(e) => {
                eprintln!("[orchestrator] Build failed:\n{e}");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Main — the orchestrator entry point
// ---------------------------------------------------------------------------

fn window_conf() -> Conf {
    Conf {
        window_title: "Live-Patching Engine Prototype".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("=== Live-Patching Engine Prototype ===");
    println!();
    println!("Try editing game_hot/src/lib.rs while this runs!");
    println!("The game will hot-reload without restarting.");
    println!();

    // ---- Phase 1: Initial build ----
    println!("[orchestrator] Building game_hot...");
    let lib_path = build_game_lib(false).expect("initial build failed");
    println!("[orchestrator] Built: {}", lib_path.display());

    // ---- Phase 2: Load game functions ----
    let game = load_game(&lib_path).expect("failed to load game library");

    // ---- Phase 3: Set up hot table ----
    let hot_table = Arc::new(HotFnTable::new());

    // Inject initial function pointers.
    //
    // SAFETY: No engine is running yet, so no concurrent access.
    unsafe { patch_game(&hot_table, &game); }

    // Keep track of loaded libraries so old code stays mapped.
    let old_libraries = Arc::new(std::sync::Mutex::new(vec![game.lib]));

    // ---- Phase 4: Start file watcher ----
    let (reload_tx, reload_rx) = std::sync::mpsc::channel();
    let _watcher = watch_game_sources(reload_tx)
        .expect("failed to start file watcher");

    // ---- Phase 5: Start hot-reload background thread ----
    let hot_table_clone = Arc::clone(&hot_table);
    let old_libs_clone = Arc::clone(&old_libraries);
    std::thread::spawn(move || {
        hot_reload_loop(hot_table_clone, reload_rx, old_libs_clone);
    });

    // ---- Phase 6: Run the engine (blocks until quit) ----
    //
    // The engine reads from `hot_table` every frame. The background thread
    // patches it whenever game code changes. No locks, no restarts.
    engine::run(hot_table).await;

    println!("[orchestrator] Engine stopped. Goodbye.");
}
