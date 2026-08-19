# live_coding_mini_engine

A tiny **macroquad-based mini engine** with **Live Coding** hot-reload for the
game code, in the style of UE Live Coding / Live++:

- **standalone** — the host binary.  Opens the window, statically links the
  engine, **watches the project source**, rebuilds the project DLL on change,
  and hot-swaps it into the engine.  The window and engine never restart.
- **engine** (`mini_engine`) — the renderer.  Owns the macroquad window and
  render loop, holds the **engine-owned game state** (the quads), and calls
  the project's `update` function every frame.
- **project** — the game logic, compiled as a **cdylib**.  Exports
  `project_set_api` (receives the engine's API table) and `project_update`
  (called by the engine each frame).  Animates the quads so they jump.  It is
  also the **large-project test case**: an auto-generated interconnected
  module graph (`modules.rs` + `modules/module_*.rs`, reachable from
  `project_update`) plus ~2000 auto-generated bloat functions
  (`bloat_gen_no_export.rs`, compile volume only).

```
┌──────────── standalone (exe) ────────────┐
│  window + engine (macroquad)  ◄────────┐ │
│  ┌───────────────────────────────┐     │ │
│  │  MiniEngine                   │     │ │  watches project/ + rebuilds
│  │  state: GameState (quads)     │     │ │  + atomically swaps update fn
│  │  update_fn ──► project_update │     └─┼─►  project.dll (cdylib)
│  └───────────────────────────────┘     │ │
└──────────────────────────────────────────┘
```

## Running

```powershell
cargo run -p standalone
```

A window opens with eight quads jumping around.  Edit
`project/src/lib.rs`, save, and the quads' behaviour updates **live** — the
window and engine stay up; only the game DLL is rebuilt and swapped.

Press `Escape` to quit.

## How the live coding works

1. **Startup** — `standalone` builds `project` (`cargo build -p project
   --target-dir target_reload`), copies the DLL to a unique versioned name,
   loads it with `libloading`, hands it the engine's `ProjectApi` table via
   `project_set_api`, runs `project_init`, and resolves the address of **every
   hot function** through the DLL's **single exported `project_resolve_symbol`**
   entry point (which maps a name to the address of `project_update`,
   `run_update`, `run_compute`, and all `tick_N`/`compute_N`/`sample_N`/
   `scale_value_N`/`offset_value_N`).  These become the base symbol table.
2. **Per frame** — the engine calls the loaded `project_update(dt)`.  The
   project reads the engine-owned `GameState` through the API table and moves
   the quads (drift + bounce + jump).  The engine then renders them with
   macroquad.
3. **Edit** — `notify` watches the whole `project/` directory recursively
   (every `.rs` + `Cargo.toml`) and pushes events to a channel; the render
   loop drains them on the main thread.  The host reads **all** project source
   files (lib.rs + bloat + module graph), so an edit anywhere is detected.
4. **Per-function patch** — if *only the body* of one or more **hot-exported
   functions** changed (leaf `project_update` OR a function in the middle of
   the call graph like `tick_000`, `sample_003`, `scale_value_000`), each one
   is compiled **standalone** via `rustc` (~200–250 ms) into a small DLL and
   its **prologue in the loaded base DLL is patched in place** (`E9 rel32`,
   with a trampoline if the target is >2 GB away).  The base DLL stays loaded
   and pointers never change — true Live Coding for leaves AND middle
   functions.
5. **Anything else** — a changed bloat file, settings, structs,
   `project_set_api`/`project_init`, a new function, a signature change, or
   `Cargo.toml` changes trigger a **full `cargo build`**; the fresh DLL is
   loaded alongside, the base prologue is re-patched to it, and the hot
   symbol table is re-resolved.  A build error prints the compiler output and
   the previous code keeps running.

## Key design points

- **True live coding for ANY hot function**: per-function `rustc` compile +
  in-place prologue patch for leaves AND functions in the middle of the call
  graph.  The window, engine and function pointers never change — only the
  first 5 bytes of the patched function's prologue are rewritten.
- **The game DLL never links macroquad.**  It talks to the engine only through
  the `#[repr(C)]` `ProjectApi` function-pointer table (mirrored
  field-for-field in `project/src/lib.rs`).  This is the same API-table
  pattern as `hot_reloading/game_rs` ↔ `hot_reloading/flappy`.
- **Game state is engine-owned** and lives in a heap `Box` at a stable
  address, so it **survives every reload** — quad positions, phases and speeds
  are not reset when code is patched or swapped.
- **One exported resolver instead of exporting every function**: the graph
  functions are plain `pub fn` (clean Rust, no `#[no_mangle]`, no export-table
  pollution — the DLL exports only `project_set_api`, `project_init`,
  `project_update` and `project_resolve_symbol`).  The host resolves any hot
  function's address by calling `project_resolve_symbol(name)`, generated to
  match every graph function (and mirrored inside each patch DLL).
- **Patches call back into the base DLL (no dependency islands)**: a patch
  module contains the changed function's real new body plus a **thin
  forwarding wrapper for every other hot symbol**, each jumping through a
  dependency-address table the host fills at load (`project_set_dependencies`,
  exactly like the `project_set_api` table).  So a patched `project_update`
  calls the *base* `run_update`, a patched `tick_000` calls the *base*
  `sample_003`, and patching `sample_003` later affects every caller — the
  base DLL is the single source of truth and **patches compose**.
- **Multi-file analysis**: the change detector combines lib.rs + bloat + the
  module graph into one source stream (stripping `pub extern` and `pub fn`
  bodies), so editing `modules/module_000.rs` or `bloat_gen_no_export.rs` is
  detected; only hot-function body edits are per-function patched, everything
  else triggers a full rebuild.

## Large-project test content

The project is deliberately grown to stress the live-coding paths:

- `modules.rs` + `modules/module_000.rs`..`module_019.rs` — a 20-module
  **interconnected graph** (ring topology): the root fans out into every
  module, each `tick_N`/`compute_N` pulls leaf samples from two *other*
  modules, and `project_update` folds both `run_update` and `run_compute`
  into a small noise term that perturbs the quads' jump — so every module is
  reachable from the hot-reloaded function.
- `bloat_gen_no_export.rs` — 2000 `pub(crate)` bloat functions with string
  work and arithmetic loops (no `format!`, per the macro-expansion lesson).
  They are dead code on the hot path, so they only add full-rebuild cost; the
  leaf patch never sees them.
- Regenerate either with `python generate_modules.py [--count N]` and
  `python gen_bloat.py [--count N]` from `live_coding_mini_engine/`.

Measured on the large project: cold build+load ~0.7 s, full rebuild+re-patch
~0.6–1.8 s, per-function patch-in-place ~200–250 ms for any hot function
(project_update, tick_000, sample_003, scale_value_000 — all verified).

## How per-function patching composes (the mechanism)

```
base DLL (loaded once, single source of truth)      patch DLL (per changed fn)
┌────────────────────────────────────────────┐     ┌──────────────────────────────┐
│ project_update  E9 ──────► (patched)      │     │ project_update  (new body)   │
│ run_update ──────────────► (wrappers)     │     │   calls modules::run_update │
│ tick_000     E9 ──────► (patched)         │     │   ──► wrapper ──► base addr  │
│ sample_003   E9 ──────► (patched)         │◄────┼─ dependency table (usize[])  │
└────────────────────────────────────────────┘     └──────────────────────────────┘
```

1. **Startup** — the base DLL is loaded once and every hot-exported function's
   address is resolved by name into the host's symbol table.
2. **Edit a hot function** — its new body is compiled standalone; every OTHER
   hot function in the patch module becomes a forwarding wrapper that jumps
   through the dependency table to the base DLL's copy.
3. **Patch** — all other threads are suspended, `VirtualProtect` makes the
   original function's page writable, the first 5 bytes become `E9 <rel32>`.
   If the new code is further than ±2 GB away, a cached **trampoline**
   (absolute `jmp rax`) is used.  The instruction cache is flushed and page
   protection restored, then threads resume.
4. **Composition** — because every patched function calls back into the base
   DLL, patching `tick_000` then `sample_003` (which `tick_000` calls) works:
   the patched `tick_000`'s wrapper hits the base `sample_003`, whose prologue
   now points at the new code.  Patches stack without rebuilding each other.
5. **Non-hot changes** — new functions, signature changes, structs/consts,
   bloat, `project_set_api`/`project_init` or `Cargo.toml` edits trigger a
   **full `cargo build`** + load-alongside; the base prologue is re-patched to
   the fresh DLL and the symbol table is re-resolved.  A failed build never
   unloads anything — the game keeps running the last good code.

## What makes this different from the jump-table approach

| | Jump table (subsecond) | **Live Coding (this project)** |
|---|---|---|
| Call overhead | One hash-map lookup per call | **Zero** (direct call) |
| Opt-in | Requires `HotFn` wrapper | **None** — any caller is redirected |
| New functions | Not supported (no old address) | **Supported** via name registry |
| Mechanism | Swap pointer in `HashMap` | Patch function prologue in place |
| Memory | Base DLL + patch DLLs | Base DLL + patch DLLs + trampolines |

## What does NOT happen

- The base DLL is never unloaded before a new one is ready — reloads are
  load-alongside (no unload gap, no freed-pointer window)
- A failed `cargo build` never stops the game — the previous DLL keeps running
- No process-wide wrapper or indirection is introduced at call sites
- `cargo build` is only called when a change cannot be prologue-patched:
  functions with internal dependencies (e.g. `update`), changes outside
  function bodies, or a leaf that references crate internals.  Pure leaf-body
  changes (`compute`) are patched directly via `rustc` + prologue patch
- The prologue patch runs with all other threads suspended

## Hardening added (the "solve the cons" pass)

- **Load-alongside reload** — no unload gap; failed builds keep the old DLL
- **Host-owned state** — mutable game state lives in the host (`HostServices`,
  `set_services`) and survives every patch and reload
- **Typed `HotFn<S>`** — all `transmute`s confined to one audited wrapper
- **Signature registry** — ABI mismatches are rejected at patch time
- **Thread suspension** — every prologue patch runs with other threads stopped
- **Mini-lexer** — extraction ignores strings/comments/raw strings
- **Resource reuse** — previous patch DLLs freed on re-patch; trampolines
  cached per target; base-DLL temp copies deleted after the swap
- **Wider watching** — `Cargo.toml` + `build.rs` are watched too

## Limitations (same as UE Live Coding)

- Cannot change function signatures (ABI must stay compatible) — mismatches
  are now caught at patch time by the signature registry
- Cannot change class/struct layout of existing types — non-extern changes
  force a full rebuild, which handles it consistently
- Patched code lives in memory / temp DLLs only — a full rebuild is still
  needed to persist changes into the shipped binary
- Prologue patching is x86/x64-only (Windows) and still can't change the
  ABI of an existing function
