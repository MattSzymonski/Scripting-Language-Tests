//! 2D primitives: Vec2, Rect, and packed Color — mirror of `Flappy.Primitives` in C#.

use macroquad::prelude::*;

// ---------------------------------------------------------------------------
// Vec2
// ---------------------------------------------------------------------------

/// A small 2D vector, enough for the game's needs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Perpendicular (rotated 90°), used to build the bird's beak.
    pub fn perp(self) -> Self {
        Self::new(-self.y, self.x)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s)
    }
}

// ---------------------------------------------------------------------------
// Rect
// ---------------------------------------------------------------------------

/// An axis-aligned rectangle used for collision tests.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn overlaps(&self, o: &Rect) -> bool {
        self.x < o.x + o.w && self.x + self.w > o.x && self.y < o.y + o.h && self.y + self.h > o.y
    }
}

// ---------------------------------------------------------------------------
// Color (packed 0xRRGGBBAA)
// ---------------------------------------------------------------------------

/// A packed `0xRRGGBBAA` colour matching the C# and FFI unpacker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackedColor {
    pub rgba: u32,
}

impl PackedColor {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            rgba: ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32),
        }
    }

    /// Convert to macroquad [`Color`].
    pub fn to_mq(self) -> Color {
        Color::from_rgba(
            ((self.rgba >> 24) & 0xff) as u8,
            ((self.rgba >> 16) & 0xff) as u8,
            ((self.rgba >> 8) & 0xff) as u8,
            (self.rgba & 0xff) as u8,
        )
    }

    pub fn with_alpha(self, a: u8) -> Self {
        Self::new(
            ((self.rgba >> 24) & 0xff) as u8,
            ((self.rgba >> 16) & 0xff) as u8,
            ((self.rgba >> 8) & 0xff) as u8,
            a,
        )
    }
}
