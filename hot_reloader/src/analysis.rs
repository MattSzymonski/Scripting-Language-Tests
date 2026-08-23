//! Mini-lexer + source extractors for change detection and patch-module
//! generation.
//!
//! The lexer produces a mask (`true` = real code, `false` = inside a string,
//! char literal or comment); every extractor and the hot-body stripper only
//! look at masked-in positions, so braces/`fn`/`const` inside strings or
//! comments can never confuse them.

// ---------------------------------------------------------------------------
// Lexical mask
// ---------------------------------------------------------------------------

/// Lexical mask: `true` where `source` is real code, `false` inside strings,
/// char literals, or comments.  Brace counting and marker searches only look
/// at code positions, so braces inside strings/comments can never confuse the
/// extractor.
pub fn code_mask(source: &str) -> Vec<bool> {
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

// ---------------------------------------------------------------------------
// Function markers
// ---------------------------------------------------------------------------

/// Find `pub extern "C" fn ` at or after `from`.  The leading `pub extern `
/// must be real code; the quoted `"C"` part is lexed as a string literal, so
/// it is matched verbatim rather than via the code mask.
pub fn find_extern_marker(bytes: &[u8], mask: &[bool], from: usize) -> Option<usize> {
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

/// Find `pub fn ` (a plain public function, NOT `pub extern`) at or after
/// `from`, mask-aware.  `pub(crate) fn` and `pub extern "C" fn` do not match
/// the literal `pub fn ` marker.
pub fn find_pub_fn_marker(bytes: &[u8], mask: &[bool], from: usize) -> Option<usize> {
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
/// `fn get_aaa`), mask-aware.  Used to make lib.rs helper functions hot.
pub fn find_private_fn_marker(bytes: &[u8], mask: &[bool], from: usize) -> Option<usize> {
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
                let mut words = declaration.split_whitespace();
                let has_pub = words.clone().any(|word| word.contains("pub"));
                let has_extern = words.any(|word| word.contains("extern"));
                if !has_pub && !has_extern {
                    return Some(i);
                }
            }
        }
        i += 1;
    }
    None
}

// ---------------------------------------------------------------------------
// Function body extractors
// ---------------------------------------------------------------------------

/// Lexically-aware extractor: `pub extern "C" fn NAME(params) [-> Ret] { body }`.
/// Returns `Vec<(name, params, body, return_type)>`.
pub fn extract_function_bodies(source: &str) -> Vec<(String, String, String, String)> {
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

/// Lexically-aware extractor for plain `pub fn NAME(params) [-> Ret] { body }`
/// (the module-graph hot functions, which are NOT exported - the host finds
/// them through `project_resolve_symbol` instead).  Returns
/// `Vec<(name, params, body, return_type)>`.
pub fn extract_public_functions(source: &str) -> Vec<(String, String, String, String)> {
    let mask = code_mask(source);
    let bytes = source.as_bytes();
    let marker_total = b"pub fn ".len();
    let mut results = Vec::new();
    let mut i = 0usize;
    let n = bytes.len();

    while let Some(start) = find_pub_fn_marker(bytes, &mask, i) {
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

/// Lexically-aware extractor for private lib.rs helpers
/// (`fn NAME(params) [-> Ret] { body }`, no `pub`/`extern`) - these are hot
/// too, so editing them can be per-function patched.  Returns
/// `Vec<(name, params, body, return_type)>`.
pub fn extract_private_functions(source: &str) -> Vec<(String, String, String, String)> {
    let mask = code_mask(source);
    let bytes = source.as_bytes();
    let marker_total = b"fn ".len();
    let mut results = Vec::new();
    let mut i = 0usize;
    let n = bytes.len();

    while let Some(start) = find_private_fn_marker(bytes, &mask, i) {
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

// ---------------------------------------------------------------------------
// Hot-body stripping (change detection)
// ---------------------------------------------------------------------------

/// Strips the bodies of every hot function (extern, `pub fn` and hot private
/// helpers), so a hot-function body edit counts as a "body-only" change.  The
/// `hot_names` set decides which private lib.rs helpers are hot; non-hot
/// infrastructure (`fn state`, `fn spawn_default_quads`) and `pub(crate)`
/// items keep their bodies, so editing them is correctly treated as a full
/// rebuild.
pub fn strip_hot_bodies(source: &str, hot_names: &std::collections::HashSet<String>) -> String {
    #[derive(Clone, Copy)]
    enum MarkerKind {
        /// `pub extern "C" fn` / `pub fn`: bodies always stripped.
        Always,
        /// private lib helper `fn`: strip only if its name is hot.
        Private,
    }

    let mask = code_mask(source);
    let bytes = source.as_bytes();
    let mut result = String::new();
    let mut i = 0usize;
    let n = bytes.len();

    loop {
        // Find the earliest hot-function marker of any kind.
        let next_extern = find_extern_marker(bytes, &mask, i).map(|s| (s, MarkerKind::Always));
        let next_pub = find_pub_fn_marker(bytes, &mask, i).map(|s| (s, MarkerKind::Always));
        let next_private =
            find_private_fn_marker(bytes, &mask, i).map(|s| (s, MarkerKind::Private));

        let (start, kind) = match (next_extern, next_pub, next_private) {
            (Some(a), Some(b), Some(c)) => {
                let (s, k) = if a.0 <= b.0 && a.0 <= c.0 {
                    a
                } else if b.0 <= c.0 {
                    b
                } else {
                    c
                };
                (s, k)
            }
            (Some(a), Some(b), None) => {
                if a.0 <= b.0 {
                    a
                } else {
                    b
                }
            }
            (Some(a), None, Some(c)) => {
                if a.0 <= c.0 {
                    a
                } else {
                    c
                }
            }
            (None, Some(b), Some(c)) => {
                if b.0 <= c.0 {
                    b
                } else {
                    c
                }
            }
            (Some(a), None, None) => a,
            (None, Some(b), None) => b,
            (None, None, Some(c)) => c,
            (None, None, None) => {
                result.push_str(&source[i..]);
                return result;
            }
        };

        // For a private helper, decide strip (hot at root) vs keep (non-hot
        // infrastructure).  `hot_names` here carries only root-scope hot
        // names, so every hot lib helper (including one that shares a bare
        // name with a module function) is stripped and stays hot-patchable.
        let mut should_strip = true;
        if matches!(kind, MarkerKind::Private) {
            let marker_len = b"fn ".len();
            let mut name_start = start + marker_len;
            while name_start < n && (bytes[name_start] == b' ' || bytes[name_start] == b'\t') {
                name_start += 1;
            }
            let mut name_end = name_start;
            while name_end < n && bytes[name_end] != b'(' {
                name_end += 1;
            }
            let name = &source[name_start..name_end];
            should_strip = hot_names.contains(name);
        }

        result.push_str(&source[i..start]);

        let mut open: Option<usize> = None;
        let mut k = start;
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
        let Some(close) = close else {
            result.push_str(&source[start..]);
            return result;
        };

        if should_strip {
            // Strip the body: keep the signature + opening brace.
            result.push_str(&source[start..=open]);
        } else {
            // Infrastructure helper (or pub(crate) bloat): keep it whole.
            result.push_str(&source[start..=close]);
        }
        i = close + 1;
    }
}

// ---------------------------------------------------------------------------
// Top-level consts / types / helpers (patch-module injection)
// ---------------------------------------------------------------------------

/// Extract every top-level `const` declaration from `source` as trimmed
/// declaration lines (skipping strings/comments).  Used to inject the
/// project's current constant values into a standalone-compiled function.
pub fn extract_top_level_consts(source: &str) -> Vec<String> {
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
            // Name (after the whitespace following `const`).
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

            // A real const declaration has `NAME:` after the name; the `const`
            // in `*const T` does not.
            let mut colon_pos = name_end;
            while colon_pos < n && (bytes[colon_pos] == b' ' || bytes[colon_pos] == b'\t') {
                colon_pos += 1;
            }
            let is_declaration = colon_pos < n && bytes[colon_pos] == b':';

            if is_declaration {
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

/// Extract the project's top-level `struct` / `enum` / `type` definitions
/// (skipping strings/comments), so a standalone-compiled body can construct
/// and use project-local types.  Every definition is injected verbatim -
/// including its leading attributes (`#[repr(C)]`, ...) and the project's own
/// ABI mirrors (e.g. `ProjectApi`) - so the patch module is a faithful copy
/// of the project's type layout (a mirror without `#[repr(C)]` would get
/// Rust's unspecified default layout and misread the engine's memory).
pub fn extract_top_level_type_definitions(source: &str) -> Vec<String> {
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
                let item_start = attribute_block_start(source, bytes, &mask, i);
                {
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
                        definitions.push(source[item_start..=end].trim().to_string());
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
/// skipping strings/comments.  Used to inject the project's helper functions
/// (including its state accessor - e.g. `state()`) into the standalone-
/// compiled patch module, so the module is a faithful copy of the project.
/// `pub extern` functions are handled separately by
/// [`extract_extern_function_definitions`].
pub fn extract_top_level_functions(source: &str) -> Vec<String> {
    let mask = code_mask(source);
    let bytes = source.as_bytes();
    let n = bytes.len();
    let mut functions = Vec::new();
    let mut i = 0usize;

    while i < n {
        // A real `fn` declaration starts a statement (preceded by a
        // delimiter), never the `fn` inside a type or expression.  The marker
        // `fn ` already includes the trailing space, so `bytes[i + 3]` is the
        // first letter of the function name - no word-boundary check needed
        // (and one would wrongly reject every real function name).
        let prev_is_delimiter = i == 0
            || matches!(
                bytes[i - 1],
                b'\n' | b'\r' | b'\t' | b' ' | b';' | b'{' | b'}' | b']'
            );
        let is_word_start = prev_is_delimiter && mask[i] && bytes[i..].starts_with(b"fn ");
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
                let item_start = attribute_block_start(source, bytes, &mask, i);
                {
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
                            functions.push(source[item_start..=close].trim().to_string());
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

/// Extract the project's top-level `use` statements (depth-0 imports), so a
/// standalone-compiled body keeps the imports the project's helpers rely on
/// (e.g. `use std::ffi::c_void;`).  `pub use` is captured from the `pub`.
/// Function-local `use` (inside bodies) is ignored.
pub fn extract_top_level_imports(source: &str) -> Vec<String> {
    let mask = code_mask(source);
    let bytes = source.as_bytes();
    let n = bytes.len();
    let mut imports = Vec::new();
    let mut depth = 0u32;
    let mut i = 0usize;

    while i < n {
        if mask[i] {
            match bytes[i] {
                b'{' => depth += 1,
                b'}' => depth = depth.saturating_sub(1),
                _ => {}
            }
            let prev_is_delimiter = i == 0
                || matches!(
                    bytes[i - 1],
                    b'\n' | b'\r' | b'\t' | b' ' | b';' | b'{' | b'}' | b']'
                );
            if depth == 0
                && prev_is_delimiter
                && bytes[i..].starts_with(b"use")
                && (i + 3 >= n || matches!(bytes[i + 3], b' ' | b'\t' | b'\n' | b'\r'))
            {
                // Capture `pub use` from the `pub` too.
                let mut statement_start = i;
                while statement_start > 0
                    && !matches!(bytes[statement_start - 1], b'\n' | b'\r' | b';' | b'{' | b'}')
                {
                    statement_start -= 1;
                }
                let prefix = &source[statement_start..i];
                let start = if prefix.split_whitespace().last() == Some("pub") {
                    statement_start
                } else {
                    i
                };

                let mut j = i;
                let mut round = 0u32;
                let mut square = 0u32;
                while j < n {
                    if mask[j] {
                        match bytes[j] {
                            b'(' => round += 1,
                            b')' => round = round.saturating_sub(1),
                            b'[' => square += 1,
                            b']' => square = square.saturating_sub(1),
                            b';' if round == 0 && square == 0 => {
                                imports.push(source[start..=j].trim().to_string());
                                i = j + 1;
                                break;
                            }
                            _ => {}
                        }
                    }
                    j += 1;
                }
                continue;
            }
        }
        i += 1;
    }

    imports
}

/// Extract the project's top-level `static` / `static mut` declarations
/// (module level only), so the standalone-compiled patch module can keep the
/// project's own state pointer (e.g. `static mut API`).  Function-local
/// statics (inside `fn` bodies) are ignored.
pub fn extract_top_level_statics(source: &str) -> Vec<String> {
    let mask = code_mask(source);
    let bytes = source.as_bytes();
    let n = bytes.len();
    let mut statics = Vec::new();
    let mut depth = 0u32;
    let mut i = 0usize;

    while i < n {
        if mask[i] {
            match bytes[i] {
                b'{' => depth += 1,
                b'}' => depth = depth.saturating_sub(1),
                _ => {}
            }
            let prev_is_delimiter = i == 0
                || matches!(
                    bytes[i - 1],
                    b'\n' | b'\r' | b'\t' | b' ' | b';' | b'{' | b'}' | b']'
                );
            if depth == 0
                && prev_is_delimiter
                && bytes[i..].starts_with(b"static")
                && (i + 6 >= n
                    || !(bytes[i + 6].is_ascii_alphanumeric() || bytes[i + 6] == b'_'))
            {
                let mut j = i;
                let mut round = 0u32;
                let mut square = 0u32;
                while j < n {
                    if mask[j] {
                        match bytes[j] {
                            b'(' => round += 1,
                            b')' => round = round.saturating_sub(1),
                            b'[' => square += 1,
                            b']' => square = square.saturating_sub(1),
                            b';' if round == 0 && square == 0 => {
                                statics.push(source[i..=j].trim().to_string());
                                i = j + 1;
                                break;
                            }
                            _ => {}
                        }
                    }
                    j += 1;
                }
                continue;
            }
        }
        i += 1;
    }

    statics
}

/// Walk backward from a `pub extern` marker over its leading attribute block,
/// returning the byte index where the attributes begin.  Handles single-line
/// `#[...]` attributes and multi-line `#[cfg(...)]`-style attributes whose
/// brackets span several lines (string/comment-masked).  Doc comments (`///`)
/// above the attributes are not part of the definition and are left out.
fn attribute_block_start(source: &str, bytes: &[u8], mask: &[bool], marker_start: usize) -> usize {
    // Does the marker's own line carry an attribute before `pub` (e.g.
    // `#[no_mangle] pub extern "C" fn ...`)?  If so the whole line belongs.
    let mut marker_line_start = marker_start;
    while marker_line_start > 0 && bytes[marker_line_start - 1] != b'\n' {
        marker_line_start -= 1;
    }
    if source[marker_line_start..marker_start].trim_start().starts_with('#') {
        return marker_line_start;
    }

    // Otherwise walk up from the line ABOVE the marker.
    let mut item_start = marker_line_start;
    // Running `[` - `]` balance of the attribute text consumed so far (from
    // the bottom up).  Nonzero means an attribute opened above and is still
    // continuing, so more lines above belong to the block.
    let mut balance = 0isize;
    loop {
        // `item_start` sits right after the `\n` ending the line above the
        // one just consumed; step over it so the examined slice is that line.
        if item_start == 0 {
            break;
        }
        if bytes[item_start - 1] == b'\n' {
            item_start -= 1;
        }
        let mut line_start = item_start;
        while line_start > 0 && bytes[line_start - 1] != b'\n' {
            line_start -= 1;
        }
        let trimmed = source[line_start..=item_start].trim();
        if trimmed.is_empty() {
            break;
        }
        let opens = mask[line_start..=item_start]
            .iter()
            .zip(&bytes[line_start..=item_start])
            .filter(|(is_code, byte)| **is_code && **byte == b'[')
            .count();
        let closes = mask[line_start..=item_start]
            .iter()
            .zip(&bytes[line_start..=item_start])
            .filter(|(is_code, byte)| **is_code && **byte == b']')
            .count();
        // A line belongs to the attribute block when it opens one (`#[...`),
        // continues one opened above (unbalanced), or closes one opened above
        // (more `]` than `[` - e.g. the `))]` tail of a multi-line attribute).
        let is_attribute_line = trimmed.starts_with('#') || balance != 0 || closes > opens;
        if !is_attribute_line {
            break;
        }
        balance += opens as isize - closes as isize;
        item_start = line_start;
    }
    item_start
}

/// Extract every `pub extern "C" fn` definition from `source` as
/// `(name, full_definition)`, INCLUDING the leading attribute block
/// (`#[no_mangle]`, `#[allow(...)]`), through the matching closing brace.
/// Used to inject the project's own plumbing exports (e.g. `project_set_api`)
/// into the patch module verbatim, so the module's `set_api` always matches
/// the project's actual signature (whatever the project calls its API type).
pub fn extract_extern_function_definitions(source: &str) -> Vec<(String, String)> {
    let mask = code_mask(source);
    let bytes = source.as_bytes();
    let marker_total = b"pub extern ".len() + b"\"C\" fn ".len();
    let n = bytes.len();
    let mut results = Vec::new();
    let mut i = 0usize;

    while let Some(start) = find_extern_marker(bytes, &mask, i) {
        let name_start = start + marker_total;
        let mut name_end = name_start;
        while name_end < n && bytes[name_end] != b'(' {
            name_end += 1;
        }
        let name = source[name_start..name_end].trim().to_string();

        // Locate the body and its matching close, exactly like
        // `extract_function_bodies`.
        let mut body_open = 0usize;
        let mut k = start;
        while k < n {
            if mask[k] && bytes[k] == b'{' {
                body_open = k;
                break;
            }
            k += 1;
        }
        if body_open == 0 {
            break;
        }
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
        if body_end == 0 {
            break;
        }

        let item_start = attribute_block_start(source, bytes, &mask, start);
        results.push((name, source[item_start..=body_end].trim().to_string()));
        i = body_end + 1;
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn code_mask_ignores_braces_inside_strings_and_comments() {
        // The `{` and `}` inside the string/comment must be masked out.
        let source = "fn f() { // { not code\n    let s = \"{ not code }\";\n    { 1 }\n}";
        let mask = code_mask(source);
        // Only the real block `{ 1 }` and the fn body braces count as code:
        // fn f() { ... { 1 } } = 4 real braces; the two inside the comment
        // and string are masked.
        let mut braces: Vec<usize> = Vec::new();
        for (index, is_code) in mask.iter().enumerate() {
            if *is_code
                && (source.as_bytes()[index] == b'{' || source.as_bytes()[index] == b'}')
            {
                braces.push(index);
            }
        }
        assert_eq!(braces.len(), 4, "only the real braces should be code");
    }

    #[test]
    fn extract_function_bodies_returns_name_params_body_ret() {
        let source = r#"
            pub extern "C" fn project_update(delta_time: f32) -> f32 {
                let x = delta_time * 2.0;
                x
            }
        "#;
        let extracted = extract_function_bodies(source);
        assert_eq!(extracted.len(), 1);
        let (name, params, body, ret) = &extracted[0];
        assert_eq!(name, "project_update");
        assert_eq!(params, "delta_time: f32");
        assert_eq!(ret, "f32");
        assert!(body.contains("delta_time * 2.0"));
    }

    #[test]
    fn extract_public_functions_ignores_privates_and_externs() {
        let source = r#"
            pub fn tick_000(input: i32) -> i32 { input + 1 }
            fn private_helper() {}
            pub extern "C" fn exported() {}
            pub(crate) fn crate_only() {}
        "#;
        let names: Vec<String> = extract_public_functions(source)
            .into_iter()
            .map(|(n, _, _, _)| n)
            .collect();
        assert_eq!(names, vec!["tick_000"]);
    }

    #[test]
    fn extract_private_functions_finds_only_plain_fns() {
        let source = r#"
            fn get_aaa() -> i32 { 3 }
            pub fn public_one() {}
            pub extern "C" fn exported() {}
        "#;
        let names: Vec<String> = extract_private_functions(source)
            .into_iter()
            .map(|(n, _, _, _)| n)
            .collect();
        assert_eq!(names, vec!["get_aaa"]);
    }

    #[test]
    fn strip_hot_bodies_strips_hot_but_keeps_non_hot_helpers() {
        let source = "fn hot_helper() -> i32 {\n    5\n}\n\nfn cold_helper() -> i32 {\n    9\n}\n";
        let hot: HashSet<String> = ["hot_helper".to_string()].into_iter().collect();
        let stripped = strip_hot_bodies(source, &hot);
        // hot helper keeps signature + opening brace; cold helper kept whole.
        assert!(stripped.contains("fn hot_helper() -> i32 {"));
        assert!(!stripped.contains("    5\n"));
        assert!(stripped.contains("    9\n"));
    }

    #[test]
    fn strip_hot_bodies_strips_pub_fn_always() {
        let source = "pub fn tick_000(input: i32) -> i32 {\n    input + 1\n}\n";
        let hot = HashSet::new();
        let stripped = strip_hot_bodies(source, &hot);
        assert!(stripped.contains("pub fn tick_000(input: i32) -> i32 {"));
        assert!(!stripped.contains("input + 1"));
    }

    #[test]
    fn extract_top_level_consts_includes_max_quads_and_skips_pointer_const() {
        let source = r#"
            const MAX_QUADS: usize = 8;
            const QUAD_SIZE: f32 = 46.0;
            let ptr: *const u8 = std::ptr::null();
        "#;
        let consts = extract_top_level_consts(source);
        let text = consts.join("\n");
        assert!(text.contains("QUAD_SIZE"));
        // MAX_QUADS is a real declaration and is injected verbatim now.
        assert!(text.contains("MAX_QUADS"));
        // The `const` inside `*const u8` is not a declaration.
        assert!(!text.contains("let ptr"));
    }

    #[test]
    fn extract_top_level_functions_includes_helpers_and_skips_externs() {
        let source = r#"
            fn state() {}
            fn get_aaa() -> i32 { 3 }
            pub extern "C" fn project_update() {}
        "#;
        let functions = extract_top_level_functions(source);
        let names: Vec<&str> = functions
            .iter()
            .map(|definition| {
                definition
                    .strip_prefix("fn ")
                    .and_then(|rest| rest.split(['(', ' ', '\n', '\t']).next())
                    .unwrap_or("")
            })
            .collect();
        // `state` is a project helper and is injected like any other now.
        assert_eq!(names, vec!["state", "get_aaa"]);
    }

    #[test]
    fn extract_top_level_statics_returns_module_level_only() {
        let source = r#"
            static mut API: *const u8 = std::ptr::null();
            fn helper() {
                static LOCAL: i32 = 3;
                LOCAL
            }
        "#;
        let statics = extract_top_level_statics(source);
        assert_eq!(statics.len(), 1);
        assert!(statics[0].contains("static mut API"));
        assert!(!statics[0].contains("LOCAL"));
    }

    #[test]
    fn extract_top_level_imports_returns_use_statements() {
        let source = r#"
            use std::ffi::c_void;
            pub use std::collections::HashMap;
            fn helper() {
                use std::fmt;
                fmt::format
            }
        "#;
        let imports = extract_top_level_imports(source);
        assert_eq!(imports.len(), 2);
        assert!(imports.iter().any(|i| i.starts_with("use std::ffi::c_void")));
        assert!(imports.iter().any(|i| i.starts_with("pub use")));
    }

    #[test]
    fn extract_extern_function_definitions_include_attributes() {
        let source = r#"
            /// doc comment is not part of the definition
            #[unsafe(no_mangle)]
            #[allow(private_interfaces)]
            pub extern "C" fn project_set_api(api: *const u8) {
                unsafe { let _ = api; }
            }
            #[no_mangle]
            pub extern "C" fn project_init() {}
        "#;
        let definitions = extract_extern_function_definitions(source);
        assert_eq!(definitions.len(), 2);
        let (name, text) = &definitions[0];
        assert_eq!(name, "project_set_api");
        assert!(text.starts_with("#[unsafe(no_mangle)]"));
        assert!(text.contains("#[allow(private_interfaces)]"));
        assert!(text.contains("pub extern \"C\" fn project_set_api"));
        assert!(!text.contains("/// doc comment"));
    }

    #[test]
    fn extract_top_level_type_definitions_include_attributes() {
        let source = r#"
            /// doc comment is not part of the definition
            #[repr(C)]
            struct GameState {
                tick: f32,
                quads: [u8; 8],
            }
            enum Simple { A, B }
        "#;
        let definitions = extract_top_level_type_definitions(source);
        assert_eq!(definitions.len(), 2);
        // `#[repr(C)]` must survive so the injected ABI mirror keeps the
        // engine-compatible layout (repr(Rust) layout is unspecified).
        assert!(definitions[0].starts_with("#[repr(C)]"));
        assert!(definitions[0].contains("struct GameState"));
        assert!(definitions[1].starts_with("enum Simple"));
    }
}
