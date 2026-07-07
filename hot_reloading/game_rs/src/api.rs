//! Mirror of `flappy::ffi::EngineApi`.
//!
//! This crate is compiled as a **cdylib** and must not link macroquad itself —
//! a second copy of macroquad's global state here would be disconnected from
//! the host's window/GL context. So, exactly like `live_patching/game_hot`
//! mirrors `live_engine::api::EngineAPI`, we duplicate just the function-
//! pointer *signatures* here and call through them; the real implementations
//! (which do link macroquad) live in the host, in `flappy/src/ffi.rs`.
//!
//! ## Golden rule
//!
//! This struct's fields must stay in the exact same order and types as
//! `flappy::ffi::EngineApi`. Add new fields to the end of both.

#[repr(C)]
pub struct EngineApi {
    pub screen_width: extern "C" fn() -> f32,
    pub screen_height: extern "C" fn() -> f32,
    pub get_time: extern "C" fn() -> f64,

    pub key_pressed: extern "C" fn(i32) -> i32,
    pub mouse_left_pressed: extern "C" fn() -> i32,

    pub clear_background: extern "C" fn(u32),
    pub draw_rectangle: extern "C" fn(f32, f32, f32, f32, u32),
    pub draw_rectangle_lines: extern "C" fn(f32, f32, f32, f32, f32, u32),
    pub draw_circle: extern "C" fn(f32, f32, f32, u32),
    pub draw_circle_lines: extern "C" fn(f32, f32, f32, f32, u32),
    pub draw_line: extern "C" fn(f32, f32, f32, f32, f32, u32),
    pub draw_triangle: extern "C" fn(f32, f32, f32, f32, f32, f32, u32),
    pub draw_text: extern "C" fn(*const u8, f32, f32, f32, u32),
    pub measure_text_width: extern "C" fn(*const u8, f32) -> f32,

    pub rand_range: extern "C" fn(f32, f32) -> f32,
    pub quit: extern "C" fn(),
}

/// Key codes shared with the host's `map_key` in `flappy/src/ffi.rs`.
pub mod key {
    pub const SPACE: i32 = 1;
    pub const UP: i32 = 2;
    pub const ESCAPE: i32 = 6;
}
