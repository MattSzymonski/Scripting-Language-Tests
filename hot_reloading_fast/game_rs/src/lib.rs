//! # game_rs — hot-patchable Flappy Bird logic
//!
//! Statically linked into `flappy` as a normal dependency and hot-patched at
//! runtime via `subsecond`/`dx serve --hot-patch`: edit this crate's source
//! while the game runs, save, and the *already-running* process gets a
//! function-level patch — no rebuild-and-reload of a whole module, no restart.
//!
//! ## Rules for the exported functions
//!
//! 1. Kept as `#[no_mangle] pub extern "C" fn` with raw pointers (rather than
//!    plain `pub fn`) so `flappy`'s call sites stay identical in shape to
//!    before — only the linking model (static rlib vs. loaded cdylib) changed.
//! 2. Must only touch [`api::EngineApi`] (a mirror of `flappy::ffi::EngineApi`,
//!    kept in lockstep field-for-field) and the raw `world` buffer — never link
//!    macroquad directly (see `api.rs` for why).
//! 3. `world` points at a fixed-size buffer the host allocates **once**, sized
//!    via [`game_world_size`], and keeps alive across every patch — so unlike
//!    the C# hot-reload path, gameplay state (bird position, pipes, score)
//!    survives a reload here.

pub mod api;
/// Auto-generated bulk code (12 files, ~253k lines) - exists only to make
/// this crate much heavier for a hot-patch speed-at-scale experiment.
mod bulk_00;
mod bulk_01;
mod bulk_02;
mod bulk_03;
mod bulk_04;
mod bulk_05;
mod bulk_06;
mod bulk_07;
mod bulk_08;
mod bulk_09;
mod bulk_10;
mod bulk_11;
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

    // Keeps every generated bulk module genuinely reachable so none of it
    // gets stripped - the result is meaningless, just discarded.
    std::hint::black_box(
        bulk_00::touch_bulk_0()
            + bulk_01::touch_bulk_1()
            + bulk_02::touch_bulk_2()
            + bulk_03::touch_bulk_3()
            + bulk_04::touch_bulk_4()
            + bulk_05::touch_bulk_5()
            + bulk_06::touch_bulk_6()
            + bulk_07::touch_bulk_7()
            + bulk_08::touch_bulk_8()
            + bulk_09::touch_bulk_9()
            + bulk_10::touch_bulk_10()
            + bulk_11::touch_bulk_11(),
    );
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
