//! The mini engine: owns the window/render loop, holds the engine-owned game
//! state, and calls the hot-reloaded project's `update` function every frame
//! through an atomically-swapped pointer.
//!
//! The host (`standalone`) never rebuilds or touches this crate's code - it
//! only swaps the project update pointer.  That is why the window and the
//! engine keep running untouched during a live coding cycle.

use std::sync::atomic::{AtomicPtr, Ordering};

use macroquad::prelude::*;

use crate::api::{self, ProjectApi};
use crate::state::GameState;

/// The hot-reloadable project's update entry point: `extern "C" fn(dt: f32)`.
pub type ProjectUpdateFn = extern "C" fn(delta_time: f32);

/// Turn a 0xRRGGBBAA integer into a macroquad colour.
fn color_from_u32(rgba: u32) -> Color {
    Color::from_rgba(
        ((rgba >> 24) & 0xff) as u8,
        ((rgba >> 16) & 0xff) as u8,
        ((rgba >> 8) & 0xff) as u8,
        (rgba & 0xff) as u8,
    )
}

/// A small, self-contained engine instance.
///
/// The game state lives in a heap `Box` so its address is stable even if the
/// engine struct itself is moved or wrapped in an `Arc` - the project DLL
/// holds a raw pointer to it for the whole process lifetime.
pub struct MiniEngine {
    state: Box<GameState>,
    /// Currently active project update function (null = no project loaded).
    update_fn: AtomicPtr<()>,
    /// Background colour cleared each frame.
    clear_color: Color,
}

impl MiniEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            state: Box::new(GameState::default()),
            update_fn: AtomicPtr::new(std::ptr::null_mut()),
            clear_color: Color::from_rgba(18, 18, 28, 255),
        };
        // Publish the state's stable heap address to the API layer.
        api::set_game_state_pointer(&mut *engine.state);
        engine
    }

    /// The API table handed to every loaded project DLL.
    pub fn project_api(&self) -> ProjectApi {
        ProjectApi::new()
    }

    /// Refresh the cached window size.  Must be called on the main thread;
    /// the host calls it once at startup and the render loop calls it every
    /// frame, so the API's screen size is always current (and thread-safe).
    pub fn update_screen_size(&self) {
        api::update_screen_size_cache();
    }

    /// Atomically swap the project's update function.  The host calls this
    /// after every successful reload (and with a null pointer to unload).
    pub fn set_project_update(&self, update: *mut ()) {
        self.update_fn.store(update, Ordering::Release);
    }

    /// Run the render loop until the window closes or Escape is pressed.
    ///
    /// Each frame: call the host's `on_frame` hook (used for live-coding
    /// checks), then the project's update (if one is loaded), then clear,
    /// draw the quads and present.  The project mutates the state through its
    /// raw pointer; we only ever read it here (single-threaded).
    pub async fn run<F>(&self, mut on_frame: F)
    where
        F: FnMut(&MiniEngine, f32),
    {
        loop {
            let delta_time = get_frame_time();

            // Keep the API's cached screen size in sync (main thread only).
            api::update_screen_size_cache();

            // Host hook: check for project changes and live-patch them.
            on_frame(self, delta_time);

            // Call the hot-reloaded project's update, if one is loaded.
            let update_pointer = self.update_fn.load(Ordering::Acquire);
            if !update_pointer.is_null() {
                let update: ProjectUpdateFn = unsafe { std::mem::transmute(update_pointer) };
                update(delta_time);
            }

            clear_background(self.clear_color);

            // Render every live quad from the engine-owned state.
            for quad in &self.state.quads[..self.state.quad_count] {
                draw_rectangle(
                    quad.x - quad.w * 0.5,
                    quad.y - quad.h * 0.5,
                    quad.w,
                    quad.h,
                    color_from_u32(quad.color),
                );
            }

            draw_text(
                "live-coding mini engine - edit project/src/lib.rs and save",
                16.0,
                24.0,
                20.0,
                WHITE,
            );

            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            next_frame().await;
        }
    }
}
