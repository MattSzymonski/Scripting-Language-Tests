# live_coding_mini_engine — Summary

A tiny **macroquad** game engine whose game logic lives in a separately
compiled, **live-code-reloadable** cdylib (`project`).  When you edit the game
code and save, only the game DLL is rebuilt and atomically swapped in — the
window and the engine keep running untouched.

---

## 1. Project layout

```
live_coding_mini_engine/
├── Cargo.toml            workspace: [standalone, engine, project]
├── README.md             quick-start
├── standalone/           host binary (window + watch + hot-swap)
│   └── src/main.rs       macroquad main, builds/loads project, notify watcher
├── engine/               the renderer (lib, statically linked)
│   └── src/
│       ├── lib.rs        re-exports
│       ├── state.rs      Quad + GameState (engine-owned, survives reloads)
│       ├── api.rs        ProjectApi function-pointer table (C ABI)
│       └── engine.rs     MiniEngine: render loop + atomic update pointer
└── project/              the game (cdylib)
    └── src/lib.rs        project_set_api + project_update (jumping quads)
```

## 2. How it works

```
standalone (exe)                       project.dll (cdylib)
┌────────────────────────────────┐
│ MiniEngine (macroquad)         │      ┌──────────────────────┐
│   state: GameState (quads)     │      │ project_update(dt)   │
│   update_fn ──(unchanging)─────┼─────►│   reads state via    │
│   render: draw quads           │      │   ProjectApi table   │
└───────────────┬────────────────┘      └──────────────────────┘
                │ watch project/ → leaf: rustc + prologue-patch
                │                → other: full build + re-patch
```

1. **Startup** — `standalone` runs `cargo build -p project`, copies
   `project.dll` to a unique `project_vN.dll`, loads it with `libloading`,
   calls `project_set_api` + `project_init`, and resolves the **base
   `project_update` address**.  That address never changes afterwards.
2. **Each frame** — the engine calls `project_update(delta_time)` through the
   base pointer (the pointer never changes — only its prologue bytes do).
   The project reaches the engine-owned `GameState` through
   `ProjectApi::get_state` and moves the quads (drift + edge bounce +
   sinusoidal jump).  The engine then renders the quads with macroquad.
3. **Leaf function edit** — if *only* the `project_update` body changed, the
   standalone compiles that one function via `rustc` (~170 ms) into a small
   DLL and **patches the prologue of the base `project_update` in place**
   (`E9 rel32`, trampoline if >2 GB away).  True Live Coding: base DLL stays
   loaded, pointer unchanged.
4. **Anything else** — settings, structs, `project_init`, `Cargo.toml`
   changes trigger a **full `cargo build`**; the fresh DLL is loaded alongside
   and the base prologue is **re-patched** to it (pointer swap as fallback).
   Old libraries are retained forever, so an in-flight call never hits
   unmapped memory.
5. **Failure** — a compile error prints the full compiler output; the previous
   code keeps running.

### 2.1 The C ABI contract

The project DLL must not link macroquad (a second macroquad global state would
be disconnected from the host's window).  Instead it only talks through a
`#[repr(C)]` table of plain function pointers, mirrored field-for-field in both
crates:

| Engine (`engine/src/api.rs`) | Project (`project/src/lib.rs`) |
|---|---|
| `ProjectApi { get_state, screen_width, screen_height }` | identical `struct ProjectApi` |
| `Quad { x, y, base_y, w, h, vx, jump_phase, jump_speed, jump_height, color }` | identical `struct Quad` |
| `GameState { tick, quad_count, quads: [Quad; 8] }` | identical `struct GameState` |

### 2.2 Why state survives reloads

The `GameState` lives in the **engine process**, inside a heap `Box` at a
stable address.  The project DLL only ever holds a raw pointer to it (via the
API).  When the DLL is swapped, the new `project_update` continues reading and
writing the same memory — quad positions, phases and speeds are never reset.

## 3. Measured numbers (typical)

| Step | Time |
|---|---|
| Leaf `project_update` edit (standalone rustc + prologue patch) | ~170 ms |
| Full rebuild + re-patch (settings/structs) | ~0.3–0.5 s |
| Watch reaction | ~0.3–0.8 s (debounced) |

## 4. Pros

- **True Live Coding for `project_update`** — per-function `rustc` compile +
  in-place prologue patch; base DLL stays loaded, the update pointer never
  changes.
- **Window + engine never restart** — the render loop is seamless.
- **Game state survives reloads** (engine-owned state pointer).
- **The game DLL has no heavy dependencies** — it talks through a small C ABI
  table, so standalone rebuilds are fast.
- **Failure-safe** — build errors keep the previous code running.
- **Cross-platform buildable** — macroquad, libloading and notify all work on
  Windows, Linux and macOS (prologue patching itself is Windows x86/x64; other
  OSes use the pointer-swap fallback).

## 5. Cons

- **Leaf patching covers one function** — only `project_update` body edits get
  the fast in-place patch; everything else (settings, structs, `project_init`)
  is a full rebuild + re-patch (still live, but tied to crate size).
- **ABI is fixed** — `project_update`'s signature and the `ProjectApi`/state
  layouts cannot change without restarting the host (they are part of the
  cross-DLL contract).
- **Self-contained leaf module requirement** — a `project_update` body can
  only reference items the generated module provides (the API table, `state()`
  and the injected `const` declarations); anything else falls back to a full
  rebuild.
- **Old libraries are retained forever** (never freed) — safe but grows memory
  over a very long session.
- **New engine/API code still needs a full restart** — only `project/` is
  hot-reloadable, not `engine/` or `standalone/`.

## 6. When to use it

- **Use this** when you want a minimal engine whose game logic is live-coded:
  leaf `project_update` edits patch in place (~170 ms), bigger changes fall
  back to a full rebuild, and state survives everything.
- **Use `live_coding/`** for the same prologue-patching technique with a
  richer runtime (signature registry, host services, leaf fallback).
- **Use `hot_reloading/`** for a simpler whole-DLL pointer-swap hot reload
  with a `world` buffer pattern.
