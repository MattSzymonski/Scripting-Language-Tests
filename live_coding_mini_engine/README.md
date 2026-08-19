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
   `project_set_api`, runs `project_init`, and resolves the **base
   `project_update` address** (this address never changes).
2. **Per frame** — the engine calls the loaded `project_update(dt)`.  The
   project reads the engine-owned `GameState` through the API table and moves
   the quads (drift + bounce + jump).  The engine then renders them with
   macroquad.
3. **Edit** — `notify` watches the whole `project/` directory recursively
   (every `.rs` + `Cargo.toml`) and pushes events to a channel; the render
   loop drains them on the main thread.  The host reads **all** project source
   files (lib.rs + bloat + module graph), so an edit anywhere is detected.
4. **Leaf function edit** — if *only* the `project_update` body changed, that
   one function is compiled **standalone** via `rustc` (~250–330 ms on the
   large project) into a small DLL, and its **prologue in the loaded base DLL
   is patched in place** (`E9 rel32`, with a trampoline if the target is >2 GB
   away).  The base DLL stays loaded and the update pointer never changes —
   true Live Coding.
5. **Anything else** — a changed module/bloat file, settings, structs,
   `project_init`, `Cargo.toml` changes trigger a **full `cargo build`**; the
   fresh DLL is loaded alongside and the **base prologue is re-patched** to it
   (with a pointer swap as a fallback if the patch fails).  A build error
   prints the compiler output and the previous code keeps running.

## Key design points

- **True live coding for `project_update`**: per-function `rustc` compile +
  in-place prologue patch, exactly like the `live_coding/` runtime.  The
  window, engine and function pointer never change — only the first 5 bytes
  of the base `project_update` are rewritten.
- **The game DLL never links macroquad.**  It talks to the engine only through
  the `#[repr(C)]` `ProjectApi` function-pointer table (mirrored
  field-for-field in `project/src/lib.rs`).  This is the same API-table
  pattern as `hot_reloading/game_rs` ↔ `hot_reloading/flappy`.
- **Game state is engine-owned** and lives in a heap `Box` at a stable
  address, so it **survives every reload** — quad positions, phases and speeds
  are not reset when code is patched or swapped.
- **Self-contained leaf modules**: a standalone-compiled `project_update` gets
  the current project constants, types and helper functions injected, the
  **whole interconnected module graph inlined** as a nested `mod modules`
  (so `project_update` can call `modules::run_update(...)` in the leaf too),
  plus its own `ProjectApi` mirror, `state()` accessor and `project_set_api`
  export.  If a body references something the standalone module can't provide,
  the host automatically falls back to a full rebuild.
- **Multi-file analysis**: the change detector combines lib.rs + bloat + the
  module graph into one source stream, so editing `modules/module_000.rs` or
  `bloat_gen_no_export.rs` is detected (and correctly treated as a full
  rebuild, since only `project_update` bodies can be leaf-patched).

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

Measured on the large project: cold build+load ~1.5 s, full rebuild+re-patch
~1 s, leaf patch-in-place ~250–330 ms.

## How it works (the Live Coding mechanism)

```
base DLL loaded ONCE          patch DLL (freshly compiled)     trampoline (if needed)
┌─────────────────────┐      ┌──────────────────────┐      ┌────────────────────┐
│ update, compute     │      │ new compute code     │      │ mov rax, <addr>    │
│ ...                 │      │ (compiled via rustc) │      │ jmp rax            │
│ E9 <rel32> ─────────┼─────►│                      │      │                    │
│ (patched prologue)  │      └──────────────────────┘      └────────────────────┘
└─────────────────────┘
```

1. **Startup** — the base DLL is copied to a unique temp path and loaded with
   `LoadLibraryW`; `update` and `compute` are resolved into **typed `HotFn`
   wrappers** (no wrapper, no jump table).  The host hands the DLL its service
   table (`set_services`) so game code can reach **host-owned state** that
   survives every patch and reload.

2. **Edit** — you change code anywhere in `patch_dll/` and save.

3. **Detect** — the loop content-hashes every watched file every 800 ms.

4. **Compile** — a changed leaf function is extracted (with a lexically-aware
   mini-lexer that ignores strings/comments) and compiled to a tiny standalone
   DLL via `rustc` in ~160–200 ms.

5. **Patch** — all other threads are suspended, `VirtualProtect` makes the
   original function's page writable, the first 5 bytes become `E9 <rel32>`.
   If the new code is further than ±2 GB away, a cached **trampoline**
   (absolute `jmp rax`) is used.  The instruction cache is flushed and page
   protection restored, then threads resume.

6. **Redirect** — the next call through the ORIGINAL function pointer jumps to
   the new code.  Everything that held a pointer to the old function is now
   running the new implementation.

7. **New functions** — a function that did not exist in the base DLL is
   compiled, loaded, and **registered by name** (with its ABI signature) in a
   symbol registry, then resolvable via `resolve_symbol("name")` — the Rust
   equivalent of UE Live Coding's "add a new function live".

8. **Anything else** — structs, internal helpers, other files, or a leaf that
   references crate internals triggers a **full `cargo build`**; the fresh DLL
   is copied to a unique temp path, loaded **alongside** the old one, the
   host's pointers are swapped, and the old DLL is freed.  A failed build
   never unloads anything — the game keeps running the last good code.

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
