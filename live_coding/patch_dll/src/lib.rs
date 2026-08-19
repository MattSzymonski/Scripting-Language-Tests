// patch_dll — Game DLL for the live_patch hot-reload runtime.
//
// Compiles to a Windows cdylib.  Exports `update` and `compute` as
// `extern "C"` entry points — these are the functions the live_patch
// host calls every tick through the jump table.
//
// Edit this file (or test.rs), save, and the hot-reload loop picks
// up the change.  Leaf functions like `compute` are compiled individually
// via `rustc` in ~160 ms.  Functions with internal dependencies (like
// `update`, which calls `alpha()` and `test::alpha()`) trigger a full
// `cargo build` reload.
//
// To stress-test with 5 000 bloat functions, add:
//   mod bloat_gen_no_export;
// and copy bloat_gen_no_export.rs from classic_full_hot_reload/game_lib/src/.

mod test;

// ---------------------------------------------------------------------------
// Internal helpers — not exported, called by update()
// ---------------------------------------------------------------------------

pub fn beta() -> String {
    "betasds".to_string()
}

pub fn alpha() {
    println!("  alpdawadwhaaa {}", beta());
    println!("  alphasawddadwww");
}

pub fn alphssa() {
    println!("  alpha {}", beta());
    println!("  alphasd111111aaaaaaaaaaaaaa1aaaaaaaaaaaaadwww123");
}
// ---------------------------------------------------------------------------
// Host services — host-owned state that survives every patch and reload
// ---------------------------------------------------------------------------

/// Must match the host's `live_coding::HostServices` layout.
#[repr(C)]
struct HostServices {
    get_state: unsafe extern "C" fn() -> *mut std::ffi::c_void,
}

/// Must match the host's `live_coding::HostGameState` layout.
#[repr(C)]
struct HostGameState {
    total_ticks: i64,
    score: i64,
}

static mut HOST_SERVICES_PTR: *const HostServices = std::ptr::null();

/// Called by the host after every load/reload with its service table.
#[unsafe(no_mangle)]
#[allow(private_interfaces)]
pub extern "C" fn set_services(services: *const HostServices) {
    unsafe {
        HOST_SERVICES_PTR = services;
    }
}

/// Fallback scratch state used if `set_services` has not been called yet.
static SCRATCH_STATE: std::sync::Mutex<HostGameState> = std::sync::Mutex::new(HostGameState {
    total_ticks: 0,
    score: 0,
});

/// Access the host-owned game state (never reset by patches or reloads).
fn host_state() -> &'static mut HostGameState {
    unsafe {
        // The host calls set_services right after every load/reload, so this
        // normally reaches the host-owned state directly.
        let state_pointer = if HOST_SERVICES_PTR.is_null() {
            // Fallback: borrow from the scratch buffer (its address is fixed).
            let mut guard = SCRATCH_STATE.lock().unwrap();
            &mut *guard as *mut HostGameState
        } else {
            ((*HOST_SERVICES_PTR).get_state)() as *mut HostGameState
        };
        &mut *state_pointer
    }
}

// ---------------------------------------------------------------------------
// Public exports — the hot-reloadable entry points
// ---------------------------------------------------------------------------

struct aaa {
    a: i32,
    b: i32,
    c: i32,
}

struct bbb {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

/// Main game update — called every tick by the host.
#[unsafe(no_mangle)]
pub extern "C" fn update(tick: i32) {
    let state = host_state();
    state.total_ticks += 1;
    state.score += tick as i64;
    println!(
        "[game update] tick={tick} | host state: total_ticks={}, score={} (survives reload)",
        state.total_ticks, state.score
    );
    alpha();
    test::alpha();
    let a = aaa { a: 1, b: 2, c: 3 };
    println!("  AWDWaAWDAWDaa struct: a={}, b={}, c={}", a.a, a.b, a.c);
    let b = bbb {
        a: 4,
        b: 5,
        c: 6,
        d: 7,
    };
    println!("  bbb struct: a={}, b={}, c={}, d={}", b.a, b.b, b.c, b.d);
    alphssa();
}

/// Compute helper — a leaf function (no internal dependencies).
/// Perfect candidate for prologue-patching via Live Coding.
#[unsafe(no_mangle)]
pub extern "C" fn compute(x: i32, y: i32) -> i32 {
    x * y * 3
}
