//! Vertical pipe pairs — mirror of `Flappy.Pipe` in C#.

use macroquad::prelude::*;

use crate::config;
use crate::primitives::{PackedColor, Rect};

/// A vertical pipe pair scrolling left, with a gap to fly through.
pub struct Pipe {
    pub x: f32,
    pub gap_y: f32,
    pub width: f32,
    pub gap: f32,
    pub scored: bool,
}

impl Pipe {
    pub fn new(x: f32, gap_y: f32) -> Self {
        Self {
            x,
            gap_y,
            width: config::PIPE_WIDTH,
            gap: config::PIPE_GAP,
            scored: false,
        }
    }

    /// The two solid rectangles (top and bottom) used for collision.
    pub fn solids(&self, screen_h: f32) -> (Rect, Rect) {
        let top_h = self.gap_y - self.gap / 2.0;
        let bottom_y = self.gap_y + self.gap / 2.0;
        let bottom_h = screen_h - config::GROUND_HEIGHT - bottom_y;
        (
            Rect::new(self.x, 0.0, self.width, top_h),
            Rect::new(self.x, bottom_y, self.width, bottom_h),
        )
    }

    pub fn is_off_screen(&self) -> bool {
        self.x + self.width < 0.0
    }

    pub fn update(&mut self, dt: f32) {
        self.x -= config::SCROLL_SPEED * dt;
    }

    pub fn draw(&self, screen_h: f32) {
        let (top, bottom) = self.solids(screen_h);
        let green = PackedColor::new(70, 190, 70, 255).to_mq();
        let dark_green = PackedColor::new(40, 130, 40, 255).to_mq();

        for solid in &[top, bottom] {
            draw_rectangle(solid.x, solid.y, solid.w, solid.h, green);
            draw_rectangle_lines(solid.x, solid.y, solid.w, solid.h, 4.0, dark_green);
        }

        // Classic wider "lip" at each side of the gap.
        const LIP_H: f32 = 26.0;
        const OVERHANG: f32 = 6.0;
        let top_lip_y = self.gap_y - self.gap / 2.0 - LIP_H;
        let bottom_lip_y = self.gap_y + self.gap / 2.0;

        for &lip_y in &[top_lip_y, bottom_lip_y] {
            draw_rectangle(
                self.x - OVERHANG,
                lip_y,
                self.width + OVERHANG * 2.0,
                LIP_H,
                green,
            );
            draw_rectangle_lines(
                self.x - OVERHANG,
                lip_y,
                self.width + OVERHANG * 2.0,
                LIP_H,
                4.0,
                dark_green,
            );
        }
    }
}
