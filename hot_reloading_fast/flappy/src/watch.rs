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

    const DEBOUNCE: Duration = Duration::from_millis(300);

    std::thread::spawn(move || loop {
        // Block until the first event of a new burst arrives.
        if rx.recv().is_err() {
            break;
        }
        // Quiet-period debounce: keep resetting the wait as long as more
        // events keep arriving (e.g. a raw save followed by a delayed
        // format-on-save rewrite); only fire once the stream has been
        // quiet for DEBOUNCE.
        while rx.recv_timeout(DEBOUNCE).is_ok() {}
        on_change();
    });

    Ok(watcher)
}
