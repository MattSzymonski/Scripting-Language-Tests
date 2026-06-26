//! A [`Scene`] is one self-contained screen of the game (menu, gameplay,
//! game-over, ...). The engine runs exactly one scene at a time and swaps it
//! out when a scene calls [`EngineContext::change_scene`].

use crate::context::EngineContext;

/// A top-level game state the engine drives each frame.
///
/// Where [`GameObject`](crate::object::GameObject) is a single entity, a
/// `Scene` owns and coordinates a whole collection of them plus any global
/// logic (scoring, spawning, collision resolution, ...).
pub trait Scene {
    /// Called once when the scene becomes active. Defaults to a no-op.
    fn on_enter(&mut self, _ctx: &mut EngineContext) {}

    /// Update all game logic for this frame.
    fn update(&mut self, ctx: &mut EngineContext);

    /// Draw the scene for this frame.
    fn draw(&self, ctx: &EngineContext);
}
