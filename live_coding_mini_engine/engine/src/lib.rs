//! # mini_engine (live-coding variant)
//!
//! A tiny game engine built on [`macroquad`](https://docs.rs/macroquad) that
//! renders jumping quads.  It does not contain any game logic: the logic lives
//! in a separately compiled, hot-reloadable `project` DLL that the host
//! (`standalone`) loads and swaps at runtime.
//!
//! The pieces:
//!
//! - [`GameState`] / [`Quad`] — engine-owned state that survives every
//!   project reload (the game DLL only holds a pointer to it).
//! - [`ProjectApi`] — a `#[repr(C)]` table of function pointers handed to the
//!   project DLL so it can reach the engine state and screen size *without*
//!   linking macroquad (the DLL must never create a second macroquad state).
//! - [`MiniEngine`] — owns the window/render loop, calls the project's
//!   `update(dt)` every frame through an atomically-swapped pointer, and draws
//!   the quads.

mod api;
mod engine;
pub mod patch;
mod state;

pub use api::ProjectApi;
pub use engine::{MiniEngine, ProjectUpdateFn};
pub use state::{GameState, Quad, MAX_QUADS};

/// Everything a host needs in one `use`.
pub mod prelude {
    pub use crate::{GameState, MiniEngine, ProjectApi, ProjectUpdateFn, Quad, MAX_QUADS};
    pub use macroquad::prelude::*;
}
