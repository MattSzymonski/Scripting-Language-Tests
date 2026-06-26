# How the C# Scripting Works — From Zero

This document explains, **assuming you know nothing about it**, how a game
written entirely in **C#** can run on top of a game engine written in **Rust**.

We will build the idea up slowly, defining every term the first time it appears.
By the end you should understand:

- why anyone would mix two languages in one program,
- the difference between a "native" program (Rust) and a "managed" one (C#),
- what the ".NET runtime" is and how we *embed* it inside our Rust program,
- what **FFI** ("Foreign Function Interface") means and why it's the key idea,
- exactly how our engine and our game call back and forth, line by line.

> Throughout, file references point at the real code:
> [flappy/src/ffi.rs](../flappy/src/ffi.rs),
> [flappy/src/main.rs](../flappy/src/main.rs),
> [game_cs/src/EngineApi.cs](../game_cs/src/EngineApi.cs),
> [game_cs/src/Engine.cs](../game_cs/src/Engine.cs),
> [game_cs/src/Interop.cs](../game_cs/src/Interop.cs).

---

## Table of contents

1. [The 30-second picture](#1-the-30-second-picture)
2. [Why mix two languages at all?](#2-why-mix-two-languages-at-all)
3. [Two kinds of programs: native vs. managed](#3-two-kinds-of-programs-native-vs-managed)
4. [What is the .NET runtime?](#4-what-is-the-net-runtime)
5. [Hosting the runtime inside our own program](#5-hosting-the-runtime-inside-our-own-program)
6. [What is FFI?](#6-what-is-ffi)
7. [The bridge has two directions](#7-the-bridge-has-two-directions)
8. [Function pointers, explained](#8-function-pointers-explained)
9. [The ABI contract: keeping both sides identical](#9-the-abi-contract-keeping-both-sides-identical)
10. [`[UnmanagedCallersOnly]` and calling conventions](#10-unmanagedcallersonly-and-calling-conventions)
11. [Marshalling: what data can cross the boundary](#11-marshalling-what-data-can-cross-the-boundary)
12. [A single frame, step by step](#12-a-single-frame-step-by-step)
13. [The dangerous bits: memory, lifetimes, threads, exceptions](#13-the-dangerous-bits-memory-lifetimes-threads-exceptions)
14. [How to add a new engine feature](#14-how-to-add-a-new-engine-feature)
15. [Just-in-time compilation](#15-just-in-time-compilation)
16. [Memory ownership and hot reload](#16-memory-ownership-and-hot-reload)
17. [Glossary](#17-glossary)
18. [Further reading](#18-further-reading)

---

## 1. The 30-second picture

Our program is **one** running application (`flappy.exe`). Inside it, two worlds
coexist:

```
            ┌──────────────────────── flappy.exe (one process) ───────────────────────┐
            │                                                                          │
            │   RUST SIDE  (the engine)                 C# SIDE  (the game)            │
            │   ───────────────────────                 ──────────────────            │
            │   • opens the window                       • Bird, Pipe, FlappyScene     │
            │   • runs the game loop      ── calls ──▶   • decides what happens        │
            │   • talks to the GPU                         each frame                  │
            │                             ◀── calls ──   • asks engine to draw things  │
            │                                                                          │
            └──────────────────────────────────────────────────────────────────────────┘
```

- The **Rust side** is the *engine*. It owns the window and the main loop, and
  it knows how to actually put pixels on screen.
- The **C# side** is the *game*. It contains all the gameplay logic — gravity,
  pipes, scoring — but it cannot draw anything by itself. It *asks* the engine
  to draw.

The whole trick of this document is explaining **how those two sides call each
other's functions**, even though they are written in different languages and
compiled by different compilers.

The technique that makes it possible is called **FFI**. Everything below is a
slow walk toward fully understanding that one acronym.

---

## 2. Why mix two languages at all?

A fair question: if Rust can already draw the game (the first version of this
project *was* pure Rust), why bring in C#?

The pattern here is called **engine + scripting**, and almost every commercial
game engine uses it:

| Layer        | Language        | Changes how often? | Cares about…                        |
| ------------ | --------------- | ------------------ | ----------------------------------- |
| **Engine**   | fast/low-level  | rarely             | performance, GPU, memory, the window |
| **Game**     | easy/high-level | constantly         | gameplay rules, fun, iteration speed |

- Unity's engine is C++; you write your game in **C#**.
- Unreal's engine is C++; you write gameplay in C++ or **Blueprints**.
- Godot's engine is C++; you write your game in **GDScript** or C#.

The engine is the stable, high-performance foundation. The "script" is the part
you rewrite ten times a day while figuring out what's fun. Keeping them in
different languages lets each layer use the best tool for its job.

In our project:

- **Rust** = the engine. It's compiled to fast native code and handles the
  window + drawing.
- **C#** = the script. It's quick to write and edit, and it holds 100% of the
  Flappy Bird gameplay.

---

## 3. Two kinds of programs: native vs. managed

To understand the bridge, you first need to understand that **Rust programs and
C# programs run in fundamentally different ways**.

### 3.1 What a CPU actually runs

A CPU (the processor in your computer) understands exactly one thing: **machine
code** — long sequences of tiny numeric instructions like "add these two
numbers," "copy this value here," "jump to that location." Machine code is
specific to a *kind* of CPU (e.g. "x64" desktop processors).

No CPU understands "Rust" or "C#" directly. Something must translate. The
*when* and *how* of that translation is the big difference.

### 3.2 Rust: compiled ahead of time to native code

When you run `cargo build`, the Rust compiler turns your Rust source into
**machine code** right then, on your machine, and writes it into an `.exe`. When
you double-click that `.exe`, the CPU runs those instructions directly.

We call this a **native** program: the file *is* machine code, ready to go.
There is no helper sitting underneath it. This is why Rust (like C and C++) is
fast and has no "runtime" requirement — the OS loads the `.exe` and jumps in.

### 3.3 C#: compiled to an intermediate language, run by a runtime

C# works in **two stages**:

1. When you run `dotnet build`, the C# compiler does **not** produce machine
   code. It produces a `.dll` file containing **IL** (*Intermediate Language*,
   also called CIL or MSIL). IL is a compact, CPU-independent "half-way"
   instruction set — not Rust-style source, but not machine code either.

2. To actually run, that IL needs a program called the **.NET runtime** (more on
   it in the next section). The runtime reads the IL and, *while the program is
   running*, translates it into machine code on the fly. This last-second
   translation is called **JIT compilation** ("Just-In-Time").

We call C# a **managed** language because that runtime "manages" the program
while it executes: it JIT-compiles the IL, automatically frees unused memory
(the **garbage collector**, or **GC**), checks array bounds, and so on.

### 3.4 The consequence

```
   RUST  (.exe)                         C#  (.dll)
   ──────────────                       ─────────────────────────────
   source ─► [rustc] ─► machine code    source ─► [csc] ─► IL
                          │                                  │
                          ▼                                  ▼
                    CPU runs it                     .NET runtime JIT-compiles
                    directly                        IL ─► machine code, then runs
```

So a C# `.dll` is **not** something the CPU can run on its own. It is more like
a recipe that only the .NET runtime knows how to cook. This single fact is why
our Rust program has to do extra work: before it can call any C# code, it must
**start a .NET runtime** to bring that C# code to life.

That is the subject of the next two sections.

---

## 4. What is the .NET runtime?

The **.NET runtime** (its engine is historically called the **CLR**, the *Common
Language Runtime*) is itself a native program — a set of `.dll` files that ship
with the .NET SDK. Its job is to *host and run managed code*. Concretely it:

- **loads** your IL `.dll` into memory,
- **JIT-compiles** IL methods into machine code the first time each is called,
- provides **garbage collection** (frees objects you stop using),
- provides the giant **standard library** (`System.*` — strings, lists, math…),
- enforces **type and memory safety** for managed code.

### 4.1 The normal way C# starts

Normally you never think about the runtime because the tooling hides it. When
you type:

```bash
dotnet run
```

a small launcher finds the right .NET runtime on your machine, starts it, points
it at your program's `Main` method, and off it goes. The runtime is the *host*;
your code is the *guest*.

### 4.2 Our situation is upside-down

In our project there is **no `dotnet run`**. The program that starts is a *Rust*
`.exe`. There's no `Main` in C# to launch.

So we must do by hand what `dotnet run` normally does for us: our Rust program
has to **start a .NET runtime itself**, load the C# `.dll`, and call into it.
This is called **hosting the runtime**, and it's the topic of the next section.

> Mental model: normally the .NET runtime is the *building* and your C# app is a
> *tenant*. We're doing the reverse — our Rust `.exe` is the building, and it
> invites the .NET runtime in as a *tenant* so the C# game can live inside it.

---

## 5. Hosting the runtime inside our own program

"Hosting the runtime" means: from native code, **boot up the .NET runtime and
get it to run specific managed methods for us.** Microsoft ships an official C
library for exactly this, and there's a friendly Rust wrapper around it called
[`netcorehost`](https://crates.io/crates/netcorehost). We use that wrapper.

### 5.1 The pieces involved

- **`hostfxr`** — a small Microsoft library ("host **f**ramework re**s**olve**r**")
  whose job is to figure out *which* installed .NET runtime to use and start it.
- **`runtimeconfig.json`** — a tiny file built next to our C# `.dll` that says
  "this code targets .NET 10." `hostfxr` reads it to pick the matching runtime.
  Ours looks like:

  ```json
  {
    "runtimeOptions": {
      "tfm": "net10.0",
      "framework": { "name": "Microsoft.NETCore.App", "version": "10.0.0" }
    }
  }
  ```

  (A normal app gets this for free; because our C# is a *library*, we asked the
  build to generate it with `<GenerateRuntimeConfigurationFiles>` in
  [game_cs.csproj](../game_cs/game_cs.csproj).)

### 5.2 What our Rust host actually does

This all lives in `load_managed_game()` in
[flappy/src/main.rs](../flappy/src/main.rs). In plain English, the steps are:

1. **Find and load `hostfxr`** (`nethost::load_hostfxr()`).
2. **Initialize a runtime** for our config file
   (`hostfxr.initialize_for_runtime_config("game_cs.runtimeconfig.json")`).
   At this moment a live .NET runtime now exists inside our process.
3. **Get a "delegate loader" for our assembly**
   (`context.get_delegate_loader_for_assembly("game_cs.dll")`). An *assembly* is
   just .NET's word for a compiled `.dll`. This loads the C# game (in its own
   isolated loading context) and gives us a tool to look up functions in it.
4. **Look up three functions by name** — `Init`, `Update`, `Draw` — from the
   type `"Flappy.Interop, game_cs"` (read as: the `Flappy.Interop` class, in the
   `game_cs` assembly).

The result of step 4 is three **function pointers** (explained in §8): the
*memory addresses* of the JIT-compiled C# methods. From Rust's point of view
they are now just plain callable functions:

```rust
type InitFn   = extern "system" fn(*const EngineApi);
type UpdateFn = extern "system" fn(f32);
type DrawFn   = extern "system" fn();
```

Once we hold those three addresses, the runtime mostly fades into the
background. Calling `update(dt)` from Rust now runs C# code. **That** is the
moment the two worlds connect — and to understand what's really happening when
we "call across," we need FFI.

---

## 6. What is FFI?

**FFI** stands for **Foreign Function Interface**. It is the general name for
"code written in one language calling functions written in (or compiled by) a
different language."

The reason FFI needs rules — and isn't automatic — is subtle, so let's go slow.

### 6.1 Languages don't "talk"; CPUs run instructions

When Rust calls a Rust function, here's what *actually* happens at the machine
level:

- The caller puts the **arguments** in agreed-upon places — some CPU **registers**
  (tiny named storage slots inside the CPU) and/or on the **stack** (a scratch
  region of memory).
- It records a **return address** (where to continue afterward).
- It **jumps** to the function's machine code.
- The function reads its arguments from those agreed-upon places, does its work,
  puts the **return value** in an agreed-upon place, and jumps back.

Notice the recurring phrase **"agreed-upon."** Which register holds the first
argument? Who cleans up the stack afterward? How is an integer vs. a float
passed? None of that is universal — it's a *convention* both sides must follow.

### 6.2 The ABI: the contract both sides obey

That set of conventions is called the **ABI** — the *Application Binary
Interface*. The ABI answers questions like:

- In what order and in which registers/stack slots do arguments go?
- How are return values handed back?
- How big is an `int`? A pointer? How is a struct laid out in memory?
- Who is responsible for cleaning up the stack? (This part is the **calling
  convention**.)

If two pieces of compiled code **agree on the ABI**, then it literally does not
matter what source language each was written in. Machine code calling machine
code only cares that everyone followed the same contract.

### 6.3 The lingua franca: the "C ABI"

Over decades, the **C language's ABI** became the universal meeting point. Nearly
every language can "speak C" at its boundaries: Rust can mark functions
`extern "C"`, C# can call and expose functions using C conventions, Python,
Go, Swift — all of them can. The C ABI is the *Esperanto* of compiled code.

So our strategy is exactly the industry-standard one:

> **Rust and C# never speak "Rust" or "C#" to each other. At the boundary they
> both speak the C ABI.** That shared contract is what makes the calls work.

### 6.4 The catch: only "plain data" crosses

Because the boundary is just "machine code following the C ABI," only simple,
predictable data can pass through it:

- **numbers** (`int`, `float`, `double`…) — fine, they're just bits,
- **pointers** (memory addresses) — fine, an address is just a number,
- **plain structs** of the above — fine, if both sides agree on the layout.

What **cannot** cross directly are each language's rich, managed objects: a C#
`string`, a C# `List<T>`, a Rust `String`, a Rust `Vec<T>`. Those have internal
representations the other side knows nothing about. To send such things across,
you must convert them into plain data first — that conversion is called
**marshalling** (see §11).

Keep this rule in mind; it explains almost every design choice in our bridge.

### 6.5 Do we have to write the marshalling ourselves?

For **blittable** data (numbers, pointers — §11.1), there's *nothing to write*:
the bytes already mean the same thing on both sides, so they cross for free.

For **non-blittable** data (strings, arrays, collections, anything with a managed
representation), *someone* has to convert it to plain data and back. Whether that
"someone" is you depends on **which interop mechanism you use**:

- **The path we use — `[UnmanagedCallersOnly]` + raw function pointers** — is the
  "bare metal" one. The runtime does **no** automatic conversion here; only
  blittable types are even allowed across. So for things like text we marshal
  **by hand** (encode the string to UTF-8 bytes, pass a pointer — §11.4). It's a
  few lines, and it's fully under our control.
- **Classic P/Invoke (`[DllImport]`)** — .NET's older interop path — *does* ship
  an automatic marshaller. Annotate parameters (e.g. `[MarshalAs]`) and the
  runtime converts common types (strings, arrays, certain structs) for you. The
  newer **`[LibraryImport]`** source generator does the same, but generates the
  conversion code at *compile* time for speed and AOT-friendliness.

So: with our function-pointer approach we hand-roll the few conversions we need;
with P/Invoke much of it is automatic. Either way the *concept* is identical —
turn rich values into C-ABI-plain data.

**Are there libraries/tools for this?** Yes, at several levels:

- **Binding generators for Rust ↔ C# specifically.**
  [`csbindgen`](https://github.com/Cysharp/csbindgen) reads your Rust `extern`
  functions and **generates the matching C# P/Invoke code** automatically.
  [`interoptopus`](https://github.com/ralfbiedert/interoptopus) lets you describe
  a Rust API once and generates bindings (and the marshalling glue) for C#, C,
  and Python. [`cbindgen`](https://github.com/mozilla/cbindgen) generates a C
  header from Rust, which C# can then bind to. These remove the manual,
  error-prone work of keeping both sides in sync (the §9 problem).
- **Language-agnostic serialization** for structured data:
  **FlatBuffers**, **Protocol Buffers**, **Cap'n Proto**. These define a schema
  once and generate readers/writers in *many* languages; you pass a byte buffer
  across the boundary and each side decodes it. Heavier than a raw struct, but it
  sidesteps ABI matching entirely and works for any language pair.

We hand-write our tiny bridge because it's small and educational; a large project
would likely generate it.

### 6.6 Is marshalling different for each pair of languages?

Yes and no — and the distinction is the whole reason the C ABI matters.

- **The mechanics are language-specific.** Each language has its own object
  representations and its own FFI tools, so "how you marshal" genuinely differs
  for Rust↔C#, Rust↔Python, C++↔C#, and so on. A C# `string` is marshalled
  differently from a Python `str`.
- **But the *meeting point* is shared.** The universal trick (§6.3) is to route
  everything through the **C ABI** as a common hub. Each language only has to know
  how to convert **to and from C-shaped plain data** — it never has to understand
  any *other* language directly. So you don't write a separate bridge for every
  language *pair*; each language writes **one** "speak C" adapter, and they all
  meet in the middle.

That's why this document keeps insisting both sides "speak C at the boundary": it
turns an `N × N` problem (every pair) into an `N` problem (every language ↔ C).

### 6.7 Reading a Rust-defined struct from the game side

A very common need: **Rust defines a struct, and C# wants to read it** (e.g. a
block of shared game state — see §16.2). This works by the exact same ABI
agreement we use for the function table in §9, just applied to *data*:

1. **Rust fixes the layout** with `#[repr(C)]`. By default Rust makes *no*
   guarantee about field order or padding (it may rearrange fields); `#[repr(C)]`
   forces the predictable C layout — declared order, C padding rules.

   ```rust
   #[repr(C)]
   pub struct SharedState {
       pub bird_y: f32,
       pub score:  u32,
       pub alive:  u8,   // use a 1-byte int, not Rust's bool, to be safe
   }
   ```

2. **C# declares a mirror struct** with `[StructLayout(LayoutKind.Sequential)]`,
   **same fields, same order, matching types**:

   ```csharp
   [StructLayout(LayoutKind.Sequential)]
   public struct SharedState
   {
       public float BirdY;
       public uint  Score;
       public byte  Alive;
   }
   ```

3. **Rust hands C# a pointer** to its struct (through an `EngineApi` function, or
   as a parameter), and C# reads it — either by dereferencing the pointer in an
   `unsafe` context, or by copying it out with `Marshal.PtrToStructure<SharedState>(ptr)`.

The type sizes must line up exactly across the languages:

| Rust            | C#               | Size  |
| --------------- | ---------------- | ----- |
| `i32` / `u32`   | `int` / `uint`   | 4     |
| `f32` / `f64`   | `float` / `double` | 4 / 8 |
| `u8` / `i8`     | `byte` / `sbyte` | 1     |
| `*const T`/`*mut T` | `IntPtr` or `T*` | 8 (on 64-bit) |

Watch out for the usual traps: Rust's `bool` and `enum` need care (prefer an
explicit `u8`/`#[repr(i32)]`), and **a struct may not directly contain a Rust
`String`/`Vec` or a C# `string`/array** — those aren't blittable. To share a
list, put a **pointer + length** in the struct and read the elements through the
pointer.

> This is the same "keep both sides in lockstep" contract as §9, and the same
> generators from §6.5 (`csbindgen`, `interoptopus`) can produce the C# mirror
> struct from the Rust definition automatically, so the two can't drift apart.

---

## 7. The bridge has two directions

Our two sides need to call each other *both ways*, so we built two channels.

```
   RUST (engine)                                   C# (game)
   ─────────────                                   ─────────
                    (A) Rust calls C#
   Init / Update / Draw  ───────────────────────►  the game's per-frame logic

                    (B) C# calls Rust
   EngineApi: draw_circle, key_pressed, …  ◀──────  the game asks to draw / read input
```

### 7.1 Direction A — Rust calls C# (three entry points)

These are the three functions the host looked up in §5. Each is a C# method that
the engine invokes:

- **`Init(EngineApi* api)`** — called once at startup. C# saves the table of
  engine functions (channel B) and creates the game scene.
- **`Update(float dt)`** — called every frame. `dt` is the *delta time*: how many
  seconds passed since the previous frame. C# advances the gameplay.
- **`Draw()`** — called every frame, after `Update`. C# issues its draw calls.

They live in [game_cs/src/Interop.cs](../game_cs/src/Interop.cs). On the Rust
side, a small adapter named `ScriptedScene`
([flappy/src/main.rs](../flappy/src/main.rs)) plugs these into the engine's
normal game loop, so from the engine's perspective the C# game is just an
ordinary "scene."

### 7.2 Direction B — C# calls Rust (the `EngineApi` table)

The game can't draw or read input on its own. So at startup the host hands C# a
single struct, the **`EngineApi`**, which is a *list of function pointers* —
addresses of Rust functions the game may call. There's one for `draw_circle`,
one for `draw_text`, one for `key_pressed`, and so on. Each of those Rust
functions simply forwards to the real drawing/input library (macroquad).

You can think of `EngineApi` as a **remote control** the engine hands to the
game: "here are all the buttons you're allowed to press." We look at exactly
what a "function pointer" is next.

---

## 8. Function pointers, explained

A **function pointer** is one of the most important ideas here, so let's nail it.

We said the CPU runs machine code that lives somewhere in memory. A function is
just a *block of machine code at some address*. A **function pointer** is simply
**the address of that block** — a number that says "the code for this function
starts at memory location X."

If you have that address and you know the function's signature (its arguments
and return type, i.e. its slice of the ABI), you can **call it** — even if you
have no idea what language it was written in. You just hand the CPU the address
and the arguments, and say "go."

This is the entire mechanism behind both directions of our bridge:

- **Direction A:** when the host looked up `Update` in the C# assembly (§5), the
  runtime JIT-compiled it and gave back its *address*. Rust stores that address
  as an `extern "system" fn(f32)` and calls it like any function. The CPU jumps
  into freshly-minted machine code that came from C#.

- **Direction B:** the `EngineApi` struct is *nothing but a bundle of addresses*
  of Rust functions. C# receives the bundle and, to draw a circle, jumps to the
  `draw_circle` address with the right arguments.

So neither side needs to "understand" the other language. They just exchange
**addresses of machine code** plus an agreed **ABI** for how to call them. That's
FFI in one sentence.

Here is the actual table, abbreviated, from
[flappy/src/ffi.rs](../flappy/src/ffi.rs):

```rust
#[repr(C)]                                  // "lay this struct out the C way"
pub struct EngineApi {
    pub screen_width:  extern "C" fn() -> f32,
    pub key_pressed:   extern "C" fn(i32) -> i32,
    pub draw_circle:   extern "C" fn(f32, f32, f32, u32),
    pub draw_text:     extern "C" fn(*const u8, f32, f32, f32, u32),
    // … about twenty entries total …
    pub quit:          extern "C" fn(),
}
```

Each field is a function pointer. `extern "C"` means "this function uses the C
calling convention" — i.e. it speaks the shared ABI so C# can call it.

---

## 9. The ABI contract: keeping both sides identical

The `EngineApi` is a struct that is **created in Rust and read in C#**. For that
to work, both languages must agree on its exact **memory layout**: which field
sits at which byte offset, and how big each field is.

### 9.1 Why layout is normally *not* guaranteed

By default, both Rust and C# reserve the right to **reorder or pad** the fields
of a struct however they find most efficient. That's fine when only one language
touches the struct — but disastrous if Rust writes a struct one way and C# reads
it expecting another. The fields would be misaligned: C# might read the
`draw_text` address where Rust stored `draw_circle`, and the program would jump
to the wrong machine code and crash.

### 9.2 How we pin the layout down

Both sides explicitly opt into the **C layout**, which says "lay fields out in
declared order, no reordering":

- **Rust:** `#[repr(C)]` on the struct (see the snippet above).
- **C#:** `[StructLayout(LayoutKind.Sequential)]` on the matching struct in
  [game_cs/src/EngineApi.cs](../game_cs/src/EngineApi.cs):

  ```csharp
  [StructLayout(LayoutKind.Sequential)]
  public unsafe struct EngineApi
  {
      public delegate* unmanaged[Cdecl]<float> ScreenWidth;
      public delegate* unmanaged[Cdecl]<int, int> KeyPressed;
      public delegate* unmanaged[Cdecl]<float, float, float, uint, void> DrawCircle;
      public delegate* unmanaged[Cdecl]<byte*, float, float, float, uint, void> DrawText;
      // … same entries, same order …
      public delegate* unmanaged[Cdecl]<void> Quit;
  }
  ```

  Here `delegate* unmanaged[Cdecl]<…>` is simply C#'s syntax for "a pointer to a
  native function using the C (`Cdecl`) calling convention." The type arguments
  list the parameters with the **return type last** (`<float>` means "no args,
  returns float"; `<int, int>` means "takes an int, returns an int";
  `<…, void>` means "returns nothing").

### 9.3 The golden rule

> **The two `EngineApi` structs must stay in lockstep: same fields, same types,
> same order.** The field *order* is the contract. If you add, remove, or reorder
> a field on one side, you must mirror it on the other — otherwise the addresses
> line up wrong and the program jumps into the void.

This is the single most fragile part of any FFI bridge, which is why both files
carry comments pointing at each other.

---

## 10. `[UnmanagedCallersOnly]` and calling conventions

Two details make the C# entry points (Direction A) actually callable from native
Rust.

### 10.1 `[UnmanagedCallersOnly]`

Normally, calling a C# method involves the runtime doing managed bookkeeping
around the call. Native code can't participate in that. The attribute
`[UnmanagedCallersOnly]` tells the runtime:

> "Compile this method so it can be called **directly** by native code, with no
> managed wrapper — as if it were a plain C function."

You can see it in [game_cs/src/Interop.cs](../game_cs/src/Interop.cs):

```csharp
[UnmanagedCallersOnly]
public static void Update(float dt) { … }
```

Such methods come with restrictions, all of which we respect:

- They must be **`static`** (no object instance is passed across the boundary).
- Their parameters and return type must be **blittable** — plain data that needs
  no conversion (numbers and pointers; see §11). `float` and `EngineApi*` qualify.
- They must not let a C# **exception escape** across the boundary (more in §13).

### 10.2 Calling conventions: `cdecl`, `stdcall`, `extern "system"`

The **calling convention** is the fine print of the ABI: precisely who puts what
where, and who cleans up the stack afterward. Historically there were several
(`cdecl`, `stdcall`, …) and mismatching them corrupts the stack.

- On the Rust side the entry points are typed `extern "system"`, and the
  `EngineApi` functions are `extern "C"`.
- On the C# side the pointers are declared `Cdecl`, and `[UnmanagedCallersOnly]`
  defaults to the platform convention.

On 64-bit Windows (our target) this is a non-issue: there is effectively **one**
calling convention for 64-bit code, so `extern "C"`, `extern "system"`, and
`Cdecl` all describe the same thing. We're explicit anyway, because on other
platforms (or 32-bit) the distinction would matter.

---

## 11. Marshalling: what data can cross the boundary

Recall the rule from §6.4: only **plain data** can cross — numbers, pointers, and
C-layout structs of those. **Marshalling** is the act of converting richer values
into that plain form (and back). Here's every kind of data our bridge sends, and
how we keep it "plain."

### 11.1 Numbers — free

`float` (`f32`), `double` (`f64`), `int` (`i32`), `uint` (`u32`): these are
identical bit patterns on both sides. They pass straight through with zero
conversion. Most of our arguments (positions, sizes, timings) are just floats.

Types that pass with no conversion like this are called **blittable** ("can be
**bl**ock-transf**err**ed" — copied byte-for-byte).

### 11.2 Colors — packed into one `uint`

A color is four parts: red, green, blue, alpha (opacity), each 0–255. Rather than
pass four arguments (or a struct) everywhere, we pack all four into a single
32-bit integer in the pattern `0xRRGGBBAA`:

- **C# packs it** when constructing a `Color`
  ([game_cs/src/Primitives.cs](../game_cs/src/Primitives.cs)):

  ```csharp
  Rgba = ((uint)r << 24) | ((uint)g << 16) | ((uint)b << 8) | a;
  ```

- **Rust unpacks it** back into the engine's color type
  ([flappy/src/ffi.rs](../flappy/src/ffi.rs)):

  ```rust
  fn color(rgba: u32) -> Color {
      Color::from_rgba(
          ((rgba >> 24) & 0xff) as u8,   // red
          ((rgba >> 16) & 0xff) as u8,   // green
          ((rgba >>  8) & 0xff) as u8,   // blue
          ( rgba        & 0xff) as u8,   // alpha
      )
  }
  ```

A single `uint` is blittable, so this is both efficient and simple.

### 11.3 Keys and mouse buttons — small integers with an agreed meaning

C# can't hand Rust a C# enum directly, so we agree on a **numbering**. "Space" is
`1`, "Up" is `2`, "Escape" is `6`, and so on:

- C# side: the `Key` enum in [game_cs/src/Engine.cs](../game_cs/src/Engine.cs)
  assigns those numbers.
- Rust side: `map_key` in [flappy/src/ffi.rs](../flappy/src/ffi.rs) turns the
  number back into the engine's real key type.

It's the same idea as colors: encode a meaning as a plain integer, decode it on
the far side. (Booleans ride along the same way — Rust returns `0` or `1` as an
`i32`, and C# treats non-zero as `true`.)

### 11.4 Strings — the hard case, encoded as UTF-8 bytes

Text is the one genuinely tricky type, because a C# `string` is a managed object
the GC can move around in memory, and its internal format isn't what C expects.
A C-style string is just **a sequence of bytes ending in a zero byte** (the
"null terminator"). So to draw text we marshal manually:

1. **C# encodes** the string into UTF-8 bytes and appends a `0`
   ([game_cs/src/Engine.cs](../game_cs/src/Engine.cs)):

   ```csharp
   public static void DrawText(string text, float x, float y, float size, Color c)
   {
       fixed (byte* p = Utf8(text))         // 'fixed' pins the bytes in place
           _api.DrawText(p, x, y, size, c.Rgba);
   }
   ```

   The `fixed` keyword is essential: it tells the garbage collector "do **not**
   move this byte array while I'm handing its address to native code," for the
   duration of the call. Without it, the GC could relocate the array mid-call and
   Rust would read garbage.

2. **Rust reads** the bytes up to the null terminator and interprets them as a
   string ([flappy/src/ffi.rs](../flappy/src/ffi.rs)):

   ```rust
   unsafe fn str_from<'a>(ptr: *const u8) -> &'a str {
       CStr::from_ptr(ptr as *const i8).to_str().unwrap_or("")
   }
   ```

This is the textbook example of marshalling: a rich managed object (`string`) is
converted into plain data (a pointer to null-terminated bytes) to make the trip,
because plain data is all the boundary can carry.

---

## 12. A single frame, step by step

Let's tie everything together by following one frame of gameplay from the moment
you launch the program.

**Startup (happens once):**

1. You run `flappy.exe`. The OS loads the **native Rust** code and starts it.
2. Rust opens the game window (via macroquad).
3. Rust **boots the .NET runtime** and loads `game_cs.dll` (§5).
4. Rust looks up the addresses of `Init`, `Update`, `Draw` (§5, §8).
5. Rust builds the `EngineApi` table of its own function addresses and calls
   **`Init(&api)`**. Inside `Init`, C# copies the table into a static field and
   creates the `FlappyScene`. The game is now alive.

**Every frame (repeats ~60 times per second):**

```
   ┌──────────────────────────────────────────────────────────────────────┐
   │ Rust game loop (mini_engine)                                           │
   │                                                                        │
   │  1. measure dt (seconds since last frame)                              │
   │                                                                        │
   │  2. call C#  Update(dt) ───────────────────────────────────────────┐  │
   │                                                                     │  │
   │      C#: bird.Vel += gravity * dt;  move pipes;  check collisions   │  │
   │          to read input, C# calls back ──► Rust key_pressed(1)  ◀────┼──┤
   │          (Rust asks macroquad, returns 0/1)                         │  │
   │     ◀───────────────────────────────────────────────────────────────  │
   │                                                                        │
   │  3. clear the screen                                                   │
   │                                                                        │
   │  4. call C#  Draw() ────────────────────────────────────────────────┐ │
   │                                                                      │ │
   │      C#: for each thing, call back ──► Rust draw_circle/draw_text ◀──┼─┤
   │          (Rust forwards each to macroquad → GPU)                     │ │
   │     ◀──────────────────────────────────────────────────────────────── │
   │                                                                        │
   │  5. present the finished frame to the window                          │
   │                                                                        │
   │  6. if C# called quit(), stop; else loop back to 1                     │
   └──────────────────────────────────────────────────────────────────────┘
```

Notice the **constant back-and-forth**: Rust calls into C# (Direction A) for the
frame's logic and drawing, and *while inside that C# code*, C# repeatedly calls
back into Rust (Direction B) to read input and request drawing. Every one of
those arrows is a function-pointer call across the C ABI. Dozens of crossings
happen each frame, smoothly, because both sides honor the same contract.

---

## 13. The dangerous bits: memory, lifetimes, threads, exceptions

FFI removes the safety nets each language normally provides. Here are the real
hazards and how this project stays safe — useful to understand because they
explain several otherwise-odd choices in the code.

### 13.1 Lifetimes: don't hand out an address that later vanishes

When Rust passes `&api` (the address of the `EngineApi`) into C#, that data must
stay valid for as long as C# might use it. In our code, `api` lives in `main` for
the entire run, **and** C# immediately *copies* the whole table into its own
static field (`Engine.Bind` does `_api = *api;`). After that copy, C# no longer
depends on Rust's original at all. Two layers of safety.

The function *pointers* inside the table are addresses of compiled functions,
which exist for the whole life of the program — so they never dangle.

### 13.2 The GC can move memory — so we pin when needed

The .NET garbage collector may relocate managed objects to compact memory. If we
gave native code the address of a managed object and the GC moved it, that
address would suddenly point at the wrong place. That's why string marshalling
uses `fixed` (§11.4) to **pin** the bytes for the duration of the call. We never
hand Rust a long-lived pointer into managed memory.

### 13.3 Threads: everything happens on one thread

Drawing libraries and runtimes can be picky about threads. We sidestep all of it:
the Rust loop calls `Update`/`Draw` on the **same thread** that owns the window,
and C#'s callbacks run on that same thread. No data is shared across threads, so
there are no race conditions to reason about.

The one tiny piece of cross-cutting state — the "should we quit?" flag set by
C#'s `quit()` and read by the loop — uses an **atomic** boolean
(`AtomicBool` in [flappy/src/ffi.rs](../flappy/src/ffi.rs)) so it's safe
regardless.

### 13.4 Exceptions must not escape C#

If a C# method throws an exception and lets it propagate *out* across the native
boundary, the runtime can't unwind through foreign machine code and the process
dies hard. So each entry point wraps its body in `try/catch`
([game_cs/src/Interop.cs](../game_cs/src/Interop.cs)):

```csharp
[UnmanagedCallersOnly]
public static void Update(float dt)
{
    try { _scene?.Update(dt); }
    catch (Exception e) { Console.Error.WriteLine($"[game_cs] Update failed: {e}"); }
}
```

A bug in the game prints an error instead of taking down the whole program.

### 13.5 Why this is "unsafe" code

Both sides mark these regions specially: Rust uses `unsafe` blocks, C# uses the
`unsafe` keyword and `AllowUnsafeBlocks`. That's the languages saying: *"I can no
longer verify this for you — you, the programmer, are promising the ABI matches
and the pointers are valid."* The compiler trusts us; the §9 golden rule is the
promise we're making.

---

## 14. How to add a new engine feature

Putting the theory to work — say you want C# to be able to draw an ellipse. The
recipe follows directly from §9:

1. **Rust:** write an `extern "C"` function in
   [flappy/src/ffi.rs](../flappy/src/ffi.rs) that forwards to macroquad, and add
   a matching field to the `EngineApi` struct (and wire it up in
   `EngineApi::new`).
2. **C#:** add a `delegate* unmanaged[Cdecl]<…>` field to the `EngineApi` struct
   in [game_cs/src/EngineApi.cs](../game_cs/src/EngineApi.cs) **in the exact same
   position**.
3. **C#:** expose a friendly method on the `Engine` facade in
   [game_cs/src/Engine.cs](../game_cs/src/Engine.cs) so game code can call it
   nicely.

The cardinal rule, once more: **the two `EngineApi` structs must remain identical
in field order and types.** Add to the *end* of both, and you can't go wrong.

---

## 15. Just-in-time compilation

We touched on JIT back in §3 and §4. This section goes deeper and answers the
practical questions that usually follow: what the runtime costs us, what it's
responsible for, and whether we could get rid of it entirely.

### 15.1 Is C# JIT-compiled?

**By default, yes.** As described in §3.3, the C# compiler produces **IL**, and
the .NET runtime **JIT-compiles** that IL into machine code at runtime — each
method is compiled the first time it's actually called, then cached for the rest
of the run. That's exactly what happens in our project: when Rust first calls the
C# `Update` method, the runtime compiles it to machine code on the spot, and
every later call reuses that compiled code.

But "JIT" is only the *default*. .NET actually offers a spectrum of compilation
strategies, and it's worth knowing they exist:

| Strategy            | When IL → machine code happens     | Needs the runtime at run time? |
| ------------------- | ---------------------------------- | ------------------------------ |
| **Pure JIT**        | at runtime, on first call          | yes (this is what we use)      |
| **Tiered JIT**      | quick code first, re-optimized hot paths later | yes               |
| **ReadyToRun (R2R)**| partly at build time, rest JIT'd   | yes                            |
| **Native AOT**      | entirely at build time             | **no** (see §15.5)             |

Modern .NET uses **tiered compilation** by default: it JITs a method quickly at
first (fast to compile, less optimized), and if that method turns out to be
"hot" (called a lot), it silently recompiles it with full optimizations. This
gives both fast startup and fast steady-state code.

### 15.2 Does the runtime add performance / memory overhead?

Yes — every managed runtime has costs. For a small game like ours they're
negligible, but it's important to know what they are:

**Startup cost.** Booting the runtime and JIT-compiling the first calls takes
time — typically tens to a few hundred milliseconds. You may notice the very
first frame doing a touch more work as methods get compiled on first use. After
that warm-up, it's gone.

**Memory footprint.** A managed process carries baggage a native one doesn't:

- the runtime's own `.dll`s loaded into memory,
- the **GC heap** (managed objects live here, plus room the GC keeps in reserve),
- the **JIT code cache** (the machine code it generated),
- type metadata for every loaded type.

In practice a minimal .NET app uses on the order of *tens of megabytes* of RAM
before your game allocates anything. A native Rust program might use a fraction
of that.

**Steady-state (per-frame) cost.** Three things to be aware of:

- **JIT'd code is fast** — after warm-up it runs at roughly native speed, often
  within a few percent of equivalent C++. This is usually *not* a problem.
- **Garbage collection** is the one that matters for games. When the GC runs to
  reclaim memory, it can briefly pause your code. An unlucky pause in the middle
  of a frame shows up as a stutter (a dropped frame). The fix is to avoid
  allocating garbage every frame (reuse objects, avoid per-frame `new`), which is
  standard game-dev practice in C#/Unity.
- **Crossing the FFI boundary** (§6–§11) has a tiny per-call cost — the managed
  ↔ native transition, plus any marshalling (e.g. encoding a string). For the
  handful-to-hundreds of calls we make per frame it's immeasurable, but a million
  tiny calls per frame would add up.

#### On-disk size: the runtime floor (and how small you *can* get)

There's one more "overhead" that surprises people coming from Rust or C++:
**binary size**. Even with **Native AOT** (§15.5) — which removes the JIT and the
need for an installed runtime — a C# program has a **size floor** that a Rust
program does not.

The reason is that Native AOT must **statically bake the runtime services into
the binary**: the garbage collector, type metadata, exception-handling
machinery, and the slices of the base class library your code touches. None of
that can be fully removed while still being "normal" C#, so it forms a fixed
baseline cost — present even in a "hello world." Rust links its standard library
too, but it has no GC and no managed metadata, and it dead-code-eliminates
aggressively, so its floor is far lower.

Rough on-disk sizes for a *trivial* program (release, stripped/trimmed):

| Build                              | Typical size | Standalone? |
| ---------------------------------- | ------------ | ----------- |
| **Rust**, size-optimized           | ~50–300 KB   | yes         |
| **Rust**, default release          | ~0.3–1 MB    | yes         |
| **C# Native AOT**, fully trimmed    | ~1.5–3.5 MB  | yes         |
| **C# framework-dependent** (`.dll`) | tens of KB   | **no** — needs an installed runtime (§15.6) |

So if your goal is a **single self-contained binary under ~0.5 MB** (which is
easy in Rust), standard .NET tooling **cannot reach it**: the Native AOT floor is
~1.5 MB+, and self-contained-with-runtime is tens of MB. The framework-dependent
`.dll` *looks* tiny, but only because the ~100 MB+ runtime lives elsewhere on the
machine — the footprint is hidden, not gone.

**The edge case — `bflat`.** A community AOT compiler called
[`bflat`](https://flattened.net/) (built on the same Native AOT stack) can go
much smaller. In its extreme **"zerolib"** mode — **no GC, no reflection, no
exception data, no base class library** — it can produce .NET native binaries in
the *hundreds of KB*, occasionally near the Rust range. The catch is that you
give up almost everything that makes C# pleasant: no `string` conveniences, no
collections, no `System.*`, no GC. At that point you're writing painful,
stripped-down C# to match a size Rust gives you for free — which usually defeats
the reason to pick C# as the scripting language in the first place.

> **Takeaway:** the size floor isn't a tooling deficiency you can optimize away —
> it's the cost of the very runtime services (GC, metadata, reflection) that
> *are* the point of a managed language. Pick C# for productivity and safety and
> budget a few MB; pick Rust when a tiny, dependency-free binary is the hard
> requirement.


### 15.3 What is the runtime actually responsible for?

The .NET runtime is a lot more than "a JIT." Its responsibilities include:

- **JIT compilation** — turning IL into machine code (§3.3).
- **Memory management / garbage collection** — allocating objects and
  automatically freeing the ones you no longer use, so you never `free()` by hand.
- **The type system & metadata** — knowing the shape of every type, enabling
  **reflection** (inspecting types at runtime), generics, etc.
- **Assembly loading & resolution** — finding and loading `.dll`s (it's what
  loaded `game_cs.dll` for us, in its own isolated load context).
- **Exception handling** — the machinery behind `try/catch` and stack unwinding.
- **Threads & the thread pool** — scheduling managed threads, `async`/`await`,
  `Task`s.
- **Interop / marshalling** — the plumbing for `P/Invoke` and
  `[UnmanagedCallersOnly]` that lets managed and native code call each other
  (i.e. the very mechanism this whole document is about).
- **The base class library (BCL)** — `System.*`: strings, collections, math,
  files, networking, and thousands of other built-in types.

When you hear "C# is managed," *this list* is what's doing the managing.

### 15.4 Pros and cons of relying on a runtime

**Pros**

- **Productivity & safety** — automatic memory management eliminates whole
  classes of bugs (use-after-free, leaks, double-free). You write gameplay, not
  memory bookkeeping.
- **A huge standard library and ecosystem** — collections, JSON, math, async, and
  NuGet packages, all available to the game.
- **Portability** — the same IL runs on any OS/CPU that has a .NET runtime
  (§15.6); the runtime hides the hardware differences.
- **Reflection & metaprogramming** — the runtime keeps full type information, so
  tools, serializers, and editors can inspect and drive your code.
- **Adaptive optimization & hot reload** — because compilation happens at
  runtime, the JIT can re-optimize hot paths, and code can even be swapped while
  running (the basis of hot-reload workflows).

**Cons**

- **You need the runtime present** — it must be shipped or installed (§15.6).
- **Startup latency & memory footprint** — §15.2.
- **GC pauses** — non-deterministic pauses are the classic enemy of smooth frame
  times; they require discipline to avoid.
- **Less predictable performance** — JIT warm-up and GC make timing less
  deterministic than ahead-of-time native code.
- **Interop friction** — every managed ↔ native crossing needs the care described
  in §13 (pinning, no escaping exceptions, ABI matching).

This is the classic **managed vs. native** trade-off: you swap some control and
predictability for a large gain in safety and productivity. For the *game*
(scripting) layer that's a great deal; for the *engine* layer we deliberately
chose native Rust instead.

### 15.5 Can C# be compiled to static machine code for the final build?

**Yes — with Native AOT** ("Ahead-Of-Time"), available since .NET 7. Native AOT
compiles your C# **all the way to a native binary at build time**, with no IL and
no JIT left at run time. The necessary runtime services (GC, type system) are
*statically linked into the binary itself*. The result is an ordinary native
library/executable — much like what Rust or C++ produces.

What you gain: no runtime to install, fast startup (no JIT warm-up), smaller
memory footprint, and a single self-contained file.

What you give up: anything that depends on generating code at runtime. Native AOT
**restricts reflection**, forbids runtime IL generation (`Reflection.Emit`), and
some libraries that rely on those won't work. Build times are longer, and you
compile separately per OS/CPU.

There are also intermediate options:

- **ReadyToRun (R2R)** — precompiles IL to native *at build time* but keeps the
  full runtime and JIT around as a fallback. Faster startup, still needs the
  runtime.
- **IL2CPP** — Unity's approach: it translates IL into C++ and then compiles that
  natively. Same spirit as Native AOT, different machinery.

### 15.6 Shipping the runtime: install, bundle, or eliminate?

This is really the question "what does a user need on their machine to run the
game?" .NET gives three **deployment models**:

| Model                  | Runtime must be installed on the OS? | Output size | Per-OS build? |
| ---------------------- | ------------------------------------ | ----------- | ------------- |
| **Framework-dependent**| **Yes** — a matching .NET runtime    | small (just your `.dll`s) | no (portable IL) |
| **Self-contained**     | No — the runtime is bundled in       | large (+~70 MB) | yes |
| **Native AOT**         | No — no runtime exists at all (§15.5)| medium native binary | yes |

1. **Framework-dependent** (what this project does today). The build output is
   just your IL `.dll`s; they rely on a .NET runtime already installed on the
   machine. This is why running our game requires the **.NET 10 runtime to be
   installed** — `hostfxr` finds that installed runtime and uses it. Smallest to
   distribute, but the user (or installer) must have the right .NET present.

2. **Self-contained.** You bundle a private copy of the runtime *with* your game
   (a folder, or even a single packed file). Nothing needs to be pre-installed —
   it "just runs" — at the cost of a much larger download (the runtime is tens of
   megabytes). You produce one build per target platform.

3. **Native AOT.** No runtime at all; the game *is* a native binary (§15.5).
   Nothing to install, smallest dependency surface, but the most build-time
   restrictions.

**Does each OS need the .NET runtime, and is Linux/macOS supported?**

.NET is **fully cross-platform**: it runs on **Windows, Linux, and macOS**
(including Apple Silicon / arm64), plus mobile and more. So:

- With **framework-dependent** deployment, *each* machine needs a .NET runtime
  installed for *its* OS and CPU architecture. A Windows player needs the Windows
  runtime, a Linux player the Linux one, and so on.
- With **self-contained** or **Native AOT**, the user needs *nothing* installed —
  but **you** must produce a separate build for each target, identified by its
  **RID** (Runtime IDentifier, e.g. `win-x64`, `linux-x64`, `osx-arm64`). The IL
  is portable; a *bundled* or *AOT-compiled* runtime is not — it's native to one
  OS+architecture.

In short: the IL your C# compiles to is OS-independent, but *running* it always
requires a runtime for the specific OS+CPU — whether that runtime is
pre-installed (model 1), shipped alongside (model 2), or baked in (model 3).

---

## 16. Memory ownership and hot reload

So far we've treated the two sides as cooperating peers. But they each have their
*own* memory, managed in completely different ways, and that has big consequences
— especially once you want to **hot-reload** the game (rebuild the C# while the
engine keeps running). This section works through who owns what, and what breaks
when the game's `.dll` is swapped underneath a live engine.

### 16.1 Which side owns which memory?

There are **two separate heaps** in our process, with two different managers:

- **The native heap (Rust side).** Everything Rust allocates — the window, GPU
  buffers, macroquad's internals, the `EngineApi` table — lives here and is freed
  by Rust, deterministically, when it goes out of scope. No garbage collector is
  involved.
- **The managed heap (C# side).** Every C# object — `Bird`, `Pipe`,
  `FlappyScene`, the lists, the strings — lives in the **.NET GC heap** and is
  freed automatically by the garbage collector (§15) when nothing references it.

The golden rule of cross-language memory is:

> **Whoever allocates, frees — in their own allocator.** Rust never frees a C#
> (GC) object; C# never frees a Rust allocation. The two heaps don't mix.

What actually crosses the boundary is therefore *not* shared ownership of an
allocation — it's either **copies of plain data** or a **pointer whose owner
stays responsible for it**:

- When C# calls `draw_circle(x, y, r, color)`, it passes *copies* of a few
  numbers. Rust reads them and keeps nothing. Nothing is owned across the line.
- When Rust passes `&api` into `Init`, C# immediately **copies** the table into
  its own field (§13.1). After that, neither side holds a pointer into the
  other's memory.
- The string case (§11.4) is the one temporary exception: C# hands Rust a pointer
  into managed memory *for the duration of one call only*, and **pins** it
  (`fixed`) so the GC can't move it meanwhile. The instant the call returns, that
  borrow is over.

### 16.2 Can game data live on the *engine* side, with only logic in C#?

**Yes — and it's a powerful pattern.** You can keep the authoritative game state
in **engine-owned (native) memory** and let C# be purely the *logic* that reads
and writes it. The data never lives on the GC heap at all.

Two ways to wire it up:

1. **A shared `#[repr(C)]` struct.** Rust allocates, say, a `GameState` (or an
   array of entity/component data) and hands C# a pointer to it. C# declares a
   matching `[StructLayout(Sequential)]` struct and reads/writes the fields
   directly through that pointer (`unsafe`). This is the §9 ABI contract again,
   now applied to *data* instead of function tables.
2. **Accessor functions.** Add getters/setters to the `EngineApi`
   (`get_bird_y()`, `set_bird_y(v)`, …). C# never sees the raw memory; it pokes
   it through function calls. Safer, but chattier.

Why you'd want this:

- **No GC pressure.** Data that isn't on the managed heap can't trigger or
  lengthen GC pauses (§15.2) — the data-oriented approach big engines prefer.
- **The engine controls the memory layout** (cache-friendly arrays, etc.).
- **It survives hot reload** — because the data lives in the engine, which is
  *not* the thing being reloaded (this is the key to §16.5).

The cost: every access crosses the FFI boundary or uses an `unsafe` pointer
dereference, and you give up C#'s safety and convenience *for that data*. You're
trading ergonomics for control and reload-survivability.

> This is essentially how Unity's **DOTS/ECS** works: component data lives in
> native engine-owned arrays, and C# "systems" are logic that operates over that
> data by handle — exactly "data on the engine side, logic on the game side."

### 16.3 How does Unity's garbage collector work between game and engine?

Unity is the same two-heap picture and it's a useful model
to study:

- **The engine is native C++** and manages its own objects (meshes, textures, the
  real `Transform` data, …) **manually** — they are *not* garbage collected. They
  live and die by Unity's own lifecycle (e.g. `Destroy()`).
- **Your scripts are C#** running on a runtime (Mono, or **IL2CPP** — IL
  translated to C++ then compiled native) **with a garbage collector**. The GC
  manages *only* the managed heap: your script objects, arrays, strings,
  closures, boxed values.

The link between them is a **thin managed wrapper over a native object**. A C#
`GameObject` is a small managed shell that holds a handle/pointer to the heavy
native object. So:

- The GC can collect the lightweight C# wrapper, but it **never touches the native
  data** behind it — that's the engine's job.
- This is the source of Unity's infamous **"fake null"**: if the native object is
  `Destroy()`-ed but a C# wrapper still references it, Unity overrides `==` so the
  wrapper *compares equal to null* even though the managed object still exists.
  It's the two memory worlds disagreeing about lifetime.

Two more practical notes:

- **IL2CPP / AOT does not remove the GC.** Compiling ahead-of-time (§15.5) changes
  how code is compiled, not how managed *memory* is managed — there's still a GC
  for managed objects.
- Unity's GC was historically **non-compacting and stop-the-world** (frame
  spikes); they later added an **incremental** mode that spreads collection across
  frames. This is why "don't allocate per frame" is gospel in Unity — the same
  advice from §15.2.

Mapping it back to us: our **Rust engine = Unity's native side**, our **C# game =
Unity scripts**. We simply expose far fewer engine objects across the line.

### 16.4 Hot reload: the function pointers change when the `.dll` is rebuilt

The central problem. Recall (§5, §8) that the engine
calls C# through **cached function pointers** — the addresses of the JIT-compiled
`Init`/`Update`/`Draw`. If you rebuild the game and load the new assembly, the
runtime JIT-compiles its methods at **new addresses**. The old pointers the
engine is still holding now point at code that's been thrown away — calling them
is a crash.

So naive hot reload (just load a new `.dll`) cannot work. Two pieces are needed:

**(a) The assembly must be *unloadable*.** The simple hostfxr "load assembly and
get function pointer" path we use today loads the game and never lets go. To
reload, you instead load the game into a **collectible `AssemblyLoadContext`
(ALC)** — .NET's unit of "a set of loaded assemblies you can later throw away."
To reload: unload the old ALC (its types and JIT'd code are collected), create a
fresh ALC, load the rebuilt `.dll` into it.

**(b) The engine must call through a level of indirection, not a raw cached
pointer.** The standard trick is a **two-assembly split**:

```
   ENGINE (Rust)                STABLE LOADER (C#, never reloaded)        GAME (C#, reloaded)
   ─────────────                ──────────────────────────────────       ───────────────────
   cached pointer ──► Loader.Update()  ──(forwards to current game)──►    Game.Update()
   (NEVER changes)              holds a collectible ALC + a reference      lives inside the ALC;
                                to the current Game instance              swapped on reload
```

- A small **loader assembly** is loaded *once* and never reloads. The engine
  caches *its* `Init`/`Update`/`Draw` addresses — and those never move.
- The loader owns the collectible ALC and a reference to the *current* game
  object. Its `Update` just forwards to `currentGame.Update()`.
- On reload, the loader unloads the old ALC, loads the rebuilt game into a new
  one, creates a new game instance, and points its internal reference at it.

The engine's pointers stay valid forever because they point at the *stable
loader*; only the loader's private reference to the game changes. The "pointers
change" problem disappears behind that indirection.

### 16.5 Keeping game state alive across a recompile

Unloading the game assembly (§16.4) destroys **everything inside it**: its static
fields and all the GC objects in its ALC. So by default, rebuilding the game
`.dll` wipes the bird's position, the pipes, and the score. To preserve state
across a reload you must keep it somewhere that *isn't* unloaded. Three options:

1. **Keep the authoritative state on the engine side (best fit for us).** This is
   §16.2 put to work: if the bird/pipes/score live in engine-owned native memory,
   reloading the C# *logic* leaves the *data* completely untouched. The new game
   code simply re-attaches to the same state and continues. Bonus: it also cuts GC
   pressure (§16.3). This is the cleanest answer to your question.
2. **Serialize across the reload.** Before unloading, the game serializes its
   state into a plain blob (bytes/JSON) held by the engine or the stable loader;
   after the new assembly loads, it deserializes the blob back into the new types.
   Only serializable fields survive — live references, open file handles, threads,
   and closures do not.
3. **Store state in a separate, never-reloaded assembly** that both the old and new
   game versions share, and hand it to the new instance on reload.

> **What Unity does** is option 2: on script recompile it performs a *domain
> reload* — it serializes the serializable fields of your `MonoBehaviour`s, tears
> down and rebuilds the managed domain, then restores those fields. (Newer Unity
> lets you *disable* domain reload for faster play-mode entry, at the price of
> having to reset `static` state yourself — a direct demonstration that statics
> don't survive a reload unless something outside the reload preserves them.)

---

## 17. Glossary

- **ABI (Application Binary Interface)** — the binary contract for how compiled
  code passes arguments, returns values, and lays out data. The thing both sides
  must agree on.
- **Assembly** — .NET's term for a compiled `.dll` (or `.exe`) of managed code.
- **AssemblyLoadContext (ALC)** — a .NET container for a set of loaded assemblies;
  a *collectible* ALC can be unloaded as a unit, which is what makes hot reload
  possible.
- **Domain reload** — Unity's process of serializing managed state, tearing down
  and rebuilding the managed environment after a script recompile, then restoring
  that state.
- **Hot reload** — rebuilding and swapping the game's code while the engine keeps
  running, without restarting the process.
- **Blittable** — a type whose bytes mean the same thing on both sides, so it can
  be copied across the boundary with no conversion (numbers, pointers).
- **Calling convention** — the fine print of the ABI: which registers/stack slots
  carry arguments and who cleans up. (`cdecl`, `stdcall`, …)
- **CLR (Common Language Runtime)** — the engine inside the .NET runtime that
  JIT-compiles and runs managed code.
- **FFI (Foreign Function Interface)** — calling functions across a language
  boundary by agreeing on the C ABI.
- **Function pointer** — the memory address of a function's machine code; enough
  to call it, regardless of source language.
- **GC (Garbage Collector)** — the .NET subsystem that automatically frees unused
  objects (and may move them in memory).
- **hostfxr** — Microsoft's native library that selects and starts the right .NET
  runtime; how we host the runtime from Rust.
- **IL (Intermediate Language)** — the CPU-independent instructions a C# compiler
  produces; the runtime JIT-compiles IL into machine code.
- **JIT (Just-In-Time) compilation** — translating IL into machine code at
  runtime, on first use.
- **Machine code** — the raw numeric instructions a CPU executes directly.
- **Managed code** — code that runs under a runtime (C#); contrast **native**.
- **Marshalling** — converting rich values (like strings) into plain data that
  can cross an FFI boundary, and back.
- **Native AOT (Ahead-Of-Time)** — compiling C# all the way to a native binary at
  build time, with the runtime statically linked in, so no separate .NET runtime
  or JIT is needed at run time.
- **Native code** — machine code that runs directly on the CPU with no runtime
  underneath (Rust, C, C++).
- **Pinning** — temporarily forbidding the GC from moving a managed object so its
  address can be safely shared with native code (`fixed` in C#).
- **P/Invoke (Platform Invoke)** — .NET's classic mechanism for calling native C
  functions via `[DllImport]`, which includes an automatic marshaller for many
  common non-blittable types.
- **Binding generator** — a tool (e.g. `csbindgen`, `interoptopus`, `cbindgen`)
  that auto-generates the FFI glue and matching type declarations between two
  languages so they can't drift out of sync.
- **RID (Runtime IDentifier)** — a tag like `win-x64`, `linux-x64`, or `osx-arm64`
  naming a specific OS + CPU target; self-contained and AOT builds are produced
  per RID.
- **Tiered compilation** — the JIT strategy of compiling a method quickly at first
  and re-optimizing it later if it turns out to be "hot."
- **`[UnmanagedCallersOnly]`** — a C# attribute marking a method as directly
  callable by native code, like a plain C function.
- **`#[repr(C)]` / `[StructLayout(Sequential)]`** — directives that force a struct
  into predictable C-style memory layout so both languages agree on it.
- **Runtime hosting** — starting a language runtime (here, .NET) from inside
  another program rather than via its normal launcher.

---

## 18. Further reading

- .NET hosting (the official C API we wrap):
  <https://learn.microsoft.com/dotnet/core/tutorials/netcore-hosting>
- `[UnmanagedCallersOnly]`:
  <https://learn.microsoft.com/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute>
- The `netcorehost` Rust crate we use:
  <https://crates.io/crates/netcorehost>
- Rust FFI ("calling code in other languages"):
  <https://doc.rust-lang.org/nomicon/ffi.html>
- C# function pointers (`delegate* unmanaged`):
  <https://learn.microsoft.com/dotnet/csharp/language-reference/unsafe-code#function-pointers>

---

*This document describes the bridge in this repository. The two ends of the
contract are [flappy/src/ffi.rs](../flappy/src/ffi.rs) (Rust) and
[game_cs/src/EngineApi.cs](../game_cs/src/EngineApi.cs) (C#). When you change one,
change the other.*

# TO CLARIFY

- Can we have intellisense while editing game code?
- How can we add more than one scripting language? For example javascript.
- How we can maximally reduce C# DLL generation time?
- Can we even use c# debugger if data is stored on engine side?
- Can we make c# part completely safe like rust?
- What if C# uses threading?
- On the game side we define components. In rust game code this is done via macros, so created and compiled to the artifact. However, how to do the same when C# is the scripting language for the game?