# Scripting-Language-Tests

A playground for experimenting with different approaches to game logic — from
traditional compilation to scripting languages to live function-level patching.
All built on [macroquad](https://macroquad.rs).

## What's in the box

Three independent scenarios. Each uses a different approach to game logic — and
importantly, **scenarios 1 & 2 share one engine, scenario 3 has its own**.

| #   | Scenario                      | Engine                  | How game logic runs                                             |
| --- | ----------------------------- | ----------------------- | --------------------------------------------------------------- |
| 1   | **Flappy Bird — pure Rust**   | `engine/` (mini_engine) | Compiled into the binary — `game_rs/` is a regular Rust library |
| 2   | **Flappy Bird — C# scripted** | `engine/` (mini_engine) | C# code in `game_cs/`, hosted via .NET runtime FFI              |
| 3   | **Live-patching prototype**   | `live_engine/`          | `game_hot/` compiled as a cdylib, hot-reloaded at runtime       |

> **Why two engines?**
> The live-patching prototype needed a much simpler engine — no `Scene` trait,
> no `World`, no object hierarchy. Rather than complicate `mini_engine`, it
> lives in its own crate (`live_engine/`) focused purely on the hot-reload
> mechanism. All three run on the same rendering backend (macroquad).

---

## Scenario 1 — Flappy Bird (pure Rust)

No external dependencies beyond `cargo`.

```bash
cargo run
```

**What to expect:** a 480×720 Flappy Bird window. Press **Space** or click to
flap the bird through pipes. **Esc** quits.

The game logic lives in `game_rs/` and is compiled as a regular Rust library.

---

## Scenario 2 — Flappy Bird (C# scripted)

Requires **.NET SDK 8+** installed (`dotnet --list-sdks` to check).

```bash
cargo run --features cs
```

**What to expect:** the exact same Flappy Bird game, but the bird, pipes,
scoring, and rendering are all written in C# (`game_cs/`). The Rust engine
hosts the .NET runtime and calls into the C# code every frame.

First run takes longer — the build script compiles the C# project automatically.
If you don't have .NET SDK 8, install it or change the target framework in
`game_cs/game_cs.csproj` to match what you have (e.g. `net9.0`).

---

## Scenario 3 — Live-patching prototype

> Uses its own engine (`live_engine/`) — not the same `mini_engine` as scenarios 1 & 2.

```bash
cargo run -p orchestrator
```

**What to expect:** an 800×600 window with a yellow circle you can move with
**arrow keys** and jump with **Space**. Below the circle you'll see
"HOT-PATCHABLE GAME v1".

**To test live-patching:** keep the game running. In a second terminal or
editor, open `game_hot/src/lib.rs` and make a change — for example, change
the circle color or gravity. Save the file.

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
├── engine/            # mini_engine — the Rust engine for scenarios 1 & 2
├── flappy/            # Flappy Bird binary (scenarios 1 & 2)
├── game_rs/           # Flappy Bird game logic — pure Rust
├── game_cs/           # Flappy Bird game logic — C#
├── game_hot/          # Hot-patchable game functions (scenario 3)
├── live_engine/       # Separate engine for scenario 3 (live-patching)
├── orchestrator/      # Builds, watches, and hot-patches game code
└── docs/              # Additional documentation
```

## Prerequisites

- **Rust** (stable) — `cargo`
- **.NET SDK 8+** — only for scenario 2

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
