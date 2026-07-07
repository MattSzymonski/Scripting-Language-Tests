# How Live-Patching Works — From Zero

This document explains, **assuming you know nothing about it**, how a Rust game
engine can reload game logic **while the game is running** — without closing the
window, without restarting the process, and without losing a single frame.

We build the idea up in two halves. First, **what this repository actually
built**: a concrete, working prototype you can read the source of. Second, **how
real function-level patching works in general**, and how it's actually done in
production tools for both native languages (Rust, C++) and managed ones (C#) —
because the prototype here takes a deliberate shortcut, and it's worth knowing
exactly what that shortcut is and what the harder, real version looks like.

By the end you should understand:

- the difference between "reload the whole module" and "patch individual
  functions in place," and where this prototype sits on that spectrum,
- what a **function pointer** is and why it's the key mechanism,
- what a **hot function table** is and how the engine calls through it,
- how we **atomically swap** function addresses so the engine never sees garbage,
- how the **orchestrator** watches files, rebuilds code, and injects new
  functions — all while the engine keeps running,
- what we actually implemented vs. what we simulated,
- **in general**, how a real patcher identifies exactly what changed
  (**partial compilation**), how it resolves addresses in a program that's
  already running (**address injection**), and how it gets new code to actually
  run in place of old code,
- why this is harder for Rust specifically than for a managed, JIT-compiled
  language like C# — and what real tools (Live++, subsecond) do about it.

> Throughout, file references point at the real code:
> [`live_engine/src/hot_table.rs`](../live_engine/src/hot_table.rs),
> [`live_engine/src/hot_memory.rs`](../live_engine/src/hot_memory.rs),
> [`live_engine/src/engine.rs`](../live_engine/src/engine.rs),
> [`live_engine/src/api.rs`](../live_engine/src/api.rs),
> [`orchestrator/src/main.rs`](../orchestrator/src/main.rs),
> [`game_hot/src/lib.rs`](../game_hot/src/lib.rs).

---

## Table of contents

**Part 1 — What this repository built**

1. [The 30-second picture](#1-the-30-second-picture)
2. [Why live-patching? Three tiers of "not restarting"](#2-why-live-patching-three-tiers-of-not-restarting)
3. [A quick recap: what is a function pointer?](#3-a-quick-recap-what-is-a-function-pointer)
4. [The hot function table: one level of indirection](#4-the-hot-function-table-one-level-of-indirection)
5. [Atomic swaps: changing pointers without crashing](#5-atomic-swaps-changing-pointers-without-crashing)
6. [The ABI contract for hot functions](#6-the-abi-contract-for-hot-functions)
7. [Opaque handles: why the game never touches engine internals](#7-opaque-handles-why-the-game-never-touches-engine-internals)
8. [The orchestrator: the controller that makes it all happen](#8-the-orchestrator-the-controller-that-makes-it-all-happen)
9. [A single patch cycle, step by step](#9-a-single-patch-cycle-step-by-step)
10. [What we built vs. what we simulated](#10-what-we-built-vs-what-we-simulated)

**Part 2 — How real function-level patching works**

11. [The general mechanism: three problems every real patcher solves](#11-the-general-mechanism-three-problems-every-real-patcher-solves)
12. [Why this is extra hard for Rust](#12-why-this-is-extra-hard-for-rust)
13. [Why managed languages mostly get this for free](#13-why-managed-languages-mostly-get-this-for-free)
14. [Real-world implementations](#14-real-world-implementations)

**Part 3 — Reference**

15. [Safety: what keeps this from crashing](#15-safety-what-keeps-this-from-crashing)
16. [Glossary](#16-glossary)
17. [Further reading](#17-further-reading)

---

# Part 1 — What this repository built

## 1. The 30-second picture

Our program has **one** running process. Inside it, three pieces work together:

```
┌────────────────────── one process ──────────────────────────────┐
│                                                                  │
│  ┌─────────────────────┐     ┌───────────────────────────────┐  │
│  │ Orchestrator         │     │ Engine                        │  │
│  │                      │     │                               │  │
│  │ • watches source     │     │ • every frame, calls through  │  │
│  │ • rebuilds on change │     │   the hot function table      │  │
│  │ • loads new .dll     │     │                               │  │
│  │ • patches the table  │     │   let f = table.update();     │  │
│  └─────────┬────────────┘     │   f(api, engine, world, dt);  │  │
│            │ atomic swap       └───────────┬───────────────────┘  │
│            └─────────────► HotFnTable ◄────┘                      │
│                            ┌──────┬──────┐                        │
│                            │init  │update│ render │ world_size │  │
│                            └──────┴──────┴────────┴────────────┘  │
│                                      │                            │
│                                      │ calls through ABI          │
│                                      ▼                            │
│                           ┌──────────────────┐                    │
│                           │ game_hot.dll     │                    │
│                           │                  │                    │
│                           │ game_update()    │ ◄── you edit this  │
│                           │ game_render()    │     while running  │
│                           └──────────────────┘                    │
└──────────────────────────────────────────────────────────────────┘
```

- The **orchestrator** is the boss. It decides *which* version of each game
  function is currently active.
- The **hot function table** is the switchboard. Every call from the engine to
  the game passes through it.
- The **engine** doesn't know or care about patching. It just reads the table
  and calls whatever function it finds there.

When you edit the game code and save, the orchestrator rebuilds the `.dll`,
loads the new functions, and swaps their addresses into the table. The engine
picks them up on the next frame. That's the whole idea — and, as you'll see in
§10, it's also the whole *limit* of what this prototype does. Part 2 explains
what a system would need to add to go further.

---

## 2. Why live-patching? Three tiers of "not restarting"

### 2.1 The traditional way: edit → build → restart

In a normal game development workflow:

1. You edit some code.
2. You stop the game.
3. You rebuild.
4. You restart the game.
5. You navigate back to where you were to see the change.

Step 5 is the killer. If you're tuning the feel of a jump — tweaking gravity,
tweaking flap velocity — the restart loop might take 10–30 seconds per
iteration. That kills creative flow.

### 2.2 Tier 1: reload the whole module

A common step up is **module hot-reloading**: compile the game as a dynamic
library (`.dll` / `.so`), and when it changes, load a new copy and swap it in.
This is what many game engines (Unity, Unreal with Live Coding) effectively do,
and it's what **this prototype does**.

Problems with whole-module reload:
- You lose all **game state** — the bird position, the score, the pipe layout —
  unless you deliberately keep it outside the reloaded module (this prototype's
  `WorldHandle`, §7, does exactly that).
- The reload itself takes a **noticeable pause** while the new library is
  compiled and loaded — around a second, dominated by `cargo build` (§9).
- You rebuild **everything in the module**, even functions that didn't change.

### 2.3 Tier 2: true function-level patching

What you'd really want:

> Change *only* the functions that actually changed, inject just their machine
> code into the running process, and keep everything else — state, window,
> loop, and every other function — completely untouched.

This is a fundamentally harder problem, covered in full in Part 2 (§11–§14). It
requires knowing precisely what changed at the function level, not the file
level, and requires getting new machine code into a process that's already
running and already has other code calling into the parts you're replacing.

### 2.4 Where this prototype sits

This prototype implements **Tier 1** convincingly: independent, atomically
swappable function slots (§4), a stable ABI (§6), and state that survives a
reload (§7). It does **not** implement Tier 2 — every reload rebuilds the whole
`game_hot` crate, not just the changed function. §10 gives the full honest
accounting, and §11–§14 explain what Tier 2 actually requires and who has
actually built it.

---

## 3. A quick recap: what is a function pointer?

(A fuller explanation is in the [C# scripting
doc](./how-csharp-scripting-works.md#8-function-pointers-explained). Here's the
short version.)

A CPU runs **machine code** — tiny numeric instructions stored in memory. A
function is just a block of machine code sitting at some memory address. A
**function pointer** is simply **that address** — a number that says "the code
for this function starts at location X."

If you have that address and you know the function's **signature** (what
arguments it expects and what it returns), you can call it — even if you have
no idea what language it was written in. You just hand the CPU the address and
the arguments and say "go."

This is the entire mechanism behind live-patching, at any tier:

- The engine stores function pointers in a table.
- To call the game, it reads the current pointer from the table and calls it.
- To *patch* the game, something writes a **new** pointer into the table.
- The engine doesn't notice — it just calls whatever address it finds there.

---

## 4. The hot function table: one level of indirection

### 4.1 The problem with calling directly

If the engine's source code said:

```rust
// ❌ This can't be patched at runtime
game_hot::game_update(&api, engine, world, dt);
```

then the address of `game_update` would be baked into the compiled `.exe` at
build time. Changing it at runtime would require rewriting the `.exe` on disk.

### 4.2 The solution: call through a table

Instead, the engine stores function addresses in a **table** — a struct full of
function pointers that lives in normal memory and can be modified at any time:

```rust
// ✅ These can be changed at any time
let update_fn = hot_table.read_update();   // reads current address
let render_fn = hot_table.read_render();   // reads current address

update_fn(&api, engine, world, dt);
render_fn(&api, engine, world);
```

The table itself, from
[`live_engine/src/hot_table.rs`](../live_engine/src/hot_table.rs):

```rust
pub struct HotFnTable {
    pub init:       AtomicPtr<()>,  // called once at startup
    pub update:     AtomicPtr<()>,  // called every frame
    pub render:     AtomicPtr<()>,  // called every frame
    pub world_size: AtomicPtr<()>,  // returns needed memory size
}
```

Each field is an `AtomicPtr` — a pointer that can be safely read and written
from multiple threads simultaneously (more on this in §5).

### 4.3 Why "hot"?

The table is "hot" because it's **actively read every frame** by the engine
while being **actively written** by the orchestrator during patches. It's the
busiest, most important 32 bytes in the program.

### 4.4 The critical invariant

> **The engine never calls game code directly. Every call goes through the hot
> table.** This single rule is what makes live-patching possible.

If the engine ever "short-circuited" the table (called a game function by its
compiled name), that call would be **unpatchable**. The table is the only
channel. This is the same invariant every real hot-patching system relies on —
§14.2 shows how `subsecond` enforces it per call site instead of engine-wide.

---

## 5. Atomic swaps: changing pointers without crashing

### 5.1 The race condition

Consider what happens if the orchestrator writes a new function address into the
table *while* the engine is reading it on another thread:

```
Time ──────────────────────────────────────────►

Engine thread:     read pointer...  [use it to call function]
Orchestrator thread:    [write new pointer]
                              ↑
                        What does the engine see?
```

If the write is not **atomic** (done in a single CPU instruction), the engine
might read a half-written, garbage pointer — part old address, part new one. The
CPU would then jump to a meaningless address and crash.

### 5.2 The solution: `AtomicPtr`

Rust's `AtomicPtr<T>` guarantees that every read and write is a single,
indivisible operation. It also provides two **memory orderings** that together
form a contract:

| Side                    | Operation                | Ordering | Meaning                                               |
| ----------------------- | ------------------------ | -------- | ----------------------------------------------------- |
| **Orchestrator** writes | `swap(new_ptr, Release)` | Release  | "Everything I did before this write is now visible."  |
| **Engine** reads        | `load(Acquire)`          | Acquire  | "I can see everything the orchestrator made visible." |

When the engine uses `Acquire` and the orchestrator uses `Release`, the hardware
guarantees that the engine will see a **fully valid** pointer — either the old
one or the new one, never a torn mix of the two.

In code (orchestrator side):

```rust
pub fn patch_update(&self, new_fn: GameUpdateFn) -> GameUpdateFn {
    // Atomically write the new address, return the old one.
    let old = self.update.swap(new_fn as *mut (), Ordering::Release);
    unsafe { std::mem::transmute(old) }
}
```

In code (engine side):

```rust
pub fn read_update(&self) -> GameUpdateFn {
    // Atomically read the current address.
    let ptr = self.update.load(Ordering::Acquire);
    unsafe { std::mem::transmute(ptr) }
}
```

### 5.3 What happens to in-flight calls?

When the orchestrator swaps a pointer, a call to the **old** function might
still be executing on the engine thread. That's fine — the old function's code
is still in memory because we keep the old `.dll` loaded. Once that call
returns, the engine will read the **new** pointer on the next frame, and
subsequent calls will go to the new function.

This is called **graceful transition**: old calls finish normally, new calls use
the new code. No interruption, no crash. (Compare this to `subsecond`'s more
aggressive approach in §14.2, which can unwind and retry a call that's already
mid-execution.)

---

## 6. The ABI contract for hot functions

### 6.1 Why `extern "C"` is required

When Rust compiles a function normally, it **mangles** the name — encodes the
module path, generic parameters, and types into a long, unique string. This is
great for preventing name collisions, but it makes the symbol impossible to
predict or look up by name at runtime.

`extern "C"` disables mangling. The symbol name in the compiled `.dll` is
exactly what you wrote:

```rust
#[no_mangle]              // "don't mangle this name"
pub extern "C" fn game_update(api, engine, world, dt) { ... }
//          ^^^^^^^^       ^^^^^^^^^^^
//          C ABI          exported as exactly "game_update"
```

This lets the orchestrator find it:

```rust
let update_fn: Symbol<GameUpdateFn> = lib.get(b"game_update")?;
//                                               ^^^^^^^^^^^^^^
//                                               matches the source exactly
```

### 6.2 What can cross the boundary

The same FFI rules apply here as in the [C# scripting doc](./how-csharp-scripting-works.md#11-marshalling-what-data-can-cross-the-boundary).
Only **plain, predictable data** can cross:

| ✅ Safe to pass                           | ❌ Cannot pass directly |
| ---------------------------------------- | ---------------------- |
| `f32`, `f64`, `i32`, `u32`               | `String`, `Vec<T>`     |
| `*const T`, `*mut T` (raw pointers)      | `Box<dyn Trait>`       |
| `#[repr(C)]` structs of the above        | Rust enums with data   |
| Function pointers (`extern "C" fn(...)`) | Closures, generics     |

Our hot functions follow these rules strictly. All parameters are raw pointers
or primitive numbers.

### 6.3 The function signatures

From [`live_engine/src/api.rs`](../live_engine/src/api.rs):

```rust
pub type GameUpdateFn = extern "C" fn(
    api:    *const EngineAPI,   // engine services (draw, input, time)
    engine: EngineHandle,       // opaque engine handle
    world:  WorldHandle,        // opaque game state handle
    dt:     f32,                // seconds since last frame
);

pub type GameRenderFn = extern "C" fn(
    api:    *const EngineAPI,
    engine: EngineHandle,
    world:  WorldHandle,
);

pub type GameWorldSizeFn = extern "C" fn() -> usize;
```

The signatures never change. If you need new parameters, you add a new function
slot to the table — you don't change existing signatures. That would break the
ABI contract.

---

## 7. Opaque handles: why the game never touches engine internals

### 7.1 The problem with sharing types

If the game code directly included the engine's `EngineState` struct, then
changing `EngineState` (adding a field, reordering, changing a type) would
break all existing compiled game `.dll` files. The old code would read the wrong
offsets and crash.

### 7.2 The solution: opaque handles

The engine passes two opaque pointers to the game:

| Handle         | Type          | What it points to             | Who owns it                         |
| -------------- | ------------- | ------------------------------ | ------------------------------------ |
| `EngineHandle` | `*mut c_void` | The engine's private state     | Engine                              |
| `WorldHandle`  | `*mut c_void` | A buffer for game world state  | Engine allocates, game reads/writes |

The game **never dereferences the engine handle directly**. It only passes it
back to the engine through `EngineAPI` calls:

```rust
// Game code — asking the engine if a key is pressed
let pressed = (api.is_key_pressed)(engine, KEY_SPACE);
//                                  ^^^^^^
//                                  opaque handle, passed back to engine
```

The **world handle** is different: the game *does* cast it to its own
`GameState` struct and reads/writes it. But the engine never touches that struct
— it just allocates `world_size()` bytes and hands over the pointer. This
separation means:

- The engine's internals can change without breaking game `.dll` files.
- The game's state layout can change between patches (with care — see §12.4).
- Game state **survives a reload**, because it lives outside the module that
  gets swapped — the same idea Tier-2 systems rely on (§14.2's "framework-level
  re-instancing" for struct changes is a more automated version of this).

---

## 8. The orchestrator: the controller that makes it all happen

The orchestrator is a **separate binary** (`orchestrator/`) that has one job:
keep the game code fresh. It has four steps running in a loop:

### 8.1 The build step

```rust
fn build_game_lib(release: bool) -> Result<PathBuf, String> {
    Command::new("cargo")
        .args(["build", "-p", "game_hot"])
        .output()?;
    // Returns path to target/debug/game_hot.dll
}
```

This runs `cargo build` on the `game_hot` crate, producing a `.dll`. We do this
inside the running process by spawning `cargo` as a child process. Note: this
rebuilds the **entire crate**, not just the function you changed — that's the
Tier-1-vs-Tier-2 distinction from §2 in concrete terms.

### 8.2 The load step

```rust
fn load_game(lib_path: &Path) -> Result<LoadedGame, ...> {
    let lib = unsafe { Library::new(&versioned_path)? };
    let init:   Symbol<GameInitFn>       = lib.get(b"game_init")?;
    let update: Symbol<GameUpdateFn>     = lib.get(b"game_update")?;
    let render: Symbol<GameRenderFn>     = lib.get(b"game_render")?;
    let wsize:  Symbol<GameWorldSizeFn>  = lib.get(b"game_world_size")?;
    // ...
}
```

`Library::new()` tells the operating system to load the `.dll` into the
process's address space. `lib.get(b"name")` finds a function by its exported
name and returns its address — a function pointer.

### 8.3 The watch step

```rust
let mut watcher = notify::recommended_watcher(move |event| {
    if event.paths.iter().any(|p| p.extension() == Some("rs")) {
        reload_tx.send(()).unwrap();  // trigger rebuild
    }
})?;
watcher.watch(&game_src_dir, RecursiveMode::NonRecursive)?;
```

The `notify` crate hooks into the operating system's file-change notification
system. When you save `game_hot/src/lib.rs`, the OS tells notify, notify tells
our channel, and the reload loop kicks in.

### 8.4 The patch step

```rust
unsafe fn patch_game(hot_table: &HotFnTable, game: &LoadedGame) {
    hot_table.patch_init(game.init);
    hot_table.patch_update(game.update);
    hot_table.patch_render(game.render);
    hot_table.patch_world_size(game.world_size);
}
```

Four atomic swaps. Each one independently replaces a function pointer. The
calling convention is `Release` on the write, `Acquire` on the read (§5.2).

### 8.5 Why versioned file names?

Windows locks `.dll` files that are loaded. To load a new version without
unloading the old one, we **copy** each build to a versioned name:

```rust
let versioned = format!("game_hot_v{}.dll", patch_counter.fetch_add(1));
std::fs::copy(&built_path, &versioned)?;
```

This means `game_hot_v1.dll`, `game_hot_v2.dll`, etc. all coexist in the
`target/debug/` directory. The old ones are kept loaded so in-flight calls
complete safely; new ones are loaded fresh.

---

## 9. A single patch cycle, step by step

Let's follow one complete cycle from "you save the file" to "the change appears
on screen."

**Step 0 — The game is running.** The engine loop is spinning at ~60 FPS,
calling through the hot table every frame. The orchestrator's file watcher
thread is sleeping, waiting for the OS to tell it something changed.

**Step 1 — You edit and save.** You change the gravity constant in
`game_hot/src/lib.rs` from `800.0` to `300.0`. You press Ctrl+S.

**Step 2 — The OS notifies.** The operating system detects the file
modification and wakes up the `notify` watcher. The watcher checks that the
changed file has a `.rs` extension and sends a signal through a channel to the
reload thread.

**Step 3 — Debounce.** The reload thread receives the signal, then waits 300ms
to see if more events arrive (editors often write a file in multiple bursts).
After the cooldown, it decides: "ok, rebuild now."

**Step 4 — Rebuild.** The orchestrator spawns `cargo build -p game_hot`. Rust
compiles the changed `lib.rs` into a new `game_hot.dll`. The old `.dll` is
still loaded and fully functional — the build happens to a different file.

**Step 5 — Copy to versioned name.** The orchestrator copies the fresh `.dll`
to `game_hot_v4.dll` (if this is the 4th patch). This avoids Windows
file-locking issues.

**Step 6 — Load the new library.**

```rust
let lib = Library::new("game_hot_v4.dll")?;
let new_update = lib.get::<GameUpdateFn>(b"game_update")?;
```

The OS maps the new `.dll` into the process. `libloading` resolves the symbol
`game_update` to its address. This address points to the freshly-compiled
machine code for the new version.

**Step 7 — Atomic swap.**

```rust
hot_table.patch_update(*new_update);  // Release ordering
```

The `AtomicPtr` in the hot table now points to the new `game_update`.

**Step 8 — The engine picks it up.** On the **very next frame**, the engine
reads the hot table:

```rust
let update_fn = hot_table.read_update();  // Acquire ordering
update_fn(&api, engine, world, dt);       // calls the NEW function
```

The new gravity value (`300.0`) takes effect immediately.

**Step 9 — Old library stays.** The orchestrator pushes the new `Library` into
an `old_libraries: Vec<Library>` list. The old `game_hot_v3.dll` stays loaded.
If a call to the old `game_update` is still executing on another thread, it
completes safely using the old code.

**Total time from save to effect: ~1–2 seconds** (most of which is `cargo
build`). One frame later, the change is visible. The game never stopped.

---

## 10. What we built vs. what we simulated

This is an honest accounting of where the prototype is real and where it
approximates the ideal described in §2.3.

### 10.1 What's real

| Feature                               | Implementation                                                     |
| -------------------------------------- | -------------------------------------------------------------------- |
| **Hot function table**                | `HotFnTable` with independent `AtomicPtr` slots                    |
| **Atomic swapping**                   | `swap(Release)` / `load(Acquire)` — real hardware guarantees       |
| **Independent per-function patching** | `patch_update()` doesn't touch `patch_render()`                    |
| **Old code retention**                | Old `Library` values kept in a `Vec`                               |
| **File watching**                     | `notify` crate with OS-level file events                           |
| **Automatic rebuild**                 | `cargo build` spawned as child process                             |
| **Executable memory allocator**       | `VirtualAlloc(PAGE_EXECUTE_READWRITE)` — real OS call, see §11.3   |
| **ABI-stable function signatures**    | `extern "C"`, `#[no_mangle]`, `#[repr(C)]`                         |
| **Opaque handle separation**          | `EngineHandle` vs `WorldHandle` — game never touches engine memory |
| **Graceful transition**               | Old calls finish, new calls use new code — no interruption         |

### 10.2 What's simulated

| Feature                                         | What we do instead                                                 | Why                                                                                                                                                                                                                         |
| ------------------------------------------------- | ---------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Function-level recompilation**                | Rebuild the entire `game_hot` crate, not just the changed function     | We don't diff object files or drive an incremental linker (§11.2 explains what that would take; §14.2 shows a real implementation, `subsecond`).                        |
| **Copying machine code into engine RWX memory** | Load the whole `.dll` via the OS loader                                | The OS loader handles **relocation** — adjusting absolute addresses inside the machine code to match where the `.dll` actually landed in memory. §11.3 explains what doing this ourselves, against a *live* process, would require. |
| **Fine-grained dependency tracking**            | Rebuild if *any* `.rs` file in `game_hot/` changes                     | Rust's module system means changing one file can affect others. Tracking exact dependencies would need compiler-level integration (§11.2).                                |

The `ExecutableMemory` allocator in
[`live_engine/src/hot_memory.rs`](../live_engine/src/hot_memory.rs) is real
infrastructure — it really calls `VirtualAlloc` with execute permissions — but
nothing ever copies function bytes into it or jumps into it. It's the one
visible fragment of the Tier-2 system Part 2 describes, left unconnected.

---

# Part 2 — How real function-level patching works

Everything above was specific to this repository's Tier-1 prototype. This part
zooms out: given that Tier 2 (true function-level patching) is possible — and
it is, provably, since tools ship doing it — what does it actually take, in
general, for *any* language? And why does the answer differ so much between
Rust and a language like C#?

## 11. The general mechanism: three problems every real patcher solves

Independent of language, a system that patches individual functions in a
**running** process has to solve three problems. This prototype sidesteps all
three by reloading the whole module instead; §14 shows tools that solve them
for real.

### 11.1 Overview

| Problem                                          | What it means                                                                 | This prototype's shortcut                          |
| --------------------------------------------------| -------------------------------------------------------------------------------| ---------------------------------------------------|
| **1. What actually changed?**                    | Identify the exact function(s) whose compiled code differs from before        | Don't ask — recompile the entire crate every time  |
| **2. How do old and new code find each other?**   | Resolve addresses between code that already exists in memory and code that was just compiled somewhere else | Don't ask — the OS's normal loader relocates the whole new `.dll` for us |
| **3. How does new code actually start running?**  | Get the CPU to execute the new bytes instead of (or via) the old ones          | Load the new `.dll` and atomically swap a pointer (§4, §5) |

The rest of this section explains each problem in general terms. §12 and §14
show how Rust-specific compilation quirks make problem 1 harder, and how real
tools (subsecond, Live++) solve all three.

### 11.2 Problem 1 — partial compilation: knowing exactly what changed

A normal build treats "did anything change?" at the level of *whole files* or
*whole crates* — that's why this prototype's orchestrator rebuilds all of
`game_hot` for a one-line edit (§8.1). A true patcher needs a much finer
answer: **which individual functions' compiled bytes are actually different?**

Two pieces make this possible in principle:

1. **Incremental compilation.** Modern compilers (including `rustc`) already
   split a crate into smaller units ("codegen units") and only recompile the
   ones whose source changed, reusing cached object files for the rest. This
   narrows things down from "the whole crate" to "the handful of translation
   units that touched your edit" — but a single unit can still contain many
   functions, most of which didn't actually change.
2. **Object-file diffing.** To get down to individual functions, something has
   to compare the *freshly compiled* object code against the *previous*
   build's object code, symbol by symbol, and report exactly which function
   bodies differ. This is not something `cargo build` does for you — it's
   custom tooling built on top of (or instead of) the normal link step. §14.2
   walks through a real implementation of this (`subsecond`'s `ThinLink`).

Once you know precisely which functions changed, you only need to compile and
ship *those* — which is what makes real patch cycles fast (milliseconds
instead of the ~1 second `cargo build -p game_hot` costs here).

### 11.3 Problem 2 — address injection: resolving addresses against a live process

Freshly compiled code for a changed function doesn't exist in a vacuum — it
calls other functions, reads global statics, references string constants. All
of those live at specific memory addresses. Normally, a linker resolves these
references once, at build time, by laying out the whole binary and computing
every address (a process called **relocation**). A patcher doesn't get that
luxury: the "rest of the program" already exists, already running, at
addresses that were decided **the last time the process started** — not now,
while you're compiling a patch.

Two extra wrinkles make this harder than it sounds:

- **ASLR (Address Space Layout Randomization).** For security, operating
  systems load a program at a different, randomized base address every time
  it runs. The addresses baked into last night's build are not the addresses
  the program is actually using right now.
- **The patch is a separate compilation.** The new function's machine code was
  produced independently of the running process; nothing about it inherently
  knows where anything in that process actually lives.

A workable trick — used in practice (§14.2) — exploits the fact that ASLR
applies **one shift to the whole binary**, not a different random shift per
symbol: find one symbol whose *build-time* address you know (e.g. `main`), ask
the *running* process where that symbol actually ended up, and the difference
between the two is a single offset you can apply to every other address the
new code needs. This avoids re-implementing a full linker's relocation logic —
you're solving one equation, not thousands.

This prototype never faces this problem, because it never tries to link new
code against the live process — the OS's normal `.dll` loader relocates the
*entire* new library as a unit, exactly as if the process were starting fresh.

### 11.4 Problem 3 — getting new code to actually run

Once you have new machine code and know where everything it needs actually
lives, you still need the CPU to run it *instead of* the old version. There are
two fundamentally different families of solution:

**Family A — indirection (safe, what this prototype and `subsecond` do).**
Every call to a patchable function already goes through some kind of table —
a hand-written `HotFnTable` (§4) or an automatically generated jump table
(§14.2). Patching means writing one new pointer into that table. The old
function's machine code is left completely alone in memory; it just stops
being referenced. This requires **discipline at every call site** (§4.4's
invariant) but never touches already-executing code, so it's memory-safe by
construction.

**Family B — binary patching (unsafe, what Live++ does, §14.1).** The old
function's own machine code is overwritten in place: its first few
instructions are replaced with a jump ("**trampoline**") to the new code's
address. Every existing caller — even ones with the old address hard-coded,
no indirection required — ends up at the new code, because the *old* address
now immediately redirects. This needs no cooperation from call sites, but it
requires care that no thread is executing exactly those bytes at the moment
you overwrite them, and on architectures with separate instruction/data caches
(notably ARM), an explicit **instruction cache flush** so the CPU doesn't keep
running stale cached instructions after the bytes underneath have changed.

Either way, the new code has to live somewhere the CPU is *allowed* to execute
from — memory marked with the Execute permission bit, not just Read/Write
(§6.2's `RWX` explanation in the old version of this doc covered this; the
short version: `VirtualAlloc(PAGE_EXECUTE_READWRITE)` on Windows, `mmap` with
`PROT_EXEC` on Unix).

> **Summary:** partial compilation (§11.2) tells you *what* to patch. Address
> injection (§11.3) tells you *what addresses the patch needs to use*.
> Indirection or binary patching (§11.4) is *how the CPU ends up running it*.
> A production system needs all three; this prototype needs none of them,
> because it replaces the whole module instead.

---

## 12. Why this is extra hard for Rust

Section 11 was general. These problems are hard for *any* ahead-of-time
compiled, runtime-less language — but Rust adds a few of its own wrinkles on
top, which is why this prototype's hot functions are written the way they are.

### 12.1 Monomorphization

When you write `vec.push(5)` in Rust, the compiler generates a **specialized
copy** of `push` for `i32`. This is called monomorphization — generic code
becomes concrete code at compile time. You cannot pre-compile a generic function
and hot-patch it later, because the concrete version you need only exists after
the compiler sees the concrete type — there's no single "the function" to
diff or patch.

**Our workaround:** hot functions use no generics. They take raw pointers and
primitive types only. All monomorphization happens on the engine side, which is
compiled ahead of time.

### 12.2 Symbol mangling

Rust mangles symbol names by default. `fn update()` in module `game` becomes
something like `_ZN4game6update17h3f8a9b2c1d4e5f6E`. You cannot look this up by
name at runtime, and it isn't even guaranteed to stay the same between two
otherwise-identical builds.

**Our workaround:** `#[no_mangle]` and `extern "C"` — the symbol name is exactly
`game_update`.

### 12.3 Borrow checker doesn't cross the ABI

The Rust borrow checker only works within a single compilation unit. Once you
cross an `extern "C"` boundary, you're in raw-pointer land. The compiler won't
prevent you from aliasing `&mut` references or using freed memory.

**Our workaround:** clear ownership rules. The engine owns all memory. The game
gets opaque handles and borrows nothing. The `WorldHandle` buffer is owned by
the engine and loaned to the game for the duration of each frame call.

### 12.4 Struct layout stability

If `GameState` adds a field between patches, any already-loaded old `.dll` that
expects the old layout will read garbage. This is the **state migration
problem**.

**Our workaround in this prototype:** we don't handle it. If you change
`GameState`, the old code becomes invalid. A production system would version
the state and provide migration functions — or, as §14.2 shows, push the
problem up to the application framework (e.g. re-instantiating a component
when its shape changes).

### 12.5 Inlining

The Rust compiler may **inline** a function — copy its body directly into the
caller instead of generating a separate callable function. An inlined function
has no address you can patch, which defeats *both* families from §11.4: there's
no table entry to redirect and no standalone function body to trampoline into.

**Our workaround:** `#[no_mangle] pub extern "C" fn` prevents inlining across
the library boundary (the compiler must keep it as a separate, exported
symbol).

---

## 13. Why managed languages mostly get this for free

### 13.1 The real dividing line: who already controls dispatch?

Every call in a running program resolves to an address somehow. The question
that determines how hard hot-patching is: **is that resolution step already
indirected through something you can reach into, or is it a fixed address
baked in at build time?**

- In a **managed, JIT-compiled** language (C#, Java, JavaScript), the runtime
  *already* owns this. Every method call goes through the runtime's own
  metadata and JIT-compiled-code cache — nothing is ever "just an address
  baked into the binary." The runtime can re-JIT a method and repoint its
  internal tables without any help from you.
- In an **ahead-of-time compiled, runtime-less** language (Rust, C, C++), a
  plain function call *is* a fixed address baked in at link time. There's no
  built-in indirection layer to reach into — unless you build one yourself,
  which is exactly what §4's hot table *is*: a hand-built substitute for the
  indirection a managed runtime gets for free.

### 13.2 .NET's actual Hot Reload / Edit and Continue

.NET has a real feature called **Hot Reload** (also known by its older name,
**Edit and Continue**) — the mechanism behind `dotnet watch` and Visual
Studio's "apply code changes while debugging." It works roughly like this:

1. You edit a method body and save.
2. The tooling recompiles *only that method* to new IL.
3. It hands the new IL to the running runtime through a metadata-update API.
4. The runtime redefines that method's compiled code. The **very next call**
   to it — from anywhere in the program, no indirection layer required — runs
   the new body.

This is genuine method-level patching, and it needs almost none of the
machinery Part 1 builds up: no hot table, no manual `AtomicPtr`, no versioned
`.dll` files, no opaque handles. The runtime already had a "table" — its
internal method-dispatch metadata — and Hot Reload just tells it to update one
entry. It isn't unlimited: you typically can't add/remove fields on a type,
change a method's signature, or add new virtual methods while the process is
running — the runtime's metadata format has edit-kinds it does and doesn't
support, similar in spirit to §12.4's struct-layout problem.

### 13.3 What this means for the C# scenario in this repo

The C#-scripted scenario in this repo's sibling `hot_reloading/` workspace does
**not** use this feature. Its `game_cs_loader` does a full assembly reload
through a collectible `AssemblyLoadContext` — the same "swap the whole module
behind a stable indirection point" design as Part 1 of this document, just one
level up (assemblies instead of `.dll`s, an `AssemblyLoadContext` instead of a
hot table). See
[`docs/how-csharp-scripting-works.md`](./how-csharp-scripting-works.md) §14
for that design. So in *this specific repo*, neither language actually does
true function-level patching — C# just happens to have the *option* to,
built into its runtime, if it were wired up differently.

---

## 14. Real-world implementations

Section 11 was theory. Here's who has actually built each piece, for real.

### 14.1 Live++ — binary patching for C++

**[Live++](https://liveplusplus.tech/)** is a commercial tool used in shipped
AAA games. It solves all three of §11's problems for **C++** — a language with
exactly the same "no runtime, direct addresses" constraints as Rust:

- **Partial compilation:** it drives an incremental build of just the changed
  translation units.
- **Address injection:** it resolves the new code's references against the
  actual, currently-running process image.
- **Getting code to run:** it uses **Family B from §11.4** — real binary
  patching. It overwrites the *old* function's entry point with a trampoline
  jump to the newly compiled code, so every existing call site — with no
  cooperation required — ends up running the new version.

This is the clearest existing proof that Tier 2 (§2.3) is a solved problem in
general, for AOT-compiled languages — just not a cheap one to build.

### 14.2 subsecond — partial compilation and jump tables for Rust

**[`subsecond`](https://github.com/DioxusLabs/dioxus)** (from the Dioxus
project) takes **Family A from §11.4** — safe indirection, not binary
patching — and pairs it with real object-file-level partial compilation. Its
own docs are explicit about the choice:

> "Unlike libraries like `detour`, Subsecond *does not* modify your process
> memory. Patching pointers is wildly unsafe and can lead to crashes and
> undefined behavior."

**How it solves Problem 1 (partial compilation) — its `ThinLink` linker:**

> "we intercept the Rust linking phase and then drive `rustc` manually. There's
> some diffing logic that compares assembly between compiles and then a
> linking phase where we patch symbols against the running process."

Concretely: let `rustc`'s own incremental compiler produce fresh object files
as usual (§11.2), then **diff those object files against the previous build's**
at the symbol level to find exactly which function bodies differ — not which
*file* changed, which *function*. Only those functions get linked into a new
artifact.

**How it solves Problem 2 (address injection) — anchoring on `main`:**

The new artifact's code still needs to call back into the already-running
program. `subsecond`'s source comments describe using **the `main` symbol as a
sentinel**: since ASLR (§11.3) shifts the whole binary by one random offset,
knowing where `main` was *supposed* to be (from the build) versus where it
*actually* is (reported by the running process) gives a single correction
factor, applied to every other address the patch needs — exactly the trick
§11.3 describes in general.

**How it solves Problem 3 (getting code to run) — jump tables, generated:**

The tiny compiled artifact — just the changed functions — is loaded into the
process the normal way: `libloading::Library::new(&table.lib)` on desktop
(memfd + `dlopen` on Android; `WebAssembly.instantiate` on web). At runtime, a
global `AtomicPtr<JumpTable>` maps each changed function's *original* address
to its *new* one. Every call site wrapped in `subsecond::call(...)` — opt-in,
not automatic — checks this table before calling, exactly like this
document's `HotFnTable` (§4), just generated automatically instead of
hand-written, and keyed by address instead of by named struct field.

**Where it's more aggressive than this prototype:** rather than always letting
an in-flight call finish on old code (§5.3's "graceful transition"),
`subsecond` has a `HotFnPanic` mechanism that can unwind a *currently
executing* call back to its nearest `subsecond::call()` boundary and re-invoke
it with the fresh code — closer to true "replace it right now" semantics for
code structured around a per-frame `tick()`-style entry point.

**Its own documented limits** line up exactly with §12's Rust-specific list:
struct layout/alignment changes need "framework-level re-instancing" (§12.4);
static initializer changes aren't observed; thread-locals reset on a new
patch; `main()` itself is never hooked (you still can't hot-patch program
startup).

### 14.3 Rust dylib-reload crates — the same category as this prototype

**[`hot-lib-reloader`](https://github.com/rksm/hot-lib-reloader-rs)** and
**[`dexterous_developer`](https://github.com/DGriffin91/dexterous_developer)**
use the *same* technique as this repository: compile game logic as a
`cdylib`, reload it with `libloading`, and route every call through function
pointers (or a trait object) so the reload is invisible to the caller. Hot
*reloading* via whole-module swap, not hot *patching* — Tier 1, same as
everything in Part 1 of this document.

### 14.4 Bottom line, compared

| Category                                                    | Example                                                    | What changes at runtime             | Effort required                                                       |
| -------------------------------------------------------------| -------------------------------------------------------------| --------------------------------------| -------------------------------------------------------------------------|
| Managed / JIT runtime already owns dispatch                 | C#'s real Hot Reload / Edit and Continue (§13.2)           | One method's compiled body          | Small — the runtime already does most of the work                     |
| AOT language, whole-module reload (**what this repo does**) | This prototype; `hot-lib-reloader`; `dexterous_developer`  | The entire compiled `.dll`/assembly | Moderate — a hot table (or ALC) plus a file watcher                    |
| AOT language, true function-level patching                  | Live++ (C++, binary patching); `subsecond` (Rust, jump tables) | Just the changed function's bytes   | Large — object-file diffing, address injection against a live process, and either a custom binary patcher or an automatically-generated indirection layer |

> **So: is any of this Rust-specific?** No — Tier 2 is a consequence of Rust
> (like C and C++) having no runtime that already owns code dispatch, not a
> Rust limitation *per se*. The same whole-module-reload shortcut this
> document builds is exactly what Unity, Unreal's Live Coding, and Rust's own
> dylib-reload crates all do for the same underlying reason. True
> function-level patching is *possible* for Rust — Live++ proves the technique
> works for AOT-compiled languages in general, and `subsecond` is a working,
> shipping implementation of it for Rust specifically — it's just
> substantially more engineering than the version C#'s runtime hands you
> nearly for free.

---

# Part 3 — Reference

## 15. Safety: what keeps this from crashing

### 15.1 The invariants we maintain

| Invariant                             | Why it matters                           |
| ---------------------------------------| ------------------------------------------|
| Engine never calls game code directly | Every call is patchable                  |
| Hot table reads use `Acquire`         | Sees fully-constructed pointers          |
| Hot table writes use `Release`        | Publishes fully-constructed pointers     |
| Old libraries are never unloaded      | In-flight calls complete safely          |
| `extern "C"` on all hot functions     | Stable ABI, predictable symbol names     |
| `#[repr(C)]` on all shared structs    | Predictable memory layout                |
| Opaque handles for engine state       | Engine internals can change freely       |
| World buffer allocated by engine      | Game never allocates across the boundary |

### 15.2 What's `unsafe` and why

Several operations in this prototype are `unsafe`:

- **`VirtualAlloc` / `VirtualFree`**: raw OS memory management.
- **`Library::new()`**: loading arbitrary native code.
- **`lib.get::<T>(b"name")`**: assuming the symbol has the declared type.
- **`std::mem::transmute`**: converting raw pointers to typed function pointers.
- **Dereferencing raw pointers** in engine callbacks and game functions.

In each case, the `unsafe` is bounded by a clear invariant. For example, the
`transmute` from `*mut ()` to `GameUpdateFn` is safe because:
- We only store `GameUpdateFn` values in that slot.
- We only read them back as `GameUpdateFn`.
- The slot is private; external code can't put wrong-type pointers in it.

### 15.3 What happens on failure

- **Build fails**: the orchestrator prints the error. The old functions keep
  running. The game doesn't stop.
- **Load fails** (symbol not found): the orchestrator prints the error. Old
  functions keep running.
- **DLL version mismatch** (ABI changed): undefined behavior — but our ABI is
  deliberately simple and stable.

---

## 16. Glossary

| Term                                 | Meaning                                                                                        |
| --------------------------------------| ------------------------------------------------------------------------------------------------|
| **Function pointer**                 | A memory address pointing to a block of machine code.                                          |
| **Hot function table**               | A struct of function pointers the engine calls through every frame (§4).                       |
| **Jump table**                       | This document's term plus `subsecond`'s own: a table mapping old function addresses to new ones, consulted on each call (§14.2). |
| **Atomic swap**                      | Changing a value in memory in a single, indivisible CPU operation.                             |
| **Release / Acquire**                | Memory orderings that guarantee visibility across threads.                                     |
| **Executable memory**                | Memory pages the CPU is allowed to run code from.                                              |
| **RWX**                              | Read + Write + Execute — memory permissions needed for JIT'd or injected code.                 |
| **Relocation**                       | Adjusting the absolute addresses inside compiled code to match where it actually landed in memory (§11.3). |
| **ASLR**                             | Address Space Layout Randomization — the OS loads a program at a different random base address each run, which complicates address injection (§11.3). |
| **Address injection**                | Resolving the addresses a freshly-compiled patch needs against a process that's already running, rather than at normal build time (§11.3). |
| **Partial / incremental compilation**| Compiling (and, for patching, diffing) only the code that actually changed, instead of the whole crate (§11.2). |
| **Trampoline**                       | A small patch at a function's old address that jumps to its new location — how binary patching redirects existing call sites without rewriting them (§11.4, §14.1). |
| **Binary patching**                  | Overwriting a function's machine code in a running process in place, rather than routing calls through an indirection table (§11.4 Family B). |
| **Position-independent code (PIC)**  | Machine code that doesn't assume a fixed load address, so it can be placed anywhere in memory — required for injecting freshly-compiled code into a running process. |
| **Orchestrator**                     | The binary that watches files, rebuilds, and patches the hot table.                            |
| **Opaque handle**                    | A pointer whose target type is hidden from the receiver.                                       |
| **ABI**                              | Application Binary Interface — the contract for how functions are called at the machine level. |
| **`extern "C"`**                     | Rust syntax for "use the C ABI for this function."                                             |
| **`#[no_mangle]`**                   | Tells Rust to export the function with the exact name you wrote.                               |
| **`#[repr(C)]`**                     | Tells Rust to lay out the struct fields in C order (no reordering).                            |
| **cdylib**                           | A C-compatible dynamic library (`.dll` on Windows, `.so` on Linux).                             |
| **libloading**                       | A Rust crate for loading dynamic libraries at runtime.                                         |
| **Monomorphization**                 | Compiler process of generating a concrete function from a generic one.                         |
| **Mangling**                         | Encoding type information into a function's exported symbol name.                              |
| **JIT**                              | Just-In-Time compilation — turning source/IR into machine code at runtime.                     |
| **Hot Reload / Edit and Continue**   | .NET's real method-level patching feature — the runtime redefines a method's compiled body in place (§13.2). |
| **Debounce**                         | Waiting for a burst of events to settle before acting.                                         |

---

## 17. Further reading

- **[How C# Scripting Works](./how-csharp-scripting-works.md)** — the companion
  doc covering FFI, the C ABI, and hosting the .NET runtime.
- **Rust Nomicon: [FFI](https://doc.rust-lang.org/nomicon/ffi.html)** — the
  official Rust reference on foreign function interfaces.
- **Rust Reference: [`extern` functions](https://doc.rust-lang.org/reference/items/external-blocks.html)**
- **[Cranelift](https://cranelift.dev/)** — a Rust-native code generator that
  could serve as the compilation backend for a real partial-compilation
  patcher (§11.2).
- **[Live++](https://liveplusplus.tech/)** — a commercial hot-patching tool for
  C++ that does real function-level binary patching in shipped AAA games
  (§14.1).
- **[`subsecond`](https://docs.rs/subsecond/latest/subsecond/)** (Dioxus
  project) — a working implementation of true function-level hot-patching for
  Rust (§14.2).
- **[`hot-lib-reloader`](https://github.com/rksm/hot-lib-reloader-rs)** and
  **[`dexterous_developer`](https://github.com/DGriffin91/dexterous_developer)**
  — Rust crates using the same dylib-reload technique as this prototype (§14.3).
- **Microsoft Docs: [.NET Hot Reload](https://learn.microsoft.com/en-us/visualstudio/debugger/hot-reload)**
  — the real method-level patching feature referenced in §13.2.
- **[`libloading` docs](https://docs.rs/libloading/)** — dynamic library loading.
- **[`notify` docs](https://docs.rs/notify/)** — cross-platform file watching.
- **[Handmade Hero — Live Code Editing](https://handmadehero.org/)** — a C++
  game built entirely around hot-reloading, with extensive discussion of the
  architecture.
- **Microsoft Docs: [VirtualAlloc](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc)** — Windows executable memory allocation.
- **Game Programming Patterns: [Data Locality](https://gameprogrammingpatterns.com/data-locality.html)** — related patterns for keeping game state hot-patch-friendly.
