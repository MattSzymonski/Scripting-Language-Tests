//! Engine-owned game state shared with the hot-reloaded project DLL.
//!
//! These structs are part of the C ABI contract with the `project` crate:
//! every field, in this order, must exist identically on both sides.  The
//! state lives in the engine (host) process, so it survives every reload.

/// Maximum number of quads the engine can hold at once.
pub const MAX_QUADS: usize = 8;

/// One jumping quad.  The project DLL moves it each frame; the engine renders
/// it from these fields.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Quad {
    /// Current render position (centre) - updated every frame by the project.
    pub x: f32,
    pub y: f32,
    /// Rest height the quad jumps around.
    pub base_y: f32,
    /// Width and height in pixels.
    pub w: f32,
    pub h: f32,
    /// Horizontal drift speed in pixels per second.
    pub vx: f32,
    /// Oscillation phase used for the jump (radians, advanced by jump_speed).
    pub jump_phase: f32,
    /// How fast the jump oscillation runs (radians per second).
    pub jump_speed: f32,
    /// Peak jump height in pixels above base_y.
    pub jump_height: f32,
    /// Fill colour as 0xRRGGBBAA.
    pub color: u32,
}

impl Default for Quad {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            base_y: 0.0,
            w: 46.0,
            h: 46.0,
            vx: 60.0,
            jump_phase: 0.0,
            jump_speed: 2.0,
            jump_height: 60.0,
            color: 0xFF_888888,
        }
    }
}

/// The full engine-owned game state.  A raw pointer to this is handed to the
/// project DLL through [`crate::ProjectApi::get_state`].
#[repr(C)]
pub struct GameState {
    /// Seconds since the engine started.
    pub tick: f32,
    /// How many of `quads` are live.
    pub quad_count: usize,
    /// The quads to simulate and render.
    pub quads: [Quad; MAX_QUADS],
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            tick: 0.0,
            quad_count: 0,
            quads: [Quad::default(); MAX_QUADS],
        }
    }
}
