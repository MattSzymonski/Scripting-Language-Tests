//! # mini_engine
//!
//! A minimal game engine shell built on top of
//! [`macroquad`](https://docs.rs/macroquad). It provides three small building
//! blocks:
//!
//! - [`Scene`] — the game logic, updated and drawn once per frame.
//! - [`Engine`] — owns the main loop and runs a scene.
//! - [`EngineContext`] — per-frame services (frame timing, quitting).
//!
//! ## Minimal example
//!
//! ```no_run
//! use mini_engine::prelude::*;
//!
//! struct Hello;
//! impl Scene for Hello {
//!     fn update(&mut self, ctx: &mut EngineContext) {
//!         if is_key_pressed(KeyCode::Escape) {
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
mod scene;

pub use context::EngineContext;
pub use engine::Engine;
pub use scene::Scene;

/// Everything you need to build a game, in one `use`.
pub mod prelude {
    pub use crate::{Engine, EngineContext, Scene};

    // Re-export the macroquad essentials so games depend on one crate surface.
    pub use macroquad::prelude::*;
}
