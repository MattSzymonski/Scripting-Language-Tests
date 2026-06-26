//! An obstacle: a pair of pipes with a gap the bird must fly through.

use mini_engine::prelude::*;

use crate::config;

/// A vertical pipe pair scrolling left across the screen.
pub struct Pipe {
    /// Left edge x of the pipe column.
    pub x: f32,
    /// Y centre of the gap.
    pub gap_y: f32,
    pub width: f32,
    pub gap: f32,
    /// Whether the bird has already passed (and scored on) this pipe.
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
    pub fn solids(&self, screen_h: f32) -> [Rect; 2] {
        let top_h = self.gap_y - self.gap / 2.0;
        let bottom_y = self.gap_y + self.gap / 2.0;
        let bottom_h = screen_h - config::GROUND_HEIGHT - bottom_y;
        [
            Rect::new(self.x, 0.0, self.width, top_h),
            Rect::new(self.x, bottom_y, self.width, bottom_h),
        ]
    }

    /// `true` once the pipe has fully scrolled off the left edge.
    pub fn off_screen(&self) -> bool {
        self.x + self.width < 0.0
    }
}

impl GameObject for Pipe {
    fn update(&mut self, ctx: &mut EngineContext) {
        self.x -= config::SCROLL_SPEED * ctx.dt;
    }

    fn is_alive(&self) -> bool {
        !self.off_screen()
    }

    fn draw(&self, ctx: &EngineContext) {
        for solid in self.solids(ctx.screen_height()) {
            draw_rectangle(solid.x, solid.y, solid.w, solid.h, GREEN);
            draw_rectangle_lines(solid.x, solid.y, solid.w, solid.h, 4.0, DARKGREEN);
        }

        // A wider "lip" at each end of the gap, the classic pipe look.
        let lip_h = 26.0;
        let lip_overhang = 6.0;
        let top_lip_y = self.gap_y - self.gap / 2.0 - lip_h;
        let bottom_lip_y = self.gap_y + self.gap / 2.0;
        for lip_y in [top_lip_y, bottom_lip_y] {
            draw_rectangle(
                self.x - lip_overhang,
                lip_y,
                self.width + lip_overhang * 2.0,
                lip_h,
                GREEN,
            );
            draw_rectangle_lines(
                self.x - lip_overhang,
                lip_y,
                self.width + lip_overhang * 2.0,
                lip_h,
                4.0,
                DARKGREEN,
            );
        }
    }
}
