# Reducing Build Artifact Size

How to make the compiled `game_rs.exe` smaller, ranked by bang-for-buck. Every
number was measured on this project (macroquad Flappy Bird); reproduce with the
commands shown.

Profile lives in the workspace root [`Cargo.toml`](../Cargo.toml); the binary is
[`game_rs/src/main.rs`](../game_rs/src/main.rs).

## Baseline

Measure before optimizing. Starting profile was `opt-level = 3`, `lto = true`:

```bash
cargo build --release --bin game_rs
```

**Baseline: ~708 KB.** Most of that is *not* your code — it's `std`, the panic
and formatting machinery, symbol/debug tables, and dependencies (macroquad pulls
in `image`, `png`, `fontdue`, `glam`, …). "Delete what you don't need" beats
micro-optimizing your own functions.

## 1. Stable size profile — do this first

Five lines in `[profile.release]`, no code changes, no extra tooling:

```toml
[profile.release]
opt-level = "z"      # optimize for size, not speed (was 3)
lto = true           # whole-program dead-code elimination
codegen-units = 1    # one unit → best cross-function optimization
panic = "abort"      # drop stack-unwinding tables
strip = true         # drop symbols + debug info
```

| Lever | Why it shrinks | Cost |
| --- | --- | --- |
| `strip = true` | Removes symbol/debug tables — usually the biggest single win | No backtraces/debugger |
| `opt-level = "z"` | Stops size-for-speed tricks (inlining, unrolling); `"s"` is gentler | Slower CPU code |
| `lto = true` | Deletes uncalled code across crates | Slower link |
| `codegen-units = 1` | Whole crate optimized together | Slower compile (no parallelism) |
| `panic = "abort"` | No unwinding machinery | No `catch_unwind` |

**Measured: 708 KB → 458 KB (−35%).** Pure upside for a shipping build; for a
game, `opt-level = "z"` is imperceptible since it's GPU/frame-cap bound.

## 2. Rebuild std for size (nightly)

`std` ships precompiled with *its* settings, so it ignores your profile.
`build-std` recompiles it with yours — the biggest lever after step 1:

```bash
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly

cargo +nightly build --release \
  -Z build-std=std,panic_abort \
  -Z build-std-features=panic_immediate_abort \
  --target x86_64-pc-windows-msvc --bin game_rs
```

`panic_immediate_abort` also drops panic *message formatting* (a big chunk).
Typical extra saving **20–40%**. Costs: nightly only, explicit `--target`
required, no panic messages. (Only stable is installed here — needs the setup
above.)

## 3. Find & trim the fat

Measure which crate costs bytes before cutting:

```bash
cargo install cargo-bloat
cargo bloat --release --bin game_rs --crates
```

Then disable unused crate features (`default-features = false`) or drop heavy
deps. For this project the deps are load-bearing (macroquad *is* the engine), so
`cargo bloat` mostly confirms there's little to remove — which is the point:
don't waste effort on crates you can't cut.

## 4. Compress the finished binary — UPX

UPX packs the built `.exe` and prepends a stub that decompresses it into memory
at launch. It shrinks the **file on disk only** — not runtime RAM.

```bash
upx --best --lzma target/release/game_rs.exe   # --lzma = smaller; -d reverses
```

**Measured: 458 KB → 180 KB (−61%).** Combined with step 1, that's **~75% off
the baseline.**

Caveats — this is a distribution step, not a build step:
- ⚠️ **Antivirus / SmartScreen often flag packed exes** — packing is a malware
  technique. For a distributed game this alone is often a reason to skip it.
- ❌ **No RAM savings**; ⚠️ small startup delay; ⚠️ **invalidates code signing**
  (pack *then* sign); ⚠️ don't pack shared DLLs.

Install on a locked-down Windows box (winget blocked by Group Policy): UPX is a
portable single `.exe` — download the release zip from GitHub, extract `upx.exe`
to a folder, add it to your user PATH. (Installed here at
`%USERPROFILE%\tools\upx.exe`.)

## Trade-offs

| Technique | Size win | Runtime | Compile | Other cost |
| --- | --- | --- | --- | --- |
| Stable profile (§1) | −35% | Faster link/exec, `"z"` slower CPU | Slower | No debug symbols |
| `build-std` (§2) | −20–40% more | None | Slower | Nightly; no panic messages |
| Trim deps (§3) | Varies | None | Faster | May lose features |
| UPX (§4) | −60% on disk | Startup delay | None | AV flags; breaks signing; no RAM win |

## Recommended setup

Keep fast iteration *and* a small shippable binary via a custom profile:

```toml
[profile.release]           # day-to-day: quick build + run
opt-level = 3
lto = "thin"

[profile.min-size]          # shipping build
inherits = "release"
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

```bash
cargo run --release --bin game_rs                 # develop
cargo build --profile min-size --bin game_rs      # ship (~458 KB)
upx --best --lzma target/min-size/game_rs.exe     # optional (~180 KB)
```

Order of impact: **stable profile → build-std → trim deps → UPX.**

## Other techniques (situational)

Not needed here, but common in the wider ecosystem:
- **Linker flags:** MSVC `/OPT:REF,ICF` (drop + fold identical functions); `lld`.
- **Nightly `-Z`:** `location-detail=none`, `fmt-debug=none` — drop panic
  location strings and `Debug` formatting (pairs with build-std).
- **Fewer generics:** monomorphization duplicates code per type; `&dyn Trait` or
  a non-generic inner fn cuts copies.
- **`optimize_for_size` std feature:** `-Z build-std-features=optimize_for_size`
  builds `std` itself for size; stacks with `panic_immediate_abort`.
- **Skip the runtime shim:** `#![no_main]` + `#[no_mangle] extern "C" fn main`
  drops Rust's `lang_start` startup scaffolding without full `no_std`.
- **`#![no_std]`:** the radical route to tiny binaries — not viable with macroquad.
- **Smaller formatting/alloc:** `ufmt` in place of `core::fmt`; tiny WASM
  allocators (`talc`, `lol_alloc`, `dlmalloc`).
- **External assets:** load fonts/textures at runtime instead of `include_bytes!`.
- **WASM target:** a separate pipeline — `wasm-opt -Oz --converge`, `wasm-snip`,
  `twiggy`.

## Dark arts (demoscene, for context)

None of this applies to a normal Rust app — it's the extreme end, included to show
where the ideas run out. These come from **sizecoding** competitions with intros
capped at 64k, 4k, 1k, 256 bytes, even 32. At those limits people don't shave a
binary, they never build a normal one: a 4k intro renders a full animated 3D
scene with music.

### Compressing linkers
Generic packers like UPX are huge by scene standards, so the scene wrote its own:
- **Crinkler** (Blueberry & Mentor) — the 4k/1k legend; a linker and compressor
  fused into one, using a context-modeling arithmetic coder and, the key trick,
  **reordering code/data to minimize the *compressed* size**.
- **kkrunchy** (ryg, Farbrausch) — the 64k equivalent, context-mixing.
- **Squishy, oneKpaq, apLib** — other ratio-vs-stub-size tradeoffs.

Transferable lesson: packers do better on **homogeneous, predictable** code —
the deep version of what LTO + ICF do for you.

### Self-decompressing loader
The standard 4k shape, and what a Crinkler/kkrunchy stub is: **tiny loader →
compressed payload → decompress into RWX memory → jump to it.** Almost every
modern intro is a variation on this; the whole visible file is the packed blob
plus a hand-written depacker.

### Ditch the C runtime & scaffolding
- **Fewer imports = more room:** every imported function and framework call costs
  header/scaffold bytes; the less you import, the more space is left for the
  effect and its data.
- **No CRT startup:** custom `/ENTRY`, skip `mainCRTStartup` boilerplate.
- **Shrink imports:** prefer **manual, ordinal, or hash-based** resolution — walk
  the PEB to find `kernel32` and look functions up by hash — instead of a full
  named import directory.
- **Link the system `msvcrt.dll`** so `sin`/`malloc` cost zero file bytes.

### PE header abuse
- **Overlap the header** with code/data — reuse loader-ignored header bytes; code
  can live in the DOS stub or the PE headers themselves.
- **Metadata as code/constants:** import descriptors, section headers, and
  relocation data can double as executable bytes or constant tables.
- **Merge sections** (`/MERGE`) — fewer headers, less alignment padding.
- **Minimize alignment** to the least the loader tolerates. Below 256 bytes the
  header *is* most of the file. Modern Windows loaders are surprisingly tolerant
  of unusual alignments and partially-invalid headers.

### Procedural everything (the real secret)
The biggest win is having **no assets to compress**:
- Textures from runtime noise (Perlin/FBM), not stored images.
- Geometry from math — raymarched SDFs, a world in a few equations.
- Music from a softsynth (**4klang, Clinkster, Oidos**) — score + instruments as
  a tiny parameter blob.

### GPU/OS as the free standard library
- A 4k intro is often **one fragment shader** doing raymarched rendering; the
  `.exe` just makes a GL/D3D context and draws a fullscreen triangle. The
  renderer is the driver — zero file bytes.

### Assembly & instruction golf
- **Opcode-size optimization:** same algorithm, smaller bytes — shorter ModRM
  encodings, accumulator-specific forms, `lea` for math, sign-extended
  immediates, `xor reg,reg` to zero, and register choices that encode shorter.
- **Flag programming:** Carry/Zero/Sign/Overflow are free one-bit variables;
  compute X/Y without `DIV`; read the timer tick "for free." Compilers rarely
  exploit flags this aggressively — humans do.
- **Constant synthesis:** derive a constant from values already in registers
  instead of storing it (e.g. build `0x3f800000` mid-computation) — near-free if
  the intermediate already exists.
- **Exploit default machine state:** known register/stack state at entry gives
  free constants (e.g. stack top may be `0x0000` in some setups) — but this is
  environment-specific and *not* guaranteed across emulators/OSes.
- **Stack as storage:** temporary arrays, implicit variables, return-address
  tricks, no explicit stack frames.
- Self-modifying code.

### Overlapping instruction streams
Because x86 is **variable-length**, the same bytes decode into different valid
programs depending on where execution starts — e.g. `A8 01 74 03` is one thing
entered at byte 0, another entered mid-sequence. Two code paths for the price of
one byte range; modern CPUs don't care.

### Relocation elimination
Reloc tables still cost bytes in a PE. Make the image position-independent so
there's **no relocation table**: RIP-relative addressing, `call`/`pop` to fetch
the instruction pointer, and relative jumps instead of absolute addresses.

### Compression-aware programming
The biggest lever in modern 4k intros: minimize the **compressed** size, not the
raw bytes. Adding 30 bytes can shrink the packed executable by 100+.
- **Feed the packer:** reorder functions, choose instructions that create
  repeated byte patterns, arrange tables to improve LZ matches, and rename
  symbols (before stripping) for regularity.
- **Duplicate over abstract:** repeated code compresses away, whereas an
  abstraction can cost more than it saves.
- **Data is code, code is data:** let one structure serve multiple roles, pack
  constants into existing values, reuse the same bytes for both. Ideally every
  byte is instruction, constant, table, alignment, *and* file structure at once.
- Reuse bit patterns / float representations; group similar bytes.

### Extreme tiers & platforms
- **MS-DOS `.COM`** — no header, loads at 0x100; the 256b/128b/32b canvas.
- **Bootsector intros (512 B)** — the whole program is a boot sector, no OS.
- **TIC-80 / PICO-8** — fantasy consoles, a modern sizecoding playground.

### Super obscure (the rabbit hole's bottom)
Where the file format itself becomes the game — none of this survives a linker or
a runtime, listed only to show how deep it goes:
- **Header/code overlap:** ELF structures may overlap and sit anywhere, so the
  same bytes act as ELF header *and* program header *and* code. Ignored fields
  (`p_paddr`, `e_flags`) hold instructions; the load address is chosen so its
  bytes decode into opcodes. Raiter's "teensy ELF" hits **45 bytes**.
- **Strip past `strip`:** the section-header table isn't needed to run.
  `sstrip` (ELFkickers) and Section-Header-Stripper truncate it away.
- **TinyPE / rule-breaking PE:** the Windows equivalent (~97 bytes) — dropped
  data directories, code in the DOS stub / Rich-header region (see Corkami).
- **OS as the depacker:** call a decompressor already in the OS (Windows
  `ntdll!RtlDecompressBuffer`) so your unpack stub costs ~0 bytes.
- **Polyglots & loader tolerance:** one file valid as both ELF and PE (the ELF
  magic `7F 45` is a valid x86 jump); loaders zero-pad undersized files and
  ignore misalignment.

## Further reading

- [The Cargo Book — Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [`min-sized-rust`](https://github.com/johnthagen/min-sized-rust) — the canonical checklist
- [`cargo-bloat`](https://github.com/RazrFalcon/cargo-bloat) · [UPX](https://upx.github.io/)
- Demoscene: [Sizecoding Wiki](https://www.sizecoding.org/wiki/Main_Page) · [Crinkler](http://www.crinkler.net/) · [Pouët](https://www.pouet.net/)
- Tiny binaries: [Teensy ELF](https://www.muppetlabs.com/~breadbox/software/tiny/teensy.html) · [ELFkickers `sstrip`](https://github.com/BR903/ELFkickers/tree/master/sstrip) · [Section-Header-Stripper](https://github.com/blackle/Section-Header-Stripper)
