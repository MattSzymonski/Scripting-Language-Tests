#!/usr/bin/env python3
# REQUIREMENTS
#   Python 3.8 or newer, Windows (the mini engine's prologue patching is
#   Windows/x64), a Rust toolchain with `cargo` on PATH, and the
#   `live_coding_mini_engine` workspace checked out.
#
# DESCRIPTION
#   End-to-end test suite for the live-coding mini engine.  It builds the
#   standalone host, launches it, then drives REAL source edits against
#   project/src/** and asserts that the host's stdout log shows the expected
#   behaviour:
#
#     * hot function body edits (project_update, lib helpers, a middle module
#       function, a leaf, and the modules.rs graph root) are recompiled
#       standalone and patched in place ("PATCHED <name> IN PLACE"),
#     * plumbing / const / bloat / new-function edits fall back to a full
#       rebuild ("hot-patchable = FALSE ... full rebuild"),
#     * a no-op save (identical content) is detected and skipped
#       ("nothing actually changed ... - skipping"),
#     * adding a brand-new module function is detected as a structure change,
#     * a lib helper that shares a bare name with a module function is STILL
#       hot (qualified registry keys - the collision test).
#
#   Run with --repeat N to loop the whole suite (flakiness check), --no-build
#   to reuse the last compiled host, or --stop-on-first-failure to abort at
#   the first failed test.
#
#   Every test restores the sources it touched, so the tree is left exactly
#   as it was.  Pass --keep-running to leave the host window open afterwards
#   (useful for manual inspection); the sources are still restored.
#
# USAGE
#   python run_live_coding_tests.py [options]
#
# EXAMPLE USAGE
#   python run_live_coding_tests.py
#   python run_live_coding_tests.py --verbose
#   python run_live_coding_tests.py --skip-collision-test --keep-running
#
# --- SCRIPT ---

import argparse
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

# Log anchors that the host must print for the suite to make sense.  The
# TEXT_* forms are used for plain substring checks; the RE_* forms are the
# same literals regex-escaped (the parens in the "nothing actually changed"
# line are literal text, not regex groups).
TEXT_COLD_LOADED = "base project.dll loaded"
TEXT_HOT_PATCHABLE = "hot-patchable = TRUE"
TEXT_FULL_REBUILD = "full rebuild"
TEXT_NOTHING_CHANGED = "nothing actually changed (identical to cache) - skipping"

RE_COLD_LOADED = re.escape(TEXT_COLD_LOADED)
RE_HOT_PATCHABLE = re.escape(TEXT_HOT_PATCHABLE)
RE_FULL_REBUILD = re.escape(TEXT_FULL_REBUILD)
RE_NOTHING_CHANGED = re.escape(TEXT_NOTHING_CHANGED)

# Strip ANSI colour escapes so log matching works whether or not the host
# detects a terminal.
ANSI_ESCAPE = re.compile(r"\x1b\[[0-9;]*m")

# The project's opt-in functional-verification hook (LIVE_CODING_VERIFY=1)
# prints this line whenever the deterministic checksum of the hot functions
# changes.  The suite parses it to prove patched code actually runs.
VERIFY_LINE = re.compile(r"\[verify\] checksum=0x([0-9A-Fa-f]{8})")


class TestFailure(AssertionError):
    """Raised when a single test case does not behave as expected."""


# ---------------------------------------------------------------------------
# Source-file helpers: read/write text preserving the file's exact line
# endings (the project sources use CRLF), and safe single-occurrence edits.
# ---------------------------------------------------------------------------

def read_file_preserving(path: Path) -> str:
    """Read a source file as text without translating line endings."""
    with open(path, "r", encoding="utf-8", errors="replace", newline="") as handle:
        return handle.read()


def write_file_preserving(path: Path, text: str) -> None:
    """Write a source file as text without translating line endings."""
    with open(path, "w", encoding="utf-8", newline="") as handle:
        handle.write(text)


def file_newline(text: str) -> str:
    """The dominant line ending of a source file (CRLF on this project)."""
    return "\r\n" if "\r\n" in text else "\n"


def regex_sub_once(path: Path, pattern: str, replacement, test_name: str, flags: int = re.DOTALL):
    """Replace exactly one regex match in a source file, preserving endings.

    `replacement` may be a string or a callable (match -> string).  Raises
    TestFailure when the pattern does not match exactly once, so a drifting
    source (numeric value changed by the user) fails loudly instead of
    silently no-op'ing the test.
    """
    text = read_file_preserving(path)
    updated, count = re.subn(pattern, replacement, text, count=1, flags=flags)
    if count != 1:
        raise TestFailure(
            f"[{test_name}] regex {pattern!r} matched {count} time(s) (expected 1) in {path}"
        )
    write_file_preserving(path, updated)
    return updated


def make_number_bumper(group_index: int):
    """Build a re.sub replacement that increments the captured integer.

    Only the captured span is replaced, so digits elsewhere in the match
    (e.g. inside "scale_value_0032") are never touched.
    """
    def replace(match: re.Match) -> str:
        span_start, span_end = match.span(group_index)
        match_start = match.start()
        new_number = int(match.group(group_index)) + 1
        return (
            match.group(0)[: span_start - match_start]
            + str(new_number)
            + match.group(0)[span_end - match_start:]
        )

    return replace


def replace_text_once(path: Path, old_text: str, new_text: str, test_name: str) -> None:
    """Replace exactly one literal substring in a source file.

    Raises TestFailure when the anchor is missing or ambiguous, so a drifting
    source fails loudly instead of silently no-op'ing the test.
    """
    text = read_file_preserving(path)
    if text.count(old_text) != 1:
        raise TestFailure(
            f"[{test_name}] anchor {old_text!r} not found exactly once in {path}"
        )
    write_file_preserving(path, text.replace(old_text, new_text))


def make_float_bumper(group_index: int):
    """Build a re.sub replacement that increments a captured float by 1.0."""
    def replace(match: re.Match) -> str:
        span_start, span_end = match.span(group_index)
        match_start = match.start()
        new_value = f"{float(match.group(group_index)) + 1.0:.1f}"
        return (
            match.group(0)[: span_start - match_start]
            + new_value
            + match.group(0)[span_end - match_start:]
        )

    return replace


# ---------------------------------------------------------------------------
# Host process management: build, launch, log tailing, idle detection.
# ---------------------------------------------------------------------------

def kill_standalone_processes() -> None:
    """Terminate any lingering standalone host (it locks the exe on disk)."""
    try:
        subprocess.run(
            ["taskkill", "/F", "/IM", "standalone.exe", "/T"],
            capture_output=True,
            check=False,
        )
    except OSError:
        pass


class LiveCodingHost:
    """Owns the standalone.exe subprocess and its redirected stdout log."""

    def __init__(self, workspace: Path, log_dir: Path, verbose: bool):
        self.workspace = workspace
        self.verbose = verbose
        self.log_path = log_dir / "host_stdout.log"
        self.error_path = log_dir / "host_stderr.log"
        self.process = None

    def build(self, timeout_seconds: int = 600) -> None:
        """Compile the standalone host (rebuilds the project crate too)."""
        kill_standalone_processes()
        print("  building standalone host (cargo build -p standalone)...")
        completed = subprocess.run(
            ["cargo", "build", "-p", "standalone"],
            cwd=str(self.workspace),
            capture_output=True,
            text=True,
            timeout=timeout_seconds,
        )
        if completed.returncode != 0:
            raise TestFailure(
                "cargo build failed:\n"
                + (completed.stderr or completed.stdout or "")[-4000:]
            )

    def start(self) -> None:
        """Launch the host with the verification hook enabled and logs captured."""
        kill_standalone_processes()
        executable = self.workspace / "target" / "debug" / "standalone.exe"
        if not executable.exists():
            raise TestFailure(f"host executable not found: {executable}")
        environment = dict(os.environ)
        environment["LIVE_CODING_VERIFY"] = "1"
        with open(self.log_path, "wb") as log_handle, open(self.error_path, "wb") as error_handle:
            self.process = subprocess.Popen(
                [str(executable)],
                cwd=str(self.workspace),
                stdout=log_handle,
                stderr=error_handle,
                env=environment,
            )

    def stop(self) -> None:
        """Terminate the host and clean up any stragglers."""
        if self.process is not None and self.process.poll() is None:
            self.process.terminate()
            try:
                self.process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                self.process.kill()
        kill_standalone_processes()

    def read_log(self) -> str:
        """Full current host stdout, ANSI escapes stripped."""
        try:
            with open(self.log_path, "r", encoding="utf-8", errors="replace") as handle:
                return ANSI_ESCAPE.sub("", handle.read())
        except OSError:
            return ""

    def read_error_log(self) -> str:
        """Full current host stderr."""
        try:
            with open(self.error_path, "r", encoding="utf-8", errors="replace") as handle:
                return handle.read()
        except OSError:
            return ""

    def is_running(self) -> bool:
        """True while the host subprocess is still alive."""
        return self.process is not None and self.process.poll() is None

    def describe_recent_log(self, previous_text: str, tail_lines: int = 40) -> str:
        """The log content produced since `previous_text` was snapshotted."""
        text = self.read_log()
        delta = text[len(previous_text):] if text.startswith(previous_text) else text
        lines = delta.strip().splitlines()
        return "\n".join(lines[-tail_lines:])

    def read_verification_checksum(self):
        """The most recently printed verification checksum, or None."""
        matches = VERIFY_LINE.findall(self.read_log())
        if not matches:
            return None
        return int(matches[-1], 16)

    def wait_for_checksum(self, predicate, timeout_seconds: int):
        """Wait until the most recent checksum satisfies `predicate`."""
        deadline = time.monotonic() + timeout_seconds
        while time.monotonic() < deadline:
            if not self.is_running():
                return None
            checksum = self.read_verification_checksum()
            if checksum is not None and predicate(checksum):
                return checksum
            time.sleep(0.2)
        return None

    def wait_for_log(
        self,
        previous_text: str,
        patterns,
        timeout_seconds: int,
        require_all: bool = True,
    ) -> bool:
        """Wait until the patterns appear in log content not in previous_text.

        `patterns` is a list of regex strings.  With require_all (default)
        every pattern must match the new content; otherwise the first match
        wins.  Returns True on success, False on timeout.
        """
        compiled = [re.compile(pattern) for pattern in patterns]
        # Only accept content produced AFTER a fresh change event, so a slow
        # finishing previous event (e.g. a full rebuild) can never make this
        # call match stale output from an earlier test.  The guard is skipped
        # when no change event has happened yet (the cold-load wait).
        previous_change_count = previous_text.count("========== change #")
        deadline = time.monotonic() + timeout_seconds
        while time.monotonic() < deadline:
            if not self.is_running():
                return False
            text = self.read_log()
            if previous_change_count == 0 or text.count("========== change #") > previous_change_count:
                delta = text[len(previous_text):] if text.startswith(previous_text) else text
                if delta:
                    if require_all:
                        if all(compiled_search.search(delta) for compiled_search in compiled):
                            return True
                    else:
                        if any(compiled_search.search(delta) for compiled_search in compiled):
                            return True
            time.sleep(0.2)
        return False

    def wait_until_idle(self, idle_seconds: float, timeout_seconds: int) -> bool:
        """Wait until the host log stops growing for idle_seconds."""
        deadline = time.monotonic() + timeout_seconds
        last_size = len(self.read_log())
        stable_since = time.monotonic()
        while time.monotonic() < deadline:
            if not self.is_running():
                return False
            time.sleep(0.2)
            size = len(self.read_log())
            if size != last_size:
                last_size = size
                stable_since = time.monotonic()
            elif time.monotonic() - stable_since >= idle_seconds:
                return True
        return False


# ---------------------------------------------------------------------------
# The test suite.
# ---------------------------------------------------------------------------

class LiveCodingTestSuite:
    """Runs each live-coding scenario against a freshly launched host."""

    def __init__(
        self,
        workspace: Path,
        log_dir: Path,
        verbose: bool,
        skip_collision: bool,
        no_build: bool = False,
        wait_timeout: int = 60,
        stop_on_first_failure: bool = False,
    ):
        self.workspace = workspace
        self.verbose = verbose
        self.skip_collision = skip_collision
        self.no_build = no_build
        self.wait_timeout = wait_timeout
        self.stop_on_first_failure = stop_on_first_failure
        self.project_src = workspace / "project" / "src"
        self.lib_rs = self.project_src / "lib.rs"
        self.modules_rs = self.project_src / "modules.rs"
        self.module_000 = self.project_src / "modules" / "module_000.rs"
        self.bloat_rs = self.project_src / "bloat_gen_no_export.rs"
        self.host = LiveCodingHost(workspace, log_dir, verbose)
        self.original_sources: dict = {}
        self.passed: list = []
        self.failed: list = []

    # -- infrastructure ----------------------------------------------------

    def snapshot_originals(self) -> None:
        """Record every source file the suite may touch, for restoration."""
        for path in (self.lib_rs, self.module_000, self.modules_rs, self.bloat_rs):
            self.original_sources[path] = read_file_preserving(path)

    def restore_originals(self) -> None:
        """Put every touched source file back to its suite-start content."""
        for path, original_text in self.original_sources.items():
            write_file_preserving(path, original_text)

    def assert_hot_patch(self, previous_text: str, function_name: str) -> None:
        """Assert the host recompiled and patched `function_name` in place."""
        matched = self.host.wait_for_log(
            previous_text,
            [
                rf"hot-patchable = TRUE \({re.escape(function_name)}\)",
                rf"PATCHED {re.escape(function_name)} IN PLACE",
            ],
            self.wait_timeout,
        )
        if not matched:
            raise TestFailure(
                f"expected {function_name} to be hot-patched in place but the "
                f"host did not report it\n"
                f"--- recent host log ---\n{self.host.describe_recent_log(previous_text)}"
            )

    def assert_full_rebuild(self, previous_text: str, reason_pattern: str = None) -> None:
        """Assert the host fell back to a full rebuild, optionally with a reason."""
        patterns = [r"hot-patchable = FALSE", RE_FULL_REBUILD]
        if reason_pattern:
            patterns.append(reason_pattern)
        matched = self.host.wait_for_log(previous_text, patterns, self.wait_timeout + 30)
        if not matched:
            raise TestFailure(
                "expected a full rebuild but the host did not report one\n"
                f"--- recent host log ---\n{self.host.describe_recent_log(previous_text)}"
            )

    def capture_checksum_baseline(self) -> int:
        """The verification checksum before an edit (the current live code)."""
        baseline = self.host.wait_for_checksum(lambda checksum: True, self.wait_timeout)
        if baseline is None:
            raise TestFailure(
                "no [verify] checksum available - is the host running with "
                "LIVE_CODING_VERIFY set?"
            )
        return baseline

    def assert_checksum_changed(self, baseline: int, function_name: str) -> int:
        """After patching `function_name`, the live checksum must differ.

        This is the functional proof: the patched function's new body is
        actually executing inside the running process, not just reported as
        patched by the host.
        """
        changed = self.host.wait_for_checksum(
            lambda checksum: checksum != baseline, self.wait_timeout
        )
        if changed is None:
            raise TestFailure(
                f"patching {function_name} did not change the live verification "
                f"checksum (still 0x{baseline:08X}) - the patched code is not running"
            )
        return changed

    def assert_checksum_restored(self, baseline: int, function_name: str) -> None:
        """After restoring the source, the live checksum must come back."""
        restored = self.host.wait_for_checksum(
            lambda checksum: checksum == baseline, self.wait_timeout
        )
        if restored is None:
            raise TestFailure(
                f"restoring {function_name} did not bring the live verification "
                f"checksum back to 0x{baseline:08X}"
            )

    # -- individual tests --------------------------------------------------

    def test_cold_load_detects_base_dll(self) -> None:
        """The freshly started host must build, load and cache the base DLL."""
        log = self.host.read_log()
        if TEXT_COLD_LOADED not in log:
            raise TestFailure("host never reported the base DLL loaded")
        match = re.search(r"(\d+) hot symbol\(s\) cached", log)
        if not match:
            raise TestFailure("cold-build log has no hot-symbol count")
        symbol_count = int(match.group(1))
        if symbol_count < 100:
            raise TestFailure(f"expected >= 100 hot symbols on cold start, got {symbol_count}")

    def test_hot_project_update_patches_in_place(self) -> None:
        """Editing the project_update body must patch that function in place."""
        previous = self.host.read_log()
        regex_sub_once(
            self.lib_rs,
            r"(scale_value_0032\(game_state\.tick as i32, )(\d+)(\);)",
            make_number_bumper(2),
            "hot-project-update",
        )
        self.assert_hot_patch(previous, "project_update")
        self.restore_originals()
        self.host.wait_until_idle(2.0, 60)

    def test_verification_checksum_is_deterministic(self) -> None:
        """The live checksum must be stable across frames (pure function)."""
        baseline = self.capture_checksum_baseline()
        time.sleep(1.0)
        current = self.host.read_verification_checksum()
        if current != baseline:
            raise TestFailure(
                f"verification checksum drifted without any edit: "
                f"0x{baseline:08X} -> 0x{current:08X}"
            )

    def test_hot_get_aaa_patches_in_place(self) -> None:
        """Editing the get_aaa body must patch it in place AND the new code
        must actually run (the live verification checksum changes)."""
        previous = self.host.read_log()
        baseline = self.capture_checksum_baseline()
        regex_sub_once(
            self.lib_rs,
            r"(fn get_aaa\(\) -> i32 \{\s*)(\d+)(\s*\})",
            make_number_bumper(2),
            "hot-get-aaa",
        )
        self.assert_hot_patch(previous, "get_aaa")
        self.assert_checksum_changed(baseline, "get_aaa")
        self.restore_originals()
        self.host.wait_until_idle(2.0, self.wait_timeout)
        self.assert_checksum_restored(baseline, "get_aaa")

    def test_hot_scale_value_0032_patches_in_place(self) -> None:
        """Editing the scale_value_0032 body must patch it in place AND the
        new code must actually run."""
        previous = self.host.read_log()
        baseline = self.capture_checksum_baseline()
        regex_sub_once(
            self.lib_rs,
            r"(input \* factor \* get_aaa\(\) \* )(\d+)",
            make_number_bumper(2),
            "hot-scale-value-0032",
        )
        self.assert_hot_patch(previous, "scale_value_0032")
        self.assert_checksum_changed(baseline, "scale_value_0032")
        self.restore_originals()
        self.host.wait_until_idle(2.0, self.wait_timeout)
        self.assert_checksum_restored(baseline, "scale_value_0032")

    def test_hot_module_function_patches_in_place(self) -> None:
        """Editing a module file's tick_000 body must patch it in place AND
        the new code must actually run (it feeds the checksum via run_update)."""
        previous = self.host.read_log()
        baseline = self.capture_checksum_baseline()
        regex_sub_once(
            self.module_000,
            r"(input \* )(\d+)",
            make_number_bumper(2),
            "hot-module-tick-000",
        )
        self.assert_hot_patch(previous, "tick_000")
        self.assert_checksum_changed(baseline, "tick_000")
        self.restore_originals()
        self.host.wait_until_idle(2.0, self.wait_timeout)
        self.assert_checksum_restored(baseline, "tick_000")

    def test_hot_leaf_sample_000_patches_in_place(self) -> None:
        """Editing a leaf function (sample_000) must patch it in place AND
        the new code must actually run (direct call in the checksum)."""
        previous = self.host.read_log()
        baseline = self.capture_checksum_baseline()
        regex_sub_once(
            self.module_000,
            r"(seed \* )(\d+)",
            make_number_bumper(2),
            "hot-leaf-sample-000",
        )
        self.assert_hot_patch(previous, "sample_000")
        self.assert_checksum_changed(baseline, "sample_000")
        self.restore_originals()
        self.host.wait_until_idle(2.0, self.wait_timeout)
        self.assert_checksum_restored(baseline, "sample_000")

    def test_hot_run_update_patches_in_place(self) -> None:
        """Editing modules.rs run_update (the graph root) must patch in place
        AND the new code must actually run."""
        previous = self.host.read_log()
        baseline = self.capture_checksum_baseline()
        regex_sub_once(
            self.modules_rs,
            r"(module_000::tick_000\(input \+ )(\d+)",
            make_number_bumper(2),
            "hot-run-update",
        )
        self.assert_hot_patch(previous, "run_update")
        self.assert_checksum_changed(baseline, "run_update")
        self.restore_originals()
        self.host.wait_until_idle(2.0, self.wait_timeout)
        self.assert_checksum_restored(baseline, "run_update")

    def test_plumbing_edit_forces_full_rebuild(self) -> None:
        """Editing project_set_api (plumbing) must trigger a full rebuild."""
        previous = self.host.read_log()
        text = read_file_preserving(self.lib_rs)
        if text.count("API = api;") != 1:
            raise TestFailure("anchor 'API = api;' not found exactly once in lib.rs")
        newline = file_newline(text)
        updated = text.replace(
            "API = api;",
            "API = api;" + newline + "        let _hot_reload_probe = 1;",
        )
        write_file_preserving(self.lib_rs, updated)

        self.assert_full_rebuild(
            previous, r"plumbing function changed \(project_set_api / project_init\)"
        )
        self.restore_originals()
        self.host.wait_until_idle(2.0, self.wait_timeout + 30)

    def test_const_edit_forces_full_rebuild(self) -> None:
        """Editing a top-level const (QUAD_SIZE) must trigger a full rebuild."""
        previous = self.host.read_log()
        regex_sub_once(
            self.lib_rs,
            r"(const QUAD_SIZE: f32 = )([0-9.]+)",
            make_float_bumper(2),
            "const-quad-size",
        )
        self.assert_full_rebuild(
            previous, r"non-body content changed \(structs/consts/static/bloat\)"
        )
        self.restore_originals()
        self.host.wait_until_idle(2.0, self.wait_timeout + 30)

    def test_bloat_edit_forces_full_rebuild(self) -> None:
        """Editing the dead-code bloat file must trigger a full rebuild, and
        the checksum must be identical after the reload (determinism across
        fresh DLL loads - bloat is dead code on the checksum path)."""
        previous = self.host.read_log()
        baseline = self.capture_checksum_baseline()
        replace_text_once(
            self.bloat_rs,
            "// 2000 unique functions",
            "// 2001 unique functions",
            "bloat-edit",
        )
        self.assert_full_rebuild(
            previous, r"non-body content changed \(structs/consts/static/bloat\)"
        )
        # A full reload builds a brand-new DLL; the checksum (a pure function
        # of the code) must come out identical - proving determinism across
        # independent loads.
        after_reload = self.host.wait_for_checksum(
            lambda checksum: True, self.wait_timeout
        )
        if after_reload != baseline:
            raise TestFailure(
                f"full rebuild changed the checksum despite identical behaviour: "
                f"0x{baseline:08X} -> 0x{after_reload:08X}"
            )
        self.restore_originals()
        self.host.wait_until_idle(2.0, self.wait_timeout + 30)

    def test_new_module_function_forces_full_rebuild(self) -> None:
        """Adding a brand-new module function is a structure change."""
        previous = self.host.read_log()
        original = read_file_preserving(self.module_000)
        newline = file_newline(original)
        extra_function = (
            "pub fn extra_tick_200(input: i32) -> i32 {"
            + newline
            + "    input + 200"
            + newline
            + "}"
        )
        write_file_preserving(self.module_000, original + newline + extra_function)
        self.assert_full_rebuild(previous, r"hot function set changed shape \(new/removed\)")
        self.restore_originals()
        self.host.wait_until_idle(2.0, self.wait_timeout + 30)

    def test_noop_save_is_skipped(self) -> None:
        """Rewriting a file with identical content must be skipped as a no-op."""
        previous = self.host.read_log()
        text = read_file_preserving(self.lib_rs)
        write_file_preserving(self.lib_rs, text)  # same bytes, fresh mtime
        matched = self.host.wait_for_log(previous, [RE_NOTHING_CHANGED], self.wait_timeout)
        if not matched:
            raise TestFailure("no-op save was not reported as 'nothing actually changed'")
        # Content is already the original - no restore needed.
        self.host.wait_until_idle(2.0, self.wait_timeout)

    def test_colliding_lib_helper_is_still_hot(self) -> None:
        """A lib helper sharing a name with a module fn must STILL hot-patch.

        Qualified registry keys make lib `scale_value_003` and
        `modules::module_003::scale_value_003` distinct, so adding the
        colliding helper re-registers 106 symbols (structure change -> full
        rebuild) and editing its body patches it in place.
        """
        previous = self.host.read_log()
        newline = file_newline(read_file_preserving(self.lib_rs))
        colliding_helper = (
            "fn scale_value_003(input: i32, factor: i32) -> i32 {"
            + newline
            + "    input * factor * get_aaa() * 2"
            + newline
            + "}"
        )

        # The current hot-symbol count (before adding the helper) - the
        # collision-aware rebuild must register exactly one more.
        current_count_match = re.search(r"(\d+) hot symbol", self.host.read_log())
        previous_count = int(current_count_match.group(1)) if current_count_match else 105
        expected_new_count = previous_count + 1

        # Insert the colliding lib helper just before scale_value_0032.
        regex_sub_once(
            self.lib_rs,
            r"(fn scale_value_0032\(input: i32, factor: i32\) -> i32 \{)",
            lambda match: colliding_helper + newline + newline + match.group(1),
            "collision-add-helper",
        )

        # Adding a hot function is a structure change -> full rebuild with
        # both the lib helper and the module twin registered (one extra
        # symbol) and the lib twin shown as a new hot symbol in the analysis.
        matched = self.host.wait_for_log(
            previous,
            [
                rf"{expected_new_count} hot symbol",
                RE_FULL_REBUILD,
                r"scale_value_003[^\n]*CHANGED",
            ],
            120,
        )
        if not matched:
            raise TestFailure(
                f"adding the colliding helper did not full-rebuild with "
                f"{expected_new_count} hot symbols"
            )

        # Editing the colliding lib helper's body must hot-patch in place
        # (the module twin stays untouched).
        previous = self.host.read_log()
        regex_sub_once(
            self.lib_rs,
            r"(fn scale_value_003\(input: i32, factor: i32\) -> i32 \{\s*input \* factor \* get_aaa\(\) \* )(\d+)",
            make_number_bumper(2),
            "collision-edit-helper",
        )
        self.assert_hot_patch(previous, "scale_value_003")

        # Remove the helper (restore) - structure change triggers a final
        # full rebuild back to the original 105-symbol shape.
        self.restore_originals()
        self.host.wait_until_idle(2.0, 120)

    # -- runner -------------------------------------------------------------

    def run(self) -> int:
        print("=" * 72)
        print("live-coding mini engine - end-to-end test suite")
        print("=" * 72)
        self.snapshot_originals()

        try:
            if not self.no_build:
                self.host.build()
            self.host.start()

            # Wait for the cold build + load to finish before the first edit.
            if not self.host.wait_for_log("", [RE_COLD_LOADED], 120):
                raise TestFailure(
                    "host did not report the base DLL loaded within 120s "
                    f"(stderr: {self.host.read_error_log()[-800:]})"
                )
            self.host.wait_until_idle(1.0, 30)

            tests = [
                self.test_cold_load_detects_base_dll,
                self.test_verification_checksum_is_deterministic,
                self.test_hot_project_update_patches_in_place,
                self.test_hot_get_aaa_patches_in_place,
                self.test_hot_scale_value_0032_patches_in_place,
                self.test_hot_module_function_patches_in_place,
                self.test_hot_leaf_sample_000_patches_in_place,
                self.test_hot_run_update_patches_in_place,
                self.test_plumbing_edit_forces_full_rebuild,
                self.test_const_edit_forces_full_rebuild,
                self.test_bloat_edit_forces_full_rebuild,
                self.test_new_module_function_forces_full_rebuild,
                self.test_noop_save_is_skipped,
            ]
            if not self.skip_collision:
                tests.append(self.test_colliding_lib_helper_is_still_hot)

            for test in tests:
                name = test.__name__
                print(f"\n  RUNNING  {name}")
                try:
                    test()
                except TestFailure as failure:
                    self.failed.append(name)
                    print(f"  FAILED   {name}\n    {failure}")
                    if self.stop_on_first_failure:
                        break
                except Exception as exception:  # noqa: BLE001 - suite reports any failure
                    self.failed.append(name)
                    print(f"  ERROR    {name}\n    {type(exception).__name__}: {exception}")
                    if self.stop_on_first_failure:
                        break
                else:
                    self.passed.append(name)
                    print(f"  PASSED   {name}")
        finally:
            # Always restore the sources, even on a failed/aborted test.
            self.restore_originals()

        print("\n" + "=" * 72)
        print(f"  passed: {len(self.passed)}   failed: {len(self.failed)}")
        for name in self.failed:
            print(f"    FAILED: {name}")
        print("=" * 72)
        return 0 if not self.failed else 1


# ---------------------------------------------------------------------------
# Entry point.
# ---------------------------------------------------------------------------

def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="End-to-end tests for the live-coding mini engine."
    )
    parser.add_argument(
        "--workspace",
        default=str(Path(__file__).resolve().parent),
        help="path to the live_coding_mini_engine workspace (default: script directory)",
    )
    parser.add_argument(
        "--verbose",
        action="store_true",
        help="print extra detail (host log deltas) while testing",
    )
    parser.add_argument(
        "--keep-running",
        action="store_true",
        help="leave the host window open after the suite finishes",
    )
    parser.add_argument(
        "--skip-collision-test",
        dest="skip_collision",
        action="store_true",
        help="skip the qualified-key collision test (adds ~10s)",
    )
    parser.add_argument(
        "--log-dir",
        default=None,
        help="directory for host logs (default: a temporary directory)",
    )
    parser.add_argument(
        "--no-build",
        action="store_true",
        help="reuse the last compiled host instead of running cargo build",
    )
    parser.add_argument(
        "--repeat",
        type=int,
        default=1,
        help="run the whole suite this many times (flakiness check)",
    )
    parser.add_argument(
        "--wait-timeout",
        type=int,
        default=60,
        help="seconds to wait for each expected log marker (default: 60)",
    )
    parser.add_argument(
        "--stop-on-first-failure",
        action="store_true",
        help="stop the suite after the first failed test",
    )
    return parser.parse_args()


def main() -> int:
    arguments = parse_arguments()
    workspace = Path(arguments.workspace).resolve()
    if not (workspace / "Cargo.toml").exists():
        print(f"error: {workspace} does not contain a Cargo.toml", file=sys.stderr)
        return 2
    if not arguments.no_build and shutil.which("cargo") is None:
        print("error: cargo not found on PATH - cannot build the host", file=sys.stderr)
        return 2

    if arguments.log_dir:
        log_dir = Path(arguments.log_dir)
        log_dir.mkdir(parents=True, exist_ok=True)
        keep_logs = True
    else:
        log_dir = Path(tempfile.mkdtemp(prefix="live_coding_tests_"))
        keep_logs = False

    rounds = max(1, arguments.repeat)
    overall_return_code = 0
    for round_number in range(1, rounds + 1):
        if rounds > 1:
            print(f"\n========== round {round_number}/{rounds} ==========")
        suite = LiveCodingTestSuite(
            workspace=workspace,
            log_dir=log_dir,
            verbose=arguments.verbose,
            skip_collision=arguments.skip_collision,
            no_build=arguments.no_build or round_number > 1,
            wait_timeout=arguments.wait_timeout,
            stop_on_first_failure=arguments.stop_on_first_failure,
        )
        try:
            round_return_code = suite.run()
        finally:
            if not arguments.keep_running:
                suite.host.stop()
        if round_return_code != 0:
            overall_return_code = round_return_code
            break

    if arguments.keep_running:
        print(f"\n  host left running - logs at {log_dir}")
    if not keep_logs and not arguments.keep_running:
        shutil.rmtree(log_dir, ignore_errors=True)
    return overall_return_code


if __name__ == "__main__":
    sys.exit(main())
