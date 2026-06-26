# mini_engine + Flappy Bird

A **super-mini object-oriented game engine** in Rust, plus a small **Flappy Bird**
game that uses it. Built on [`macroquad`](https://github.com/not-fl3/macroquad)
for cross-platform windowing and rendering.

## Workspace layout

```
.
├── engine/          # mini_engine: the reusable, game-agnostic engine crate
│   └── src/
│       ├── object.rs    # GameObject  — base "class" for every entity
│       ├── world.rs     # World       — owns objects, drives their lifecycle
│       ├── scene.rs     # Scene       — one screen of the game
│       ├── context.rs   # EngineContext — per-frame services (time/input/window)
│       ├── engine.rs    # Engine      — the main loop
│       └── lib.rs       # re-exports + `prelude`
└── flappy/          # the game crate, depends on mini_engine
    └── src/
        ├── config.rs    # tunable gameplay constants
        ├── bird.rs      # Bird : GameObject
        ├── pipe.rs      # Pipe : GameObject
        ├── scene.rs     # FlappyScene : Scene (states, collisions, scoring)
        └── main.rs      # window config + engine bootstrap
```

## The engine in one picture

The engine is intentionally tiny — four object-oriented building blocks:

| Type            | Role                                                            |
| --------------- | -------------------------------------------------------------- |
| `GameObject`    | Trait with `update` / `draw` / `bounds` / `is_alive`. The base class for entities. |
| `World`         | A container that owns `Box<dyn GameObject>`s and updates/draws/reaps them. |
| `Scene`         | One screen of the game; coordinates objects and global logic.  |
| `Engine`        | Owns the main loop: refresh context → update scene → draw scene → present. |
| `EngineContext` | Per-frame services passed everywhere: `dt`, input, window size, `change_scene`, `quit`. |

A minimal game is just:

```rust
use mini_engine::prelude::*;

struct Hello;
impl Scene for Hello {
    fn update(&mut self, ctx: &mut EngineContext) {
        if ctx.key_pressed(KeyCode::Escape) { ctx.quit(); }
    }
    fn draw(&self, _ctx: &EngineContext) {
        draw_text("hello, engine", 20.0, 40.0, 30.0, WHITE);
    }
}

#[macroquad::main("demo")]
async fn main() {
    Engine::new().run(Hello).await;
}
```

## Running the game

```bash
cargo run -p flappy            # debug
cargo run -p flappy --release  # smoother
```

### Controls

- **Space / Up arrow / Left click** — flap
- **Esc** — quit

Fly through the gaps in the pipes. Each pipe you pass scores a point; hitting a
pipe or the ground ends the run. Your best score persists across retries within
a session.

## How Flappy uses the engine

- `Bird` and `Pipe` both implement `GameObject` — they each know how to update
  themselves (gravity, scrolling) and draw themselves.
- `FlappyScene` implements `Scene`. It owns the bird and a `Vec<Pipe>`, runs the
  `Ready → Playing → GameOver` state machine, spawns pipes on a timer, and does
  collision/scoring using each object's `bounds()` rectangle.
- On retry, the scene calls `ctx.change_scene(...)` and the `Engine` swaps it in.
