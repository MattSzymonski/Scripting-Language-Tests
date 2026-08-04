//! A [`Scene`] is the game logic the engine drives each frame.

use crate::context::EngineContext;

/// The one thing the engine knows how to run: update, then draw.
pub trait Scene {
    /// Update all game logic for this frame.
    fn update(&mut self, ctx: &mut EngineContext);

    /// Draw the scene for this frame.
    fn draw(&self, ctx: &EngineContext);
}
