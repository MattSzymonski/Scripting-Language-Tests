// REQUIREMENTS
//   Rust (stable), Windows (prologue patching; other OSes fall back to a
//   pointer swap).  Run from the workspace root:
//     cargo run -p standalone
//
// DESCRIPTION
//   Live Coding host for the mini engine.  Statically links the engine (which
//   owns the macroquad window + render loop) and drives the reusable
//   `live_coding` crate (LiveCodeSession): it loads the `project` cdylib
//   ONCE, hands it the engine's API table, and watches the project source.
//   When you edit and save:
//
//     - If ONLY a hot function's body changed, that one function is
//       recompiled standalone via rustc (~200 ms) and its PROLOGUE in the
//       loaded base DLL is patched in place (`E9 rel32`) - true Live Coding.
//       The base DLL stays loaded and the update pointer never changes.
//     - Anything else (settings, structs, other functions, Cargo.toml)
//       triggers a full cargo rebuild; the fresh DLL is loaded alongside and
//       the base prologue is re-patched to it.
//
//   The window, the engine and the project update pointer never change - only
//   the prologue bytes do.
//
//   This binary is deliberately thin: all of the source analysis, change
//   detection, patch-module generation and prologue patching live in the
//   `live_coding` crate, so the same Live Coding machinery can be embedded in
//   any other host (a game, a tool, a test harness) without the mini engine.
//
// USAGE / EXAMPLE USAGE
//   cargo run -p standalone        # from live_coding_mini_engine/
//   Then edit project/src/lib.rs and save.  Press Escape to quit.
//
// --- SCRIPT ---

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use live_coding::{ChangeOutcome, LiveCodeSession, LiveCodeSessionConfig};
use mini_engine::MiniEngine;
use notify::Watcher;

#[macroquad::main("live-coding mini engine")]
async fn main() {
    let engine = Arc::new(MiniEngine::new());
    // Seed the API's screen-size cache now (main thread, window exists) so the
    // initial project_init sees it.
    engine.update_screen_size();

    // Configure the live-coding session for the mini-engine project layout and
    // hand it the engine's API table (a pointer into the DLLs' project_set_api).
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let api = engine.project_api();
    let config = LiveCodeSessionConfig::mini_engine(&workspace, &api as *const _ as *const ());
    let mut session = LiveCodeSession::new(config).expect("invalid live_coding configuration");

    // Initial build + load; point the engine at the base project_update.
    match session.load_initial() {
        Ok(update_address) => engine.set_project_update(update_address as *mut ()),
        Err(errors) => eprintln!(
            "{} initial build failed:\n{}",
            live_coding::style::hot_prefix(),
            errors
        ),
    }

    // A notify watcher that pushes the changed file paths through a channel.
    // All patching happens on the main thread (drained from the channel), so
    // there is never a concurrent executor inside the function being patched.
    let (change_tx, change_rx) = std::sync::mpsc::channel::<Vec<PathBuf>>();
    let watch_dir = workspace.join("project");
    let mut watcher = notify::recommended_watcher(
        move |result: Result<notify::Event, notify::Error>| {
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
    .expect("failed to create file watcher");
    watcher
        .configure(notify::Config::default().with_poll_interval(Duration::from_millis(500)))
        .expect("failed to configure file watcher");
    watcher
        .watch(&watch_dir, notify::RecursiveMode::Recursive)
        .expect("failed to watch project directory");
    println!(
        "{} watching {} for source changes...",
        live_coding::style::paint_bright_cyan("[watch]"),
        watch_dir.display()
    );

    // Render loop with a per-frame hook that drains change events and
    // live-patches (all on the main thread).
    engine
        .run(move |engine_ref, _delta_time| {
            if let Ok(changed_paths) = change_rx.try_recv() {
                // Merge any extra events from the same save burst.
                let mut all_paths = changed_paths;
                while let Ok(more_paths) = change_rx.try_recv() {
                    for path in more_paths {
                        if !all_paths.contains(&path) {
                            all_paths.push(path);
                        }
                    }
                }
                // Let the editor finish writing before reading the file.
                std::thread::sleep(Duration::from_millis(150));

                match session.handle_change(&all_paths) {
                    // Only a failed base re-patch ever needs the engine to
                    // repoint; every other outcome leaves the pointer valid.
                    Ok(ChangeOutcome::FullReloadNeedsPointerSwap { update_address }) => {
                        engine_ref.set_project_update(update_address as *mut ());
                    }
                    Ok(_) => {}
                    Err(errors) => eprintln!(
                        "{} change handling failed - keeping previous code running:\n{}",
                        live_coding::style::hot_prefix(),
                        errors
                    ),
                }
            }
        })
        .await;
}
