//! Small reusable "watch a directory for source changes, debounce, then call
//! a closure on a background thread" helper, shared by the `hot` (Rust) and
//! `cs` (C#) features. Mirrors the watch+debounce loop in
//! `live_patching/orchestrator/src/main.rs`, generalized over the file
//! extension and the reaction to a change.

use std::path::PathBuf;
use std::time::Duration;

use notify::{Config, Event, EventKind, RecursiveMode, Watcher};

/// Watch `dir` (recursively) for created/modified files ending in
/// `.{extension}` and invoke `on_change` on a background thread after a burst
/// of events settles. Keep the returned watcher alive for as long as you want
/// to keep watching — dropping it stops the watch.
pub fn spawn(
    dir: PathBuf,
    extension: &'static str,
    on_change: impl Fn() + Send + 'static,
) -> notify::Result<notify::RecommendedWatcher> {
    let (tx, rx) = std::sync::mpsc::channel::<()>();

    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            let is_relevant = matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_))
                && event
                    .paths
                    .iter()
                    .any(|p| p.extension().map_or(false, |e| e == extension));
            if is_relevant {
                let _ = tx.send(());
            }
        }
    })?;

    watcher.configure(Config::default().with_poll_interval(Duration::from_millis(500)))?;
    watcher.watch(&dir, RecursiveMode::Recursive)?;

    println!("[watch] watching {} for *.{extension} changes...", dir.display());

    std::thread::spawn(move || loop {
        if rx.recv().is_err() {
            break;
        }
        // Debounce: drain a burst of events, then let the editor finish writing.
        while rx.try_recv().is_ok() {}
        std::thread::sleep(Duration::from_millis(300));
        on_change();
    });

    Ok(watcher)
}
