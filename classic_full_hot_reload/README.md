# function_live_patching_simple

When you save a changed function in `patch_dll/src/lib.rs`, the loop extracts just that function's body, compiles it into a tiny standalone DLL via `rustc` in ~160ms, loads it with `LoadLibraryW`, and swaps a single pointer in the global jump table — the base DLL with the other 4,999 functions stays loaded and untouched the entire time.

This is the same concept as [subsecond](https://crates.io/crates/subsecond).

## Running

```powershell
# Build both crates
cargo build

# Run the live-reload loop
cargo run -p live_patch -- loop

# Run all static demos (no loop)
cargo run -p live_patch
```

The loop watches `patch_dll/src/lib.rs`.  Edit any function, save, and it auto-recompiles and patches
within one second.

## Architecture

```
live_patch.exe           main process, owns the global jump table
patch_dll.dll            base DLL, all functions live here, loaded ONCE
%TEMP%\live_patch_func_*.dll   patch DLLs, one per recompiled function
```

## Pipeline

1. **Startup** — `LoadLibraryW("patch_dll.dll")` loads the base DLL.  Three symbols are resolved
   via `GetProcAddress` and the initial jump table is built.

2. **Edit** — you change a function body in `patch_dll/src/lib.rs` and save.

3. **Detect** — the loop polls `file_modified_time()` every 800ms.  Timestamp changed → reload.

4. **Parse** — the source is read and every `pub extern "C" fn NAME(params) { body }` is extracted.

5. **Diff** — each function body is compared against the cached body from the previous reload.
   Unchanged functions are skipped.

6. **Compile** — for each changed function, a temporary `.rs` file is written containing ONLY that
   one function.  `rustc --crate-type=cdylib` compiles it to a tiny `.dll` in `%TEMP%`.

7. **Load** — `LoadLibraryW` maps the patch DLL into the process alongside the base DLL.

8. **Resolve** — `GetProcAddress` returns the in-memory address of the new function.

9. **Swap** — `HashMap::insert` updates one entry in the global jump table.  Nothing else changes.

10. **Dispatch** — the next call to the function looks up its address in the jump table and jumps
    to the new code in the patch DLL.

## What does NOT happen

- `patch_dll.dll` is never unloaded
- The other 4,999 functions are never recompiled or relinked
- No process memory is overwritten — only one HashMap entry changes
- `cargo build` is never called during reload — `rustc` is invoked directly
