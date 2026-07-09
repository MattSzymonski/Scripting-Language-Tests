//! Flappy Bird host — two modes: hot-reloaded Rust (default) or C# scripting
//! (opt-in).
//!
//! ```text
//! cargo run                     # Rust version, hot-reloaded at runtime (game_rs)
//! cargo run --features cs       # C#  version (game_cs, needs .NET SDK)
//! ```

use macroquad::prelude::*;

// ---------------------------------------------------------------------------
// Hot-reloaded Rust path (default)
// ---------------------------------------------------------------------------

#[cfg(not(feature = "cs"))]
mod imp {
    use std::ffi::c_void;

    use macroquad::prelude::*;
    use mini_engine::{Engine, EngineContext, Scene};

    use crate::ffi::{self, EngineApi};
    use crate::hot_rs::{self, HotGame};

    /// Forwards `Scene::update`/`draw` through the hot table — same shape as
    /// `ScriptedScene` below for the C# path, just calling into a reloadable
    /// cdylib instead of a hosted .NET assembly.
    struct HotScene {
        api: EngineApi,
        /// The world buffer is allocated **once** and outlives every reload,
        /// so gameplay state (bird position, pipes, score) survives a
        /// hot-reload here — unlike the C# path, where each reload starts a
        /// fresh managed heap.
        world: Vec<u8>,
        hot: HotGame,
    }

    impl Scene for HotScene {
        fn update(&mut self, ctx: &mut EngineContext) {
            let update = self.hot.table.read_update();
            update(&self.api, self.world.as_mut_ptr() as *mut c_void, ctx.dt);
            if ffi::quit_requested() {
                ctx.quit();
            }
        }

        fn draw(&self, _ctx: &EngineContext) {
            let draw = self.hot.table.read_draw();
            draw(&self.api, self.world.as_ptr() as *mut u8 as *mut c_void);
        }
    }

    pub fn window_title() -> &'static str {
        "Flappy Bird — Rust (hot-reload)"
    }

    pub async fn run() {
        let hot = hot_rs::start();

        let world_size = (hot.table.read_world_size())();
        let mut world: Vec<u8> = vec![0u8; world_size.max(1)];
        let api = EngineApi::new();

        (hot.table.read_init())(&api, world.as_mut_ptr() as *mut c_void);

        Engine::new()
            .with_clear_color(BLACK)
            .run(HotScene { api, world, hot })
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

/// Needed by both paths — both call back into the engine through the same
/// `EngineApi` function table.
mod ffi;
/// Only needed for the C# path.
#[cfg(feature = "cs")]
mod hostfxr;
/// Only needed for the hot-reloaded-Rust path.
#[cfg(not(feature = "cs"))]
mod hot_rs;
/// Only needed for the hot-reloaded-Rust path (watches `game_rs/src`).
#[cfg(not(feature = "cs"))]
mod watch;

fn window_conf() -> Conf {
    Conf {
        window_title: imp::window_title().to_owned(),
        window_width: 480,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    imp::run().await;
}
