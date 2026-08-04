//! The engine entry point: owns the main loop.

use macroquad::prelude::*;

use crate::context::EngineContext;
use crate::scene::Scene;

/// Drives the game loop: refresh context -> update scene -> draw scene ->
/// present frame, until the scene quits or the window closes.
pub struct Engine {
    clear_color: Color,
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            clear_color: BLACK,
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the background colour the engine clears to each frame.
    pub fn with_clear_color(mut self, color: Color) -> Self {
        self.clear_color = color;
        self
    }

    /// Run the loop starting from `scene`. Only returns when the scene calls
    /// [`EngineContext::quit`] or the window closes.
    pub async fn run<S: Scene>(self, mut scene: S) {
        let mut ctx = EngineContext::new();

        loop {
            ctx.dt = get_frame_time();

            scene.update(&mut ctx);

            clear_background(self.clear_color);
            scene.draw(&ctx);

            if ctx.should_quit {
                break;
            }

            next_frame().await;
        }
    }
}
