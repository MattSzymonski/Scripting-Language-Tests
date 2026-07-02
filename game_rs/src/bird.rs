//! The player-controlled bird — mirror of `Flappy.Bird` in C#.

use macroquad::prelude::*;

use crate::config;
use crate::primitives::{PackedColor, Rect, Vec2};

/// The player-controlled bird: a circle pulled down by gravity that flaps
/// upward on input.
pub struct Bird {
    pub pos: Vec2,
    pub vel: f32,
    pub radius: f32,
    pub dead: bool,
    rotation: f32,
}

impl Bird {
    pub fn new(start: Vec2) -> Self {
        Self {
            pos: start,
            vel: 0.0,
            radius: config::BIRD_RADIUS,
            dead: false,
            rotation: 0.0,
        }
    }

    pub fn flap(&mut self) {
        self.vel = config::FLAP_VELOCITY;
    }

    /// Advance the bird by `dt` seconds. Input is checked inside so the caller
    /// doesn't need to worry about it.
    pub fn update(&mut self, dt: f32, screen_h: f32, flap_input: bool) {
        if flap_input {
            self.flap();
        }

        // Integrate gravity.
        self.vel += config::GRAVITY * dt;
        self.pos = Vec2::new(self.pos.x, self.pos.y + self.vel * dt);

        // Ceiling clamp.
        if self.pos.y < self.radius {
            self.pos = Vec2::new(self.pos.x, self.radius);
            self.vel = 0.0;
        }

        // Ground is fatal.
        let floor = screen_h - config::GROUND_HEIGHT;
        if self.pos.y + self.radius >= floor {
            self.pos = Vec2::new(self.pos.x, floor - self.radius);
            self.dead = true;
        }

        // Tilt toward the direction of travel.
        let target = (self.vel / 600.0).clamp(-0.5, 1.4);
        self.rotation += (target - self.rotation) * (10.0 * dt).min(1.0);
    }

    pub fn draw(&self) {
        let pos_x = self.pos.x;
        let pos_y = self.pos.y;
        let r = self.radius;

        // Body.
        draw_circle(pos_x, pos_y, r, PackedColor::new(255, 205, 40, 255).to_mq()); // Gold
        draw_circle_lines(
            pos_x,
            pos_y,
            r,
            2.0,
            PackedColor::new(235, 140, 30, 255).to_mq(),
        );
        // Orange

        // Beak: a triangle pointing the way the bird is tilted.
        let dir = Vec2::new(self.rotation.cos(), self.rotation.sin());
        let tip = self.pos + dir * (r + 8.0);
        let perp = dir.perp() * (r * 0.4);
        draw_triangle(
            (self.pos + dir * r + perp).into_mq(),
            (self.pos + dir * r - perp).into_mq(),
            tip.into_mq(),
            PackedColor::new(235, 140, 30, 255).to_mq(), // Orange
        );

        // Eye.
        draw_circle(pos_x + r * 0.35, pos_y - r * 0.35, r * 0.18, WHITE);
        draw_circle(pos_x + r * 0.42, pos_y - r * 0.35, r * 0.09, BLACK);
    }

    /// Collision box, slightly shrunk for fairness.
    pub fn bounds(&self) -> Rect {
        let r = self.radius * 0.85;
        Rect::new(self.pos.x - r, self.pos.y - r, r * 2.0, r * 2.0)
    }
}

// Little helper to convert our Vec2 into macroquad's Vec2 for draw_triangle.
trait IntoMqVec2 {
    fn into_mq(self) -> macroquad::prelude::Vec2;
}

impl IntoMqVec2 for Vec2 {
    fn into_mq(self) -> macroquad::prelude::Vec2 {
        macroquad::prelude::vec2(self.x, self.y)
    }
}
