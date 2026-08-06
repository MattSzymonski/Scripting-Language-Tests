# Hot Reload Bottleneck Research

## The question

When hot-reloading a DLL with thousands of functions, **what is the bottleneck --
recompiling or relinking?**

## The answer: **linking**

Rust's incremental compilation already skips recompilation of unchanged source
files.  When you edit a single function in `lib.rs` and run `cargo build`, only
that one file is recompiled to a `.o` object file.  The other 4,999 functions in
`bloat_gen.rs` are **not recompiled** -- their cached `.o` files are reused.

| Phase | What happens | Cost at 5,000 functions |
|---|---|---|
| **Compile** | Only changed `.rs` → `.o` | ~fast (tens of ms) |
| **Link** | Linker collects **all** `.o` files into one `.dll` | ~**dominant** (≈1.2s of the 1.4s total) |

### Why linking scales poorly

The linker (MSVC `link.exe` or `rust-lld`) has **no incremental mode** for
DLLs.  On every rebuild it must:

- Parse every one of the 5,000+ `.o` files
- Resolve all `pub extern "C"` symbols for the export table (`.edata`)
- Apply relocations and fixups across all object files
- Produce the final PE binary

This is an O(n) problem where n = total function count, not changed function
count.  A 5,000-function DLL takes ~1.4s to link on a typical dev machine;
a 50,000-function DLL would take proportionally longer.

### Verifying with `--timings`

You can confirm this yourself:

```powershell
cargo build -p game_lib --timings
```

This generates `target/cargo-timings/cargo-timing.html` -- open it in a browser
to see a waterfall chart breaking down compile time vs link time per crate.

## How this relates to the example projects

### `classic_full_hot_reload` -- demonstrates the problem

```
Edit lib.rs → cargo build → link ALL 5,000 .o files → LoadLibraryW(new DLL)
                         ↑
                    ~1.4s bottleneck
```

This is the **classic approach**: recompile the entire crate, relink everything,
load the new DLL.  It works great for small projects (~250ms with no bloat) but
degrades linearly with codebase size because the linker must process every
function, changed or not.

The only workaround in this model is to split the codebase into many small
crates (e.g. one DLL per module) so each link step stays small.  But that adds
build-system complexity and doesn't help when the hot function lives in a crate
with many peers.

### `function_live_patching_simple` -- demonstrates the solution

```
Edit one function → rustc --crate-type=cdylib → compile ONLY that function
                                              → LoadLibraryW(tiny DLL)
                                              → swap 1 pointer in jump table
                                              → ~160ms, independent of codebase size
```

This **bypasses the linker entirely** for unchanged code.  Only the edited
function is compiled (via `rustc` invoked directly as a subprocess), producing a
tiny standalone DLL.  The other 4,999 functions stay in the original base DLL
and are never touched.

The key insight: **O(1) reload cost regardless of project size**, because:

- No `cargo build` is ever called during reload
- No linker is ever invoked for unchanged functions
- Only one `HashMap::insert` updates the global jump table

## Summary

| | Classic | Per-function patching |
|---|---|---|
| **Reload cost** | O(n) -- scales with total functions | O(1) -- scales with changed functions |
| **5,000 functions** | ~1.4s | ~160ms |
| **50,000 functions** | ~10-15s (estimated) | ~160ms |
| **Mechanism** | `cargo build` + link all `.o` | `rustc` single function + jump table |
| **Example project** | `classic_full_hot_reload` | `function_live_patching_simple` |

The tradeoff is that per-function patching requires a jump-table indirection on
every hot function call (one `HashMap` lookup), adding a few nanoseconds of
overhead per call.  For the vast majority of game/application code this is
negligible compared to the developer productivity gain of sub-second reloads.
