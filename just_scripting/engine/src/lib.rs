//! # mini_engine
//!
//! A *super-mini* object-oriented game engine built on top of
//! [`macroquad`](https://docs.rs/macroquad). It provides four small building
//! blocks:
//!
//! - [`GameObject`] — the base "class" for every entity (update / draw / bounds).
//! - [`World`] — a container that owns objects and drives their lifecycle.
//! - [`Scene`] — one screen of the game that coordinates objects and global logic.
//! - [`Engine`] — owns the main loop and the active scene.
//! - [`EngineContext`] — per-frame services (time, input, window, scene control).
//!
//! ## Minimal example
//!
//! ```no_run
//! use mini_engine::prelude::*;
//!
//! struct Hello;
//! impl Scene for Hello {
//!     fn update(&mut self, ctx: &mut EngineContext) {
//!         if ctx.key_pressed(KeyCode::Escape) {
//!             ctx.quit();
//!         }
//!     }
//!     fn draw(&self, _ctx: &EngineContext) {
//!         draw_text("hello, engine", 20.0, 40.0, 30.0, WHITE);
//!     }
//! }
//!
//! #[macroquad::main("demo")]
//! async fn main() {
//!     Engine::new().run(Hello).await;
//! }
//! ```

mod context;
mod engine;
mod object;
mod scene;
mod world;

pub use context::EngineContext;
pub use engine::Engine;
pub use object::GameObject;
pub use scene::Scene;
pub use world::World;

/// Everything you need to build a game, in one `use`.
pub mod prelude {
    pub use crate::{Engine, EngineContext, GameObject, Scene, World};

    // Re-export the macroquad essentials so games depend on one crate surface.
    pub use macroquad::prelude::*;
}
