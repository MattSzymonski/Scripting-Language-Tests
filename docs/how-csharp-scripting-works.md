# Scripting a Native Engine with a Managed Language — A Compendium

This is a from-first-principles guide to a single, widely used architecture: a
**native engine** (written in a compiled language like Rust, C, or C++) that runs
**game or application logic written in a managed language** (like C# on .NET). It
explains, assuming no prior knowledge, how two programs written in different
languages and produced by different compilers end up calling each other's
functions inside one process.

Every term is defined the first time it appears. By the end you should understand:

- why so many systems deliberately mix a fast native layer with an easy scripting
  layer;
- the difference between a **native** program and a **managed** one, and what a
  language **runtime** actually does;
- what **FFI** ("Foreign Function Interface") is, and why the **C ABI** is the key
  that makes cross-language calls possible;
- how data crosses the boundary (**marshalling**), how a native program **hosts**
  a managed runtime, and what one frame of execution looks like end to end;
- the hazards (memory, threads, exceptions), how safe the managed side can be,
  who owns which memory, and how **hot reload** works;
- the runtime's costs (performance, memory, binary size), and the developer
  experience: tooling, debugging, and supporting more than one scripting language.

Throughout, we use **Rust (engine) + C# on .NET (script)** as the running concrete
example because it shows the mechanics most clearly, but the ideas apply to any
native↔managed pairing — C++ + Lua, C + Python, a game engine + JavaScript, and so
on. Sections 1–3 set up the idea; 4–9 build the bridge; 10–12 cover doing it
safely; 13–15 cover memory and lifecycle; 16–17 cover runtime characteristics; and
18–20 cover the day-to-day developer experience.

---

## Table of contents

1. [Why split a program into engine and script](#1-why-split-a-program-into-engine-and-script)
2. [Two kinds of program: native and managed](#2-two-kinds-of-program-native-and-managed)
3. [The managed runtime](#3-the-managed-runtime)
4. [FFI: calling across a language boundary](#4-ffi-calling-across-a-language-boundary)
5. [Function pointers and the two-way bridge](#5-function-pointers-and-the-two-way-bridge)
6. [The binary contract: layout, conventions, and entry points](#6-the-binary-contract-layout-conventions-and-entry-points)
7. [Marshalling: moving data across](#7-marshalling-moving-data-across)
8. [Hosting a runtime from native code](#8-hosting-a-runtime-from-native-code)
9. [Anatomy of a frame](#9-anatomy-of-a-frame)
10. [Hazards: lifetimes, memory, threads, exceptions](#10-hazards-lifetimes-memory-threads-exceptions)
11. [Threading](#11-threading)
12. [How safe can the managed side be?](#12-how-safe-can-the-managed-side-be)
13. [Two heaps: who owns which memory](#13-two-heaps-who-owns-which-memory)
14. [Hot reload](#14-hot-reload)
15. [Defining script-side components and data](#15-defining-script-side-components-and-data)
16. [The runtime's cost: performance, memory, and size](#16-the-runtimes-cost-performance-memory-and-size)
17. [Compilation strategies and deployment](#17-compilation-strategies-and-deployment)
18. [Editor tooling and IntelliSense](#18-editor-tooling-and-intellisense)
19. [Debugging](#19-debugging)
20. [Supporting multiple scripting languages](#20-supporting-multiple-scripting-languages)
21. [Glossary](#21-glossary)
22. [Further reading](#22-further-reading)

---

## 1. Why split a program into engine and script

Picture the finished program as **one** running process. Inside it, two worlds
coexist:

```
   ┌───────────────────────── one process ─────────────────────────┐
   │                                                                │
   │   NATIVE ENGINE  (the host)          MANAGED SCRIPT (the game) │
   │   ─────────────────────────          ──────────────────────── │
   │   • owns the window & main loop      • holds the gameplay      │
   │   • talks to the GPU / OS   ── calls ──▶  logic and rules      │
   │   • fast, low-level                  • decides what happens     │
   │                             ◀── calls ──  each frame            │
   │                                      • asks the engine to draw  │
   │                                                                │
   └────────────────────────────────────────────────────────────────┘
```

- The **engine** owns the window, the main loop, and the hardware. It knows how to
  actually put pixels on screen but has no opinion about *the game*.
- The **script** contains the logic — movement, collisions, scoring, AI — but
  cannot draw or read input by itself. It *asks* the engine to do those things.

This split is called the **engine + scripting** pattern, and almost every
commercial game engine uses some version of it:

| Layer      | Language         | Changes how often? | Optimizes for…                       |
| ---------- | ---------------- | ------------------ | ------------------------------------ |
| **Engine** | fast/low-level   | rarely             | performance, GPU, memory, the window |
| **Script** | easy/high-level  | constantly         | iteration speed, expressiveness, fun |

- Unity's engine is C++; you write your game in **C#**.
- Unreal's engine is C++; you write gameplay in C++ or **Blueprints**.
- Godot's engine is C++; you write your game in **GDScript** or C#.

The engine is the stable, high-performance foundation you change rarely. The
script is the part you rewrite ten times a day while figuring out what works.
Keeping them in different languages lets each layer use the best tool for its job —
a systems language where speed and control matter, and a productive managed
language where iteration speed matters.

The rest of this document is about the hard part: **how the two sides call each
other's functions** despite being different languages compiled by different
compilers. The enabling technique is **FFI**, and everything below is a slow walk
toward fully understanding it.

---

## 2. Two kinds of program: native and managed

To understand the bridge, you first have to see that native and managed programs
*run in fundamentally different ways*.

### 2.1 What a CPU actually runs

A CPU understands exactly one thing: **machine code** — long sequences of tiny
numeric instructions like "add these two numbers," "copy this value here," "jump
to that location." Machine code is specific to a *kind* of CPU (e.g. "x64" desktop
processors, "arm64" phones and Apple Silicon).

No CPU understands a source language like Rust or C# directly. Something must
translate source into machine code. The **when** and **how** of that translation
is the whole difference between native and managed.

### 2.2 Native: compiled ahead of time to machine code

When you compile a language like Rust, C, or C++, the compiler turns your source
into **machine code** right then, on the build machine, and writes it into an
executable. When you run that executable, the CPU executes those instructions
directly.

This is a **native** program: the file *is* machine code, ready to go. Nothing
sits underneath it at run time. That's why native code is fast and has no "runtime
requirement" — the OS loads the executable and jumps straight in.

### 2.3 Managed: compiled to an intermediate language, run by a runtime

A managed language like C# (on .NET) or Java (on the JVM) works in **two stages**:

1. The compiler does **not** produce machine code. It produces an **intermediate
   language** — for .NET that's **IL** (also called CIL/MSIL); for the JVM it's
   *bytecode*. IL is a compact, CPU-independent "half-way" instruction set: not
   source, but not machine code either. It's stored in a compiled module (a `.dll`
   for .NET).

2. To actually run, that IL needs a program called a **runtime**. The runtime
   reads the IL and, *while the program runs*, translates it into machine code on
   the fly. This last-second translation is **JIT compilation** ("Just-In-Time").

We call such a language **managed** because the runtime *manages* execution: it
JIT-compiles the IL, automatically frees unused memory (the **garbage collector**,
or **GC**), checks array bounds, enforces type safety, and more.

### 2.4 The consequence

```
   NATIVE                                MANAGED
   ──────────────                        ─────────────────────────────
   source ─► [compiler] ─► machine code  source ─► [compiler] ─► IL
                            │                                     │
                            ▼                                     ▼
                      CPU runs it                         runtime JIT-compiles
                      directly                            IL ─► machine code, runs
```

A managed module is **not** something the CPU can run on its own; it is more like
a recipe that only the runtime knows how to cook. This single fact drives the
central challenge of this document: **before native code can call any managed
code, it must first start a runtime** to bring that managed code to life. That is
the subject of §8, but first we need to understand the runtime itself.

---

## 3. The managed runtime

A **managed runtime** (for .NET, its engine is historically the **CLR**, the
*Common Language Runtime*) is itself a native program — a set of libraries that
ship with the platform SDK. Its job is to *host and run managed code*. Concretely
it:

- **loads** IL modules into memory,
- **JIT-compiles** IL methods into machine code the first time each is called,
- provides **garbage collection** (frees objects you stop using),
- provides a large **standard library** (strings, collections, math, files,
  networking…),
- enforces **type and memory safety** for managed code,
- keeps **type metadata** enabling **reflection** (inspecting types at run time),
  generics, and interop.

### 3.1 The normal way managed code starts

Normally you never think about the runtime because the tooling hides it. When you
launch a managed app, a small launcher finds the right runtime on the machine,
starts it, points it at the program's entry method (`Main`), and off it goes. The
runtime is the *host*; your code is the *guest*.

> **Mental model:** normally the runtime is the *building* and your app is a
> *tenant*. When a native engine hosts a script, we do the reverse — the native
> executable is the building, and it invites the runtime in as a tenant so the
> managed code can live inside it (§8).

### 3.2 Why this matters for embedding

Because the runtime is what turns IL into running code, embedding a scripting
language into a native engine is fundamentally about **standing up a runtime
inside a process that didn't start as a managed app** — there is no `Main`, and
the process began life as native code. Everything from JIT to GC to the standard
library comes along for the ride once that runtime exists. The costs of carrying
it (startup, memory, GC pauses, binary size) are covered in §16–§17; the mechanics
of starting it are in §8. First, the calls themselves.

---

## 4. FFI: calling across a language boundary

**FFI** stands for **Foreign Function Interface**: the general name for code
written in one language calling functions written in (or compiled by) a different
language. FFI needs rules — it isn't automatic — and the reason is subtle, so we
go slowly.

### 4.1 Languages don't "talk"; CPUs run instructions

When one function calls another, here's what *actually* happens at the machine
level:

- The caller puts the **arguments** in agreed-upon places — some CPU **registers**
  (tiny named storage slots in the CPU) and/or the **stack** (a scratch region of
  memory).
- It records a **return address** (where to continue afterward).
- It **jumps** to the callee's machine code.
- The callee reads its arguments from those agreed-upon places, does its work,
  puts the **return value** in an agreed-upon place, and jumps back.

Notice the recurring phrase **"agreed-upon."** Which register holds the first
argument? Who cleans up the stack afterward? How is an integer passed versus a
float? None of that is universal — it's a *convention* both sides must follow.

### 4.2 The ABI: the contract both sides obey

That set of conventions is the **ABI** — the *Application Binary Interface*. It
answers questions like:

- In what order and in which registers/stack slots do arguments go?
- How are return values handed back?
- How big is an `int`? A pointer? How is a struct laid out in memory?
- Who cleans up the stack afterward? (This last part is the **calling
  convention**.)

If two pieces of compiled code **agree on the ABI**, it literally does not matter
what source language each was written in. Machine code calling machine code only
cares that everyone followed the same contract.

### 4.3 The lingua franca: the "C ABI"

Over decades, the **C language's ABI** became the universal meeting point. Nearly
every language can "speak C" at its boundaries: Rust can mark functions
`extern "C"`, C# can expose and call functions using C conventions, and Python,
Go, Swift, and others can too. The C ABI is the *Esperanto* of compiled code.

So the industry-standard strategy — and the one this whole document assumes — is:

> **The two languages never speak "their own language" to each other. At the
> boundary they both speak the C ABI.** That shared contract is what makes the
> calls work.

This choice has a powerful consequence for scaling. If you route everything
through the C ABI as a common hub, each language only needs to know how to convert
**to and from C-shaped plain data** — it never has to understand any *other*
language directly. That turns an **N × N** problem (a custom bridge for every
language *pair*) into an **N** problem (each language ↔ C). It's why a single
engine can host C#, Lua, JavaScript, and more without a bespoke bridge per pair
(§20).

### 4.4 The catch: only "plain data" crosses

Because the boundary is just "machine code following the C ABI," only simple,
predictable data can pass through it:

- **numbers** (`int`, `float`, `double`…) — fine, they're just bits;
- **pointers** (memory addresses) — fine, an address is just a number;
- **plain structs** of the above — fine, *if both sides agree on the layout* (§6).

What **cannot** cross directly are each language's rich, managed objects: a
managed `string` or `List<T>`, a native `String` or `Vec<T>`. Those have internal
representations the other side knows nothing about. To send such things across,
you must first convert them into plain data — a conversion called **marshalling**
(§7).

Keep this rule in mind; it explains almost every design choice at the boundary.

---

## 5. Function pointers and the two-way bridge

### 5.1 What a function pointer is

The CPU runs machine code that lives somewhere in memory. A function is just a
*block of machine code at some address*. A **function pointer** is simply **the
address of that block** — a number that says "the code for this function starts at
memory location X."

If you have that address and you know the function's **signature** (its arguments
and return type — its slice of the ABI), you can **call it**, even with no idea
what language it was written in. You hand the CPU the address and the arguments and
say "go." This is the entire mechanism behind cross-language calls: both sides
exchange **addresses of machine code** plus an agreed **ABI** for how to call them.
That's FFI in one sentence.

### 5.2 The bridge has two directions

An engine and a script need to call each other *both ways*, so there are two
channels:

```
   ENGINE (native)                                SCRIPT (managed)
   ───────────────                                ────────────────
                     (A) engine calls script
   Init / Update / Draw  ─────────────────────►   the per-frame game logic

                     (B) script calls engine
   draw / input / … functions  ◀───────────────   the game asks to draw / read input
```

**Direction A — the engine calls the script (entry points).** At startup, the host
looks up a handful of managed functions *by name* and stores their addresses.
Typical entry points for a game are:

- **`Init`** — called once at startup. The script saves the table of engine
  functions (direction B) and creates its initial state.
- **`Update(dt)`** — called every frame. `dt` is the *delta time*: seconds since
  the previous frame. The script advances the gameplay.
- **`Draw`** — called every frame after `Update`. The script issues its draw
  requests.

When the host looks these up, the runtime JIT-compiles each and returns its
*address*. The native side stores that address as a typed function pointer and
calls it like any function; the CPU jumps into freshly minted machine code that
came from the managed language.

**Direction B — the script calls the engine (an API table).** The script cannot
draw or read input on its own, so at startup the host hands it a single struct — an
**API table** we'll call `EngineApi` — which is a *list of function pointers*:
addresses of native functions the script may call (`draw_circle`, `draw_text`,
`key_pressed`, and so on). Each of those native functions simply forwards to the
real rendering/OS library underneath.

Think of `EngineApi` as a **remote control** the engine hands the script: "here are
all the buttons you're allowed to press." A first sketch, in Rust:

```rust
#[repr(C)]                                  // "lay this struct out the C way" (§6)
pub struct EngineApi {
    pub screen_width:  extern "C" fn() -> f32,
    pub key_pressed:   extern "C" fn(i32) -> i32,
    pub draw_circle:   extern "C" fn(f32, f32, f32, u32),
    pub draw_text:     extern "C" fn(*const u8, f32, f32, f32, u32),
    // … as many entries as the engine chooses to expose …
    pub quit:          extern "C" fn(),
}
```

Each field is a function pointer. `extern "C"` means "this function uses the C
calling convention" — it speaks the shared ABI so the managed side can call it.
Neither side needs to *understand* the other language; they just trade addresses
and agree on how to call them. The one thing they must agree on precisely is the
*layout* of that struct — the subject of the next section.

---

## 6. The binary contract: layout, conventions, and entry points

The API table is **created on the native side and read on the managed side**. For
that to work, both languages must agree on its exact **memory layout** (which field
sits at which byte offset, how big each field is), the **calling convention** of
each function, and how the managed entry points are **exported** so native code can
call them. These three agreements are the fragile heart of any FFI bridge.

### 6.1 Struct layout is normally *not* guaranteed

By default, most compiled languages reserve the right to **reorder or pad** the
fields of a struct however they find most efficient. That's fine when only one
language touches the struct — but disastrous if one side writes it one way and the
other reads it expecting a different arrangement. The fields would be misaligned:
the reader might fetch the `draw_text` address where the writer stored
`draw_circle`, then jump to the wrong machine code and crash.

So both sides must explicitly opt into the **C layout** — "lay fields out in
declared order, no reordering":

- **Native (Rust):** `#[repr(C)]` on the struct (as in §5).
- **Managed (C#):** `[StructLayout(LayoutKind.Sequential)]` on the mirror struct:

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

  Here `delegate* unmanaged[Cdecl]<…>` is C#'s syntax for "a pointer to a native
  function using the C (`Cdecl`) calling convention." The type arguments list the
  parameters with the **return type last** (`<float>` = "no args, returns float";
  `<int, int>` = "takes an int, returns an int"; `<…, void>` = "returns nothing").

### 6.2 The golden rule

> **The two struct definitions must stay in lockstep: same fields, same types,
> same order.** The field *order* is the contract. If you add, remove, or reorder
> a field on one side, you must mirror it on the other — otherwise the addresses
> line up wrong and the program jumps into the void.

This is the single most fragile part of any FFI bridge. In practice you keep the
two definitions physically next to each other in your mind (comments cross-linking
the files help), or — better — you **generate** one from the other (§7.5) so they
*cannot* drift apart.

**Adding to the API in practice.** Growing the surface is a mechanical three-step
that follows directly from the golden rule:

1. **Native:** write a new `extern "C"` function that forwards to the underlying
   library, and add a matching field to the API struct.
2. **Managed:** add a `delegate* unmanaged[Cdecl]<…>` field to the mirror struct
   **in the exact same position**.
3. **Managed:** expose a friendly wrapper method so script code calls it nicely
   (and marshals any rich arguments — §7).

Add to the *end* of both structs and you can't misalign the existing entries.

### 6.3 Exporting a managed method to native code

Direction A (§5) needs the managed entry points to be callable *as if they were
plain C functions*. Normally, calling a managed method involves the runtime doing
bookkeeping around the call that native code can't participate in. The fix is an
attribute that tells the runtime to compile the method for direct native calls —
in .NET, `[UnmanagedCallersOnly]`:

```csharp
[UnmanagedCallersOnly]
public static void Update(float dt) { /* … */ }
```

Such methods come with restrictions (which mirror the "plain data only" rule of
§4.4):

- They must be **`static`** — no object instance is passed across the boundary.
- Their parameters and return type must be **blittable** — plain data needing no
  conversion (numbers and pointers; §7.1). A `float` or a pointer qualifies; a
  managed `string` does not.
- They must not let an **exception escape** across the boundary (§10.4).

### 6.4 Calling conventions

The **calling convention** is the fine print of the ABI: precisely who puts what
where, and who cleans up the stack afterward. Historically several existed
(`cdecl`, `stdcall`, …), and mismatching them corrupts the stack. In our example
the native functions are `extern "C"`, the entry points use the platform default,
and the managed pointers are declared `Cdecl`.

On modern 64-bit targets this is largely a non-issue: there is effectively **one**
calling convention for 64-bit code on a given OS, so `extern "C"`, the platform
default, and `Cdecl` describe the same thing. It's still worth being explicit,
because on other platforms (or 32-bit) the distinction matters.

### 6.5 The same contract applies to shared data structs

The layout agreement isn't only for function tables. A very common need is for the
**native side to define a data struct that the managed side reads** (e.g. a block
of shared state, §13.2). It works by the exact same rule, applied to data:

1. **Native fixes the layout** with `#[repr(C)]`:

   ```rust
   #[repr(C)]
   pub struct SharedState {
       pub position_y: f32,
       pub score:      u32,
       pub alive:      u8,   // a 1-byte int, not a native `bool`, to be safe
   }
   ```

2. **Managed declares a mirror** with `[StructLayout(LayoutKind.Sequential)]`,
   **same fields, same order, matching types**:

   ```csharp
   [StructLayout(LayoutKind.Sequential)]
   public struct SharedState
   {
       public float PositionY;
       public uint  Score;
       public byte  Alive;
   }
   ```

3. **Native hands over a pointer**, and managed code reads it — by dereferencing in
   an `unsafe` context, or by copying it out (in .NET,
   `Marshal.PtrToStructure<SharedState>(ptr)`).

Type sizes must line up exactly across the languages:

| Native (Rust)       | Managed (C#)       | Size (bytes)  |
| ------------------- | ------------------ | ------------- |
| `i32` / `u32`       | `int` / `uint`     | 4             |
| `f32` / `f64`       | `float` / `double` | 4 / 8         |
| `u8` / `i8`         | `byte` / `sbyte`   | 1             |
| `*const T`/`*mut T` | `IntPtr` or `T*`   | 8 (on 64-bit) |

Watch the usual traps: native `bool` and `enum` need care (prefer an explicit
1-byte int or a fixed-size discriminant), and **a shared struct may not directly
contain a native `String`/`Vec` or a managed `string`/array** — those aren't
blittable. To share a list, put a **pointer + length** in the struct and read the
elements through the pointer.

---

## 7. Marshalling: moving data across

Recall the rule from §4.4: only **plain data** crosses — numbers, pointers, and
C-layout structs of those. **Marshalling** is the act of converting richer values
into that plain form (and back). Here are the common cases and how each stays
"plain."

### 7.1 Numbers — free

`float`, `double`, `int`, `uint` are identical bit patterns on both sides. They
pass straight through with zero conversion. Most engine arguments (positions,
sizes, timings) are just numbers.

Types that pass with no conversion like this are called **blittable** ("can be
**bl**ock-transf**err**ed" — copied byte-for-byte).

### 7.2 Packing several values into one number

Sometimes it's cleaner to pack multiple small values into one blittable number
than to pass many arguments or a struct. A classic example is a color: four parts
(red, green, blue, alpha), each 0–255, packed into a single 32-bit integer in the
pattern `0xRRGGBBAA`.

- **The managed side packs it:**

  ```csharp
  uint rgba = ((uint)r << 24) | ((uint)g << 16) | ((uint)b << 8) | a;
  ```

- **The native side unpacks it:**

  ```rust
  fn unpack(rgba: u32) -> (u8, u8, u8, u8) {
      (((rgba >> 24) & 0xff) as u8,   // red
       ((rgba >> 16) & 0xff) as u8,   // green
       ((rgba >>  8) & 0xff) as u8,   // blue
       ( rgba        & 0xff) as u8)   // alpha
  }
  ```

A single `uint` is blittable, so this is both efficient and simple.

### 7.3 Enums and booleans — small integers with an agreed meaning

One side can't hand the other a native enum value directly, so you agree on a
**numbering**: e.g. "Space = 1, Up = 2, Escape = 6." One side maps its enum to the
number; the other maps the number back to its own enum. Booleans ride along the
same way — return `0` or `1` as an integer, and treat non-zero as `true`. It's the
same idea as packing: encode a meaning as a plain integer, decode it on the far
side.

### 7.4 Strings — the genuinely hard case

Text is the one tricky type, because a managed `string` is an object the GC can
move around in memory, and its internal format (often UTF-16) isn't what C expects.
A C-style string is just **a sequence of bytes ending in a zero byte** (the "null
terminator"). So text is usually marshalled by hand:

1. **The managed side encodes** the string to UTF-8 bytes with a trailing `0`, and
   **pins** it so the GC can't move it during the call:

   ```csharp
   public static unsafe void DrawText(string text, float x, float y, float size, uint rgba)
   {
       fixed (byte* p = Utf8(text))     // 'fixed' pins the bytes in place
           _api.DrawText(p, x, y, size, rgba);
   }
   ```

   The `fixed` keyword is essential: it tells the garbage collector "do **not**
   move this byte array while I hand its address to native code," for the duration
   of the call. Without it, the GC could relocate the array mid-call and the native
   side would read garbage.

2. **The native side reads** the bytes up to the null terminator and interprets
   them as a string:

   ```rust
   unsafe fn str_from<'a>(ptr: *const u8) -> &'a str {
       std::ffi::CStr::from_ptr(ptr as *const i8).to_str().unwrap_or("")
   }
   ```

This is the textbook example of marshalling: a rich managed object is converted
into plain data (a pointer to null-terminated bytes) to make the trip, because
plain data is all the boundary can carry.

### 7.5 Who writes the marshalling — and do tools help?

For **blittable** data there's *nothing to write*: the bytes already mean the same
thing on both sides. For **non-blittable** data (strings, arrays, collections),
*someone* has to convert. Who depends on the interop mechanism:

- **Raw function pointers + an "unmanaged callers only" export** (the bare-metal
  path shown above) do **no** automatic conversion — only blittable types are even
  allowed across, so you marshal the rest by hand. It's a few lines and fully under
  your control.
- **Classic P/Invoke (`[DllImport]`)** ships an **automatic marshaller**: annotate
  parameters and the runtime converts common types (strings, arrays, some structs)
  for you. The newer **`[LibraryImport]`** source generator does the same but emits
  the conversion code at *compile* time, for speed and AOT-friendliness (§17).

Either way, the *concept* is identical — turn rich values into C-ABI plain data.
And you needn't do it all by hand:

- **Binding generators** read one side's definitions and generate the other side's
  glue automatically — e.g. tools that read native `extern` functions and emit the
  matching managed declarations, or that describe an API once and emit bindings for
  several languages. These eliminate the manual, error-prone work of keeping both
  sides in sync (the §6.2 problem).
- **Language-agnostic serialization** (e.g. FlatBuffers, Protocol Buffers, Cap'n
  Proto) defines a schema once and generates readers/writers in *many* languages;
  you pass a byte buffer across and each side decodes it. Heavier than a raw struct,
  but it sidesteps ABI matching entirely and works for any language pair.

A tiny bridge is worth hand-writing for clarity; a large one is usually generated.

---

## 8. Hosting a runtime from native code

Everything so far assumed the managed code is already running. But recall §2.4: a
managed module is inert until a runtime brings it to life, and in this architecture
the process started as *native* code — there is no managed `Main` to launch. So the
native engine must **host the runtime**: from native code, boot up the managed
runtime and get it to run specific managed methods.

Platforms ship a native library for exactly this. On .NET it's **`hostfxr`** ("host
framework resolver"), and there are friendly wrappers for calling it from native
languages. The pieces involved:

- **The host library (`hostfxr`)** decides *which* installed runtime to use and
  starts it.
- **A runtime-config file** built next to the managed module tells the host which
  runtime version the code targets, so the right one is selected. (A normal app
  gets this for free; a *library* being embedded usually has to opt into generating
  it.)

In plain English, the host does this once at startup:

1. **Find and load the host library.**
2. **Initialize a runtime** for the config file. At this moment a live managed
   runtime exists *inside the native process*.
3. **Get a loader for the compiled module** (an *assembly*, in .NET terms) — this
   loads the script (in its own isolated load context, which matters for hot
   reload, §14) and provides a way to look up functions in it.
4. **Look up the entry points by name** — `Init`, `Update`, `Draw` — from a named
   type in the assembly.

The result of step 4 is a set of **function pointers**: the addresses of the
JIT-compiled managed methods. From the native side they're now just callable
functions:

```rust
type InitFn   = extern "system" fn(*const EngineApi);
type UpdateFn = extern "system" fn(f32);
type DrawFn   = extern "system" fn();
```

Once the host holds those addresses, the runtime mostly fades into the background.
Calling `update(dt)` from native code now runs managed code. **That** is the moment
the two worlds connect — and it's exactly the direction-A half of the bridge from
§5, now wired up.

---

## 9. Anatomy of a frame

Tie it all together by following execution from launch through one frame.

**Startup (happens once):**

1. The OS loads the **native** engine executable and starts it.
2. The engine opens its window and initializes its rendering/OS backend.
3. The engine **boots the managed runtime** and loads the script assembly (§8).
4. The engine looks up the addresses of `Init`, `Update`, `Draw` (§5, §8).
5. The engine builds the `EngineApi` table of its own function addresses and calls
   **`Init(&api)`**. Inside `Init`, the script copies the table into a field of its
   own and creates its initial game state. The game is now alive.

**Every frame (repeats many times per second):**

```
   ┌──────────────────────────────────────────────────────────────────────┐
   │ native game loop                                                       │
   │                                                                        │
   │  1. measure dt (seconds since last frame)                              │
   │                                                                        │
   │  2. call script  Update(dt) ───────────────────────────────────────┐  │
   │                                                                     │  │
   │      script: advance state; move entities; check collisions         │  │
   │              to read input, script calls back ─► engine key_pressed ┼──┤
   │              (engine asks the OS, returns 0/1)                      │  │
   │     ◀───────────────────────────────────────────────────────────────  │
   │                                                                        │
   │  3. clear the screen                                                   │
   │                                                                        │
   │  4. call script  Draw() ────────────────────────────────────────────┐ │
   │                                                                      │ │
   │      script: for each thing, call back ─► engine draw_circle/text ◀──┼─┤
   │              (engine forwards each to the renderer → GPU)           │ │
   │     ◀──────────────────────────────────────────────────────────────── │
   │                                                                        │
   │  5. present the finished frame to the window                          │
   │                                                                        │
   │  6. if the script asked to quit, stop; else loop back to 1             │
   └──────────────────────────────────────────────────────────────────────┘
```

Notice the **constant back-and-forth**: the engine calls into the script
(direction A) for the frame's logic and drawing, and *while inside that script
code*, the script repeatedly calls back into the engine (direction B) to read input
and request drawing. Every one of those arrows is a function-pointer call across
the C ABI. Dozens of crossings happen each frame, smoothly, because both sides
honor the same contract.

---

## 10. Hazards: lifetimes, memory, threads, exceptions

FFI removes the safety nets each language normally provides. Here are the real
hazards and the standard ways to stay safe — worth understanding because they
explain several otherwise-odd design choices at the boundary.

### 10.1 Lifetimes: don't hand out an address that later vanishes

When native code passes the address of a struct (like `&api`) into the script, that
data must stay valid for as long as the script might use it. The robust pattern is:
keep the original alive for the whole run **and** have the script immediately
*copy* what it needs into its own storage. After that copy, the script no longer
depends on the original at all — two layers of safety. Function pointers inside the
table are addresses of compiled functions, which exist for the whole life of the
program, so they never dangle.

### 10.2 The GC can move memory — so pin when needed

A garbage collector may relocate managed objects to compact memory. If native code
holds the address of a managed object and the GC moves it, that address suddenly
points at the wrong place. That's why string marshalling **pins** the bytes for the
duration of the call (§7.4). The rule of thumb: **never hand native code a
long-lived pointer into managed memory** — copy across, or pin only for the length
of a single call.

### 10.3 Threads: keep the boundary single-threaded

Rendering libraries and runtimes are often picky about threads. The simplest safe
design calls the script's `Update`/`Draw` on the **same thread** that owns the
window, and lets the script's callbacks run on that same thread. If no data is
shared across threads, there are no race conditions to reason about. Any genuinely
cross-thread flag (like "should we quit?") should use an **atomic** so it's safe
regardless. Threading gets a section of its own (§11), because it's the most common
way people get into trouble.

### 10.4 Exceptions must not escape the managed side

If a managed method throws and lets the exception propagate *out* across the native
boundary, the runtime can't unwind through foreign machine code and the process
dies hard. So each entry point must **catch at the boundary**:

```csharp
[UnmanagedCallersOnly]
public static void Update(float dt)
{
    try { _scene?.Update(dt); }
    catch (Exception e) { Console.Error.WriteLine($"Update failed: {e}"); }
}
```

A bug in the script then prints an error instead of taking down the whole process.

### 10.5 Why this is "unsafe" code

Both sides mark these regions specially — native code uses `unsafe` blocks, C# uses
the `unsafe` keyword. That's the languages saying: *"I can no longer verify this for
you; you, the programmer, are promising the ABI matches and the pointers are
valid."* The compiler trusts you; the golden rule of §6.2 is the promise you're
making.

---

## 11. Threading

Section 10.3 assumed a single-threaded boundary. But managed languages are fully
multithreaded — what happens if the script *does* spawn threads?

The short version: **the script can freely use threads for computation, but every
call back into the engine must stay on the main thread.** Breaking that rule is the
fast path to a crash. Here's the full picture.

### 11.1 The runtime supports threads — all of them

Once the runtime is hosted, the script has the entire concurrency toolbox: a thread
pool, tasks, `async`/`await`, raw threads, parallel loops, concurrent collections.
Being hosted inside a native process disables none of it — the runtime schedules
managed threads just as it would in a normal app.

### 11.2 The golden rule: the engine is single-threaded

The engine's API functions forward to a rendering/OS backend, and — like almost
every graphics/windowing library — that backend is **thread-affine**: the GPU
context, the window's event loop, and the input state all belong to **the one
thread that owns the window**. They are not synchronized for concurrent use.

> **All engine calls — drawing, input, quitting — must happen on the main thread,
> inside `Update`/`Draw`. Calling an engine function from a background thread is
> undefined behavior and will typically crash.**

This isn't a limitation of any particular scripting language; it's that the engine
side is single-threaded by design, and the restriction applies to any language
calling in.

### 11.3 The pattern: compute off-thread, touch the engine on-thread

The workable split is the one engines use everywhere:

- **Background threads do pure computation** that doesn't touch the engine —
  pathfinding, AI, procedural generation, asset loading, simulation over managed
  data.
- **The main thread owns all engine interaction.** `Update`/`Draw` reads the
  results the background work produced and issues the actual engine calls.

Since the engine offers no built-in "run this on the main thread" dispatcher, you
build a tiny one — a thread-safe queue drained each frame:

```csharp
static readonly ConcurrentQueue<Action> _mainThreadWork = new();

// background thread: hand results back instead of calling the engine directly
_mainThreadWork.Enqueue(() => Engine.DrawCircle(x, y, r, color));

// in Update(dt), on the main thread:
while (_mainThreadWork.TryDequeue(out var job)) job();
```

This is exactly the "you may only call engine APIs from the main thread" rule
(familiar from Unity) plus a main-thread dispatcher.

### 11.4 You lose the free safety that single-threading gave you

A single-threaded boundary gets data-race freedom *for free* by never sharing
across threads. The moment you introduce real threads over shared mutable state,
that guarantee is gone — and managed languages do **not** statically prevent data
races the way Rust's `Send`/`Sync` do (§12.4). Protecting shared state is now your
responsibility: locks, atomics, `volatile`, concurrent collections, or **immutable
snapshots** passed between threads.

A memory-model nuance worth knowing: a managed data race won't corrupt the
runtime's *own* heap — reference and properly-aligned word-sized reads/writes are
atomic — but **larger structs can tear** (a half-updated value), and you can read
stale or reordered data. The failure mode is logic bugs and torn values, not the
heap corruption a C/C++ race would cause. **Shared *native* state is stricter
still:** if data lives in engine-owned memory (§13.2) and several threads poke it
through raw pointers, none of the managed comforts apply — synchronize explicitly,
or keep that data main-thread-only.

### 11.5 Sharp edges specific to an embedded runtime

Three traps come from the process having **no normal message loop**:

- **`async`/`await` resumes on the wrong thread.** A normal UI app installs a
  synchronization context that bounces `await` continuations back to a captured
  thread. An embedded host usually has none, so continuations resume on thread-pool
  threads. Code that touches the engine *after* an `await` will likely run
  off-thread and crash. Marshal back explicitly (via the §11.3 queue) before any
  engine call.
- **Background-thread exceptions bypass your boundary guard.** The `try/catch`
  around `Update`/`Draw` only protects the main thread. An unhandled exception on a
  worker thread will, by default, tear down the whole process. Wrap background work
  in its own `try/catch` and surface the error back to the main thread.
- **Don't block the main thread on background work.** Waiting synchronously on a
  task inside `Update` stalls the frame and risks deadlock. Kick off the work and
  **poll** for completion in a later frame.

> **Bottom line:** yes, the script can use threads — offloading heavy *pure
> computation* is a good idea — but keep all engine interaction on the main thread,
> communicate results back through a thread-safe queue or atomics, and remember
> that data-race safety is now on you.

---

## 12. How safe can the managed side be?

A tempting goal: if the native language is prized for compile-time safety, can the
script be *equally* safe? The honest answer needs the word "safe" broken apart,
because it bundles several different guarantees — and managed languages win some
outright while genuinely trailing on others.

### 12.1 Managed code is *already* memory-safe

The bugs a borrow checker exists to prevent — use-after-free, double-free, dangling
pointers, buffer overflows — **simply do not occur in ordinary (non-`unsafe`)
managed code.** The runtime guarantees it by different means:

- the **GC** frees objects only when nothing references them (no use-after-free, no
  double-free),
- **array/collection bounds are checked** (no buffer overrun),
- there is no manual `free()` to get wrong.

A native language achieves this *at compile time* via ownership; a managed language
achieves it *at run time* via the GC and checks. Different mechanism, same result
**for the managed heap**. So in the pure memory-safety dimension, everyday script
logic is already as safe as safe native code — arguably easier to write.

### 12.2 The unsafe part is the bridge — and the native side has one too

The place the managed side suspends its guarantees is exactly the FFI bridge (§10):
raw pointers, pinning, function-pointer tables, dereferencing shared structs. That
code is genuinely unsafe.

But notice the **symmetry**: the native engine side is `unsafe` at that same
boundary too — it calls C-ABI function pointers, trusts the ABI, and reads raw
pointers. **Neither language makes FFI safe.** Both switch off their guarantees at
the seam and ask the programmer to uphold the contract by hand. So "make it
completely safe like the native language" is slightly a misframing: *the native
language isn't fully safe across an FFI boundary either.* The realistic, achievable
target is the one idiomatic native code aims for: **shrink `unsafe` to a tiny,
audited core and make everything else provably safe.**

### 12.3 Getting the script's own code to 100% safe

The strategy is to encapsulate `unsafe` behind a safe API:

- **Confine unsafe to a thin facade.** Script code calls friendly wrappers
  (`Engine.DrawCircle(...)`) that are ordinary safe code; the pointer work lives
  inside the facade. Gameplay authors never type `unsafe`.
- **Structurally forbid unsafe in the gameplay module.** Split the code so the
  *bridge* module allows unsafe while the *game-logic* module disables it — then the
  compiler physically rejects any `unsafe` in gameplay code. A hard, structural
  guarantee, not a convention.
- **Generate the bridge (§7.5)** to remove hand-written pointer code and eliminate
  the layout drift that is the main way unsafe code goes wrong.
- **Prefer the automatic-marshalling path** where you don't need raw pointers, so
  there's less pointer code to get right.

The result is precisely the safety profile idiomatic native code uses at an FFI
boundary: **100% safe application code sitting on a small, generated-or-audited
unsafe core.**

### 12.4 Two places managed code stays weaker

Being honest about where the analogy breaks:

- **Null.** Some native languages have no null; managed references historically
  can be null. Opt-in compile-time null analysis (e.g. C#'s nullable reference
  types) recovers most of this by flagging possible-null uses — but it's *advisory
  analysis*, not an ironclad type-system guarantee.
- **Data races.** This is the real gap. Rust *statically* prevents data races via
  `Send`/`Sync` + the borrow checker; managed languages have **no such compile-time
  protection** (§11.4). You dodge it with a single-thread model, or handle it with
  runtime discipline (locks, atomics, immutability). Even so, a managed race causes
  torn values and logic bugs, not the heap corruption a C/C++ race can — so it's
  still "safer than C/C++," just not "proven-safe like Rust."

> **Bottom line:** for memory safety, **yes** — script logic can be 100% safe, with
> `unsafe` confined to a tiny bridge, which is exactly how idiomatic native code
> treats an FFI boundary. The residual differences are null (largely closed by
> nullable-reference analysis) and data races (closed in practice by a
> single-thread model, but never *statically* guaranteed).

---

## 13. Two heaps: who owns which memory

The two sides are cooperating peers, but they each have their *own* memory, managed
in completely different ways. Getting ownership straight is what keeps the bridge
from leaking or crashing — and it's the key to hot reload (§14).

### 13.1 Which side owns which memory

There are **two separate heaps** in the process, with two different managers:

- **The native heap.** Everything the engine allocates — the window, GPU buffers,
  the backend's internals, the API table — lives here and is freed by the engine,
  deterministically, when it goes out of scope. No garbage collector involved.
- **The managed heap.** Every script object — entities, lists, strings — lives in
  the GC heap and is freed automatically by the garbage collector when nothing
  references it.

The golden rule of cross-language memory:

> **Whoever allocates, frees — in their own allocator.** The engine never frees a
> managed (GC) object; the script never frees a native allocation. The two heaps
> don't mix.

So what crosses the boundary is *not* shared ownership of an allocation — it's
either **copies of plain data** or a **pointer whose owner stays responsible for
it**:

- A call like `draw_circle(x, y, r, color)` passes *copies* of a few numbers; the
  engine reads them and keeps nothing.
- When the engine passes its API table into `Init`, the script **copies** it into
  its own field (§10.1); afterward neither side holds a pointer into the other's
  memory.
- Strings (§7.4) are the one temporary exception: the script hands the engine a
  pointer into managed memory *for one call only*, pinned so the GC can't move it.
  The instant the call returns, that borrow is over.

### 13.2 Keeping game data on the engine side

You can go further and keep the **authoritative game state in engine-owned (native)
memory**, letting the script be purely the *logic* that reads and writes it. The
data never lives on the GC heap at all. Two ways to wire it up:

1. **A shared `#[repr(C)]` struct** (or array of them). The engine allocates the
   state and hands over a pointer; the script declares a matching mirror struct and
   reads/writes fields directly through the pointer (§6.5). This is the layout
   contract applied to data.
2. **Accessor functions.** Add getters/setters to the API (`get_x()`, `set_x(v)`).
   The script never sees raw memory; it pokes state through function calls. Safer,
   but chattier.

Why you'd want this:

- **No GC pressure.** Data off the managed heap can't trigger or lengthen GC pauses
  (§16) — the data-oriented approach big engines prefer.
- **The engine controls the memory layout** (cache-friendly arrays, etc.).
- **It survives hot reload** — because the data lives in the engine, which is *not*
  the thing being reloaded (§14).

The cost: every access crosses the boundary or uses an `unsafe` pointer
dereference, and you give up the managed language's safety and convenience *for
that data*. You trade ergonomics for control and reload-survivability. This is
essentially how Unity's **DOTS/ECS** works: component data lives in native
engine-owned arrays, and C# "systems" are logic that operates over that data by
handle.

### 13.3 Case study: how a big engine's GC interacts with native objects

Unity is the same two-heap picture at scale, and a useful model to study:

- **The engine is native C++** and manages its own objects (meshes, textures, the
  real transform data) **manually** — they are *not* garbage collected; they live
  and die by the engine's own lifecycle (e.g. `Destroy()`).
- **Scripts are C#** on a runtime **with a garbage collector**. The GC manages
  *only* the managed heap: script objects, arrays, strings, closures.

The link between them is a **thin managed wrapper over a native object** — a small
managed shell holding a handle/pointer to the heavy native object. So:

- The GC can collect the lightweight wrapper, but it **never touches the native
  data** behind it — that's the engine's job.
- This is the source of Unity's infamous **"fake null"**: if the native object is
  destroyed but a managed wrapper still references it, the engine overrides `==` so
  the wrapper *compares equal to null* even though the managed object still exists.
  It's the two memory worlds disagreeing about lifetime.

Two more practical notes that generalize: compiling ahead-of-time (§17) changes how
*code* is compiled, not how managed *memory* is managed — there's still a GC for
managed objects. And a GC that is non-compacting and stop-the-world causes frame
spikes; incremental collectors spread the work across frames. Either way, "don't
allocate per frame" is standard advice (§16).

---

## 14. Hot reload

**Hot reload** means rebuilding and swapping the script's code while the engine
keeps running, without restarting the process. It's one of the biggest payoffs of a
scripting layer — and it interacts sharply with the two-heap picture of §13.

### 14.1 The core problem: cached function pointers go stale

Recall (§5, §8) that the engine calls the script through **cached function
pointers** — the addresses of the JIT-compiled entry points. If you rebuild the
script and load the new module, the runtime JIT-compiles its methods at **new
addresses**. The old pointers the engine still holds now point at code that's been
thrown away — calling them is a crash. So naive hot reload (just load a new module)
cannot work. Two pieces are needed.

### 14.2 The module must be unloadable

The simple "load an assembly and grab a function pointer" path loads the script and
never lets go. To reload, load it instead into a **collectible load context** — the
runtime's unit of "a set of loaded assemblies you can later throw away" (in .NET, a
collectible `AssemblyLoadContext`). To reload: unload the old context (its types and
JIT'd code are collected), create a fresh one, load the rebuilt module into it.

### 14.3 The engine must call through indirection

The engine's cached pointers must never point *directly* at code that will be
thrown away. The standard trick is a **two-part split**:

```
   ENGINE (native)           STABLE LOADER (never reloaded)          GAME (reloaded)
   ───────────────           ──────────────────────────────         ───────────────
   cached pointer ─► Loader.Update()  ──(forwards to current game)──► Game.Update()
   (NEVER changes)           holds a collectible context +           lives in the context;
                             a reference to the current game         swapped on reload
```

- A small **loader** is loaded *once* and never reloads. The engine caches *its*
  entry-point addresses — and those never move.
- The loader owns the collectible context and a reference to the *current* game
  object; its `Update` just forwards to `currentGame.Update()`.
- On reload, the loader unloads the old context, loads the rebuilt game into a new
  one, creates a new game instance, and repoints its internal reference.

The engine's pointers stay valid forever because they point at the *stable loader*;
only the loader's private reference changes. The "pointers change" problem
disappears behind that indirection.

### 14.4 Keeping state alive across a reload

Unloading the game module destroys **everything inside it** — static fields and all
its GC objects. So by default a rebuild wipes the game's state. To preserve it,
keep it somewhere that *isn't* unloaded. Three options:

1. **Keep the authoritative state on the engine side** (§13.2). If the state lives
   in engine-owned native memory, reloading the *logic* leaves the *data* untouched;
   the new code re-attaches and continues. Bonus: it also cuts GC pressure. This is
   usually the cleanest answer.
2. **Serialize across the reload.** Before unloading, serialize the state into a
   plain blob held by the engine or the loader; after the new module loads,
   deserialize it into the new types. Only serializable fields survive — live
   references, handles, threads, and closures do not.
3. **Store state in a separate, never-reloaded module** shared by both the old and
   new game versions, handed to the new instance on reload.

> **What Unity does** is option 2: on script recompile it performs a *domain
> reload* — serialize the serializable fields, tear down and rebuild the managed
> environment, then restore those fields. (Newer Unity can *disable* domain reload
> for faster iteration, at the price of resetting `static` state yourself — a direct
> demonstration that statics don't survive a reload unless something outside the
> reload preserves them.)

Interpreted scripting languages (§20) make this far easier — reloading often means
just re-reading and re-evaluating a source file — but the state-preservation
question is the same regardless of language.

---

## 15. Defining script-side components and data

In a pure-native engine, component/data types are often declared with a **macro**
(e.g. a `derive`-style attribute on a struct) that runs *at compile time* and bakes
everything the engine needs — registration, a type id, storage/reflection glue —
straight into the compiled binary. When the gameplay lives in a *scripting*
language, where does that mechanism go? The script language has no native macros,
and the engine never sees the script's types when *it* is compiled. The answer has
two halves: how you generate the boilerplate, and — the part that actually matters —
where the data lives.

### 15.1 What the native macro was really doing

A `derive`-style macro is compile-time code generation: given a struct, it emits
trait impls, a unique **type id**, an entry in a **registry**, and often storage or
reflection metadata — all during the engine build, compiled into the binary. So the
full set of components is fixed and known the moment the artifact exists. The
property to reproduce is **"declare a struct, get all the wiring for free."**

### 15.2 The direct analog: compile-time source generation

The true counterpart to a native macro is **compile-time code generation in the
scripting language** — for C#, **Roslyn Source Generators**. A generator runs
*during the script build*, inspects annotated types, and **emits additional code**
into the same module:

```csharp
[Component]
public partial struct Position { public float X, Y; }
```

plus a generator produces — at build time — the type id, registry registration,
fast accessors, serialization, and so on. Structurally this is the *same story* as
the native macro, just on the script side of the build. It has no runtime
reflection cost and is AOT-friendly (§17), which makes it the best fit for
performance-sensitive code.

### 15.3 The lighter analog: attributes + runtime reflection

The surface-level equivalent of a native attribute is a scripting-language
**attribute** (`[Component]`) — but note the difference: an attribute is *just
metadata*, not code generation. To turn it into behavior you use **reflection at
run time**: on startup, scan the loaded assemblies, find every type marked
`[Component]`, and register it dynamically. This is how Unity discovers
`MonoBehaviour`s. It's simpler to write, at the cost of a little startup scanning
and reduced AOT compatibility. A third, no-magic option is **explicit
registration** (`world.Register<Position>()`), fine for small projects.

### 15.4 The real question: where does the component data live?

Generating boilerplate is the easy half. The design decision that matters is
**which side owns the storage** — mapping onto the two heaps of §13.

- **Option A — data on the script (managed) side.** The engine stays agnostic: the
  script runs its own ECS, components are ordinary managed types, and the engine
  only calls `Update`/`Draw` and offers drawing/input. The engine never needs to
  know a component's shape. Simplest path; costs are the managed ones — GC pressure,
  and the data doesn't survive hot reload unless you preserve it (§14.4).
- **Option B — data on the engine (native) side** (data-oriented / DOTS-style,
  §13.2). The authoritative arrays are engine-owned, so the engine must **allocate
  storage sized to each component** — it needs each type's size and layout. But the
  types are *declared in the script*, so you **cannot bake them into the engine
  binary the way a native macro does.** Instead you **register a schema at run
  time:** at startup the script tells the engine "there's a component `Position`,
  8 bytes, with this layout"; the engine allocates raw byte-array storage per type
  and treats the bytes as opaque; the script reads/writes through the mirror-struct
  + pointer pattern of §6.5.

So the component *definitions* are still compiled into an artifact — just the
*script* module (via §15.2), not the engine — and then **communicated to the engine
at run time**, rather than compiled into it.

### 15.5 Reconciling with "compiled into the binary"

A native macro can bake components into the binary because, in a pure-native app,
**the game *is* the binary** — components and engine ship together. The whole point
of a scripting layer is the opposite: to add or change components *without
recompiling the engine*. So the mechanism necessarily shifts — compile-time codegen
moves to the *script* build, and the engine learns the component set *at run time*.
The two can be stitched together automatically: a generator can emit both the
script registration and the matching schema-registration calls (even a native-side
mirror), the same "generate the glue so it can't drift" idea as §7.5.

> **Bottom line:** use compile-time source generation as the macro-equivalent way
> to define components in the script (or attributes + reflection for a simpler,
> runtime-discovered version). Then decide *where the data lives*: on the script
> side (simplest, engine stays agnostic), or in engine-owned storage (best for
> performance and hot reload), in which case the engine learns each component's
> layout via runtime schema registration.

---

## 16. The runtime's cost: performance, memory, and size

A managed runtime is not free. For most applications the costs are negligible, but
you should know exactly what they are.

**Startup cost.** Booting the runtime and JIT-compiling the first calls takes time —
typically tens to a few hundred milliseconds. The very first frame does a touch
more work as methods compile on first use. After that warm-up, it's gone.

**Memory footprint.** A managed process carries baggage a native one doesn't: the
runtime's own libraries, the **GC heap** (managed objects plus reserve the GC
keeps), the **JIT code cache** (machine code it generated), and **type metadata**
for every loaded type. In practice a minimal managed app uses on the order of *tens
of megabytes* before your code allocates anything; a native program can use a
fraction of that.

**Steady-state (per-frame) cost.** Three things to watch:

- **JIT'd code is fast** — after warm-up it runs at roughly native speed, often
  within a few percent of equivalent C++. Usually *not* a problem.
- **Garbage collection** is the one that matters for real-time work. When the GC
  runs, it can briefly pause your code; an unlucky pause mid-frame is a visible
  stutter. The fix is to avoid allocating garbage every frame (reuse objects, avoid
  per-frame allocation) — standard practice in managed game code.
- **Crossing the boundary** has a tiny per-call cost — the managed↔native
  transition plus any marshalling. For a handful to hundreds of calls per frame it's
  immeasurable; a million tiny calls per frame would add up.

**On-disk size: the runtime floor.** Even with ahead-of-time compilation (§17) —
which removes the JIT and the need for an installed runtime — a managed program has
a **size floor** a native program does not. AOT must **statically bake the runtime
services into the binary**: the garbage collector, type metadata, exception
machinery, and the parts of the standard library your code touches. None of that
can be fully removed while staying "normal" managed code, so it's a fixed baseline —
present even in a "hello world." A native language links its standard library too,
but has no GC and no managed metadata and dead-code-eliminates aggressively, so its
floor is far lower.

Rough on-disk sizes for a *trivial* program (release, stripped/trimmed):

| Build                                 | Typical size | Standalone?                        |
| ------------------------------------- | ------------ | ---------------------------------- |
| **Native**, size-optimized            | ~50–300 KB   | yes                                |
| **Native**, default release           | ~0.3–1 MB    | yes                                |
| **Managed, AOT**, fully trimmed       | ~1.5–3.5 MB  | yes                                |
| **Managed, framework-dependent**      | tens of KB   | **no** — needs an installed runtime |

If your goal is a single self-contained binary under ~0.5 MB (easy in a native
language), mainstream managed tooling **cannot reach it**: the AOT floor is a few MB,
and bundling the whole runtime is tens of MB. A framework-dependent module *looks*
tiny only because the large runtime lives elsewhere on the machine — the footprint
is hidden, not gone. (Specialized, strip-everything AOT toolchains can get much
smaller by dropping the GC, reflection, and standard library — but at that point
you've given up most of what made the managed language pleasant, defeating the
reason to choose it.)

> **Bottom line:** the size floor isn't a tooling deficiency you can optimize away —
> it's the cost of the very runtime services (GC, metadata, reflection) that *are*
> the point of a managed language. Choose managed for productivity and safety and
> budget a few MB; choose native when a tiny, dependency-free binary is the hard
> requirement. For the *script* layer, the managed trade is usually a great deal;
> for the *engine* layer, native is the deliberate choice.

---

## 17. Compilation strategies and deployment

The runtime cost above depends heavily on *how* and *when* the managed code is
turned into machine code, and on how the runtime reaches the end user.

### 17.1 A spectrum of compilation strategies

"JIT" is only the default. Managed platforms offer a spectrum:

| Strategy             | When IL → machine code happens                 | Needs a runtime at run time? |
| -------------------- | ---------------------------------------------- | ---------------------------- |
| **Pure JIT**         | at run time, on first call                     | yes                          |
| **Tiered JIT**       | quick code first, hot paths re-optimized later | yes                          |
| **ReadyToRun (R2R)** | partly at build time, rest JIT'd               | yes                          |
| **AOT**              | entirely at build time                         | **no**                       |

Modern runtimes use **tiered compilation** by default: JIT a method quickly at first
(fast to compile, less optimized), then silently recompile it with full
optimizations if it turns out to be "hot" (called a lot) — giving both fast startup
and fast steady-state code.

**Ahead-of-Time (AOT)** compiles the managed code all the way to a native binary at
build time, with no IL and no JIT left at run time; the runtime services (GC, type
system) are statically linked in. You gain: no runtime to install, fast startup, a
smaller footprint, a single self-contained file. You give up: anything that depends
on generating code at run time — AOT **restricts reflection** and forbids runtime
code generation, so some libraries won't work; build times are longer, and you
compile separately per OS/CPU. Intermediate options exist too: **R2R** precompiles
IL to native at build time but keeps the JIT as a fallback; and translating IL to
C++ and compiling *that* natively (Unity's IL2CPP) is the same spirit as AOT with
different machinery. Note that AOT changes how *code* is produced, not how managed
*memory* works — there is still a GC (§13.3).

### 17.2 What the runtime is responsible for

Worth listing, because it's what you'd have to reimplement if you dropped the
runtime:

- **JIT/AOT compilation** — turning IL into machine code.
- **Memory management / GC** — allocating and automatically freeing objects.
- **The type system & metadata** — enabling reflection, generics, interop.
- **Assembly loading & resolution** — finding and loading modules (including into
  the collectible contexts that make hot reload possible, §14).
- **Exception handling** — the machinery behind `try/catch` and stack unwinding.
- **Threads & scheduling** — managed threads, the thread pool, `async`/`await`.
- **Interop / marshalling** — the plumbing that lets managed and native code call
  each other (the very mechanism this document is about).
- **The standard library** — strings, collections, math, files, networking, and
  thousands of built-in types.

### 17.3 Deployment: what must be on the user's machine

Managed platforms typically offer three **deployment models**:

| Model                   | Runtime pre-installed on the OS?      | Output size              | Per-OS build?     |
| ----------------------- | ------------------------------------- | ------------------------ | ----------------- |
| **Framework-dependent** | **Yes** — a matching runtime          | small (just your module) | no (portable IL)  |
| **Self-contained**      | No — the runtime is bundled in        | large (+ tens of MB)     | yes               |
| **AOT**                 | No — no runtime exists at all         | medium native binary     | yes               |

- **Framework-dependent** ships just your IL module and relies on a runtime already
  installed. Smallest to distribute, but the user (or an installer) must have the
  right runtime present.
- **Self-contained** bundles a private copy of the runtime with your app — nothing
  needs pre-installing, at the cost of a much larger download. One build per target.
- **AOT** has no runtime at all; the program *is* a native binary — nothing to
  install, smallest dependency surface, most build-time restrictions.

Managed platforms are generally **cross-platform** (Windows, Linux, macOS including
arm64, and more). The IL your code compiles to is OS-independent, but *running* it
always needs a runtime for the specific OS+CPU — whether pre-installed, bundled, or
baked in. Self-contained and AOT builds are produced per **RID** (Runtime
IDentifier, e.g. `win-x64`, `linux-x64`, `osx-arm64`): the IL is portable, but a
bundled or AOT-compiled runtime is native to one OS + architecture.

### 17.4 The trade-off, summarized

Relying on a runtime buys **productivity and safety** (automatic memory management
eliminates whole bug classes), a **large standard library and ecosystem**,
**portability** (the same IL runs anywhere a runtime exists), **reflection and
metaprogramming**, and **adaptive optimization + hot reload** (compiling at run time
lets the JIT re-optimize and lets code be swapped live). It costs you a **required
runtime**, **startup latency and memory footprint**, **GC pauses**, **less
predictable timing** than AOT native code, and **interop friction** at every
boundary crossing. That's the classic managed-vs-native trade: control and
predictability for a large gain in safety and iteration speed.

---

## 18. Editor tooling and IntelliSense

A natural worry: since the script isn't launched the normal way (no standalone
`Main`, hosted by a native process), does that break the editor experience —
autocomplete, parameter hints, go-to-definition, error squiggles, collectively
**IntelliSense**?

**No. IntelliSense works fully, and none of the hosting machinery affects it.**

### 18.1 IntelliSense is an *editing-time* feature, not a *runtime* one

IntelliSense is produced by a **language service** (for C#, **Roslyn**, exposed via
editor extensions) that works purely by **reading your source and the metadata of
the assemblies you reference**, compiling in the background as you type. It never
*runs* your program. So how the module is loaded, that there's no standalone entry
point, and that the host is a native process are all **runtime** concerns that
happen long after editing. The editor sees none of it and needs none of it.

```
   EDITING TIME (IntelliSense)                RUNTIME (the bridge)
   ──────────────────────────                 ────────────────────
   language service reads source +            engine boots runtime, loads
   referenced assembly metadata      ⇢ no ⇢   the module, looks up entry
   → completions, errors, hints       link    points, calls across the C ABI
```

From the editor's point of view, the script is simply an ordinary class-library
project. Open it and you get the full experience: autocomplete, signature help,
go-to-definition, find-references, rename, and live error checking against the whole
standard library.

### 18.2 Where the completions come from, and the one real limit

Script code goes through the friendly facade (§12.3), which is ordinary managed
code, so `Engine.DrawCircle(...)` gets full completion, as do your own types. Even
the low-level pieces (the function-pointer table, the exported entry points, unsafe
marshalling helpers) get IntelliSense, because they're still just ordinary code the
editor understands.

The single limitation: **IntelliSense does not cross the language line.** From the
script you can't complete into or navigate to the *native* implementation behind a
function pointer — the script only knows the pointer's *signature*, not the body.
In practice this doesn't bite, because script authors never edit the native side;
the native side gets its own first-class tooling separately. Note too that no editor
will catch **layout drift** (§6.2) between the two struct definitions — each file is
individually valid — which is another argument for generating the bridge (§7.5).

To get it working you just need the usual pieces: open the *project* (not a stray
file) so references resolve, install the platform **SDK** (its reference assemblies
power standard-library completions), and install the language extension.

> **Bottom line:** IntelliSense is a normal, first-class authoring experience,
> completely decoupled from the hosting mechanism. (Whether you can also *debug* the
> script while it runs inside the native host is the next section.)

---

## 19. Debugging

Two questions matter: can you debug the script *at all* when it's hosted by a native
process, and can you still inspect state when the data lives on the engine side
(§13.2)? Short answers: **yes**, and **yes** — with one honest limit about crossing
into native code.

### 19.1 You *attach*, not *launch*

A managed debugger does its job by **attaching to a running process** and
understanding the runtime's structures inside it: managed objects, JIT-compiled
stack frames, and locals — mapped back to your source via **debug symbols** (PDBs
on .NET). Nothing about that requires the normal launcher or a standalone entry
point. The recipe:

1. Build the script with **debug symbols** present.
2. Launch the native host as usual — it boots the runtime and loads the script.
3. From your IDE, **attach to the process**, choosing the **managed** debug engine.

Now breakpoints bind, stepping through `Update`/`Draw` works, and the watch/locals
windows show managed objects normally. That a native process is the host is
invisible to the managed debugger — it only cares that a runtime is alive in the
process. (Tip: to catch startup code, have the entry point call a "launch the
debugger" API so it prompts to attach at exactly that moment.)

### 19.2 Inspecting engine-owned data

The experience depends on how the state is exposed (§13.2):

- **Shared struct + pointer.** This debugs **well**: the managed debugger knows the
  mirror type's layout and reads native memory through the pointer, rendering it
  with named fields. Dereference into a local, or type `ptr->Field` in a watch
  window. The essential caveat: it interprets those bytes **using the mirror
  declaration**, so what you see is correct only if the mirror is in lockstep with
  the native definition (§6.2). If they've drifted, the debugger confidently shows
  **misaligned garbage** and can't warn you — debugging surfaces the same contract
  risk the whole bridge rests on.
- **Accessor functions.** There's no struct to display and no pointer to interpret,
  so you inspect **return values** of the accessor calls, set breakpoints in the
  logic that uses them, and evaluate accessors in a watch window. More limited — one
  value at a time through the API's keyhole. If debuggability of shared state
  matters, that favors the mirror-struct approach.

### 19.3 The hard limit, and practical notes

A managed debugger understands *managed* code. When the script calls an engine
function, that call crosses into **native machine code**, which the managed debugger
treats as a black box: you can **step over** it but not **step into** it, set
breakpoints in it, or see its locals. To debug across the line — breakpoints in both
sides in one session — you need **mixed-mode (native + managed) debugging**, which
some IDEs support but is fiddly and platform-dependent. The common practice is to
debug **one side at a time**: the managed debugger for script problems, a native
debugger for engine problems.

A few gotchas: build with real debug symbols (optimized/inlined release code makes
stepping and locals less faithful); tiered JIT (§17.1) can make very early stepping
feel odd as methods recompile, but it settles; **AOT is different** (§17) — with no
runtime present, the *managed* debugger no longer applies and the binary debugs like
native code, so debug the normal JIT build during development; and debugging through
a hot reload (§14) is easiest reasoned about one loaded version at a time.

> **Bottom line:** storing data on the engine side does **not** cost you the
> debugger. Attach a managed debugger to the host process and debug the script
> normally; engine-owned state is fully inspectable through a mirror struct +
> pointer, or visible via return values with accessors. The only thing you give up
> is stepping *into* the native code — a separate tool, not a limitation of the
> design.

---

## 20. Supporting multiple scripting languages

Nothing in the design is *about* one particular scripting language — the whole point
of routing through the **C ABI** (§4.3) was to make the engine indifferent to what's
on the other side. Adding a second or third language is mostly "teach that language
to (a) be driven through the entry points and (b) call the engine's API." *How* you
do that depends on which of two families the language belongs to — the most
important distinction in this section.

### 20.1 Two families of scripting language

**Family A — languages that compile to native code and speak the C ABI directly.**
C# (via an "unmanaged callers" export), but also C, C++, Rust, Go, Swift, Zig. These
produce a native module that exports plain C functions. Integrating one is *exactly
the pattern this whole document describes*: build it to a native library, load it,
look up its entry points, hand it the API struct. For a *managed* member you also
host its runtime (§8); for an already-native one you skip the runtime and just load
the library.

**Family B — languages that run inside an interpreter or VM.** JavaScript, Lua,
Python, WebAssembly. These do **not** compile to a native module you call directly.
Instead you **embed the language's engine** (an interpreter/VM, itself a native
library) into your engine, and the model **inverts**:

```
   FAMILY A (C#, C++, Rust…)            FAMILY B (JavaScript, Lua, Python…)
   ─────────────────────────           ───────────────────────────────────
   engine looks up a native             engine embeds a VM and asks it:
   Update() and calls its address   vs. "run the script function named 'update'"

   API = C struct of                    API = host functions REGISTERED
   raw function pointers                into the VM's global scope
```

In Family B there are no raw function pointers into the script. The engine holds a
**VM handle**, loads the script *source* (or bytecode), and: to run a frame, invokes
a script-defined function *by name* through the VM's API; to expose the engine,
**registers native callbacks** into the VM as built-in functions (e.g. a global
`engine.drawCircle(...)`) instead of passing a pointer table. The marshalling
principle (§7) is unchanged — rich values still become plain data — but the
*mechanics* are the VM's binding API rather than the bare C ABI.

### 20.2 A clean way to hold several backends

Introduce one small abstraction in the engine: a trait/interface every backend
implements, and have the game loop talk only to it:

```rust
pub trait ScriptBackend {
    fn init(&mut self, api: &EngineApi);   // called once
    fn update(&mut self, dt: f32);         // called each frame
    fn draw(&mut self);                    // called each frame
}
```

- A **managed backend** implements it by hosting the runtime and forwarding to the
  looked-up entry points (§5, §8).
- A **VM backend** implements it by forwarding each call into an embedded VM.

The rest of the engine neither knows nor cares which language is behind the trait —
one implementation per language. This is the `N`-problem of §4.3 paying off.

### 20.3 Worked sketch: embedding JavaScript

1. **Pick a JS engine to embed.** The realistic trade is power vs. footprint: a full
   production VM (V8-class) is JIT-fast and complete but heavy; a small embeddable
   interpreter (QuickJS-class) is tiny and easy; a pure-in-your-native-language
   engine (Boa-class) has no C dependency but is slower and less complete. For a
   small game the small interpreter is usually the pragmatic pick.
2. **Entry points become "call a function by name."** The backend keeps the VM alive
   and calls its `init`/`update`/`draw` globals each frame.
3. **Expose the API as registered host functions.** Bind native closures into the
   VM's global scope; each receives the VM's argument values, converts them, and
   forwards to the same underlying engine calls the managed path uses.
4. **Marshalling is VM-flavoured.** Each VM has its own value model. JavaScript, for
   instance, has a single number type (an IEEE-754 `double`), so engine values pass
   as `double` and cast at the boundary; strings are converted by the embedding
   library rather than hand-pinned; and you never pass a live VM object across — read
   out plain fields, or keep authoritative data engine-side (§13.2).

### 20.4 Other languages, and running several at once

The same two-family logic covers the usual suspects: **Lua** (a tiny, fast,
classic embedded scripting choice), **Python** (rich ecosystem, heavier), and
**WebAssembly** (`wasmtime`/`wasmer`-class hosts). WASM deserves special mention:
because *many* languages compile to it, a single WASM backend can effectively give
you several scripting languages at once, all sandboxed and portable — at the cost of
WASM's own marshalling model (only numbers cross natively; everything else goes
through linear memory, much like the pointer + length rule of §6.5).

Because the engine talks to an abstract backend (§20.2), you can even run **several
backends at once** — some systems in one language, some in another. The one hard
constraint is the thread model (§11): most script VMs are *not* freely thread-safe,
so keep each on the game thread. Most projects still pick *one* language for
consistency; mixing is mainly useful for third-party plugins or migration.

### 20.5 Trade-offs when choosing

- **Hot reload gets *easier*, not harder.** The elaborate module-swap dance of §14
  exists because a compiled managed assembly loads as native code. Interpreted
  Family-B languages hot-reload almost for free: re-read and re-evaluate the source.
- **Performance & footprint span a wide range.** JIT VMs are fast but heavier; small
  interpreters are light but slower; remember the per-call boundary cost (§16)
  multiplies with chatty scripts.
- **Tooling varies (§18).** A strongly-typed scripting language with mature editor
  support (or TypeScript over JavaScript) gives a better authoring experience than a
  dynamically-typed one.
- **Safety & sandboxing.** Embedded VMs can be sandboxed so untrusted scripts can't
  touch the filesystem or crash the host — valuable for user-made mods. Native
  Family-A plugins have no such sandbox; a bad pointer takes down the process (§10).

> **Bottom line:** the C-ABI boundary is the language-neutral hub the whole system
> is built around. Adding a language is "write one more backend" — trivial for
> native, C-ABI-speaking languages (Family A), and a matter of *embedding a VM and
> registering host functions* for interpreted ones (Family B).

---

## 21. Glossary

- **ABI (Application Binary Interface)** — the binary contract for how compiled
  code passes arguments, returns values, and lays out data. The thing both sides
  must agree on.
- **Assembly** — the managed platform's term for a compiled module (`.dll`/`.exe`)
  of managed code.
- **AssemblyLoadContext (ALC)** — a .NET container for a set of loaded assemblies; a
  *collectible* one can be unloaded as a unit, which is what makes hot reload
  possible.
- **Binding generator** — a tool that auto-generates the FFI glue and matching type
  declarations between two languages so they can't drift out of sync.
- **Blittable** — a type whose bytes mean the same thing on both sides, so it can be
  copied across the boundary with no conversion (numbers, pointers).
- **Calling convention** — the fine print of the ABI: which registers/stack slots
  carry arguments and who cleans up (`cdecl`, `stdcall`, …).
- **CLR (Common Language Runtime)** — the engine inside the .NET runtime that
  JIT-compiles and runs managed code.
- **Data race** — unsynchronized concurrent access to the same memory from more than
  one thread, at least one of them writing. Some native languages prevent it at
  compile time; managed languages generally do not (§11, §12).
- **Domain reload** — Unity's process of serializing managed state, tearing down and
  rebuilding the managed environment after a script recompile, then restoring it.
- **FFI (Foreign Function Interface)** — calling functions across a language boundary
  by agreeing on the C ABI.
- **Function pointer** — the memory address of a function's machine code; enough to
  call it, regardless of source language.
- **GC (Garbage Collector)** — the runtime subsystem that automatically frees unused
  objects (and may move them in memory).
- **hostfxr** — the .NET native library that selects and starts the right runtime;
  the standard way to host .NET from native code.
- **Hot reload** — rebuilding and swapping the script's code while the engine keeps
  running, without restarting the process.
- **IL (Intermediate Language)** — the CPU-independent instructions a managed
  compiler produces; the runtime JIT-compiles IL into machine code. (JVM: bytecode.)
- **JIT (Just-In-Time) compilation** — translating IL into machine code at run time,
  on first use.
- **Machine code** — the raw numeric instructions a CPU executes directly.
- **Managed code** — code that runs under a runtime; contrast **native**.
- **Marshalling** — converting rich values (like strings) into plain data that can
  cross an FFI boundary, and back.
- **Native AOT (Ahead-Of-Time)** — compiling managed code all the way to a native
  binary at build time, with runtime services statically linked in, so no separate
  runtime or JIT is needed at run time.
- **Native code** — machine code that runs directly on the CPU with no runtime
  underneath (Rust, C, C++).
- **Nullable reference analysis** — an opt-in compile-time check (e.g. C#'s nullable
  reference types) that flags possibly-null uses, recovering much of a "no null"
  guarantee — advisory, not ironclad (§12).
- **PDB** — a debug symbol file mapping compiled code back to source lines, names,
  and locals so a debugger can display them (§19).
- **Pinning** — temporarily forbidding the GC from moving a managed object so its
  address can be safely shared with native code (`fixed` in C#).
- **P/Invoke (Platform Invoke)** — .NET's classic mechanism for calling native C
  functions, which includes an automatic marshaller for many non-blittable types.
- **`#[repr(C)]` / `[StructLayout(Sequential)]`** — directives that force a struct
  into predictable C-style memory layout so both languages agree on it.
- **RID (Runtime IDentifier)** — a tag like `win-x64`, `linux-x64`, or `osx-arm64`
  naming a specific OS + CPU target; self-contained and AOT builds are produced per
  RID.
- **Runtime hosting** — starting a language runtime from inside another program
  rather than via its normal launcher.
- **Source generator** — a component that runs at managed-compile time and emits
  extra code into the module; the managed counterpart to a native `derive`-style
  macro (§15).
- **Tiered compilation** — the JIT strategy of compiling a method quickly at first
  and re-optimizing it later if it turns out to be "hot."
- **`[UnmanagedCallersOnly]`** — a C# attribute marking a method as directly callable
  by native code, like a plain C function.

---

## 22. Further reading

- .NET hosting (the official C API for embedding the runtime):
  <https://learn.microsoft.com/dotnet/core/tutorials/netcore-hosting>
- `[UnmanagedCallersOnly]`:
  <https://learn.microsoft.com/dotnet/api/system.runtime.interopservices.unmanagedcallersonlyattribute>
- Rust FFI ("calling code in other languages"):
  <https://doc.rust-lang.org/nomicon/ffi.html>
- C# function pointers (`delegate* unmanaged`):
  <https://learn.microsoft.com/dotnet/csharp/language-reference/unsafe-code#function-pointers>
- WebAssembly as an embedding target: <https://webassembly.org/>
