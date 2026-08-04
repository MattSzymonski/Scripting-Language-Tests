// mini_engine/src/lib.rs — Minimal game engine on macroquad
//
// REQUIREMENTS
//   macroquad 0.4, libloading 0.8
//
// DESCRIPTION
//   Provides a Renderer (macroquad draw wrappers), a GameLibrary (DLL
//   hot-loader), and an Engine that ties them together. The engine polls
//   the game DLL every frame and draws whatever the game tells it to.
//
// USAGE
//   let mut engine = Engine::new("game.dll");
//   engine.start();                          // calls game.start()
//   loop {
//       engine.update(get_frame_time());      // calls game.update(), draws
//       next_frame().await;
//   }
//
// EXAMPLE USAGE
//   See standalone/src/main.rs for the full window + watcher setup.
//
// --- SCRIPT ---

use macroquad::prelude::*;
use std::path::PathBuf;

// ── Renderer ──────────────────────────────────────────────────────────────

/// Wraps macroquad immediate-mode drawing so the engine can render
/// primitives without the game knowing about macroquad directly.
pub struct Renderer;

impl Renderer {
    /// Clear the screen with a solid background color.
    pub fn clear_background(&self, color: Color) {
        clear_background(color);
    }

    /// Draw a filled circle (used for the ball).
    pub fn draw_circle(&self, center_x: f32, center_y: f32, radius: f32, color: Color) {
        draw_circle(center_x, center_y, radius, color);
    }

    /// Draw on-screen debug text.
    pub fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        draw_text(text, x, y, font_size, color);
    }
}

// ── Game Interface ────────────────────────────────────────────────────────
//
// The game DLL must export these three symbols (all extern "C"):
//   start()           — reset/initialize game state
//   update(dt: f32)   — step simulation, write result to output pointer
//   get_state()       — return pointer to BallState
//
// ABI stability: BallState is #[repr(C)] so layout is predictable across
// compilation units.

/// The data the engine reads from the game DLL every frame.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BallState {
    pub position_x: f32,
    pub position_y: f32,
    pub radius: f32,
    pub color_r: f32,
    pub color_g: f32,
    pub color_b: f32,
    pub color_a: f32,
}

impl Default for BallState {
    fn default() -> Self {
        Self {
            position_x: 400.0,
            position_y: 300.0,
            radius: 20.0,
            color_r: 1.0,
            color_g: 0.3,
            color_b: 0.3,
            color_a: 1.0,
        }
    }
}

// ── DLL Loader ────────────────────────────────────────────────────────────

/// Holds a loaded game DLL and its exported function pointers.
pub struct GameLibrary {
    /// The loaded library handle (kept alive for the lifetime of the game).
    #[allow(dead_code)]
    library: libloading::Library,
    /// Called once on engine start / space-press reset.
    pub start_fn: unsafe extern "C" fn(),
    /// Called every frame with delta time in seconds.
    pub update_fn: unsafe extern "C" fn(delta_time: f32),
    /// Returns a pointer to the game's static BallState.
    pub get_state_fn: unsafe extern "C" fn() -> *const BallState,
    /// Path to the DLL on disk (for reload comparison).
    pub path: PathBuf,
}

impl GameLibrary {
    /// Load a game DLL from the given path.
    ///
    /// # Safety
    /// The DLL must export `start`, `update`, and `get_state` as `extern "C"`
    /// symbols.
    pub unsafe fn load(path: impl Into<PathBuf>) -> Result<Self, String> {
        let path: PathBuf = path.into();
        let library = unsafe {
            libloading::Library::new(&path).map_err(|e| format!("failed to load game DLL: {e}"))?
        };

        // Resolve the three required symbols
        let start_fn: libloading::Symbol<unsafe extern "C" fn()> = unsafe {
            library
                .get(b"start")
                .map_err(|e| format!("symbol 'start' not found: {e}"))?
        };
        let start_fn = *start_fn;

        let update_fn: libloading::Symbol<unsafe extern "C" fn(f32)> = unsafe {
            library
                .get(b"update")
                .map_err(|e| format!("symbol 'update' not found: {e}"))?
        };
        let update_fn = *update_fn;

        let get_state_fn: libloading::Symbol<unsafe extern "C" fn() -> *const BallState> = unsafe {
            library
                .get(b"get_state")
                .map_err(|e| format!("symbol 'get_state' not found: {e}"))?
        };
        let get_state_fn = *get_state_fn;

        Ok(Self {
            library,
            start_fn,
            update_fn,
            get_state_fn,
            path,
        })
    }
}

// ── Engine ────────────────────────────────────────────────────────────────

/// Ties the Renderer and GameLibrary together. Call `update()` every frame.
pub struct Engine {
    renderer: Renderer,
    game_library: GameLibrary,
}

impl Engine {
    /// Create an engine that loads the game from `game_dll_path`.
    ///
    /// # Panics
    /// If the DLL cannot be loaded or symbols are missing.
    pub fn new(game_dll_path: impl Into<PathBuf>) -> Self {
        let game_library =
            unsafe { GameLibrary::load(game_dll_path).expect("failed to load initial game DLL") };
        Self {
            renderer: Renderer,
            game_library,
        }
    }

    /// Call `game.start()` (resets ball position and velocity).
    pub fn start(&self) {
        unsafe { (self.game_library.start_fn)() };
    }

    /// Advance one frame: update the game simulation, then draw.
    pub fn update(&self, delta_time: f32) {
        // Step the game simulation
        unsafe { (self.game_library.update_fn)(delta_time) };

        // Read the game state
        let state = unsafe { &*(self.game_library.get_state_fn)() };

        // Render
        self.renderer.clear_background(BLACK);
        let ball_color = Color::new(state.color_r, state.color_g, state.color_b, state.color_a);
        self.renderer
            .draw_circle(state.position_x, state.position_y, state.radius, ball_color);

        // HUD text
        let fps_text = format!("FPS: {:.0}", macroquad::time::get_fps());
        self.renderer.draw_text(&fps_text, 10.0, 25.0, 22.0, GRAY);
        self.renderer.draw_text(
            "Press SPACE to bounce | Edit game/src/lib.rs to hot-reload",
            10.0,
            50.0,
            16.0,
            GRAY,
        );
    }

    /// Reload the game DLL from a new path (the freshly compiled versioned
    /// copy). The old DLL is intentionally leaked — its code may still be
    /// referenced by in-flight frames, and on Windows the file is locked
    /// until the process exits anyway.
    pub fn reload_game(&mut self, new_path: impl Into<PathBuf>) -> Result<(), String> {
        let new_path: PathBuf = new_path.into();
        let new_library = unsafe { GameLibrary::load(&new_path)? };
        let old = std::mem::replace(&mut self.game_library, new_library);
        // Old library is intentionally leaked — its code may still be
        // referenced by pending callbacks (not an issue with our simple
        // loop, but good practice for hot-reload safety).
        std::mem::forget(old);
        Ok(())
    }

    /// Access the renderer for custom draw calls outside the main loop.
    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }
}
