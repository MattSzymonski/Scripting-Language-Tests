//! DLL Stress Test — Main Entry Point
//!
//! Loads the AAA-scenario module graph (core/subsystem/leaf tiers, real
//! cross-DLL PE imports — see generate_aaa_modules.py), renders every module
//! on a grid, and measures real recursive DLL loading performance. Supports
//! three load modes:
//!
//!   cargo run -p stress_test --release                  # sequential (default)
//!   cargo run -p stress_test --release -- --parallel     # parallel loading
//!   cargo run -p stress_test --release -- --lazy         # lazy/deferred
//!
//! Requires the scenario to already be generated + built:
//!   python generate_aaa_modules.py && python build_aaa_scenario.py

mod bird_menu;
mod ffi;
mod manifest;
mod module_loader;
mod win32;

use bird_menu::BirdMenuScene;
use manifest::Manifest;
use mini_engine::Engine;
use module_loader::{LoadMode, PluginManager};
use std::path::PathBuf;

/// Resolve the dll_stresstesting/ project root relative to the working directory.
fn project_root() -> PathBuf {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd.ends_with("stress_test") {
        cwd.parent().unwrap_or(&cwd).to_path_buf()
    } else {
        cwd
    }
}

#[macroquad::main("DLL Stress Test — AAA Scenario")]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = if args.iter().any(|a| a == "--parallel") {
        LoadMode::Parallel
    } else if args.iter().any(|a| a == "--lazy") {
        LoadMode::Lazy
    } else {
        LoadMode::Sequential
    };

    let root = project_root();
    let manifest_path = root.join("aaa_scenario").join("manifest.json");

    println!("=== DLL Stress Test: AAA Scenario ===");
    println!("Mode:      {:?}", mode);
    println!("Manifest:  {}", manifest_path.display());

    let manifest = match Manifest::load(&manifest_path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to load manifest: {}", e);
            eprintln!();
            eprintln!("Generate and build the scenario first:");
            eprintln!("  python generate_aaa_modules.py   (defaults to ~1929 modules)");
            eprintln!("  python build_aaa_scenario.py");
            return;
        }
    };

    println!(
        "Modules:   {} core + {} subsystem + {} leaf = {} total",
        manifest.counts.core, manifest.counts.subsystem, manifest.counts.leaf, manifest.counts.total
    );
    println!();

    let mut manager = PluginManager::new(mode, manifest, &manifest_path);

    match manager.load_all() {
        Ok(()) => {
            let metrics = manager.metrics();
            if metrics.loaded_count == 0 {
                eprintln!();
                eprintln!("ERROR: No modules loaded. Did you build the scenario?");
                eprintln!("  python build_aaa_scenario.py");
                return;
            }
        }
        Err(e) => {
            eprintln!("Failed to load modules: {}", e);
            return;
        }
    }

    let csv_path = root.join("aaa_scenario").join("last_run_results.csv");
    match manager.write_csv(&csv_path) {
        Ok(()) => println!("Results written to: {}", csv_path.display()),
        Err(e) => eprintln!("Failed to write CSV: {}", e),
    }

    manager.start_watching();

    let scene = BirdMenuScene::new(manager);

    println!();
    println!("Controls:");
    println!("  [up/down] or [W/S]  — scroll through modules");
    println!("  [H]                 — toggle metrics overlay");
    println!("  [Esc]               — quit");
    println!();
    println!("Edit any leaf module's source to trigger a hot-reload.");
    println!("  e.g., aaa_scenario/leaf/aaa_leaf_0042/src/lib.rs");
    println!();

    Engine::new()
        .with_clear_color(macroquad::prelude::Color::new(0.08, 0.08, 0.12, 1.0))
        .run(scene)
        .await;
}
