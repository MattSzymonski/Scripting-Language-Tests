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
    println!("[game update] 123xxxxxtickZZZZawdwZZ1awd2asw3awd!aaa={tick}");
    alpha();
    test::alpha();
    let a = aaa { a: 1, b: 2, c: 3 };
    println!("  aaa struct: a={}, b={}, c={}", a.a, a.b, a.c);
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
