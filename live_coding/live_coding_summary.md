# live_coding — Summary

A **Live++ / Unreal Engine Live Coding** style hot-reload for Rust on Windows.
The core idea: instead of routing every game call through a jump table (the
`subsecond` approach), this runtime **patches the function prologue in place** —
the first 5 bytes of an exported function are overwritten with a `jmp rel32`
that redirects to freshly compiled code. Existing callers keep their original
function pointer and are transparently redirected. No wrapper, no lookup, zero
call overhead after the patch.

---

## 1. Project layout

```
live_coding/
├── Cargo.toml            workspace: [live_patch, patch_dll]
├── README.md             quick-start + mechanism overview
├── live_coding_summary.md   this file
├── live_patch/           the runtime (lib) + host (bin)
│   └── src/
│       ├── lib.rs        Win32 FFI, prologue patching, trampolines, rustc
│       │                 single-function compilation, symbol registry
│       └── main.rs       host: watch loop, source parser, diffing, error handling
└── patch_dll/            the "game" DLL being hot-reloaded
    └── src/
        ├── lib.rs        exports update() (has deps) and compute() (leaf)
        └── test.rs       helper module
```

Two crates, zero external dependencies. Everything is raw Win32 FFI + `rustc`
invoked as a subprocess.

---

## 2. How it works

### 2.1 The three ways a change can be applied

When you edit `patch_dll/src/lib.rs` and save, the host classifies each changed
function by name:

| Function | Kind | What happens |
|---|---|---|
| `compute` | **Leaf** (no internal deps) | Compiled standalone via `rustc` (~150–180 ms), loaded, and the **prologue of the live function is patched in place**. Base DLL stays loaded. |
| `update` | **Has internal deps** (calls `alpha()`, `test::alpha()`) | Triggers a **full `cargo build` + DLL reload** (~385 ms). A leaf can't reference non-exported crate items, so only dependency-free bodies can be patched standalone. |
| anything else (extern) | **New function** | Compiled, loaded, and **registered by name** in the runtime symbol registry — resolvable at runtime even though it never existed at startup. |
| structs / internal helpers / test.rs / new modules | **Non-extern change** | Detected by whole-directory watching and triggers a **full `cargo build` + reload** — normal compilation, so anything is supported. |

### 2.2 The hot-reload loop (`main.rs`)

1. **Cold build** — `cargo build -p patch_dll --target-dir target_reload`, with
   compiler output captured (see §2.5).
2. **Load once** — the base DLL is copied to a unique temp path and loaded
   with `LoadLibraryW`; `update` / `compute` are resolved into **typed `HotFn`
   wrappers**.  These are the *only* call sites in the loop:
   `unsafe { update_fn.call(tick) }` and `unsafe { compute_fn.call(x, y) }`.
   The host also hands the DLL its service table (`set_services`) so game code
   can reach host-owned state.  No jump table.
3. **Watch** — content-hash every `.rs` under `patch_dll/src/` plus
   `Cargo.toml` and `build.rs` every 800 ms; any change is detected.  A full
   rebuild is triggered by any non-extern change (structs, internal helpers,
   `test.rs`, new modules) or by `update` changing; pure leaf-body edits take
   the fast prologue-patch path.
4. **Parse** — `extract_function_bodies()` uses a lexical mini-lexer
   (`code_mask`) that ignores strings, char literals, comments, and raw
   strings while pulling each `pub extern "C" fn NAME(params) -> Ret { body }`
   out of the source.
5. **Diff** — each extracted body is compared against a cache of
   `(name, params, body, ret_type)`. Only genuinely changed functions are
   processed.
6. **Apply** — per-function dispatch as in §2.1.
7. **Cache** — only *successfully applied* changes are committed to the cache,
   so a failed patch or failed build is retried on the next save.

### 2.3 The prologue patch (`lib.rs`)

```
original function               replacement code (fresh DLL)
┌─────────────────────┐         ┌──────────────────────┐
│ ... original body   │         │ new compute code     │
│ E9 <rel32> ─────────┼────────►│                      │
└─────────────────────┘         └──────────────────────┘
        │
        └── if > ±2 GB away: trampoline ──► mov rax, <imm64>; jmp rax
```

The mechanism:

1. Compute `relative = new_address - (old_address + 5)`.
2. If the target is within **±2 GB** (rel32 reach), write a direct `E9 <rel32>`.
3. Otherwise allocate a **trampoline** near the original function
   (`VirtualAlloc` probing ±1 GB in 16 KB strides), holding `48 B8 <imm64> FF E0`
   (`mov rax, imm64; jmp rax`), and have the prologue jump to the trampoline.
4. `VirtualProtect` the page to `PAGE_EXECUTE_READWRITE`, write the 5 bytes,
   `FlushInstructionCache`, then restore the original page protection.

After this, the *same* function pointer the host has held since startup now
executes the new code. All indirect references to the function are redirected
too — that's the key advantage over a jump table.

### 2.4 Standalone single-function compilation

`compile_single_function()` writes a tiny `.rs` to the temp dir:

```rust
#[unsafe(no_mangle)]
pub extern "C" fn compute(x: i32, y: i32) -> i32 {
    <the extracted body>
}
```

then runs `rustc --crate-type=cdylib --crate-name live_coding_func --edition 2024`
directly — bypassing cargo and the linker for speed. Temp filenames use
PID + a monotonic counter so repeated compiles never collide and never fight
over a file lock. **Consequence:** the standalone body can only reference
items that are `extern "C"`-exported or self-contained — if it references
crate internals, the standalone `rustc` compile fails and the host
automatically falls back to a full build.

### 2.5 Compilation error handling

Added to make the loop robust when you save broken code:

- **Compiler output is captured**, not swallowed: `build_patch_dll()` runs
  `cargo build` via `.output()` and returns the full rustc diagnostics on
  failure; `compile_single_function()` returns rustc's stderr on failure.
- **On full-build failure the game keeps running**: reloads are
  **load-alongside** — the fresh DLL is copied to a unique temp path and loaded
  while the old one stays running, pointers are swapped, then the old is freed.
  The base DLL is *never* unloaded before a new one is ready, so a failed build
  (or failed load) simply leaves the previous DLL running untouched.  No unload
  gap, no stale freed pointers.
- **The cache only commits successful changes**, so re-saving the same broken
  body retries instead of being skipped as "unchanged".

### 2.6 Runtime symbol registry

A `HashMap<String, (u64, String)>` behind a `Mutex` maps function names to
(address, ABI signature).  New functions are registered at runtime and
resolvable by name, and `verify_signature` rejects ABI mismatches at patch
time — the Rust analogue of UE Live Coding's ability to add new code live,
hardened against signature drift.

---

## 3. Measured numbers

| Step | Time |
|---|---|
| Cold build (`cargo build`) | ~411 ms |
| Leaf patch (rustc + load + patch) | ~155–180 ms |
| Full rebuild + reload (`update` change) | ~385 ms |
| Watch poll interval | 800 ms |

---

## 4. Pros

- **Zero call overhead after the patch.** Direct calls, no hash-map lookup, no
  wrapper indirection — unlike the jump-table approach.
- **No opt-in required.** Any existing caller of the patched function is
  redirected automatically; you don't have to route every call through a
  `HotFn`.
- **New functions at runtime.** The name-keyed registry is the closest thing
  Rust has to UE's "add a new class/function live" flow.
- **Very fast leaf patches** (~160 ms) because single functions are compiled
  directly with `rustc`, skipping cargo's dependency analysis and linking.
- **Robust to compile errors.** Compiler diagnostics are shown, the game keeps
  running the last good DLL, and broken changes are retried on the next save.
- **No full DLL reload for leaf functions** — game state held in the base DLL
  survives leaf patches.
- **Self-contained**: no external crates, no build scripts, just Win32 FFI and
  the `rustc` binary already installed via rustup.
- **Trampoline support** handles the real-world case where temp-DLL code and
  the base DLL land more than 2 GB apart.

---

## 5. Cons (final status after the §7 implementation)

### 5.1 Still present — inherent to the approach (same as UE / Live++)

- **Windows only, x86/x64 only.** The whole runtime is `#[cfg(windows)]` Win32
  FFI (`LoadLibraryW`, `VirtualProtect`, `FlushInstructionCache`, ...).
  Porting needs a POSIX backend (`dlopen` / `mprotect` / `__clear_cache`), and
  AArch64's fixed 4-byte instructions break the 5-byte prologue trick entirely.
- **Signature changes are impossible.** A patched function keeps its ABI — the
  caller passes arguments per the original signature, so parameters and return
  type can't change.  The signature registry now *detects* a mismatch at patch
  time instead of crashing later, but it still can't be changed.
- **Layout changes of existing types.** Only safe via a full rebuild (which the
  watcher forces for any non-extern change).  Changing a struct's fields while
  only a leaf is patched would make the base DLL and the patched copy disagree
  on offsets — the full-rebuild safety net prevents this from happening.
- **Patched code is in-memory only.** Changes live in temp DLLs; a full rebuild
  is still needed to persist them into the shipped binary.
- **Inherently `unsafe`.** Patching machine code can never be checked at
  compile time.  The risk is now contained (see 5.2) but not eliminated.

### 5.2 Resolved by the implementation

- ✅ **State divergence** → state is **host-owned** (`HostServices` /
  `set_services`); mutable game state survives every patch and reload.
- ✅ **Crude string-based parser** → a lexical mini-lexer (`code_mask`) ignores
  strings, char literals, comments, and raw strings when finding functions and
  counting braces.
- ✅ **Not thread-safe to patch** → `suspend_other_threads()` /
  `resume_threads()` (Toolhelp32) wrap every prologue patch.
- ✅ **Memory/resource growth** → previous patch DLLs are freed on re-patch,
  trampolines are cached per target, and base-DLL temp copies are deleted after
  the swap.
- ✅ **`unsafe` everywhere** → mitigated: all `transmute`s are confined to a
  typed `HotFn<S>` wrapper, plus the signature registry.
- ✅ **Unload gap + state loss on full reload** → **load-alongside**: the fresh
  DLL is loaded while the old one keeps running, pointers are swapped, then the
  old is freed.  A failed build never unloads anything.
- ✅ **Leaf patches can't call crate internals** → a failed standalone compile
  falls back to a full build automatically.
- ✅ **Whole-file watching** → `Cargo.toml` and `build.rs` are watched alongside
  every `.rs` file.

---

## 6. Cons deep-dive: validation, mechanics, and alternatives

> Note: this is the analysis that motivated the §7 fixes — §5.2 records which
> cons are now resolved.  The "— TRUE" verdicts below describe each con as
> originally observed, before the implementation.

Each con from §5 is validated against the actual implementation, explained at
the mechanism level, and paired with the alternatives used in production
hot-reload systems.

### 6.1 Windows only — TRUE

The runtime is built on Win32 FFI: `LoadLibraryW` / `GetProcAddress` /
`FreeLibrary` (DLL loading), `VirtualProtect` / `VirtualAlloc` (code-page
protection + near allocation for trampolines), and `FlushInstructionCache`
(instruction-cache coherence). The `#[cfg(not(windows))]` stubs compile but do
nothing.

Porting needs a per-OS backend:
- **Loading/resolution**: `dlopen` / `dlsym` / `dlclose` on POSIX — or the
  `libloading` crate, which wraps all of them.
- **Code-page write permission**: `mprotect` with `PROT_READ|PROT_WRITE|PROT_EXEC`.
- **Near allocation for trampolines**: `mmap` with `MAP_32BIT` (Linux) or a
  search + `mmap(MAP_FIXED_NOREPLACE)` (macOS/Linux).
- **Instruction-cache flush**: `__clear_cache` (GCC/Clang builtin) or
  `sys_icache_invalidate` on ARM.

Big caveat: **AArch64** (Apple Silicon, ARM Windows) has fixed 4-byte
instructions, so the 5-byte `E9 rel32` trick doesn't exist. An AArch64 `b`
branch is 4 bytes but only reaches ±128 MB, so trampolines/veneers are needed
far more aggressively — a large part of why UE Live Coding stayed
x86/x64-first for so long.

### 6.2 Signature changes are impossible — TRUE (and unavoidable)

The patch redirects an existing call site's execution. The caller was compiled
against the old signature and passes arguments in fixed registers
(RCX/RDX/R8/R9 + stack on Win x64, RDI/RSI/RDX/RCX/R8/R9 on SysV) and expects
the return value in RAX. The replacement runs with exactly those registers — it
cannot reinterpret them. Adding/removing parameters or changing the return type
means garbage arguments, stack imbalance, or a crash.

This applies to the two entry points (`update`, `compute`) **even through a
full rebuild**, because the host resolves them by name and transmutes them to
the fixed `UpdateFn` / `ComputeFn` types. Only brand-new functions (registered
by name) may have arbitrary signatures.

Alternatives:
- Keep entry-point signatures stable; evolve behaviour through **new
  functions** (already supported) or a stable context pointer
  (`fn update(ctx: *mut c_void)`) whose pointed-to data changes.
- Versioned entry points (`compute_v1`, `compute_v2`) behind a thin, stable
  wrapper.
- No production system changes an existing signature live — even UE requires a
  restart for that.

### 6.3 Cannot change layout of existing types — TRUE for the fast path only

Field offsets are baked into every function compiled against the struct. The
fast path compiles `compute` standalone with a **fresh** view of the struct,
while the still-loaded base DLL has the old offsets. If both touch the same
instance → reads/writes at wrong offsets → silent memory corruption.

Nuance: changing a struct's layout **is** supported when the change flows
through the **full-rebuild path** (the whole crate recompiles together, so all
offsets agree). It only breaks when you change a struct that shared code
touches and patch only a leaf that uses it.

Alternatives:
- Never change layout live; add "fields" via a host-owned side-table keyed by
  instance, or a fixed-size struct with a scratch region.
- When a struct layout changes, force a **full rebuild** — the current watcher
  already does this for any non-extern change, which is the safety net.
- Version the type and refuse to fast-patch functions referencing versioned
  types (requires dependency tracking, i.e. a real parser).

### 6.4 Leaf patches are single-function, standalone — TRUE

`compile_single_function` feeds rustc a file containing only the extracted
body. It cannot link against the base DLL's non-exported, name-mangled
internals (`alpha`, `beta`, `test::alpha`) — there is no import library, no
headers, nothing to link against. Only self-contained bodies work; anything
calling crate internals must fall back to the full build.

Alternatives:
- **Full-crate compile + load-alongside (Live++ style)**: compile the *whole
  crate* to a fresh DLL, load it **alongside** the old one (different temp
  path), and rebind. The new code links against all internals. You trade the
  160 ms fast path for ~385 ms, but every function becomes patchable. A hybrid
  keeps the fast path for pure leaves.
- Export internals: mark helpers `#[unsafe(no_mangle)] pub extern "C"` and give
  standalone bodies a runtime import table (`GetProcAddress`) to reach them —
  but that is effectively re-implementing a linker.

### 6.5 State divergence — TRUE

A `static`, `static mut`, or `thread_local!` lives in its DLL's data segment.
The patched replacement has its **own** copy; old code in the base DLL keeps
the old copy. The two diverge silently. A whole-DLL reload has the same
problem, just more visibly (state resets to initial values).

Alternatives:
- Keep all mutable game state in the **host** (or a dedicated never-reloaded
  "state" DLL), exposed to game code via exported `extern "C"` accessors. Host
  memory survives every patch and reload — the clean Rust fix.
- Snapshot/restore: export `save_state()` / `load_state()` and re-inject
  across reloads (classic approach; requires enumerating all state).
- Accept it like UE does: global state does not survive; prefer per-instance
  state passed through the entry points.

### 6.6 Crude string-based parser — TRUE

`extract_function_bodies` scans for the literal marker and counts braces. It
mis-parses braces inside string literals (`let s = "}";`), char literals,
comments, or raw strings; a doc comment containing the marker also confuses it.
Fine for the demo's controlled files, not for general Rust.

Alternatives:
- **Add a tiny lexer**: track string / char / line-comment / block-comment
  state while scanning — ~50 lines, no dependencies, handles the vast majority
  of real cases (nested blocks, strings, comments).
- Use **`syn`** (the proc-macro parser): tokenize and parse properly, extract
  real `ItemFn` spans, handle every edge case. Production-grade, at the cost of
  a dependency and slower builds.
- tree-sitter-rust as an external parser (heavier integration).

### 6.7 Not thread-safe to patch — TRUE

The 5-byte prologue write is not atomic (only aligned ≤8-byte stores are), and
a thread mid-execution in the first 5 bytes — or one that already decoded them
— can observe a torn instruction. `VirtualProtect` and the icache flush are
process-wide side effects. The demo is safe only because it is single-threaded
and patches between ticks.

Alternatives (what UE / Live++ actually do):
- **Suspend every other thread** during the patch window
  (`SuspendThread`/`ResumeThread`), patch, flush, resume. Never suspend the
  patching thread, and avoid suspending a thread holding a lock the patcher
  needs.
- Patch only at **safe points** (between frames, all workers idle) — the
  current single-threaded design is this, generalized.
- **Use an atomic slot instead of a prologue**: route calls through a
  per-function pointer swapped with an atomic store (the jump-table approach).
  An atomic pointer swap is thread-safe by construction — the fundamental
  trade-off between Live Coding (zero overhead, unsafe to patch) and jump
  tables (one indirection, safe to swap).

### 6.8 Memory / resource growth — TRUE

Every leaf patch loads a new temp DLL (whose file stays locked on Windows, so
it can never be deleted while loaded) and, for far targets, leaks a
page-rounded 12-byte trampoline. Repeated edits accumulate loaded modules and
code pages.

Alternatives:
- **Free the previous patch DLL on re-patch**: once the prologue is re-patched,
  the old patch DLL for that function is unreferenced (single-threaded) and can
  be `FreeLibrary`-ed — growth becomes one DLL per function, not per edit.
- **Trampoline pool**: cache one trampoline per target function and reuse it.
- **Periodic compaction**: every N patches, do one full rebuild, which frees
  everything and starts fresh.

### 6.9 `unsafe` everywhere — TRUE, and inherent to the tool class

Transmuting addresses to `fn` pointers, flipping page protection, raw FFI —
none of it can be compile-time checked. A wrong signature, stale address, or
double-unload is UB with no warning. This is true of every hot patcher, JIT,
and debugger; it is not a Rust deficiency.

Alternatives / mitigations:
- **Contain the unsafe** in a small audited module (mostly done: `lib.rs`) with
  a documented invariant list.
- **Typed wrappers**: `struct HotFn<S>` that transmutes once and exposes a safe
  `call`; the signature lives in the type.
- **Signature registry**: store `(name, address, signature)` and verify the
  replacement's signature matches before patching — catches mismatches at patch
  time instead of crash time.
- Prefer **whole-DLL reload** when possible: the loader does the binding and
  the unsafe surface shrinks to two transmutes.

### 6.10 Full rebuild needed for deps; unload gap + state loss — TRUE

Any non-leaf change → unload base DLL → `cargo build` (the file must be
unlocked) → reload. During the build the game must not call the old pointers
(they are freed); in this single-threaded host the build runs while the loop
sleeps, so it is invisible — but any second thread would crash. A reload also
resets all DLL-resident state.

Alternatives:
- **Build to a temp path, then swap (Live++ style)**: compile to a fresh
  filename, load the new DLL **while the old is still loaded**, rebind
  prologues/pointers to the new one, then `FreeLibrary` the old. No unload gap,
  and the old DLL stays callable until the swap is complete.
- Keep state in the host (see 6.5) so reload never loses it.

### 6.11 Whole-file watching, not whole-crate — TRUE

The watcher hashes every `.rs` under `patch_dll/src/`, but only pure leaf-body
edits get the fast path; every other change is a full rebuild. It also does
**not** watch `Cargo.toml`, `build.rs`, or dependency changes, so adding a crate
dependency would go unnoticed until some `.rs` edit happens.

Alternatives:
- Watch `Cargo.toml` and `build.rs` too (any build-affecting file) and trigger
  a rebuild on those.
- Use the `notify` crate instead of 800 ms polling (lower latency).
- Accept the semantics: the fast path is inherently limited to leaf bodies. If
  you want *all* edits to feel fast, the load-alongside approach (6.4 / 6.10)
  is the real answer — the full build becomes the only path, with no unload
  gap.

### Bottom line

Three cons are absolute and shared with every production system: signatures
(6.2), thread-safety (6.7 — unless you suspend threads), and `unsafe` (6.9).
Most of the rest collapse with two architectural moves:

1. **Load-alongside + rebind (Live++ style)** — fixes 6.4, 6.10, 6.11 and
   removes the unload gap.
2. **Host-owned state + a real parser + thread suspension** — fixes 6.5, 6.6,
   6.7.

And the ever-present escape hatch: when a change is too structural, do a full
reload (the current fallback) — it is safe and always correct.

---

## 7. Ideas to solve the cons — IMPLEMENTED

> **Status: implemented.** The host and runtime were upgraded so nearly every
> idea below is now working code (verified end-to-end). The few remaining
> items are marked (deferred).

A point-by-point list of concrete ideas, each mapped to the con it fixes.
Effort is rough: 🟢 small, 🟡 medium, 🔴 large (a mini-project).

| Con (§5) | Idea to solve it | Effort | Status |
|---|---|---|---|
| Windows only (5.1) | `Platform` trait + POSIX backend (`dlopen`/`dlsym`/`dlclose`, `mprotect`, `__clear_cache`) or `libloading` | 🟡 | ⏳ deferred |
| Signature changes (5.1) | Keep entry-point signatures stable; signature registry rejects mismatches at patch time | 🟢 | ✅ mitigated |
| Layout changes (5.1) | Full rebuild on any non-extern change (watcher); version types; side-table storage | 🟢🟡 | ✅ safety net |
| Leaf can't call internals (5.2) | Standalone-compile failure falls back to a full build (see 7.1) | 🔴 | ✅ implemented |
| State divergence (5.2) | Host-owned state via `HostServices` / `set_services` (see 7.2) | 🟡 | ✅ implemented |
| Crude parser (5.2) | Mini-lexer `code_mask` ignoring strings/comments (see 7.3) | 🟢🟡 | ✅ implemented |
| Not thread-safe (5.2) | `SuspendThread`/`ResumeThread` around the patch window (see 7.5) | 🟡 | ✅ implemented |
| Resource growth (5.2) | Free previous patch DLL on re-patch; cached trampolines | 🟢🟡 | ✅ implemented |
| `unsafe` everywhere (5.2) | Typed `HotFn<S>` + signature registry (see 7.4) | 🟢🟡 | ✅ mitigated |
| Unload gap (5.2) | Load-alongside: copy → load → swap → free old (see 7.1) | 🔴 | ✅ implemented |
| Watching semantics (5.2) | Watch `Cargo.toml` + `build.rs` (optionally `notify`) | 🟢 | ✅ implemented |

### 7.1 The big fix: full-crate compile + load-alongside + rebind (leaf-internals, unload gap, watching)

Today a leaf is compiled *standalone*, so it cannot reference crate internals,
and a full rebuild unloads the base DLL first (the unload gap). The Live++-style
alternative fixes both at once:

```
1. cargo build the whole patch_dll crate → fresh patch_dll.dll on disk
2. copy it to a unique temp name  (target_reload/debug/patch_dll_<n>.dll)
3. LoadLibraryW(temp)  ← old DLL STAYS loaded
4. resolve update/compute (and any other exports) in the NEW DLL
5. prologue-patch the OLD functions → NEW code  (or rebind pointers)
6. FreeLibrary(old)  ← now the original file is unlockable again
```

- Because step 1 compiles the whole crate, the new code links against all
  internals — cons 4 disappears (every function becomes patchable).
- Because the old DLL stays loaded until the swap (step 6), there is never a
  window with freed pointers — con 10 disappears.
- Cost: every change is a full build (~385 ms), so the 160 ms leaf fast path is
  gone unless you keep a hybrid: pure-leaf edits take the old fast path, anything
  else takes load-alongside.

### 7.2 Host-owned state (state divergence)

Move all mutable state out of the reloadable DLL into the host, exposed through
stable exports:

```rust
// host (live_patch) — survives every patch and reload
static GAME_STATE: LazyLock<Mutex<GameState>> = LazyLock::new(|| Mutex::new(GameState::default()));

#[no_mangle]
pub extern "C" fn host_state_get() -> *mut c_void { /* ptr into GAME_STATE */ }

// game DLL — reads/writes through the host, so state is never lost
pub extern "C" fn update(tick: i32) {
    let state = unsafe { &mut *host_state_get() as &mut GameState };
    state.counter += 1;
}
```

The fallback (if state must live in the DLL): export `save_state()` /
`load_state()` and re-inject across each reload — requires enumerating all
state, so it's the more brittle option.

### 7.3 A mini-lexer instead of a string scanner (crude parser)

The extractor mis-parses braces inside strings/comments. A ~50-line lexer that
tracks lexical state while scanning fixes the common cases with no dependency:

```rust
enum LexState { Code, LineComment, BlockComment { depth: u32 }, Str, RawStr { hashes: usize } }
// advance one char at a time, flipping state on ", //, /* */, r#"..."#, r"..." ;
// only count {} while LexState::Code
```

If you need to handle every edge case (attributes, generics, macros), use `syn`
and walk `ItemFn` spans — production-grade, at the cost of a dependency.

### 7.4 Shrinking the `unsafe` surface (unsafe everywhere)

Confine the raw transmute to a typed wrapper so the host can't misuse it:

```rust
/// One hot-patched function.  The signature is encoded in the type parameter.
#[derive(Clone, Copy)]
pub struct HotFn<S> {
    address: u64,
    marker: PhantomData<S>,
}

impl<S> HotFn<S> {
    /// SAFETY: `address` must point to a function matching `S`, and the DLL it
    /// lives in must stay loaded for as long as this value is used.
    pub unsafe fn new(address: u64) -> Self { Self { address, marker: PhantomData } }
    pub fn address(&self) -> u64 { self.address }
}

impl<Args, Ret> HotFn<unsafe extern "C" fn(Args) -> Ret> {
    pub unsafe fn call(&self, args: Args) -> Ret {
        let f: unsafe extern "C" fn(Args) -> Ret = std::mem::transmute(self.address);
        f(args)
    }
}
```

Plus a **signature registry**: store `(name, address, signature_string)` and
verify the replacement's signature string matches the original *before*
patching, so an ABI mismatch is caught at patch time instead of crashing the
next call.

### 7.5 Thread-safe patching (not thread-safe to patch)

If you keep prologue patching, wrap the write in a suspend window:

```rust
// Windows: enumerate threads (Toolhelp), SuspendThread each except the caller,
// patch (VirtualProtect + write + FlushInstructionCache), then ResumeThread.
// Danger: never suspend the patching thread, and avoid suspending a thread that
// holds a lock the patcher needs (classic deadlock).
```

The structurally-safe alternative is the atomic-slot (jump-table) design: call
through a per-function pointer swapped with an atomic store. Zero overhead is
fundamentally incompatible with safe concurrent patching.

### 7.6 Suggested roadmap

- **Phase 1 (quick wins, 🟢):** mini-lexer (7.3), free the previous patch DLL on
  re-patch, watch `Cargo.toml`/`build.rs`, policy note that signatures never
  change live.
- **Phase 2 (robustness, 🟡):** host-owned state (7.2), typed `HotFn` + signature
  registry (7.4), thread suspension (7.5), trampoline pool.
- **Phase 3 (capability, 🔴):** full-crate load-alongside + rebind (7.1) — the
  single change that unlocks calling internals from patched code and removes the
  unload gap.

---

## 8. When to use it (vs. the alternatives)

- **Use this** when you want *true* live patching of leaf functions with zero
  call overhead and the ability to add new functions at runtime — the closest
  match to UE/Live++ in Rust.
- **Use the jump-table (`subsecond`) approach** when your functions change
  frequently, have internal dependencies, or you need the patch to survive a
  full rebuild gracefully and don't mind the per-call lookup.
- **Use classic full hot reload** when you want a simple, safe, whole-DLL
  reload with full `cargo` semantics and don't need sub-second turnaround.

For a production system you would want: a real parser (e.g. `syn`) to extract
bodies, a thread-suspension step before patching, a way to rebind static state,
and cleanup of temp DLLs/trampolines.
