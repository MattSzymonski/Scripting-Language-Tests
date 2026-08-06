// game_lib — The "game" code that gets hot-reloaded.
//
// Edit any function below, save, and the host will:
// 1. Recompile this entire crate to a new DLL (cargo build)
// 2. Load it with LoadLibraryW
// 3. Start calling the new versions
//
// Old DLLs accumulate as game_v1.dll, game_v2.dll, etc.
mod bloat_gen;

/// Called every frame by the host.  v2 — edited live!
#[unsafe(no_mangle)]
pub extern "C" fn update(tick: i32) {
    println!("  [game v2] tick #{tick} — RELOADED! This is the new DLL12222233");
}

/// Returns a computed value.  v2 — multiplication instead of addition.
#[unsafe(no_mangle)]
pub extern "C" fn compute(x: i32, y: i32) -> i32 {
    x * y
}
