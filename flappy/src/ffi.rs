//! The C ABI surface the C# game calls back into.
//!
//! [`EngineApi`] is a `#[repr(C)]` table of function pointers whose layout must
//! match the `EngineApi` struct on the C# side *field for field, in order*.
//! Each function simply forwards to macroquad.

use std::ffi::CStr;
use std::sync::atomic::{AtomicBool, Ordering};

use macroquad::prelude::*;

/// Set when C# calls `quit()`; read by the host loop to stop the engine.
static QUIT: AtomicBool = AtomicBool::new(false);

pub fn quit_requested() -> bool {
    QUIT.load(Ordering::Relaxed)
}

/// Table of native callbacks handed to the managed side. The field order here
/// is the contract — keep it identical to `Flappy.EngineApi` in C#.
#[repr(C)]
pub struct EngineApi {
    pub screen_width: extern "C" fn() -> f32,
    pub screen_height: extern "C" fn() -> f32,
    pub get_time: extern "C" fn() -> f64,

    pub key_pressed: extern "C" fn(i32) -> i32,
    pub key_down: extern "C" fn(i32) -> i32,
    pub mouse_pressed: extern "C" fn(i32) -> i32,
    pub mouse_x: extern "C" fn() -> f32,
    pub mouse_y: extern "C" fn() -> f32,

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

impl EngineApi {
    pub fn new() -> Self {
        Self {
            screen_width: ffi_screen_width,
            screen_height: ffi_screen_height,
            get_time: ffi_get_time,
            key_pressed: ffi_key_pressed,
            key_down: ffi_key_down,
            mouse_pressed: ffi_mouse_pressed,
            mouse_x: ffi_mouse_x,
            mouse_y: ffi_mouse_y,
            clear_background: ffi_clear_background,
            draw_rectangle: ffi_draw_rectangle,
            draw_rectangle_lines: ffi_draw_rectangle_lines,
            draw_circle: ffi_draw_circle,
            draw_circle_lines: ffi_draw_circle_lines,
            draw_line: ffi_draw_line,
            draw_triangle: ffi_draw_triangle,
            draw_text: ffi_draw_text,
            measure_text_width: ffi_measure_text_width,
            rand_range: ffi_rand_range,
            quit: ffi_quit,
        }
    }
}

/// Unpack a `0xRRGGBBAA` integer into a macroquad [`Color`].
fn color(rgba: u32) -> Color {
    Color::from_rgba(
        ((rgba >> 24) & 0xff) as u8,
        ((rgba >> 16) & 0xff) as u8,
        ((rgba >> 8) & 0xff) as u8,
        (rgba & 0xff) as u8,
    )
}

/// Map the integer key codes shared with C# (`Flappy.Key`) onto macroquad keys.
fn map_key(code: i32) -> Option<KeyCode> {
    Some(match code {
        1 => KeyCode::Space,
        2 => KeyCode::Up,
        3 => KeyCode::Down,
        4 => KeyCode::Left,
        5 => KeyCode::Right,
        6 => KeyCode::Escape,
        7 => KeyCode::Enter,
        _ => return None,
    })
}

fn map_mouse(button: i32) -> MouseButton {
    match button {
        1 => MouseButton::Right,
        2 => MouseButton::Middle,
        _ => MouseButton::Left,
    }
}

/// Read a null-terminated UTF-8 string passed from managed code.
///
/// # Safety
/// `ptr` must point to a valid null-terminated UTF-8 buffer.
unsafe fn str_from<'a>(ptr: *const u8) -> &'a str {
    if ptr.is_null() {
        return "";
    }
    CStr::from_ptr(ptr as *const i8).to_str().unwrap_or("")
}

// --- callbacks -------------------------------------------------------------

extern "C" fn ffi_screen_width() -> f32 {
    screen_width()
}
extern "C" fn ffi_screen_height() -> f32 {
    screen_height()
}
extern "C" fn ffi_get_time() -> f64 {
    get_time()
}

extern "C" fn ffi_key_pressed(code: i32) -> i32 {
    map_key(code).map_or(0, |k| is_key_pressed(k) as i32)
}
extern "C" fn ffi_key_down(code: i32) -> i32 {
    map_key(code).map_or(0, |k| is_key_down(k) as i32)
}
extern "C" fn ffi_mouse_pressed(button: i32) -> i32 {
    is_mouse_button_pressed(map_mouse(button)) as i32
}
extern "C" fn ffi_mouse_x() -> f32 {
    mouse_position().0
}
extern "C" fn ffi_mouse_y() -> f32 {
    mouse_position().1
}

extern "C" fn ffi_clear_background(c: u32) {
    clear_background(color(c));
}
extern "C" fn ffi_draw_rectangle(x: f32, y: f32, w: f32, h: f32, c: u32) {
    draw_rectangle(x, y, w, h, color(c));
}
extern "C" fn ffi_draw_rectangle_lines(x: f32, y: f32, w: f32, h: f32, t: f32, c: u32) {
    draw_rectangle_lines(x, y, w, h, t, color(c));
}
extern "C" fn ffi_draw_circle(x: f32, y: f32, r: f32, c: u32) {
    draw_circle(x, y, r, color(c));
}
extern "C" fn ffi_draw_circle_lines(x: f32, y: f32, r: f32, t: f32, c: u32) {
    draw_circle_lines(x, y, r, t, color(c));
}
extern "C" fn ffi_draw_line(x1: f32, y1: f32, x2: f32, y2: f32, t: f32, c: u32) {
    draw_line(x1, y1, x2, y2, t, color(c));
}
extern "C" fn ffi_draw_triangle(ax: f32, ay: f32, bx: f32, by: f32, cx: f32, cy: f32, c: u32) {
    draw_triangle(vec2(ax, ay), vec2(bx, by), vec2(cx, cy), color(c));
}
extern "C" fn ffi_draw_text(text: *const u8, x: f32, y: f32, size: f32, c: u32) {
    let s = unsafe { str_from(text) };
    draw_text(s, x, y, size, color(c));
}
extern "C" fn ffi_measure_text_width(text: *const u8, size: f32) -> f32 {
    let s = unsafe { str_from(text) };
    measure_text(s, None, size as u16, 1.0).width
}

extern "C" fn ffi_rand_range(min: f32, max: f32) -> f32 {
    rand::gen_range(min, max)
}
extern "C" fn ffi_quit() {
    QUIT.store(true, Ordering::Relaxed);
}
