# Scripting-Language-Tests

A playground for experimenting with different approaches to game logic — from
scripting languages to live function-level patching. All built on
[macroquad](https://macroquad.rs).

![Flappy Bird — C# on mini_engine](docs/screenshot.png)

## What's in the box

Three scenarios, grouped into **two self-contained Cargo workspaces**:

| #   | Scenario                     | Workspace         | Engine                  | How game logic runs                                                         |
| --- | ----------------------------- | ------------------ | ------------------------ | ----------------------------------------------------------------------------- |
| 1   | **Flappy Bird — Rust**        | `hot_reloading/`  | `engine/` (mini_engine) | `game_rs/` compiled as a cdylib, hot-reloaded at runtime                    |
| 2   | **Flappy Bird — C# scripted** | `hot_reloading/`  | `engine/` (mini_engine) | C# code in `game_cs/`, hosted via .NET runtime FFI, hot-reloaded at runtime |
| 3   | **Live-patching prototype**   | `live_patching/`  | `live_engine/`          | `game_hot/` compiled as a cdylib, hot-reloaded at runtime                   |

- **`hot_reloading/`** — scenarios 1 & 2 *share* one engine (`mini_engine`) and
  differ only in which language the game logic is written in. Both are
  hot-reloadable; there is no statically-compiled-in variant.
- **`live_patching/`** — scenario 3, a function-level live-patching prototype
  with its **own**, deliberately simpler engine.

> **Why two engines / two workspaces?**
> The live-patching prototype needed a much simpler engine. Rather than
> complicate `mini_engine`, it lives in its own crate (`live_engine/`) focused
> purely on the hot-reload mechanism. Its orchestrator treats its own folder as
> the build root (it runs `cargo build -p game_hot` and watches
> `game_hot/src/`), so `live_patching/` is its own workspace, fully independent
> of `hot_reloading/`. All three scenarios run on the same rendering backend
> (macroquad).

Each workspace has its own `target/` and `Cargo.lock`; there is no root
workspace. Run commands are shown per scenario below.

---

## Scenario 1 — Flappy Bird (Rust, hot-reloaded)

```bash
cd hot_reloading
cargo run
```

**What to expect:** a 480×720 Flappy Bird window. Press **Space** or click to
flap the bird through pipes. **Esc** quits.

The game logic lives in `hot_reloading/game_rs/`, compiled as a `cdylib` and
loaded via `libloading` — the same technique scenario 3 uses, adapted to
`mini_engine`. `flappy` builds it once at startup, then watches `game_rs/src/`
for changes.

**To test live-reloading:** keep the game running. In a second terminal or
editor, open `game_rs/src/game.rs` and change something — e.g. `GRAVITY` or
`FLAP_VELOCITY` in the `config` module, or a color in the `color` module. Save
the file.

The terminal running the game will print:

```
[hot] change detected — rebuilding game_rs...
[hot] PATCHED (v3)
```

Switch back to the window — your change takes effect on the next frame, no
restart. Because the world buffer (bird position, pipes, score) is allocated
**once** by the host and outlives every reload, **gameplay state survives** a
hot-reload here — unlike scenario 2 below.

---

## Scenario 2 — Flappy Bird (C# scripted)

Requires the **.NET SDK** installed (`dotnet --list-sdks` to check). The C#
project targets `net8.0`; any newer SDK can build it.

```bash
cd hot_reloading
cargo run --features cs
```

**What to expect:** the exact same Flappy Bird game, but the bird, pipes,
scoring, and rendering are all written in C# (`hot_reloading/game_cs/`). The
Rust engine hosts the .NET runtime and calls into the C# code every frame.

First run takes longer — the build script compiles both C# projects
automatically (`game_cs`, the game, and `game_cs_loader`, described below). If
your SDK can't build `net8.0`, change the target framework in both `.csproj`
files to one you have (e.g. `net9.0`).

**This is also hot-reloadable.** Rust never loads `game_cs.dll` directly — it
loads `game_cs_loader.dll` instead, a small stable assembly
(`hot_reloading/game_cs_loader/`) that owns a *collectible*
`AssemblyLoadContext` and reloads `game_cs.dll` underneath whenever it changes
on disk. While the game runs, rebuild `game_cs` in another terminal:

```bash
dotnet build hot_reloading/game_cs -c Release
```

The loader polls the compiled dll a few times a second and reloads it once the
build lands:

```
[game_cs_loader] reloaded game_cs.dll
```

Unlike scenario 1, **gameplay state resets** on each reload — unloading the
old `AssemblyLoadContext` destroys its entire managed heap, including the
bird/pipes/score, which live in ordinary C# objects. The game restarts at the
"Ready" screen with the new code. (Keeping state alive across a C# reload is
possible — move it into engine-owned native memory instead of the managed
heap — but is out of scope here; see
[`docs/how-csharp-scripting-works.md`](docs/how-csharp-scripting-works.md) for
the design.)

---

## Scenario 3 — Live-patching prototype

> Uses its own engine (`live_engine/`) — not the same `mini_engine` as
> scenarios 1 & 2 — and its own workspace.

```bash
cd live_patching
cargo run              # or: cargo run -p orchestrator
```

**What to expect:** an 800×600 window with a yellow circle you can move with
**arrow keys** and jump with **Space**. Below the circle you'll see
"HOT-PATCHABLE GAME v1".

**To test live-patching:** keep the game running. In a second terminal or
editor, open `live_patching/game_hot/src/lib.rs` and make a change — for
example, change the circle color or gravity. Save the file.

The terminal running the game will print:

```
[orchestrator] Change detected — rebuilding game_hot...
[orchestrator] PATCHED — hot table updated (v3)
```

Switch back to the game window — your change takes effect on the next frame.
No restart, no flicker. You can do this repeatedly.

Changes you can try:
- Player color → find `rgba(255, 200, 50, ...)` in `game_render`
- Gravity → change the `GRAVITY` constant in `game_update`
- Jump strength → change `FLAP_VELOCITY`
- Add entirely new mechanics → edit `game_update`

---

## Project layout

```
├── hot_reloading/         # Workspace for scenarios 1 & 2 (shared engine)
│   ├── engine/            #   mini_engine — the Rust engine
│   ├── flappy/            #   Flappy Bird binary (default run target)
│   ├── game_rs/           #   Flappy Bird game logic — Rust, hot-reloaded (scenario 1)
│   ├── game_cs/           #   Flappy Bird game logic — C# (scenario 2)
│   └── game_cs_loader/    #   Stable loader that makes game_cs hot-reloadable (scenario 2)
├── live_patching/         # Workspace for scenario 3 (own engine)
│   ├── live_engine/       #   Separate engine for live-patching
│   ├── game_hot/          #   Hot-patchable game functions (cdylib)
│   └── orchestrator/      #   Builds, watches, and hot-patches game code
└── docs/                  # Additional documentation
```

## Docs

- [`docs/how-csharp-scripting-works.md`](docs/how-csharp-scripting-works.md) —
  a from-first-principles guide to embedding a managed language (C#) inside a
  native engine: the FFI boundary, marshalling, memory ownership, hot reload,
  safety, and tooling. General-purpose, not tied to this repo.
- [`docs/how-live-patching-works.md`](docs/how-live-patching-works.md) — a
  from-first-principles guide to scenario 3's mechanism: function pointers,
  the hot function table, atomic swaps, executable memory, and what's real vs.
  simulated in this prototype.
- [`docs/reducing-build-artifact-size.md`](docs/reducing-build-artifact-size.md) —
  notes from a separate binary-size experiment (`just_scripting/`), not part
  of the three scenarios above.

## Prerequisites

- **Rust** (stable) — `cargo`
- **.NET SDK** — only for scenario 2 (the projects target `net8.0`)

## Controls (scenarios 1 & 2)

| Key                | Action |
| ------------------ | ------ |
| Space / Up / Click | Flap   |
| Esc                | Quit   |

## Controls (scenario 3)

| Key        | Action |
| ---------- | ------ |
| Arrow keys | Move   |
| Space      | Jump   |
| Esc        | Quit   |
