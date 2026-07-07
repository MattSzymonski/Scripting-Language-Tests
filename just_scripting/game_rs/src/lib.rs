//! # game_rs
//!
//! Flappy Bird game logic — a pure-Rust port of `game_cs`.
//!
//! Exports [`FlappyScene`] which implements [`mini_engine::Scene`] and can be
//! passed directly to [`mini_engine::Engine::run`].

mod bird;
mod config;
mod pipe;
mod primitives;

pub mod flappy_scene;

pub use flappy_scene::FlappyScene;
