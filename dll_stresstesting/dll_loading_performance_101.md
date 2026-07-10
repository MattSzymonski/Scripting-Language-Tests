# DLL Loading Performance 101

A general reference on how dynamic libraries load, link, and resolve symbols, and why that
process is sometimes slow. Windows/PE-focused (since that's where the deepest mechanics and
tooling live), with a short comparison to other platforms at the end. Nothing here is tied to
any particular engine, framework, or codebase — it applies to any native DLL/EXE.

## Table of contents

1. [Static vs. dynamic linking](#1-static-vs-dynamic-linking)
2. [Anatomy of a DLL](#2-anatomy-of-a-dll)
3. [The load sequence, step by step](#3-the-load-sequence-step-by-step)
4. [Symbols: exports, imports, and the IAT](#4-symbols-exports-imports-and-the-iat)
5. [Name mangling and demangling](#5-name-mangling-and-demangling)
6. [Why "load time" is deceptive](#6-why-load-time-is-deceptive)
7. [What actually makes a DLL load slowly](#7-what-actually-makes-a-dll-load-slowly)
8. [Security features that add load-time cost](#8-security-features-that-add-load-time-cost)
9. [Diagnosing load performance](#9-diagnosing-load-performance)
10. [Mitigation strategies](#10-mitigation-strategies)
11. [Other platforms](#11-other-platforms)
12. [Glossary](#12-glossary)

---

## 1. Static vs. dynamic linking

- **Static linking**: the linker copies the code of a dependency directly into your executable
  at build time. One file, no runtime dependency resolution, but every executable that uses the
  library duplicates its code in memory and on disk.
- **Dynamic linking**: your executable only records *which* symbols it needs and *which* library
  provides them. The actual code lives in a separate file (`.dll` / `.so` / `.dylib`) that gets
  mapped into the process at runtime, and the OS loader connects the two.

Dynamic linking trades build-time simplicity for runtime cost: multiple processes can share one
copy of a library in physical memory, libraries can be updated independently (or swapped for
plugins), but now every process launch pays a cost to *find*, *map*, and *link* each dependency —
and that cost is what this document is about.

---

## 2. Anatomy of a DLL

A DLL is a PE (Portable Executable) file — the same container format as a Windows `.exe`, just
flagged as a library instead of an application.

**Headers** (read first, describe everything else):
- **DOS header** — legacy stub (`MZ` signature) kept for backward compatibility; mostly just
  points to the real header.
- **COFF/File header** — machine type (x86/x64/ARM64), number of sections, timestamp,
  characteristics (is this a DLL? large-address-aware? etc).
- **Optional header** (not actually optional for DLLs) — entry point address, preferred load
  address (`ImageBase`), subsystem, and a table of **data directories**: fixed-size pointers to
  where the export table, import table, resources, debug info, TLS data, and so on live inside
  the file.

**Sections** — the file's content, each with a name, a size, and read/write/execute permissions:

| Typical section | Contents | Permissions |
|---|---|---|
| `.text` | compiled machine code | Read + Execute |
| `.rdata` | read-only data: string literals, the import/export tables themselves | Read |
| `.data` | mutable global/static variables | Read + Write |
| `.pdata` | exception-handling metadata (x64) | Read |
| `.rsrc` | embedded resources: icons, version info, manifests | Read (+ Write sometimes) |
| `.reloc` | base relocation table (see ASLR, below) | Read |

**Entry point** — every DLL has one function the loader calls at specific lifecycle events
(load, unload, thread attach/detach on Windows this is `DllMain`). It is **not** your "main"
business logic entry point — well-behaved libraries do the absolute minimum here, because of the
*loader lock* (section 6).

---

## 3. The load sequence, step by step

When something asks the OS to load a DLL (whether explicitly, via a `LoadLibrary`-style call, or
implicitly because the loading executable/another DLL already declared it as a dependency), this
is roughly what happens, **all before control returns to whoever asked for the load**:

1. **Locate the file.** Search a defined order of directories (application directory, system
   directories, directories on the loader's search path, side-by-side/manifest redirection).
   Skipped entirely if the library is already resident (see step 6).
2. **Map the file into memory.** The file is memory-mapped, not read wholesale into a buffer —
   pages are faulted in from disk as they're actually touched. This is why a DLL's *virtual size*
   can differ from its *raw file size* on disk (section alignment/padding), and why disk speed
   and cache state matter more than raw file size.
3. **Apply base relocations**, if the DLL couldn't be mapped at its preferred `ImageBase` (very
   common with ASLR on) — every absolute address baked into the code/data at compile time needs
   patching to reflect the actual load address. Cheap for small binaries, more work for large
   ones or when the preferred base is contested.
4. **Resolve the import table.** For every DLL this one declares as a dependency: if it's *not
   already loaded* in the process, recursively run this entire sequence for it first. If it
   *is* already loaded, just look up the already-resolved addresses. This recursion is the
   single biggest source of "why is this taking so long" — see section 6.
5. **Run TLS callbacks**, if any are registered — these execute *before* the entry point, and are
   easy to forget about because they're not visible in a stack trace the way `DllMain` is.
6. **Run static/global initializers.** For native C++ specifically: constructors of global and
   static objects run here, as part of CRT startup, before your library's own entry point is
   called. Any real work stuffed into a global constructor — allocating a lookup table, parsing a
   config file, initializing a subsystem — happens synchronously here, on the thread that
   triggered the load, before that thread can do anything else.
7. **Call the entry point** (`DllMain(DLL_PROCESS_ATTACH, ...)` on Windows). Only *after* this
   returns is the DLL considered fully loaded, and only then does the original "please load this
   DLL" call return to its caller.
8. From this point on, individual symbols can be looked up by name/ordinal (`GetProcAddress`-style
   APIs), and the library is usable.

If the library is loaded again later (by anything else in the same process), steps 1-7 are
skipped entirely — the loader just increments a reference count and returns the existing handle
almost instantly. **This asymmetry — first load pays full price, every subsequent load in the
same process is nearly free — is the single most important thing to understand about DLL load
performance.**

---

## 4. Symbols: exports, imports, and the IAT

A **symbol** is just a named location in code or data — a function or a global variable — that
one compilation unit wants to make visible to, or borrow from, another.

**Export table**: a DLL's advertisement of what it makes available to others. Each entry is a
name (optional — some are ordinal-only, see below) mapped to a relative address inside the DLL.
An export can also be a **forwarder**: instead of pointing to code, it points to *another DLL's
export* by name (`"OtherLib.SomeFunction"`) — the loader transparently redirects the call. This
is how facade/compatibility DLLs work without duplicating code.

**Import table**: the mirror image — a DLL's declaration of "I need symbol X from library Y."
For each imported library, there's a list of names/ordinals it needs.

**IAT (Import Address Table)**: the actual mechanism that makes dynamic linking fast *after* load
time. Every call from your code to an imported function doesn't jump directly to that function —
it jumps through a slot in the IAT, a small array of pointers. At load time, the loader fills each
slot with the real resolved address. At runtime, calling an imported function costs one extra
memory indirection compared to a direct call — the *linking* cost was already paid up front,
during load.

**Ordinal exports**: instead of a name, an export can be identified purely by a small integer
index into the export table. Lookup by ordinal is marginally faster (no string comparison), but
it's fragile: if the DLL is rebuilt and the *order* of exports changes, ordinal-based callers
silently start calling the wrong function. Name-based exports are self-describing and don't have
this hazard, at a small runtime cost.

**Delay-loading**: a middle ground. A DLL can declare a dependency as "delay-loaded," meaning the
loader does *not* resolve or load it up front — the first *call* through that import triggers
loading and resolution on demand, with a stub that transparently patches itself after the first
call. This moves cost from "every process startup" to "whenever this rarely-used feature is
actually exercised," which is a real win if a large or slow dependency is only needed
occasionally.

```
Your code: call [IAT slot #3]
                    │
    ┌───────────────┴────────────────────────┐
    │ Load time: loader resolves & fills slot │
    │ Run time:  one indirect jump per call   │
    └──────────────────────────────────────────┘
```

---

## 5. Name mangling and demangling

C has no function overloading or namespaces, so a C function's exported name is just its plain
name (`CreateWidget`). C++ allows overloading, namespaces, templates, and classes — the linker
still only understands flat, unique names, so compilers **mangle** (or "decorate") C++ names into
a single string encoding the full signature: namespace, class, function name, parameter types,
calling convention, const/volatile-ness, and so on.

Two compilers, two different (incompatible) mangling schemes — this is one reason a DLL built
with one compiler/version is often not safely usable from a different one for C++ APIs, and why
cross-compiler/cross-language plugin ABIs are usually defined in terms of plain C (`extern "C"`).

Mangled names are unreadable to humans but fully recoverable, since the encoding is systematic —
**demangling** decodes them back into a readable signature. On Windows this can be done via
DbgHelp's `UnDecorateSymbolName`; on Linux/macOS, `c++filt` or `abi::__cxa_demangle` do the
equivalent for the Itanium C++ ABI mangling scheme. A DLL with hundreds or thousands of raw
exported symbols is almost never hundreds of hand-written public functions — demangling first is
essential before drawing any conclusion about what's actually exported, because a single class
can contribute a dozen+ export entries (constructors, destructor, and any compiler-generated
helper functions) without a human ever writing that many functions.

---

## 6. Why "load time" is deceptive

The load sequence in section 3 happens **recursively and synchronously**, and this has a
non-obvious consequence: the wall-clock time attributed to loading DLL `A` can include the *full*
cost of loading, relocating, linking, and running the static initializers of DLLs `B`, `C`, and
`D`, if `A` is the first thing in the process to need them.

Concretely: if you measure "how long did it take to load `A`" from the outside (wrapping the
`LoadLibrary`-equivalent call), and `A`'s import table references `B` which isn't loaded yet, your
measurement necessarily includes all of `B`'s load cost too — and if `B` imports `C`, that's
included as well. None of this is visible from `A`'s own file size, its own export count, or even
its own *direct* import count, because those describe `A`'s structure, not the state of the rest
of the process at that moment.

This produces a very specific, often confusing symptom: **two structurally similar DLLs, from the
same component, loaded moments apart, can have wildly different load times** — not because either
one is badly built, but because whichever one happens to load *first* pays the one-time cost of
bringing up a shared dependency (and its static initializers), while the second one finds
everything already resident and pays almost nothing. If you only look at the slow module's own
metrics, you will never find the cause, because the cause isn't in that module at all — it's in
whatever it happened to be the first to touch.

**Loader lock**: while step 4-7 of the load sequence are running, the OS holds a process-wide lock
to keep the module list consistent. Code running inside a static initializer or entry point is
running *inside* that lock, which is why doing certain things there is dangerous or outright
unsupported: loading other libraries in complex ways, creating a thread and waiting on it,
calling back into arbitrary other libraries that might themselves try to load something — all of
these can deadlock, because they may need a lock that's already held by the very thread trying to
acquire it. This is a well-known category of subtle, hard-to-reproduce startup hangs, entirely
separate from raw load-time performance but sharing the same root cause (too much real work
happening during the load sequence instead of after it).

---

## 7. What actually makes a DLL load slowly

Roughly in order of how often each one turns out to be the real cause:

1. **Transitive dependency loading** (section 6) — the module blamed for the slowness is often
   just the unlucky first one to touch a heavy, not-yet-loaded dependency chain.
2. **Cold disk cache / page faults.** The first time a file's pages are touched after a reboot
   (or after memory pressure evicted them), the OS has to actually read from disk. A re-run
   minutes later, with the same pages still cached, can be dramatically faster for the exact same
   file — this is why "test once" measurements of load time are unreliable, and why you should
   always compare a cold run to a warm run before concluding anything about *why* something is
   slow.
3. **Antivirus / EDR real-time scanning.** Security software commonly hooks library loads and
   scans the file synchronously before allowing execution to proceed — this blocks the loading
   thread. This can be slow independent of file size, especially the first time that exact file
   (by hash) is seen, and especially if the product does a cloud reputation lookup as part of the
   decision (a network round-trip on the load-time critical path). This is one of the most common
   causes of "a tiny file took multiple seconds to load, for no reason I can find in the file
   itself," precisely because the reason isn't in the file at all.
4. **Heavy static initializers.** Global/static C++ objects whose constructors do real work —
   allocating large buffers, parsing files, building lookup tables, spinning up subsystems —
   execute unconditionally at load time, on the loading thread, before anything else can happen.
   This is invisible from the outside; nothing about the PE file's structure tells you a
   constructor is expensive.
5. **Large/deep dependency graphs.** Every additional hop in the dependency chain is another
   potential recursive load. A module with only a handful of *direct* imports can still sit at the
   root of a very large *transitive* tree.
6. **Large import tables**, simply because there are more symbols to resolve — usually a much
   smaller effect than the above, but non-zero, especially multiplied across many DLLs at once
   during a burst of startup loading.
7. **TLS callbacks doing real work** — same category as static initializers, but easier to miss
   since they're less commonly used and run even earlier than the entry point.
8. **Slow or remote storage.** Network drives, virtualized/cloud-synced storage (on-demand
   hydration), or simply a slow disk make every "map the file" and "read a page" step slower,
   compounding with everything above.
9. **Relocation cost**, when a DLL can't load at its preferred base address (common with ASLR)
   and needs its internal absolute addresses patched. Usually minor, but scales with how many
   such fixups exist and how large the binary is.

---

## 8. Security features that add load-time cost

Modern binaries carry several loader-enforced protections, each with a small but real load-time
or runtime cost — worth knowing about so they aren't mistaken for bugs, and so "just disable it"
isn't reached for as a false performance fix:

- **ASLR** (address space layout randomization) — the loader picks a randomized base address
  instead of the file's preferred one, which means relocations almost always have to be applied
  (see above). Disabling it removes that cost but removes a major exploit-mitigation, not a
  reasonable trade-off outside of very specific, contained scenarios.
- **DEP/NX** (data execution prevention) — marks writable pages as non-executable. Enforced by
  the CPU's page tables, effectively free at load time.
- **Control Flow Guard (CFG)** — instruments indirect calls/jumps with a validity check against a
  known-valid call-target bitmap. Small per-call runtime overhead, negligible load-time cost, but
  does add to binary size slightly.
- **SafeSEH / exception handler validation** — the loader/runtime validates that registered
  exception handlers are in a known table before invoking them, preventing a classic exploitation
  technique. Effectively free.
- **Code-signing (Authenticode, etc.)** — the OS loader itself typically does *not* verify a
  signature on every load (that's opt-in, e.g. via application control policies). Its main
  load-time relevance is indirect: unsigned or unknown-reputation binaries are exactly the ones
  antivirus/EDR products are most likely to scan thoroughly or flag for cloud lookup (see section
  7, item 3).

None of these are "the problem" in the vast majority of slow-load investigations — they're small,
constant per-binary overheads. If disabling one measurably fixes a load-time issue, that's usually
a strong signal something *else* (most likely antivirus interaction) was the actual bottleneck and
happened to correlate.

---

## 9. Diagnosing load performance

In order of how much signal they give you relative to effort:

1. **Warm vs. cold comparison.** Run the same load twice, back to back, without a reboot in
   between. If the second run is dramatically faster, you're looking at a caching/IO/AV-scan
   effect, not CPU-bound work in the binary. If both runs are equally slow, it's genuine work
   happening somewhere in the chain (static initializers, entry point, or a dependency's).
2. **OS-level tracing** — this is the only way to see *inside* a single load call and get a
   breakdown of every nested library load, every scan, every page fault, with real timestamps.
   On Windows: Event Tracing for Windows (ETW), captured with Windows Performance Recorder and
   viewed in Windows Performance Analyzer. Equivalents exist on other platforms (see section 11).
3. **Process/file activity monitoring** — general-purpose tools (e.g. Sysinternals Process
   Monitor on Windows, `strace`/`ltrace` on Linux) that show every file read, every module load,
   and — critically — let you spot an antivirus/EDR process's activity overlapping in time with
   your slow load window.
4. **Static inspection of the binary itself** — even without running anything, a binary's own
   headers/sections/import table tell you about its *structure*: how many direct dependencies it
   has, whether they're heavy-sounding subsystems, section entropy (a proxy for packed/compressed
   content), whether debug info is present, whether it's signed. Tools: `dumpbin` (MSVC toolchain,
   Windows), `objdump`/`readelf` (Linux), `otool` (macOS), or a PE/ELF/Mach-O parsing script.
   This won't show you *dynamic* costs (static initializer runtime, transitive load order at
   runtime), but it narrows down what's structurally plausible.
5. **Bisection by disabling suspects one at a time** — temporarily excluding a directory from
   antivirus real-time scanning (if policy allows, in a controlled dev environment), moving the
   binary to local vs. network storage, or commenting out a suspected heavy static initializer,
   and re-measuring. Crude but conclusive.

---

## 10. Mitigation strategies

- **Reduce direct dependencies.** Fewer imports mean fewer things that might need a fresh,
  synchronous load. Check whether a declared dependency is actually used, or a leftover.
- **Delay-load rarely-used heavy dependencies** so their cost is paid on first actual use instead
  of unconditionally at startup.
- **Keep entry points and static initializers minimal.** Do the minimum needed to make the module
  safely loadable; defer real initialization (allocating large structures, reading config,
  spinning up subsystems) to an explicit function called *after* the load sequence completes and
  the loader lock is released.
- **Avoid unnecessary TLS callbacks** for the same reason.
- **Be deliberate about what's exported.** A smaller, intentional export surface (vs. exporting
  entire classes by default) is faster to search, easier to reason about for ABI stability, and
  avoids leaking implementation details across a module boundary.
- **Consider consolidating very fine-grained DLL splits.** Many small DLLs each carry the fixed
  overhead of file lookup, mapping, and import resolution; a smaller number of coarser DLLs (or a
  fully static/monolithic build for shipping) trades runtime flexibility for fewer load-time round
  trips. This is a real architectural trade-off, not a free win — plugin-style extensibility
  generally requires the fine-grained split.
- **Keep binaries on fast, local, "hot" storage** during development, and make sure they're
  excluded/whitelisted from real-time AV scanning where policy allows — this is often the single
  biggest and cheapest win for iteration-time load performance, and the easiest to overlook
  because it's not a code change.
- **Always benchmark warm vs. cold**, and re-benchmark after any change — load-time behavior is
  sensitive to process/machine state in ways that make single measurements unreliable.

---

## 11. Other platforms

The concepts above (symbols, exports/imports, lazy vs. eager resolution, static initializers,
transitive loading) apply everywhere; only the container format and tooling names differ.

| Concept | Windows | Linux | macOS |
|---|---|---|---|
| Library file | `.dll` | `.so` | `.dylib` |
| Container format | PE (Portable Executable) | ELF | Mach-O |
| Load API | `LoadLibrary` | `dlopen` (explicit) / dynamic linker (implicit) | `dlopen` / dynamic linker |
| Entry point | `DllMain` | constructor/destructor attributes (`__attribute__((constructor))`) | same as Linux |
| Symbol table inspector | `dumpbin` | `readelf`, `nm`, `objdump` | `otool`, `nm` |
| Demangler | DbgHelp `UnDecorateSymbolName` | `c++filt` | `c++filt` |
| Lazy binding | delay-load imports (opt-in) | lazy PLT binding (default, via `LD_BIND_NOW` to disable) | lazy binding (default) |
| OS-level trace | ETW / WPA | `strace`, `perf`, `LD_DEBUG=all` | `dtrace`, `fs_usage` |
| ASLR | `DYNAMIC_BASE` flag | PIE (position-independent executable) | enabled by default (PIE) |

The Linux dynamic linker defaults to **lazy binding** for most symbols (resolved on first call,
similar in spirit to Windows delay-loading, but the default rather than opt-in), which changes
where in the timeline symbol-resolution cost shows up compared to Windows' default of resolving
everything eagerly at load time.

---

## 12. Glossary

- **PE / COFF** — Portable Executable / Common Object File Format: the container format for
  Windows executables and DLLs.
- **RVA** (Relative Virtual Address) — an address expressed as an offset from the image's base
  address, rather than an absolute address; used throughout PE so the file works no matter where
  it ends up loaded.
- **Image base** — the preferred load address baked into the file at build time.
- **Section** — a named, permission-tagged region of the file (`.text`, `.data`, etc.).
- **Export table** — a DLL's list of symbols it makes available to others.
- **Import table** — a DLL's list of symbols it needs from other DLLs.
- **IAT** (Import Address Table) — the array of pointers that import calls indirect through;
  filled in by the loader at load time.
- **Ordinal** — a small integer identifying an export, as an alternative to a name.
- **Forwarder** — an export that redirects to another DLL's export instead of pointing to code.
- **Relocation** — a fixup applied to an absolute address when a binary loads somewhere other than
  its preferred image base.
- **ASLR** — Address Space Layout Randomization; randomizes load addresses as a security measure.
- **DEP / NX** — Data Execution Prevention / No-eXecute; marks data pages non-executable.
- **CFG** (Control Flow Guard) — validates indirect call targets against a known-good set.
- **TLS callback** — a function registered to run at thread/process attach, before the entry
  point.
- **Loader lock** — a process-wide lock held during module load/unload bookkeeping; code running
  inside it has restricted rules about what it can safely do.
- **Static/global initializer** — constructor logic for a global or static object, run before the
  entry point.
- **Symbol** — a named function or variable that can be referenced across a compilation/module
  boundary.
- **Mangling / demangling** — encoding a C++ name's full signature into a linker-safe flat string,
  and decoding it back to something human-readable.
- **Delay-loading** — deferring resolution/loading of a declared dependency until its first actual
  use, instead of at load time.
- **Entropy** (in this context) — a statistical measure (0-8 bits) of how "random-looking" a
  section's bytes are; unusually high entropy is a common signal of compressed, encrypted, or
  packed content.

# Example AAA Unreal Engine game project breakdown

## Raw numbers

- Each module is a DLL
- There are 1929 modules

Total DLL loading time: 36000ms
Average DLL loading time: 19ms

Total DLL file size is 1800mb (almost 2gb)

StartupModule is just a code function is each module that is getting called after loading (Unreal related)

On average each DLL has dependency on 16 other DLLs.
Where 6 of them are always OS related DLLs like:
- KERNEL32.dll
- VCRUNTIME140.dll
- VCRUNTIME140_1.dll
- api-ms-win-crt-string-l1-1-0.dll
- api-ms-win-crt-math-l1-1-0.dll
- api-ms-win-crt-runtime-l1-1-0.dll

## A representative module: UnrealEditor-RedRoads.dll

Full analysis: [UnrealEditor-RedRoads_dll_analysis.md](UnrealEditor-RedRoads_dll_analysis.md)

- File size: 479,744 bytes (~0.46 MB) — about half the project's average of ~0.93 MB/DLL
- 13 direct dependencies (7 real modules + 6 OS/CRT DLLs) — slightly below the project's 16-dependency average
- 407 exports
- Structurally unremarkable by every static metric — exactly the profile of the large majority
  of modules in a project this size: small, few dependencies, loads in a few milliseconds, and
  never shows up in a load-time profiling report. It's included here as a "typical" data point,
  not because it's slow.

## Derived numbers / sanity checks

- **Average file size per DLL**: 1,800 MB / 1,929 ≈ **0.93 MB**.
- **19 ms × 1,929 modules ≈ 36,651 ms** — matches the stated 36,000 ms total closely enough to
  confirm these per-module times are effectively summed on a single critical path (sequential
  module loading during boot), not overlapped/parallelized across modules.
- At normal SSD/NVMe throughput, physically reading 1.8 GB off disk takes a small fraction of a
  second. Spending 36 seconds to move that much data, spread across 1,929 files, means the
  bottleneck is **not bytes read** — it's the *fixed per-DLL overhead* (locate the file, parse
  PE headers, walk the import table, resolve every symbol, apply relocations, set page
  permissions, run static initializers, possibly get intercepted by AV/EDR) paid up to 1,929
  times, plus whatever a smaller number of genuinely expensive outliers cost.
- **16 average dependencies × 1,929 modules ≈ 31,000 dependency edges** in this project's DLL
  graph (before deduplication). A graph that large is exactly the regime where transitive,
  recursive dependency loading — see the general
  [DLL Loading Performance 101](dll_loading_performance_101.md) writeup, section 6 — has room to
  compound: loading one module can silently trigger loading a chain of others that aren't
  resident yet.

## Where the 36 seconds actually goes

- A 19 ms *average* with a 36-second total strongly implies a long-tail distribution, not a
  uniform one: most of the 1,929 modules (like `RedRoads` above) cost low single-digit
  milliseconds, while a much smaller number of outliers — whichever modules happen to be first in
  the boot sequence to touch a heavy, not-yet-loaded shared subsystem — account for a
  disproportionate share of the total. This isn't hypothetical for this project: two structurally
  near-identical modules from the same plugin were directly observed differing by more than 3x in
  load time, with a 560 MB vs. 0-byte difference in memory delta, purely because of which one
  happened to touch a shared dependency first.
- Consequence: optimizing "the average module" (shrinking every DLL a little, trimming every
  export table a little) has low leverage here — file size and export/import *counts* don't
  meaningfully predict load cost at this scale. Optimizing the handful of true outliers has
  outsized leverage, because that's where the actual seconds live.
- `StartupModule` cost (this project's own post-load init function per module) is a separate,
  *additive* line item on top of the OS-loader cost described above. It's squarely each module
  owner's responsibility to fix, unlike the OS-loader-side transitive-dependency cost, which is
  more of an architecture/build-topology problem (how many DLLs exist, and what depends on what).
- Practical next step: rank all 1,929 modules by total load time (which the profiler already
  produces) and focus investigation on the top-N outliers only — the bulk of the 36 seconds is
  almost certainly concentrated there, not spread evenly across the corpus.