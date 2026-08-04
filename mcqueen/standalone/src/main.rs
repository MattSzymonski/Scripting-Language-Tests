// standalone/src/main.rs — Single game crate + dll_patch hot-reload
//
// Watches game/src/ for .rs changes. On change: rebuilds game.dll,
// copies to a versioned name, and tells the engine to apply the new
// DLL as a dll_patch jump-table patch. The original game.dll stays
// loaded; only the function pointers in the jump table change.
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

// ── Helpers ───────────────────────────────────────────────────────────────

fn timestamp_now() -> String {
    let d = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let s = d.as_secs();
    let ms = d.subsec_millis();
    format!(
        "{:02}:{:02}:{:02}.{:03}",
        (s / 3600) % 24,
        (s / 60) % 60,
        s % 60,
        ms
    )
}

struct PendingPatch {
    path: PathBuf,
    started_at: Instant,
}

// ── Watcher ───────────────────────────────────────────────────────────────

fn spawn_watcher(
    reload_flag: Arc<AtomicBool>,
    pending: Arc<Mutex<Option<PendingPatch>>>,
    workspace_root: PathBuf,
) {
    std::thread::spawn(move || {
        let watch_dir = workspace_root.join("game").join("src");
        let build_dir = workspace_root.join("target").join("debug");

        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(e) = res {
                    let is_rs = e
                        .paths
                        .iter()
                        .any(|p| p.extension().map(|x| x == "rs").unwrap_or(false));
                    if is_rs && matches!(e.kind, EventKind::Modify(_)) {
                        let _ = tx.send(());
                    }
                }
            },
            Config::default(),
        )
        .expect("watcher");

        watcher
            .watch(&watch_dir, RecursiveMode::Recursive)
            .expect("watch");
        println!("[watcher] monitoring {}\n", watch_dir.display());

        let mut n: u32 = 0;
        while let Ok(()) = rx.recv() {
            std::thread::sleep(Duration::from_millis(400));
            n += 1;
            let started = Instant::now();
            println!("[#{n}] {} | rebuild started", timestamp_now());

            let ok = Command::new("cargo")
                .args(["build", "-p", "game"])
                .current_dir(&workspace_root)
                .status()
                .map(|s| s.success())
                .unwrap_or(false);

            if !ok {
                eprintln!("[#{n}] build failed");
                continue;
            }

            let src = build_dir.join("game.dll");
            let dst = build_dir.join(format!("game_v{n}.dll"));
            if std::fs::copy(&src, &dst).is_err() {
                continue;
            }

            *pending.lock().unwrap() = Some(PendingPatch {
                path: dst,
                started_at: started,
            });
            reload_flag.store(true, Ordering::Release);
            while rx.try_recv().is_ok() {}
        }
    });
}

// ── Window ────────────────────────────────────────────────────────────────

fn window_conf() -> Conf {
    Conf {
        window_title: "mcqueen — single game crate + dll_patch".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

// ── Main ──────────────────────────────────────────────────────────────────

#[macroquad::main(window_conf)]
async fn main() {
    let ws = {
        let mut p: PathBuf = std::env::current_exe().unwrap();
        p.pop(); // exe
        p.pop(); // debug
        p.pop(); // target
        p
    };
    let build = ws.join("target").join("debug");

    // Initial build
    println!("[init] building game.dll...");
    if !Command::new("cargo")
        .args(["build", "-p", "game"])
        .current_dir(&ws)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        eprintln!("[init] build failed");
        return;
    }

    // Copy to versioned name (never load game.dll directly)
    let base = build.join("game_base.dll");
    std::fs::copy(build.join("game.dll"), &base).expect("copy");
    let mut engine = Engine::new(&base);

    println!("[init] ready. SPACE=bounce. Edit game/src/lib.rs to hot-patch.\n");

    let flag = Arc::new(AtomicBool::new(false));
    let pending: Arc<Mutex<Option<PendingPatch>>> = Arc::new(Mutex::new(None));
    spawn_watcher(Arc::clone(&flag), Arc::clone(&pending), ws);

    loop {
        let dt = get_frame_time();
        if is_key_pressed(KeyCode::Space) {
            engine.start();
        }
        if flag.swap(false, Ordering::Acquire) {
            if let Some(p) = pending.lock().unwrap().take() {
                match engine.apply_patch(&p.path) {
                    Ok(()) => println!(
                        "[main] {} | patched — {} ms\n",
                        timestamp_now(),
                        p.started_at.elapsed().as_millis()
                    ),
                    Err(e) => eprintln!("[main] patch failed: {e}\n"),
                }
            }
        }
        engine.update(dt);
        next_frame().await;
    }
}
