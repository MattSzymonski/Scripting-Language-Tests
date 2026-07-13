#!/usr/bin/env python3
"""Generate a tiered, genuinely-cross-linked Rust DLL module graph for the
"Example AAA Unreal Engine game project breakdown" stress-test scenario (see
dll_loading_performance_101.md).

Unlike a flat pile of independent plugin DLLs, this produces three tiers that
import from each other via real PE import tables (verified: build.rs passes
an absolute path to each dependency's MSVC import lib — `<name>.dll.lib` — via
`cargo:rustc-link-arg`, and Windows' own loader resolves the resulting
dependency chain at LoadLibrary time, recursively, exactly as described in
section 3/6 of the write-up):

  core       — foundational modules with zero intra-project dependencies
               (analogous to Core/CoreUObject/Engine). Few in number, skew
               toward heavier load-time cost.
  subsystem  — depend on 1-3 core modules. Analogous to engine subsystems.
  leaf       — the bulk of the module count. Depend on 2-7 modules drawn from
               core+subsystem (weighted so a few "hub" modules are popular,
               like Core/Engine in a real project). Never depended upon by
               anything else, and the only tier that's hot-reloadable.

Every module gets, independently of every other property:
  - a payload size bucket (drives on-disk file size, via `include_bytes!` of
    a generated payload.bin — NOT a const-evaluated array, which would make
    rustc's const-eval interpreter the compile-time bottleneck for anything
    beyond a few KB)
  - a load-time cost bucket (drives a busy-loop inside a `#[ctor]` — the Rust
    analog of a C++ global/static initializer, run before any export is
    callable)
  - a startup cost bucket (drives a busy-loop inside an explicit, separately-
    exported `module_startup()` — the analog of Unreal's StartupModule: an
    additive cost paid *after* the OS loader is done, not during it)

Size and cost are deliberately independent: the write-up's central point
(section 6) is that structurally similar modules can have wildly different
load times, because the real cost usually lives in a shared dependency
someone happens to load first — not in the module's own size or export count.

Build order matters: subsystem/leaf crates link against dependencies' .dll.lib
files that must already exist. See build_aaa_scenario.py, which builds
core -> subsystem -> leaf in that order into one shared --target-dir.

Usage:
    python generate_aaa_modules.py --scale full      # ~1929 modules (default)
    python generate_aaa_modules.py --scale medium     # ~330 modules
    python generate_aaa_modules.py --scale small       # ~51 modules, smoke test
    python generate_aaa_modules.py --core-count 15 --subsystem-count 150 --leaf-count 1764
"""

import argparse
import json
import os
import random
import shutil
import sys
import zlib


def stable_hash(s):
    """Deterministic string->int hash (Python's builtin hash() is randomized
    per-process via PYTHONHASHSEED, which would silently break the --seed
    reproducibility this generator promises)."""
    return zlib.crc32(s.encode("utf-8"))

SEED = 42
OUT_DIR_NAME = "aaa_scenario"

SCALE_PRESETS = {
    "small": (3, 8, 40),
    "medium": (6, 24, 300),
    "full": (15, 150, 1764),
}

# ── Independent distributions ────────────────────────────────────────────
# Each is a list of (label, weight, (lo, hi)) buckets; a bucket is chosen by
# weight, then a value is drawn uniformly from its (lo, hi) range.

PAYLOAD_BYTES_TIERS = [
    ("tiny", 0.45, (8_000, 60_000)),
    ("small", 0.25, (60_000, 300_000)),
    ("medium", 0.15, (300_000, 1_200_000)),
    ("large", 0.10, (1_200_000, 4_000_000)),
    ("huge", 0.05, (4_000_000, 12_000_000)),
]

# Load-time (ctor) cost, in busy-loop iterations. Distribution differs per
# tier: core modules are collectively "usually nontrivial to bring up" (like
# Engine/CoreUObject), leaves are almost all cheap except rare outliers.
CTOR_COST_TIERS = {
    "core": [
        ("trivial", 0.20, (50, 2_000)),
        ("light", 0.30, (5_000, 60_000)),
        ("medium", 0.35, (200_000, 900_000)),
        ("heavy", 0.15, (3_000_000, 14_000_000)),
    ],
    "subsystem": [
        ("trivial", 0.55, (50, 2_000)),
        ("light", 0.30, (5_000, 60_000)),
        ("medium", 0.12, (200_000, 900_000)),
        ("heavy", 0.03, (3_000_000, 14_000_000)),
    ],
    "leaf": [
        ("trivial", 0.90, (20, 800)),
        ("light", 0.07, (5_000, 40_000)),
        ("medium", 0.025, (150_000, 500_000)),
        ("heavy", 0.005, (2_000_000, 9_000_000)),
    ],
}

# StartupModule-equivalent cost (explicit call, after load).
STARTUP_COST_TIERS = [
    ("trivial", 0.85, (20, 300)),
    ("light", 0.10, (1_000, 8_000)),
    ("heavy", 0.05, (50_000, 300_000)),
]

EXPORT_COUNT_RANGE = {
    "core": (10, 300),
    "subsystem": (5, 120),
    "leaf": (1, 40),
}

DEP_COUNT_RANGE = {
    "core": (0, 0),
    "subsystem": (1, 3),
    "leaf": (2, 7),
}

# ── Bird identity (leaf tier only gets the fun palette; core/subsystem get a
# systematic, muted identity so the visualizer can tell tiers apart at a
# glance) ─────────────────────────────────────────────────────────────────

ADJECTIVES = [
    "Swift", "Brave", "Tiny", "Mighty", "Crimson", "Azure", "Golden", "Silver",
    "Shadow", "Flame", "Frost", "Thunder", "Crystal", "Ember", "Storm", "Blaze",
    "Shimmer", "Dusk", "Dawn", "Glimmer", "Spark", "Flash", "Bolt", "Drift",
    "Flicker", "Glide", "Soar", "Dart", "Flutter", "Whisper",
]
NOUNS = [
    "Falcon", "Eagle", "Hawk", "Sparrow", "Raven", "Dove", "Heron", "Kestrel",
    "Phoenix", "Gryphon", "Swift", "Skimmer", "Darter", "Glider", "Wing",
    "Talon", "Beak", "Feather", "Quill", "Pinion", "Plume", "Crest", "Tuft",
    "Fletch", "Aerie", "Eyrie", "Nest", "Roost", "Perch", "Bough",
]
LEAF_COLOR_PALETTE = [
    (231, 76, 60), (46, 204, 113), (52, 152, 219), (155, 89, 182),
    (241, 196, 15), (230, 126, 34), (26, 188, 156), (236, 240, 241),
    (192, 57, 43), (39, 174, 96), (41, 128, 185), (142, 68, 173),
    (243, 156, 18), (211, 84, 0), (22, 160, 133), (255, 107, 107), (99, 205, 218),
]


def pick_bucket(rng, tiers):
    """Weighted-pick a (label, lo, hi) tuple from a tier list, then a value in [lo, hi]."""
    labels, weights, ranges = zip(*tiers)
    idx = rng.choices(range(len(tiers)), weights=weights, k=1)[0]
    label = labels[idx]
    lo, hi = ranges[idx]
    value = rng.randint(lo, hi) if isinstance(lo, int) else rng.uniform(lo, hi)
    return label, value


def weighted_sample_without_replacement(rng, items_with_weights, k):
    """Pick up to k distinct items, weighted, without replacement."""
    pool = list(items_with_weights)
    chosen = []
    for _ in range(min(k, len(pool))):
        total = sum(w for _, w in pool)
        r = rng.uniform(0, total)
        upto = 0.0
        for i, (item, w) in enumerate(pool):
            upto += w
            if upto >= r:
                chosen.append(item)
                pool.pop(i)
                break
        else:
            chosen.append(pool.pop()[0])
    return chosen


def name_bytes_literal(name):
    raw = name.encode("utf-8")[:31]
    padded = list(raw) + [0] * (32 - len(raw))
    return "[" + ", ".join(f"{b}u8" for b in padded) + "]"


def gen_payload_bytes(n, seed):
    """Fast, moderate-entropy filler: a small random pattern tiled to length n.
    Deliberately not `os.urandom` (would push section entropy near 8.0 and
    trip the "possibly packed" heuristic in analyze_dll.py) and deliberately
    not computed inside a Rust `const fn` (would make rustc's const-eval
    interpreter, not the linker, the compile-time bottleneck for anything
    beyond a few KB)."""
    rng = random.Random(seed)
    pattern_len = 4096
    pattern = bytes(rng.randrange(256) for _ in range(pattern_len))
    reps = n // pattern_len + 1
    return (pattern * reps)[:n]


class Module:
    __slots__ = (
        "name", "tier", "index", "deps", "payload_bytes", "payload_bucket",
        "ctor_iterations", "ctor_bucket", "startup_iterations", "startup_bucket",
        "export_count", "identity_name", "color", "size",
    )


def build_modules(rng, core_count, subsystem_count, leaf_count):
    modules = []
    popularity = {}  # name -> weight, used so a few modules become "hubs"

    def make_identity(tier, idx, rng):
        if tier == "leaf":
            adj = rng.choice(ADJECTIVES)
            noun = rng.choice(NOUNS)
            name = f"{adj}_{noun}_{idx:04d}"
            base = rng.choice(LEAF_COLOR_PALETTE)
            color = tuple(min(255, max(0, c + rng.randint(-20, 20))) for c in base)
            size = round(rng.uniform(0.5, 2.0), 2)
        elif tier == "subsystem":
            name = f"Subsystem_{idx:03d}"
            color = (70, 120, 200)
            size = 1.0
        else:
            name = f"Core_{idx:02d}"
            color = (200, 200, 200)
            size = 1.0
        return name, color, size

    # ── Core tier: no deps ──
    for i in range(core_count):
        m = Module()
        m.name = f"aaa_core_{i:02d}"
        m.tier = "core"
        m.index = i
        m.deps = []
        m.payload_bucket, size_val = pick_bucket(rng, PAYLOAD_BYTES_TIERS)
        m.payload_bytes = int(size_val)
        m.ctor_bucket, ctor_val = pick_bucket(rng, CTOR_COST_TIERS["core"])
        m.ctor_iterations = int(ctor_val)
        m.startup_bucket, startup_val = pick_bucket(rng, STARTUP_COST_TIERS)
        m.startup_iterations = int(startup_val)
        lo, hi = EXPORT_COUNT_RANGE["core"]
        m.export_count = rng.randint(lo, hi)
        m.identity_name, m.color, m.size = make_identity("core", i, rng)
        popularity[m.name] = rng.paretovariate(1.4)
        modules.append(m)

    core_pool = [(m.name, popularity[m.name]) for m in modules]

    # ── Subsystem tier: 1-3 core deps ──
    for i in range(subsystem_count):
        m = Module()
        m.name = f"aaa_subsystem_{i:03d}"
        m.tier = "subsystem"
        m.index = i
        lo, hi = DEP_COUNT_RANGE["subsystem"]
        k = rng.randint(lo, hi)
        m.deps = weighted_sample_without_replacement(rng, core_pool, k)
        m.payload_bucket, size_val = pick_bucket(rng, PAYLOAD_BYTES_TIERS)
        m.payload_bytes = int(size_val)
        m.ctor_bucket, ctor_val = pick_bucket(rng, CTOR_COST_TIERS["subsystem"])
        m.ctor_iterations = int(ctor_val)
        m.startup_bucket, startup_val = pick_bucket(rng, STARTUP_COST_TIERS)
        m.startup_iterations = int(startup_val)
        lo, hi = EXPORT_COUNT_RANGE["subsystem"]
        m.export_count = rng.randint(lo, hi)
        m.identity_name, m.color, m.size = make_identity("subsystem", i, rng)
        popularity[m.name] = rng.paretovariate(1.4)
        modules.append(m)

    hub_pool = core_pool + [(m.name, popularity[m.name]) for m in modules if m.tier == "subsystem"]

    # ── Leaf tier: 2-7 deps drawn from core+subsystem ──
    for i in range(leaf_count):
        m = Module()
        m.name = f"aaa_leaf_{i:04d}"
        m.tier = "leaf"
        m.index = i
        lo, hi = DEP_COUNT_RANGE["leaf"]
        k = rng.randint(lo, hi)
        m.deps = weighted_sample_without_replacement(rng, hub_pool, k)
        m.payload_bucket, size_val = pick_bucket(rng, PAYLOAD_BYTES_TIERS)
        m.payload_bytes = int(size_val)
        m.ctor_bucket, ctor_val = pick_bucket(rng, CTOR_COST_TIERS["leaf"])
        m.ctor_iterations = int(ctor_val)
        m.startup_bucket, startup_val = pick_bucket(rng, STARTUP_COST_TIERS)
        m.startup_iterations = int(startup_val)
        lo, hi = EXPORT_COUNT_RANGE["leaf"]
        m.export_count = rng.randint(lo, hi)
        m.identity_name, m.color, m.size = make_identity("leaf", i, rng)
        modules.append(m)

    return modules


TIER_ID = {"core": 0, "subsystem": 1, "leaf": 2}


def render_lib_rs(m):
    extern_block = ""
    dep_touch_calls = ""
    if m.deps:
        decls = "\n".join(f"    fn {dep}_touch() -> u64;" for dep in m.deps)
        extern_block = f'extern "C" {{\n{decls}\n}}\n'
        dep_touch_calls = "\n".join(
            f"    acc = acc.wrapping_add(unsafe {{ {dep}_touch() }});" for dep in m.deps
        )

    extra_exports = "\n".join(
        f'#[no_mangle] pub extern "C" fn {m.name}_export_{i}() -> u32 {{ {i}u32 }}'
        for i in range(m.export_count)
    )

    dep_doc = f"//! Depends on: {', '.join(m.deps)}\n" if m.deps else "//! Depends on: (none — tier root)\n"

    return f"""//! AAA scenario module: {m.name} (tier: {m.tier})
//! Auto-generated by generate_aaa_modules.py — do not hand-edit.
{dep_doc}
use std::sync::atomic::{{AtomicU64, Ordering}};

/// Shared identity/metrics struct returned by every module in the scenario
/// (mirrors the old per-plugin BirdDefinition, generalized with a tier tag).
#[repr(C)]
pub struct ModuleIdentity {{
    pub name: [u8; 32],
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
    pub tier: u8,
    pub size: f32,
}}

{extern_block}
static PAYLOAD: &[u8] = include_bytes!("payload.bin");

static TOUCH: AtomicU64 = AtomicU64::new(0);

/// Runs at DLL load time, before any export is callable — the Rust analog of
/// a C++ global/static initializer (dll_loading_performance_101.md, section 3
/// step 6). This cost is charged to whichever module's LoadLibrary call is
/// first to pull this DLL into the process (section 6: "why load time is
/// deceptive") — not necessarily to this module's own "owner".
#[ctor::ctor]
fn on_load() {{
    let mut acc: u64 = {stable_hash(m.name)}u64;
    for i in 0..{m.ctor_iterations}u64 {{
        acc = acc.wrapping_mul(6364136223846793005).wrapping_add(i);
    }}
    acc = acc.wrapping_add(PAYLOAD[(acc as usize) % PAYLOAD.len()] as u64);
{dep_touch_calls}
    TOUCH.store(acc, Ordering::SeqCst);
}}

/// Exported so dependents get a genuine PE import edge — see this crate's
/// build.rs, and any dependent's own extern block above.
#[no_mangle]
pub extern "C" fn {m.name}_touch() -> u64 {{
    TOUCH.load(Ordering::SeqCst)
}}

/// "CreateInstance" phase — mirrors instantiating a module's IModuleInterface.
#[no_mangle]
pub extern "C" fn create_module_instance() -> ModuleIdentity {{
    ModuleIdentity {{
        name: {name_bytes_literal(m.identity_name)},
        color_r: {m.color[0]}u8,
        color_g: {m.color[1]}u8,
        color_b: {m.color[2]}u8,
        tier: {TIER_ID[m.tier]}u8,
        size: {m.size}f32,
    }}
}}

/// "StartupModule" phase — a separate, additive, explicit post-load call
/// (dll_loading_performance_101.md, AAA breakdown: "a separate, *additive*
/// line item on top of the OS-loader cost ... squarely each module owner's
/// responsibility").
#[no_mangle]
pub extern "C" fn module_startup() -> u32 {{
    let mut acc: u32 = {stable_hash(m.name + "_startup")}u32;
    for i in 0..{m.startup_iterations}u32 {{
        acc = acc.wrapping_mul(2246822519).wrapping_add(i);
    }}
    acc
}}

{extra_exports}
"""


def render_build_rs(m):
    dep_list = ", ".join(f'"{dep}"' for dep in m.deps)
    return f"""// Auto-generated by generate_aaa_modules.py — do not hand-edit.
//
// Links this module against its dependencies' MSVC import libraries
// (`<name>.dll.lib`, produced alongside every cdylib on windows-msvc). Passing
// the resolved absolute path directly via `rustc-link-arg` sidesteps the
// `.dll.lib` vs `.lib` naming mismatch that `cargo:rustc-link-lib=dylib=`
// would hit. This is what makes the resulting DLL's PE import table contain a
// real entry for each dependency, so Windows' own loader does the recursive
// transitive loading described in dll_loading_performance_101.md sections 3/6
// — nothing here needs to fake that behavior in Rust code.
//
// Requires the dependency tiers to already be built into the shared target
// dir (see build_aaa_scenario.py, which builds core -> subsystem -> leaf in
// that order for exactly this reason).
use std::path::PathBuf;

fn main() {{
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let shared_release = manifest_dir.join("..").join("..").join("target").join("release");
    let deps: &[&str] = &[{dep_list}];
    for dep in deps {{
        let lib_path = shared_release.join(format!("{{}}.dll.lib", dep));
        let lib_path = std::fs::canonicalize(&lib_path).unwrap_or_else(|e| {{
            panic!(
                "cannot find import library for dependency `{{}}` at {{}}: {{}} \\
                 (build earlier tiers first — see build_aaa_scenario.py)",
                dep, lib_path.display(), e
            )
        }});
        println!("cargo:rustc-link-arg={{}}", lib_path.display());
    }}
    println!("cargo:rerun-if-changed=build.rs");
}}
"""


def render_crate_cargo_toml(m):
    return f"""[package]
name = "{m.name}"
version = "0.1.0"
edition = "2021"
description = "AAA scenario module ({m.tier} tier) — auto-generated"

[lib]
crate-type = ["cdylib"]

[dependencies]
ctor = "0.2"
"""


def render_workspace_cargo_toml(names):
    members = "\n".join(f'    "{name}",' for name in names)
    return f"""[workspace]
resolver = "2"
members = [
{members}
]

[profile.release]
opt-level = "s"
strip = true
lto = false
"""


def write_crate(tier_dir, m):
    crate_dir = os.path.join(tier_dir, m.name)
    src_dir = os.path.join(crate_dir, "src")
    os.makedirs(src_dir, exist_ok=True)

    with open(os.path.join(crate_dir, "Cargo.toml"), "w", encoding="utf-8") as f:
        f.write(render_crate_cargo_toml(m))

    if m.deps:
        with open(os.path.join(crate_dir, "build.rs"), "w", encoding="utf-8") as f:
            f.write(render_build_rs(m))

    with open(os.path.join(src_dir, "lib.rs"), "w", encoding="utf-8") as f:
        f.write(render_lib_rs(m))

    payload = gen_payload_bytes(m.payload_bytes, seed=stable_hash(m.name))
    with open(os.path.join(src_dir, "payload.bin"), "wb") as f:
        f.write(payload)


def write_manifest(out_dir, modules, seed, core_count, subsystem_count, leaf_count):
    boot_rng = random.Random(seed + 1_000_003)
    names = [m.name for m in modules]
    boot_order = names[:]
    boot_rng.shuffle(boot_order)

    manifest = {
        "seed": seed,
        "counts": {"core": core_count, "subsystem": subsystem_count, "leaf": leaf_count,
                   "total": len(modules)},
        "target_release_dir": "target/release",
        "boot_order": boot_order,
        "modules": {
            m.name: {
                "tier": m.tier,
                "dll": f"{m.name}.dll",
                "deps": m.deps,
                "hot_reloadable": m.tier == "leaf",
                "payload_bucket": m.payload_bucket,
                "payload_bytes": m.payload_bytes,
                "ctor_bucket": m.ctor_bucket,
                "ctor_iterations": m.ctor_iterations,
                "startup_bucket": m.startup_bucket,
                "startup_iterations": m.startup_iterations,
                "export_count": m.export_count + 3,
                "identity": {
                    "name": m.identity_name,
                    "color": list(m.color),
                    "size": m.size,
                },
            }
            for m in modules
        },
    }
    with open(os.path.join(out_dir, "manifest.json"), "w", encoding="utf-8") as f:
        json.dump(manifest, f, indent=1)


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--scale", choices=sorted(SCALE_PRESETS), default="full",
                         help="Preset module counts (default: full, ~1929 modules)")
    parser.add_argument("--core-count", type=int, default=None)
    parser.add_argument("--subsystem-count", type=int, default=None)
    parser.add_argument("--leaf-count", type=int, default=None)
    parser.add_argument("--seed", type=int, default=SEED)
    parser.add_argument("--out", type=str, default=None, help=f"Output dir (default: ./{OUT_DIR_NAME}/)")
    parser.add_argument("--clean", action="store_true", help="Remove existing generated tree before writing")
    args = parser.parse_args()

    preset = SCALE_PRESETS[args.scale]
    core_count = args.core_count if args.core_count is not None else preset[0]
    subsystem_count = args.subsystem_count if args.subsystem_count is not None else preset[1]
    leaf_count = args.leaf_count if args.leaf_count is not None else preset[2]

    out_dir = os.path.abspath(args.out or os.path.join(os.path.dirname(__file__) or ".", OUT_DIR_NAME))

    if args.clean and os.path.isdir(out_dir):
        print(f"Removing existing {out_dir} ...")
        shutil.rmtree(out_dir)

    core_dir = os.path.join(out_dir, "core")
    subsystem_dir = os.path.join(out_dir, "subsystem")
    leaf_dir = os.path.join(out_dir, "leaf")
    for d in (core_dir, subsystem_dir, leaf_dir):
        os.makedirs(d, exist_ok=True)

    total = core_count + subsystem_count + leaf_count
    print(f"Generating AAA scenario: {core_count} core + {subsystem_count} subsystem + "
          f"{leaf_count} leaf = {total} modules (seed={args.seed}) into {out_dir}")

    rng = random.Random(args.seed)
    modules = build_modules(rng, core_count, subsystem_count, leaf_count)

    for m in modules:
        tier_dir = {"core": core_dir, "subsystem": subsystem_dir, "leaf": leaf_dir}[m.tier]
        write_crate(tier_dir, m)

    for tier_dir, names in (
        (core_dir, [m.name for m in modules if m.tier == "core"]),
        (subsystem_dir, [m.name for m in modules if m.tier == "subsystem"]),
        (leaf_dir, [m.name for m in modules if m.tier == "leaf"]),
    ):
        with open(os.path.join(tier_dir, "Cargo.toml"), "w", encoding="utf-8") as f:
            f.write(render_workspace_cargo_toml(names))

    write_manifest(out_dir, modules, args.seed, core_count, subsystem_count, leaf_count)

    total_payload = sum(m.payload_bytes for m in modules)
    print(f"Done. {total} modules written.")
    print(f"  Total embedded payload: {total_payload / 1_000_000:.1f} MB "
          f"(avg {total_payload / total / 1000:.1f} KB/module before Rust/CRT overhead)")
    print()
    print("Next: python build_aaa_scenario.py")


if __name__ == "__main__":
    main()
