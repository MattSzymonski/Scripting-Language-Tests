# live_coding

A **Live++ / Unreal Engine Live Coding** style hot reload for Rust.  Instead of
routing calls through a jump table (the subsecond approach), this runtime
**patches the function prologue in place**: the first 5 bytes of the original
exported function are overwritten with a `jmp rel32` that redirects to freshly
compiled code.  Existing direct callers keep their original function pointer
and are transparently redirected — **no wrapper, no lookup, no runtime
overhead after the patch**.

## Running

```powershell
# Build both crates
cargo build

# Run the live-reload loop
cargo run -p live_patch -- loop

# Run static demos (prologue patch + new-function registration)
cargo run -p live_patch
```

The loop watches **every** `.rs` file under `patch_dll/src/` (lib.rs, test.rs,
any new modules) **plus `Cargo.toml` and `build.rs`**.  Edit any function,
struct, or helper and save — leaf functions are prologue-patched in place in
~160–200 ms, and any other change triggers a full `cargo build` with a
**load-alongside** reload (fresh DLL loaded while the old keeps running, then
the old is freed — no unload gap).

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
