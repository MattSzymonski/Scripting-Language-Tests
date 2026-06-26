//! The single gameplay scene, which also handles the ready / game-over states.

use mini_engine::prelude::*;

use crate::bird::Bird;
use crate::config;
use crate::pipe::Pipe;

#[derive(Clone, Copy, PartialEq, Eq)]
enum GameState {
    /// Waiting for the first flap; bird bobs in place.
    Ready,
    Playing,
    GameOver,
}

pub struct FlappyScene {
    state: GameState,
    bird: Bird,
    pipes: Vec<Pipe>,
    spawn_timer: f32,
    score: u32,
    best: u32,
}

impl FlappyScene {
    pub fn new() -> Self {
        Self::with_best(0)
    }

    fn with_best(best: u32) -> Self {
        Self {
            state: GameState::Ready,
            bird: Bird::new(vec2(config::BIRD_X, config::WINDOW_HEIGHT as f32 * 0.45)),
            pipes: Vec::new(),
            spawn_timer: 0.0,
            score: 0,
            best,
        }
    }

    fn floor_y(&self, ctx: &EngineContext) -> f32 {
        ctx.screen_height() - config::GROUND_HEIGHT
    }

    fn spawn_pipe(&mut self, ctx: &EngineContext) {
        let min_y = config::GAP_MARGIN;
        let max_y = self.floor_y(ctx) - config::GAP_MARGIN;
        let gap_y = rand::gen_range(min_y, max_y);
        self.pipes.push(Pipe::new(ctx.screen_width(), gap_y));
    }

    fn update_playing(&mut self, ctx: &mut EngineContext) {
        self.bird.update(ctx);

        // Spawn pipes on a timer.
        self.spawn_timer += ctx.dt;
        if self.spawn_timer >= config::SPAWN_INTERVAL {
            self.spawn_timer -= config::SPAWN_INTERVAL;
            self.spawn_pipe(ctx);
        }

        // Scroll and reap pipes.
        for pipe in &mut self.pipes {
            pipe.update(ctx);
        }
        self.pipes.retain(|p| p.is_alive());

        // Scoring + collision.
        let screen_h = ctx.screen_height();
        let bird_box = self.bird.bounds().expect("bird has bounds");
        for pipe in &mut self.pipes {
            if !pipe.scored && pipe.x + pipe.width < self.bird.pos.x {
                pipe.scored = true;
                self.score += 1;
            }
            if pipe.solids(screen_h).iter().any(|s| s.overlaps(&bird_box)) {
                self.bird.dead = true;
            }
        }

        if self.bird.dead {
            self.best = self.best.max(self.score);
            self.state = GameState::GameOver;
        }
    }
}

impl Default for FlappyScene {
    fn default() -> Self {
        Self::new()
    }
}

impl Scene for FlappyScene {
    fn on_enter(&mut self, _ctx: &mut EngineContext) {
        // Vary the pipe pattern between runs.
        rand::srand((get_time() * 1_000_000.0) as u64 ^ 0x9E37_79B9);
    }

    fn update(&mut self, ctx: &mut EngineContext) {
        if ctx.key_pressed(KeyCode::Escape) {
            ctx.quit();
            return;
        }

        let flapped = ctx.key_pressed(KeyCode::Space)
            || ctx.key_pressed(KeyCode::Up)
            || ctx.mouse_pressed(MouseButton::Left);

        match self.state {
            GameState::Ready => {
                // Gentle hover.
                let base = config::WINDOW_HEIGHT as f32 * 0.45;
                self.bird.pos.y = base + (ctx.time * 3.0).sin() * 10.0;
                if flapped {
                    self.state = GameState::Playing;
                    self.bird.flap();
                }
            }
            GameState::Playing => self.update_playing(ctx),
            GameState::GameOver => {
                // Let the bird settle onto the ground.
                if self.bird.pos.y + self.bird.radius < self.floor_y(ctx) {
                    self.bird.vel += config::GRAVITY * ctx.dt;
                    self.bird.pos.y += self.bird.vel * ctx.dt;
                }
                if flapped {
                    ctx.change_scene(FlappyScene::with_best(self.best));
                }
            }
        }
    }

    fn draw(&self, ctx: &EngineContext) {
        draw_background(ctx);

        for pipe in &self.pipes {
            pipe.draw(ctx);
        }

        draw_ground(ctx);
        self.bird.draw(ctx);

        draw_hud(self, ctx);
    }
}

// --- presentation ----------------------------------------------------------

fn draw_background(ctx: &EngineContext) {
    // Sky gradient approximated with two bands.
    let w = ctx.screen_width();
    let h = ctx.screen_height();
    clear_background(Color::new(0.45, 0.75, 0.95, 1.0));
    draw_rectangle(0.0, 0.0, w, h * 0.55, Color::new(0.40, 0.70, 0.92, 1.0));

    // A couple of lazy clouds.
    for (cx, cy, r) in [(90.0, 120.0, 26.0), (340.0, 80.0, 20.0), (250.0, 200.0, 18.0)] {
        let cloud = Color::new(1.0, 1.0, 1.0, 0.85);
        draw_circle(cx, cy, r, cloud);
        draw_circle(cx + r, cy + 4.0, r * 0.8, cloud);
        draw_circle(cx - r, cy + 6.0, r * 0.7, cloud);
    }
}

fn draw_ground(ctx: &EngineContext) {
    let w = ctx.screen_width();
    let h = ctx.screen_height();
    let top = h - config::GROUND_HEIGHT;
    draw_rectangle(0.0, top, w, config::GROUND_HEIGHT, Color::new(0.86, 0.74, 0.40, 1.0));
    draw_rectangle(0.0, top, w, 14.0, Color::new(0.50, 0.78, 0.36, 1.0));
    draw_line(0.0, top, w, top, 4.0, Color::new(0.36, 0.62, 0.26, 1.0));
}

fn draw_hud(scene: &FlappyScene, ctx: &EngineContext) {
    let w = ctx.screen_width();

    // Score, centred near the top.
    let score = scene.score.to_string();
    let dims = measure_text(&score, None, 64, 1.0);
    let sx = (w - dims.width) / 2.0;
    draw_text(&score, sx + 2.0, 84.0, 64.0, Color::new(0.0, 0.0, 0.0, 0.4));
    draw_text(&score, sx, 82.0, 64.0, WHITE);

    match scene.state {
        GameState::Ready => {
            center_text(ctx, "FLAPPY BIRD", 0.30, 56.0, GOLD);
            center_text(ctx, "Press SPACE / click to flap", 0.42, 26.0, WHITE);
            center_text(ctx, "ESC to quit", 0.47, 22.0, WHITE);
        }
        GameState::Playing => {}
        GameState::GameOver => {
            // Dim overlay.
            draw_rectangle(0.0, 0.0, w, ctx.screen_height(), Color::new(0.0, 0.0, 0.0, 0.35));
            center_text(ctx, "GAME OVER", 0.32, 54.0, RED);
            center_text(ctx, &format!("Score: {}", scene.score), 0.44, 32.0, WHITE);
            center_text(ctx, &format!("Best: {}", scene.best), 0.50, 28.0, GOLD);
            center_text(ctx, "Press SPACE / click to retry", 0.60, 24.0, WHITE);
        }
    }
}

/// Draw horizontally-centred text at a fractional height of the screen.
fn center_text(ctx: &EngineContext, text: &str, y_frac: f32, size: f32, color: Color) {
    let dims = measure_text(text, None, size as u16, 1.0);
    let x = (ctx.screen_width() - dims.width) / 2.0;
    let y = ctx.screen_height() * y_frac;
    draw_text(text, x + 2.0, y + 2.0, size, Color::new(0.0, 0.0, 0.0, 0.45));
    draw_text(text, x, y, size, color);
}
