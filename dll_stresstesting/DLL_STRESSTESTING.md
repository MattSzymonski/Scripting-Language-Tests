# DLL Stress Testing — AAA Scenario

A Rust stress-test scenario that reproduces, at a real PE/loader level, the
"Example AAA Unreal Engine game project breakdown" numbers from
[dll_loading_performance_101.md](dll_loading_performance_101.md) (~1929
modules, 36s total load time, long-tail distribution) — for a Rust game
engine where "lots of modules, each a DLL" is the actual architecture being
planned, not just a thought experiment.

This document explains the background, the design decisions, how to run it,
and — since this was built on one machine and is meant to be run on another —
exactly how to pick the work back up elsewhere.

## Table of contents

1. [Background: why this exists](#1-background-why-this-exists)
2. [Approaches considered](#2-approaches-considered)
3. [Our approach](#3-our-approach)
4. [Repository layout](#4-repository-layout)
5. [How to run it](#5-how-to-run-it)
6. [What to expect / how to read the output](#6-what-to-expect--how-to-read-the-output)
7. [Known limitation: EDR/AV interference](#7-known-limitation-edrav-interference)
8. [Postmortem: the mass load failures were a search-path bug, not EDR](#8-postmortem-the-mass-load-failures-were-a-search-path-bug-not-edr)
9. [Resuming on another machine](#9-resuming-on-another-machine)
10. [Tuning knobs](#10-tuning-knobs)

---

## 1. Background: why this exists

[dll_loading_performance_101.md](dll_loading_performance_101.md) is a general
reference on how DLLs load, link, and resolve symbols on Windows, and why
that's sometimes slow. Its last section, "Example AAA Unreal Engine game
project breakdown," gives concrete real-world numbers from an actual Unreal
project:

- 1929 modules (each a DLL), total load time ~36 seconds, ~19ms average
- ~1.8GB total DLL size, ~0.93MB average per DLL
- ~16 average dependencies per DLL (6 of them always OS/CRT DLLs)
- A long-tail distribution: most modules cost low single-digit milliseconds;
  a small number of outliers — whichever modules happen to be first in the
  boot sequence to touch a heavy, not-yet-loaded shared dependency — account
  for a disproportionate share of the total
- `StartupModule` (a module's own post-load init call) is a separate,
  *additive* cost on top of the OS loader's own work

The task: build a stress-test scenario in this repo (a Rust game engine
project) that reproduces this shape, focused on Rust plugins.

## 2. Approaches considered

Two fundamentally different ways to build this were on the table:

**A. Simulated timings.** Keep a flat pile of independent plugin DLLs (this
is what the repo had before — see git history), and fake the "shared
dependency" effect with manual bookkeeping (e.g. a process-wide counter or
busy-loop that's expensive only the first time some code path runs). Fast to
build and iterate, but the recursive-loading effect is code-emulated, not
something the OS loader is actually doing — it wouldn't tell you anything new
about how DLL loading really behaves.

**B. Real native inter-DLL linking.** Generate tiers of Rust `cdylib` crates
that genuinely import symbols from each other at the PE level, the same way
`UnrealEditor-X.dll` imports from `UnrealEditor-Engine.dll`. Windows'
own loader then does the actual recursive/transitive loading described in
sections 3 and 6 of the 101 write-up — the "first module to touch a shared
dependency pays the cost" effect happens for real. More setup work (build
ordering, linker plumbing), but the results are genuinely representative of
real DLL loading, not a code-level approximation.

**We chose B.** The whole point of this exercise is to learn what actually
happens when a Rust engine has ~2000 plugin DLLs; faking the interesting part
would defeat that.

## 3. Our approach

### 3.1 Three tiers, real PE imports

```
core        (few, e.g. 15)   — zero intra-project dependencies (Core/Engine analog)
subsystem   (e.g. 150)       — depend on 1-3 core modules
leaf        (the bulk, e.g. 1764) — depend on 2-7 modules drawn from core+subsystem
```

Leaf modules are never depended upon by anything else, and are the only tier
that's hot-reloadable (core/subsystem model "engine" code; leaves model
gameplay/feature plugins, continuing this repo's existing "bird plugin"
theme from the flappy-bird-adjacent projects).

Every dependency edge is a **real PE import**, not a Rust `[dependencies]`
entry (Rust's `cdylib` crate type has no notion of depending on another
`cdylib` at the Cargo level). This was verified empirically before building
the generator (see commit history / session transcript): a `cdylib` built
with the MSVC toolchain (`x86_64-pc-windows-msvc`) produces both `name.dll`
and an import library, `name.dll.lib`. A dependent crate's `build.rs` resolves
the absolute path to that `.dll.lib` and emits:

```rust
println!("cargo:rustc-link-arg={}", lib_path.display());
```

passing the exact file path rather than `cargo:rustc-link-lib=dylib=name`
(which would make the MSVC linker look for `name.lib`, not `name.dll.lib` —
a well-known Rust/Windows naming mismatch). The dependent crate declares
`extern "C" { fn dep_name_touch() -> u64; }` and actually calls it (from
inside its own `#[ctor]`, see below) so the compiler can't strip the import
as dead code. Verified with `analyze_dll.py` on a generated module:

```
=== IMPORTS (11 DLLs, 62 functions) ===
  [module] aaa_subsystem_004.dll   1 functions
  [module] aaa_core_02.dll         1 functions
  ...
  [system] KERNEL32.dll            36 functions
  [system] VCRUNTIME140.dll         7 functions
  ...
```

— a genuine PE import table entry per dependency, plus exactly the kind of
OS/CRT baseline imports (KERNEL32, VCRUNTIME140, api-ms-win-crt-*) the
write-up describes every module carrying.

### 3.2 Build order

Cargo's own dependency graph has no idea about the link above (it's a raw
linker arg, not a `[dependencies]` entry), so nothing else enforces build
order. `build_aaa_scenario.py` builds **core → subsystem → leaf**, in that
order, all sharing one `--target-dir` (`aaa_scenario/target/`) so every
`.dll` and `.dll.lib` ends up in the same folder.

> **Correction (see [section 8](#8-postmortem-the-mass-load-failures-were-a-search-path-bug-not-edr)):**
> this section originally claimed Windows' loader searches the directory of
> the module *currently being loaded* first when resolving that module's own
> dependencies, and that co-locating every module's output was therefore
> sufficient on its own. That's wrong: by default, the loader resolves a
> loaded module's transitive imports relative to the **calling process's own
> executable directory**, not the directory of the module being loaded. Since
> `stress_test.exe` and `aaa_scenario/target/release/` are different
> directories, co-location alone did not make dependency resolution work —
> `stress_test` explicitly opts into `LOAD_WITH_ALTERED_SEARCH_PATH` to get
> the behavior this section originally (incorrectly) assumed was the default.

### 3.3 Independent size and cost distributions

Every module independently draws:

- a **payload size** bucket (drives on-disk file size, via `include_bytes!`
  of a generated `payload.bin` — deliberately *not* a `const fn`-computed
  array, which would make rustc's const-eval interpreter, not the linker, the
  compile-time bottleneck for anything past a few KB)
- a **load-time (`#[ctor]`) cost** bucket — a busy-loop that runs at DLL load
  time, before any export is callable: the Rust analog of a C++ global/static
  initializer (101 doc, section 3, step 6)
- a **startup cost** bucket — a busy-loop inside an explicit, separately
  exported `module_startup()`, called *after* the module is loaded: the
  analog of Unreal's `StartupModule`

Size and cost are deliberately independent random draws. This is the
write-up's central, most counter-intuitive point (section 6): two
structurally similar modules — same size, same export count — can have
wildly different load times, because the real cost usually lives in
*whichever shared dependency someone happens to load first*, not in the
module's own file size or export table. Tying cost to size would have
quietly disproved the thing we're trying to demonstrate.

Core modules skew toward heavier `#[ctor]` cost (collectively "usually
nontrivial to bring up," like Engine/CoreUObject); leaves are almost all
cheap except rare (~0.5%) outliers. Dependency selection is popularity
weighted (`random.paretovariate`), so a few core/subsystem modules become
"hubs" that most leaves end up depending on — combined with the independent
cost draw, this is what produces the long tail: whichever leaf happens to be
first (in the shuffled boot order, see below) to touch an expensive,
not-yet-loaded hub pays that hub's full `#[ctor]` cost as part of its own
`LoadLibrary` time; everyone else who depends on the same hub later finds it
already resident and pays almost nothing.

### 3.4 Boot order

`manifest.json` includes a `boot_order`: all module names, shuffled with a
fixed seed. This is deliberately *not* topologically sorted — real engines
don't generally load modules in dependency order either (Unreal's module
manager iterates by registration order, not a pre-sorted dependency graph).
The shuffle is what creates the "unlucky first loader" scenario instead of
artificially guaranteeing dependencies always load before dependents.

### 3.5 Harness metrics

The `stress_test` binary loads every module in `boot_order` and times four
phases per module, mirroring `AAA-Scenario.csv`'s columns exactly:

1. **LoadLibrary** — `libloading::Library::new()`. Because dependencies are
   real PE imports, this genuinely triggers Windows' recursive loader for any
   not-yet-resident dependency, `#[ctor]` cost included. Nothing here is
   simulated.
2. **GetSymbol** — resolving `create_module_instance` / `module_startup`.
3. **CreateInstance** — calling `create_module_instance()` (mirrors
   instantiating a module's `IModuleInterface`).
4. **StartupModule** — calling `module_startup()`.

Two extra diagnostics, both implemented as small raw Win32 FFI calls
(`stress_test/src/win32.rs` — no `winapi`/`windows` crate dependency needed
for just two calls):

- **Transitive pull-in detection**: before/after `LoadLibrary`, check via
  `GetModuleHandleW` (by bare file name) which of a module's *transitive*
  dependencies flipped from "not resident" to "resident." This is the
  concrete, per-run, observable evidence of section 6's "unlucky first
  loader" — not an inference, an actual measurement.
- **Memory delta**: process working-set size via `K32GetProcessMemoryInfo`,
  sampled around each module's load — mirrors the `MemoryUsed_bytes` column
  already present in the reference `AAA-Scenario.csv`.

Results are written to `aaa_scenario/last_run_results.csv` using
`AAA-Scenario.csv`'s exact column layout (plus one trailing `Tier` column),
so a run of this harness can be diffed directly against the real Unreal data
already in this repo.

### 3.6 Hot reload

Only the leaf tier is hot-reloadable. A leaf's DLL is copied to a versioned
file name (`aaa_leaf_0042_v3.dll`) before loading — the original stays
rebuildable while the old version is still mapped. Editing a leaf's
`src/lib.rs` triggers a `notify`-based watcher, a `cargo build -p <name>`,
and a reload. Core/subsystem modules are not hot-reloadable (they're
"engine," not hot-reload content, and — since other modules hold real PE
imports to their exact original file name — renaming them would break every
dependent's import table).

## 4. Repository layout

```
dll_stresstesting/
  dll_loading_performance_101.md   — the general DLL-loading reference
  DLL_STRESSTESTING.md             — this file
  AAA-Scenario.csv                 — real Unreal reference data (pre-existing)
  analyze_dll.py                   — static PE analyzer (pre-existing, used to verify imports)

  generate_aaa_modules.py          — generates the tiered module graph + manifest.json
  build_aaa_scenario.py            — builds core -> subsystem -> leaf in order
  run_stress_test.py               — one-shot runner: (re)generate + build (optional,
                                      asks first) -> run stress_test.exe -> time the
                                      load -> close it -> print the result

  aaa_scenario/                    — ENTIRELY GENERATED, gitignored. Contains:
    manifest.json                  —   module graph: tier, deps, size/cost buckets, boot order
    core/  subsystem/  leaf/       —   one Cargo workspace per tier
    target/release/                —   shared build output (all .dll + .dll.lib together)
    last_run_results.csv           —   written by each stress_test run

  stress_test/                     — the harness (macroquad visualization + metrics)
    src/manifest.rs                —   loads manifest.json
    src/win32.rs                   —   GetModuleHandleW / GetProcessMemoryInfo FFI
    src/ffi.rs                     —   ModuleIdentity / ModuleMetrics types
    src/module_loader.rs           —   the loading + timing + CSV logic
    src/bird_menu.rs               —   grid visualization + metrics overlay
    src/main.rs                    —   entry point, arg parsing

  engine/ flappy/ game_rs/ game_cs*/  — unrelated existing projects, untouched
```

`aaa_scenario/` is fully generated and gitignored — nothing under it is
tracked. Only `generate_aaa_modules.py`, `build_aaa_scenario.py`, and the
`stress_test` crate are checked in.

## 5. How to run it

Requires: Rust (`x86_64-pc-windows-msvc` target — this is Windows/MSVC-only,
consistent with the 101 doc's own Windows/PE focus) and Python 3 (stdlib
only; no pip packages needed for these two scripts — `analyze_dll.py`
separately needs `pip install pefile` if you want to inspect a DLL by hand).

```powershell
cd dll_stresstesting

# 1. Generate the module graph (defaults to --scale full, ~1929 modules).
#    Use --scale medium (~330) or --scale small (~51) for faster iteration.
python generate_aaa_modules.py --scale medium

# 2. Build core -> subsystem -> leaf, in that order, into a shared target dir.
python build_aaa_scenario.py --tier core        # or omit --tier to build all three
python build_aaa_scenario.py --tier subsystem
python build_aaa_scenario.py --tier leaf
#    (or just: python build_aaa_scenario.py    — builds whatever tiers exist,
#     regenerating first only if aaa_scenario/ is missing or --regen is passed)

# 3. Run the stress test.
cargo run -p stress_test --release                  # sequential (default)
cargo run -p stress_test --release -- --parallel     # rayon parallel loading
```

Or, to do all of the above (optionally) and get a single measured wall-clock
number for "process launch → modules loaded" printed at the end, use
`run_stress_test.py` instead of driving the three steps by hand:

```powershell
python run_stress_test.py                # asks before regenerating/building
python run_stress_test.py --yes          # skip regen+build, just run + time the existing build
python run_stress_test.py --regen --scale medium --build --mode parallel
```

Controls in the running window: `↑`/`↓` or `W`/`S` to scroll the module
grid, `H` to toggle the metrics overlay, `Esc` to quit. Edit any leaf
module's `src/lib.rs` (e.g. `aaa_scenario/leaf/aaa_leaf_0042/src/lib.rs`)
while it's running to trigger a hot-reload.

Full-scale (~1929 modules) generation + build took roughly 5-10 minutes on
the machine this was built on, dominated by the leaf tier (the bulk of the
module count, but each crate is small and independent so cargo parallelizes
them well). Medium scale (~330) took under a minute.

## 6. What to expect / how to read the output

Console output on load:

```
=== DLL Stress Test: AAA Scenario ===
Mode:      Sequential
Modules:   15 core + 150 subsystem + 1764 leaf = 1929 total
Loading mode: SEQUENTIAL (boot order, one DLL at a time)
PluginManager: loaded 1929/1929 modules in <N> ms
Results written to: .../aaa_scenario/last_run_results.csv
```

The overlay (press `H`) shows: mode, loaded/total, tier breakdown, total
wall-clock split into the four phases, FPS, reload count, and the slowest
modules by `Total_ms` — annotated with which dependency (if any) *that
specific module* was the first to pull in transitively. A module in that
list with a `-> pulled in: ...` annotation is the concrete "unlucky first
loader" — direct, measured evidence of section 6, not a guess.

`aaa_scenario/last_run_results.csv` uses the exact same columns as
`AAA-Scenario.csv` (plus a trailing `Tier` column) — open both side by side
to compare this run's shape (average, total, long-tail spread) against the
real Unreal project's numbers.

What a *healthy* run looks like: nearly all modules loaded (loaded count ≈
total count), most `Total_ms` values in the low single digits, and a small
number of clear outliers in the overlay's "slowest modules" list, several of
which show a `pulled in` annotation. If instead a large fraction of modules
show `Status = FAIL`, see the next section.

## 7. Known limitation: EDR/AV interference

While building this, on a corporate-managed machine running SentinelOne, a
large fraction of `LoadLibrary` calls failed intermittently with "the
specified module could not be found" (`ERROR_MOD_NOT_FOUND`, Win32 code 126)
— even though the target file demonstrably existed and could be loaded fine
in isolation. Repeated runs against the same file set got *worse*, not
better (9 → 29 → 109 failures out of 330 across consecutive runs), which
points to escalating behavioral scrutiny rather than a one-off scan delay: a
process rapidly `LoadLibrary`-ing hundreds of freshly-written, unsigned DLLs
in quick succession is a plausible signature for EDR heuristics watching for
process-injection/dropper behavior, and this is exactly the "antivirus/EDR
real-time scanning" bottleneck called out in the 101 doc's section 7, item 3.

This is not a bug in the harness, and not something to work around by
disabling or excluding a managed endpoint's security tooling. The mitigation
built into `module_loader.rs` is a bounded retry with short backoff around
`LoadLibrary` (`LOAD_RETRY_ATTEMPTS`, currently 8 attempts, 50ms × attempt
number backoff) — the wait is honestly counted as part of `LoadLibrary_ms`,
since that's genuinely how long a caller waited to get the module loaded.

If you see a high failure count when running this:
- It's almost certainly this, not a bug — check whether an EDR/AV agent with
  real-time protection is active (`Get-CimInstance -Namespace
  root/SecurityCenter2 -ClassName AntivirusProduct`).
- On an unmonitored dev box, or after getting a folder exclusion approved by
  your security team for `aaa_scenario/`, you should see close to 100%
  success.
- If it's still flaky, try increasing `LOAD_RETRY_ATTEMPTS` in
  `module_loader.rs`, or generating/building at a smaller `--scale` to reduce
  how many files get touched per run.

## 8. Postmortem: the mass load failures were a search-path bug, not EDR

After the harness had been sitting untouched for a while, resuming work at
`--scale full` (1929 modules) reproduced the mass-`FAIL` symptom described in
section 7 — but investigating it properly turned up a different, fully
deterministic root cause, not EDR.

**Symptom.** The same ~27-module subset (out of 51 at `--scale small`, and a
proportionally similar subset at full scale) failed every single run with
`libloading failed: LoadLibraryExW failed`, in the same order, **even
immediately after a full `--clean` regenerate and rebuild** — not the
"different count each run" pattern you'd expect from a genuinely transient
antivirus scan lock.

**Investigation.**
1. Directly `LoadLibraryEx`'d a failing module's `.dll` outside the harness
   (via a small PowerShell/.NET P/Invoke snippet) and captured the real
   `GetLastError()`: **126, `ERROR_MOD_NOT_FOUND`** — consistently, not
   intermittently.
2. Checked every transitive dependency `.dll`/`.dll.lib` referenced by that
   module actually existed on disk in `aaa_scenario/target/release/` —
   they did.
3. Inspected the failing module's PE import table with `analyze_dll.py`
   (`pefile`) — it correctly listed its declared dependencies by bare name
   (`aaa_core_04.dll`, `aaa_subsystem_073.dll`, plus the usual OS/CRT set).
   Nothing wrong with the generated file itself.
4. Loaded each of those two direct dependencies **standalone**, from the same
   ad-hoc script — both succeeded. So every individual file was fine in
   isolation; only loading the dependent *from the harness* failed.
5. That combination — a module's declared dependencies load fine on their
   own, but the module itself fails from the harness with 126 — is the
   signature of a **DLL search-path problem**, not a missing or corrupt file:
   `stress_test.exe` lives in `dll_stresstesting/target/release/`, while the
   generated module DLLs live in the *different* directory
   `aaa_scenario/target/release/`. Windows' default `LoadLibrary` search order
   resolves a loaded module's own transitive imports relative to the
   **calling process's executable directory**, not the directory the module
   was itself loaded from (see the new section 11, item 12 of
   [dll_loading_performance_101.md](dll_loading_performance_101.md)). Section
   3.2's original assumption that "same-directory co-location" alone was
   enough turned out to be wrong for exactly this reason.
6. This also explains the *shape* of the original failures in a way pure EDR
   scrutiny doesn't: `boot_order` is a random shuffle (section 3.4). A module
   only succeeds if every one of its dependencies happened to *already* be
   resident (loaded earlier as some other module's own direct `LoadLibrary`
   target) by the time its turn came up — otherwise resolving its import from
   scratch requires the file-system search that was never going to find
   `aaa_scenario/target/release/` in the first place. That is a coin flip per
   module dictated by shuffle order, not a probabilistic AV scan outcome —
   consistent with the same ~50%-ish subset failing identically on repeat
   runs against the same seed.

**Fix.** `load_library_with_retry` (and the hot-reload path in
`reload_module`) in `stress_test/src/module_loader.rs` now load every module
with the `LOAD_WITH_ALTERED_SEARCH_PATH` flag, via
`libloading::os::windows::Library::load_with_flags`, instead of plain
`libloading::Library::new`. That flag makes the loader resolve a module's
imports relative to *its own* directory rather than the process's, which is
exactly the co-location behavior section 3.2 originally (incorrectly)
assumed was the default.

**Verification.** Full `--scale full` run (1929 modules) after the fix,
via `run_stress_test.py --yes`:

```
PluginManager: loaded 1929/1929 modules in 22894.1 ms
Results written to: .../aaa_scenario/last_run_results.csv
```

**1929/1929 loaded, zero failures** — versus mass failures before the fix on
the identical generated scenario. 22.9s for 1929 modules is the same order of
magnitude as the real Unreal reference (36s), which is itself a reasonable
sanity check that the scenario is now measuring the phenomenon it was built
to measure, rather than mostly measuring load failures.

Breaking that 22.9s down by phase, from `last_run_results.csv`:

| Phase | Total | Share |
|---|---|---|
| LoadLibrary | 8,684 ms | 99.7% |
| GetSymbol | 5.2 ms | 0.1% |
| CreateInstance | 0.0 ms | 0.0% |
| StartupModule | 20.7 ms | 0.2% |
| **Sum of phases** | **8,710 ms** | |

Note the phase sum (8.7s) doesn't fully account for the reported total
(22.9s, from Rust's own `start.elapsed()` around the whole loop) — the ~14s
gap is bookkeeping that happens *outside* the four timed phases: the
residency check (`GetModuleHandleW`) over each module's full transitive
dependency closure before its load, and the two working-set memory samples
(`K32GetProcessMemoryInfo`) around it. Neither is currently its own timed
field in `ModuleMetrics`; a `bookkeeping_ms` column would close that gap if
it turns out to matter for future analysis.

**What this means for section 7.** EDR/AV interference is a real,
independently-documented phenomenon (`ERROR_MOD_NOT_FOUND` can also occur from
a genuine transient file lock during a real-time scan) and the bounded
retry/backoff there is still worth keeping as a hardening measure — but it
was **not** the explanation for the mass failures actually observed and
root-caused in this session. If you see a *large, deterministically
repeatable* subset of failures (same modules, same order, surviving a clean
rebuild), suspect this search-path issue first — it's now fixed, so a stock
checkout shouldn't hit it, but it's worth knowing if the symptom resurfaces
after modifying how/where `stress_test.exe` or the generated DLLs get placed.

## 9. Resuming on another machine

Step by step, from a fresh clone/pull of this repo:

1. **Confirm the toolchain**: `rustc -vV` should show `host:
   x86_64-pc-windows-msvc`. `cargo --version` should work. `python
   --version` should be Python 3.x.
2. **Pull the latest commit** containing this work (see below for what
   changed — new files `generate_aaa_modules.py`, `build_aaa_scenario.py`,
   `stress_test/src/{manifest,win32,module_loader}.rs`; removed
   `generate_plugins.py`, `plugins_heavy/`, `stress_test/src/plugin_loader.rs`;
   modified `stress_test/Cargo.toml`, `stress_test/src/{ffi,bird_menu,main}.rs`,
   root `.gitignore` and `Cargo.lock`).
3. **Nothing under `dll_stresstesting/aaa_scenario/` is committed** — it's
   fully generated and gitignored. You start from nothing there on a fresh
   checkout; that's expected, not a missing-file problem.
4. **Generate + build**, per [section 5](#5-how-to-run-it) above. Start with
   `--scale small` (~51 modules) first to confirm the toolchain/linking
   mechanism works end-to-end quickly, before committing to a `medium` or
   `full` run.
5. **Run** `cargo run -p stress_test --release` (or `python
   run_stress_test.py --yes`) and check the console: you want `loaded ==
   total` (or very close). A stock checkout already has the
   `LOAD_WITH_ALTERED_SEARCH_PATH` fix from
   [section 8](#8-postmortem-the-mass-load-failures-were-a-search-path-bug-not-edr),
   so mass failures shouldn't recur; if they do anyway, check that section
   first, then [section 7](#7-known-limitation-edrav-interference) for the
   EDR caveat.
6. **Compare** `aaa_scenario/last_run_results.csv` against
   `AAA-Scenario.csv` to see how this run's numbers stack up against the
   real Unreal reference data.
7. If you want to scale up to the full ~1929-module scenario after
   confirming small/medium works: `python generate_aaa_modules.py --scale
   full --clean && python build_aaa_scenario.py`. Budget several minutes for
   the build, dominated by the leaf tier.

## 10. Tuning knobs

All in `generate_aaa_modules.py` unless noted:

- `SCALE_PRESETS` — module counts per `--scale` preset.
- `PAYLOAD_BYTES_TIERS` — file-size distribution (calibrated to ~800KB
  average at medium scale against the write-up's ~0.93MB reference; re-check
  with `os.path.getsize` across `aaa_scenario/target/release/*.dll` after a
  build if you change these).
- `CTOR_COST_TIERS`, `STARTUP_COST_TIERS` — load-time vs. startup cost
  distributions, per tier.
- `DEP_COUNT_RANGE` — how many dependencies each tier draws.
- `BASELINE_OS_IMPORTS` (in `stress_test/src/module_loader.rs`) — the
  assumed OS/CRT import count added to a module's declared dependency count
  for the CSV's `NumImportedDlls` column (measured empirically via
  `analyze_dll.py`, not re-derived at runtime — this harness doesn't embed a
  PE parser).
- `LOAD_RETRY_ATTEMPTS` (in `stress_test/src/module_loader.rs`) — see
  [section 7](#7-known-limitation-edrav-interference).
