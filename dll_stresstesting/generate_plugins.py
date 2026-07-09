#!/usr/bin/env python3
"""Generate 300 Rust plugin crates for DLL stress testing.

Each plugin is a minimal cdylib crate that exports a single function:
    create_bird() -> BirdDefinition

Properties are generated with a seeded RNG for reproducibility across runs.

Usage:
    python generate_plugins.py                  # light plugins into ./plugins/
    python generate_plugins.py --heavy          # heavy plugins into ./plugins_heavy/
    python generate_plugins.py --count 100      # generate N plugins instead of 300
"""

import argparse
import os
import random
import sys

# ── Configuration ────────────────────────────────────────
SEED = 42
PLUGIN_COUNT = 300
OUTPUT_DIR = "plugins"

# Bird name components — combined to create unique names
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

# Color palette — vibrant, distinguishable colors
COLOR_PALETTE = [
    (231, 76, 60),    # red
    (46, 204, 113),   # green
    (52, 152, 219),   # blue
    (155, 89, 182),   # purple
    (241, 196, 15),   # yellow
    (230, 126, 34),   # orange
    (26, 188, 156),   # turquoise
    (236, 240, 241),  # white
    (52, 73, 94),     # dark blue-gray
    (192, 57, 43),    # dark red
    (39, 174, 96),    # dark green
    (41, 128, 185),   # dark blue
    (142, 68, 173),   # dark purple
    (243, 156, 18),   # dark yellow/amber
    (211, 84, 0),     # dark orange
    (22, 160, 133),   # dark turquoise
    (149, 165, 166),  # gray
    (127, 140, 141),  # dark gray
    (255, 107, 107),  # salmon
    (99, 205, 218),   # cyan
]


def generate_bird_properties(index: int, rng: random.Random) -> dict:
    """Generate deterministic bird properties for a plugin index."""
    adjective = rng.choice(ADJECTIVES)
    noun = rng.choice(NOUNS)
    name = f"{adjective}_{noun}_{index:03d}"

    # Pick a base color and add slight variation
    base_r, base_g, base_b = rng.choice(COLOR_PALETTE)
    color_r = min(255, max(0, base_r + rng.randint(-20, 20)))
    color_g = min(255, max(0, base_g + rng.randint(-20, 20)))
    color_b = min(255, max(0, base_b + rng.randint(-20, 20)))

    size = round(rng.uniform(0.5, 2.0), 2)

    return {
        "name": name,
        "color_r": color_r,
        "color_g": color_g,
        "color_b": color_b,
        "size": size,
    }


def format_name_bytes(name: str) -> str:
    """Format a string as a Rust byte array literal (fixed 32 bytes)."""
    name_bytes = name.encode("utf-8")[:31]
    padded = name_bytes + b"\x00" * (32 - len(name_bytes))
    return "[" + ", ".join(str(b) + "u8" for b in padded) + "]"


def generate_plugin_lib_rs(properties: dict) -> str:
    """Generate the src/lib.rs content for a single plugin."""
    name_bytes = properties["name"].encode("utf-8")[:31]
    padded = list(name_bytes) + [0] * (32 - len(name_bytes))
    name_array = "[" + ", ".join(f"{b}u8" for b in padded) + "]"

    return f"""//! Plugin: {properties['name']}
//! Auto-generated DLL stress-test bird plugin.

/// Definition of a bird rendered on the menu screen.
#[repr(C)]
pub struct BirdDefinition {{
    /// Null-terminated ASCII name (max 31 chars + null).
    pub name: [u8; 32],
    /// Red channel 0-255.
    pub color_r: u8,
    /// Green channel 0-255.
    pub color_g: u8,
    /// Blue channel 0-255.
    pub color_b: u8,
    /// Display size multiplier (0.5 - 2.0).
    pub size: f32,
}}

/// Create a bird with this plugin's unique properties.
/// Called by the host engine when loading this DLL.
#[no_mangle]
pub extern "C" fn create_bird() -> BirdDefinition {{
    BirdDefinition {{
        name: {name_array},
        color_r: {properties['color_r']}u8,
        color_g: {properties['color_g']}u8,
        color_b: {properties['color_b']}u8,
        size: {properties['size']:.2}f32,
    }}
}}
"""


def generate_plugin_cargo_toml(plugin_name: str) -> str:
    """Generate Cargo.toml for a single plugin crate."""
    return f"""[package]
name = "{plugin_name}"
version = "0.1.0"
edition = "2021"
description = "DLL stress-test bird plugin — auto-generated"

[lib]
crate-type = ["cdylib"]

[profile.release]
opt-level = "s"
lto = true
strip = true
"""


def generate_workspace_cargo_toml(plugin_names: list[str], heavy: bool = False) -> str:
    """Generate the workspace Cargo.toml for all plugins."""
    members = "\n".join(f'    "{name}",' for name in plugin_names)
    heavy_member = '\n    "heavy_common",' if heavy else ""
    return f"""[workspace]
resolver = "2"
members = [{heavy_member}
{members}
]

[profile.release]
opt-level = "s"
lto = true
strip = true
"""


# ── Heavy plugin generation ──────────────────────────────

def generate_heavy_plugin_cargo_toml(plugin_name: str) -> str:
    """Generate Cargo.toml for a heavy plugin with shared dependency."""
    return f"""[package]
name = "{plugin_name}"
version = "0.1.0"
edition = "2021"
description = "DLL stress-test HEAVY bird plugin — auto-generated"

[lib]
crate-type = ["cdylib"]

[dependencies]
heavy_common = {{ path = "../heavy_common" }}

[profile.release]
opt-level = "s"
lto = true
strip = true
"""


def generate_heavy_plugin_lib_rs(properties: dict) -> str:
    """Generate src/lib.rs for a heavy plugin with static data, multiple exports, and init work."""
    name_bytes = properties["name"].encode("utf-8")[:31]
    padded = list(name_bytes) + [0] * (32 - len(name_bytes))
    name_array = "[" + ", ".join(f"{b}u8" for b in padded) + "]"

    # Generate static embedded data — bloats the DLL (~5MB of .data section)
    # 4 arrays of different sizes to prevent compiler from optimizing them away
    static_data_values = ", ".join(
        str(((i * 2654435761 + properties["color_r"] * 1234567) % (1 << 32)) & 0xFFFFFFFF) + "u32"
        for i in range(0, 16384, 4)  # 4096 entries
    )

    return f"""//! SUPER-HEAVY Plugin: {properties['name']}
//! Simulates Unreal plugin: 5MB+ static data, 10 exports, real dependency tree.

use heavy_common::*;

// ── Static data arrays (~5 MB combined) ──
// Multiple arrays, different types — compiler can't easily deduplicate.

#[used]
static EMBEDDED_DATA_A: [u32; 4096] = [
    {static_data_values}
];

#[used]
static EMBEDDED_CONFIG: [u8; 65536] = {{
    let mut arr = [0u8; 65536];
    let mut i = 0;
    while i < 65536 {{
        arr[i] = (i.wrapping_mul({properties['color_r']} as usize + 7) % 256) as u8;
        i += 1;
    }}
    arr
}};

// ── Export 1: create_bird ──

#[no_mangle]
pub extern "C" fn create_bird() -> BirdDefinition {{
    initialize_plugin_module("{properties['name']}");

    // Validate all static data (prevents linker from stripping)
    let checksum: u32 = EMBEDDED_DATA_A.iter().fold(0u32, |a, b| a.wrapping_add(*b));
    let bytesum: u32 = EMBEDDED_CONFIG.iter().fold(0u32, |a, b| a.wrapping_add(*b as u32));
    let _ = std::hint::black_box((checksum, bytesum));

    // Simulate asset buffer allocation
    let mut buffer: Vec<u8> = Vec::with_capacity(4096);
    buffer.extend_from_slice(&EMBEDDED_CONFIG[..4096]);
    let _ = std::hint::black_box(buffer.capacity());

    BirdDefinition {{
        name: {name_array},
        color_r: {properties['color_r']}u8,
        color_g: {properties['color_g']}u8,
        color_b: {properties['color_b']}u8,
        size: {properties['size']:.2}f32,
    }}
}}

// ── Exports 2-10 (symbol resolution stress) ──

#[no_mangle] pub extern "C" fn plugin_get_version() -> u32 {{ 1 }}
#[no_mangle] pub extern "C" fn plugin_get_name() -> *const u8 {{ b"{properties['name']}\\0".as_ptr() }}
#[no_mangle] pub extern "C" fn plugin_get_data_size() -> usize {{ 4096 * 4 + 65536 }}
#[no_mangle] pub extern "C" fn plugin_validate() -> bool {{ !EMBEDDED_DATA_A.is_empty() && !EMBEDDED_CONFIG.is_empty() }}
#[no_mangle] pub extern "C" fn plugin_get_checksum() -> u64 {{ EMBEDDED_DATA_A.iter().fold(0u64, |a, b| a.wrapping_add(*b as u64)) }}
#[no_mangle] pub extern "C" fn plugin_get_config_byte(index: u32) -> u8 {{ EMBEDDED_CONFIG.get(index as usize).copied().unwrap_or(0) }}
#[no_mangle] pub extern "C" fn plugin_get_color_packed() -> u32 {{ (({properties['color_r']} as u32) << 16) | (({properties['color_g']} as u32) << 8) | ({properties['color_b']} as u32) }}
#[no_mangle] pub extern "C" fn plugin_get_static_count() -> u32 {{ EMBEDDED_DATA_A.len() as u32 }}
#[no_mangle] pub extern "C" fn plugin_self_test() -> bool {{ plugin_get_version() == 1 && plugin_validate() }}
"""



def main():
    parser = argparse.ArgumentParser(
        description="Generate Rust DLL stress-test plugin crates"
    )
    parser.add_argument(
        "--count", type=int, default=PLUGIN_COUNT,
        help=f"Number of plugins to generate (default: {PLUGIN_COUNT})"
    )
    parser.add_argument(
        "--output", type=str, default=None,
        help=f"Output directory (default: plugins/ or plugins_heavy/)"
    )
    parser.add_argument(
        "--seed", type=int, default=SEED,
        help=f"RNG seed for reproducibility (default: {SEED})"
    )
    parser.add_argument(
        "--heavy", action="store_true",
        help="Generate heavy plugins (shared dep, static data, multiple exports, init work)"
    )
    args = parser.parse_args()

    rng = random.Random(args.seed)

    if args.output is None:
        args.output = "plugins_heavy" if args.heavy else "plugins"
    output_path = os.path.abspath(args.output)
    os.makedirs(output_path, exist_ok=True)

    mode = "HEAVY" if args.heavy else "light"
    print(f"Generating {args.count} {mode} plugin crates into {output_path}/")
    print(f"Seed: {args.seed}")
    if args.heavy:
        print(f"  Shared dependency: heavy_common (simulates engine core)")
        print(f"  Per plugin: 100KB+ static data, 5 exports, init work")
    print()

    plugin_names = []

    for i in range(args.count):
        plugin_name = f"plugin_{i:03d}"
        plugin_names.append(plugin_name)

        plugin_dir = os.path.join(output_path, plugin_name)
        src_dir = os.path.join(plugin_dir, "src")
        os.makedirs(src_dir, exist_ok=True)

        properties = generate_bird_properties(i, rng)

        # Write Cargo.toml
        cargo_toml_path = os.path.join(plugin_dir, "Cargo.toml")
        with open(cargo_toml_path, "w", encoding="utf-8") as f:
            if args.heavy:
                f.write(generate_heavy_plugin_cargo_toml(plugin_name))
            else:
                f.write(generate_plugin_cargo_toml(plugin_name))

        # Write src/lib.rs
        lib_rs_path = os.path.join(src_dir, "lib.rs")
        with open(lib_rs_path, "w", encoding="utf-8") as f:
            if args.heavy:
                f.write(generate_heavy_plugin_lib_rs(properties))
            else:
                f.write(generate_plugin_lib_rs(properties))

        if (i + 1) % 50 == 0:
            print(f"  Generated {i + 1}/{args.count} plugins...")

    # Write workspace Cargo.toml
    workspace_toml = os.path.join(output_path, "Cargo.toml")
    with open(workspace_toml, "w", encoding="utf-8") as f:
        f.write(generate_workspace_cargo_toml(plugin_names, heavy=args.heavy))

    print(f"\nDone! Generated {args.count} {mode} plugins.")
    print(f"  Plugins: {output_path}/plugin_000/ ... {output_path}/plugin_{args.count - 1:03d}/")
    print(f"  Workspace: {workspace_toml}")
    print()
    print("Next steps:")
    print(f"  cd {output_path} && cargo build --release")
    print(f"  cargo run -p stress_test -- --heavy  (or --parallel --heavy)")


if __name__ == "__main__":
    main()
