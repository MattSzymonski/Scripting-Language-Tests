# Function-Level Hot-Patching for Separately-Linked Rust DLLs on Windows — Research Summary

Investigation into whether subsecond-style single-function patching (not whole-crate/whole-DLL reload) can be extended to a dependency DLL like `game_rs`, rather than only the tip/`main()` crate. Compiled from: direct reading of dx's `ThinLink` source (`patch.rs`), five background research agents, and empirical tests run against the local toolchain.

**Bottom line up front:** technically constructible, nobody has shipped it soundly for Rust yet, and the two hardest sub-problems (safe live patching of intra-DLL direct calls, and recovering per-function build provenance from Rust's toolchain) are each independently hard. Treat this as a multi-week systems project, not a wrapper around a public API.

---

## 1. What ThinLink (dx/subsecond) actually does — from source

Read directly from `packages/cli/src/build/patch.rs` in the local dioxus clone (`/c/Users/mateusz.szymonski/dev/dioxus`):

- **Windows jump-table construction** (`create_windows_jump_table`, line 349): parses **both** the old and new PDBs via the `pdb` crate, builds `name → RVA` maps from `S_Public` symbols, intersects them by name, and uses the `main` symbol's RVA in each as the ASLR anchor (`aslr_reference` / `new_base_address`).
- **Undefined-symbol stub generation** (`create_undefined_symbol_stub`, line 852): for every symbol the freshly-compiled patch object references but doesn't define, emits a tiny hand-written trampoline (`movabs rax, addr; jmp rax` on x64, `mov eax, addr; jmp eax` on x86, a `movz/movk` sequence on aarch64) pointing at that symbol's **already-known live address** in the running process — this is what lets the patch "link" against a live process instead of a real linker pass.
- **`apply_patch`** (subsecond crate, `subsecond/src/lib.rs:498`): `libloading::Library::new(&table.lib)` to load the patch DLL, computes two *separate* ASLR offsets (one for the original/tip binary via `aslr_reference()` — which resolves `main` via `GetProcAddress(GetModuleHandleA(null), "main")`, i.e. **specifically the main executable module**, and one for the freshly-loaded patch library via its own exported `main` symbol), then walks the `JumpTable.map` applying the appropriate offset to each address, and commits.
- **Confirmed limitation, straight from `docs.rs/subsecond`**: "Subsecond currently only patches the 'tip' crate — the crate containing `main.rs`. Changes to crates outside this crate will be ignored, which can be confusing," attributed to "limitations in rustc itself where the build-graph is non-deterministic and changes to functions that forward generics can cause a cascade of codegen changes." Everything we measured for `game_rs` (a dependency crate) went through a *different*, coarser fallback path ("Workspace hotpatch replay") that recompiles the whole crate via a raw `rustc` replay — not real function-level patching.
- **Other documented caveats**: no struct/layout-change detection (crashes on mismatch), thread-locals reset to initial value per patch, statics' destructors never run for newly-added globals, static initializer changes aren't observed.

## 2. Empirically verified facts (not just documentation)

Run directly against the local toolchain (rustc 1.95.0 stable + nightly, MSVC x86_64-pc-windows-msvc):

- **`-Z patchable-function-entry=N,M` works today** (nightly-only). Confirmed via `--emit asm` inspection: `-Z patchable-function-entry=16,16` emits exactly 16 NOP bytes immediately before a function's entry label, growing linearly with N (tested 2, 16, 20). **16 bytes of pre-entry padding is enough to hold a full 64-bit absolute jump (`movabs+jmp` = 12 bytes, or `jmp [rip+0]+imm64` = 14 bytes)** — this would eliminate the classic ±2GB `jmp rel32` reachability problem that every native inline-hooking system (Detours, blink) has to solve with a near-trampoline allocator.
- **RVA+size diffing across two builds is insufficient — byte-level hashing is required.** Built a real differ (`pdb` crate + `object` crate) against two builds of a 4-function test crate with one function's body edited (same resulting code size). RVA+size diffing found **zero** changed functions (false negative). Hashing each function's actual on-disk bytes (resolved via PDB RVA → PE section → byte range) correctly isolated exactly the one edited function out of 244 total, with zero false positives.
- **Generic-instantiation symbol names are stable across a rebuild that doesn't touch them** — the same differ run showed all generic monomorphizations (e.g. `wrap::<u8>`, `wrap::<u32>`) had byte-identical hashes and unchanged names/RVAs when only an unrelated function changed. This is necessary for any name-based cross-build diffing to work at all; confirmed rather than assumed.

## 3. Prior art surveyed — mechanisms, and what each explicitly refuses to support

### hot-lib-reloader-rs (whole-library swap, closest existing Rust tool)
- Uses `crate-type = ["rlib", "dylib"]` (Rust ABI, not `cdylib`/`extern "C"`) — this is the direct cause of three **open, unresolved Windows loader bugs** (`LoadLibraryExW` error 126/127, `LNK1104`) traced to the dylib's dynamic dependency on `std-<hash>.dll`.
- Never loads the file cargo writes — copies to a versioned shadow filename first (`lib-hot-N.dll`) specifically to dodge Windows file-locking, then `FreeLibrary`s the old one and deletes it.
- Debounced `notify` events alone were found unreliable (differ per-OS/per-linker-strategy) — gated by a CRC32 content hash of the file as the actual trigger.
- Safety model: an `RwLock` where every call holds a read guard for the duration of the call, and reload requires a write lock — guarantees no reload happens mid-call, but the guard is scoped to the call only. **Anything returned across the boundary that references the old library (a `&'static str`, a trait object's vtable, a captured closure/fn pointer) is left completely unprotected** — no generation/epoch tracking exists.
- Explicitly unsupported (from its own README): generic functions (can't be `#[no_mangle]`'d), function signature changes, struct/enum layout changes, global state inside the reloaded library (must live in the host and be passed in), `TypeId` stability across reloads (breaks anything using it, e.g. most ECS systems for (de)serialization).
- Zero PDB handling anywhere in the codebase — confirmed via full source grep.

### blink (crosire/blink) — the closest thing to "build your own ThinLink," in ~2,300 lines, no dependencies
- **Does not invoke `link.exe` at all.** Implements its own COFF loader/relocator in-process: maps the freshly-compiled `.obj` into `VirtualAlloc`'d RWX memory (found via upward `VirtualQuery` scanning from the host image base, specifically to keep `REL32` fixups within ±2GB), resolves undefined symbols against a process-wide symbol map, and never frees the allocation.
- **Recovers the original compiler invocation from the `.debug$S` CodeView section of the `.obj` file itself** (not the PDB) — specifically the `S_ENVBLOCK` record's `cwd`/`cl`/`cmd` keys, which MSVC embeds automatically when compiling with `/Z7`/`/Zi`.
- Symbol map built from **only `S_PUB32` (public) PDB records** plus the PE import address table (recursing into imported DLLs' own PDBs) — file-local statics and non-public symbols are invisible to it, and referencing one is a hard "unresolved external" failure.
- **Patches by unconditionally overwriting the function's entry point** (`E9 rel32` on x86, 12-byte `movabs+jmp` on x64) — destructive, no trampoline, the original body becomes unreachable. A documented `SECREL` relocation is admitted in a code comment to be "probably not correct." The out-of-range check for the 12-byte x64 patch has a signed/unsigned comparison bug that silently under-covers negative and 2-4GB-positive displacements.
- Thread safety: suspends every other thread in the process for the duration of a patch with **no instruction-pointer inspection** — if a suspended thread's IP happens to be inside the bytes being overwritten, it resumes into garbage. This is a real, acknowledged soundness gap (contrast with Detours below).
- Existing globals/statics are preserved by reusing the *old* live address when a new `.data`/`.bss` symbol name matches one already known — but this exact-section-name match misses COMDAT-suffixed variants, and **new statics get zero-initialized only; no constructor/initializer ever runs**. No `.pdata`/`.xdata` (unwind info) registration anywhere — exceptions/panics propagating through patched code will fail to unwind.

### Microsoft Detours — the mechanism that actually solves blink's biggest gaps
- Preserves the original function as a callable trampoline (copies+relocates the overwritten instructions, appends a jump back) rather than destroying it.
- **Solves the thread-safety hole blink has**: its transaction API (`DetourTransactionBegin/UpdateThread/Attach/Commit`) suspends enlisted threads, and at commit time actually **inspects each suspended thread's instruction pointer and, if it falls inside the bytes being overwritten, relocates it to the equivalent offset in the trampoline** before resuming — not just "hope nothing was executing there."
- Still bounded by the same ±2GB `rel32` reach problem — implements its own region allocator (`VirtualQuery`-scanning, 64KB regions, up to ~681 trampoline slots per region) to guarantee trampolines land within range of the target.
- Explicitly refuses to patch functions too short to safely overwrite (`ERROR_INVALID_BLOCK`) and has **no way to detect** "does some other branch in this function jump into the bytes I'm about to overwrite" — that specific guarantee only exists if the code was compiled with MSVC's `/hotpatch` convention.

### MSVC's own answer: `/hotpatch` + `/FUNCTIONPADMIN` + incremental-link jump tables (ILT)
- `/hotpatch` (implied always on x64) guarantees the first instruction is ≥2 bytes and that no intra-function branch targets it — exactly the invariant that makes a 2-byte atomic entry patch safe. `/FUNCTIONPADMIN` reserves 5-6 bytes of padding *before* every function specifically to hold a `jmp rel32` for this scheme (the classic `mov edi,edi` / short-jump-into-padding pattern, documented by Raymond Chen).
- **`/INCREMENTAL` linking uses a completely different, arguably better-suited mechanism**: every call site compiles to call through a per-function jump-table thunk (`@ILT+N`), so a relink that only changes one function's body rewrites **exactly one `rel32` entry** — no caller is touched, no other function moves. This is the "indirection table" alternative to inline-hook patching, and it composes naturally with how a Rust equivalent might be built (emit your own per-function indirection slot rather than overwriting prologues).
- **Someone already tried to make this work for Rust and documented exactly where it broke** (Stefan Reinalter, Live++'s author, on `internals.rust-lang.org`): rustc unconditionally passes `/OPT:REF` to the linker (blocking the `/OPT:NOREF` Live++ needs to keep dead-code-eligible symbols alive) with no way to override it at the time; and critically, **`.obj` files get moved into the `incremental/` cache directory after compilation, so the paths recorded in the PDB's module-info no longer point at them** — breaking the exact "PDB module → `.obj` on disk → recover compiler invocation" mechanism blink and Live++ both depend on. A workaround (`-Csave-temps`) was identified but this remains a real, load-bearing gap to re-verify against the current toolchain before relying on it.
- Live++ itself (the commercial gold standard) requires `/OPT:NOICF` (disable identical-COMDAT-folding) — directly relevant to Rust, since heavy generic monomorphization produces exactly the kind of identical-body duplication ICF would fold, corrupting per-symbol patching if left enabled.
- MSVC Edit-and-Continue's own documented unsupported-changes list is a useful checklist of what "safe function patching" excludes even with full compiler/linker cooperation: struct/class layout changes, adding >64KB of new code/data, changes affecting run-time initialization, static libraries (silently not updated, no warning), and anything in an optimized build.

### `-Z patchable-function-entry` / `-Zhotpatch` in rustc
- A real, nightly-only, currently-in-progress effort (`rust-lang/rust#134004`) to give rustc the same entry-padding guarantee `/hotpatch` gives MSVC. As of the last visible activity, likely to stall for lack of maintainer capacity — verify current status before depending on it.
- Confirmed by our own empirical test (§2) that the flag works today on nightly and produces padding of the requested size.

## 4. Where this leaves the original question

Two genuinely hard, independent problems, neither solved end-to-end anywhere we found for Rust:

1. **Safe, sound patching of intra-DLL direct calls** in a live process. IAT/EAT patching (the "easy" Windows mechanisms) provably cannot intercept these — direct `call rel32` to a locally-defined function goes through no table. The only two approaches that work (destructive prologue overwrite à la blink, or trampoline-preserving inline hooking à la Detours) both need the ±2GB reach problem solved, and only Detours' variant is actually thread-safe against concurrently-executing code. `-Z patchable-function-entry`'s 16-byte padding is a genuinely promising building block here that none of the existing native tools have available (they rely on compiler-specific `/hotpatch` conventions blink/Detours don't require but benefit from).
2. **Recovering per-function build provenance from rustc's toolchain** the way blink/Live++ do from MSVC's. This is the piece that's specifically documented as broken for Rust today (`.obj` relocation into `incremental/` breaking the PDB→object-file link) and would need to be re-verified/fixed before any diffing pipeline could work — independent of whatever patching primitive is chosen.

Combined with the earlier finding that subsecond's own real advantage (skip recompiling/relinking everything *outside* the edited crate) has nothing to do with which patching primitive is used, and that `game_rs`'s architecture as a separate dependency crate is exactly the case subsecond's docs say isn't supported — the realistic paths remain what was already concluded: move the hot-reloadable code into the tip crate to use subsecond as shipped, or budget this as a genuine multi-week from-scratch systems project (COFF parsing, PDB diffing, a Detours-style safe patcher, and — per §3's rustc gap — fixing the toolchain-provenance problem no one has fixed yet) rather than "call `apply_patch` with a hand-built `JumpTable`."

## Sources
- dx/ThinLink source: `packages/cli/src/build/patch.rs`, `packages/subsecond/subsecond/src/lib.rs` (local clone)
- https://docs.rs/subsecond/latest/subsecond/
- https://github.com/rksm/hot-lib-reloader-rs (source, README, CHANGELOG, issues #3, #16, #19, #20, #21, #26, #29, #30, #34, #37, #47)
- https://github.com/crosire/blink (source, README)
- https://github.com/microsoft/Detours (source, wiki)
- https://learn.microsoft.com/en-us/cpp/build/reference/hotpatch-create-hotpatchable-image
- https://learn.microsoft.com/en-us/cpp/build/reference/functionpadmin-create-hotpatchable-image
- https://learn.microsoft.com/en-us/cpp/build/reference/incremental-link-incrementally
- https://learn.microsoft.com/en-us/visualstudio/debugger/supported-code-changes-cpp
- https://devblogs.microsoft.com/oldnewthing/20110921-00/?p=9583
- https://internals.rust-lang.org/t/trying-to-make-live-support-rust/16519
- https://github.com/rust-lang/rust/pull/134004
- https://liveplusplus.tech/docs/documentation.html
