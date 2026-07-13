//! Module grid scene: renders every module in the AAA scenario (core,
//! subsystem, and leaf tiers together) and shows a live metrics overlay.
//!
//! Core modules draw as small pale squares, subsystem modules as blue
//! squares, and leaf modules as the original colored "birds" (the only tier
//! that's actually hot-reloadable / visually distinct per-instance).

use macroquad::prelude::*;

use mini_engine::{EngineContext, Scene};

use crate::ffi::ModuleTier;
use crate::module_loader::{LoadMetrics, PluginManager};

// ── Layout constants ────────────────────────────────────

const BASE_SIZE: f32 = 16.0;
const GRID_MARGIN: f32 = 8.0;
const NAME_FONT_SIZE: f32 = 9.0;
const OVERLAY_FONT_SIZE: f32 = 15.0;
const GRID_COLS: u32 = 40;
const OVERLAY_BG_ALPHA: f32 = 0.7;

pub struct BirdMenuScene {
    identities: Vec<(String, crate::ffi::ModuleIdentity, bool)>,
    metrics: LoadMetrics,
    plugin_manager: PluginManager,
    scroll_y: f32,
    show_overlay: bool,
}

impl BirdMenuScene {
    pub fn new(plugin_manager: PluginManager) -> Self {
        let identities = plugin_manager.all_identities();
        let metrics = plugin_manager.metrics();
        Self {
            identities,
            metrics,
            plugin_manager,
            scroll_y: 0.0,
            show_overlay: true,
        }
    }

    fn grid_position(&self, index: u32, screen_w: f32) -> (f32, f32) {
        let col = (index % GRID_COLS) as f32;
        let row = (index / GRID_COLS) as f32;

        let cell_w = BASE_SIZE * 2.0 + GRID_MARGIN;
        let cell_h = BASE_SIZE * 2.0 + NAME_FONT_SIZE + GRID_MARGIN;

        let grid_w = GRID_COLS as f32 * cell_w;
        let start_x = (screen_w - grid_w) / 2.0;

        let hash = (index.wrapping_mul(2654435761)) as f32 / u32::MAX as f32;
        let offset_x = (hash - 0.5) * 6.0;
        let offset_y = ((hash * 7919.0).fract() - 0.5) * 6.0;

        (
            start_x + col * cell_w + cell_w / 2.0 + offset_x,
            30.0 + row * cell_h + cell_h / 2.0 + offset_y - self.scroll_y,
        )
    }
}

impl Scene for BirdMenuScene {
    fn update(&mut self, ctx: &mut EngineContext) {
        let scroll_speed = 400.0 * ctx.dt;

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

        self.plugin_manager.process_reloads();
        self.plugin_manager.update_frame();

        self.metrics = self.plugin_manager.metrics();
        self.identities = self.plugin_manager.all_identities();
    }

    fn draw(&self, _ctx: &EngineContext) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        for (index, (_name, identity, loaded)) in self.identities.iter().enumerate() {
            if !loaded {
                continue;
            }
            let (x, y) = self.grid_position(index as u32, screen_w);
            if y < -40.0 || y > screen_h + 40.0 || x < -40.0 || x > screen_w + 40.0 {
                continue;
            }

            let (r, g, b, a) = identity.color();
            let tier = identity.tier();
            let size = match tier {
                ModuleTier::Core => BASE_SIZE * 1.3,
                ModuleTier::Subsystem => BASE_SIZE * 1.1,
                ModuleTier::Leaf => BASE_SIZE * identity.size,
            };

            draw_rectangle(x - size / 2.0, y - size / 2.0, size, size, Color::new(r, g, b, a));

            // Only leaf modules get the little "bird beak" flourish.
            if tier == ModuleTier::Leaf {
                draw_rectangle(x + size / 2.0 - 3.0, y - 2.0, 5.0, 5.0, Color::new(1.0, 0.8, 0.0, 1.0));
            }

            if tier == ModuleTier::Leaf {
                let name = identity.name_string();
                let name_w = measure_text(&name, None, NAME_FONT_SIZE as u16, 1.0).width;
                draw_text(
                    &name,
                    x - name_w / 2.0,
                    y + size / 2.0 + NAME_FONT_SIZE + 1.0,
                    NAME_FONT_SIZE,
                    Color::new(0.8, 0.8, 0.8, 1.0),
                );
            }
        }

        if self.show_overlay {
            self.draw_overlay();
        }
    }
}

impl BirdMenuScene {
    #[allow(unused_assignments)]
    fn draw_overlay(&self) {
        let panel_x = 12.0;
        let panel_y = 12.0;
        let panel_w = 460.0;
        let line_h = OVERLAY_FONT_SIZE + 4.0;
        let outlier_lines = self.metrics.top_outliers.len().min(8);
        let lines = 12 + outlier_lines;
        let panel_h = lines as f32 * line_h + 16.0;

        draw_rectangle(panel_x, panel_y, panel_w, panel_h, Color::new(0.0, 0.0, 0.0, OVERLAY_BG_ALPHA));

        let mut y = panel_y + 8.0 + OVERLAY_FONT_SIZE;
        let color = Color::new(0.9, 0.9, 0.9, 1.0);
        let highlight = Color::new(0.3, 1.0, 0.5, 1.0);
        let warn = Color::new(1.0, 0.75, 0.3, 1.0);
        let dim = Color::new(0.6, 0.6, 0.6, 1.0);

        macro_rules! line {
            ($col:expr, $($arg:tt)*) => {{
                draw_text(&format!($($arg)*), panel_x + 8.0, y, OVERLAY_FONT_SIZE, $col);
                y += line_h;
            }};
        }

        line!(color, "-- AAA Scenario DLL Stress Test --");
        line!(
            color,
            "Mode: {} | Modules: {}/{} loaded",
            self.metrics.mode, self.metrics.loaded_count, self.metrics.total_modules
        );
        line!(
            color,
            "  core: {}  subsystem: {}  leaf: {}",
            self.metrics.core_count, self.metrics.subsystem_count, self.metrics.leaf_count
        );
        line!(highlight, "Total wall-clock: {:.1} ms", self.metrics.total_load_ms);
        line!(color, "  LoadLibrary:    {:.1} ms", self.metrics.total_load_library_ms);
        line!(color, "  GetSymbol:      {:.1} ms", self.metrics.total_get_symbol_ms);
        line!(color, "  CreateInstance: {:.1} ms", self.metrics.total_create_instance_ms);
        line!(color, "  StartupModule:  {:.1} ms", self.metrics.total_startup_module_ms);
        line!(color, "FPS: {:.0} | Reloads: {}", self.metrics.fps, self.metrics.reload_count);

        if !self.metrics.top_outliers.is_empty() {
            line!(dim, "-- Slowest modules (unlucky first loaders) --");
            for (name, total_ms, pulled_in) in self.metrics.top_outliers.iter().take(8) {
                if pulled_in.is_empty() {
                    line!(color, "  {:.1}ms  {}", total_ms, name);
                } else {
                    line!(warn, "  {:.1}ms  {} -> pulled in {}", total_ms, name, pulled_in.join(", "));
                }
            }
        }

        line!(dim, "[H] toggle overlay  [WS/up-down] scroll  [Esc] quit");
    }
}
