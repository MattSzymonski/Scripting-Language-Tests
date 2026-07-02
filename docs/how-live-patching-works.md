# How Live-Patching Works — From Zero

This document explains, **assuming you know nothing about it**, how a Rust game
engine can reload game logic **while the game is running** — without closing the
window, without restarting the process, and without losing a single frame.

We will build the idea up slowly, defining every term the first time it appears.
By the end you should understand:

- why you'd want to hot-reload game code at all,
- the difference between "reload the whole DLL" and "patch individual functions,"
- what a **function pointer** is and why it's the key mechanism,
- what a **hot function table** is and how the engine calls through it,
- how we **atomically swap** function addresses so the engine never sees garbage,
- how the **orchestrator** watches files, rebuilds code, and injects new
  functions — all while the engine keeps running,
- what we actually implemented vs. what we simulated,
- the Rust-specific limitations that make true function-level patching hard.

> Throughout, file references point at the real code:
> [`live_engine/src/hot_table.rs`](../live_engine/src/hot_table.rs),
> [`live_engine/src/hot_memory.rs`](../live_engine/src/hot_memory.rs),
> [`live_engine/src/engine.rs`](../live_engine/src/engine.rs),
> [`live_engine/src/api.rs`](../live_engine/src/api.rs),
> [`orchestrator/src/main.rs`](../orchestrator/src/main.rs),
> [`game_hot/src/lib.rs`](../game_hot/src/lib.rs).

---

## Table of contents

1. [The 30-second picture](#1-the-30-second-picture)
2. [Why live-patching? The problem with full restarts](#2-why-live-patching-the-problem-with-full-restarts)
3. [A quick recap: what is a function pointer?](#3-a-quick-recap-what-is-a-function-pointer)
4. [The hot function table: one level of indirection](#4-the-hot-function-table-one-level-of-indirection)
5. [Atomic swaps: changing pointers without crashing](#5-atomic-swaps-changing-pointers-without-crashing)
6. [Executable memory: where the CPU runs code](#6-executable-memory-where-the-cpu-runs-code)
7. [The ABI contract for hot functions](#7-the-abi-contract-for-hot-functions)
8. [Opaque handles: why the game never touches engine internals](#8-opaque-handles-why-the-game-never-touches-engine-internals)
9. [The orchestrator: the controller that makes it all happen](#9-the-orchestrator-the-controller-that-makes-it-all-happen)
10. [A single patch cycle, step by step](#10-a-single-patch-cycle-step-by-step)
11. [What we built vs. what we simulated](#11-what-we-built-vs-what-we-simulated)
12. [Rust-specific challenges](#12-rust-specific-challenges)
13. [Safety: what keeps this from crashing](#13-safety-what-keeps-this-from-crashing)
14. [Glossary](#14-glossary)
15. [Further reading](#15-further-reading)

---

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
picks them up on the next frame. That's the whole idea.

---

## 2. Why live-patching? The problem with full restarts

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

### 2.2 The coarse solution: reload the whole DLL

A common step up is **DLL hot-reloading**: compile the game as a dynamic library
(`.dll` / `.so`), and when it changes, load a new copy and restart the scene.
This is what many game engines (Unity, Unreal with Live Coding) effectively do.

Problems with whole-DLL reload:
- You lose all **game state** — the bird position, the score, the pipe layout.
  You have to serialize and restore it.
- The reload itself takes a **noticeable pause** while the new library
  initializes.
- You might trigger **reloading code that didn't change**, wasting time.

### 2.3 The fine-grained goal: function-level patching

What we really want:

> Change *only* the functions that actually changed, swap their addresses into
> place atomically, and keep everything else — state, window, loop — untouched.

This is what our prototype does at the **table level**: each function slot is
independently swappable. The render function can be patched without touching the
update function. The init function can be patched without touching render.

True *function-level* patching — compiling only the changed Rust function to
machine code and injecting just those bytes — is a harder problem we discuss
in §11.

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

This is the entire mechanism behind live-patching:

- The engine stores function pointers in a table.
- To call the game, it reads the current pointer from the table and calls it.
- To *patch* the game, the orchestrator writes a **new** pointer into the table.
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

> **The engine never calls game code directly.** Every call goes through the hot
> table. This single rule is what makes live-patching possible.

If the engine ever "short-circuited" the table (called a game function by its
compiled name), that call would be **unpatchable**. The table is the only
channel.

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
the new code. No interruption, no crash.

---

## 6. Executable memory: where the CPU runs code

### 6.1 Memory is not all the same

Your computer's memory has **permission flags**. A page of memory can be:

| Permission                     | Used for                           |
| ------------------------------ | ---------------------------------- |
| Read (`R`)                     | Reading data                       |
| Read + Write (`RW`)            | Normal variables, heap, stack      |
| Read + Execute (`RX`)          | Running machine code               |
| Read + Write + Execute (`RWX`) | Rare — generating and running code |

The CPU will **refuse** to execute code from a page that doesn't have the
Execute flag set. This is a hardware security feature (it's called
**W^X** or **DEP** — Data Execution Prevention).

### 6.2 Why we need RWX memory

To inject new machine code at runtime, you need memory that is:
- **Writable** (so you can put the code there),
- **Executable** (so the CPU will run it).

You request this from the operating system. On Windows, the function is
`VirtualAlloc` with the `PAGE_EXECUTE_READWRITE` flag. On Linux/macOS, it's
`mmap` with `PROT_READ | PROT_WRITE | PROT_EXEC`.

Our project has a real executable memory allocator in
[`live_engine/src/hot_memory.rs`](../live_engine/src/hot_memory.rs):

```rust
let ptr = VirtualAlloc(
    null_mut(),          // let OS choose location
    size,                // how many bytes
    0x3000,              // MEM_COMMIT | MEM_RESERVE
    0x40,                // PAGE_EXECUTE_READWRITE
);
```

### 6.3 How we use it (and don't)

In a **true function-level patching** system, you would:
1. Compile a single function to position-independent machine code,
2. Copy those bytes into your RWX memory,
3. Store the address of that memory in the hot table.

In our **prototype**, we allocate RWX memory (the infrastructure is real), but
we load whole `.dll` files via the operating system's normal loader (which
handles RWX memory and relocation internally). The `ExecutableMemory` allocator
is ready for the next step — JIT-compiled function bytes — but today we don't
copy individual function bytes into it.

This is one of the key "simulated vs real" distinctions (see §11).

---

## 7. The ABI contract for hot functions

### 7.1 Why `extern "C"` is required

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

### 7.2 What can cross the boundary

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

### 7.3 The function signatures

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

## 8. Opaque handles: why the game never touches engine internals

### 8.1 The problem with sharing types

If the game code directly included the engine's `EngineState` struct, then
changing `EngineState` (adding a field, reordering, changing a type) would
break all existing compiled game `.dll` files. The old code would read the wrong
offsets and crash.

### 8.2 The solution: opaque handles

The engine passes two opaque pointers to the game:

| Handle         | Type          | What it points to             | Who owns it                         |
| -------------- | ------------- | ----------------------------- | ----------------------------------- |
| `EngineHandle` | `*mut c_void` | The engine's private state    | Engine                              |
| `WorldHandle`  | `*mut c_void` | A buffer for game world state | Engine allocates, game reads/writes |

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

---

## 9. The orchestrator: the controller that makes it all happen

The orchestrator is a **separate binary** (`orchestrator/`) that has one job:
keep the game code fresh. It has three main loops running in parallel:

### 9.1 The build step

```rust
fn build_game_lib(release: bool) -> Result<PathBuf, String> {
    Command::new("cargo")
        .args(["build", "-p", "game_hot"])
        .output()?;
    // Returns path to target/debug/game_hot.dll
}
```

This runs `cargo build` on the `game_hot` crate, producing a `.dll`. We do this
inside the running process by spawning `cargo` as a child process.

### 9.2 The load step

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

### 9.3 The watch step

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

### 9.4 The patch step

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

### 9.5 Why versioned file names?

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

## 10. A single patch cycle, step by step

Let's follow one complete cycle from "you save the file" to "the change appears
on screen."

### Step 0 — The game is running

The engine loop is spinning at ~60 FPS, calling through the hot table every
frame. The orchestrator's file watcher thread is sleeping, waiting for the OS to
tell it something changed.

### Step 1 — You edit and save

You change the gravity constant in `game_hot/src/lib.rs` from `800.0` to
`300.0`. You press Ctrl+S.

### Step 2 — The OS notifies

The operating system detects the file modification and wakes up the `notify`
watcher. The watcher checks that the changed file has a `.rs` extension and
sends a signal through a channel to the reload thread.

### Step 3 — Debounce

The reload thread receives the signal, then waits 300ms to see if more events
arrive (editors often write a file in multiple bursts). After the cooldown, it
decides: "ok, rebuild now."

### Step 4 — Rebuild

The orchestrator spawns `cargo build -p game_hot`. Rust compiles the changed
`lib.rs` into a new `game_hot.dll`. The old `.dll` is still loaded and fully
functional — the build happens to a different file.

### Step 5 — Copy to versioned name

The orchestrator copies the fresh `.dll` to `game_hot_v4.dll` (if this is the
4th patch). This avoids Windows file-locking issues.

### Step 6 — Load the new library

```rust
let lib = Library::new("game_hot_v4.dll")?;
let new_update = lib.get::<GameUpdateFn>(b"game_update")?;
```

The OS maps the new `.dll` into the process. `libloading` resolves the symbol
`game_update` to its address. This address points to the freshly-compiled
machine code for the new version.

### Step 7 — Atomic swap

```rust
hot_table.patch_update(*new_update);  // Release ordering
```

The `AtomicPtr` in the hot table now points to the new `game_update`.

### Step 8 — The engine picks it up

On the **very next frame**, the engine reads the hot table:

```rust
let update_fn = hot_table.read_update();  // Acquire ordering
update_fn(&api, engine, world, dt);       // calls the NEW function
```

The new gravity value (`300.0`) takes effect. The player character floats down
gently instead of dropping fast.

### Step 9 — Old library stays

The orchestrator pushes the new `Library` into an `old_libraries: Vec<Library>`
list. The old `game_hot_v3.dll` stays loaded. If a call to the old `game_update`
is still executing on another thread, it completes safely using the old code.

**Total time from save to effect: ~1–2 seconds** (most of which is `cargo
build`). One frame later, the change is visible. The game never stopped.

---

## 11. What we built vs. what we simulated

This is an honest accounting of where the prototype is real and where it
approximates the ideal.

### 11.1 What's real

| Feature                               | Implementation                                                     |
| ------------------------------------- | ------------------------------------------------------------------ |
| **Hot function table**                | `HotFnTable` with independent `AtomicPtr` slots                    |
| **Atomic swapping**                   | `swap(Release)` / `load(Acquire)` — real hardware guarantees       |
| **Independent per-function patching** | `patch_update()` doesn't touch `patch_render()`                    |
| **Old code retention**                | Old `Library` values kept in a `Vec`                               |
| **File watching**                     | `notify` crate with OS-level file events                           |
| **Automatic rebuild**                 | `cargo build` spawned as child process                             |
| **Executable memory allocator**       | `VirtualAlloc(PAGE_EXECUTE_READWRITE)` — real OS call              |
| **ABI-stable function signatures**    | `extern "C"`, `#[no_mangle]`, `#[repr(C)]`                         |
| **Opaque handle separation**          | `EngineHandle` vs `WorldHandle` — game never touches engine memory |
| **Graceful transition**               | Old calls finish, new calls use new code — no interruption         |

### 11.2 What's simulated

| Feature                                         | What we do instead                                                 | Why                                                                                                                                                                                                                         |
| ----------------------------------------------- | ------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Function-level recompilation**                | Rebuild the entire `game_hot` crate, not just the changed function | Rust has no built-in incremental compilation to object files for live reload. A true per-function compile would need a JIT compiler (cranelift, LLVM JIT) integrated into the orchestrator.                                 |
| **Copying machine code into engine RWX memory** | Load the whole `.dll` via the OS loader                            | The OS loader handles **relocation** — adjusting absolute addresses inside the machine code to match where the `.dll` actually landed in memory. Doing this manually is extremely complex. The DLL loader does it for free. |
| **Fine-grained dependency tracking**            | Rebuild if *any* `.rs` file in `game_hot/` changes                 | Rust's module system means changing one file can affect others. Tracking exact dependencies would need compiler-level integration.                                                                                          |

### 11.3 The path to true function-level patching

A production system would replace `cargo build` with a **JIT compiler** like
[Cranelift](https://cranelift.dev/). The flow would be:

1. Parse the changed Rust source.
2. Compile *only* the changed function to position-independent machine code.
3. Write those bytes into `ExecutableMemory` pages.
4. Store the address in the hot table.
5. The old code stays in its pages (retained, not freed).

This eliminates the DLL loading step entirely and reduces iteration time from
~1 second to ~milliseconds. It's a significant engineering effort — Cranelift
integration, Rust MIR-level compilation, relocation handling — but the
architecture we built (hot table, atomic swaps, RWX memory, opaque handles) is
exactly what such a system would plug into.

---

## 12. Rust-specific challenges

### 12.1 Monomorphization

When you write `vec.push(5)` in Rust, the compiler generates a **specialized
copy** of `push` for `i32`. This is called monomorphization — generic code
becomes concrete code at compile time. You cannot pre-compile a generic function
and hot-patch it later, because the concrete version you need only exists after
the compiler sees the concrete type.

**Our workaround:** hot functions use no generics. They take raw pointers and
primitive types only. All monomorphization happens on the engine side, which is
compiled ahead of time.

### 12.2 Symbol mangling

Rust mangles symbol names by default. `fn update()` in module `game` becomes
something like `_ZN4game6update17h3f8a9b2c1d4e5f6E`. You cannot look this up by
name at runtime.

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
the state and provide migration functions.

### 12.5 Inlining

The Rust compiler may **inline** a function — copy its body directly into the
caller instead of generating a separate callable function. An inlined function
has no address you can patch.

**Our workaround:** `#[no_mangle] pub extern "C" fn` prevents inlining across
the library boundary (the compiler must keep it as a separate, exported
symbol).

---

## 13. Safety: what keeps this from crashing

### 13.1 The invariants we maintain

| Invariant                             | Why it matters                           |
| ------------------------------------- | ---------------------------------------- |
| Engine never calls game code directly | Every call is patchable                  |
| Hot table reads use `Acquire`         | Sees fully-constructed pointers          |
| Hot table writes use `Release`        | Publishes fully-constructed pointers     |
| Old libraries are never unloaded      | In-flight calls complete safely          |
| `extern "C"` on all hot functions     | Stable ABI, predictable symbol names     |
| `#[repr(C)]` on all shared structs    | Predictable memory layout                |
| Opaque handles for engine state       | Engine internals can change freely       |
| World buffer allocated by engine      | Game never allocates across the boundary |

### 13.2 What's `unsafe` and why

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

### 13.3 What happens on failure

- **Build fails**: the orchestrator prints the error. The old functions keep
  running. The game doesn't stop.
- **Load fails** (symbol not found): the orchestrator prints the error. Old
  functions keep running.
- **DLL version mismatch** (ABI changed): undefined behavior — but our ABI is
  deliberately simple and stable.

---

## 14. Glossary

| Term                   | Meaning                                                                                        |
| ---------------------- | ---------------------------------------------------------------------------------------------- |
| **Function pointer**   | A memory address pointing to a block of machine code.                                          |
| **Hot function table** | A struct of function pointers the engine calls through every frame.                            |
| **Atomic swap**        | Changing a value in memory in a single, indivisible CPU operation.                             |
| **Release / Acquire**  | Memory orderings that guarantee visibility across threads.                                     |
| **Executable memory**  | Memory pages the CPU is allowed to run code from.                                              |
| **RWX**                | Read + Write + Execute — memory permissions needed for JIT code.                               |
| **Orchestrator**       | The binary that watches files, rebuilds, and patches the hot table.                            |
| **Opaque handle**      | A pointer whose target type is hidden from the receiver.                                       |
| **ABI**                | Application Binary Interface — the contract for how functions are called at the machine level. |
| **`extern "C"`**       | Rust syntax for "use the C ABI for this function."                                             |
| **`#[no_mangle]`**     | Tells Rust to export the function with the exact name you wrote.                               |
| **`#[repr(C)]`**       | Tells Rust to lay out the struct fields in C order (no reordering).                            |
| **cdylib**             | A C-compatible dynamic library (`.dll` on Windows, `.so` on Linux).                            |
| **libloading**         | A Rust crate for loading dynamic libraries at runtime.                                         |
| **Monomorphization**   | Compiler process of generating a concrete function from a generic one.                         |
| **Mangling**           | Encoding type information into a function's exported symbol name.                              |
| **JIT**                | Just-In-Time compilation — turning source/IR into machine code at runtime.                     |
| **Debounce**           | Waiting for a burst of events to settle before acting.                                         |

---

## 15. Further reading

- **[How C# Scripting Works](./how-csharp-scripting-works.md)** — the companion
  doc covering FFI, the C ABI, and hosting the .NET runtime.
- **Rust Nomicon: [FFI](https://doc.rust-lang.org/nomicon/ffi.html)** — the
  official Rust reference on foreign function interfaces.
- **Rust Reference: [`extern` functions](https://doc.rust-lang.org/reference/items/external-blocks.html)**
- **[Cranelift JIT](https://cranelift.dev/)** — a Rust-native code generator
  that could replace `cargo build` for true function-level patching.
- **[`libloading` docs](https://docs.rs/libloading/)** — dynamic library loading.
- **[`notify` docs](https://docs.rs/notify/)** — cross-platform file watching.
- **[Handmade Hero — Live Code Editing](https://handmadehero.org/)** — a C++
  game built entirely around hot-reloading, with extensive discussion of the
  architecture.
- **Microsoft Docs: [VirtualAlloc](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc)** — Windows executable memory allocation.
- **Game Programming Patterns: [Data Locality](https://gameprogrammingpatterns.com/data-locality.html)** — related patterns for keeping game state hot-patch-friendly.
