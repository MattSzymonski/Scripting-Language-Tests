// game/src/lib.rs — Bouncing ball game logic (ORIGINAL / fallback)
//
// This is the base DLL loaded once at startup. Its functions are the
// fallback implementations. When patch_game.dll is loaded, dll_patch
// redirects calls to the patched versions automatically.
//
// The bloat module inflates this DLL's compile time so the benefit
// of patching only patch_game.dll (fast) vs rebuilding game.dll (slow)
// is clearly visible.
//
// --- SCRIPT ---

// ── Compile-time bloat ────────────────────────────────────────────────────
mod bloat_gen;

// ── Ball State ────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BallState {
    pub position_x: f32,
    pub position_y: f32,
    pub radius: f32,
    pub color_r: f32,
    pub color_g: f32,
    pub color_b: f32,
    pub color_a: f32,
}

// ── Physics State ─────────────────────────────────────────────────────────

#[repr(C)]
pub struct PhysicsState {
    pub delta_time: f32,
    pub position_x: f32,
    pub position_y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub radius: f32,
    pub active: bool,
}

// ── Simulation Constants ──────────────────────────────────────────────────

const GRAVITY: f32 = 800.0;
const BOUNCE_VELOCITY_Y: f32 = -500.0;
const BOUNCE_VELOCITY_X: f32 = 150.0;
const RESTITUTION: f32 = 0.7;
const FLOOR_Y: f32 = 580.0;
const CEILING_Y: f32 = 20.0;
const LEFT_WALL: f32 = 20.0;
const RIGHT_WALL: f32 = 780.0;

// ── Sentinel ──────────────────────────────────────────────────────────────

/// ASLR anchor. Must be present at the same RVA in both game.dll and
/// patch_game.dll for dll_patch's per-module slide computation.
#[unsafe(no_mangle)]
pub extern "C" fn __subsecond_anchor() {}

// ── Exported Functions (fallback implementations) ─────────────────────────

#[unsafe(no_mangle)]
pub extern "C" fn start(state: *mut PhysicsState) {
    let s = unsafe { &mut *state };
    s.position_x = 400.0;
    s.position_y = 500.0;
    s.velocity_x = BOUNCE_VELOCITY_X;
    s.velocity_y = BOUNCE_VELOCITY_Y;
    s.active = true;
}

#[unsafe(no_mangle)]
pub extern "C" fn update(state: *mut PhysicsState) {
    let s = unsafe { &mut *state };
    if !s.active {
        return;
    }
    let clamped_delta = s.delta_time.min(0.1);
    s.velocity_y += GRAVITY * clamped_delta;
    s.position_x += s.velocity_x * clamped_delta;
    s.position_y += s.velocity_y * clamped_delta;

    let bottom = s.position_y + s.radius;
    if bottom >= FLOOR_Y {
        s.position_y = FLOOR_Y - s.radius;
        s.velocity_y = -s.velocity_y.abs() * RESTITUTION;
        if s.velocity_y.abs() < 10.0 {
            s.velocity_y = 0.0;
        }
    }
    let top = s.position_y - s.radius;
    if top <= CEILING_Y {
        s.position_y = CEILING_Y + s.radius;
        s.velocity_y = s.velocity_y.abs() * RESTITUTION;
    }
    let left = s.position_x - s.radius;
    if left <= LEFT_WALL {
        s.position_x = LEFT_WALL + s.radius;
        s.velocity_x = s.velocity_x.abs() * RESTITUTION;
    }
    let right = s.position_x + s.radius;
    if right >= RIGHT_WALL {
        s.position_x = RIGHT_WALL - s.radius;
        s.velocity_x = -s.velocity_x.abs() * RESTITUTION;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_state(state: *const PhysicsState) -> BallState {
    let s = unsafe { &*state };
    let color = if s.active {
        (1.0_f32, 0.3_f32, 0.3_f32, 1.0_f32)
    } else {
        (0.5_f32, 0.5_f32, 0.5_f32, 1.0_f32)
    };
    BallState {
        position_x: s.position_x,
        position_y: s.position_y,
        radius: s.radius,
        color_r: color.0,
        color_g: color.1,
        color_b: color.2,
        color_a: color.3,
    }
}
