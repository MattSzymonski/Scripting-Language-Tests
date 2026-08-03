#!/usr/bin/env python3
"""End-to-end runner for the AAA DLL stress-test scenario.

Walks through the full pipeline in one command:

  1. (optional) clean + regenerate the module crates (generate_aaa_modules.py)
  2. (optional) compile all generated crates (build_aaa_scenario.py)
  3. start a timer
  4. launch stress_test.exe and let it load every module
  5. stop the timer the moment loading finishes, close the program, and
     print the measured time

The timer wraps the whole process lifetime (launch -> "modules loaded"),
not just the load_all() call Rust times internally -- so it also captures
process/window startup overhead, the same way a player would experience a
real game's boot time. For comparison, the script also prints the
Rust-internal load_all() duration it parses from stdout.

Usage:
    python run_stress_test.py                    # ask about every step
    python run_stress_test.py --yes               # skip regen+build, just run + time
    python run_stress_test.py --regen --scale medium --build --mode parallel
"""

import argparse
import os
import queue
import re
import subprocess
import sys
import threading
import time

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
GENERATE_SCRIPT = os.path.join(SCRIPT_DIR, "generate_aaa_modules.py")
BUILD_SCRIPT = os.path.join(SCRIPT_DIR, "build_aaa_scenario.py")
EXE_PATH = os.path.join(SCRIPT_DIR, "target", "release", "stress_test.exe")

SCALES = ["small", "medium", "full"]

READY_MARKER = "Results written to:"
LOADED_LINE_RE = re.compile(r"PluginManager: loaded (\d+)/(\d+) modules in ([\d.]+) ms")


def ask_yes_no(prompt, default=False):
    suffix = "[Y/n]" if default else "[y/N]"
    while True:
        reply = input(f"{prompt} {suffix} ").strip().lower()
        if not reply:
            return default
        if reply in ("y", "yes"):
            return True
        if reply in ("n", "no"):
            return False
        print("Please answer y or n.")


def ask_scale():
    print("Heaviness (module count):")
    print("  1) small   (~51 modules   - quick smoke test)")
    print("  2) medium  (~330 modules)")
    print("  3) full    (~1929 modules - matches the real AAA reference data)")
    while True:
        reply = input("Choose [1-3, default 1]: ").strip().lower()
        if reply in ("", "1", "small"):
            return "small"
        if reply in ("2", "medium"):
            return "medium"
        if reply in ("3", "full"):
            return "full"
        print("Please enter 1, 2, 3, or a scale name.")


def run_checked(cmd, cwd=None):
    print(f"$ {' '.join(cmd)}")
    result = subprocess.run(cmd, cwd=cwd)
    if result.returncode != 0:
        print(f"Command failed (exit {result.returncode}): {' '.join(cmd)}", file=sys.stderr)
        sys.exit(result.returncode)


def step_regenerate(args):
    do_it = args.regen
    if do_it is None:
        do_it = ask_yes_no("Clean and regenerate module crates?", default=False)
    if not do_it:
        print("Skipping module regeneration.")
        return

    scale = args.scale or ask_scale()
    run_checked([sys.executable, GENERATE_SCRIPT, "--scale", scale, "--clean"])


def step_build(args):
    do_it = args.build
    if do_it is None:
        do_it = ask_yes_no("Compile all generated crates now?", default=False)
    if not do_it:
        print("Skipping crate compilation.")
        return

    cmd = [sys.executable, BUILD_SCRIPT]
    if args.jobs:
        cmd += ["--jobs", str(args.jobs)]
    run_checked(cmd)


def ensure_harness_built():
    print("\nEnsuring the stress_test harness itself is built (release) ...")
    run_checked(["cargo", "build", "-p", "stress_test", "--release"], cwd=SCRIPT_DIR)


def stream_reader(pipe, line_queue):
    for line in iter(pipe.readline, ""):
        line_queue.put(line)
    line_queue.put(None)  # sentinel: pipe closed / process exited


def run_and_time(mode, timeout):
    if not os.path.isfile(EXE_PATH):
        print(f"error: {EXE_PATH} not found after build", file=sys.stderr)
        sys.exit(1)

    if mode == "lazy":
        print("Note: --lazy is a reserved/unimplemented mode in the harness -- "
              "it loads nothing and the process will exit almost immediately.")

    cmd = [EXE_PATH]
    if mode == "parallel":
        cmd.append("--parallel")
    elif mode == "lazy":
        cmd.append("--lazy")

    print(f"\n$ {' '.join(cmd)}   (cwd={SCRIPT_DIR})")
    print("--- program output ---")

    proc = subprocess.Popen(
        cmd, cwd=SCRIPT_DIR,
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
        text=True, bufsize=1,
    )

    line_queue = queue.Queue()
    reader = threading.Thread(target=stream_reader, args=(proc.stdout, line_queue), daemon=True)

    start = time.perf_counter()
    reader.start()

    internal_ms = None
    end = None
    timed_out = False
    deadline = start + timeout

    while True:
        remaining = deadline - time.perf_counter()
        if remaining <= 0:
            timed_out = True
            break
        try:
            line = line_queue.get(timeout=remaining)
        except queue.Empty:
            timed_out = True
            break
        if line is None:  # process exited without hitting the marker
            break
        print(line, end="")
        m = LOADED_LINE_RE.search(line)
        if m:
            internal_ms = float(m.group(3))
        if READY_MARKER in line:
            end = time.perf_counter()
            break

    if end is None:
        end = time.perf_counter()

    if timed_out:
        print(f"\nTimed out waiting for '{READY_MARKER}' after {timeout:.0f}s.", file=sys.stderr)

    print("--- closing program ---")
    proc.terminate()
    try:
        proc.wait(timeout=5)
    except subprocess.TimeoutExpired:
        proc.kill()
        proc.wait()

    elapsed_ms = (end - start) * 1000.0

    print()
    print("=" * 60)
    print(f"Wall-clock time (process launch -> modules loaded): {elapsed_ms:.1f} ms")
    if internal_ms is not None:
        print(f"Rust-reported load_all() time:                       {internal_ms:.1f} ms")
        print(f"Process/window startup overhead:                     {(elapsed_ms - internal_ms):.1f} ms")
    else:
        print("(Rust never reported a load_all() time -- loading may not have completed.)")
    print("=" * 60)


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--regen", dest="regen", action="store_true", default=None,
                         help="Regenerate module crates without asking")
    parser.add_argument("--no-regen", dest="regen", action="store_false",
                         help="Skip regeneration without asking")
    parser.add_argument("--scale", choices=SCALES, default=None,
                         help="Heaviness preset for regeneration (skips that prompt)")
    parser.add_argument("--build", dest="build", action="store_true", default=None,
                         help="Compile crates without asking")
    parser.add_argument("--no-build", dest="build", action="store_false",
                         help="Skip compiling without asking")
    parser.add_argument("--jobs", type=int, default=None, help="Passed to build_aaa_scenario.py --jobs")
    parser.add_argument("--mode", choices=["sequential", "parallel", "lazy"], default="sequential",
                         help="Load mode passed to stress_test.exe (default: sequential)")
    parser.add_argument("--timeout", type=float, default=180.0,
                         help="Seconds to wait for loading to finish before giving up (default: 180)")
    parser.add_argument("--yes", action="store_true",
                         help="Shortcut for --no-regen --no-build: just run + time the existing build")
    args = parser.parse_args()

    if args.yes:
        if args.regen is None:
            args.regen = False
        if args.build is None:
            args.build = False

    step_regenerate(args)
    step_build(args)
    ensure_harness_built()
    run_and_time(args.mode, args.timeout)


if __name__ == "__main__":
    main()
