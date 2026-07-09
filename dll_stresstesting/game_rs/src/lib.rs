//! # game_rs — hot-reloadable Flappy Bird logic
//!
//! Compiled as a **cdylib** and loaded at runtime by `flappy` (`cargo run`),
//! mirroring `live_patching/game_hot`'s pattern: edit this crate's source
//! while the game runs, save, and the host rebuilds + atomically swaps the
//! function pointers on the next frame. No restart.
//!
//! ## Rules for the exported functions
//!
//! 1. Must be `#[no_mangle] pub extern "C" fn` — stable ABI, no name mangling.
//! 2. Must only touch [`api::EngineApi`] (a mirror of `flappy::ffi::EngineApi`,
//!    kept in lockstep field-for-field) and the raw `world` buffer — never link
//!    macroquad directly (see `api.rs` for why).
//! 3. `world` points at a fixed-size buffer the host allocates **once**, sized
//!    via [`game_world_size`], and keeps alive across every reload — so unlike
//!    the C# hot-reload path, gameplay state (bird position, pipes, score)
//!    survives a reload here.

mod api;
mod game;

use std::ffi::c_void;

use api::EngineApi;
use game::GameState;

/// Returns the byte size the host should allocate for the world buffer.
#[no_mangle]
pub extern "C" fn game_world_size() -> usize {
    std::mem::size_of::<GameState>()
}

/// Called once at startup (and again after each reload's world buffer is
/// (re)allocated — in practice only once, since the host allocates it once).
#[no_mangle]
pub extern "C" fn game_init(api: *const EngineApi, world: *mut c_void) {
    if api.is_null() || world.is_null() {
        return;
    }
    let api = unsafe { &*api };
    let state = unsafe { &mut *(world as *mut GameState) };
    game::init(state, (api.screen_height)());
}

/// Called every frame to advance game logic.
///
/// **Try editing this while the game runs** — e.g. tweak `GRAVITY` or
/// `FLAP_VELOCITY` in `game.rs`'s `config` module — and save.
#[no_mangle]
pub extern "C" fn game_update(api: *const EngineApi, world: *mut c_void, dt: f32) {
    if api.is_null() || world.is_null() {
        return;
    }
    let api = unsafe { &*api };
    let state = unsafe { &mut *(world as *mut GameState) };
    game::update(state, api, dt);
}

/// Called every frame, after `game_update`, to draw.
#[no_mangle]
pub extern "C" fn game_draw(api: *const EngineApi, world: *mut c_void) {
    if api.is_null() || world.is_null() {
        return;
    }
    let api = unsafe { &*api };
    let state = unsafe { &*(world as *const GameState) };
    game::draw(state, api);
}
