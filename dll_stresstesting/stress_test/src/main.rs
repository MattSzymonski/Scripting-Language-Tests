//! DLL Stress Test — Main Entry Point
//!
//! Loads 300 bird plugin DLLs, renders them on a menu screen, and measures
//! DLL linking/loading performance. Supports three load modes:
//!
//!   cargo run -p stress_test --release                  # sequential (default)
//!   cargo run -p stress_test --release -- --parallel     # parallel loading
//!   cargo run -p stress_test --release -- --lazy         # lazy/deferred

mod bird_menu;
mod ffi;
mod plugin_loader;

use bird_menu::BirdMenuScene;
use mini_engine::Engine;
use plugin_loader::{LoadMode, PluginManager};
use std::path::PathBuf;

/// Resolve the project root directory relative to the executable.
fn project_root() -> PathBuf {
    // When running with `cargo run -p stress_test`, the working directory
    // is the workspace root (dll_stresstesting/).
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    // If we're running from the stress_test/ subdirectory, go up one level.
    if cwd.ends_with("stress_test") {
        cwd.parent().unwrap_or(&cwd).to_path_buf()
    } else {
        cwd
    }
}

#[macroquad::main("DLL Stress Test — 300 Bird Plugins")]
async fn main() {
    // ── Parse flags ────────────────────────────────────
    let args: Vec<String> = std::env::args().collect();
    let mode = if args.iter().any(|a| a == "--parallel") {
        LoadMode::Parallel
    } else if args.iter().any(|a| a == "--lazy") {
        LoadMode::Lazy
    } else {
        LoadMode::Sequential
    };
    let heavy = args.iter().any(|a| a == "--heavy");

    let root = project_root();
    let plugin_dir_name = if heavy { "plugins_heavy" } else { "plugins" };
    let plugin_source_dir = root.join(plugin_dir_name);
    let plugin_target_dir = root.join(plugin_dir_name).join("target").join("release");

    let dll_type = if heavy { "HEAVY" } else { "light" };
    println!("=== DLL Stress Test ===");
    println!("DLL type:       {}", dll_type);
    println!("Mode:           {:?}", mode);
    println!("Project root:   {}", root.display());
    println!("Plugin source:  {}", plugin_source_dir.display());
    println!("Plugin target:  {}", plugin_target_dir.display());
    println!();

    // ── Load plugins ────────────────────────────────────
    let mut manager = PluginManager::new(mode, &plugin_source_dir, &plugin_target_dir);

    match manager.load_all() {
        Ok(()) => {
            let metrics = manager.metrics();
            println!(
                "Loaded {}/{} plugins in {:.1} ms",
                metrics.loaded_count,
                metrics.total_plugins,
                metrics.total_load_ms
            );
            if metrics.loaded_count == 0 {
                eprintln!();
                eprintln!("ERROR: No plugins loaded. Did you build them first?");
                eprintln!("  cd {} && cargo build --release", plugin_source_dir.display());
                return;
            }
        }
        Err(e) => {
            eprintln!("Failed to load plugins: {}", e);
            return;
        }
    }

    // ── Start file watcher ──────────────────────────────
    manager.start_watching();

    // ── Run the engine ──────────────────────────────────
    let scene = BirdMenuScene::new(manager);

    println!();
    println!("Controls:");
    println!("  [↑↓] or [W/S]  — scroll through birds");
    println!("  [H]            — toggle metrics overlay");
    println!("  [Esc]          — quit");
    println!();
    println!("Edit any plugin source file to trigger a hot-reload.");
    println!("  e.g., plugins/plugin_042/src/lib.rs");
    println!();

    Engine::new()
        .with_clear_color(macroquad::prelude::Color::new(0.08, 0.08, 0.12, 1.0))
        .run(scene)
        .await;
}
