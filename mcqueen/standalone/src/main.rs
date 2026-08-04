// standalone/src/main.rs — Window, watcher, and hot-reload orchestrator
//
// REQUIREMENTS
//   mini_engine (path = "../engine"), macroquad 0.4, notify 6
//
// DESCRIPTION
//   Creates a macroquad window. Watches the game crate source directory
//   for changes. When a .rs file is modified, runs `cargo build -p game`
//   and reloads the resulting DLL into the engine — all without restarting
//   the window. Press SPACE to launch/reset the bouncing ball.
//
// USAGE
//   cargo run -p standalone
//     Then edit game/src/lib.rs (e.g. change GRAVITY or BOUNCE_VELOCITY_Y).
//     The watcher rebuilds and reloads automatically.
//
// EXAMPLE USAGE
//   cargo run -p standalone
//
// --- SCRIPT ---

use macroquad::prelude::*;
use mini_engine::Engine;
use notify::event::EventKind;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Basename of the game DLL that cargo produces.
const GAME_DLL_NAME: &str = "game.dll";

/// How long to wait after the last file-change event before triggering a
/// rebuild (debounce — avoids rebuilding mid-edit).
const DEBOUNCE_MS: u64 = 400;

// ── Helpers ───────────────────────────────────────────────────────────────

/// Wall-clock timestamp with millisecond precision (UTC).
fn timestamp_now() -> String {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let total_seconds = duration.as_secs();
    let milliseconds = duration.subsec_millis();
    let hours = (total_seconds / 3600) % 24;
    let minutes = (total_seconds / 60) % 60;
    let seconds = total_seconds % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}.{milliseconds:03}")
}

/// Data passed from the watcher thread to the main thread when a new
/// DLL is ready to be loaded.
struct PendingReload {
    path: PathBuf,
    started_at: Instant,
}

// ── File Watcher ──────────────────────────────────────────────────────────

/// Spawn a background thread that watches `game/src/` for .rs changes,
/// rebuilds via cargo, copies to a versioned name, and communicates the
/// new DLL path back to the main thread via `pending_reload_path`.
fn spawn_watcher(
    reload_flag: Arc<AtomicBool>,
    pending_reload: Arc<Mutex<Option<PendingReload>>>,
    workspace_root: PathBuf,
) {
    std::thread::spawn(move || {
        let watch_dir = workspace_root.join("game").join("src");
        let build_dir = workspace_root.join("target").join("debug");

        let (transmitter, receiver) = std::sync::mpsc::channel();

        let mut watcher = RecommendedWatcher::new(
            move |result: Result<Event, notify::Error>| {
                if let Ok(event) = result {
                    if matches!(event.kind, EventKind::Modify(_)) {
                        let _ = transmitter.send(());
                    }
                }
            },
            Config::default(),
        )
        .expect("create file watcher");

        watcher
            .watch(&watch_dir, RecursiveMode::Recursive)
            .expect("watch game/src/ directory");

        println!(
            "[watcher] monitoring {} for changes...\n",
            watch_dir.display()
        );

        let mut reload_counter: u32 = 0;

        for () in receiver {
            // Debounce: wait for edits to settle
            std::thread::sleep(Duration::from_millis(DEBOUNCE_MS));

            reload_counter += 1;
            let started_at = Instant::now();
            println!(
                "[watcher #{reload_counter}] {} | rebuild started",
                timestamp_now()
            );

            // Recompile the game crate
            let status = Command::new("cargo")
                .args(["build", "-p", "game"])
                .current_dir(&workspace_root)
                .status()
                .expect("spawn cargo build");

            if !status.success() {
                eprintln!(
                    "[watcher #{reload_counter}] cargo build failed — check compiler errors above"
                );
                continue;
            }

            // Copy the fresh DLL to a versioned name to avoid Windows file-lock
            // issues when the old one is still mapped.
            let source_dll = build_dir.join(GAME_DLL_NAME);
            let versioned_name = format!("game_v{reload_counter}.dll");
            let destination_dll = build_dir.join(&versioned_name);
            if let Err(error) = std::fs::copy(&source_dll, &destination_dll) {
                eprintln!("[watcher #{reload_counter}] copy failed: {error}");
                continue;
            }

            // Communicate the new path and start time to the main thread
            *pending_reload.lock().unwrap() = Some(PendingReload {
                path: destination_dll.clone(),
                started_at,
            });
            reload_flag.store(true, Ordering::Release);
        }
    });
}

// ── Window Configuration ──────────────────────────────────────────────────

fn window_configuration() -> Conf {
    Conf {
        window_title: "mcqueen — Bouncing Ball (edit game/src/lib.rs to hot-reload)".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

// ── Main ──────────────────────────────────────────────────────────────────

#[macroquad::main(window_configuration)]
async fn main() {
    // Locate the workspace root (two levels up from standalone crate)
    let workspace_root = {
        let executable_path = std::env::current_exe().unwrap();
        // Navigate from target/debug/standalone.exe up to workspace root
        let mut path: PathBuf = executable_path;
        path.pop(); // standalone.exe
        path.pop(); // debug
        path.pop(); // target
        path
    };

    let build_dir = workspace_root.join("target").join("debug");

    // Initial build of the game DLL
    println!("[init] building game DLL...");
    let status = Command::new("cargo")
        .args(["build", "-p", "game"])
        .current_dir(&workspace_root)
        .status()
        .expect("spawn cargo build");
    if !status.success() {
        eprintln!("[init] initial cargo build failed — is the game crate valid?");
        return;
    }

    // Copy to a versioned name so we NEVER load game.dll directly.
    // Loading game.dll directly would lock it, preventing cargo from
    // overwriting it on the next rebuild.
    let initial_versioned = build_dir.join("game_v0.dll");
    let source_dll = build_dir.join(GAME_DLL_NAME);
    std::fs::copy(&source_dll, &initial_versioned)
        .expect("copy game.dll to game_v0.dll");

    // Track pending reload data from the watcher thread
    let pending_reload: Arc<Mutex<Option<PendingReload>>> = Arc::new(Mutex::new(None));

    // Create the engine (loads game_v0.dll, leaving game.dll unlocked)
    let mut engine = Engine::new(&initial_versioned);
    println!("[init] engine ready. Press SPACE to bounce the ball!\n");

    // Shared flag: watcher sets to true when a new DLL is ready
    let reload_flag = Arc::new(AtomicBool::new(false));

    // Spawn the background file watcher
    spawn_watcher(
        Arc::clone(&reload_flag),
        Arc::clone(&pending_reload),
        workspace_root.clone(),
    );

    // ── Main Loop ──────────────────────────────────────────────────────
    loop {
        let delta_time = get_frame_time();

        // Check for SPACE press -> launch/reset the ball
        if is_key_pressed(KeyCode::Space) {
            engine.start();
        }

        // Check for hot-reload signal from the watcher thread
        if reload_flag.swap(false, Ordering::Acquire) {
            let pending = pending_reload.lock().unwrap().take();
            if let Some(reload) = pending {
                match engine.reload_game(&reload.path) {
                    Ok(()) => {
                        let elapsed_ms = reload.started_at.elapsed().as_millis();
                        println!(
                            "[main] {} | reload complete — {} ms total\n",
                            timestamp_now(),
                            elapsed_ms
                        );
                    }
                    Err(error) => eprintln!("[main] reload failed: {error}\n"),
                }
            }
        }

        // Advance game + render
        engine.update(delta_time);

        next_frame().await;
    }
}
