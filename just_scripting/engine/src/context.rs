//! The per-frame context handed to every scene and game object.
//!
//! Think of this as the "engine services" object in a classic OOP engine:
//! it exposes time, the window, input, and a handful of commands the running
//! scene can issue back to the engine (switching scenes, quitting).

use macroquad::prelude::*;

use crate::scene::Scene;

/// Engine-provided state and services, refreshed once per frame and passed by
/// reference into [`Scene::update`](crate::scene::Scene::update) /
/// [`Scene::draw`](crate::scene::Scene::draw) and the game objects beneath them.
pub struct EngineContext {
    /// Seconds elapsed since the previous frame.
    pub dt: f32,
    /// Total seconds the engine has been running.
    pub time: f32,
    /// Number of frames rendered so far.
    pub frame: u64,

    pub(crate) next_scene: Option<Box<dyn Scene>>,
    pub(crate) should_quit: bool,
}

impl EngineContext {
    pub(crate) fn new() -> Self {
        Self {
            dt: 0.0,
            time: 0.0,
            frame: 0,
            next_scene: None,
            should_quit: false,
        }
    }

    /// Current window size in pixels.
    pub fn screen_size(&self) -> Vec2 {
        vec2(screen_width(), screen_height())
    }

    pub fn screen_width(&self) -> f32 {
        screen_width()
    }

    pub fn screen_height(&self) -> f32 {
        screen_height()
    }

    // --- input helpers -----------------------------------------------------

    /// `true` on the single frame `key` transitions from up to down.
    pub fn key_pressed(&self, key: KeyCode) -> bool {
        is_key_pressed(key)
    }

    /// `true` for every frame `key` is held.
    pub fn key_down(&self, key: KeyCode) -> bool {
        is_key_down(key)
    }

    /// `true` on the single frame a mouse button transitions from up to down.
    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        is_mouse_button_pressed(button)
    }

    /// Mouse position in screen pixels.
    pub fn mouse_position(&self) -> Vec2 {
        let (x, y) = mouse_position();
        vec2(x, y)
    }

    // --- engine commands ---------------------------------------------------

    /// Replace the active scene at the end of the current frame.
    pub fn change_scene<S: Scene + 'static>(&mut self, scene: S) {
        self.next_scene = Some(Box::new(scene));
    }

    /// Ask the engine to shut down after this frame.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
