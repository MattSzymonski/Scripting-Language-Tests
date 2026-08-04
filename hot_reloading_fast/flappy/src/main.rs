//! Flappy Bird host — two modes: Rust hot-*patching* via subsecond (default),
//! or C# scripting (opt-in).
//!
//! ```text
//! dx serve --hot-patch -p flappy   # Rust version, function-level hot-patching (game_rs)
//! cargo run --features cs          # C#  version (game_cs, needs .NET SDK)
//! ```

use macroquad::prelude::*;

// ---------------------------------------------------------------------------
// Subsecond hot-patching path (default, run via `dx serve --hot-patch`)
// ---------------------------------------------------------------------------
//
// `game_rs` is statically linked in here as a normal dependency (not built
// into a separate cdylib and loaded via `libloading`, the way an earlier
// version of this project worked) — subsecond can only patch functions that
// are part of the same build graph `dx` manages. Each call into game_rs is
// wrapped in `subsecond::call` so ThinLink can redirect it to a freshly-
// patched version without restarting the process.

#[cfg(not(feature = "cs"))]
mod imp {
    use std::ffi::c_void;

    use macroquad::prelude::*;
    use mini_engine::{Engine, EngineContext, Scene};

    use crate::ffi::{self, engine_api_for_game_rs};

    struct SubsecondScene {
        api: game_rs::api::EngineApi,
        world: Vec<u8>,
    }

    impl Scene for SubsecondScene {
        fn update(&mut self, ctx: &mut EngineContext) {
            let api = &self.api as *const game_rs::api::EngineApi;
            let world = self.world.as_mut_ptr() as *mut c_void;
            let dt = ctx.dt;
            subsecond::call(|| {
                game_rs::game_update(api, world, dt);
            });
            if ffi::quit_requested() {
                ctx.quit();
            }
        }

        fn draw(&self, _ctx: &EngineContext) {
            let api = &self.api as *const game_rs::api::EngineApi;
            let world = self.world.as_ptr() as *mut u8 as *mut c_void;
            subsecond::call(|| {
                game_rs::game_draw(api, world);
            });
        }
    }

    pub fn window_title() -> &'static str {
        "Flappy Bird — Rust (subsecond hot-patch)"
    }

    pub async fn run() {
        // Reports this process's ASLR base address to `dx serve` over a websocket
        // so it knows where in memory to apply future patches — without this,
        // dx logs "Ignoring hotpatch since there is no ASLR reference."
        dioxus_devtools::connect_subsecond();

        let world_size = game_rs::game_world_size();
        let mut world: Vec<u8> = vec![0u8; world_size.max(1)];
        let api = engine_api_for_game_rs();

        game_rs::game_init(&api as *const game_rs::api::EngineApi, world.as_mut_ptr() as *mut c_void);

        Engine::new()
            .with_clear_color(BLACK)
            .run(SubsecondScene { api, world })
            .await;
    }
}

// ---------------------------------------------------------------------------
// C# scripting path (`--features cs`)
// ---------------------------------------------------------------------------

#[cfg(feature = "cs")]
mod imp {
    use std::path::PathBuf;

    use macroquad::prelude::*;
    use mini_engine::{Engine, EngineContext, Scene};

    use crate::ffi::EngineApi;
    use crate::hostfxr::HostfxrContext;

    type InitFn = extern "system" fn(*const EngineApi);
    type UpdateFn = extern "system" fn(f32);
    type DrawFn = extern "system" fn();

    struct ScriptedScene {
        update: UpdateFn,
        draw: DrawFn,
    }

    impl Scene for ScriptedScene {
        fn update(&mut self, ctx: &mut EngineContext) {
            (self.update)(ctx.dt);
            if crate::ffi::quit_requested() {
                ctx.quit();
            }
        }

        fn draw(&self, _ctx: &EngineContext) {
            (self.draw)();
        }
    }

    fn managed_dir() -> PathBuf {
        if let Ok(dir) = std::env::var("FLAPPY_MANAGED_DIR") {
            return PathBuf::from(dir);
        }
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("game_cs")
            .join("bin")
            .join("Release")
            .join("net8.0")
    }

    fn loader_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("game_cs_loader")
            .join("bin")
            .join("Release")
            .join("net8.0")
    }

    /// Loads `game_cs_loader` — a small assembly that is loaded once (via
    /// hostfxr, same as before) and never reloaded. It owns a collectible
    /// `AssemblyLoadContext` and reloads the *actual* game (`game_cs.dll`)
    /// underneath whenever it changes on disk; see
    /// `game_cs_loader/src/GameHost.cs`. From here, `game_cs` is
    /// hot-reloadable: while the game runs, run `dotnet build game_cs -c
    /// Release` and the loader picks up the new version on its own, no
    /// restart needed. (Gameplay state resets on each reload; see the
    /// loader's doc comment for why.)
    fn load_managed_game() -> Result<(InitFn, UpdateFn, DrawFn), Box<dyn std::error::Error>> {
        let game_dir = managed_dir();
        let game_assembly = game_dir.join("game_cs.dll");
        if !game_assembly.exists() {
            return Err(format!(
                "C# assembly not found at {}.\n\
                 \n\
                 Make sure game_cs is targeting a .NET SDK you have installed.\n\
                 Build it manually:  dotnet build game_cs -c Release",
                game_assembly.display()
            )
            .into());
        }

        let loader_dir = loader_dir();
        let assembly = loader_dir.join("game_cs_loader.dll");
        let runtime_config = loader_dir.join("game_cs_loader.runtimeconfig.json");
        if !assembly.exists() {
            return Err(format!(
                "C# loader assembly not found at {}.\n\
                 Build it manually:  dotnet build game_cs_loader -c Release",
                assembly.display()
            )
            .into());
        }

        // The loader reads this to find `game_cs.dll` — it never hard-codes
        // a path relative to its own (separate) output directory.
        std::env::set_var("FLAPPY_MANAGED_DIR", &game_dir);

        // Load hostfxr.dll directly — no static libnethost needed.
        let hostfxr = HostfxrContext::new(&runtime_config)?;

        let init = hostfxr.get_unmanaged_fn::<InitFn>(
            &assembly,
            "Flappy.Loader.LoaderInterop, game_cs_loader",
            "Init",
        )?;
        let update = hostfxr.get_unmanaged_fn::<UpdateFn>(
            &assembly,
            "Flappy.Loader.LoaderInterop, game_cs_loader",
            "Update",
        )?;
        let draw = hostfxr.get_unmanaged_fn::<DrawFn>(
            &assembly,
            "Flappy.Loader.LoaderInterop, game_cs_loader",
            "Draw",
        )?;

        Ok((init, update, draw))
    }

    pub fn window_title() -> &'static str {
        "Flappy Bird — C# on mini_engine (hot-reload)"
    }

    pub async fn run() {
        let (init, update, draw) = match load_managed_game() {
            Ok(fns) => fns,
            Err(e) => {
                eprintln!("failed to start C# game: {e}");
                return;
            }
        };

        let api = EngineApi::new();
        init(&api as *const EngineApi);

        Engine::new()
            .with_clear_color(BLACK)
            .run(ScriptedScene { update, draw })
            .await;
    }
}

// ---------------------------------------------------------------------------
// Shared entry point
// ---------------------------------------------------------------------------

/// Needed by both paths — each calls back into the engine through the same
/// `EngineApi` function table shape.
mod ffi;
/// Only needed for the C# path.
#[cfg(feature = "cs")]
mod hostfxr;

fn window_conf() -> Conf {
    Conf {
        window_title: imp::window_title().to_owned(),
        window_width: 480,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

/// Fixed top-left screen position the window is moved to on every launch.
/// miniquad's `Conf` has no window-position field, so this is applied at
/// runtime via `set_window_position` instead of at window-creation time.
const WINDOW_X: u32 = 1400;
const WINDOW_Y: u32 = 800;

#[macroquad::main(window_conf)]
async fn main() {
    macroquad::miniquad::window::set_window_position(WINDOW_X, WINDOW_Y);
    imp::run().await;
}
