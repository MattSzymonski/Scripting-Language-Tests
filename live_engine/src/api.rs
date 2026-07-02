//! # ABI types shared between the engine and hot-patchable game functions.
//!
//! ## Design rules (hard constraints for live-patching)
//!
//! 1. **All types crossing the boundary must be `#[repr(C)]`** — no Rust
//!    layout optimisations allowed.
//! 2. **No complex Rust types** (Vec, String, Box<dyn Trait>, enums with data).
//!    Use opaque pointers/handles and primitive types only.
//! 3. **Struct layouts must never change** after the first deployment.
//!    Add new fields at the end only.  Use a `version` field if needed.
//! 4. **Function signatures are `extern "C"`** — stable ABI, no mangling.

use std::ffi::c_void;

// ---------------------------------------------------------------------------
// Opaque handles (the engine owns the real data behind these)
// ---------------------------------------------------------------------------

/// Opaque pointer to the engine's internal state.
/// The game receives this but never dereferences it directly —
/// all interaction goes through [`EngineAPI`].
pub type EngineHandle = *mut c_void;

/// Opaque pointer to the game world / scene state.
/// Owned by the engine, passed to game functions for manipulation.
pub type WorldHandle = *mut c_void;

// ---------------------------------------------------------------------------
// Engine → Game: the services the engine exposes to hot functions
// ---------------------------------------------------------------------------

/// Table of function pointers the engine hands to game code.
/// This is the **only** way game code interacts with the engine.
///
/// ## Stability
/// Fields can be *appended* but never removed or reordered.
/// Mark unused slots as reserved/null.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EngineAPI {
    /// Get elapsed time in seconds.
    pub get_time: extern "C" fn(EngineHandle) -> f64,

    /// Is a key currently held? (virtual key code → bool)
    pub is_key_down: extern "C" fn(EngineHandle, i32) -> i32,

    /// Was a key pressed this frame?
    pub is_key_pressed: extern "C" fn(EngineHandle, i32) -> i32,

    /// Draw a filled rectangle.
    pub draw_rect: extern "C" fn(EngineHandle, f32, f32, f32, f32, u32),

    /// Draw a circle.
    pub draw_circle: extern "C" fn(EngineHandle, f32, f32, f32, u32),

    /// Draw text.
    pub draw_text: extern "C" fn(EngineHandle, *const u8, f32, f32, f32, u32),

    /// Get screen width.
    pub screen_width: extern "C" fn(EngineHandle) -> f32,

    /// Get screen height.
    pub screen_height: extern "C" fn(EngineHandle) -> f32,

    /// Request engine shutdown.
    pub quit: extern "C" fn(EngineHandle),

    /// Reserved for future use — must be null.
    pub _reserved_1: *const c_void,
    pub _reserved_2: *const c_void,
}

// ---------------------------------------------------------------------------
// Game function signatures — the "hot" entry points
// ---------------------------------------------------------------------------

/// Called every frame to update game logic.
/// - `api`: engine services table
/// - `engine`: opaque engine handle (for API callbacks)
/// - `world`: opaque world handle (for game state)
/// - `dt`: delta time in seconds since last frame
pub type GameUpdateFn =
    extern "C" fn(api: *const EngineAPI, engine: EngineHandle, world: WorldHandle, dt: f32);

/// Called every frame to render the game.
pub type GameRenderFn =
    extern "C" fn(api: *const EngineAPI, engine: EngineHandle, world: WorldHandle);

/// Called once at startup.
pub type GameInitFn =
    extern "C" fn(api: *const EngineAPI, engine: EngineHandle, world: WorldHandle);

/// Returns the byte size the game needs for its world state.
/// Called by the engine before init so it can allocate the right buffer.
pub type GameWorldSizeFn = extern "C" fn() -> usize;
