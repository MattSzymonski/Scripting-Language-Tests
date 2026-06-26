//! Flappy Bird host.
//!
//! This binary is the *native shell*: it owns the macroquad window and the
//! `mini_engine` game loop, hosts the .NET runtime, and bridges the two so the
//! actual game (in C#) can drive the engine as a "script".
//!
//! - **Rust -> C#:** every frame the engine calls the managed `Update`/`Draw`
//!   entry points (see [`ScriptedScene`]).
//! - **C# -> Rust:** the C# code calls back through [`EngineApi`], a table of
//!   function pointers wrapping macroquad's drawing/input/time functions.

mod ffi;

use std::path::PathBuf;

use mini_engine::{Engine, EngineContext, Scene};
use macroquad::prelude::*;

use netcorehost::{nethost, pdcstr};
use netcorehost::pdcstring::PdCString;

use ffi::EngineApi;

/// Managed entry points resolved out of the C# assembly. Their addresses are
/// JIT-compiled native thunks we can call like ordinary `extern` functions.
type InitFn = extern "system" fn(*const EngineApi);
type UpdateFn = extern "system" fn(f32);
type DrawFn = extern "system" fn();

/// A `mini_engine` scene whose behaviour is entirely delegated to C#.
struct ScriptedScene {
    update: UpdateFn,
    draw: DrawFn,
}

impl Scene for ScriptedScene {
    fn update(&mut self, ctx: &mut EngineContext) {
        (self.update)(ctx.dt);
        // The C# side calls `quit()` through the API, which sets this flag.
        if ffi::quit_requested() {
            ctx.quit();
        }
    }

    fn draw(&self, _ctx: &EngineContext) {
        (self.draw)();
    }
}

/// Locate the built C# assembly directory. Honors `FLAPPY_MANAGED_DIR`, else
/// falls back to the project's `dotnet build -c Release` output.
fn managed_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("FLAPPY_MANAGED_DIR") {
        return PathBuf::from(dir);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("game_cs")
        .join("bin")
        .join("Release")
        .join("net10.0")
}

/// Boot the .NET runtime, load the C# game, and return its entry points plus
/// the engine API table the game will call back into.
fn load_managed_game() -> Result<(InitFn, UpdateFn, DrawFn), Box<dyn std::error::Error>> {
    let dir = managed_dir();
    let assembly = dir.join("game_cs.dll");
    let runtime_config = dir.join("game_cs.runtimeconfig.json");

    if !assembly.exists() {
        return Err(format!(
            "C# assembly not found at {}.\nBuild it first with:  dotnet build game_cs -c Release",
            assembly.display()
        )
        .into());
    }

    let hostfxr = nethost::load_hostfxr()?;
    let config_pd = PdCString::from_os_str(runtime_config.as_os_str())?;
    let context = hostfxr.initialize_for_runtime_config(&config_pd)?;

    let assembly_pd = PdCString::from_os_str(assembly.as_os_str())?;
    let loader = context.get_delegate_loader_for_assembly(assembly_pd)?;

    // Assembly-qualified type name: "<Namespace>.<Type>, <AssemblyName>".
    let type_name = pdcstr!("Flappy.Interop, game_cs");

    let init = *loader
        .get_function_with_unmanaged_callers_only::<InitFn>(type_name, pdcstr!("Init"))?;
    let update = *loader
        .get_function_with_unmanaged_callers_only::<UpdateFn>(type_name, pdcstr!("Update"))?;
    let draw = *loader
        .get_function_with_unmanaged_callers_only::<DrawFn>(type_name, pdcstr!("Draw"))?;

    Ok((init, update, draw))
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Flappy Bird — C# on mini_engine".to_owned(),
        window_width: 480,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let (init, update, draw) = match load_managed_game() {
        Ok(fns) => fns,
        Err(e) => {
            // Show the error in-window for a few seconds so it isn't missed.
            eprintln!("failed to start C# game: {e}");
            for _ in 0..600 {
                clear_background(BLACK);
                draw_text("Failed to load C# game:", 20.0, 40.0, 28.0, RED);
                for (i, line) in e.to_string().lines().enumerate() {
                    draw_text(line, 20.0, 80.0 + i as f32 * 26.0, 22.0, WHITE);
                }
                next_frame().await;
            }
            return;
        }
    };

    // Hand the engine API table to C# once, up front. `api` lives for the whole
    // run; C# copies it into its own static on the managed side.
    let api = EngineApi::new();
    init(&api as *const EngineApi);

    Engine::new()
        .with_clear_color(BLACK)
        .run(ScriptedScene { update, draw })
        .await;
}
