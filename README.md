# mini_engine + Flappy Bird (dual Rust / C#)

A **super-mini object-oriented game engine** in Rust, with **two flavours of
Flappy Bird** — a pure-Rust version and a C#-scripted version, all sharing the
same engine core.

The engine (window, game loop, rendering) is Rust on
[`macroquad`](https://github.com/not-fl3/macroquad). Pick your poison:

| `cargo run` | Pure Rust (`game_rs`) — zero dependencies beyond `cargo` |
| `cargo run --features cs` | C# (`game_cs`) — .NET-hosted, drives the engine via FFI |

![Flappy Bird running](docs/screenshot.png)

> **New to this?** For a from-scratch explanation of how C# code can run on a
> Rust engine — what FFI is, how the .NET runtime is hosted, and how the two
> sides call each other — read
> [docs/how-csharp-scripting-works.md](docs/how-csharp-scripting-works.md).

## How the pieces fit

```
┌─────────────────────────────┐         ┌──────────────────────────────┐
│  Rust host (flappy.exe)      │         │  game_rs (pure Rust)          │
│                              │         │                               │
│  mini_engine loop  ──────────┼── Update/Draw ──▶  FlappyScene          │
│  (macroquad window)          │         │   Bird / Pipe                 │
│                              │         │                               │
│     OR  (--features cs)      │         │  game_cs (.NET hosted)        │
│                              │         │                               │
│  EngineApi (fn pointers) ◀───┼── DrawCircle / KeyPressed / … ──────────┤
│  → macroquad draw/input      │         │   via the Engine facade       │
└─────────────────────────────┘         └──────────────────────────────┘
```

- **Rust version** (`game_rs`): the game logic is compiled directly into the
  binary. No runtime, no FFI, no external tooling. Just a direct
  `mini_engine::Scene` implementation.

- **C# version** (`game_cs`, opt-in): the host loads `hostfxr.dll` from the
  installed .NET SDK at runtime, initializes the .NET runtime, resolves three
  `[UnmanagedCallersOnly]` entry points (`Init` / `Update` / `Draw`), and calls
  `Update`/`Draw` every frame from a `mini_engine` `Scene`.
- **C# → Rust:** at startup the host hands C# an `EngineApi` — a `#[repr(C)]`
  table of function pointers wrapping macroquad (draw, input, time, rng). C#
  game code calls them through a friendly `Engine` facade.

## Workspace layout

```
.
├── engine/            # mini_engine — the reusable Rust engine core
│   └── src/{object,world,scene,context,engine}.rs
├── flappy/            # the native host (flappy.exe)
│   ├── build.rs       #   auto-builds game_cs when `--features cs` is set
│   └── src/
│       ├── main.rs    #   dual-mode entry point (Rust vs. C# via cfg)
│       ├── ffi.rs     #   EngineApi: the C ABI exposed to C#
│       └── hostfxr.rs #   dynamic hostfxr loading (C# path only)
├── game_rs/           # the game — 100% Rust (default)
│   └── src/
│       ├── primitives.rs   # Vec2 / Rect / Color
│       ├── config.rs       # Gameplay constants
│       ├── bird.rs         # Player bird
│       ├── pipe.rs         # Scrolling pipe pairs
│       ├── flappy_scene.rs # Ready → Playing → GameOver state machine
│       └── lib.rs
└── game_cs/           # the game — 100% C# (opt-in)
    └── src/
        ├── EngineApi.cs    # struct mirroring flappy/src/ffi.rs (the contract)
        ├── Engine.cs       # Key/MouseButton + the Engine facade
        ├── Primitives.cs   # Vec2 / Rect / Color
        ├── GameObject.cs   # GameObject base class + Config constants
        ├── Bird.cs         # Bird : GameObject
        ├── Pipe.cs         # Pipe : GameObject
        ├── FlappyScene.cs  # Ready → Playing → GameOver, scoring, collisions
        └── Interop.cs      # [UnmanagedCallersOnly] Init/Update/Draw
```

## Prerequisites

- **Rust** (stable) — `cargo`
- **.NET SDK 8+** — `dotnet` on `PATH` (only needed for the C# path)

## Build & run

### Rust version (default — no .NET needed)

```bash
cd flappy
cargo run
```

### C# version (requires .NET SDK)

```bash
cd flappy
cargo run --features cs
```

The `cs` feature activates `build.rs` which runs `dotnet build game_cs -c Release`
before the host, so everything is a single command.

Equivalent manual C# steps:

```bash
dotnet build game_cs -c Release
cd flappy
cargo run --features cs
```

The C# host looks for the assembly at `game_cs/bin/Release/net8.0/`.
Override with the `FLAPPY_MANAGED_DIR` environment variable if you build
elsewhere or target a different framework.

### Controls

- **Space / Up arrow / Left click** — flap
- **Esc** — quit

Fly through the gaps; each pipe passed scores a point. Hitting a pipe or the
ground ends the run, and your best score persists across retries in a session.

## Extending the bridge

To expose a new engine capability to C#:

1. Add an `extern "C"` function and a field to `EngineApi` in
   [flappy/src/ffi.rs](flappy/src/ffi.rs).
2. Add the matching `delegate* unmanaged[Cdecl]<…>` field **in the same position**
   to `EngineApi` in [game_cs/src/EngineApi.cs](game_cs/src/EngineApi.cs).
3. Surface it on the `Engine` facade in [game_cs/src/Engine.cs](game_cs/src/Engine.cs).

The field order is the ABI contract — keep both `EngineApi` structs in lockstep.
