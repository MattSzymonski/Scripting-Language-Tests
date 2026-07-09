//! Bird menu scene: renders all loaded birds on screen with names.
//!
//! Implements the `Scene` trait. Arranges birds in a deterministic grid with
//! slight random offsets, draws each as a colored rectangle with its name below,
//! and shows a metrics overlay in the top-left corner.

use macroquad::prelude::*;

use mini_engine::{EngineContext, Scene};

use crate::ffi::PluginInfo;
use crate::plugin_loader::{LoadMetrics, PluginManager};

// ── Layout constants ────────────────────────────────────

/// Base size of a bird rectangle in pixels (before size multiplier).
const BASE_BIRD_SIZE: f32 = 20.0;
/// Margin between birds in the grid.
const GRID_MARGIN: f32 = 12.0;
/// Font size for bird names.
const NAME_FONT_SIZE: f32 = 10.0;
/// Font size for the overlay panel.
const OVERLAY_FONT_SIZE: f32 = 16.0;
/// Columns in the bird grid.
const GRID_COLS: u32 = 20;
/// Overlay background opacity (0.0 – 1.0).
const OVERLAY_BG_ALPHA: f32 = 0.6;

/// The stress-test menu scene.
pub struct BirdMenuScene {
    /// All plugins and their bird definitions.
    plugin_infos: Vec<PluginInfo>,
    /// Current metrics snapshot.
    metrics: LoadMetrics,
    /// The plugin manager (holds loaded DLLs).
    plugin_manager: PluginManager,
    /// Camera scroll offset (keyboard scroll).
    scroll_y: f32,
    /// Whether to show the overlay.
    show_overlay: bool,
}

impl BirdMenuScene {
    /// Create a new bird menu scene from a fully-loaded plugin manager.
    pub fn new(plugin_manager: PluginManager) -> Self {
        let plugin_infos = plugin_manager.all_plugin_infos();
        let metrics = plugin_manager.metrics();

        Self {
            plugin_infos,
            metrics,
            plugin_manager,
            scroll_y: 0.0,
            show_overlay: true,
        }
    }

    /// Calculate the grid position for a plugin index.
    fn grid_position(&self, index: u32, screen_w: f32, _screen_h: f32) -> (f32, f32) {
        let col = (index % GRID_COLS) as f32;
        let row = (index / GRID_COLS) as f32;

        let cell_w = BASE_BIRD_SIZE * 2.0 + GRID_MARGIN;
        let cell_h = BASE_BIRD_SIZE * 2.0 + NAME_FONT_SIZE + GRID_MARGIN;

        let grid_w = GRID_COLS as f32 * cell_w;
        let start_x = (screen_w - grid_w) / 2.0;

        // Deterministic "random" offset using a simple hash
        let hash = (index.wrapping_mul(2654435761)) as f32 / u32::MAX as f32;
        let offset_x = (hash - 0.5) * 8.0;
        let offset_y = ((hash * 7919.0).fract() - 0.5) * 8.0;

        (
            start_x + col * cell_w + cell_w / 2.0 + offset_x,
            30.0 + row * cell_h + cell_h / 2.0 + offset_y - self.scroll_y,
        )
    }
}

impl Scene for BirdMenuScene {
    fn update(&mut self, ctx: &mut EngineContext) {
        // Input
        let scroll_speed = 300.0 * ctx.dt;

        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.scroll_y += scroll_speed;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.scroll_y -= scroll_speed;
            if self.scroll_y < 0.0 {
                self.scroll_y = 0.0;
            }
        }
        if is_key_pressed(KeyCode::H) {
            self.show_overlay = !self.show_overlay;
        }
        if is_key_pressed(KeyCode::Escape) {
            ctx.quit();
        }

        // Process any hot-reload events from the watcher
        self.plugin_manager.process_reloads();
        self.plugin_manager.update_frame();

        // Refresh metrics and plugin info (in case of reloads)
        self.metrics = self.plugin_manager.metrics();
        self.plugin_infos = self.plugin_manager.all_plugin_infos();
    }

    fn draw(&self, _ctx: &EngineContext) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // ── Draw birds ──────────────────────────────────
        for info in &self.plugin_infos {
            if !info.loaded {
                continue;
            }

            let (x, y) = self.grid_position(info.index, screen_w, screen_h);

            // Skip off-screen birds
            if y < -40.0 || y > screen_h + 40.0 {
                continue;
            }
            if x < -40.0 || x > screen_w + 40.0 {
                continue;
            }

            let (r, g, b, a) = info.bird.color();
            let size = BASE_BIRD_SIZE * info.bird.size;

            // Draw bird body (colored rectangle)
            draw_rectangle(x - size / 2.0, y - size / 2.0, size, size, Color::new(r, g, b, a));

            // Draw bird "beak" (small triangle or just a smaller rect)
            draw_rectangle(
                x + size / 2.0 - 4.0,
                y - 3.0,
                6.0,
                6.0,
                Color::new(1.0, 0.8, 0.0, 1.0),
            );

            // Draw bird name below
            let name = info.bird.name_string();
            let name_w = measure_text(&name, None, NAME_FONT_SIZE as u16, 1.0).width;
            draw_text(
                &name,
                x - name_w / 2.0,
                y + size / 2.0 + NAME_FONT_SIZE + 2.0,
                NAME_FONT_SIZE,
                Color::new(0.9, 0.9, 0.9, 1.0),
            );
        }

        // ── Overlay panel ───────────────────────────────
        if self.show_overlay {
            let panel_x = 12.0;
            let panel_y = 12.0;
            let panel_w = 380.0;
            let line_h = OVERLAY_FONT_SIZE + 4.0;
            let lines = 11;
            let panel_h = lines as f32 * line_h + 16.0;

            // Background
            draw_rectangle(
                panel_x,
                panel_y,
                panel_w,
                panel_h,
                Color::new(0.0, 0.0, 0.0, OVERLAY_BG_ALPHA),
            );

            let mut y = panel_y + 8.0 + OVERLAY_FONT_SIZE;
            let color = Color::new(0.9, 0.9, 0.9, 1.0);
            let highlight = Color::new(0.3, 1.0, 0.5, 1.0);
            let warn = Color::new(1.0, 0.8, 0.3, 1.0);

            draw_text("── DLL Stress Test ──", panel_x + 8.0, y, OVERLAY_FONT_SIZE, color);
            y += line_h;

            draw_text(
                &format!("Mode: {} | Plugins: {}/{} loaded",
                    self.metrics.mode,
                    self.metrics.loaded_count,
                    self.metrics.total_plugins),
                panel_x + 8.0, y, OVERLAY_FONT_SIZE, color,
            );
            y += line_h;

            draw_text(
                &format!("Total wall-clock: {:.1} ms", self.metrics.total_load_ms),
                panel_x + 8.0, y, OVERLAY_FONT_SIZE, highlight,
            );
            y += line_h;

            draw_text(
                &format!("  I/O (file copy):     {:.1} ms", self.metrics.total_io_ms),
                panel_x + 16.0, y, OVERLAY_FONT_SIZE, color,
            );
            y += line_h;

            draw_text(
                &format!("  Symbol resolution:   {:.1} ms", self.metrics.total_symbol_ms),
                panel_x + 16.0, y, OVERLAY_FONT_SIZE, color,
            );
            y += line_h;

            draw_text(
                &format!("  Init (create_bird):  {:.1} ms", self.metrics.total_init_ms),
                panel_x + 16.0, y, OVERLAY_FONT_SIZE, color,
            );
            y += line_h;

            draw_text(
                &format!("FPS: {:.0} | Reloads: {}", self.metrics.fps, self.metrics.reload_count),
                panel_x + 8.0, y, OVERLAY_FONT_SIZE, color,
            );
            y += line_h;

            draw_text(
                "[H] toggle overlay  [↑↓] scroll  [Esc] quit",
                panel_x + 8.0,
                y,
                OVERLAY_FONT_SIZE,
                Color::new(0.6, 0.6, 0.6, 1.0),
            );
        }
    }
}
