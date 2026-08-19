//! The C-ABI bridge between the engine and the hot-reloaded project DLL.
//!
//! The project DLL must not link macroquad (a second copy of macroquad's
//! global state would be disconnected from the host's window).  Instead the
//! host hands the DLL a [`ProjectApi`] table of plain function pointers, and
//! the DLL calls through that table.  This mirrors the `game_rs` / `flappy`
//! API pattern already used in `hot_reloading/`.

use std::ffi::c_void;
use std::sync::atomic::{AtomicPtr, AtomicU32, Ordering};

use macroquad::prelude::*;

use crate::state::GameState;

/// Pointer to the single engine-owned [`GameState`].  Set once at startup and
/// never moved afterwards (the state lives in a heap `Box`), so the address
/// stays valid for the whole process.
static GAME_STATE_POINTER: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

/// Cached window size.  macroquad's `screen_width()`/`screen_height()` can
/// only be called on the main thread, but `project_init` may run on the file-
/// watcher thread - so the API reads this cache instead of macroquad directly.
static SCREEN_WIDTH_BITS: AtomicU32 = AtomicU32::new(0);
static SCREEN_HEIGHT_BITS: AtomicU32 = AtomicU32::new(0);

/// Record where the engine-owned state lives so the API can hand it out.
pub(crate) fn set_game_state_pointer(state: *mut GameState) {
    GAME_STATE_POINTER.store(state as *mut c_void, Ordering::Release);
}

/// Raw pointer to the engine-owned state (used by the engine and the API).
pub(crate) fn state_pointer() -> *mut GameState {
    GAME_STATE_POINTER.load(Ordering::Acquire) as *mut GameState
}

/// Refresh the cached window size.  Must run on the main thread (the render
/// loop calls it every frame; the host also calls it once at startup).
pub(crate) fn update_screen_size_cache() {
    SCREEN_WIDTH_BITS.store(screen_width().to_bits(), Ordering::Relaxed);
    SCREEN_HEIGHT_BITS.store(screen_height().to_bits(), Ordering::Relaxed);
}

extern "C" fn api_get_state() -> *mut c_void {
    state_pointer() as *mut c_void
}

extern "C" fn api_screen_width() -> f32 {
    f32::from_bits(SCREEN_WIDTH_BITS.load(Ordering::Relaxed))
}

extern "C" fn api_screen_height() -> f32 {
    f32::from_bits(SCREEN_HEIGHT_BITS.load(Ordering::Relaxed))
}

/// Table of callbacks handed to every freshly loaded project DLL via its
/// `project_set_api` export.  Field order is the contract - keep it identical
/// to the mirror in `project/src/lib.rs`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ProjectApi {
    /// Returns a raw pointer to the engine-owned [`GameState`].
    pub get_state: extern "C" fn() -> *mut c_void,
    /// Current window width in pixels.
    pub screen_width: extern "C" fn() -> f32,
    /// Current window height in pixels.
    pub screen_height: extern "C" fn() -> f32,
}

impl ProjectApi {
    pub fn new() -> Self {
        Self {
            get_state: api_get_state,
            screen_width: api_screen_width,
            screen_height: api_screen_height,
        }
    }
}
