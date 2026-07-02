//! Tunable gameplay constants — mirror of `Flappy.Config` in C#.
//!
//! All values are `pub` so they can be tweaked without touching logic.

pub const GROUND_HEIGHT: f32 = 96.0;

// Bird
pub const BIRD_RADIUS: f32 = 18.0;
pub const GRAVITY: f32 = 1500.0;
pub const FLAP_VELOCITY: f32 = -470.0;
pub const BIRD_X: f32 = 120.0;

// Pipes
pub const PIPE_WIDTH: f32 = 78.0;
pub const PIPE_GAP: f32 = 185.0;
pub const SCROLL_SPEED: f32 = 175.0;
pub const SPAWN_INTERVAL: f32 = 1.5;
pub const GAP_MARGIN: f32 = 90.0;
