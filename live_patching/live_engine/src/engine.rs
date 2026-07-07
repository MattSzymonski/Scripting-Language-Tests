//! # Engine loop
//!
//! Runs a macroquad window that calls game logic exclusively through the
//! [`HotFnTable`]. The engine itself has no game logic — it's a pure shell.
//!
//! ## Key design point
//!
//! Every frame the engine:
//! 1. Reads current function pointers from the hot table (Acquire)
//! 2. Calls through those pointers — these may have been atomically swapped
//!    by the orchestrator since the last frame
//! 3. The orchestrator can patch the table at any time; the engine always
//!    sees a consistent snapshot for the current frame

use std::sync::Arc;

use macroquad::prelude::*;

use crate::api::{EngineAPI, EngineHandle};
use crate::hot_memory::ExecutableMemory;
use crate::hot_table::HotFnTable;

/// The engine's internal state, exposed opaquely to game code via [`EngineHandle`].
pub struct EngineState {
    pub time: f64,
    pub dt: f32,
    pub frame: u64,
    pub should_quit: bool,
    /// Per-frame key-pressed cache (cleared each frame after update).
    keys_pressed: Vec<i32>,
    /// Currently held keys.
    keys_down: Vec<i32>,
}

impl EngineState {
    fn new() -> Self {
        Self {
            time: 0.0,
            dt: 0.0,
            frame: 0,
            should_quit: false,
            keys_pressed: Vec::new(),
            keys_down: Vec::new(),
        }
    }
}

/// Build the [`EngineAPI`] vtable backed by this engine state.
fn build_api(_state_ptr: EngineHandle) -> EngineAPI {
    EngineAPI {
        get_time: engine_get_time,
        is_key_down: engine_is_key_down,
        is_key_pressed: engine_is_key_pressed,
        draw_rect: engine_draw_rect,
        draw_circle: engine_draw_circle,
        draw_text: engine_draw_text,
        screen_width: engine_screen_width,
        screen_height: engine_screen_height,
        quit: engine_quit,
        _reserved_1: std::ptr::null(),
        _reserved_2: std::ptr::null(),
    }
}

// ---------------------------------------------------------------------------
// Engine API callbacks — these forward to macroquad
// ---------------------------------------------------------------------------

extern "C" fn engine_get_time(handle: EngineHandle) -> f64 {
    let state = unsafe { &*(handle as *const EngineState) };
    state.time
}

extern "C" fn engine_is_key_down(handle: EngineHandle, key: i32) -> i32 {
    let state = unsafe { &*(handle as *const EngineState) };
    state.keys_down.contains(&key) as i32
}

extern "C" fn engine_is_key_pressed(handle: EngineHandle, key: i32) -> i32 {
    let state = unsafe { &*(handle as *const EngineState) };
    state.keys_pressed.contains(&key) as i32
}

extern "C" fn engine_draw_rect(_handle: EngineHandle, x: f32, y: f32, w: f32, h: f32, rgba: u32) {
    let c = unpack_color(rgba);
    draw_rectangle(x, y, w, h, c);
}

extern "C" fn engine_draw_circle(_handle: EngineHandle, x: f32, y: f32, r: f32, rgba: u32) {
    let c = unpack_color(rgba);
    draw_circle(x, y, r, c);
}

extern "C" fn engine_draw_text(
    _handle: EngineHandle,
    text: *const u8,
    x: f32,
    y: f32,
    size: f32,
    rgba: u32,
) {
    let c = unpack_color(rgba);
    let s = unsafe { std::ffi::CStr::from_ptr(text as *const i8) }
        .to_str()
        .unwrap_or("");
    draw_text(s, x, y, size, c);
}

extern "C" fn engine_screen_width(_handle: EngineHandle) -> f32 {
    screen_width()
}

extern "C" fn engine_screen_height(_handle: EngineHandle) -> f32 {
    screen_height()
}

extern "C" fn engine_quit(handle: EngineHandle) {
    let state = unsafe { &mut *(handle as *mut EngineState) };
    state.should_quit = true;
}

fn unpack_color(rgba: u32) -> Color {
    Color::from_rgba(
        ((rgba >> 24) & 0xff) as u8,
        ((rgba >> 16) & 0xff) as u8,
        ((rgba >> 8) & 0xff) as u8,
        (rgba & 0xff) as u8,
    )
}

// ---------------------------------------------------------------------------
// Key mapping — virtual key codes → our i32 codes (shared with game_hot)
// ---------------------------------------------------------------------------

fn collect_input(state: &mut EngineState) {
    state.keys_pressed.clear();
    state.keys_down.clear();

    // Map macroquad keys → integer codes.
    let mappings: &[(KeyCode, i32)] = &[
        (KeyCode::Space, 1),
        (KeyCode::Up, 2),
        (KeyCode::Down, 3),
        (KeyCode::Left, 4),
        (KeyCode::Right, 5),
        (KeyCode::Escape, 6),
        (KeyCode::Enter, 7),
        (KeyCode::W, 8),
        (KeyCode::A, 9),
        (KeyCode::S, 10),
        (KeyCode::D, 11),
    ];

    for &(mc_key, code) in mappings {
        if is_key_down(mc_key) {
            state.keys_down.push(code);
        }
        if is_key_pressed(mc_key) {
            state.keys_pressed.push(code);
        }
    }
}

// ---------------------------------------------------------------------------
// Engine runner
// ---------------------------------------------------------------------------

/// Configuration for the engine window.
pub fn window_conf(title: &str) -> Conf {
    Conf {
        window_title: title.to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

/// Run the engine loop. This function never returns until the game calls
/// `quit` or the window is closed.
///
/// The `hot_table` is shared with the orchestrator via `Arc` — the
/// orchestrator can patch it at any time.
///
/// # Safety
///
/// The caller must ensure the `hot_table` outlives this function call.
pub async fn run(hot_table: Arc<HotFnTable>) {
    // Allocate executable memory for future JIT use.
    // In this prototype, code lives in the loaded cdylib, but the
    // allocator infrastructure is ready for function-level code loading.
    let _hot_mem = ExecutableMemory::alloc_rwx(4096 * 16)
        .expect("failed to allocate executable memory for hot-patching");

    // Ask the game how much world state memory it needs, then allocate it.
    let world_size_fn = hot_table.read_world_size();
    let world_size = world_size_fn();
    let mut world_mem: Vec<u8> = vec![0u8; world_size.max(1)];
    let world_ptr: crate::api::WorldHandle = world_mem.as_mut_ptr() as crate::api::WorldHandle;

    let mut state = EngineState::new();
    let state_ptr: EngineHandle = &mut state as *mut EngineState as EngineHandle;
    let api = build_api(state_ptr);

    // Call init through the hot table, passing the allocated world buffer.
    let init_fn = hot_table.read_init();
    init_fn(&api, state_ptr, world_ptr);

    loop {
        state.dt = get_frame_time();
        state.time += state.dt as f64;
        state.frame += 1;

        collect_input(&mut state);

        // --- LIVE-PATCHING BOUNDARY ---
        let update_fn = hot_table.read_update();
        let render_fn = hot_table.read_render();

        update_fn(&api, state_ptr, world_ptr, state.dt);

        clear_background(BLACK);
        render_fn(&api, state_ptr, world_ptr);
        // --- End live-patching boundary ---

        if state.should_quit {
            break;
        }

        next_frame().await;
    }
}
