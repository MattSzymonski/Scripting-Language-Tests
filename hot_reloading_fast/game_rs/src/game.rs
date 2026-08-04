//! Flappy Bird game logic — the actual gameplay for the hot-reloaded Rust
//! scenario.
//!
//! Two constraints shape this file, both stemming from living in a `cdylib`
//! that's loaded and swapped at runtime instead of being linked into `flappy`
//! directly:
//! - No `Vec<Pipe>` — pipes live in a fixed-size array inside [`GameState`],
//!   since the world state is a raw byte buffer the host allocates once and
//!   keeps alive across every reload (see `flappy/src/hot_rs.rs`).
//! - No direct macroquad calls — every draw/input/random operation goes
//!   through the [`EngineApi`](crate::api::EngineApi) function-pointer table
//!   the host hands us, exactly like the C# port does.

use crate::api::{key, EngineApi};

mod config {
    pub const GROUND_HEIGHT: f32 = 96.0;

    pub const BIRD_RADIUS: f32 = 228.0;
    pub const GRAVITY: f32 = 1500.0;
    pub const FLAP_VELOCITY: f32 = -470.0;
    pub const BIRD_X: f32 = 120.0;

    pub const PIPE_WIDTH: f32 = 78.0;
    pub const PIPE_GAP: f32 = 185.0;
    pub const SCROLL_SPEED: f32 = 175.0;
    pub const SPAWN_INTERVAL: f32 = 1.5;
    pub const GAP_MARGIN: f32 = 90.0;
}

mod color {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
        ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
    }

    pub const SKY: u32 = rgba(90, 90, 90, 255);
    pub const SKY_DEEP: u32 = rgba(90, 90, 90, 255);
    pub const WHITE: u32 = rgba(255, 255, 255, 255);
    pub const WHITE_CLOUD: u32 = rgba(255, 255, 255, 217);
    pub const BLACK: u32 = rgba(0, 0, 0, 255);
    pub const BLACK_SHADOW: u32 = rgba(0, 0, 0, 100);
    pub const BLACK_OVERLAY: u32 = rgba(0, 0, 0, 90);
    pub const BLACK_TEXT_SHADOW: u32 = rgba(0, 0, 0, 115);
    pub const GOLD: u32 = rgba(255, 205, 40, 255);
    pub const RED: u32 = rgba(225, 60, 55, 255);
    pub const DARK_GREEN: u32 = rgba(40, 130, 40, 255);
    pub const GREEN: u32 = rgba(70, 190, 70, 255);
    pub const GROUND: u32 = rgba(220, 190, 105, 255);
    pub const GROUND_GRASS: u32 = rgba(125, 200, 90, 255);
    pub const BIRD_GOLD: u32 = rgba(120, 120, 120, 255);
    pub const BIRD_ORANGE: u32 = rgba(150, 150, 150, 255);
}

// ---------------------------------------------------------------------------
// World state — layout must stay stable across reloads (append-only).
// ---------------------------------------------------------------------------

pub const MAX_PIPES: usize = 16;

pub const STATE_READY: i32 = 0;
pub const STATE_PLAYING: i32 = 1;
pub const STATE_GAME_OVER: i32 = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PipeState {
    pub x: f32,
    pub gap_y: f32,
    pub width: f32,
    pub gap: f32,
    pub scored: u8,
}

impl PipeState {
    const ZERO: PipeState = PipeState {
        x: 0.0,
        gap_y: 0.0,
        width: 0.0,
        gap: 0.0,
        scored: 0,
    };
}

#[repr(C)]
pub struct GameState {
    pub state: i32,
    pub bird_x: f32,
    pub bird_y: f32,
    pub bird_vel: f32,
    pub bird_rotation: f32,
    pub bird_dead: u8,
    pub pipes: [PipeState; MAX_PIPES],
    pub pipe_count: u32,
    pub spawn_timer: f32,
    pub score: u32,
    pub best: u32,
}

struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Rect {
    fn overlaps(&self, o: &Rect) -> bool {
        self.x < o.x + o.w && self.x + self.w > o.x && self.y < o.y + o.h && self.y + self.h > o.y
    }
}

fn pipe_solids(pipe: &PipeState, screen_h: f32) -> (Rect, Rect) {
    let top_h = pipe.gap_y - pipe.gap / 2.0;
    let bottom_y = pipe.gap_y + pipe.gap / 2.0;
    let bottom_h = screen_h - config::GROUND_HEIGHT - bottom_y;
    (
        Rect { x: pipe.x, y: 0.0, w: pipe.width, h: top_h },
        Rect { x: pipe.x, y: bottom_y, w: pipe.width, h: bottom_h },
    )
}

// ---------------------------------------------------------------------------
// Lifecycle
// ---------------------------------------------------------------------------

pub fn init(state: &mut GameState, screen_h: f32) {
    *state = GameState {
        state: STATE_READY,
        bird_x: config::BIRD_X,
        bird_y: screen_h * 0.15,
        bird_vel: 0.0,
        bird_rotation: 0.0,
        bird_dead: 0,
        pipes: [PipeState::ZERO; MAX_PIPES],
        pipe_count: 0,
        spawn_timer: 0.0,
        score: 0,
        best: 0,
    };
}

fn reset(state: &mut GameState, screen_h: f32) {
    state.state = STATE_READY;
    state.bird_x = config::BIRD_X;
    state.bird_y = screen_h * 0.15;
    state.bird_vel = 0.0;
    state.bird_rotation = 0.0;
    state.bird_dead = 0;
    state.pipe_count = 0;
    state.spawn_timer = 0.0;
    state.score = 0;
    // `best` intentionally carries over.
}

// ---------------------------------------------------------------------------
// Update
// ---------------------------------------------------------------------------

pub fn update(state: &mut GameState, api: &EngineApi, dt: f32) {
    let screen_w = (api.screen_width)();
    let screen_h = (api.screen_height)();

    if (api.key_pressed)(key::ESCAPE) != 0 {
        (api.quit)();
        return;
    }

    let flap_input = (api.key_pressed)(key::SPACE) != 0
        || (api.key_pressed)(key::UP) != 0
        || (api.mouse_left_pressed)() != 0;

    match state.state {
        STATE_READY => {
            let base_y = screen_h * 0.15;
            let t = (api.get_time)() as f32;
            state.bird_y = base_y + (t * 3.0).sin() * 10.0;
            if flap_input {
                state.state = STATE_PLAYING;
                state.bird_vel = config::FLAP_VELOCITY;
            }
        }
        STATE_PLAYING => {
            bird_update(state, dt, screen_h, flap_input);
            update_playing(state, api, dt, screen_w, screen_h);
        }
        STATE_GAME_OVER => {
            let floor = screen_h - config::GROUND_HEIGHT;
            if state.bird_y + config::BIRD_RADIUS < floor {
                state.bird_vel += config::GRAVITY * dt;
                state.bird_y += state.bird_vel * dt;
            }
            if flap_input {
                reset(state, screen_h);
            }
        }
        _ => {}
    }
}

fn bird_update(state: &mut GameState, dt: f32, screen_h: f32, flap_input: bool) {
    if flap_input {
        state.bird_vel = config::FLAP_VELOCITY;
    }

    state.bird_vel += config::GRAVITY * dt;
    state.bird_y += state.bird_vel * dt;

    if state.bird_y < config::BIRD_RADIUS {
        state.bird_y = config::BIRD_RADIUS;
        state.bird_vel = 0.0;
    }

    let floor = screen_h - config::GROUND_HEIGHT;
    if state.bird_y + config::BIRD_RADIUS >= floor {
        state.bird_y = floor - config::BIRD_RADIUS;
        state.bird_dead = 1;
    }

    let target = (state.bird_vel / 600.0).clamp(-0.5, 1.4);
    state.bird_rotation += (target - state.bird_rotation) * (10.0 * dt).min(1.0);
}

fn spawn_pipe(state: &mut GameState, api: &EngineApi, screen_w: f32, screen_h: f32) {
    if state.pipe_count as usize >= MAX_PIPES {
        return; // Safety valve; never hit at normal spawn/scroll rates.
    }
    let floor = screen_h - config::GROUND_HEIGHT;
    let gap_y = (api.rand_range)(config::GAP_MARGIN, floor - config::GAP_MARGIN);
    state.pipes[state.pipe_count as usize] = PipeState {
        x: screen_w,
        gap_y,
        width: config::PIPE_WIDTH,
        gap: config::PIPE_GAP,
        scored: 0,
    };
    state.pipe_count += 1;
}

fn update_playing(state: &mut GameState, api: &EngineApi, dt: f32, screen_w: f32, screen_h: f32) {
    state.spawn_timer += dt;
    if state.spawn_timer >= config::SPAWN_INTERVAL {
        state.spawn_timer -= config::SPAWN_INTERVAL;
        spawn_pipe(state, api, screen_w, screen_h);
    }

    for i in 0..state.pipe_count as usize {
        state.pipes[i].x -= config::SCROLL_SPEED * dt;
    }

    // Reap off-screen pipes by compacting the array in place.
    let mut write = 0;
    for read in 0..state.pipe_count as usize {
        if state.pipes[read].x + state.pipes[read].width >= 0.0 {
            state.pipes[write] = state.pipes[read];
            write += 1;
        }
    }
    state.pipe_count = write as u32;

    let half = config::BIRD_RADIUS * 0.85;
    let bird_box = Rect {
        x: state.bird_x - half,
        y: state.bird_y - half,
        w: half * 2.0,
        h: half * 2.0,
    };

    for i in 0..state.pipe_count as usize {
        let pipe = &mut state.pipes[i];
        if pipe.scored == 0 && pipe.x + pipe.width < state.bird_x {
            pipe.scored = 1;
            state.score += 1;
        }
        let (top, bottom) = pipe_solids(pipe, screen_h);
        if top.overlaps(&bird_box) || bottom.overlaps(&bird_box) {
            state.bird_dead = 1;
        }
    }

    if state.bird_dead != 0 {
        state.best = state.best.max(state.score);
        state.state = STATE_GAME_OVER;
    }
}

// ---------------------------------------------------------------------------
// Draw
// ---------------------------------------------------------------------------

pub fn draw(state: &GameState, api: &EngineApi) {
    let screen_w = (api.screen_width)();
    let screen_h = (api.screen_height)();

    draw_background(api, screen_w, screen_h);

    for i in 0..state.pipe_count as usize {
        draw_pipe(&state.pipes[i], api, screen_h);
    }

    draw_ground(api, screen_w, screen_h);
    draw_bird(state, api);
    draw_hud(state, api, screen_w, screen_h);
}

fn draw_background(api: &EngineApi, screen_w: f32, screen_h: f32) {
    (api.clear_background)(color::SKY);
    (api.draw_rectangle)(0.0, 0.0, screen_w, screen_h * 0.55, color::SKY_DEEP);


}

fn draw_ground(api: &EngineApi, screen_w: f32, screen_h: f32) {
    let top = screen_h - config::GROUND_HEIGHT;
    (api.draw_rectangle)(0.0, top, screen_w, config::GROUND_HEIGHT, color::GROUND);
    (api.draw_rectangle)(0.0, top, screen_w, 14.0, color::GROUND_GRASS);
    (api.draw_line)(0.0, top, screen_w, top, 4.0, color::DARK_GREEN);
}

fn draw_bird(state: &GameState, api: &EngineApi) {
    let x = state.bird_x;
    let y = state.bird_y;
    let r = config::BIRD_RADIUS;

    (api.draw_circle)(x, y, r, color::BIRD_GOLD);
    (api.draw_circle_lines)(x, y, r, 2.0, color::BIRD_ORANGE);

    // Beak: a triangle pointing the way the bird is tilted.
    let (dx, dy) = (state.bird_rotation.cos(), state.bird_rotation.sin());
    let tip_x = x + dx * (r + 8.0);
    let tip_y = y + dy * (r + 8.0);
    let perp_x = -dy * (r * 0.4);
    let perp_y = dx * (r * 0.4);
    (api.draw_triangle)(
        x + dx * r + perp_x,
        y + dy * r + perp_y,
        x + dx * r - perp_x,
        y + dy * r - perp_y,
        tip_x,
        tip_y,
        color::BIRD_ORANGE,
    );

    // Eye.
    (api.draw_circle)(x + r * 0.35, y - r * 0.35, r * 0.18, color::WHITE);
    (api.draw_circle)(x + r * 0.42, y - r * 0.35, r * 0.09, color::BLACK);
}

fn draw_pipe(pipe: &PipeState, api: &EngineApi, screen_h: f32) {
    let (top, bottom) = pipe_solids(pipe, screen_h);

    for solid in [&top, &bottom] {
        (api.draw_rectangle)(solid.x, solid.y, solid.w, solid.h, color::GREEN);
        (api.draw_rectangle_lines)(solid.x, solid.y, solid.w, solid.h, 4.0, color::DARK_GREEN);
    }

    // Classic wider "lip" at each side of the gap.
    const LIP_H: f32 = 26.0;
    const OVERHANG: f32 = 6.0;
    let top_lip_y = pipe.gap_y - pipe.gap / 2.0 - LIP_H;
    let bottom_lip_y = pipe.gap_y + pipe.gap / 2.0;

    for lip_y in [top_lip_y, bottom_lip_y] {
        (api.draw_rectangle)(pipe.x - OVERHANG, lip_y, pipe.width + OVERHANG * 2.0, LIP_H, color::GREEN);
        (api.draw_rectangle_lines)(
            pipe.x - OVERHANG,
            lip_y,
            pipe.width + OVERHANG * 2.0,
            LIP_H,
            4.0,
            color::DARK_GREEN,
        );
    }
}

fn draw_hud(state: &GameState, api: &EngineApi, screen_w: f32, screen_h: f32) {
    // Score, centered near the top.

    match state.state {
        STATE_READY => {
        }
        STATE_GAME_OVER => {
            (api.draw_rectangle)(0.0, 0.0, screen_w, screen_h, color::BLACK_OVERLAY);
            draw_centered_text(api, "GAME OVER", screen_h * 0.32, 54.0, color::RED, screen_w);
            draw_centered_text(
                api,
                &format!("Score: {}", state.score),
                screen_h * 0.44,
                32.0,
                color::WHITE,
                screen_w,
            );
            draw_centered_text(
                api,
                &format!("Best: {}", state.best),
                screen_h * 0.50,
                28.0,
                color::GOLD,
                screen_w,
            );
            draw_centered_text(api, "Press SPACE / click to retry", screen_h * 0.60, 24.0, color::WHITE, screen_w);
        }
        _ => {}
    }
}

fn draw_score(score: u32, api: &EngineApi, screen_w: f32) {
    let mut bytes = score.to_string().into_bytes();
    bytes.push(0);
    let w = (api.measure_text_width)(bytes.as_ptr(), 64.0);
    let x = (screen_w - w) / 2.0;
    (api.draw_text)(bytes.as_ptr(), x + 2.0, 84.0, 64.0, color::BLACK_SHADOW);
    (api.draw_text)(bytes.as_ptr(), x, 82.0, 64.0, color::WHITE);
}

fn draw_centered_text(api: &EngineApi, text: &str, y: f32, size: f32, col: u32, screen_w: f32) {
    let mut bytes = text.as_bytes().to_vec();
    bytes.push(0);
    let w = (api.measure_text_width)(bytes.as_ptr(), size);
    let x = (screen_w - w) / 2.0;
    (api.draw_text)(bytes.as_ptr(), x + 2.0, y + 2.0, size, color::BLACK_TEXT_SHADOW);
    (api.draw_text)(bytes.as_ptr(), x, y, size, col);
}
