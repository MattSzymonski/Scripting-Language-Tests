//! The player-controlled bird.

use mini_engine::prelude::*;

use crate::config;

/// The flappy bird: a circle affected by gravity that flaps upward on input.
pub struct Bird {
    pub pos: Vec2,
    pub vel: f32,
    pub radius: f32,
    /// Set once the bird hits something; the scene reads this to end the round.
    pub dead: bool,
    /// Visual tilt in radians, derived from vertical velocity.
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

    /// Apply an upward impulse.
    pub fn flap(&mut self) {
        self.vel = config::FLAP_VELOCITY;
    }
}

impl GameObject for Bird {
    fn update(&mut self, ctx: &mut EngineContext) {
        // Flap on space / up arrow / left click.
        if ctx.key_pressed(KeyCode::Space)
            || ctx.key_pressed(KeyCode::Up)
            || ctx.mouse_pressed(MouseButton::Left)
        {
            self.flap();
        }

        // Integrate gravity.
        self.vel += config::GRAVITY * ctx.dt;
        self.pos.y += self.vel * ctx.dt;

        // Ceiling stops the bird; never let it leave the top of the screen.
        if self.pos.y < self.radius {
            self.pos.y = self.radius;
            self.vel = 0.0;
        }

        // Hitting the ground is fatal.
        let floor = ctx.screen_height() - config::GROUND_HEIGHT;
        if self.pos.y + self.radius >= floor {
            self.pos.y = floor - self.radius;
            self.dead = true;
        }

        // Tilt up when rising, dive when falling, for a bit of character.
        let target = (self.vel / 600.0).clamp(-0.5, 1.4);
        self.rotation += (target - self.rotation) * (10.0 * ctx.dt).min(1.0);
    }

    fn draw(&self, _ctx: &EngineContext) {
        // Body.
        draw_circle(self.pos.x, self.pos.y, self.radius, GOLD);
        draw_circle_lines(self.pos.x, self.pos.y, self.radius, 2.0, ORANGE);

        // Beak: a small triangle that points the way the bird is tilted.
        let dir = vec2(self.rotation.cos(), self.rotation.sin());
        let tip = self.pos + dir * (self.radius + 8.0);
        let perp = vec2(-dir.y, dir.x) * (self.radius * 0.4);
        draw_triangle(
            self.pos + dir * self.radius + perp,
            self.pos + dir * self.radius - perp,
            tip,
            ORANGE,
        );

        // Eye.
        draw_circle(
            self.pos.x + self.radius * 0.35,
            self.pos.y - self.radius * 0.35,
            self.radius * 0.18,
            WHITE,
        );
        draw_circle(
            self.pos.x + self.radius * 0.42,
            self.pos.y - self.radius * 0.35,
            self.radius * 0.09,
            BLACK,
        );
    }

    fn bounds(&self) -> Option<Rect> {
        // Slightly forgiving square hitbox around the circle.
        let r = self.radius * 0.85;
        Some(Rect::new(
            self.pos.x - r,
            self.pos.y - r,
            r * 2.0,
            r * 2.0,
        ))
    }
}
