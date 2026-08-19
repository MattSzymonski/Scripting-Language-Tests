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
2. **Load once** — `LoadLibraryW("patch_dll.dll")`, resolve `update` / `compute`
   to **plain function pointers**. These are the *only* call sites in the loop:
   `unsafe { update_fn(tick) }` and `unsafe { compute_fn(x, y) }`. No wrapper.
3. **Watch** — content-hash every `.rs` file under `patch_dll/src/` every
   800 ms; any change is detected, not just `lib.rs` extern edits.  A full
   rebuild is triggered by any non-extern change (structs, internal helpers,
   `test.rs`, new modules) or by `update` changing; pure leaf-body edits take
   the fast prologue-patch path.
4. **Parse** — `extract_function_bodies()` is a crude string scanner that pulls
   each `pub extern "C" fn NAME(params) -> Ret { body }` out of the source,
   tracking brace depth so nested blocks are captured correctly.
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
items that are `extern "C"`-exported or self-contained — which is exactly why
`update` (which calls internal helpers) must fall back to a full build.

### 2.5 Compilation error handling

Added to make the loop robust when you save broken code:

- **Compiler output is captured**, not swallowed: `build_patch_dll()` runs
  `cargo build` via `.output()` and returns the full rustc diagnostics on
  failure; `compile_single_function()` returns rustc's stderr on failure.
- **On full-build failure the game keeps running**: the host first unloads the
  base DLL (so the linker can overwrite the file), runs the build, and on
  failure **reloads the previous DLL** from disk (a failed build never
  overwrites it), re-resolves `update`/`compute`, and continues ticking. No
  stale freed pointers, no crash.
- **The cache only commits successful changes**, so re-saving the same broken
  body retries instead of being skipped as "unchanged".

### 2.6 Runtime symbol registry

A `HashMap<String, u64>` behind a `Mutex` maps function names to addresses.
New functions are registered at runtime and resolvable by name —
the Rust analogue of UE Live Coding's ability to add new code live.

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

## 5. Cons

- **Windows only.** The whole runtime is `#[cfg(windows)]` Win32 FFI
  (`LoadLibraryW`, `VirtualProtect`, `FlushInstructionCache`, ...).
- **Signature changes are impossible.** The patched replacement must keep the
  exact same ABI — you can't add/remove parameters or change return types.
- **Cannot change the layout of existing types.** A struct's fields are baked
  into the base DLL; the standalone replacement compiles its own view. Adding
  fields to a struct that code in the base DLL still touches will miscompile.
- **Leaf patches are single-function, standalone.** The extracted body can only
  use what is exported or self-contained; anything calling crate-internal
  helpers (`alpha`, `beta`, `test::alpha`) forces a full rebuild.
- **State divergence.** If a patched function had statics/globals in the base
  DLL, the freshly compiled copy has *its own* copy of that state. This is the
  classic hot-reload pitfall (same in UE Live Coding).
- **Crude string-based parser.** `extract_function_bodies()` scans text, not
  syntax. It handles nested braces but will be confused by braces inside string
  literals/comments or by macro-heavy bodies. It's a demo-grade parser, not a
  real front end.
- **Not thread-safe to patch.** The 5-byte prologue is overwritten between
  ticks; editing while another thread executes inside those first bytes is UB
  (the host is single-threaded, but a real game is not).
- **Memory/resource growth.** Patch DLLs in temp dirs and leaked trampolines
  accumulate over a long session.
- **`unsafe` everywhere.** Function pointers are `transmute`d, memory
  protection is flipped, and there is no compile-time guarantee the new
  function matches the old signature.
- **Full rebuild still needed** for any function with dependencies — and full
  rebuilds unload/reload the whole DLL, losing in-memory game state that isn't
  persisted.
- **Whole-file watching, not whole-crate semantics.** Any `.rs` under
  `patch_dll/src/` is detected by content-hash, but non-extern changes always
  fall back to a full rebuild — only pure leaf-function body edits get the
  fast prologue patch.

---

## 6. When to use it (vs. the alternatives)

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
