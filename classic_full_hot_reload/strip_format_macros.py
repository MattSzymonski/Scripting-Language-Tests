# REQUIREMENTS
#   Python 3.  No third-party dependencies (stdlib only).
#
# DESCRIPTION
#   Strips every `format!` macro call from a Rust source file, replacing it
#   with `String::from("x")`.  This keeps the code fully type-valid (String
#   methods like `.len()` / `.to_uppercase()` still compile) while removing
#   the expensive macro-expansion work rustc performs for every `format!`
#   call on every build (see HOT_RELOAD_RESEARCH.md — front-end cost).
#
#   The transformation is robust: it tracks balanced parentheses, so
#   `format!("a {} b", nested_call(x))` is handled correctly, and it does
#   not touch `format` identifiers that are not macro invocations (e.g. a
#   local variable named `format`).
#
# USAGE
#   python strip_format_macros.py
#       Strips macros in place from game_lib/src/bloat_gen_no_export.rs
#       (the file currently wired into lib.rs).
#
#   python strip_format_macros.py --input path/to/foo.rs --output path/to/out.rs
#       Read from a custom input path and write to a custom output path.
#       If --output is omitted the input file is overwritten in place.
#
#   python strip_format_macros.py --dry-run
#       Report how many macros would be replaced without writing anything.
#
# EXAMPLE USAGE
#   python strip_format_macros.py --dry-run
#   python strip_format_macros.py
#   python strip_format_macros.py --input game_lib/src/bloat_gen.rs \
#       --output game_lib/src/bloat_gen_nofmt.rs
#
# --- SCRIPT ---

import argparse
import sys
from pathlib import Path

# ---------------------------------------------------------------------------
# Default paths — matches the classic_full_hot_reload layout.
# ---------------------------------------------------------------------------

DEFAULT_INPUT_PATH = Path("game_lib") / "src" / "bloat_gen_no_export.rs"

# ---------------------------------------------------------------------------
# Core transformation
# ---------------------------------------------------------------------------

def strip_format_macros_from_source(source_text: str) -> tuple[str, int]:
    """Replace every `format!(...)` invocation with `String::from("x")`.

    Returns the transformed source and the number of macros replaced.

    Scanning logic:
      - Iterate over the text looking for the literal token `format!`.
      - The next non-whitespace character must be `(` — otherwise it is not
        a macro invocation (e.g. a variable named `format`) and is skipped.
      - Walk forward tracking parenthesis depth so the closing paren of the
        macro call is found even with nested calls/grouping inside.
      - Preserve everything else byte-for-byte.

    Args:
        source_text: the full Rust source file contents.

    Returns:
        A tuple of (transformed_source, replaced_count).
    """
    replacement = 'String::from("x")'
    marker_token = "format!"
    replaced_count = 0

    output_parts: list[str] = []
    search_position = 0

    while True:
        token_index = source_text.find(marker_token, search_position)
        if token_index == -1:
            # No more markers — append the remaining text and stop.
            output_parts.append(source_text[search_position:])
            break

        # Copy everything up to (but NOT including) the `format!` token.
        # The token itself is only kept when it turns out not to be a
        # macro invocation.
        output_parts.append(source_text[search_position:token_index])

        # Find the opening parenthesis after the token (skipping whitespace).
        cursor = token_index + len(marker_token)
        while cursor < len(source_text) and source_text[cursor].isspace():
            cursor += 1

        if cursor >= len(source_text) or source_text[cursor] != "(":
            # Not a macro invocation (e.g. `format` used as a plain name).
            # Keep the token verbatim and continue searching after it.
            output_parts.append(marker_token)
            search_position = token_index + len(marker_token)
            continue

        # Walk to the matching close paren, tracking depth.  Strings are
        # skipped so that a `(` or `)` inside a string literal does not
        # confuse the depth count.
        paren_depth = 1
        scan_index = cursor + 1
        string_quote: str | None = None

        while scan_index < len(source_text) and paren_depth > 0:
            current_char = source_text[scan_index]

            if string_quote is not None:
                # Inside a string literal — only the same quote (or an
                # escaped quote) matters.
                if current_char == "\\":
                    scan_index += 2  # skip the escape sequence
                    continue
                if current_char == string_quote:
                    string_quote = None
            else:
                if current_char in ('"', "'"):
                    string_quote = current_char
                elif current_char == "(":
                    paren_depth += 1
                elif current_char == ")":
                    paren_depth -= 1
                    if paren_depth == 0:
                        break

            scan_index += 1

        # Drop the original macro call and emit the replacement instead.
        output_parts.append(replacement)
        replaced_count += 1

        # Continue searching after the closing paren of this macro.
        search_position = scan_index + 1

    return "".join(output_parts), replaced_count


# ---------------------------------------------------------------------------
# File helpers
# ---------------------------------------------------------------------------

def read_source_file(file_path: Path) -> str:
    """Read a Rust source file as UTF-8 text."""
    return file_path.read_text(encoding="utf-8")


def write_source_file(file_path: Path, source_text: str) -> None:
    """Write transformed text back to disk (creating parent dirs if needed)."""
    file_path.parent.mkdir(parents=True, exist_ok=True)
    file_path.write_text(source_text, encoding="utf-8", newline="\n")


# ---------------------------------------------------------------------------
# CLI entry point
# ---------------------------------------------------------------------------

def main() -> None:
    """Parse arguments, run the transformation, and report the result."""
    argument_parser = argparse.ArgumentParser(
        description="Strip format! macro calls from a Rust file, replacing them "
        "with String::from(\"x\")."
    )
    argument_parser.add_argument(
        "--input",
        type=Path,
        default=DEFAULT_INPUT_PATH,
        help=f"Source file to transform (default: {DEFAULT_INPUT_PATH})",
    )
    argument_parser.add_argument(
        "--output",
        type=Path,
        default=None,
        help="Output file.  If omitted the input file is overwritten in place.",
    )
    argument_parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Report how many macros would be replaced without writing anything.",
    )
    arguments = argument_parser.parse_args()

    input_path = arguments.input
    output_path = arguments.output if arguments.output is not None else input_path

    if not input_path.is_file():
        print(f"[strip_format_macros] ERROR: input file not found: {input_path}")
        sys.exit(1)

    original_source = read_source_file(input_path)
    transformed_source, replaced_count = strip_format_macros_from_source(original_source)

    print(f"[strip_format_macros] scanned   : {input_path}")
    print(f"[strip_format_macros] replaced  : {replaced_count} format! call(s)")

    if arguments.dry_run:
        print("[strip_format_macros] dry-run   : no files written")
        return

    write_source_file(output_path, transformed_source)
    print(f"[strip_format_macros] wrote     : {output_path}")
    print("[strip_format_macros] done.")


if __name__ == "__main__":
    main()
