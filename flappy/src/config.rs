//! Tunable gameplay constants, all in one place.

/// Logical window size. The game is designed around this resolution.
pub const WINDOW_WIDTH: i32 = 480;
pub const WINDOW_HEIGHT: i32 = 720;

/// Height of the ground strip at the bottom of the screen, in pixels.
pub const GROUND_HEIGHT: f32 = 96.0;

// --- bird ------------------------------------------------------------------
pub const BIRD_RADIUS: f32 = 18.0;
/// Downward acceleration, pixels per second squared.
pub const GRAVITY: f32 = 1500.0;
/// Instant upward velocity applied on a flap (negative = up), px/s.
pub const FLAP_VELOCITY: f32 = -470.0;
/// Horizontal position of the bird (it never moves sideways).
pub const BIRD_X: f32 = 120.0;

// --- pipes -----------------------------------------------------------------
pub const PIPE_WIDTH: f32 = 78.0;
/// Vertical opening the bird flies through.
pub const PIPE_GAP: f32 = 185.0;
/// Horizontal scroll speed of the world, px/s.
pub const SCROLL_SPEED: f32 = 175.0;
/// Seconds between pipe spawns.
pub const SPAWN_INTERVAL: f32 = 1.5;
/// Minimum distance from the screen edges / ground that a gap centre can sit.
pub const GAP_MARGIN: f32 = 90.0;
