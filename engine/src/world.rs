//! A simple homogeneous container of [`GameObject`]s.
//!
//! `World` is an optional convenience for scenes that just need to update,
//! draw, and reap a bag of objects. Scenes that need cross-object logic
//! (collisions, scoring) can still own their entities directly.

use macroquad::prelude::Rect;

use crate::context::EngineContext;
use crate::object::GameObject;

/// Owns a collection of boxed game objects and drives their lifecycle.
#[derive(Default)]
pub struct World {
    objects: Vec<Box<dyn GameObject>>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an object to the world.
    pub fn spawn<O: GameObject + 'static>(&mut self, object: O) {
        self.objects.push(Box::new(object));
    }

    /// Number of live objects currently in the world.
    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// Update every object, then drop the ones that report `is_alive() == false`.
    pub fn update(&mut self, ctx: &mut EngineContext) {
        for object in &mut self.objects {
            object.update(ctx);
        }
        self.objects.retain(|o| o.is_alive());
    }

    /// Draw every object in insertion order.
    pub fn draw(&self, ctx: &EngineContext) {
        for object in &self.objects {
            object.draw(ctx);
        }
    }

    /// Iterate over the bounding boxes of all objects that define one.
    pub fn collider_bounds(&self) -> impl Iterator<Item = Rect> + '_ {
        self.objects.iter().filter_map(|o| o.bounds())
    }

    /// Borrow the objects, e.g. for custom queries.
    pub fn objects(&self) -> &[Box<dyn GameObject>] {
        &self.objects
    }

    /// Mutably borrow the objects.
    pub fn objects_mut(&mut self) -> &mut Vec<Box<dyn GameObject>> {
        &mut self.objects
    }
}
