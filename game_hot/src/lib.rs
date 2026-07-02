//! # game_hot — Live-patchable game functions
//!
//! This crate is compiled as a **cdylib** (dynamic library) and loaded at
//! runtime by the orchestrator. Every function exported here can be
//! individually patched while the engine runs.
//!
//! ## Rules for hot functions
//!
//! 1. Must be `pub extern "C"` — stable ABI, no name mangling.
//! 2. Must use only `#[repr(C)]` types from the engine's ABI module.
//! 3. No generics, no monomorphization across the boundary.
//! 4. No panics — catch all errors internally.
//! 5. Return values must be plain C types (or void).
//!
//! ## How patching works (prototype)
//!
//! 1. You edit this file (or add new hot functions).
//! 2. The orchestrator's file watcher detects the change.
//! 3. The orchestrator runs `cargo build -p game_hot` to produce a new .dll.
//! 4. The orchestrator loads the new .dll, resolves the function symbols.
//! 5. It calls `HotFnTable::patch_update()` / `patch_render()` to atomically
//!    swap the pointers.
//! 6. The engine picks up the new functions on the next frame.
//! 7. The old .dll stays loaded so in-flight calls complete safely.
//!
//! ## Limitations of this prototype
//!
//! - **Whole-library recompile**: we recompile the entire cdylib, not just
//!   changed functions. True function-level recompilation would require a
//!   custom compiler pipeline (cranelift, LLVM JIT, or incremental object
//!   file linking). This is noted as a **simulation**.
//!
//! - **No state migration**: if you change the meaning of world data, old
//!   state may become invalid. In a production system you'd need versioned
//!   state + migration functions.
//!
//! - **Symbol visibility**: all `extern "C"` functions are exported. In
//!   production you'd use a `.def` file or `__declspec(dllexport)` to
//!   control exactly what's exported.

// ---------------------------------------------------------------------------
// These types mirror live_engine::api. In production you'd share them via
// a separate ABI crate, but for the prototype we duplicate the signatures.
// The struct layout MUST match exactly.
// ---------------------------------------------------------------------------

use std::ffi::c_void;

type EngineHandle = *mut c_void;
type WorldHandle = *mut c_void;

#[repr(C)]
#[allow(dead_code)]
pub struct EngineAPI {
    get_time: extern "C" fn(EngineHandle) -> f64,
    is_key_down: extern "C" fn(EngineHandle, i32) -> i32,
    is_key_pressed: extern "C" fn(EngineHandle, i32) -> i32,
    draw_rect: extern "C" fn(EngineHandle, f32, f32, f32, f32, u32),
    draw_circle: extern "C" fn(EngineHandle, f32, f32, f32, u32),
    draw_text: extern "C" fn(EngineHandle, *const u8, f32, f32, f32, u32),
    screen_width: extern "C" fn(EngineHandle) -> f32,
    screen_height: extern "C" fn(EngineHandle) -> f32,
    quit: extern "C" fn(EngineHandle),
    _reserved_1: *const c_void,
    _reserved_2: *const c_void,
}

// ---------------------------------------------------------------------------
// Key codes (must match engine's key mapping)
// ---------------------------------------------------------------------------

const KEY_SPACE: i32 = 1;
const KEY_UP: i32 = 2;
const KEY_LEFT: i32 = 4;
const KEY_RIGHT: i32 = 5;
const KEY_ESCAPE: i32 = 6;

// ---------------------------------------------------------------------------
// Color helpers
// ---------------------------------------------------------------------------

fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
}

// ---------------------------------------------------------------------------
// == HOT-PATCHABLE FUNCTIONS ==
//
// These are the functions that get live-patched. Edit them, save the file,
// and the orchestrator will reload them without restarting the engine.
// ---------------------------------------------------------------------------

/// Per-frame state stored inside the world handle.
/// Layout must never change across hot-patches.
#[repr(C)]
struct GameState {
    player_x: f32,
    player_y: f32,
    velocity_y: f32,
    score: u32,
    frame_count: u64,
}

/// Returns the byte size the engine should allocate for the game's world state.
/// Must match `sizeof(GameState)`.
#[no_mangle]
pub extern "C" fn game_world_size() -> usize {
    std::mem::size_of::<GameState>()
}

/// Called once at startup.
#[no_mangle]
pub extern "C" fn game_init(_api: *const EngineAPI, _engine: EngineHandle, world: WorldHandle) {
    if world.is_null() {
        return;
    }
    let state = unsafe { &mut *(world as *mut GameState) };
    // Use the screen dimensions (first call may not have them yet,
    // so we set defaults and update in the first frame).
    state.player_x = 400.0;
    state.player_y = 300.0;
    state.velocity_y = 0.0;
    state.score = 0;
    state.frame_count = 0;
}

/// Called every frame to update game logic.
///
/// **TRY EDITING THIS FUNCTION** while the game runs:
/// - Change the gravity constant
/// - Change the flap velocity
/// - Add new mechanics
/// - Save the file, and watch the orchestrator hot-patch it!
#[no_mangle]
pub extern "C" fn game_update(
    api: *const EngineAPI,
    engine: EngineHandle,
    world: WorldHandle,
    dt: f32,
) {
    if api.is_null() || world.is_null() {
        return;
    }
    let api = unsafe { &*api };
    let state = unsafe { &mut *(world as *mut GameState) };

    state.frame_count += 1;

    // --- Quit on Escape ---
    if (api.is_key_pressed)(engine, KEY_ESCAPE) != 0 {
        (api.quit)(engine);
        return;
    }

    // --- Movement ---
    // Left/Right arrows to move horizontally.
    if (api.is_key_down)(engine, KEY_LEFT) != 0 {
        state.player_x -= 300.0 * dt;
    }
    if (api.is_key_down)(engine, KEY_RIGHT) != 0 {
        state.player_x += 300.0 * dt;
    }

    // --- Gravity + Flap (jump) ---
    const GRAVITY: f32 = 800.0;
    const FLAP_VELOCITY: f32 = -800.0;
    const GROUND_Y: f32 = 550.0;

    // Flap on Space or Up.
    if (api.is_key_pressed)(engine, KEY_SPACE) != 0 || (api.is_key_pressed)(engine, KEY_UP) != 0 {
        state.velocity_y = FLAP_VELOCITY;
    }

    state.velocity_y += GRAVITY * dt;
    state.player_y += state.velocity_y * dt;

    // Ground collision.
    if state.player_y > GROUND_Y {
        state.player_y = GROUND_Y;
        state.velocity_y = 0.0;
    }

    // Ceiling.
    if state.player_y < 20.0 {
        state.player_y = 20.0;
        state.velocity_y = 0.0;
    }

    // Screen bounds.
    let w = (api.screen_width)(engine);
    state.player_x = state.player_x.clamp(20.0, w - 20.0);

    // Score just increments with time for demo purposes.
    state.score = (state.frame_count / 60) as u32;
}

/// Called every frame to render.
///
/// **TRY EDITING THIS FUNCTION** too — change colors, add shapes, etc.
#[no_mangle]
pub extern "C" fn game_render(api: *const EngineAPI, engine: EngineHandle, world: WorldHandle) {
    if api.is_null() || world.is_null() {
        return;
    }
    let api = unsafe { &*api };
    let state = unsafe { &*(world as *const GameState) };

    let w = (api.screen_width)(engine);
    let h = (api.screen_height)(engine);

    // Background gradient (two rectangles).
    (api.draw_rect)(engine, 0.0, 0.0, w, h * 0.5, rgba(90, 30, 60, 255));
    (api.draw_rect)(engine, 0.0, h * 0.5, w, h * 0.5, rgba(20, 20, 40, 255));

    // Ground line.
    (api.draw_rect)(engine, 0.0, 550.0, w, 4.0, rgba(100, 200, 100, 255));

    // Player (a circle).
    let px = state.player_x;
    let py = state.player_y;
    (api.draw_circle)(engine, px, py, 16.0, rgba(255, 200, 50, 255));
    (api.draw_circle)(engine, px, py, 16.0, rgba(255, 255, 255, 60));

    // Score text.
    let score_text = format!("Score: {}\0", state.score);
    (api.draw_text)(
        engine,
        score_text.as_ptr(),
        16.0,
        16.0,
        28.0,
        rgba(255, 255, 255, 255),
    );

    // Hot-patch indicator.
    let hint = b"HOT-PATCHABLE GAME v1\0";
    (api.draw_text)(
        engine,
        hint.as_ptr(),
        16.0,
        h - 32.0,
        16.0,
        rgba(150, 150, 150, 200),
    );
}
