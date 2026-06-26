//! The base "class" of the engine: every entity in a scene is a [`GameObject`].

use macroquad::prelude::Rect;

use crate::context::EngineContext;

/// The behaviour shared by every entity the engine can drive.
///
/// This is the engine's equivalent of an abstract base class with virtual
/// methods. All methods have sensible defaults, so a concrete object only
/// overrides what it needs (most override `update` and `draw`).
pub trait GameObject {
    /// Advance the object's state by `ctx.dt` seconds.
    fn update(&mut self, _ctx: &mut EngineContext) {}

    /// Render the object. Called after every object has been updated.
    fn draw(&self, _ctx: &EngineContext) {}

    /// Return `false` to have the engine remove this object from its
    /// [`World`](crate::world::World) at the end of the frame.
    fn is_alive(&self) -> bool {
        true
    }

    /// Axis-aligned bounding box used for collision queries. Objects that have
    /// no physical presence can leave this as `None` (the default).
    fn bounds(&self) -> Option<Rect> {
        None
    }
}
