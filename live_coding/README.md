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
any new modules).  Edit any function, struct, or helper and save — leaf
functions are prologue-patched in place in ~160 ms, and any other change
(struct, internal helper, another file) triggers a full `cargo build` +
reload.

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

1. **Startup** — `LoadLibraryW("patch_dll.dll")` loads the base DLL.  `update`
   and `compute` are resolved to **plain function pointers**.  No wrapper, no
   jump table.

2. **Edit** — you change a function body in `patch_dll/src/lib.rs` and save.

3. **Detect** — the loop content-hashes every `.rs` file under
   `patch_dll/src/` every 800 ms.

4. **Compile** — the changed function is extracted and compiled to a tiny
   standalone DLL via `rustc` in ~150 ms.

5. **Patch** — `VirtualProtect` makes the original function's page writable;
   the first 5 bytes become `E9 <rel32>` (a jump to the new code).  If the new
   code is further than ±2 GB away, a **trampoline** is allocated near the
   original (holding an absolute `jmp rax`), and the prologue jumps to that.
   The instruction cache is flushed; page protection is restored.

6. **Redirect** — the next call through the ORIGINAL function pointer jumps to
   the new code.  Everything that held a pointer to the old function is now
   running the new implementation.

7. **New functions** — a function that did not exist in the base DLL is
   compiled, loaded, and **registered by name** in a symbol registry, then
   resolvable via `resolve_symbol("name")` — the Rust equivalent of UE Live
   Coding's "add a new function live".

## What makes this different from the jump-table approach

| | Jump table (subsecond) | **Live Coding (this project)** |
|---|---|---|
| Call overhead | One hash-map lookup per call | **Zero** (direct call) |
| Opt-in | Requires `HotFn` wrapper | **None** — any caller is redirected |
| New functions | Not supported (no old address) | **Supported** via name registry |
| Mechanism | Swap pointer in `HashMap` | Patch function prologue in place |
| Memory | Base DLL + patch DLLs | Base DLL + patch DLLs + trampolines |

## What does NOT happen

- `patch_dll.dll` is never unloaded during leaf-function patches
- No process-wide wrapper or indirection is introduced at call sites
- `cargo build` is only called when a change cannot be prologue-patched:
  functions with internal dependencies (e.g. `update`), or changes outside
  function bodies (structs, internal helpers, test.rs, new modules).  Pure
  leaf-body changes (`compute`) are patched directly via `rustc` + prologue
  patch
- The prologue patch is applied atomically between ticks (single-threaded)

## Limitations (same as UE Live Coding)

- Cannot change function signatures (ABI must stay compatible)
- Cannot change class/struct layout of existing types
- Patched code lives in memory / temp DLLs only — a full rebuild is still
  needed to persist changes into the shipped binary
- Patches are applied between ticks; don't edit while another thread is
  executing inside the first 5 bytes of the target function
