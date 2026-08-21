// REQUIREMENTS
//   Rust (stable).  Compiled as a cdylib by the standalone host.  Must NOT
//   link macroquad (a second macroquad state would be disconnected from the
//   host's window) - it talks to the engine only through the ProjectApi table.
//
// DESCRIPTION
//   Hot-reloadable game logic ("project") for the mini engine.  Exports
//   project_set_api (receives the engine's function-pointer table),
//   project_init (applies the quad settings on every load/reload),
//   project_update (called by the engine every frame) and project_resolve_symbol
//   (the single address-lookup the host uses to find every hot function -
//   the module graph itself is plain `pub fn`, not exported).  The quad
//   settings (count, colours, sizes, speeds, jump behaviour) are defined
//   HERE, in the project - not in the engine.
//
// USAGE
//   Built automatically by `cargo run -p standalone` from the workspace root.
//   Edit this file while the game runs and save to live-reload.
//
// EXAMPLE USAGE
//   cargo run -p standalone        # from live_coding_mini_engine/
//
// --- SCRIPT ---

use std::ffi::c_void;

// Auto-generated bloat (dead code, compile volume only) and the
// interconnected module graph (reachable from project_update below).
mod bloat_gen_no_export;
mod modules;

// Auto-generated hot-symbol registry (build.rs writes it to OUT_DIR): maps
// every hot function's name to its address, so no function/module ever has
// to be declared by hand.  `pub(crate)` so it is not itself treated as hot.
include!(concat!(env!("OUT_DIR"), "/hot_registry.rs"));

/// Mirror of `mini_engine::ProjectApi` - must match field-for-field, in order.
#[repr(C)]
struct ProjectApi {
    get_state: extern "C" fn() -> *mut c_void,
    screen_width: extern "C" fn() -> f32,
    screen_height: extern "C" fn() -> f32,
}

/// Mirror of `mini_engine::Quad` - must match field-for-field, in order.
#[repr(C)]
struct Quad {
    x: f32,
    y: f32,
    base_y: f32,
    w: f32,
    h: f32,
    vx: f32,
    jump_phase: f32,
    jump_speed: f32,
    jump_height: f32,
    color: u32,
}

/// Mirror of `mini_engine::GameState` - must match field-for-field, in order.
#[repr(C)]
struct GameState {
    tick: f32,
    quad_count: usize,
    quads: [Quad; MAX_QUADS],
}

/// Must match `mini_engine::MAX_QUADS`.
const MAX_QUADS: usize = 8;

// ---------------------------------------------------------------------------
// Quad settings - edit any of these and save to live-reload; project_init
// re-applies them on every load.
// ---------------------------------------------------------------------------

/// Width and height of every spawned quad, in pixels.
const QUAD_SIZE: f32 = 46.0;
/// One colour (0xRRGGBBAA) per quad, in spawn order.
const QUAD_COLOURS: [u32; MAX_QUADS] = [
    0xFF_FF6B6B,
    0xFF_FFD93D,
    0xFF_6BCB77,
    0xFF_4D96FF,
    0xFF_9B5DE5,
    0xFF_F15BB5,
    0xFF_FEE440,
    0xFF_00BBF9,
];
/// Rest height of the first quad, as a fraction of the screen height.
const QUAD_BASE_Y_MIN_FRACTION: f32 = 0.25;
/// Vertical span covered by the rest heights, as a fraction of screen height.
const QUAD_BASE_Y_SPAN_FRACTION: f32 = 0.6;
/// Initial horizontal drift speed of the first quad (px/s).
const QUAD_INITIAL_VELOCITY_X: f32 = 70.0;
/// Extra horizontal drift speed added per quad index (px/s).
const QUAD_VELOCITY_X_STEP: f32 = 23.0;
/// Jump oscillation speed of the first quad (rad/s).
const QUAD_INITIAL_JUMP_SPEED: f32 = 1.6;
/// Extra jump oscillation speed added per quad index (rad/s).
const QUAD_JUMP_SPEED_STEP: f32 = 1.35;
/// Jump height of the first quad (px).
const QUAD_INITIAL_JUMP_HEIGHT: f32 = 40.0;
/// Extra jump height added per quad index (px).
const QUAD_JUMP_HEIGHT_STEP: f32 = 14.0;
/// Half-size factor used to centre a quad on its x/y position.
const QUAD_HALF_SIZE_FACTOR: f32 = 0.5;

/// The engine's API table, stored when the host calls `project_set_api`.
static mut API: *const ProjectApi = std::ptr::null();

/// Called by the host right after every load/reload with the engine's API
/// table.  The engine owns the state; we only get a pointer to it.
#[unsafe(no_mangle)]
#[allow(private_interfaces)]
pub extern "C" fn project_set_api(api: *const ProjectApi) {
    unsafe {
        API = api;
    }
}

/// THE single exported entry point the host uses to resolve any hot
/// function's address by name (instead of exporting every graph function).
/// Returns the address, or 0 when the name is unknown.  The lookup itself
/// lives in the build.rs-generated `resolve_hot_symbol` registry.
#[unsafe(no_mangle)]
#[allow(private_interfaces)]
pub extern "C" fn project_resolve_symbol(name: *const std::os::raw::c_char) -> usize {
    if name.is_null() {
        return 0;
    }
    let name = unsafe { std::ffi::CStr::from_ptr(name) };
    let Ok(name) = name.to_str() else {
        return 0;
    };
    resolve_hot_symbol(name)
}

/// Borrow the engine-owned game state through the API table.
fn state() -> &'static mut GameState {
    unsafe {
        let api = API;
        // Contract: the host calls project_set_api before project_update is
        // ever invoked (it does - during load, before patching the table).
        assert!(
            !api.is_null(),
            "project_set_api must be called before update"
        );
        let state_pointer = ((*api).get_state)() as *mut GameState;
        &mut *state_pointer
    }
}

/// The host calls this after every load/reload to (re)apply the quad settings
/// defined below.  Because it runs on every reload, editing the settings and
/// saving re-applies them live (at the cost of resetting the quads' initial
/// placement).
#[unsafe(no_mangle)]
pub extern "C" fn project_init() {
    let game_state = state();
    let screen_width = unsafe { ((*API).screen_width)() };
    let screen_height = unsafe { ((*API).screen_height)() };
    spawn_default_quads(game_state, screen_width, screen_height);
}

/// Spawn the default spread of quads, applying the settings above.
fn spawn_default_quads(game_state: &mut GameState, screen_width: f32, screen_height: f32) {
    if screen_width <= 0.0 || screen_height <= 0.0 {
        return;
    }

    let count = QUAD_COLOURS.len();
    for (index, quad) in game_state.quads.iter_mut().enumerate().take(count) {
        // Horizontal position: spread evenly across the window width.
        let fraction = (index as f32 + 1.0) / (count as f32 + 1.0);
        quad.w = QUAD_SIZE;
        quad.h = QUAD_SIZE;
        quad.x = screen_width * fraction;
        // Rest height: a vertical band between the min and min+span.
        quad.base_y = screen_height
            * (QUAD_BASE_Y_MIN_FRACTION
                + QUAD_BASE_Y_SPAN_FRACTION * (index as f32 / count as f32));
        quad.y = quad.base_y;
        quad.vx = QUAD_INITIAL_VELOCITY_X + index as f32 * QUAD_VELOCITY_X_STEP;
        quad.jump_speed = QUAD_INITIAL_JUMP_SPEED + index as f32 * QUAD_JUMP_SPEED_STEP;
        quad.jump_height = QUAD_INITIAL_JUMP_HEIGHT + index as f32 * QUAD_JUMP_HEIGHT_STEP * 4.0;
        quad.color = QUAD_COLOURS[index];
    }
    game_state.quad_count = count;
}

// ---------------------------------------------------------------------------
// Functional-verification hook (opt-in, test tooling only).  When the host
// process has LIVE_CODING_VERIFY=1 set, project_update prints a deterministic
// checksum derived from the HOT functions' outputs with FIXED inputs.  The
// test suite compares this checksum before/after a patch to PROVE the patched
// code actually runs - the value changes iff behaviour changes.  Both
// functions are private lib helpers, so build.rs / the host must keep them on
// the infrastructure blocklist (they are not hot).
// ---------------------------------------------------------------------------

/// Deterministic fold of the hot functions' outputs (fixed inputs).
/// std::hint::black_box stops the compiler constant-folding the calls into
/// literals, which would freeze the checksum and hide live patches.  Only
/// references hot functions (which the patch module provides as wrappers) and
/// the checksum helper below, so it is self-contained when injected.
fn verification_checksum() -> u32 {
    let mut checksum: u32 = 0x811C_9DC5;
    let mut fold = |value: i32| {
        checksum = checksum.wrapping_mul(0x0100_0193);
        let bits = value as u32;
        checksum ^= bits & 0xFF;
        checksum ^= (bits >> 8) & 0xFF;
        checksum ^= (bits >> 16) & 0xFF;
        checksum ^= (bits >> 24) & 0xFF;
    };
    fold(std::hint::black_box(get_aaa()));
    fold(std::hint::black_box(scale_value_0032(7, 2)));
    fold(std::hint::black_box(modules::run_update(12345)));
    fold(std::hint::black_box(modules::run_compute(6789, 3)));
    fold(std::hint::black_box(modules::module_000::tick_000(13)));
    fold(std::hint::black_box(modules::module_000::compute_000(5, 7)));
    fold(std::hint::black_box(modules::module_000::sample_000(11)));
    fold(std::hint::black_box(modules::module_000::scale_value_000(
        17, 3,
    )));
    fold(std::hint::black_box(modules::module_000::offset_value_000(
        23, 5,
    )));
    checksum
}

/// Print the checksum only when it changes (plus once on first call), so the
/// log stays quiet between patches and the suite can spot behaviour changes.
fn print_verification_checksum() {
    static LAST_VERIFICATION_CHECKSUM: std::sync::atomic::AtomicU32 =
        std::sync::atomic::AtomicU32::new(0xFFFF_FFFF);

    if std::env::var_os("LIVE_CODING_VERIFY").is_none() {
        return;
    }
    let checksum = verification_checksum();
    let previous = LAST_VERIFICATION_CHECKSUM.swap(checksum, std::sync::atomic::Ordering::Relaxed);
    if previous != checksum {
        println!("[verify] checksum=0x{:08X}", checksum);
    }
}

fn get_aaa() -> i32 {
    56
}

fn scale_value_0032(input: i32, factor: i32) -> i32 {
    input * factor * get_aaa() * 45
}

/// The engine calls this every frame.  Animate the engine-owned quads so they
/// jump around the window - tweak anything here while the game runs and save
/// to live-reload.
#[unsafe(no_mangle)]
pub extern "C" fn project_update(delta_time: f32) {
    let game_state = state();
    game_state.tick += delta_time;

    let screen_width = unsafe { ((*API).screen_width)() };

    scale_value_0032(game_state.tick as i32, 1);

    // Fan out into the interconnected module graph and fold the results into
    // a small "tuning noise" that perturbs the jump.  This makes the whole
    // graph (tick + compute + sample edges) reachable from the hot-reloaded
    // function, so live coding really exercises the large interconnected
    // project.
    let tick_noise = modules::run_update(game_state.tick as i32) % 1000;
    let compute_noise = modules::run_compute(game_state.tick as i32, 3) % 10000;
    let tuning_noise = tick_noise + compute_noise;

    for quad in game_state.quads[..game_state.quad_count].iter_mut() {
        // Half the quad's width, used to centre it on its x position.
        let half_width = quad.w * QUAD_HALF_SIZE_FACTOR;

        // Drift sideways and bounce off the window edges.
        quad.x += quad.vx * delta_time;
        if quad.x < half_width || quad.x > screen_width - half_width {
            quad.vx = -quad.vx;
            quad.x = quad.x.clamp(half_width, screen_width - half_width - 1.0);
        }

        // Jump: y oscillates between base_y and base_y - jump_height, scaled
        // slightly by the module-graph noise so the interconnected code has a
        // visible (but subtle) effect on the animation.
        quad.jump_phase += quad.jump_speed * delta_time;
        let noise_scale = 1.0 + ((tuning_noise % 47) as f32) / 1000.0;
        quad.y = quad.base_y - quad.jump_phase.sin().abs() * quad.jump_height * noise_scale;
    }

    // Functional-verification hook: prints a deterministic checksum derived
    // from the hot functions (LIVE_CODING_VERIFY=1 only) so the test suite
    // can prove patched code actually runs.
    print_verification_checksum();
}
