# REQUIREMENTS
#   Python 3.  No third-party dependencies (stdlib only).
#
# DESCRIPTION
#   Generates an interconnected module graph for game_lib: 50 module files
#   (module_000.rs .. module_049.rs) where each module's functions call into
#   several other modules.  Unlike bloat_gen (dead code), these functions are
#   all reachable from `update`/`compute`, so the linker MUST process every
#   one of them — this is the "realistic large interconnected project" test.
#
#   Call graph design — interconnected but LINEAR in runtime:
#     - The root (modules.rs) fans out into ALL 50 modules once, so every
#       module's `tick_N` and `compute_N` are reachable.
#     - Each `tick_N` does local work, then pulls a value from 2 *other*
#       modules' leaf `sample` functions (cross-module edges).
#     - Each `compute_N` does local work, then pulls from 2 *other* modules'
#       leaf `sample` functions too.
#     - Sample targets wrap around (ring topology): every module's `sample_N`
#       is referenced by some other module, so there is no dead code.
#     - `sample_N` functions are leaves: they only do local work and call
#       the module's own private helpers.  No recursion between modules,
#       so total work stays linear (~150 calls per frame), while every .o
#       still references several other modules' symbols.
#
# USAGE
#   python generate_modules.py
#       Generates:
#         game_lib/src/modules.rs
#         game_lib/src/modules/module_000.rs .. module_049.rs
#
#   python generate_modules.py --count 100
#       Generates 100 modules instead of 50.
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
# Defaults — matches the classic_full_hot_reload_large_project layout.
# ---------------------------------------------------------------------------

MODULE_COUNT_DEFAULT = 50
SRC_DIR = Path("game_lib") / "src"
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
      - `tick_N(input)`   — entry used by the root fan-out
      - `compute_N(x,y)`  — entry used by the root fan-out
      - `sample_N(seed)`  — leaf function CALLED BY OTHER modules (cross edge)
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
        # No cross edges — fold the helpers in so nothing is unused.
        compute_tail = "    (base + scaled + offset) % 100000"

    return f"""// module_{index_str} — auto-generated by generate_modules.py, do not edit by hand.
// Part of an interconnected module graph: this module is called by the root
// fan-out AND its `sample_{index_str}` leaf is pulled by other modules.

/// Main tick entry point for this module.  Computes a local value from the
/// input frame and folds in values pulled from other modules' leaf functions.
pub fn tick_{index_str}(input: i32) -> i32 {{
    let local = (input * {module_index + 7} + 3) % 1000;
{tick_sample_block}
    {tick_tail}
}}

/// Main compute entry point for this module.  Takes x/y, scales them, then
/// folds in values pulled from other modules' leaf functions.
pub fn compute_{index_str}(x: i32, y: i32) -> i32 {{
    let base = x * {module_index + 3} + y * {module_index + 5};
    let scaled = scale_value(base, {module_index + 11});
    let offset = offset_value(base, {module_index + 13});
{compute_sample_block}
    {compute_tail}
}}

/// Leaf function called by OTHER modules.  Does local work only — never
/// recurses into another module — so the runtime stays linear even though
/// every module references this symbol.
pub fn sample_{index_str}(seed: i32) -> i32 {{
    let local = (seed * {module_index + 17} + {module_index * 3}) % 100000;
    let scaled = scale_value(local, {module_index + 19});
    let offset = offset_value(local, {module_index + 23});
    (local + scaled + offset) % 100000
}}

/// Private helper — scales a value and mixes in a per-module constant.
fn scale_value(value: i32, multiplier: i32) -> i32 {{
    (value * multiplier + {module_index * 3}) % 100000
}}

/// Private helper — offsets a value and decorates it with string work so the
/// compiler has real (non-trivial) code to emit for this module.
fn offset_value(value: i32, adder: i32) -> i32 {{
    let decorated = format!("module {index_str} value: {{}}", value + adder);
    let uppercase = decorated.to_uppercase();
    let length = uppercase.len() as i32;
    (length + value + adder) % 100000
}}
"""


def build_modules_file(module_count: int) -> str:
    """Return the contents of `modules.rs`, the module-graph root.

    Declares all generated submodules and provides two root functions that
    the game library entry points call.  The roots fan out into EVERY module
    exactly once, which is what makes every function in the graph reachable
    from `update`/`compute` while keeping runtime linear.

    Args:
        module_count: total number of modules in the graph.

    Returns:
        The complete modules.rs contents as a string.
    """
    declarations = "\n".join(
        f"pub mod module_{index:03d};" for index in range(module_count)
    )

    tick_calls = "\n".join(
        f"    let part_{index:03d} = module_{index:03d}::tick_{index:03d}(input + {index});"
        for index in range(module_count)
    )
    tick_sum = " + ".join(f"part_{index:03d}" for index in range(module_count))

    compute_calls = "\n".join(
        f"    let part_{index:03d} = module_{index:03d}::compute_{index:03d}(x + {index}, y + {index * 2});"
        for index in range(module_count)
    )
    compute_sum = " + ".join(f"part_{index:03d}" for index in range(module_count))

    return f"""// modules — auto-generated by generate_modules.py, do not edit by hand.
// Root of the interconnected module graph.  `run_update` and `run_compute`
// fan out into all {module_count} submodules, making every function in the
// graph reachable from the DLL's exported entry points.

{declarations}

/// Fan out a game tick into every module and fold the results together.
/// Called by the exported `update` function in lib.rs.
pub fn run_update(input: i32) -> i32 {{
{tick_calls}
    ({tick_sum}) % 100000
}}

/// Fan out a compute request into every module and fold the results.
/// Called by the exported `compute` function in lib.rs.
pub fn run_compute(x: i32, y: i32) -> i32 {{
{compute_calls}
    ({compute_sum}) % 100000
}}
"""


# ---------------------------------------------------------------------------
# File helpers
# ---------------------------------------------------------------------------

def write_file(file_path: Path, contents: str) -> None:
    """Write a file, creating parent directories as needed."""
    file_path.parent.mkdir(parents=True, exist_ok=True)
    file_path.write_text(contents, encoding="utf-8", newline="\n")


# ---------------------------------------------------------------------------
# CLI entry point
# ---------------------------------------------------------------------------

def main() -> None:
    """Parse arguments, generate the module files, and report what was done."""
    argument_parser = argparse.ArgumentParser(
        description="Generate an interconnected module graph for game_lib."
    )
    argument_parser.add_argument(
        "--count",
        type=int,
        default=MODULE_COUNT_DEFAULT,
        help=f"Number of modules to generate (default: {MODULE_COUNT_DEFAULT})",
    )
    argument_parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print how many files would be written without writing anything.",
    )
    arguments = argument_parser.parse_args()

    module_count = arguments.count
    if module_count < 1:
        print("[generate_modules] ERROR: --count must be at least 1")
        sys.exit(1)

    planned_files = 1 + module_count  # modules.rs + each module file
    print(f"[generate_modules] planning {module_count} modules ({planned_files} files)")

    if arguments.dry_run:
        print("[generate_modules] dry-run: no files written")
        return

    write_file(MODULES_FILE, build_modules_file(module_count))
    print(f"[generate_modules] wrote {MODULES_FILE}")

    for module_index in range(module_count):
        module_path = MODULES_DIR / f"module_{module_index:03d}.rs"
        write_file(module_path, build_module_file(module_index, module_count))

    print(f"[generate_modules] wrote {module_count} module files to {MODULES_DIR}")
    print("[generate_modules] done.")


if __name__ == "__main__":
    main()
