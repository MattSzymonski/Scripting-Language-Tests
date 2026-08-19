// REQUIREMENTS
//   Rust (stable).  Run by Cargo as the `project` crate's build script.
//
// DESCRIPTION
//   Auto-generates the hot-symbol registry (`resolve_hot_symbol`) into OUT_DIR
//   by scanning project/src/**.  Any `pub fn` in a module file and the hot
//   `pub extern "C" fn`s in lib.rs are registered automatically, so no
//   function or module ever has to be declared by hand.  lib.rs pulls the
//   registry in with include!(concat!(env!("OUT_DIR"), "/hot_registry.rs")).
//
//   The registry is what the host's single exported `project_resolve_symbol`
//   consults to turn a function's fully-qualified path into its address inside
//   the DLL.  Keys are qualified (e.g. `modules::module_003::scale_value_003`),
//   so a lib helper and a module function sharing a bare name never collide.
//
// USAGE
//   cargo build -p project        # from live_coding_mini_engine/
//
// --- SCRIPT ---

use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let src_dir = manifest_dir.join("src");
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // Collect every .rs file under src/ (recursively), stable order.
    let mut files: Vec<PathBuf> = Vec::new();
    let mut stack = vec![src_dir.clone()];
    while let Some(directory) = stack.pop() {
        if let Ok(entries) = std::fs::read_dir(&directory) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().map_or(false, |ext| ext == "rs") {
                    files.push(path);
                }
            }
        }
    }
    files.sort();

    // Every hot function's fully-qualified Rust path (e.g. `run_update`,
    // `modules::module_003::scale_value_003`).  The resolver is keyed by these
    // qualified paths, so a lib helper named like a module function (lib
    // `scale_value_003` vs `modules::module_003::scale_value_003`) is a
    // DIFFERENT key - both can be hot, no renaming needed.
    let mut entries: Vec<String> = Vec::new();
    for file in &files {
        let content = match std::fs::read_to_string(file) {
            Ok(content) => content,
            Err(_) => continue,
        };
        // Re-run this script when any source file changes (edits).
        println!("cargo:rerun-if-changed={}", file.display());

        let is_lib_rs = file.file_name().map_or(false, |name| name == "lib.rs");
        // Module path prefix of this file, e.g. "modules::module_000".
        let scope = match file.strip_prefix(&src_dir) {
            Ok(relative) => relative
                .with_extension("")
                .to_string_lossy()
                .replace(['/', '\\'], "::"),
            Err(_) => continue,
        };

        let mask = code_mask(&content);
        let bytes = content.as_bytes();
        let n = bytes.len();

        // Collect every applicable function marker (start, marker_len).
        let mut markers: Vec<(usize, usize)> = Vec::new();
        if !is_lib_rs {
            // Module files: plain `pub fn` graph functions.
            let mut i = 0usize;
            while let Some(start) = find_pub_fn_marker(bytes, &mask, i) {
                markers.push((start, b"pub fn ".len()));
                let Some(open) = find_open_brace(bytes, &mask, start) else {
                    break;
                };
                let Some(close) = find_matching_close(bytes, &mask, open) else {
                    break;
                };
                i = close + 1;
            }
        } else {
            // lib.rs: exported hot fns (project_update) ...
            let mut i = 0usize;
            while let Some(start) = find_extern_marker(bytes, &mask, i) {
                markers.push((start, b"pub extern ".len() + b"\"C\" fn ".len()));
                let Some(open) = find_open_brace(bytes, &mask, start) else {
                    break;
                };
                let Some(close) = find_matching_close(bytes, &mask, open) else {
                    break;
                };
                i = close + 1;
            }
            // ... and private helper fns (get_aaa, scale_value_003, ...).
            let mut i = 0usize;
            while let Some(start) = find_private_fn_marker(bytes, &mask, i) {
                markers.push((start, b"fn ".len()));
                let Some(open) = find_open_brace(bytes, &mask, start) else {
                    break;
                };
                let Some(close) = find_matching_close(bytes, &mask, open) else {
                    break;
                };
                i = close + 1;
            }
        }
        markers.sort();

        for (start, marker_len) in markers {
            // Function name (up to the '(').
            let mut name_start = start + marker_len;
            while name_start < n && (bytes[name_start] == b' ' || bytes[name_start] == b'\t') {
                name_start += 1;
            }
            let mut name_end = name_start;
            while name_end < n && bytes[name_end] != b'(' {
                name_end += 1;
            }
            let name = content[name_start..name_end].trim().to_string();

            // Skip load-time plumbing and infrastructure helpers (the
            // registry, state(), spawn_default_quads).  No name-collision
            // check is needed: each entry is keyed by its qualified path,
            // which is unique by construction.
            let is_infra = name == "project_set_api"
                || name == "project_init"
                || name == "project_resolve_symbol"
                || name == "resolve_hot_symbol"
                || name == "state"
                || name == "spawn_default_quads";
            if !is_infra {
                let path = if is_lib_rs {
                    name.clone()
                } else {
                    format!("{scope}::{name}")
                };
                entries.push(path);
            }
        }
    }

    // Write the registry (crate root scope - lib.rs includes it).
    let mut output = String::from(
        "// Auto-generated by build.rs - do not edit.\n\
         // Maps every hot function's fully-qualified Rust path to its address\n\
         // in this DLL for the host's single exported `project_resolve_symbol`\n\
         // entry point.  Qualified keys mean a lib helper and a module\n\
         // function can share a name and both stay hot.\n\
         pub(crate) fn resolve_hot_symbol(name: &str) -> usize {\n\
         \x20   match name {\n",
    );
    for path in &entries {
        output.push_str(&format!(
            "        \"{path}\" => {path} as *const () as usize,\n"
        ));
    }
    output.push_str("        _ => 0,\n    }\n}\n");

    let out_file = out_dir.join("hot_registry.rs");
    std::fs::write(&out_file, output).expect("failed to write hot_registry.rs");
    // Also re-run when files are added/removed (directory mtime changes).
    println!("cargo:rerun-if-changed={}", src_dir.display());
}

// ---------------------------------------------------------------------------
// Mini-lexer: true where `source` is real code (not strings/comments).
// ---------------------------------------------------------------------------

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
                    while j < n && bytes[j] != b'\n' && bytes[j] != b' ' && bytes[j] != b'\t' {
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

/// Find `pub extern ` + `"C" fn ` at or after `from` (the quoted part is
/// matched verbatim - it is a string literal to the lexer).
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

/// Find plain `pub fn ` (NOT `pub(crate) fn` / `pub extern`) at or after
/// `from`, mask-aware.
fn find_pub_fn_marker(bytes: &[u8], mask: &[bool], from: usize) -> Option<usize> {
    let marker = b"pub fn ";
    let n = bytes.len();
    let mut i = from;
    while i + marker.len() <= n {
        if &bytes[i..i + marker.len()] == marker
            && mask[i..i + marker.len()].iter().all(|&is_code| is_code)
        {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Find a private `fn ` declaration (no `pub` / `extern` prefix, e.g.
/// `fn get_aaa`), mask-aware.  Used to register lib.rs helper functions.
fn find_private_fn_marker(bytes: &[u8], mask: &[bool], from: usize) -> Option<usize> {
    let marker = b"fn ";
    let n = bytes.len();
    let mut i = from;
    while i + marker.len() <= n {
        if &bytes[i..i + marker.len()] == marker
            && mask[i..i + marker.len()].iter().all(|&is_code| is_code)
        {
            let prev_ok = i == 0
                || matches!(
                    bytes[i - 1],
                    b'\n' | b'\r' | b'\t' | b' ' | b';' | b'{' | b'}' | b']'
                );
            if prev_ok {
                // The declaration before `fn` must not contain `pub`/`extern`
                // (that would be `pub fn`, `pub(crate) fn` or `extern "C" fn`).
                let mut decl_start = i;
                while decl_start > 0
                    && !matches!(bytes[decl_start - 1], b'\n' | b'\r' | b';' | b'{' | b'}')
                {
                    decl_start -= 1;
                }
                let declaration = String::from_utf8_lossy(&bytes[decl_start..i]);
                let words: Vec<&str> = declaration.split_whitespace().collect();
                let has_pub = words.iter().any(|word| word.contains("pub"));
                let has_extern = words.iter().any(|word| word.contains("extern"));
                if !has_pub && !has_extern {
                    return Some(i);
                }
            }
        }
        i += 1;
    }
    None
}

/// First `{` at or after `from` that is real code.
fn find_open_brace(bytes: &[u8], mask: &[bool], from: usize) -> Option<usize> {
    let mut k = from;
    while k < bytes.len() {
        if mask[k] && bytes[k] == b'{' {
            return Some(k);
        }
        k += 1;
    }
    None
}

/// Matching `}` for the brace at `open`.
fn find_matching_close(bytes: &[u8], mask: &[bool], open: usize) -> Option<usize> {
    let mut depth = 1u32;
    let mut b = open + 1;
    while b < bytes.len() {
        if mask[b] {
            if bytes[b] == b'{' {
                depth += 1;
            } else if bytes[b] == b'}' {
                depth -= 1;
                if depth == 0 {
                    return Some(b);
                }
            }
        }
        b += 1;
    }
    None
}
