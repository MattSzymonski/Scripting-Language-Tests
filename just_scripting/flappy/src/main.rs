//! Flappy Bird host — dual-mode: pure Rust (default) or C# scripting (opt-in).
//!
//! ```text
//! cargo run                     # Rust version (game_rs)
//! cargo run --features cs       # C#  version (game_cs, needs .NET SDK)
//! ```

use macroquad::prelude::*;
#[cfg(not(feature = "cs"))]
use mini_engine::Engine;

// ---------------------------------------------------------------------------
// Pure-Rust path (default)
// ---------------------------------------------------------------------------

#[cfg(not(feature = "cs"))]
mod imp {
    use super::*;
    use game_rs::FlappyScene;

    pub fn window_title() -> &'static str {
        "Flappy Bird — Rust"
    }

    pub async fn run() {
        Engine::new()
            .with_clear_color(BLACK)
            .run(FlappyScene::new())
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

    fn load_managed_game() -> Result<(InitFn, UpdateFn, DrawFn), Box<dyn std::error::Error>> {
        let dir = managed_dir();
        let assembly = dir.join("game_cs.dll");
        let runtime_config = dir.join("game_cs.runtimeconfig.json");

        if !assembly.exists() {
            return Err(format!(
                "C# assembly not found at {}.\n\
                 \n\
                 Make sure game_cs is targeting a .NET SDK you have installed.\n\
                 Build it manually:  dotnet build game_cs -c Release",
                assembly.display()
            )
            .into());
        }

        // Load hostfxr.dll directly — no static libnethost needed.
        let hostfxr = HostfxrContext::new(&runtime_config)?;

        let init = hostfxr.get_unmanaged_fn::<InitFn>(
            &assembly,
            "Flappy.Interop, game_cs",
            "Init",
        )?;
        let update = hostfxr.get_unmanaged_fn::<UpdateFn>(
            &assembly,
            "Flappy.Interop, game_cs",
            "Update",
        )?;
        let draw = hostfxr.get_unmanaged_fn::<DrawFn>(
            &assembly,
            "Flappy.Interop, game_cs",
            "Draw",
        )?;

        Ok((init, update, draw))
    }

    pub fn window_title() -> &'static str {
        "Flappy Bird — C# on mini_engine"
    }

    pub async fn run() {
        let (init, update, draw) = match load_managed_game() {
            Ok(fns) => fns,
            Err(e) => {
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

/// Only needed for the C# path.
#[cfg(feature = "cs")]
mod ffi;
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

#[macroquad::main(window_conf)]
async fn main() {
    imp::run().await;
}
