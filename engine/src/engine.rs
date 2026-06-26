//! The engine entry point: owns the main loop and the active scene.

use macroquad::prelude::*;

use crate::context::EngineContext;
use crate::scene::Scene;

/// Drives the game loop: refresh context -> update scene -> draw scene ->
/// present frame -> handle scene switches, until a scene quits or the window
/// closes.
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

    /// Run the loop starting from `scene`. This consumes the engine and only
    /// returns when a scene calls [`EngineContext::quit`] or the window closes.
    pub async fn run<S: Scene + 'static>(self, scene: S) {
        let mut scene: Box<dyn Scene> = Box::new(scene);
        let mut ctx = EngineContext::new();
        scene.on_enter(&mut ctx);

        loop {
            ctx.dt = get_frame_time();
            ctx.time += ctx.dt;
            ctx.frame += 1;

            scene.update(&mut ctx);

            clear_background(self.clear_color);
            scene.draw(&ctx);

            if ctx.should_quit {
                break;
            }

            if let Some(mut next) = ctx.next_scene.take() {
                next.on_enter(&mut ctx);
                scene = next;
            }

            next_frame().await;
        }
    }
}
