# REQUIREMENTS
#   Python 3.  No third-party dependencies (stdlib only).
#
# DESCRIPTION
#   Generates an interconnected module graph for the mini-engine's `project`
#   crate: N module files (module_000.rs .. module_XXX.rs) where each module's
#   functions call into several other modules, plus the root fan-out file
#   modules.rs.  Unlike the bloat generator (dead code), every function here
#   is reachable from the exported `project_update`, so the linker MUST process
#   all of it - this is the "realistic large interconnected project" test.
#
#   Call graph design - interconnected but LINEAR in runtime:
#     - The root (modules.rs) fans out into ALL modules once, so every
#       module's `tick_N` and `compute_N` are reachable.
#     - Each `tick_N` does local work, then pulls a value from 2 *other*
#       modules' leaf `sample` functions (cross-module edges).
#     - Each `compute_N` does local work, then pulls from 2 *other* modules'
#       leaf `sample` functions too.
#     - Sample targets wrap around (ring topology): every module's `sample_N`
#       is referenced by some other module, so there is no dead code.
#     - `sample_N` functions are leaves: they only do local work and call
#       the module's own private helpers.  No recursion between modules, so
#       total work stays linear, while every module still references several
#       other modules' symbols.
#
# USAGE
#   python generate_modules.py
#       Generates (from live_coding_mini_engine/):
#         project/src/modules.rs
#         project/src/modules/module_000.rs .. module_019.rs
#
#   python generate_modules.py --count 100
#       Generates 100 modules instead of 20.
#
#   python generate_modules.py --dry-run
#       Print how many files would be written without writing anything.
#
# EXAMPLE USAGE
#   python generate_modules.py --dry-run
#   python generate_modules.py
#
# --- SCRIPT ---

import argparse
import sys
from pathlib import Path

# ---------------------------------------------------------------------------
# Defaults - matches the live_coding_mini_engine layout.
# ---------------------------------------------------------------------------

MODULE_COUNT_DEFAULT = 20
SRC_DIR = Path("project") / "src"
MODULES_DIR = SRC_DIR / "modules"
MODULES_FILE = SRC_DIR / "modules.rs"

# Cross-module edges: tick_N samples these other modules' leaf functions.
TICK_SAMPLE_OFFSETS = (3, 7)
# Cross-module edges: compute_N samples these other modules' leaf functions.
COMPUTE_SAMPLE_OFFSETS = (5, 11)


# ---------------------------------------------------------------------------
# File-content builders
# ---------------------------------------------------------------------------

def build_module_file(module_index: int, module_count: int) -> str:
    """Return the full Rust source for one interconnected module.

    Each module exposes:
      - `tick_N(input)`   - entry used by the root fan-out
      - `compute_N(x,y)`  - entry used by the root fan-out
      - `sample_N(seed)`  - leaf function CALLED BY OTHER modules (cross edge)
    plus two private helpers used by the module's own functions.

    Args:
        module_index: zero-based index of this module (0..module_count-1).
        module_count: total number of modules in the graph.

    Returns:
        The complete .rs file contents as a string.
    """
    index_str = f"{module_index:03d}"

    # Build cross-module sample calls for tick_N.  Targets wrap around so
    # every module's sample is referenced by someone (ring topology).
    tick_sample_lines = []
    tick_sample_exprs = []
    for offset in TICK_SAMPLE_OFFSETS:
        target_index = (module_index + offset) % module_count
        target_name = f"module_{target_index:03d}"
        tick_sample_lines.append(
            f"    let sample_{target_index:03d} = "
            f"crate::modules::{target_name}::sample_{target_index:03d}(local + {offset});"
        )
        tick_sample_exprs.append(f"sample_{target_index:03d}")

    # Build cross-module sample calls for compute_N.  Targets wrap around too.
    compute_sample_lines = []
    compute_sample_exprs = []
    for offset in COMPUTE_SAMPLE_OFFSETS:
        target_index = (module_index + offset) % module_count
        target_name = f"module_{target_index:03d}"
        compute_sample_lines.append(
            f"    let sample_{target_index:03d} = "
            f"crate::modules::{target_name}::sample_{target_index:03d}(base + {offset});"
        )
        compute_sample_exprs.append(f"sample_{target_index:03d}")

    tick_sample_block = "\n".join(tick_sample_lines)
    compute_sample_block = "\n".join(compute_sample_lines)

    if tick_sample_exprs:
        tick_fold = " + ".join(tick_sample_exprs)
        tick_tail = f"    (local + {tick_fold}) % 100000"
    else:
        # No cross edges (cannot happen with ring topology, but keep it safe).
        tick_tail = "    local"

    if compute_sample_exprs:
        compute_fold = " + ".join(compute_sample_exprs)
        compute_tail = f"    (base + scaled + offset + {compute_fold}) % 100000"
    else:
        compute_tail = "    (base + scaled + offset) % 100000"

    return f"""// module_{index_str} - auto-generated by generate_modules.py, do not edit by hand.
// Part of an interconnected module graph: this module is called by the root
// fan-out AND its `sample_{index_str}` leaf is pulled by other modules.

/// Main tick entry point for this module.  Computes a local value from the
/// input frame and folds in values pulled from other modules' leaf functions.
pub fn tick_{index_str}(input: i32) -> i32 {{
    let local = (input * 7 + {module_index}) % 1000;
{tick_sample_block}
{tick_tail}
}}

/// Main compute entry point for this module.  Takes x/y, scales them, then
/// folds in values pulled from other modules' leaf functions.
pub fn compute_{index_str}(x: i32, y: i32) -> i32 {{
    let base = x * 3 + y * 5;
    let scaled = scale_value_{index_str}(base, 11);
    let offset = offset_value_{index_str}(base, 13);
{compute_sample_block}
{compute_tail}
}}

/// Leaf function called by OTHER modules.  Does local work only - never
/// recurses into another module - so the runtime stays linear even though
/// every module references this symbol.
pub fn sample_{index_str}(seed: i32) -> i32 {{
    let local = (seed * 17 + {module_index}) % 100000;
    let scaled = scale_value_{index_str}(local, 19);
    let offset = offset_value_{index_str}(local, 23);
    (local + scaled + offset) % 100000
}}

/// Scales a value and mixes in a per-module constant.  Unique name per module
/// so the host's single `project_resolve_symbol` can find it by name.
pub fn scale_value_{index_str}(value: i32, multiplier: i32) -> i32 {{
    (value * multiplier + {module_index}) % 100000
}}

/// Offsets a value and decorates it with string work so the compiler has
/// real (non-trivial) code to emit for this module.  Unique name per module
/// so the host's single `project_resolve_symbol` can find it by name.
pub fn offset_value_{index_str}(value: i32, adder: i32) -> i32 {{
    let decorated = format!("module {module_index} value: {{}}", value + adder);
    let uppercase = decorated.to_uppercase();
    let length = uppercase.len() as i32;
    (length + value + adder + {module_index}) % 100000
}}
"""


def build_modules_file(module_count: int) -> str:
    """Return the root modules.rs that fans out into every module."""
    lines = [
        "// modules - auto-generated by generate_modules.py, do not edit by hand.",
        "// Root of the interconnected module graph.  `run_update` and",
        "// `run_compute` fan out into all submodules, making every function in",
        "// the graph reachable from the DLL's exported entry points.",
        "",
    ]
    for module_index in range(module_count):
        lines.append(f"pub mod module_{module_index:03d};")
    lines.extend(
        [
            "",
            "/// Fan out a game tick into every module and fold the results together.",
            "/// Called by the exported `project_update` in lib.rs.",
            "pub fn run_update(input: i32) -> i32 {",
        ]
    )
    for module_index in range(module_count):
        lines.append(f"    let part_{module_index:03d} = module_{module_index:03d}::tick_{module_index:03d}(input + {module_index});")
    fold_parts = " + ".join(f"part_{module_index:03d}" for module_index in range(module_count))
    lines.append(f"    ({fold_parts}) % 100000")
    lines.append("}")
    lines.extend(
        [
            "",
            "/// Fan out a compute request into every module and fold the results.",
            "/// Called by the exported `project_update` in lib.rs.",
            "pub fn run_compute(x: i32, y: i32) -> i32 {",
        ]
    )
    for module_index in range(module_count):
        lines.append(
            f"    let part_{module_index:03d} = module_{module_index:03d}::compute_{module_index:03d}(x + {module_index}, y + {module_index * 2});"
        )
    fold_parts = " + ".join(f"part_{module_index:03d}" for module_index in range(module_count))
    lines.append(f"    ({fold_parts}) % 100000")
    lines.append("}")
    # NOTE: the hot-symbol registry (name -> address) is NOT emitted here any
    # more - the crate's build.rs scans src/** and generates it into OUT_DIR.
    return "\n".join(lines) + "\n"


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main() -> int:
    parser = argparse.ArgumentParser(description="Generate the interconnected module graph for the mini engine's project crate.")
    parser.add_argument("--count", type=int, default=MODULE_COUNT_DEFAULT, help=f"number of modules (default {MODULE_COUNT_DEFAULT})")
    parser.add_argument("--dry-run", action="store_true", help="print what would be written without writing")
    args = parser.parse_args()

    if args.count < 1:
        print(f"error: --count must be >= 1, got {args.count}", file=sys.stderr)
        return 1

    MODULES_DIR.mkdir(parents=True, exist_ok=True)

    files_to_write = {MODULES_FILE: build_modules_file(args.count)}
    for module_index in range(args.count):
        module_path = MODULES_DIR / f"module_{module_index:03d}.rs"
        files_to_write[module_path] = build_module_file(module_index, args.count)

    if args.dry_run:
        print(f"would write {len(files_to_write)} files:")
        for path in sorted(files_to_write):
            print(f"  {path}")
        return 0

    for path, content in sorted(files_to_write.items()):
        path.write_text(content, encoding="utf-8")
        print(f"wrote {path} ({len(content)} bytes)")
    print(f"done - {len(files_to_write)} files")
    return 0


if __name__ == "__main__":
    sys.exit(main())
