# Subsecond: Live Patching for Rust

## 101 — What Is Subsecond?

Subsecond is the hot-patching engine behind Dioxus's `dx serve --hotpatch` mode. It lets you edit **Rust logic** (not just markup) in a running app and see the change take effect in well under a second, without restarting the process and without losing in-memory state.

This matters because normal Rust hot-reload tools (including Dioxus's own standard `dx serve`) can only cheaply reload `rsx! {}` markup/text/style changes — anything that touches real Rust code (a new `if` branch, a changed hook, new logic) normally forces a full recompile (2–10s) and, for a desktop/mobile app, a process restart. Subsecond is an attempt to close that gap for actual Rust code, not just templates.

It's maintained by the Dioxus team but is framework-agnostic — Bevy and Iced have also integrated it.

**Status**: experimental. It is not the default (`dx serve` alone does *not* enable it); you opt in with `--hotpatch`, and the docs are explicit that it "may cause crashes or unexpected behavior."

---

## The Core Idea: Indirection, Not Memory Rewriting

The obvious approach — overwrite the running process's executable memory in place — was tried and abandoned. If the program is blocked on I/O (waiting on a socket, a mutex, etc.), there's no safe moment to splice in new machine code: *"the program would always be waiting for IO, making our edits not take effect."*

Instead, subsecond routes every hot-reloadable function call through a **jump table**: a runtime table mapping function IDs to their *current* address. Patching a function means updating one table entry — never touching the bytes of the original function.

```
call site → look up current address in jump table → jump there
```

No existing executable memory is ever modified. New code lives in new memory; only the pointer changes.

---

## The Jump Table In Depth

The jump table is the single piece of shared state that makes the whole scheme work — it's the only thing that actually changes when a patch lands. It's defined as a `JumpTable` struct (in the `subsecond_types` crate) and shipped from the dev server to the running app after every successful incremental build. Known fields and their roles:

| Field | Purpose |
|---|---|
| `lib` | Path to the compiled patch library file (the small relocatable object/dylib holding the new code) |
| `aslr_reference` | The dev server's build-time reference address for `main`, used to compute the ASLR offset (see below) |
| `new_base_address` | The base address the freshly loaded patch library ends up at once mapped into the process |
| `map` | A `HashMap` from old function address → new function address — the actual repointing table |
| `ifunc_count` | Count of indirect-function-table entries, relevant to the WASM path (see below) |

**Applying a jump table**, roughly:
1. Load the library at `table.lib` into the process (native: `mmap` as executable pages at/near `new_base_address`; WASM: instantiate/extend the module and register new entries).
2. Compute the ASLR offset: `aslr_reference() - table.aslr_reference` (covered in detail further down).
3. Walk `table.map`, and for each `(old_addr, new_addr)` pair, adjust `new_addr` by the ASLR offset and atomically store it as the *current* address for that function.

**Why "jump table" and not just "patch the function"**: every call site subsecond instruments — whether via `subsecond::call()` or Dioxus's automatic wrapping of component renders — doesn't call the target function directly. It dereferences a slot (an `AtomicPtr`) that's looked up in this table at call time. Updating one entry in `map` is enough to redirect *every subsequent call* to that function throughout the whole process, instantly and without touching any existing call sites' compiled code.

**The WASM wrinkle (`ifunc_count`)**: WebAssembly doesn't expose raw function pointers the way native code does — indirect calls go through a `call_indirect` instruction referencing a numbered slot in the module's function table. So on the WASM target, "updating the jump table" means growing that function table and recording new entries (element segments) rather than writing a pointer — `ifunc_count` tracks how many such indirect-function-table slots the patch is adding.

---

## ThinLink In Depth

ThinLink is the custom linker component the Dioxus CLI substitutes for your platform's normal linker (`ld`/`lld`/`link.exe`) whenever you run `dx serve` (and especially `dx serve --hotpatch`). The project describes it simply as *"a wrapper around your existing linker"* — it still delegates to the real linker for the parts of the job that need one, but does as little of that as possible on each incremental build.

It has three jobs:

1. **Automatic dynamic linking of dependencies.** Your dependency crates get linked once as dylibs and then left alone — they're never relinked just because you touched a line in your own app crate. This is a large part of why incremental builds stay small: the dependency graph (`tokio`, `wgpu`, whatever else you depend on) isn't in the hot path at all.
2. **Object-file diffing for function invalidation.** After rustc's incremental compiler regenerates the handful of codegen units affected by your edit, ThinLink diffs those `.o` files symbol-by-symbol against the previous build to determine exactly which function bodies changed — narrowing "this codegen unit was touched" down to "these specific functions differ."
3. **Jump table generation.** For each function that genuinely changed, it emits the `JumpTable` structure described above — resolving the changed function(s) against addresses already known from the live, running process rather than performing a conventional whole-project link.

Because it only ever links the small diffed delta — and resolves it against an address space it already knows, rather than re-resolving symbols project-wide — incremental builds through ThinLink commonly land in the 200ms–500ms range in dev profiles, instead of the multi-second full rebuild a normal linker invocation would require.

**Availability**: ThinLink is currently baked directly into the Dioxus CLI. It isn't (yet) published as a standalone linker you could point at a non-Dioxus project by hand — using it means going through `dx serve`.

---

## Why New Code Needs New Memory: Executable Pages & W^X

Process memory is organized into pages, each carrying R/W/X permission bits enforced by the MMU. Modern systems enforce **W^X** (write XOR execute): a page is never simultaneously writable and executable, precisely to stop an attacker from writing shellcode and then jumping to it.

This is why "patch in place" isn't viable: the memory holding a compiled function is (correctly) not writable while executable. So the patched code has to arrive as **new** executable pages instead:

- **Native (macOS/Linux/Windows)**: the compiled patch is mapped directly into the running process as fresh executable pages (`mmap`-based). On Android this is specifically noted as avoiding a requirement for root, implying a direct memory-mapping approach rather than routing through the OS's full dynamic-loader/`dlopen` path each time.
- **WASM**: there's no live process to `mmap` into in the same sense. Instead, the new function's compiled code is linked into a relocatable object, and its symbols are "lifted" into the WASM module's **export table**, with new **element segments** recorded — growing the already-instantiated module's function table rather than reloading the whole module.

Either way: the original function's page is left alone. Only the jump table is updated (via an `AtomicPtr`, so the swap is a single atomic write — safe even if another thread is mid-call through the table).

---

## The Build Pipeline: From Keystroke to Patched Process

1. **File watcher** (`notify` crate) detects a saved source file.
2. **Incremental rustc invocation.** The Dioxus CLI substitutes a custom linker, **ThinLink**, for the platform linker. It leans on rustc's own incremental-compilation cache: each crate is already split into multiple **codegen units (CGUs)** — a stock Rust mechanism for parallelizing builds, not something subsecond invented. rustc fingerprints each CGU's inputs and only re-codegens the ones that actually changed, producing a handful of fresh `.o` files instead of recompiling the whole crate.
3. **Object-file diffing.** A CGU being "touched" doesn't mean every function inside it changed. ThinLink diffs the freshly regenerated `.o` file(s) against the previous build's version, symbol by symbol / at the assembly level, to isolate exactly *which function(s)* genuinely differ.
4. **Incremental linking, not a full link.** For just the changed functions, ThinLink resolves their external references directly against the addresses already known from the *running* process (see ASLR section below) — instead of doing a conventional whole-project link. This is the main reason it's fast: linking a handful of functions against an already-known address space, not re-resolving symbols project-wide. Dependency crates stay as pre-built dylibs and are never touched.
5. **Load into the live process** as new executable pages (native) or new export/element-table entries (WASM) — see above.
6. **Jump table update, sent over a WebSocket** from the dev server to the running app (referred to as the "DUT," device under test) as a message (`HotPatchStart` / `HotReloadMsg`), rather than pausing the process with OS signals.
7. **The app applies it** — atomically swapping the relevant jump-table pointer(s).

Reported performance: commonly sub-200ms to ~500-600ms incremental builds on typical hardware (e.g. M1 Mac benchmarks cited by the project).

---

## Handling Code That's Already Executing: Stack Rewinding

If the function that changed is currently on the call stack — you're mid-render when you edit the component doing the rendering — the old stack frame is still live and can't just have its target swapped underneath it.

Subsecond's answer: explicit integration points via `subsecond::call(|| { ... })`, marking places that are safe to re-enter. When code *above* a call point changes, subsecond triggers a panic that's automatically caught and retried at the nearest enclosing `subsecond::call()` boundary further up the stack — unwinding to that point and re-invoking it, which now dispatches through the jump table to the new code.

These boundaries also serve as **synchronization points** where a framework can safely do bookkeeping around a patch — closing TCP connections, re-instancing state, dropping event listeners — rather than mutating things mid-flight. Dioxus wraps component rendering in these automatically, so app code doesn't need to add its own `subsecond::call()` sites in typical use.

---

## ASLR: Why an Address Offset Is Needed at All

**ASLR (Address Space Layout Randomization)** is an OS security feature that randomizes where a process's segments load in virtual memory on each run, to defeat exploits that hardcode addresses. Consequence: the compiler has no idea what a function's *actual* runtime address will be — only an offset relative to the binary's own assumed layout.

Subsecond's fix, using `main` as a fixed anchor:

**On the running app**, it resolves `main`'s live address dynamically:
```rust
// Unix
MAIN_PTR = libc::dlsym(libc::RTLD_DEFAULT, c"main".as_ptr() as _);

// Windows
MAIN_PTR = GetProcAddress(GetModuleHandleA(std::ptr::null()), c"main".as_ptr() as _) as _;
```
`dlsym(RTLD_DEFAULT, "main")` / `GetProcAddress(...)` ask the dynamic linker to resolve the symbol as it's *actually* bound right now, post-ASLR, in this specific process launch.

**The dev server** embeds its own build-time notion of `main`'s address in the `JumpTable` message it sends (`table.aslr_reference`).

**On patch application**, the app computes the slide with one subtraction:
```rust
let old_offset = aslr_reference() - table.aslr_reference as usize;
```
That offset gets applied to every other address in the incoming jump table before use. It only needs to be computed once per process launch — the OS picks one random base per run, not per function or per patch.

---

## What Subsecond Deliberately Does Not Handle

This is the sharpest edge of the system, worth stating plainly:

- **Struct/type layout changes are not detected — they're just unsafe.** There is no layout-hash or `size_of` gate. Docs state directly: if layout/alignment changes and new code reads an old instance using the new (different) offsets, *"the program will crash."* The one runtime check that exists — `TypeId` + `ptr_address` on `HotFn` — identifies whether a function pointer has been repointed (for cache/memoization invalidation), **not** whether a type's memory layout is still compatible; `TypeId` doesn't change just because a field was added. Mitigation is architectural, not automatic: frameworks are expected to dispose of or re-instance affected state themselves. Dioxus's approach is to discard and rebuild component state rather than reuse old bytes in place.
- **Statics/globals**: tracked across patches, but destructors never run on patch, and renaming one creates a new global rather than reusing storage. Changes to static initializers aren't observed.
- **Thread-locals**: the weakest point — they reset to their initial value on each patch, because the new code's TLS binding doesn't address the original storage. Can cause real bugs in code relying on persistent thread-local state.
- **Only the "tip"/binary crate is patched.** Subsecond hot-patches code in the crate you're running via `dx serve`, not arbitrary downstream library crates in the dependency graph.

---

## Why This Doesn't (and Can't) Work Like Vite's JS HMR

Vite's HMR works because JS is dynamically loaded/interpreted — swapping a module is just re-executing a script. Rust is ahead-of-time compiled to native code or WASM, so there's no equivalent free lunch:

- **Tier 1** (`dx serve`, stable): fake it by only diffing something structured like `rsx! {}` markup and patching the template representation in place. Doesn't touch arbitrary Rust logic.
- **Tier 2** (`dx serve --hotpatch`, experimental, subsecond): actually recompile-and-relink a small fragment of machine code and splice it into the live process via the jump table — the mechanism this document describes.

There is no way to get JS-style universal instant reload for arbitrary Rust changes; subsecond is the closest attempt shipped in the Rust ecosystem so far.

---

## Quick Reference: The Whole Chain in One Table

| Step | Mechanism | Why |
|---|---|---|
| Detect a change | rustc incremental compilation (CGU fingerprinting) | Only touched codegen units get recompiled |
| Isolate what changed | ThinLink object-file/symbol diffing | A touched CGU ≠ every function in it changed |
| Link the delta | Incremental link against known running addresses | Avoids a full project-wide link |
| Get real addresses | ASLR offset via `dlsym`/`GetProcAddress` on `main` | Compiler doesn't know the randomized runtime address |
| Load new code | New executable pages (native `mmap`) / export table growth (WASM) | W^X forbids patching existing executable memory |
| Apply the patch | Atomic jump-table pointer swap | Safe without stopping the process |
| Handle live stack frames | Panic + unwind to nearest `subsecond::call()` | Can't swap the target of a frame already executing |
| Struct/layout changes | **Not handled** — framework must dispose/re-instance | No layout-compatibility check exists |

---

## Reproduced In Practice: A Windows Desktop Crash

This isn't theoretical — we hit a real, 100%-reproducible crash testing `dx serve --platform desktop --hotpatch` against a minimal Dioxus 0.7.10 desktop app on Windows 11, and it's worth recording since no matching upstream report was found.

**Setup**: `dioxus-ui-showcase`, a single-`main.rs` desktop app (no workspace split, no async component logic), `dx` CLI 0.7.10 matched exactly against `dioxus`/`subsecond`/`subsecond-types` 0.7.10 in `Cargo.lock` (a version skew between CLI and crate was caught and fixed first — see below — but turned out not to be the cause of this crash).

**Repro — three independent edits, three identical crashes:**

| # | Edit | Hot-patch build | Runtime result |
|---|---|---|---|
| 1 | `count() ± 1` → `± 5` inside two `onclick` closures | succeeded, 2864ms | `thread 'main' has overflowed its stack` → process exit `0xc00000fd` |
| 2 | Same kind of edit, re-run with only **one** `dx serve` instance running (see below) | succeeded, 2423ms | identical crash |
| 3 | `count().clamp(0, 100)` → `clamp(0, 90)` — plain arithmetic, **no closure involved** | succeeded, 10718ms | identical crash |

Every case: ThinLink's diff/compile/link pipeline genuinely succeeds (`Hot-patching: src\main.rs took Xms` is a real, successful patch), and the crash happens immediately after, in the *runtime application* of the patch — not in the build.

**Confounds we ruled out before trusting this result:**
- **CLI/crate version mismatch.** First launch warned `dx version: 0.7.10` vs `dioxus versions: [0.7.9]` (a freshly-installed CLI against an older locked crate version). Fixed with `cargo update -p dioxus` to bring the whole `dioxus`/`subsecond` family to 0.7.10. The crash persisted after fixing this, so it wasn't the cause — but it's a real setup trap worth checking for independently, since a CLI/crate skew is exactly the kind of thing that *could* break the jump-table protocol.
- **Duplicate dev servers.** `Get-CimInstance Win32_Process` revealed two separate `dx.exe serve --platform desktop --hotpatch` processes running concurrently against the same project (from two different terminals). Since subsecond's design assumes exactly one dev server paired with exactly one running app (one ASLR reference, one jump-table stream), this is a genuine unsupported hazard — but killing the duplicate and retesting with exactly one server produced the *identical* crash, ruling it out as the cause here.
- **Closure-layout hypothesis.** Rust closures are anonymous `repr(Rust)` structs with no ABI/layout stability guarantee across separate compiler invocations, and subsecond has no layout-compatibility detector (confirmed — see "What Subsecond Deliberately Does Not Handle" above). This looked like a strong candidate since the first two crashing edits were both inside `onclick` closures. Edit #3 (pure arithmetic, no closure) killed this theory too: same crash, no closure in sight.

No existing GitHub issue was found matching this exact signature (`thread 'main' has overflowed its stack` + `0xc00000fd` immediately following a successful `Hot-patching: ... took Xms` line) — the closest-titled issue, [#4150](https://github.com/DioxusLabs/dioxus/issues/4150) ("Subsecond fails on windows in a minimal setup"), turned out to be an unrelated, already-fixed link-time `rust-lld`/`link.exe` selection bug from the 0.7.0-alpha era.

### Getting a real answer: analyzing the actual crash dump

Rather than stop at hypothesis, Windows had already captured full minidumps for every crash in `%LOCALAPPDATA%\CrashDumps` (default WER behavior), and PDB symbols existed for both the main exe and — notably — for the hot-patch's own dynamically-generated library (`libdioxus-ui-showcase-patch-<timestamp>.pdb`). Loading a dump in `cdb.exe` (Windows SDK Debugging Tools) with `!analyze -v` gave a fully symbolicated stack trace:

```
dioxus_ui_showcase_1d3759a6!__chkstk+0x37                                    ← crash site: stack exhausted
libdioxus_ui_showcase_patch_1785753671274!dioxus_ui_showcase::App+0xb        ← App, running from the PATCH dll
subsecond::HotFn<...>::call_as_ptr<...>                                      ← jump-table dispatch
subsecond::HotFn<...>::try_call<...>
subsecond::HotFn<...>::call<...>
dioxus_core::properties::impl$5::rebuild<...>
dioxus_core::any_props::impl$2::render<...>
dioxus_core::scope_arena::...::run_scope::closure$0 (×3 nested)
dioxus_core::reactive_context::ReactiveContext::run_in / reset_and_run_in
dioxus_core::runtime::Runtime::with_scope_on_stack<...>
dioxus_core::virtual_dom::VirtualDom::run_scope / run_and_diff_scope / render_immediate
dioxus_desktop::webview::WebviewInstance::poll_vdom
dioxus_desktop::app::App::handle_hot_reload_msg
tao event loop → main
```

Two hard facts fall out of this:
1. **`ERROR_CODE: 0xc00000fd - A new guard page for the stack cannot be created`** — the thread's *entire* reserved stack is genuinely exhausted, not corrupted.
2. **The `STACK_TEXT` is a single, non-repeating call chain** (~35 frames) — not a recursive loop. The `subsecond::HotFn::call → try_call → call_as_ptr` frames sit directly on top of `App`, meaning every hot-patched call pays a small, fixed amount of extra stack for subsecond's dispatch indirection, on top of Dioxus-desktop's already-deep debug-mode render chain (`run_scope` → `reset_and_run_in` → `run_in` → `catch_unwind` → `render`, roughly a dozen frames of framework machinery *per rendered component*).

Cross-checking Windows Event Viewer across all crashes: every stack-overflow crash landed at the **exact same fault offset** (`0x00000000022a43d7`) across four independent process launches — proof this is deterministic, not flaky. One crash was a different exception (`0xc0000005`, access violation) with `Faulting module name: unknown` — consistent with a fault inside the freshly-`mmap`'d patch code itself, which isn't registered as a normal PE module.

### Testing the fix: does more stack help?

Verified the reserved-stack theory directly: relaunched with `dx serve --platform desktop --hotpatch --rustc-args="-Clink-arg=/STACK:8388608"` (8MB vs. Windows' ~1MB default), and **confirmed via direct PE-header parsing** that the flag took effect (`SizeOfStackReserve: 8388608 bytes`). The crash was byte-for-byte identical — same message, same exit code, similar timing. An 8x stack increase making zero difference doesn't necessarily prove infinite recursion (a deep-but-finite tree could need more than 8MB too, especially with ~100+ components each paying ~14 frames of framework overhead in a debug build) — but it does rule out "just needs a little more headroom."

### Isolating the trigger: minimal-repro elimination

To find out whether this is a fundamental subsecond/Dioxus-desktop/Windows bug or something specific to `dioxus-ui-showcase`, a fresh minimal project (`hotpatch-minimal`: one `use_signal` counter, one button) was hand-built and tested the same way. Result: **the hot-patch worked correctly** — no crash, process survived under the same PID, and the behavior change (increment step) was confirmed live in the running window. This falsifies "hotpatch is fundamentally broken on this Windows machine."

From there, ingredients from the showcase were added back one at a time, retesting after each:

| Ingredient added | Result |
|---|---|
| Baseline: one component, one signal, one closure | No crash |
| ~48 nested components across 4 levels (`Box → Header/Content/Footer → Leaf`, mirroring the showcase's `Card` composition density) | No crash |
| `Signal<i32>` + `EventHandler<i32>` passed as props into a nested component (mirroring `ReactiveSection`'s exact pattern, instead of plain closures in the root) | No crash |
| `use_future` + `dioxus::document::eval()` async JS injection on startup (mirroring the Tailwind CDN injection) | No crash |
| `tw_merge!()` (backed by the `nom` parser-combinator crate for runtime Tailwind class-conflict resolution) added to every nested component — mirroring the real `ui/card.rs`/`ui/button.rs`, where **every single sub-component** (`Card`, `CardHeader`, `CardTitle`, `CardDescription`, `CardContent`, `CardFooter`, `Button`, ...) calls it at least once | No crash |

None of these — individually or all stacked together in the same build — reproduced the crash, even though each was hot-patched successfully and each edit was a genuine Rust-logic change of the same kind that reliably crashed `dioxus-ui-showcase` every time. `tw_merge`/`nom` was a strong suspect (parser-combinator code is notorious for deep, heavily-closure-nested, poorly-inlined call chains in debug builds — potentially adding many stack frames *per component*, exactly the kind of thing that could tip an already-borderline debug-mode render chain over the edge) — but it didn't reproduce the crash either, even applied across every nested component in the tree.

The investigation was stood down at this point rather than continuing to bisect further (e.g., copying the real `ui/button.rs`/`ui/card.rs` component library wholesale, doubling the component count toward the showcase's real ~90-120, or bisecting the showcase itself by commenting out sections) — the exact minimal trigger remains unidentified.

**Conclusion**: this is a **real, deterministic, symbolicated crash** — not flakiness, not a setup mistake, and not simply "hotpatch is broken on Windows desktop" (the minimal app disproves that). It reproduces reliably in `dioxus-ui-showcase` specifically, involves subsecond's `HotFn` dispatch layer sitting directly on top of `App`'s render call chain at the moment the stack is exhausted, and survives an 8x stack-size increase. Every specific ingredient tested in isolation — deep nesting, `Signal`/`EventHandler` props, async JS interop, `tw_merge`/`nom` parsing — failed to reproduce it, individually or combined, at the scale tested (~48-50 component instances). The most defensible remaining hypothesis is therefore **cumulative scale rather than any single ingredient**: `dioxus-ui-showcase`'s full component tree (~90-120 nested component renders in one synchronous pass, roughly double what was tested here) combined with subsecond's added dispatch frames may exceed even a generously-sized debug-mode stack, where the minimal repro's ~50 components did not. This was not confirmed — scaling the minimal repro up to a comparable component count is the natural next experiment for anyone picking this back up.

**Practical takeaway**: `--hotpatch` is not reliable for `dioxus-ui-showcase` on this Windows desktop setup — every real Rust logic edit tested crashed it, and neither a duplicate-server fix nor an 8x stack increase resolved it. It **does** work for simpler component trees on the same machine, same Dioxus version, same platform — so this is not a blanket "avoid `--hotpatch` on Windows" conclusion. Plain `dx serve` (without `--hotpatch`) remains the reliable fallback for this specific app: instant markup/style reload, full rebuild + restart for Rust logic changes.

---

## Sources

- [Subsecond: Hot-patching for Rust (lib.rs)](https://lib.rs/crates/subsecond)
- [subsecond — docs.rs](https://docs.rs/subsecond/latest/subsecond/)
- [subsecond source (lib.rs.html)](https://docs.rs/subsecond/latest/src/subsecond/lib.rs.html)
- [Dioxus 0.7 release blog](https://dioxuslabs.com/blog/release-070/)
- [Dioxus hot-reload docs](https://dioxuslabs.com/learn/0.7/essentials/ui/hotreload/)
- [Binary patching rust hot-reloading, PR #3797](https://github.com/DioxusLabs/dioxus/pull/3797)
- [Discussion: "Ignoring hotpatch since there is no ASLR reference"](https://github.com/DioxusLabs/dioxus/discussions/4181)
- [bevy_simple_subsecond_system docs](https://docs.rs/bevy_simple_subsecond_system)
