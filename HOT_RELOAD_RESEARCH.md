# Hot Reload Bottleneck Research

## Front-end cost: dead code isn't free just because it's cached

The linking analysis below explains the *back-end* (link) cost of unused
functions.  But there is a **second, separate** cost that exists even when a
module's `.o` file is byte-for-byte cached and never relinked into anything:
**parsing and macro expansion re-run on every single build, regardless of what
changed.**

### The experiment

`classic_full_hot_reload/game_lib` has a `bloat_gen_no_export.rs` module: 5,000
`pub(crate) fn` functions, never called from anywhere, each containing three
`format!` calls.  Since the functions are never referenced, dead-code
elimination strips them from the linked DLL, and the module's `.o` file proved
byte-identical (verified via `Get-FileHash`) across rebuilds where nothing in
that file changed. Given that, editing an unrelated file (`test.rs`) and
rebuilding *should* be nearly instant. It wasn't:

| Configuration | `test.rs`-touch rebuild time |
|---|---|
| No bloat module in the crate | **426 ms** |
| Bloat module present, `format!` replaced with `String::from` | **669 ms** (+243 ms) |
| Bloat module present, real `format!` calls (as shipped) | **~950 ms** (+520 ms total) |

(All measurements taken with `[profile.dev] debug = 0` correctly applied at
the **workspace root** `Cargo.toml` -- profile settings in a workspace member's
own `Cargo.toml` are silently ignored by Cargo.)

### Why: incremental caching only covers the back half of the pipeline

rustc's query-based incremental compilation can skip re-running typeck, MIR
building, and codegen for a function once it proves (via a fingerprint hash)
that the function's HIR is unchanged.  But to *compute* that fingerprint, it
first has to:

1. Tokenize and parse the file
2. **Fully expand every macro in it** (`format!` included)
3. Build the HIR
4. *Then* hash it and compare to the cached fingerprint

There is no way to "peek" at whether a file changed without doing steps 1-3
first -- the front-end pass over a crate's source is proportional to source
size and macro complexity on **every build**, no matter how small the edit
was elsewhere in the crate. Only the back end (steps after fingerprinting)
benefits from incremental reuse, which is exactly why the `.o` file stayed
cached while wall-clock time still went up.

This means:

- **+243 ms** comes from simply having 5,000 functions in the crate --
  every item still needs its HIR parsed and fingerprinted on every build.
- **+~280 ms** more comes specifically from `format!` -- each macro call
  expands to multiple AST/HIR nodes (`Arguments::new_v1`, trait resolution
  for `Display`, etc.), and with 3 calls per function across 5,000 functions
  that's 15,000 macro expansions repeated on every single build.

### Takeaway

"The linker will strip it" and "the `.o` is cached" are both true and both
irrelevant to this cost -- it's paid in the compiler **front end**, before
dead-code elimination or incremental codegen reuse ever come into play. The
only real fix is to keep such generated/unused bloat **out of the crate**
that's compiled on every hot-reload cycle (a separate crate, or a
`#[cfg(feature = "bloat")]` gate that's off by default).

## The question

When hot-reloading a DLL with thousands of functions, **what is the bottleneck --
recompiling or relinking?**

## The answer: **linking**

Rust's incremental compilation already skips recompilation of unchanged source
files.  When you edit a single function in `lib.rs` and run `cargo build`, only
that one file is recompiled to a `.o` object file.  The other 4,999 functions in
`bloat_gen.rs` are **not recompiled** -- their cached `.o` files are reused.

| Phase | What happens | Cost at 5,000 functions |
|---|---|---|
| **Compile** | Only changed `.rs` → `.o` | ~fast (tens of ms) |
| **Link** | Linker collects **all** `.o` files into one `.dll` | ~**dominant** (≈1.2s of the 1.4s total) |

### What determines link cost? It's about reachability, not line count.

The linker does **not** care how many lines of source you have.  It only cares
about one thing: **how many functions are reachable from the DLL's public
exports.**  Understanding this is the key to fixing the bottleneck.

#### 1. Compilation unit: one `.rs` file → one `.o` file

```
game_lib/src/
  lib.rs          → target/.../lib.o         (update, compute)
  bloat_gen.rs    → target/.../bloat_gen.o   (5,000 bloat fns)
  mod_foo.rs      → target/.../mod_foo.o

+ all dependency crates (std, etc.) → hundreds more .o files
```

When you edit `lib.rs`:

- **rustc** recompiles only `lib.rs` → `lib.o` (fast, a single small file)
- **rustc** skips `bloat_gen.rs` — `bloat_gen.o` is cached and untouched
- **linker** opens ALL `.o` files and glues them together ← **the bottleneck lives here**

#### 2. What the linker must (and must not) process

The linker processes every function that is **reachable from an export**.
"Reachable" means the function is either:

| Category | Must linker process it? | Why |
|---|---|---|
| **Exported** (`pub extern "C"`) | ✅ Always | Public API — any `GetProcAddress` caller can ask for it |
| **Called by an export** (transitively) | ✅ Always | The export needs it to run — linker must resolve the call |
| **Private, never called** | ❌ Never | Compiler eliminates it (dead code elimination) |

This creates three distinct scenarios:

```
SCENARIO A — Best case (O(1) link, always fast)
─────────────────────────────────────────────
pub extern "C" fn update() { ... }     ← 1 export
fn helper_1() { }                      ← never called → eliminated
fn helper_2() { }                      ← never called → eliminated
...4,999 more dead private fns...      ← ALL eliminated at compile time

Linker sees: 1 tiny .o with just update() → tens of ms, forever.
```

```
SCENARIO B — Exports poison (O(n) link, this repo's stress test)
─────────────────────────────────────────────
pub extern "C" fn update() { ... }     ← 1 usable export
pub extern "C" fn bloat_0() { ... }    ← never called, but EXPORTED → must keep
pub extern "C" fn bloat_1() { ... }    ← never called, but EXPORTED → must keep
...5,000 total exports...              ← linker must process every single one

Linker sees: 5,000 exports + ~15,000 relocations → ~1.4s.
```

```
SCENARIO C — Deep call graph (O(n) link, realistic)
─────────────────────────────────────────────
pub extern "C" fn update() {           ← 1 export
    physics();                          ← calls into physics system
}
fn physics() { collision(); }          ← transitively reachable
fn collision() { format!("..."); }     ← transitively reachable → linker must resolve format!
...5,000 fns in the call tree...       ← ALL must be linked

Linker sees: 1 export → 5,000 reachable functions → ~1.4s.
```

#### Walkthrough: editing the 5th function in a call tree

A concrete example to make this tangible.  Suppose your crate looks like this:

```
lib.rs                          physics.rs
───────                         ──────────
pub extern "C" fn update() {    
    physics_tick(); ───────────→ fn physics_tick() { collision_detect(); }
                                fn collision_detect() { resolve(); }
                                fn resolve() { apply_force(); }
                                fn apply_force() { clamp(); }
                                fn clamp() { ... }        ← YOU EDIT THIS BODY
```

`clamp()` is 5 calls deep from the single export `update`, and lives in
`physics.rs` — a separate file.

**What happens on rebuild:**

| Step | Action | Time | Notes |
|---|---|---|---|
| 1. Detect change | `physics.rs` timestamp differs | instant | Only the file you saved |
| 2. Recompile | `physics.rs` → `physics.o` | tens of ms | Other `.rs` files are cached, untouched |
| 3. Link | All `.o` files → `game_lib.dll` | **varies** | Depends on what else is reachable/exported |

**The link time depends entirely on the rest of the crate:**

| What else is in the crate? | Link time | Why |
|---|---|---|
| Only `update` + the 5-fn call tree, all others dead private | ~tens of ms | Compiler elminated 4,994 functions; linker sees only 6 |
| 5,000 `pub extern "C"` exports (stress test) | ~1.4s | Linker must process every export + its relocations |
| A genuine 5,000-fn call graph reachable from `update` | ~1.4s | Linker resolves the entire transitive closure |

**Neither the call depth** (5 levels) **nor the file boundary** (`physics.rs` vs
`lib.rs`) matter to the linker.  rustc's incremental compilation handles both
efficiently: only the changed `.rs` file is recompiled.  The linker's cost is
governed by a single number: **how many functions in the crate are ultimately
reachable from any `pub extern "C"` export.**

**The rule: link cost = O(number of functions reachable from any `pub extern "C"` export).**

#### 4. Real example: an ECS game DLL

The stress test with 5,000 `pub extern "C"` exports is artificial.  What does
a **real** hot-reloadable game DLL look like?

```
lib.rs
──────
pub extern "C" fn game_init(api: *const EngineApi) {   ← the ONLY export
    // ...
    engine.register_system("counter",  counter_system);   // takes fn pointer
    engine.register_system("movement", movement_system);  // takes fn pointer
}

fn counter_system(query: Query<&mut FrameCounter>) {    ← plain fn, not exported
    for mut counter in query.iter_mut() {
        counter.count += 1;
        // ...println! on threshold...
    }
}

fn movement_system(query: Query<&mut Transform>) {      ← plain fn, not exported
    for mut transform in query.iter_mut() {
        transform.position[0] += 1.0;
    }
}
```

`counter_system` and `movement_system` are **not** `pub extern "C"` — they're
plain private `fn`.  But because `game_init` passes them as function-pointer
arguments to `register_system`, the compiler emits references to their
addresses.  The linker sees: `game_init` → `counter_system` → must resolve
everything those functions transitively call.

**The good news:** these functions don't bloat the `.edata` export table — that
5,000-export stress-test cost doesn't apply.  The linker only processes what's
genuinely reachable.

**The practical scaling:**

| Game DLL size | Link time | Example |
|---|---|---|
| 1 export, 2–20 system fns + framework boilerplate | ~50–100ms | Small game, rapid iteration |
| 1 export, 50–200 systems with moderate call depth | ~200–500ms | Larger game, still workable |
| 1 export, 500+ systems or deep dependency trees | ~1–2s | Starting to hurt iteration speed |
| 5,000 `pub extern "C"` exports (stress test) | ~1.4s | Artificial — avoid in real code |

**Key takeaway for real game DLLs:** link time scales with your **genuine
reachable codebase**, not with artificial bloat.  Keep `pub extern "C"` only on
the entry points you actually `GetProcAddress`.  Everything else should be
`fn` — the compiler strips dead code, the linker only processes what's used.

#### 5. Why `pub extern "C"` forces the linker's hand

Three attributes combine to make each bloat function un-removable by the linker:

```rust
#[unsafe(no_mangle)]              // "keep my name exactly as written"
pub extern "C" fn bloat_0() -> i32 { // "I am a public C-ABI export"
    format!("...");                   // each format! generates 2-3 relocations
}
```

- **`pub extern "C"`** — marks the function for the DLL's export table
  (`.edata`).  The linker *cannot* remove it because any `GetProcAddress`
  caller could legally request `"bloat_3742"` and expect a valid pointer.

- **`#[no_mangle]`** — prevents rustc from renaming/shrinking the symbol
  during optimization.  Signals "preserve as-is."

- **`format!` calls** — each macro expands to 2–3 relocation entries (calls
  to `format!` in `std`).  5,000 functions × ~3 relocations = ~15,000 fixups
  the linker must resolve.

**In a real codebase**, you'd only apply `pub extern "C"` to the handful of
functions you load dynamically (e.g. `update`, `compute`).  The other 4,999
would be plain `fn` — the compiler strips dead ones and the linker never
sees them.  **The stress test deliberately exports everything to demonstrate
the worst case.**  Link time collapses back to tens of milliseconds the
moment you remove `pub extern "C"` from functions that don't need it.

### Verifying with `--timings`

## How this relates to the example projects

### `classic_full_hot_reload` -- demonstrates the problem

```
Edit lib.rs → cargo build → link ALL 5,000 .o files → LoadLibraryW(new DLL)
                         ↑
                    ~1.4s bottleneck
```

This is the **classic approach**: recompile the entire crate, relink everything,
load the new DLL.  It works great for small projects (~250ms with no bloat) but
degrades linearly with codebase size because the linker must process every
function, changed or not.

The only workaround in this model is to split the codebase into many small
crates (e.g. one DLL per module) so each link step stays small.  But that adds
build-system complexity and doesn't help when the hot function lives in a crate
with many peers.

### `function_live_patching_simple` -- demonstrates the solution

```
Edit one function → rustc --crate-type=cdylib → compile ONLY that function
                                              → LoadLibraryW(tiny DLL)
                                              → swap 1 pointer in jump table
                                              → ~160ms, independent of codebase size
```

This **bypasses the linker entirely** for unchanged code.  Only the edited
function is compiled (via `rustc` invoked directly as a subprocess), producing a
tiny standalone DLL.  The other 4,999 functions stay in the original base DLL
and are never touched.

The key insight: **O(1) reload cost regardless of project size**, because:

- No `cargo build` is ever called during reload
- No linker is ever invoked for unchanged functions
- Only one `HashMap::insert` updates the global jump table

## Summary

| | Classic | Per-function patching |
|---|---|---|
| **Reload cost** | O(n) -- scales with total functions | O(1) -- scales with changed functions |
| **5,000 functions** | ~1.4s | ~160ms |
| **50,000 functions** | ~10-15s (estimated) | ~160ms |
| **Mechanism** | `cargo build` + link all `.o` | `rustc` single function + jump table |
| **Example project** | `classic_full_hot_reload` | `function_live_patching_simple` |

The tradeoff is that per-function patching requires a jump-table indirection on
every hot function call (one `HashMap` lookup), adding a few nanoseconds of
overhead per call.  For the vast majority of game/application code this is
negligible compared to the developer productivity gain of sub-second reloads.

---

## Other approaches to the linking bottleneck

The core problem is: **the linker must process every object file, even though
only one changed.**  Here are the known strategies to work around this, ranked
from "most practical today" to "most experimental."

### 1. Split into many small DLLs (dynamic linking)

Instead of one monolithic 5,000-function DLL, ship 50 DLLs with 100 functions
each.  When you edit a function, only its owning DLL is relinked.

| Pros | Cons |
|---|---|
| Zero runtime overhead for intra-DLL calls | Build system complexity explodes |
| Works with any language, any linker | Cross-DLL calls slower (indirect through IAT) |
| Each DLL links in ~30ms | Hard to decide split boundaries up front |

**Verdict:** The classic C++ game-engine approach.  Unreal Engine does this
(modules as separate DLLs).  Practical but architecturally invasive.

### 2. Faster linker (mold / lld)

Replace the system linker with a faster one.  `mold` (Linux only) can link a
~2 GB binary in under 2 seconds.  `lld` (LLVM's linker, cross-platform) is
typically 2-5x faster than MSVC `link.exe`.

| Pros | Cons |
|---|---|
| Drop-in replacement, no code changes | `mold` is Linux-only (ELF) |
| Still O(n) but with a much smaller constant | On Windows, `lld` support for DLLs is less mature |
| Already used in production (Android, Chrome) | At 50k functions you still wait seconds |

**Verdict:** Easy win on Linux with `mold`.  On Windows, switching to `lld`
(`.cargo/config.toml`: `linker = "rust-lld"`) can cut link times by ~40-60%
but does not change the O(n) scaling.

### 3. MSVC incremental linking (`/INCREMENTAL`)

MSVC's linker has an `/INCREMENTAL` flag that caches the previous link state
and only patches the differences into the existing binary.

| Pros | Cons |
|---|---|
| Native Windows solution | Only for `.exe`, **not for `.dll`** |
| Near-instant for small changes | Fragile -- falls back to full link often |
| No code changes needed | Not exposed through `rustc`/Cargo in a stable way |

**Verdict:** Theoretically perfect but practically unusable for DLL hot-reload
on Rust/Windows.  Even for EXEs the patch file (`.ilk`) grows over time and
must be periodically reset with a full link.

### 4. Per-function compilation + jump table (this repo's approach)

Compile only the changed function via `rustc` directly, load the tiny DLL,
swap one pointer in a global `HashMap`.  Demonstrated in
`function_live_patching_simple`.

| Pros | Cons |
|---|---|
| O(1) reload -- independent of codebase size | Runtime overhead: one `HashMap` lookup per call |
| No build-system changes | Requires opt-in (`HotFn` wrapper) |
| Works on Windows today | Cross-crate inlining is disabled for hot functions |
| ~160ms per function | Cannot change function signatures (only bodies) |

**Verdict:** The best practical solution for Rust on Windows today.  The jump
table indirection adds ~5-10ns per call, invisible for any function doing real
work.  This is what the `subsecond` crate (the inspiration for this repo)
implements in production.

### 5. Object-file swapping (Live++ style)

The commercial tool [Live++](https://liveplusplus.tech/) hot-patches C++
applications by compiling changed `.cpp` to `.obj`, then patching the
in-memory image of the running process: it overwrites function prologues
with `jmp` instructions to the new code.

| Pros | Cons |
|---|---|
| Zero opt-in -- patches any function | Windows-only, closed-source |
| No jump table overhead | Requires deep compiler/ABI integration |
| Sub-second for any codebase size | Fragile with inlined functions / LTO |

**Verdict:** The gold standard for C++ on Windows, but a massive engineering
effort.  Not available for Rust.  The technique is: compile `.obj`, allocate
virtual memory in the target process, write new code, atomically patch a
5-byte `jmp` at the old function's entry point.

### 6. Shared library interposition (`LD_PRELOAD` / `DYLD_INSERT_LIBRARIES`)

On Linux/macOS, you can override functions in a running process by preloading
a shared library that exports the same symbols.  The dynamic linker resolves
symbols from the preloaded library first.

| Pros | Cons |
|---|---|
| No recompilation of the host binary | Linux/macOS only, not Windows |
| OS-level mechanism, very robust | Overrides entire symbol, not individual calls |
| Zero code changes in the host | Cannot change function signatures |

**Verdict:** Elegant on Unix but not portable.  Windows has no equivalent
(its DLL search order can be manipulated but not per-symbol).  Also requires
the host to be a dynamically-linked executable, not static.

### 7. Dynamic code generation (JIT / Cranelift / LLVM ORC)

Instead of compiling functions ahead-of-time, compile them at runtime using a
JIT compiler embedded in the host process.  Cranelift and LLVM ORC can compile
Rust-like code to machine code in-memory in milliseconds.

| Pros | Cons |
|---|---|
| Sub-millisecond "reloads" | Requires embedding a full compiler in the host |
| No filesystem round-trip | Generated code quality lower than AOT rustc |
| Can change anything at runtime | Massive engineering effort |
| No linker at all | Debugging JIT'd code is hard |

**Verdict:** Overkill for hot-reload but the ultimate solution.  This is how
JavaScript engines (V8), LuaJIT, and .NET (RyuJIT) work.  For a Rust game
engine, embedding Cranelift could give <1ms function reloads.

### 8. Process isolation (microservices model)

Run each module as a separate process communicating via IPC (pipes, shared
memory, or a network protocol).  Restarting one process "reloads" its code
with no link step at all.

| Pros | Cons |
|---|---|
| Trivial to implement | IPC overhead (serialization, context switches) |
| Works with any language | Not suitable for per-frame game logic |
| Crash isolation for free | Complex state synchronization |

**Verdict:** Good for tools/editors (e.g. VS Code's extension host process),
terrible for hot-reloading game tick functions called 60 times per second.

### 9. WASM / plugin sandboxing

Compile "game code" to WebAssembly, run it in a WASM runtime (wasmtime,
wasmer).  Reloading = recompiling one `.wasm` module and instantiating it.
WASM compilation is extremely fast (no linker; the WASM format is designed for
streaming compilation).

| Pros | Cons |
|---|---|
| Near-instant reloads (~1ms for small modules) | WASM sandbox overhead |
| Cross-platform, secure by default | Cannot call host functions directly without bindings |
| Growing ecosystem (wasmtime, wasmer, wazero) | 10-50% performance penalty vs native |

**Verdict:** Promising future direction.  Several game engines (Bevy, Ambient)
are exploring WASM for scripting.  The sandbox is a feature for modding, not a
bug.  Compilation is so fast that the linking problem simply doesn't exist.

### 10. Managed runtime hosting (C# / Lua / Python embedded in a native host)

A fundamentally different strategy: **don't write hot-reloadable code in Rust
at all.**  Instead, embed a managed runtime (like .NET's CLR, LuaJIT, or
CPython) into the Rust host process.  Game logic lives in the managed language,
where hot reload is either built into the runtime or trivial to implement by
unloading and reloading an assembly/script.

This repo contains a working example in `hot_reloading/`: a Rust engine hosts
the .NET runtime, loads a C# game DLL through `game_cs_loader`, and reloads
it at runtime by disposing and recreating a collectible `AssemblyLoadContext`.

| Pros | Cons |
|---|---|
| Hot reload is **built into the runtime**, not bolted on | Requires embedding a heavy runtime (~50 MB for .NET) |
| No linker bottleneck -- .NET JIT compiles methods lazily | Two-language codebase (Rust + C#) |
| `AssemblyLoadContext.Unload()` is O(1) regardless of size | FFI boundary has serialization cost |
| C# has excellent IDE support, debugger, profiler | Managed heap is wiped on reload (state resets) |
| Mature, production-tested (Unity, Godot, Unreal) | Managed code is slower than native Rust |

**How it works (the `hot_reloading/` approach):**

```
Rust engine (native)                    .NET CLR (managed)
     │                                       │
     │  1. LoadLibraryW("game_cs_loader")    │
     │─────────────────────────────────────→ │
     │                                       │
     │  2. loader.Init(hostcall_table)       │
     │─────────────────────────────────────→ │  ┌─────────────────┐
     │                                       │  │ AssemblyLoadCtx │
     │                                       │  │  game_cs.dll    │
     │  3. loader.Update() every frame       │  │  Bird, Pipes... │
     │─────────────────────────────────────→ │  └─────────────────┘
     │                          ←───────────│
     │  4. On file change:                   │
     │     loader.Reload("game_cs.dll")      │
     │─────────────────────────────────────→ │  ┌─────────────────┐
     │                                       │  │ NEW AssemblyLCtx│
     │                                       │  │  game_cs.dll v2 │
     │                                       │  └─────────────────┘
     │                                       │  (old context GC'd)
```

The key insight: **the linking problem is someone else's problem.**  The .NET
runtime handles JIT compilation, assembly loading/unloading, and GC.  The
Rust side never touches a linker during reload — it just tells the CLR "load
this new DLL" and the CLR does the rest in milliseconds.

Compare this to the pure-Rust approaches:

| | Pure Rust (jump table) | Rust + C# (managed host) |
|---|---|---|
| **Who links?** | You (`rustc` per function or `cargo build`) | .NET CLR (automatic) |
| **Reload mechanism** | Swap pointer in HashMap | `AssemblyLoadContext.Unload()` + reload |
| **State persistence** | Yes (same heap, same stack) | No (managed heap destroyed on unload) |
| **Language for game logic** | Rust (same as engine) | C# (different from engine) |
| **Performance** | Native | JIT'd managed (~2-3x slower, negligible for gameplay) |
| **Tooling** | cargo, rust-analyzer | Visual Studio / Rider, full .NET debugger |
| **Effort to add hot reload** | ~500 lines of jump-table + `rustc` FFI | ~100 lines of `AssemblyLoadContext` |

**Verdict:** This is the pragmatic choice if you're building a game engine or
application where gameplay/scripting logic is the part that changes frequently.
Unity, Godot, and Unreal (with plugins) all use this pattern.  The "linking
problem" becomes irrelevant because managed runtimes were designed from day one
to load and unload code dynamically — they never had a static linker to begin
with.  The cost is carrying a runtime (~50 MB) and accepting managed
performance for the script layer.

### Comparison matrix

| Approach | Reload speed | Runtime overhead | Platform | Maturity |
|---|---|---|---|---|
| Many small DLLs | O(small-n) | None (intra), IAT (inter) | All | Production |
| Faster linker (mold/lld) | O(n) with smaller constant | None | Linux/Win | Production |
| MSVC `/INCREMENTAL` | Near-instant | None | Windows EXE only | Production |
| **Jump table (this repo)** | **O(1) ~160ms** | **~5ns HashMap lookup** | **All** | **Demo** |
| Object-file swapping | O(1) <100ms | None (after `jmp`) | Windows | Commercial |
| `LD_PRELOAD` | Instant | None | Linux/macOS | Production |
| JIT (Cranelift/ORC) | O(1) <1ms | Code quality | All | Experimental |
| Process isolation | Restart time | IPC latency | All | Production |
| WASM sandboxing | O(1) ~1ms | Sandbox + FFI | All | Emerging |
| **Managed runtime host** | **O(1) ~10ms** | **JIT + GC (~2-3x slower)** | **All** | **Production** |

### What should you actually use?

- **Windows + Rust today — all-Rust codebase:** Jump-table per-function patching (`function_live_patching_simple` approach)
- **Windows + Rust today — engine + scripting:** Embed .NET CLR, write game logic in C# (`hot_reloading/` approach). Solves the linking problem by making it the CLR's responsibility.
- **Linux + Rust today:** `LD_PRELOAD` interposition, or jump-table + `mold` linker
- **C++ on Windows:** Live++ (commercial) or many-small-DLLs (Unreal model)
- **if you are open to a managed scripting layer:** Embedding .NET/Lua/Python is the
  most battle-tested approach — it eliminates the linking problem entirely by
  delegating it to a runtime designed for dynamic code loading.
- **5-year bet:** WASM sandboxing will become the dominant approach as WASM
  runtimes mature and close the performance gap, potentially replacing both
  native patching and managed runtimes for scripting.
