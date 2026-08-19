// REQUIREMENTS
//   Rust (stable), Windows (prologue patching; other OSes fall back to a
//   pointer swap).  Run from the workspace root:
//     cargo run -p standalone
//
// DESCRIPTION
//   Live Coding host for the mini engine.  Statically links the engine (which
//   owns the macroquad window + render loop), loads the `project` cdylib ONCE,
//   hands it the engine's API table, and watches the project source.  When
//   you edit and save:
//
//     - If ONLY the `project_update` body changed, that one function is
//       recompiled standalone via rustc (~200 ms) and its PROLOGUE in the
//       loaded base DLL is patched in place (`E9 rel32`) - true Live Coding.
//       The base DLL stays loaded and the update pointer never changes.
//     - Anything else (settings, structs, other functions, Cargo.toml)
//       triggers a full cargo rebuild; the fresh DLL is loaded alongside and
//       the base prologue is re-patched to it.
//
//   The window, the engine and the project update pointer never change - only
//   the prologue bytes do.
//
// USAGE / EXAMPLE USAGE
//   cargo run -p standalone        # from live_coding_mini_engine/
//   Then edit project/src/lib.rs and save.  Press Escape to quit.
//
// --- SCRIPT ---

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use libloading::{Library, Symbol};
use mini_engine::{MiniEngine, ProjectApi, ProjectUpdateFn};
use notify::Watcher;

// ---------------------------------------------------------------------------
// ANSI colour helpers for the live-coding logs
// ---------------------------------------------------------------------------

/// Wrap `text` in an ANSI SGR code, but only when stdout is an interactive
/// terminal (redirected output stays plain and parseable).
fn paint(code: &str, text: &str) -> String {
    use std::io::IsTerminal;
    if std::io::stdout().is_terminal() {
        format!("\x1b[{code}m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}

fn paint_dim(text: &str) -> String {
    paint("2", text)
}
fn paint_cyan(text: &str) -> String {
    paint("36", text)
}
fn paint_bright_cyan(text: &str) -> String {
    paint("1;96", text)
}
fn paint_green(text: &str) -> String {
    paint("32", text)
}
fn paint_bright_green(text: &str) -> String {
    paint("1;92", text)
}
fn paint_yellow(text: &str) -> String {
    paint("33", text)
}
fn paint_bright_yellow(text: &str) -> String {
    paint("1;93", text)
}
fn paint_red(text: &str) -> String {
    paint("31", text)
}
fn paint_bright_red(text: &str) -> String {
    paint("1;91", text)
}
fn paint_magenta(text: &str) -> String {
    paint("35", text)
}

/// `[hot]` prefix.
fn hot_prefix() -> String {
    paint_bright_cyan("[hot]")
}


// ---------------------------------------------------------------------------
// Build + load helpers
// ---------------------------------------------------------------------------

/// Workspace root (parent of the standalone crate directory).
fn workspace_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// The project's source directory (contains lib.rs, modules.rs, modules/...).
fn project_src_dir() -> PathBuf {
    workspace_dir().join("project").join("src")
}

/// Read every `.rs` file under `project/src` (recursively) so the live-coding
/// analysis covers the whole project - lib.rs plus the bloat and module-graph
/// files - not just the main file.
fn collect_project_sources() -> Vec<(PathBuf, String)> {
    let mut sources = Vec::new();
    let mut stack = vec![project_src_dir()];
    while let Some(directory) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&directory) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    sources.push((path, content));
                }
            }
        }
    }
    // Stable order (lib.rs first, then modules.rs, then modules/...).
    sources.sort_by(|(path_a, _), (path_b, _)| path_a.cmp(path_b));
    sources
}

/// Run `cargo build -p project` and return the built DLL path on success, or
/// the captured compiler output on failure.
fn build_project() -> Result<PathBuf, String> {
    let workspace = workspace_dir();
    let target_dir = workspace.join("target_reload");

    let output = Command::new("cargo")
        .args(["build", "-p", "project", "--target-dir"])
        .arg(&target_dir)
        .current_dir(&workspace)
        .output()
        .map_err(|e| format!("failed to run cargo: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!("{stdout}{stderr}").trim().to_string());
    }

    Ok(target_dir.join("debug").join("project.dll"))
}

/// Monotonic counter used to give every loaded copy of the DLL a unique name.
static LIB_VERSION: AtomicU32 = AtomicU32::new(1);

/// Copy a freshly built DLL to a unique versioned name so the OS loader (and
/// Windows' file lock on the previous copy) never conflates versions.
fn versioned_lib_path(lib_dir: &Path) -> PathBuf {
    let version = LIB_VERSION.fetch_add(1, Ordering::Relaxed);
    lib_dir.join(format!("project_v{version}.dll"))
}

// ---------------------------------------------------------------------------
// Mini-lexer for extracting function bodies and constants from project source
// ---------------------------------------------------------------------------

/// Lexical mask: `true` where `source` is real code, `false` inside strings,
/// char literals, or comments.  Brace counting and marker searches only look
/// at code positions, so braces inside strings/comments can never confuse the
/// extractor.
fn code_mask(source: &str) -> Vec<bool> {
    enum State {
        Code,
        LineComment,
        BlockComment(u32),
        StringLit,
        CharLit,
    }

    let bytes = source.as_bytes();
    let n = bytes.len();
    let mut mask = vec![true; n];
    let mut state = State::Code;
    let mut i = 0usize;

    while i < n {
        let b = bytes[i];
        match state {
            State::Code => {
                if b == b'/' && i + 1 < n && bytes[i + 1] == b'/' {
                    mask[i] = false;
                    mask[i + 1] = false;
                    state = State::LineComment;
                    i += 2;
                    continue;
                }
                if b == b'/' && i + 1 < n && bytes[i + 1] == b'*' {
                    mask[i] = false;
                    mask[i + 1] = false;
                    state = State::BlockComment(1);
                    i += 2;
                    continue;
                }
                if b == b'"' {
                    mask[i] = false;
                    state = State::StringLit;
                    i += 1;
                    continue;
                }
                // Raw / byte string: r"..", r#".."#, b"..", br#".."#.
                if (b == b'r' || b == b'b') && i + 1 < n {
                    let mut j = i + 1;
                    let mut hashes = 0usize;
                    while j < n && bytes[j] == b'#' {
                        hashes += 1;
                        j += 1;
                    }
                    if j < n && bytes[j] == b'"' {
                        for k in i..=j {
                            mask[k] = false;
                        }
                        let mut k = j + 1;
                        let mut closed = false;
                        while k < n {
                            if bytes[k] == b'"' {
                                let mut h = 0usize;
                                let mut m = k + 1;
                                while m < n && h < hashes && bytes[m] == b'#' {
                                    h += 1;
                                    m += 1;
                                }
                                if h == hashes {
                                    for z in k..m {
                                        mask[z] = false;
                                    }
                                    i = m;
                                    closed = true;
                                    break;
                                }
                            }
                            mask[k] = false;
                            k += 1;
                        }
                        if closed {
                            continue;
                        }
                    }
                }
                // Char literal `'x'` / `'\n'` (not a lifetime `'a`).
                if b == b'\'' {
                    let mut j = i + 1;
                    let mut is_char_literal = false;
                    while j < n
                        && bytes[j] != b'\n'
                        && bytes[j] != b' '
                        && bytes[j] != b'\t'
                    {
                        if bytes[j] == b'\\' {
                            j += 1;
                            if j < n {
                                j += 1;
                            }
                            continue;
                        }
                        if bytes[j] == b'\'' {
                            is_char_literal = true;
                            break;
                        }
                        j += 1;
                    }
                    if is_char_literal {
                        mask[i] = false;
                        state = State::CharLit;
                        i += 1;
                        continue;
                    }
                }
                i += 1;
            }
            State::LineComment => {
                mask[i] = false;
                if b == b'\n' {
                    state = State::Code;
                }
                i += 1;
            }
            State::BlockComment(depth) => {
                mask[i] = false;
                if b == b'/' && i + 1 < n && bytes[i + 1] == b'*' {
                    mask[i + 1] = false;
                    state = State::BlockComment(depth + 1);
                    i += 2;
                    continue;
                }
                if b == b'*' && i + 1 < n && bytes[i + 1] == b'/' {
                    mask[i + 1] = false;
                    state = if depth == 1 {
                        State::Code
                    } else {
                        State::BlockComment(depth - 1)
                    };
                    i += 2;
                    continue;
                }
                i += 1;
            }
            State::StringLit => {
                mask[i] = false;
                if b == b'\\' && i + 1 < n {
                    mask[i + 1] = false;
                    i += 2;
                    continue;
                }
                if b == b'"' {
                    state = State::Code;
                }
                i += 1;
            }
            State::CharLit => {
                mask[i] = false;
                if b == b'\\' && i + 1 < n {
                    mask[i + 1] = false;
                    i += 2;
                    continue;
                }
                if b == b'\'' {
                    state = State::Code;
                }
                i += 1;
            }
        }
    }
    mask
}

/// Find `pub extern "C" fn ` at or after `from`.  The leading `pub extern `
/// must be real code; the quoted `"C"` part is lexed as a string literal, so
/// it is matched verbatim rather than via the code mask.
fn find_extern_marker(bytes: &[u8], mask: &[bool], from: usize) -> Option<usize> {
    let code_part = b"pub extern ";
    let quoted_part = b"\"C\" fn ";
    let total = code_part.len() + quoted_part.len();
    let n = bytes.len();
    let mut i = from;
    while i + total <= n {
        if &bytes[i..i + code_part.len()] == code_part
            && mask[i..i + code_part.len()].iter().all(|&is_code| is_code)
            && &bytes[i + code_part.len()..i + total] == quoted_part
        {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Lexically-aware extractor: `pub extern "C" fn NAME(params) [-> Ret] { body }`.
/// Returns `Vec<(name, params, body, return_type)>`.
fn extract_function_bodies(source: &str) -> Vec<(String, String, String, String)> {
    let mask = code_mask(source);
    let bytes = source.as_bytes();
    let marker_total = b"pub extern ".len() + b"\"C\" fn ".len();
    let mut results = Vec::new();
    let mut i = 0usize;
    let n = bytes.len();

    while let Some(start) = find_extern_marker(bytes, &mask, i) {
        let name_start = start + marker_total;
        let mut name_end = name_start;
        while name_end < n && bytes[name_end] != b'(' {
            name_end += 1;
        }
        let name = source[name_start..name_end].trim().to_string();

        let mut paren_depth = 0u32;
        let mut params_end = 0usize;
        let mut j = name_end;
        let mut found_open = false;
        while j < n {
            if mask[j] {
                if bytes[j] == b'(' {
                    paren_depth += 1;
                    found_open = true;
                } else if bytes[j] == b')' {
                    paren_depth -= 1;
                    if found_open && paren_depth == 0 {
                        params_end = j;
                        break;
                    }
                }
            }
            j += 1;
        }
        let params = source[name_end + 1..params_end].trim().to_string();

        let mut k = params_end + 1;
        let mut body_open = 0usize;
        while k < n {
            if mask[k] && bytes[k] == b'{' {
                body_open = k;
                break;
            }
            k += 1;
        }
        let ret_section = &source[params_end + 1..body_open];
        let ret_type = if let Some(arrow) = ret_section.find("->") {
            ret_section[arrow + 2..]
                .trim()
                .split(|c: char| c.is_whitespace() || c == '{')
                .next()
                .unwrap_or("")
                .to_string()
        } else {
            String::new()
        };

        let mut depth = 1u32;
        let mut body_end = 0usize;
        let mut b = body_open + 1;
        while b < n {
            if mask[b] {
                if bytes[b] == b'{' {
                    depth += 1;
                } else if bytes[b] == b'}' {
                    depth -= 1;
                    if depth == 0 {
                        body_end = b;
                        break;
                    }
                }
            }
            b += 1;
        }
        let body = source[body_open + 1..body_end].trim().to_string();

        results.push((name, params, body, ret_type));
        i = body_end + 1;
    }

    results
}

/// Return `source` with the BODY of every `pub extern "C" fn` removed (the
/// signatures are kept).  Used to tell "only a function body changed" (fast
/// prologue patch) from "non-body content changed" (full rebuild).
fn strip_extern_bodies(source: &str) -> String {
    let mask = code_mask(source);
    let bytes = source.as_bytes();
    let mut result = String::new();
    let mut i = 0usize;
    let n = bytes.len();

    while let Some(start) = find_extern_marker(bytes, &mask, i) {
        result.push_str(&source[i..start]);
        let mut k = start;
        let mut open: Option<usize> = None;
        while k < n {
            if mask[k] && bytes[k] == b'{' {
                open = Some(k);
                break;
            }
            k += 1;
        }
        let Some(open) = open else {
            result.push_str(&source[start..]);
            return result;
        };
        result.push_str(&source[start..=open]);
        let mut depth = 1u32;
        let mut close: Option<usize> = None;
        let mut b = open + 1;
        while b < n {
            if mask[b] {
                if bytes[b] == b'{' {
                    depth += 1;
                } else if bytes[b] == b'}' {
                    depth -= 1;
                    if depth == 0 {
                        close = Some(b);
                        break;
                    }
                }
            }
            b += 1;
        }
        match close {
            Some(close) => i = close + 1,
            None => return result,
        }
    }

    result.push_str(&source[i..]);
    result
}

/// Extract every top-level `const` declaration from `source` as trimmed
/// declaration lines (skipping strings/comments).  Used to inject the
/// project's current constant values into a standalone-compiled function.
fn extract_top_level_consts(source: &str) -> Vec<String> {
    let mask = code_mask(source);
    let bytes = source.as_bytes();
    let n = bytes.len();
    let mut consts = Vec::new();
    let mut i = 0usize;

    while i < n {
        // A real `const` declaration starts a statement (preceded by a
        // delimiter), never the `const` inside `*const T` type syntax.
        let prev_is_delimiter = i == 0
            || matches!(
                bytes[i - 1],
                b'\n' | b'\r' | b'\t' | b' ' | b';' | b'{' | b'}' | b']'
            );
        let is_word_start = prev_is_delimiter
            && mask[i]
            && bytes[i..].starts_with(b"const")
            && (i + 5 >= n || !(bytes[i + 5].is_ascii_alphanumeric() || bytes[i + 5] == b'_'));
        if is_word_start {
            // Name (after the whitespace following `const`), for the
            // MAX_QUADS exclusion.
            let mut name_start = i + 5;
            while name_start < n
                && (bytes[name_start] == b' ' || bytes[name_start] == b'\t' || bytes[name_start] == b'\n')
            {
                name_start += 1;
            }
            let mut name_end = name_start;
            while name_end < n
                && (bytes[name_end].is_ascii_alphanumeric() || bytes[name_end] == b'_')
            {
                name_end += 1;
            }
            let name = &source[name_start..name_end];

            // A real const declaration has `NAME:` after the name; the `const`
            // in `*const T` does not.
            let mut colon_pos = name_end;
            while colon_pos < n && (bytes[colon_pos] == b' ' || bytes[colon_pos] == b'\t') {
                colon_pos += 1;
            }
            let is_declaration = colon_pos < n && bytes[colon_pos] == b':';

            // Skip MAX_QUADS - the generated module defines its own (it sizes
            // the GameState mirror).
            if name != "MAX_QUADS" && is_declaration {
                // Find the declaration end: ';' at bracket depth 0, in code.
                let mut depth_round = 0u32;
                let mut depth_curly = 0u32;
                let mut depth_square = 0u32;
                let mut j = i;
                let mut end = 0usize;
                while j < n {
                    if mask[j] {
                        match bytes[j] {
                            b'(' => depth_round += 1,
                            b')' => depth_round = depth_round.saturating_sub(1),
                            b'{' => depth_curly += 1,
                            b'}' => depth_curly = depth_curly.saturating_sub(1),
                            b'[' => depth_square += 1,
                            b']' => depth_square = depth_square.saturating_sub(1),
                            b';' if depth_round == 0 && depth_curly == 0 && depth_square == 0 => {
                                end = j;
                                break;
                            }
                            _ => {}
                        }
                    }
                    j += 1;
                }
                if end > i {
                    consts.push(source[i..=end].trim().to_string());
                    i = end + 1;
                    continue;
                }
            }
        }
        i += 1;
    }

    consts
}

/// ABI mirror structs the generated module always defines itself, so they
/// must never be injected from the project source (that would duplicate them).
const MIRROR_TYPE_NAMES: [&str; 3] = ["ProjectApi", "Quad", "GameState"];

/// Extract the project's top-level `struct` / `enum` / `type` definitions
/// (skipping strings/comments and the ABI mirrors), so a standalone-compiled
/// body can construct and use project-local types.
fn extract_top_level_type_definitions(source: &str) -> Vec<String> {
    let mask = code_mask(source);
    let bytes = source.as_bytes();
    let n = bytes.len();
    let mut definitions = Vec::new();
    let mut i = 0usize;

    while i < n {
        // Match `struct `, `enum ` or `type ` as a statement start.
        let keyword: Option<&[u8]> = if bytes[i..].starts_with(b"struct ") {
            Some(b"struct ")
        } else if bytes[i..].starts_with(b"enum ") {
            Some(b"enum ")
        } else if bytes[i..].starts_with(b"type ") {
            Some(b"type ")
        } else {
            None
        };

        let prev_is_delimiter = i == 0
            || matches!(
                bytes[i - 1],
                b'\n' | b'\r' | b'\t' | b' ' | b';' | b'{' | b'}' | b']'
            );

        if let Some(keyword) = keyword {
            if prev_is_delimiter && mask[i] {
                // Type name (after the whitespace following the keyword).
                let mut name_start = i + keyword.len();
                while name_start < n
                    && (bytes[name_start] == b' ' || bytes[name_start] == b'\t' || bytes[name_start] == b'\n')
                {
                    name_start += 1;
                }
                let mut name_end = name_start;
                while name_end < n
                    && (bytes[name_end].is_ascii_alphanumeric() || bytes[name_end] == b'_')
                {
                    name_end += 1;
                }
                let name = &source[name_start..name_end];

                // Skip the ABI mirrors - the template defines them itself.
                if !MIRROR_TYPE_NAMES.contains(&name) {
                    // Find the end of the definition.
                    let end = if keyword == b"type " {
                        // `type X = ...;` - up to ';' at bracket depth 0.
                        let mut depth_round = 0u32;
                        let mut depth_curly = 0u32;
                        let mut depth_square = 0u32;
                        let mut j = i;
                        let mut end = 0usize;
                        while j < n {
                            if mask[j] {
                                match bytes[j] {
                                    b'(' => depth_round += 1,
                                    b')' => depth_round = depth_round.saturating_sub(1),
                                    b'{' => depth_curly += 1,
                                    b'}' => depth_curly = depth_curly.saturating_sub(1),
                                    b'[' => depth_square += 1,
                                    b']' => depth_square = depth_square.saturating_sub(1),
                                    b';' if depth_round == 0 && depth_curly == 0 && depth_square == 0 => {
                                        end = j;
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                            j += 1;
                        }
                        end
                    } else {
                        // `struct`/`enum` - up to the matching '}'.
                        let mut open = 0usize;
                        let mut j = i;
                        while j < n {
                            if mask[j] && bytes[j] == b'{' {
                                open = j;
                                break;
                            }
                            j += 1;
                        }
                        if open == 0 {
                            0
                        } else {
                            let mut depth = 1u32;
                            let mut close = 0usize;
                            let mut b = open + 1;
                            while b < n {
                                if mask[b] {
                                    if bytes[b] == b'{' {
                                        depth += 1;
                                    } else if bytes[b] == b'}' {
                                        depth -= 1;
                                        if depth == 0 {
                                            close = b;
                                            break;
                                        }
                                    }
                                }
                                b += 1;
                            }
                            close
                        }
                    };

                    if end > i {
                        definitions.push(source[i..=end].trim().to_string());
                        i = end + 1;
                        continue;
                    }
                }
            }
        }
        i += 1;
    }

    definitions
}

/// Extract every top-level (non-extern) `fn` definition from `source`,
/// skipping strings/comments and the template-provided `state` helper.  Used
/// to inject the project's helper functions into a standalone-compiled body.
fn extract_top_level_functions(source: &str) -> Vec<String> {
    let mask = code_mask(source);
    let bytes = source.as_bytes();
    let n = bytes.len();
    let mut functions = Vec::new();
    let mut i = 0usize;

    while i < n {
        // A real `fn` declaration starts a statement (preceded by a
        // delimiter), never the `fn` inside a type or expression.
        let prev_is_delimiter = i == 0
            || matches!(
                bytes[i - 1],
                b'\n' | b'\r' | b'\t' | b' ' | b';' | b'{' | b'}' | b']'
            );
        let is_word_start = prev_is_delimiter
            && mask[i]
            && bytes[i..].starts_with(b"fn ")
            && (i + 3 >= n || !(bytes[i + 3].is_ascii_alphanumeric() || bytes[i + 3] == b'_'));
        if is_word_start {
            // Skip extern functions (`pub extern "C" fn ...`): the declaration
            // text between the previous statement boundary and `fn` contains
            // the `extern` keyword.
            let mut decl_start = i;
            while decl_start > 0
                && !matches!(bytes[decl_start - 1], b'\n' | b'\r' | b';' | b'{' | b'}')
            {
                decl_start -= 1;
            }
            let declaration = &source[decl_start..i];
            let is_extern = declaration.split_whitespace().any(|word| word == "extern");

            if !is_extern {
                // Function name (after the whitespace following `fn`).
                let mut name_start = i + 2;
                while name_start < n
                    && (bytes[name_start] == b' '
                        || bytes[name_start] == b'\t'
                        || bytes[name_start] == b'\n')
                {
                    name_start += 1;
                }
                let mut name_end = name_start;
                while name_end < n
                    && (bytes[name_end].is_ascii_alphanumeric() || bytes[name_end] == b'_')
                {
                    name_end += 1;
                }
                let name = &source[name_start..name_end];

                // Skip `state` - the generated module defines its own.
                if name != "state" {
                    // Capture the full definition: `fn` keyword up to the
                    // matching closing brace.
                    let mut open = 0usize;
                    let mut j = i;
                    while j < n {
                        if mask[j] && bytes[j] == b'{' {
                            open = j;
                            break;
                        }
                        j += 1;
                    }
                    if open > i {
                        let mut depth = 1u32;
                        let mut close = 0usize;
                        let mut b = open + 1;
                        while b < n {
                            if mask[b] {
                                if bytes[b] == b'{' {
                                    depth += 1;
                                } else if bytes[b] == b'}' {
                                    depth -= 1;
                                    if depth == 0 {
                                        close = b;
                                        break;
                                    }
                                }
                            }
                            b += 1;
                        }
                        if close > open {
                            functions.push(source[i..=close].trim().to_string());
                            i = close + 1;
                            continue;
                        }
                    }
                }
            }
        }
        i += 1;
    }

    functions
}

/// Build a `mod modules { ... }` block for the leaf module by inlining the
/// project's `modules.rs` fan-out and every `modules/module_*.rs` file into a
/// single nested module.  The module files reference each other through
/// `crate::modules::module_NNN::...`, which resolves correctly because the
/// leaf crate root defines `mod modules` at the top level.
fn build_modules_block(project_sources: &[(PathBuf, String)]) -> String {
    let modules_rs = project_sources
        .iter()
        .find(|(path, _)| path.file_name().map_or(false, |name| name == "modules.rs"));
    let Some((_, modules_content)) = modules_rs else {
        return String::new();
    };

    // name -> content for every modules/module_NNN.rs file.
    let module_files: std::collections::HashMap<String, &str> = project_sources
        .iter()
        .filter(|(path, _)| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map_or(false, |name| name.starts_with("module_") && name.ends_with(".rs"))
        })
        .map(|(path, content)| {
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            (name, content.as_str())
        })
        .collect();

    let mut inlined = String::new();
    for line in modules_content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("pub mod ") {
            if let Some(module_name) = rest.strip_suffix(';') {
                if let Some(module_content) = module_files.get(&format!("{module_name}.rs")) {
                    inlined.push_str(&format!("pub mod {module_name} {{\n{module_content}\n}}\n"));
                    continue;
                }
            }
        }
        inlined.push_str(line);
        inlined.push('\n');
    }

    format!("mod modules {{\n{inlined}}}")
}

/// Build a self-contained Rust module for a standalone-compiled
/// `project_update`: lib.rs consts + types + helper functions, plus the
/// inlined module graph, plus the ABI mirrors and the new body.
fn build_patch_module_source(
    lib_source: &str,
    project_sources: &[(PathBuf, String)],
    params: &str,
    body: &str,
) -> String {
    let consts = extract_top_level_consts(lib_source).join("\n");
    let type_definitions = extract_top_level_type_definitions(lib_source).join("\n");
    let helper_functions = extract_top_level_functions(lib_source).join("\n");
    let modules_block = build_modules_block(project_sources);
    format!(
        "#![allow(dead_code)]\n\
         use std::ffi::c_void;\n\
         \n\
         {consts}\n\
         \n\
         {type_definitions}\n\
         \n\
         {helper_functions}\n\
         \n\
         {modules_block}\n\
         \n\
         #[repr(C)]\n\
         struct ProjectApi {{\n\
             get_state: extern \"C\" fn() -> *mut std::ffi::c_void,\n\
             screen_width: extern \"C\" fn() -> f32,\n\
             screen_height: extern \"C\" fn() -> f32,\n\
         }}\n\
         \n\
         #[repr(C)]\n\
         struct Quad {{\n\
             x: f32,\n\
             y: f32,\n\
             base_y: f32,\n\
             w: f32,\n\
             h: f32,\n\
             vx: f32,\n\
             jump_phase: f32,\n\
             jump_speed: f32,\n\
             jump_height: f32,\n\
             color: u32,\n\
         }}\n\
         \n\
         const MAX_QUADS: usize = 8;\n\
         \n\
         #[repr(C)]\n\
         struct GameState {{\n\
             tick: f32,\n\
             quad_count: usize,\n\
             quads: [Quad; MAX_QUADS],\n\
         }}\n\
         \n\
         static mut API: *const ProjectApi = std::ptr::null();\n\
         \n\
         #[no_mangle]\n\
         #[allow(private_interfaces)]\n\
         pub extern \"C\" fn project_set_api(api: *const ProjectApi) {{\n\
             unsafe {{ API = api; }}\n\
         }}\n\
         \n\
         fn state() -> &'static mut GameState {{\n\
             unsafe {{\n\
                 let api = API;\n\
                 assert!(!api.is_null(), \"project_set_api must be called before update\");\n\
                 let state_pointer = ((*api).get_state)() as *mut GameState;\n\
                 &mut *state_pointer\n\
             }}\n\
         }}\n\
         \n\
         #[no_mangle]\n\
         pub extern \"C\" fn project_update({params}) {{\n\
         {body}\n\
         }}\n"
    )
}

// ---------------------------------------------------------------------------
// Live-coding session
// ---------------------------------------------------------------------------

/// Build the whole project crate, load a fresh copy, hand it the API table,
/// run its init, and return (library, project_update address).
fn build_and_load_full(api: &ProjectApi) -> Result<(Library, usize), String> {
    let lib_path = build_project()?;
    let versioned = versioned_lib_path(lib_path.parent().unwrap());
    std::fs::copy(&lib_path, &versioned)
        .map_err(|e| format!("cannot copy to {}: {e}", versioned.display()))?;

    let library = unsafe { Library::new(&versioned) }
        .map_err(|e| format!("LoadLibrary failed: {e}"))?;

    // Hand the API table to the DLL, run its init, and copy out the update
    // address.  The Symbols borrow `library`, so they must be dropped before
    // `library` is moved into the result.
    let update_address: usize = unsafe {
        let set_api: Symbol<extern "C" fn(*const ProjectApi)> = library
            .get(b"project_set_api")
            .map_err(|e| format!("project_set_api missing: {e}"))?;
        set_api(api);

        let init: Symbol<extern "C" fn()> = library
            .get(b"project_init")
            .map_err(|e| format!("project_init missing: {e}"))?;
        init();

        let update: Symbol<ProjectUpdateFn> = library
            .get(b"project_update")
            .map_err(|e| format!("project_update missing: {e}"))?;
        *update as usize
    };

    Ok((library, update_address))
}

/// Owns the loaded project and applies live-coding changes.
struct ProjectSession {
    api: ProjectApi,
    /// Address of `project_update` in the base DLL - the prologue target that
    /// every patch rewrites.  Never changes after the initial load.
    base_update_address: Option<usize>,
    /// How many change events have been processed (for logging).
    change_count: u64,
    /// Cache of the last applied function bodies: (name, params, body, ret).
    cached_funcs: Vec<(String, String, String, String)>,
    /// lib.rs with extern bodies stripped, to detect non-body changes.
    stripped_source: String,
    /// Every loaded library (base copy, full builds, patch DLLs), retained so
    /// an in-flight call never hits unmapped memory.
    retained_libraries: Vec<Library>,
}

impl ProjectSession {
    fn new(api: ProjectApi) -> Self {
        Self {
            api,
            base_update_address: None,
            change_count: 0,
            cached_funcs: Vec::new(),
            stripped_source: String::new(),
            retained_libraries: Vec::new(),
        }
    }

    fn is_cached(&self, name: &str, body: &str) -> bool {
        self.cached_funcs
            .iter()
            .any(|(cached_name, _, cached_body, _)| cached_name == name && cached_body == body)
    }

    fn upsert_cached(&mut self, entry: (String, String, String, String)) {
        if let Some(slot) = self.cached_funcs.iter_mut().find(|cached| cached.0 == entry.0) {
            *slot = entry;
        } else {
            self.cached_funcs.push(entry);
        }
    }

    /// Initial build + load; point the engine at the base `project_update`.
    fn load_initial(&mut self, engine: &MiniEngine) {
        println!("  {}", paint_bright_cyan("Building project.dll (cold)..."));
        let build_start = Instant::now();
        match build_and_load_full(&self.api) {
            Ok((library, update_address)) => {
                self.base_update_address = Some(update_address);
                engine.set_project_update(update_address as *mut ());
                self.retained_libraries.push(library);

                // Seed the change cache from the current sources so the first
                // edit is diffed against the initial state.  The combined
                // source covers lib.rs + bloat + the module graph.
                let project_sources = collect_project_sources();
                let combined_source: String = project_sources
                    .iter()
                    .map(|(_, content)| content.as_str())
                    .collect::<Vec<_>>()
                    .join("\n");
                self.stripped_source = strip_extern_bodies(&combined_source);
                let funcs = extract_function_bodies(&combined_source);
                for entry in &funcs {
                    self.upsert_cached(entry.clone());
                }
                println!(
                    "{} cold build + load in {} - {} source file(s), {} extern function(s) cached",
                    hot_prefix(),
                    paint_green(&format!("{:?}", build_start.elapsed())),
                    paint_bright_cyan(&project_sources.len().to_string()),
                    paint_bright_cyan(&funcs.len().to_string())
                );

                println!(
                    "{} base project.dll loaded - jumping quads are live (project_update @ {})",
                    hot_prefix(),
                    paint_cyan(&format!("{update_address:#x}"))
                );
            }
            Err(errors) => eprintln!("{} initial build failed:\n{}", hot_prefix(), paint_red(&errors)),
        }
    }

    /// Compile `project_update`'s new body standalone and patch its prologue
    /// in the loaded base DLL - true Live Coding.
    fn try_leaf_patch(
        &mut self,
        lib_source: &str,
        project_sources: &[(PathBuf, String)],
        params: &str,
        body: &str,
    ) -> Result<(), String> {
        let consts = extract_top_level_consts(lib_source);
        println!(
            "    {} {}",
            paint("1;35", "--- leaf patch (true Live Coding) ---"),
            paint_magenta("per-function compile + prologue patch")
        );
        println!(
            "    injecting {} project const(s) into the self-contained module",
            paint_bright_cyan(&consts.len().to_string())
        );
        println!("    params=({params})  body={} chars", paint_cyan(&body.len().to_string()));

        let module_source = build_patch_module_source(lib_source, project_sources, params, body);
        let compile_start = Instant::now();
        println!("    rustc compiling standalone module...");
        let dll_path = mini_engine::patch::compile_rust_source(&module_source, "project_patch")?;
        println!(
            "    rustc OK in {} -> {}",
            paint_green(&format!("{:?}", compile_start.elapsed())),
            paint_cyan(&dll_path.display().to_string())
        );

        let library = unsafe { Library::new(&dll_path) }
            .map_err(|e| format!("LoadLibrary failed: {e}"))?;

        // Hand the API table to the patch DLL and copy out the update address
        // (the Symbol borrows `library`, so drop it before moving `library`).
        let new_address: usize = unsafe {
            let set_api: Symbol<extern "C" fn(*const ProjectApi)> = library
                .get(b"project_set_api")
                .map_err(|e| format!("project_set_api missing: {e}"))?;
            set_api(&self.api);

            let update: Symbol<ProjectUpdateFn> = library
                .get(b"project_update")
                .map_err(|e| format!("project_update missing: {e}"))?;
            *update as usize
        };

        let base = self.base_update_address.ok_or("no base project loaded")?;
        println!(
            "    base project_update @ {}  (pointer stays unchanged)",
            paint_cyan(&format!("{base:#x}"))
        );
        println!(
            "    new  project_update @ {}  (patch DLL)",
            paint_cyan(&format!("{new_address:#x}"))
        );
        println!("    patching base prologue...");
        unsafe { mini_engine::patch::patch_prologue(base, new_address)? };

        self.retained_libraries.push(library);
        println!(
            "{} {}",
            hot_prefix(),
            paint_bright_green(&format!(
                "PATCHED IN PLACE in {:?} - base DLL untouched, pointer unchanged",
                compile_start.elapsed()
            ))
        );
        Ok(())
    }

    /// Full rebuild + load alongside; re-patch the base prologue to the fresh
    /// code (or point the engine at it directly on the very first load).
    fn full_reload(&mut self, engine: &MiniEngine, reason: &str) -> bool {
        println!(
            "    {} (reason: {})",
            paint("1;33", "--- full rebuild + load-alongside ---"),
            paint_yellow(reason)
        );
        let build_start = Instant::now();
        println!("    cargo build -p project --target-dir target_reload ...");
        match build_and_load_full(&self.api) {
            Ok((library, update_address)) => {
                println!(
                    "    build + load OK in {}",
                    paint_green(&format!("{:?}", build_start.elapsed()))
                );
                println!(
                    "    fresh project_update @ {}",
                    paint_cyan(&format!("{update_address:#x}"))
                );
                match self.base_update_address {
                    Some(base) => {
                        println!(
                            "    re-patching base prologue ({}) -> ({})...",
                            paint_cyan(&format!("{base:#x}")),
                            paint_cyan(&format!("{update_address:#x}"))
                        );
                        match unsafe { mini_engine::patch::patch_prologue(base, update_address) } {
                            Ok(()) => {
                                println!(
                                    "    {}",
                                    paint_green("base prologue re-patched - pointer unchanged")
                                )
                            }
                            Err(e) => {
                                println!(
                                    "    {} - swapping engine pointer instead",
                                    paint_yellow(&format!("prologue patch failed ({e})"))
                                );
                                engine.set_project_update(update_address as *mut ());
                            }
                        }
                    }
                    None => {
                        println!(
                            "    {}",
                            paint_dim("no base yet - pointing the engine directly at the fresh DLL")
                        );
                        self.base_update_address = Some(update_address);
                        engine.set_project_update(update_address as *mut ());
                    }
                }
                self.retained_libraries.push(library);
                true
            }
            Err(errors) => {
                eprintln!(
                    "{} rebuild FAILED - keeping previous code running:\n{}",
                    hot_prefix(),
                    paint_red(&errors)
                );
                false
            }
        }
    }

    /// Called from the render loop when the watcher reports a change.
    fn process_change(&mut self, engine: &MiniEngine, changed_paths: &[PathBuf]) {
        self.change_count += 1;
        println!(
            "{}",
            paint(
                "1;93",
                &format!("\n[hot] ========== change #{} detected ==========", self.change_count)
            )
        );
        for path in changed_paths {
            println!("  {} {}", paint_cyan("file changed:"), path.display());
        }

        // Read the whole project (lib.rs + bloat + module graph) so edits in
        // any file are detected, and pull out lib.rs separately for the leaf
        // module's consts/types/helpers.
        let project_sources = collect_project_sources();
        if project_sources.is_empty() {
            eprintln!(
                "{} cannot read project sources under {}",
                hot_prefix(),
                paint_red(&project_src_dir().display().to_string())
            );
            return;
        }
        let combined_source: String = project_sources
            .iter()
            .map(|(_, content)| content.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        let lib_source = project_sources
            .iter()
            .find(|(path, _)| path.file_name().map_or(false, |name| name == "lib.rs"))
            .map(|(_, content)| content.clone())
            .unwrap_or_default();

        let funcs = extract_function_bodies(&combined_source);
        let stripped = strip_extern_bodies(&combined_source);
        let stripped_changed = stripped != self.stripped_source;
        self.stripped_source = stripped;

        // Per-function diff against the cache.
        println!("  {}", paint("1;34", "--- source analysis ---"));
        println!(
            "  {} source file(s), {} extern function(s) extracted:",
            paint_bright_cyan(&project_sources.len().to_string()),
            paint_bright_cyan(&funcs.len().to_string())
        );
        for entry in &funcs {
            let status_text = if self.is_cached(&entry.0, &entry.2) {
                "unchanged"
            } else {
                "CHANGED"
            };
            let padded = format!("{status_text:<9}");
            let status = if status_text == "CHANGED" {
                paint_bright_red(&padded)
            } else {
                paint_dim(&padded)
            };
            let signature = if entry.3.is_empty() {
                format!("({})", entry.1)
            } else {
                format!("({}) -> {}", entry.1, entry.3)
            };
            println!(
                "    {:<16} {} body={:>3} chars  sig={}",
                entry.0,
                status,
                entry.2.len(),
                signature
            );
        }
        let non_body_text = if stripped_changed { "CHANGED" } else { "unchanged" };
        println!(
            "  non-body content (structs / consts / static): {}",
            if stripped_changed {
                paint_bright_red(non_body_text)
            } else {
                paint_green(non_body_text)
            }
        );

        // Skip entirely when the save produced identical content (a no-op
        // touch / editor re-save) - nothing to patch or rebuild.
        let anything_changed = funcs
            .iter()
            .any(|entry| !self.is_cached(&entry.0, &entry.2))
            || stripped_changed;
        if !anything_changed {
            println!("  {}", paint("1;34", "--- decision ---"));
            println!(
                "  {}",
                paint_dim("nothing actually changed (identical to cache) - skipping")
            );
            return;
        }

        // Locate project_update and decide whether ONLY its body changed.
        let update_entry = funcs.iter().find(|entry| entry.0 == "project_update").cloned();
        let update_changed = update_entry
            .as_ref()
            .map(|entry| !self.is_cached(&entry.0, &entry.2))
            .unwrap_or(true);
        let signature_changed = match (
            update_entry.as_ref(),
            self.cached_funcs.iter().find(|cached| cached.0 == "project_update"),
        ) {
            (Some(new_entry), Some(old_entry)) => {
                new_entry.1 != old_entry.1 || new_entry.3 != old_entry.3
            }
            _ => false,
        };
        let other_changed = funcs
            .iter()
            .any(|entry| entry.0 != "project_update" && !self.is_cached(&entry.0, &entry.2));

        let leaf_eligible = update_changed
            && !other_changed
            && !stripped_changed
            && !signature_changed;

        println!("  {}", paint("1;34", "--- decision ---"));
        if leaf_eligible {
            println!(
                "  {} (only project_update body changed)",
                paint_bright_green("leaf-eligible = TRUE")
            );
        } else {
            let mut reasons = Vec::new();
            if !update_changed {
                reasons.push("project_update body unchanged");
            }
            if other_changed {
                reasons.push("another function changed");
            }
            if stripped_changed {
                reasons.push("non-body content changed (structs/consts/static)");
            }
            if signature_changed {
                reasons.push("project_update signature changed");
            }
            if update_entry.is_none() {
                reasons.push("project_update missing from source");
            }
            let reasons = if reasons.is_empty() {
                "no actionable change".to_string()
            } else {
                reasons.join("; ")
            };
            println!(
                "  {} ({reasons}) -> {}",
                paint_bright_yellow("leaf-eligible = FALSE"),
                paint_bright_yellow("full rebuild")
            );
        }

        // Fast path: only the project_update body changed - patch in place.
        if leaf_eligible {
            if let Some(entry) = &update_entry {
                match self.try_leaf_patch(&lib_source, &project_sources, &entry.1, &entry.2) {
                    Ok(()) => {
                        self.upsert_cached(entry.clone());
                        return;
                    }
                    Err(errors) => {
                        // e.g. the body references something the standalone
                        // module can't provide - fall back to a full build.
                        eprintln!(
                            "{} standalone leaf compile failed - falling back:\n{}",
                            hot_prefix(),
                            paint_red(&errors)
                        );
                    }
                }
            }
        }

        // Full-rebuild fallback (or the change wasn't leaf-eligible).
        if self.full_reload(engine, "leaf-ineligible or leaf patch failed") {
            for entry in &funcs {
                self.upsert_cached(entry.clone());
            }
        }
    }
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

#[macroquad::main("live-coding mini engine")]
async fn main() {
    let engine = Arc::new(MiniEngine::new());
    // Seed the API's screen-size cache now (main thread, window exists) so the
    // initial project_init sees it.
    engine.update_screen_size();

    let api = engine.project_api();
    let mut session = ProjectSession::new(api);

    // A notify watcher that pushes the changed file paths through a channel.
    // All patching happens on the main thread (drained from the channel), so
    // there is never a concurrent executor inside the function being patched.
    let (change_tx, change_rx) = std::sync::mpsc::channel::<Vec<PathBuf>>();
    let watch_dir = workspace_dir().join("project");
    let mut watcher = notify::recommended_watcher(
        move |result: Result<notify::Event, notify::Error>| {
            if let Ok(event) = result {
                let changed_paths: Vec<PathBuf> = event
                    .paths
                    .iter()
                    .filter(|path| {
                        path.extension().map_or(false, |ext| ext == "rs")
                            || path.file_name().map_or(false, |name| name == "Cargo.toml")
                    })
                    .cloned()
                    .collect();
                if !changed_paths.is_empty() {
                    let _ = change_tx.send(changed_paths);
                }
            }
        },
    )
    .expect("failed to create file watcher");
    watcher
        .configure(notify::Config::default().with_poll_interval(Duration::from_millis(500)))
        .expect("failed to configure file watcher");
    watcher
        .watch(&watch_dir, notify::RecursiveMode::Recursive)
        .expect("failed to watch project directory");
    println!(
        "{} watching {} for source changes...",
        paint_bright_cyan("[watch]"),
        watch_dir.display()
    );

    // Initial build + load of the base project.
    session.load_initial(&engine);

    // Render loop with a per-frame hook that drains change events and
    // live-patches (all on the main thread).
    engine
        .run(move |engine_ref, _delta_time| {
            if let Ok(changed_paths) = change_rx.try_recv() {
                // Merge any extra events from the same save burst.
                let mut all_paths = changed_paths;
                while let Ok(more_paths) = change_rx.try_recv() {
                    for path in more_paths {
                        if !all_paths.contains(&path) {
                            all_paths.push(path);
                        }
                    }
                }
                // Let the editor finish writing before reading the file.
                std::thread::sleep(Duration::from_millis(150));
                session.process_change(engine_ref, &all_paths);
            }
        })
        .await;
}
