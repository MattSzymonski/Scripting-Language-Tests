// REQUIREMENTS
//   Rust (stable), Windows, the `watch` feature enabled (cargo run --example
//   minimal_host --features hot_reloader/watch), and a hot-reloadable cdylib
//   project to point at (defaults to the sibling live_coding_mini_engine
//   project; override with LIVE_CODING_EXAMPLE_WORKSPACE).
//
// DESCRIPTION
//   A minimal, HEADLESS host for the hot_reloader crate - no game engine, no
//   macroquad, no window.  It defines its own tiny C-ABI API table (its own
//   GameState + function pointers), loads the project DLL, watches the source
//   and applies live patches, and even CALLS the live update entry point so
//   you can see the patched code actually run.
//
//   This is the "use hot_reloader externally" template: everything the host
//   needs is the config, load_initial and handle_change/watch.
//
// USAGE
//   cargo run --example minimal_host --features hot_reloader/watch
//
// EXAMPLE USAGE
//   # while it runs, edit ../live_coding_mini_engine/project/src/lib.rs and
//   # save - watch the PATCHED lines and the moving state.tick.
//
// --- SCRIPT ---

use std::path::PathBuf;
use std::sync::atomic::{AtomicPtr, Ordering};

use hot_reloader::style::{paint_bright_cyan, paint_bright_green, paint_dim, paint_yellow};
use hot_reloader::{ChangeOutcome, LiveCodeSession, LiveCodeSessionConfig};

// ---------------------------------------------------------------------------
// A minimal C-ABI mirror of what the project DLL expects.  FIELD ORDER IS THE
// CONTRACT - it must match the project's mirrors exactly.
// ---------------------------------------------------------------------------

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

#[repr(C)]
struct GameState {
    tick: f32,
    quad_count: usize,
    quads: [Quad; 8],
}

/// The function-pointer table handed to the DLL's `set_api`.
#[repr(C)]
struct Api {
    get_state: extern "C" fn() -> *mut std::ffi::c_void,
    screen_width: extern "C" fn() -> f32,
    screen_height: extern "C" fn() -> f32,
}

/// Pointer to our heap GameState (stable address, set once).
static STATE_POINTER: AtomicPtr<std::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());

extern "C" fn get_state() -> *mut std::ffi::c_void {
    STATE_POINTER.load(Ordering::Relaxed)
}

extern "C" fn screen_width() -> f32 {
    800.0
}

extern "C" fn screen_height() -> f32 {
    600.0
}

/// Transmute a resolved address into the update entry point and call it.
/// The contract is `extern "C" fn(delta_time: f32)` (see
/// `hot_reloader::ProjectUpdateFn`).
unsafe fn call_update(address: usize, delta_time: f32) {
    let update: extern "C" fn(f32) = std::mem::transmute::<usize, extern "C" fn(f32)>(address);
    update(delta_time);
}

fn main() -> hot_reloader::error::Result<()> {
    // Our engine-owned state.  The project DLL reaches it through the API
    // table's get_state pointer; it survives every reload and patch.
    let quads = std::array::from_fn(|_| Quad {
        x: 0.0,
        y: 0.0,
        base_y: 0.0,
        w: 46.0,
        h: 46.0,
        vx: 0.0,
        jump_phase: 0.0,
        jump_speed: 0.0,
        jump_height: 0.0,
        color: 0,
    });
    let state = Box::new(GameState {
        tick: 0.0,
        quad_count: 8,
        quads,
    });
    STATE_POINTER.store(
        &*state as *const GameState as *mut GameState as *mut std::ffi::c_void,
        Ordering::Release,
    );
    let api = Api {
        get_state,
        screen_width,
        screen_height,
    };

    // Point at a hot-reloadable cdylib project.  Defaults to the sibling
    // live_coding_mini_engine project; override with an env var.
    let workspace = std::env::var_os("LIVE_CODING_EXAMPLE_WORKSPACE")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../live_coding_mini_engine")
        });

    // The mini-engine convenience constructor fills in the standard layout,
    // contract and file->scope mapping.  A custom project would instead build
    // the struct by hand (and could override `contract` / `file_scope` /
    // `infra_names` / `rustc_edition` / ...).
    let config = LiveCodeSessionConfig::mini_engine(&workspace, &api as *const _ as *const ());
    let mut session = LiveCodeSession::new(config)?;

    let update_address = session.load_initial()?;
    println!(
        "{} base project loaded - update entry @ {:#x}",
        paint_bright_cyan("[host]"),
        update_address
    );

    // Prove the loaded code runs headlessly: pump a few frames.
    for _ in 0..30 {
        // SAFETY: update_address points at the exported extern "C" fn(f32).
        unsafe { call_update(update_address, 1.0 / 60.0) };
    }
    println!("{} state.tick = {}", paint_dim("[host]"), state.tick);

    println!(
        "{} watching - edit the project's lib.rs and save to live-patch (Ctrl+C to stop).",
        paint_bright_cyan("[host]")
    );

    // The batteries-included loop (watch feature): blocks forever, applies
    // changes on the main thread, and reports what happened.
    let state_ref = &state;
    let mut current_update = update_address;
    session.watch(move |outcome| {
        match outcome {
            ChangeOutcome::Patched { functions } => {
                println!(
                    "{} patched in place: {}",
                    paint_bright_green("[host]"),
                    functions.join(", ")
                );
            }
            ChangeOutcome::FullReload { update_address } => {
                println!(
                    "{} full rebuild loaded ({update_address:#x}) - base re-patched, pointer unchanged",
                    paint_bright_green("[host]")
                );
                current_update = update_address;
            }
            ChangeOutcome::FullReloadNeedsPointerSwap { update_address } => {
                println!(
                    "{} full rebuild - repoint the engine to {update_address:#x}",
                    paint_yellow("[host]")
                );
                current_update = update_address;
            }
            ChangeOutcome::Nothing => {
                println!("{} no-op save skipped", paint_dim("[host]"));
            }
        }

        // Pump the live update entry a few times to prove the (possibly
        // patched) code actually runs headlessly, then show the state moving.
        for _ in 0..5 {
            // SAFETY: current_update points at the exported extern "C" fn(f32).
            unsafe { call_update(current_update, 1.0 / 60.0) };
        }
        println!("{} state.tick = {:.2}", paint_dim("[host]"), state_ref.tick);
    })
}
