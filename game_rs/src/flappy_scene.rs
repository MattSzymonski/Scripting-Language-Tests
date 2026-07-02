//! The whole game scene — mirror of `Flappy.FlappyScene` in C#.

use macroquad::prelude::*;

use mini_engine::{EngineContext, Scene};

use crate::bird::Bird;
use crate::config;
use crate::pipe::Pipe;
use crate::primitives::{PackedColor, Vec2};

// ---------------------------------------------------------------------------
// Colour palette (kept as PackedColor for consistency with the C# port)
// ---------------------------------------------------------------------------

const COLOR_SKY: PackedColor = PackedColor::new(115, 190, 240, 255);
const COLOR_SKY_DEEP: PackedColor = PackedColor::new(100, 175, 235, 255);
const COLOR_WHITE: PackedColor = PackedColor::new(255, 255, 255, 255);
const COLOR_BLACK: PackedColor = PackedColor::new(0, 0, 0, 255);
const COLOR_GOLD: PackedColor = PackedColor::new(255, 205, 40, 255);
const COLOR_RED: PackedColor = PackedColor::new(225, 60, 55, 255);
const COLOR_DARK_GREEN: PackedColor = PackedColor::new(40, 130, 40, 255);
const COLOR_GROUND: PackedColor = PackedColor::new(220, 190, 105, 255);
const COLOR_GROUND_GRASS: PackedColor = PackedColor::new(125, 200, 90, 255);

// ---------------------------------------------------------------------------
// State machine
// ---------------------------------------------------------------------------

enum State {
    Ready,
    Playing,
    GameOver,
}

// ---------------------------------------------------------------------------
// The scene
// ---------------------------------------------------------------------------

/// The whole game: a single scene that runs the Ready → Playing → GameOver
/// state machine, spawns pipes, and handles scoring and collisions.
pub struct FlappyScene {
    state: State,
    bird: Bird,
    pipes: Vec<Pipe>,
    spawn_timer: f32,
    score: u32,
    best: u32,
}

impl FlappyScene {
    pub fn new() -> Self {
        let screen_h = screen_height();
        Self {
            state: State::Ready,
            bird: Bird::new(Vec2::new(config::BIRD_X, screen_h * 0.45)),
            pipes: Vec::new(),
            spawn_timer: 0.0,
            score: 0,
            best: 0,
        }
    }

    fn floor_y(&self, screen_h: f32) -> f32 {
        screen_h - config::GROUND_HEIGHT
    }

    fn reset(&mut self, screen_h: f32) {
        self.state = State::Ready;
        self.bird = Bird::new(Vec2::new(config::BIRD_X, screen_h * 0.45));
        self.pipes.clear();
        self.spawn_timer = 0.0;
        self.score = 0;
    }

    fn spawn_pipe(&mut self, screen_w: f32, screen_h: f32) {
        let gap_y = macroquad::rand::gen_range(config::GAP_MARGIN, self.floor_y(screen_h) - config::GAP_MARGIN);
        self.pipes.push(Pipe::new(screen_w, gap_y));
    }

    fn update_playing(&mut self, dt: f32, screen_w: f32, screen_h: f32) {
        // Spawn pipes on a timer.
        self.spawn_timer += dt;
        if self.spawn_timer >= config::SPAWN_INTERVAL {
            self.spawn_timer -= config::SPAWN_INTERVAL;
            self.spawn_pipe(screen_w, screen_h);
        }

        // Scroll and reap.
        for pipe in &mut self.pipes {
            pipe.update(dt);
        }
        self.pipes.retain(|p| !p.is_off_screen());

        // Scoring + collision.
        let bird_box = self.bird.bounds();
        for pipe in &mut self.pipes {
            if !pipe.scored && pipe.x + pipe.width < self.bird.pos.x {
                pipe.scored = true;
                self.score += 1;
            }

            let (top, bottom) = pipe.solids(screen_h);
            if top.overlaps(&bird_box) || bottom.overlaps(&bird_box) {
                self.bird.dead = true;
            }
        }

        if self.bird.dead {
            self.best = self.best.max(self.score);
            self.state = State::GameOver;
        }
    }

    // --- presentation helpers ----------------------------------------------

    fn draw_background(&self, screen_w: f32, screen_h: f32) {
        clear_background(COLOR_SKY.to_mq());
        draw_rectangle(0.0, 0.0, screen_w, screen_h * 0.55, COLOR_SKY_DEEP.to_mq());

        let clouds = [(90.0, 120.0, 26.0), (340.0, 80.0, 20.0), (250.0, 200.0, 18.0)];
        let cloud_color = COLOR_WHITE.with_alpha(217).to_mq();
        for (cx, cy, r) in clouds {
            draw_circle(cx, cy, r, cloud_color);
            draw_circle(cx + r, cy + 4.0, r * 0.8, cloud_color);
            draw_circle(cx - r, cy + 6.0, r * 0.7, cloud_color);
        }
    }

    fn draw_ground(&self, screen_w: f32, screen_h: f32) {
        let top = screen_h - config::GROUND_HEIGHT;
        draw_rectangle(0.0, top, screen_w, config::GROUND_HEIGHT, COLOR_GROUND.to_mq());
        draw_rectangle(0.0, top, screen_w, 14.0, COLOR_GROUND_GRASS.to_mq());
        draw_line(0.0, top, screen_w, top, 4.0, COLOR_DARK_GREEN.to_mq());
    }

    fn draw_hud(&self, screen_w: f32, screen_h: f32) {
        // Score, centred near the top.
        let score_text = self.score.to_string();
        let sw = measure_text(&score_text, None, 64, 1.0).width;
        let sx = (screen_w - sw) / 2.0;
        draw_text(&score_text, sx + 2.0, 84.0, 64.0, COLOR_BLACK.with_alpha(100).to_mq());
        draw_text(&score_text, sx, 82.0, 64.0, COLOR_WHITE.to_mq());

        match self.state {
            State::Ready => {
                self.center_text("FLAPPY BIRD", 0.30, 56.0, COLOR_GOLD, screen_w, screen_h);
                self.center_text("Press SPACE / click to flap", 0.42, 26.0, COLOR_WHITE, screen_w, screen_h);
                self.center_text("ESC to quit", 0.47, 22.0, COLOR_WHITE, screen_w, screen_h);
            }
            State::GameOver => {
                draw_rectangle(0.0, 0.0, screen_w, screen_h, COLOR_BLACK.with_alpha(90).to_mq());
                self.center_text("GAME OVER", 0.32, 54.0, COLOR_RED, screen_w, screen_h);
                self.center_text(&format!("Score: {}", self.score), 0.44, 32.0, COLOR_WHITE, screen_w, screen_h);
                self.center_text(&format!("Best: {}", self.best), 0.50, 28.0, COLOR_GOLD, screen_w, screen_h);
                self.center_text("Press SPACE / click to retry", 0.60, 24.0, COLOR_WHITE, screen_w, screen_h);
            }
            State::Playing => { /* no overlay */ }
        }
    }

    fn center_text(&self, text: &str, y_frac: f32, size: f32, color: PackedColor, screen_w: f32, screen_h: f32) {
        let x = (screen_w - measure_text(text, None, size as u16, 1.0).width) / 2.0;
        let y = screen_h * y_frac;
        draw_text(text, x + 2.0, y + 2.0, size, COLOR_BLACK.with_alpha(115).to_mq());
        draw_text(text, x, y, size, color.to_mq());
    }
}

impl Scene for FlappyScene {
    fn update(&mut self, ctx: &mut EngineContext) {
        let screen_w = ctx.screen_width();
        let screen_h = ctx.screen_height();

        // Global quit key.
        if ctx.key_pressed(KeyCode::Escape) {
            ctx.quit();
            return;
        }

        let flap_input = ctx.key_pressed(KeyCode::Space)
            || ctx.key_pressed(KeyCode::Up)
            || ctx.mouse_pressed(MouseButton::Left);

        match self.state {
            State::Ready => {
                // Gentle hover until the first flap.
                let base_y = screen_h * 0.45;
                self.bird.pos = Vec2::new(self.bird.pos.x, base_y + (ctx.time as f32 * 3.0).sin() * 10.0);
                if flap_input {
                    self.state = State::Playing;
                    self.bird.flap();
                }
            }
            State::Playing => {
                self.bird.update(ctx.dt, screen_h, flap_input);
                self.update_playing(ctx.dt, screen_w, screen_h);
            }
            State::GameOver => {
                // Let the bird drop onto the ground.
                if self.bird.pos.y + self.bird.radius < self.floor_y(screen_h) {
                    self.bird.vel += config::GRAVITY * ctx.dt;
                    self.bird.pos = Vec2::new(
                        self.bird.pos.x,
                        self.bird.pos.y + self.bird.vel * ctx.dt,
                    );
                }
                if flap_input {
                    self.reset(screen_h);
                }
            }
        }
    }

    fn draw(&self, _ctx: &EngineContext) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        self.draw_background(screen_w, screen_h);

        for pipe in &self.pipes {
            pipe.draw(screen_h);
        }

        self.draw_ground(screen_w, screen_h);
        self.bird.draw();
        self.draw_hud(screen_w, screen_h);
    }
}
