// dll_analysis — PE header parsing and .o relocation diagnostics.
//
// Used by the hot-reload host to inspect DLLs and object files without
// reading entire files: seek-based PE parsing and llvm-objdump integration.
//
// The core insight driving this module:
// ─────────────────────────────────────
// The cost of hot-reloading a DLL is dominated by the linker, not the compiler.
// The linker must resolve every relocation in every .o file — scanning a table
// of symbols and patching addresses.  When a crate has 5,000 exported functions
// (like the bloat_gen module in game_lib), the linker must process over 5,000
// base relocations on every rebuild, even if you only changed one line.
//
// This module exposes three key metrics that quantify the linking bottleneck:
//   1. .text (code) section size   — how much compiled code the linker emitted
//   2. Base relocation count       — how many address fixups the OS loader
//                                    must perform at DLL load time
//   3. .o crate-internal links     — how many cross-function calls within your
//                                    crate the linker must resolve; each one
//                                    is a symbol lookup and address patch
//
// By showing *which* functions call *which* other functions (source→target),
// you can see at a glance whether a small edit triggers re-linking of 2 or
// 2,000 internal call sites.

use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

// --- PE/COFF header layout ---
const HEADER_BUFFER_SIZE: usize = 2048;
const MINIMUM_PE_HEADER_SIZE: usize = 64;
const DOS_ELFANEW_OFFSET: usize = 0x3C;
const PE_SIGNATURE_SIZE: usize = 4;

// COFF header field offsets (within the 20-byte COFF header).
const COFF_HEADER_SIZE: usize = 20;
const COFF_NUMBER_OF_SECTIONS_OFFSET: usize = 2;
const COFF_SIZE_OF_OPTIONAL_HEADER_OFFSET: usize = 16;

// Optional header magic values.
const PE32_MAGIC: u16 = 0x10B;
const PE32_PLUS_MAGIC: u16 = 0x20B;

// Data directory offset within the optional header (varies by PE format).
const PE32_DATA_DIRECTORY_OFFSET: usize = 96;
const PE32_PLUS_DATA_DIRECTORY_OFFSET: usize = 112;

// Data directory entry indices.
const EXPORT_DATA_DIRECTORY_INDEX: usize = 0;
const BASE_RELOCATION_DATA_DIRECTORY_INDEX: usize = 5;

// Each data directory entry is 8 bytes: {RVA: u32, Size: u32}.
const DATA_DIRECTORY_ENTRY_SIZE: usize = 8;

// --- Section table ---
const SECTION_ENTRY_SIZE: usize = 40;
const SECTION_NAME_MAX_LENGTH: usize = 8;
// Offsets within a 40-byte section table entry.
const SECTION_VIRTUAL_SIZE_OFFSET: usize = 8;
const SECTION_VIRTUAL_ADDRESS_OFFSET: usize = 12;
const SECTION_SIZE_OF_RAW_DATA_OFFSET: usize = 16;
const SECTION_POINTER_TO_RAW_DATA_OFFSET: usize = 20;

// --- Export directory (IMAGE_EXPORT_DIRECTORY, 40 bytes) ---
const EXPORT_DIRECTORY_SIZE: usize = 40;
const EXPORT_NUMBER_OF_NAMES_OFFSET: usize = 24;
const EXPORT_ADDRESS_OF_NAMES_OFFSET: usize = 32;
const EXPORT_NAME_BUFFER_SIZE: usize = 256;

// --- Display ---
const BYTES_PER_KILOBYTE: f64 = 1024.0;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Parsed information from a PE/DLL file.
///
/// Extracted via seek-based reads of the PE headers only — the full DLL
/// (which can be >100 KB for a debug build with many exports) is never
/// fully loaded into memory.
pub struct PeInfo {
    /// Names of exported symbols (e.g. "update", "compute", "export_00001").
    pub exports: Vec<String>,
    /// Size of the .text (code) section in KiB, if present.
    /// This is where all compiled functions live — its size grows linearly
    /// with the number of functions in the crate.
    pub text_section_kb: Option<f64>,
    /// Number of base relocations (.reloc section entries).
    /// Each relocation is a 16-bit entry in a relocation block; the OS loader
    /// must walk every entry at DLL load time to fix up absolute addresses
    /// to match the DLL's actual load address (ASLR).
    pub relocation_count: usize,
}

/// Inspect a DLL and return a human-readable diagnostics string.
///
/// Composes two independent diagnostic passes:
///   1. PE header analysis — size, .text size, relocation count, export names
///   2. .o relocation breakdown — which internal functions call each other
///
/// The combined output helps answer: "how much work did the linker just do?"
pub fn inspect_dll(dll_path: &Path, target_dir: &Path) -> String {
    let mut lines: Vec<String> = Vec::new();

    // File size from metadata — instant, no I/O beyond the stat call.
    let size_kb = match std::fs::metadata(dll_path) {
        Ok(m) => m.len() as f64 / BYTES_PER_KILOBYTE,
        Err(e) => {
            return format!("  (cannot stat DLL: {e})");
        }
    };
    lines.push(format!("  DLL size:      {size_kb:.1} KB"));

    // Open the file for seek-based PE header reading.
    // We only read ~2 KB of headers, not the full multi-KB DLL body.
    let mut file = match std::fs::File::open(dll_path) {
        Ok(f) => f,
        Err(e) => return format!("  (cannot open DLL: {e})"),
    };

    match parse_pe_info_seek(&mut file) {
        Ok(info) => {
            if let Some(text_kb) = info.text_section_kb {
                lines.push(format!("  .text (code):  {text_kb:.1} KB"));
            }
            // Base relocations are the primary cost at OS load time.
            // More exports → more relocations → slower LoadLibraryW.
            lines.push(format!("  relocations:   {}", info.relocation_count));
            lines.push(format!("  exports:       {}", info.exports.len()));
            // For crates with many exports (e.g. 5,000+ bloat functions), only
            // show a sample to avoid flooding the terminal.
            if info.exports.len() > 10 {
                lines.push("  Sample exports:".into());
                for name in info.exports.iter().take(3) {
                    lines.push(format!("    - {name}"));
                }
                if info.exports.len() > 6 {
                    lines.push("    ...".into());
                    // Show the last 3 to see the end of the enumeration.
                    for name in info.exports.iter().rev().take(3).rev() {
                        lines.push(format!("    - {name}"));
                    }
                }
            }
        }
        Err(e) => {
            lines.push(format!("  (PE parse error: {e})"));
        }
    }

    // .o relocation analysis — the key insight for hot-reload performance.
    // Shows how many function calls within your crate the linker must resolve,
    // and which specific functions are involved.
    lines.push(inspect_object_relocations(target_dir));

    lines.join("\n")
}

// ---------------------------------------------------------------------------
// Object-file relocation analysis
// ---------------------------------------------------------------------------

/// Run `llvm-objdump -d -r` on the most recently modified game_lib .o file and
/// count crate-internal vs external (std/core/alloc) relocations, with
/// source→target link display for crate-internal calls.
///
/// # Why this matters for hot-reload performance
///
/// When you edit a source file and rebuild, Rust's incremental compilation
/// only recompiles the .o files for the changed compilation units.  Each .o
/// file contains relocation entries for every function call, static reference,
/// and vtable dispatch within that unit.
///
/// Relocations fall into two categories:
///   - **crate-internal**: calls to other functions in the same crate
///     (e.g. `update()` calling `compute()`, or `alpha()` calling `beta()`).
///     These are the ones that contribute to the hot-reload linking cost.
///   - **external**: calls into std/core/alloc or other external crates.
///     These are already resolved at the original link time and do not
///     add to the per-rebuild linking overhead.
///
/// # How it works
///
/// We use `llvm-objdump -d -r` which produces disassembly with relocation
/// annotations inline.  Each function is introduced by a header like:
///   `0000000000000000 <_ZN8game_lib4test5alpha17h...E>:`
///
/// Within each function, relocation lines appear indented:
///   `  0000000000000019:  IMAGE_REL_AMD64_REL32  _ZN8game_lib4test6alpha2...`
///
/// By tracking which function header we last saw, we can tag each relocation
/// with its source function, producing output like:
///   `game_lib::test::alpha → game_lib::test::beta`
fn inspect_object_relocations(target_dir: &Path) -> String {
    // Locate llvm-objdump from the Rust sysroot.
    // This tool ships with the `llvm-tools` rustup component and is always
    // at `$(rustc --print sysroot)/lib/rustlib/<target>/bin/`.
    let sysroot = match std::process::Command::new("rustc")
        .args(["--print", "sysroot"])
        .output()
    {
        Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        Err(_) => return "  .o relocations: (rustc not found)".into(),
    };

    // Find the llvm-objdump binary inside the sysroot.
    // The sysroot contains host target triple subdirectories; we scan for the
    // first one that has a bin/llvm-objdump(.exe).
    let mut objdump_exe: Option<PathBuf> = None;
    let rustlib = Path::new(&sysroot).join("lib").join("rustlib");
    if let Ok(entries) = std::fs::read_dir(&rustlib) {
        for entry in entries.flatten() {
            let candidate = entry.path().join("bin").join(if cfg!(windows) {
                "llvm-objdump.exe"
            } else {
                "llvm-objdump"
            });
            if candidate.exists() {
                objdump_exe = Some(candidate);
                break;
            }
        }
    }

    let objdump = match objdump_exe {
        Some(p) => p,
        None => return "  .o relocations: (llvm-objdump not found)".into(),
    };

    // Rust incremental compilation stores .o files under:
    //   target_reload/debug/incremental/game_lib-<hash>/s-<hash>/*.o
    //
    // We only need the most recently modified .o — that's the compilation unit
    // that was just recompiled due to the source edit.
    let incremental_dir = target_dir.join("debug").join("incremental");
    let mut object_files: Vec<PathBuf> = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&incremental_dir) {
        for entry in entries.flatten() {
            let name_str = entry.file_name().to_string_lossy().to_string();
            if name_str.starts_with("game_lib-") && entry.path().is_dir() {
                // Recursively collect all .o files under this crate's directory.
                find_files(&entry.path(), "o", &mut object_files);
                break;
            }
        }
    }

    if object_files.is_empty() {
        return "  .o relocations: (.o files not found in incremental dir)".into();
    }

    // Sort by modification time ascending; the last entry is the newest.
    // Falls back to UNIX_EPOCH for files whose metadata can't be read.
    object_files.sort_by_key(|p| {
        std::fs::metadata(p)
            .and_then(|m| m.modified())
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });
    let object_path = object_files.last().unwrap();

    // Run llvm-objdump with both -d (disassemble) and -r (print relocations).
    // The combined output interleaves relocation entries within their
    // enclosing function's disassembly, which allows us to determine the
    // source function for every relocation.
    let output = match std::process::Command::new(&objdump)
        .args(["-d", "-r"])
        .arg(&object_path)
        .output()
    {
        Ok(o) => o,
        Err(e) => return format!("  .o relocations: (failed to run: {e})"),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);

    // State machine for parsing the objdump output:
    //   current_function — set when we see `<name>:` header, remains active
    //                      until the next function header.  Each COMDAT .text
    //                      section in the disassembly corresponds to one
    //                      compiled function.
    let mut current_function: Option<String> = None;
    let mut total: usize = 0;
    let mut crate_internal: usize = 0;
    // Build call graph: caller → list of internal callees.
    let mut call_graph: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    // Track external calls by category for the summary line.
    // Categories: anon, core, std, alloc, compiler, panic, other.
    let mut external_by_category: std::collections::HashMap<&str, usize> =
        std::collections::HashMap::new();

    for line in stdout.lines() {
        let trimmed = line.trim();

        // Detect function headers:  0000000000000000 <mangled_name>:
        // The name inside angle brackets is the mangled Rust symbol
        // (or an unmangled C name for `extern "C"` functions like
        // `update` and `compute`).
        if let Some((_addr, name_part)) = trimmed.split_once('<') {
            if let Some((name, _rest)) = name_part.split_once('>') {
                // Must have a nontrivial name and end with ':' — the full
                // header line ends with `>:`.
                if name.len() > 1 && trimmed.ends_with(':') {
                    // Demangle immediately so we have the human-readable name
                    // ready for the source column in the link output.
                    current_function = Some(pretty_name(name));
                    continue;
                }
            }
        }

        // Detect relocation lines.  These appear indented within function
        // disassembly blocks, with the format:
        //   <spaces><address>:  <reloc_type>        <symbol_name>
        //
        // We check for Windows (COFF) relocation types as well as ELF/Mach-O
        // types for cross-platform compatibility.
        //
        // Common relocation types encountered:
        //   IMAGE_REL_AMD64_REL32  — 32-bit PC-relative (call/jmp instructions)
        //   IMAGE_REL_AMD64_ADDR64 — 64-bit absolute address (data references)
        //   R_X86_64_PC32         — ELF equivalent of REL32
        //   R_X86_64_REX_GOTPCRELX — ELF GOT-indirect PC-relative
        let is_reloc = trimmed.contains("IMAGE_REL_AMD64_REL32")
            || trimmed.contains("IMAGE_REL_AMD64_ADDR64")
            || trimmed.contains("R_X86_64_PC32")
            || trimmed.contains("R_X86_64_REX_GOTPCRELX")
            || trimmed.contains("R_AARCH64")
            || trimmed.contains("R_ARM");
        if !is_reloc {
            continue;
        }

        // The symbol name is the last whitespace-delimited token on the line.
        // For COFF:  IMAGE_REL_AMD64_REL32  _ZN8game_lib4test4beta17h...
        // For ELF:   R_X86_64_PC32  _ZN8game_lib4test4beta17h...-0x4
        let symbol = trimmed.split_whitespace().last().unwrap_or("");
        total += 1;

        // Classify the relocation target as either external or crate-internal.
        //
        // External symbols come from the standard library, core, alloc, or
        // compiler-generated infrastructure.  Their mangled names contain
        // characteristic substrings:
        //
        //   anon.*       — anonymous constants (string literals, static refs)
        //                  Example: anon.d2f6ec2e5cf7a7c625a745804f69385f.0
        //   3std         — std crate (mangled: length "3" + "std")
        //   4core        — core crate (mangled: length "4" + "core")
        //   5alloc       — alloc crate (String, Vec, Box internals)
        //   $alloc$      — alloc-related compiler-generated symbols
        //   $alloc.      — alloc-related compiler-generated symbols (variant)
        //   6compiler    — compiler_builtins crate (memcpy, memset, etc.)
        //   11panic      — panic infrastructure
        //
        // Everything else is assumed to be crate-internal — a function call
        // to another symbol within game_lib.
        let is_external = symbol.starts_with("anon.")
            || symbol.contains("3std")
            || symbol.contains("4core")
            || symbol.contains("5alloc")
            || symbol.contains("$alloc$")
            || symbol.contains("$alloc.")
            || symbol.contains("6compiler")
            || symbol.contains("11panic");

        if !is_external {
            crate_internal += 1;
            let src_name = current_function.as_deref().unwrap_or("?").to_string();
            let dst_name = pretty_name(symbol);
            call_graph.entry(src_name).or_default().push(dst_name);
        } else {
            // Classify the external symbol into a category for the summary.
            let category = if symbol.starts_with("anon.") {
                "anon"
            } else if symbol.contains("4core") {
                "core"
            } else if symbol.contains("3std") {
                "std"
            } else if symbol.contains("5alloc")
                || symbol.contains("$alloc$")
                || symbol.contains("$alloc.")
            {
                "alloc"
            } else if symbol.contains("6compiler") {
                "compiler"
            } else if symbol.contains("11panic") {
                "panic"
            } else {
                "other"
            };
            *external_by_category.entry(category).or_default() += 1;
        }
    }

    // Build the result: summary line, internal tree, external summary.
    let mut result = format!(
        "  .o relocations ({}): total={total}",
        object_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy(),
    );

    // ── Internal links section ──
    result.push_str(&format!("\n    {crate_internal} internal links:"));

    if !call_graph.is_empty() {
        // Identify root callers: functions that appear as sources but never
        // as targets of another function in this .o.
        let all_callees: std::collections::HashSet<&String> =
            call_graph.values().flatten().collect();
        let mut roots: Vec<&String> = call_graph
            .keys()
            .filter(|k| !all_callees.contains(k))
            .collect();
        roots.sort();

        // Recursively print the call tree with Unicode box-drawing characters:
        //   ├─  branch (not last child)
        //   └─  leaf (last child)
        //   │   vertical continuation (ancestor still has more siblings)
        //
        // Layout:
        //   Root:   "    - {name}"            (4-space indent + bullet)
        //   Child:  "      {prefix}{branch}{name}"  (6-space base + connectors)
        fn print_tree(
            func: &str,
            graph: &std::collections::HashMap<String, Vec<String>>,
            prefixes: &[bool],
            visited: &mut std::collections::HashSet<String>,
            out: &mut Vec<String>,
        ) {
            if !visited.insert(func.to_string()) {
                let mut connector = String::new();
                let ancestor_count = prefixes.len().saturating_sub(1);
                for index in 0..ancestor_count {
                    connector.push_str(if prefixes[index] { "   " } else { "│  " });
                }
                out.push(format!(
                    "      {connector}├─ {func} (recursive, shown above)"
                ));
                return;
            }

            // Build continuation prefix from ancestor is_last state.
            let mut prefix = String::new();
            let ancestor_count = prefixes.len().saturating_sub(1);
            for index in 0..ancestor_count {
                prefix.push_str(if prefixes[index] { "   " } else { "│  " });
            }

            if prefixes.is_empty() {
                // Root node — bullet with 4-space indent.
                out.push(format!("    - {func}"));
            } else {
                // Non-root — 6-space base + ancestor connectors + branch.
                let is_last = *prefixes.last().unwrap();
                let branch = if is_last { "└─ " } else { "├─ " };
                out.push(format!("      {prefix}{branch}{func}"));
            }

            if let Some(callees) = graph.get(func) {
                let count = callees.len();
                for (index, callee) in callees.iter().enumerate() {
                    let is_last_child = index == count - 1;
                    let mut child_prefixes = prefixes.to_vec();
                    child_prefixes.push(is_last_child);
                    print_tree(callee, graph, &child_prefixes, visited, out);
                }
            }
        }

        let mut tree_lines: Vec<String> = Vec::new();
        for root in &roots {
            let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();
            print_tree(root, &call_graph, &[], &mut visited, &mut tree_lines);
        }
        result.push('\n');
        result.push_str(&tree_lines.join("\n"));
    }

    // ── External links section ──
    let external_count = total.saturating_sub(crate_internal);
    result.push_str(&format!("\n    {external_count} external links:"));

    if !external_by_category.is_empty() {
        let mut categories: Vec<(&str, usize)> = external_by_category.into_iter().collect();
        categories.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        for (cat, count) in &categories {
            result.push_str(&format!("\n      - {cat}×{count}"));
        }
    }
    result
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Recursively find all files with a given extension under a directory tree.
///
/// Used to locate .o files inside Rust's incremental compilation output
/// directory, which has deeply nested hash-named subdirectories like:
///   `incremental/game_lib-<hash>/s-<hash>/<random-hash>.o`
fn find_files(dir: &Path, ext: &str, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                find_files(&path, ext, out);
            } else if path.extension().map_or(false, |e| e == ext) {
                out.push(path);
            }
        }
    }
}

/// Simple Rust name demangler for game_lib symbols.
///
/// Rust mangles symbols using a length-prefixed scheme:
///   `_ZN`                            — common prefix ("N" = nested name)
///   `8game_lib`                      — crate name (8 chars)
///   `4test`                          — module name (4 chars)
///   `5alpha`                         — function name (5 chars)
///   `17h<hex>`                       — hash suffix (17-byte hash in hex)
///   `E`                              — terminator
///
/// Full example:
///   `_ZN8game_lib4test5alpha17hf326af0f4e83092aE`
///   → `game_lib::test::alpha`
///
/// This is a best-effort demangler.  It does not handle generics, closures,
/// trait methods, or full Rust v0 mangling (the `_R` prefix scheme).
/// Unknown or non-game_lib symbols are returned as-is.
fn pretty_name(raw: &str) -> String {
    // Locate the "game_lib" crate name within the mangled string.
    // This serves as our anchor point for the length-prefix walk.
    if let Some(pos) = raw.find("game_lib") {
        // Find the length digit that precedes "game_lib".
        // In "_ZN8game_lib..." we need to step back from "g" past "8"
        // to the start of the length-prefix sequence.
        let digit_start = raw[..pos]
            .rfind(|c: char| !c.is_ascii_digit())
            .map(|p| p + 1)
            .unwrap_or(0);

        // after = "8game_lib4test5alpha17h..." — the length-prefixed
        // segments starting from the crate name.
        let after = &raw[digit_start..];

        let mut result = String::new();
        let mut remaining = after;

        // Walk the length-prefixed segments:
        //   1. Read N consecutive ASCII digits → that's the segment length L
        //   2. Extract the next L characters → that's the segment value
        //   3. Repeat until we hit the hash suffix or end of string
        // The hash suffix starts with 'h' followed by hex digits — this is
        // a content hash, not a name segment.
        while !remaining.is_empty() {
            // Find where the digit run ends to get the segment length.
            let digit_end = remaining
                .chars()
                .position(|c| !c.is_ascii_digit())
                .unwrap_or(remaining.len());
            if digit_end == 0 {
                break; // no more length-prefixed segments
            }
            let len: usize = remaining[..digit_end].parse().unwrap_or(0);
            remaining = &remaining[digit_end..];

            if remaining.len() < len {
                break; // malformed: segment would extend past the string end
            }
            let segment = &remaining[..len];

            // Detect the hash suffix.  After the last real name segment,
            // Rust appends a hash like "17h<32 hex chars>E".  If the current
            // segment starts with 'h' and the rest are all hex digits, we've
            // reached the hash — stop here.
            if segment.starts_with('h') && segment[1..].chars().all(|c| c.is_ascii_hexdigit()) {
                break;
            }

            // Join segments with Rust's `::` path separator.
            if !result.is_empty() {
                result.push_str("::");
            }
            result.push_str(segment);
            remaining = &remaining[len..];
        }

        if !result.is_empty() {
            return result;
        }
    }

    // Fallback: return the raw symbol unchanged.  This handles:
    //   - Unmangled C names (e.g. "update", "compute")
    //   - Symbols from other crates
    //   - Compiler-generated symbols that don't contain "game_lib"
    raw.to_string()
}

// ---------------------------------------------------------------------------
// PE header parser (seek-based, no full-file read)
// ---------------------------------------------------------------------------

/// Seek-based PE (Portable Executable) parser.
///
/// Reads only the ~2 KB of headers from the DLL — the DOS header, COFF header,
/// optional header, section table, and export directory.  The actual code and
/// data sections (which can be hundreds of KB) are never loaded into memory.
///
/// # PE/COFF layout (simplified)
///
/// ```text
/// +─────────────────────+  0x00
/// │   DOS Header         │  64 bytes
/// │   - e_magic: "MZ"    │
/// │   - e_lfanew @ 0x3C  │  → points to PE signature
/// +─────────────────────+
/// │   DOS Stub            │  (legacy "This program cannot be run in DOS mode")
/// +─────────────────────+  pe_offset (e_lfanew)
/// │   PE Signature        │  4 bytes: "PE\0\0"
/// +─────────────────────+  coff_header
/// │   COFF Header         │  20 bytes
/// │   - Machine           │  (0x8664 = x86-64)
/// │   - NumberOfSections  │
/// │   - SizeOfOptional    │
/// +─────────────────────+  optional_header
/// │   Optional Header     │  variable (PE32: 224, PE32+: 240 bytes)
/// │   - Magic             │  0x10B=PE32, 0x20B=PE32+
/// │   - DataDirectory[]   │  16 entries of {RVA, Size}
/// +─────────────────────+  section_table_offset
/// │   Section Table       │  NumberOfSections × 40 bytes
/// │   - Name[8]           │  ".text", ".rdata", ".reloc", etc.
/// │   - VirtualSize       │
/// │   - VirtualAddress    │  RVA (Relative Virtual Address)
/// │   - RawDataOffset     │  file offset of the section data
/// +─────────────────────+
/// ```
///
/// We extract three things:
///   1. **.text section size** — code footprint; grows with function count
///   2. **Base relocation count** — OS loader work at LoadLibraryW time
///   3. **Export names** — to verify which symbols are visible to the host
fn parse_pe_info_seek(file: &mut std::fs::File) -> Result<PeInfo, String> {
    // ────── Step 1: Read DOS + PE headers (first 2 KB) ──────
    //
    // 2 KB is enough to cover:
    //   - DOS header (64 bytes)
    //   - DOS stub (~100 bytes, variable)
    //   - PE signature (4 bytes)
    //   - COFF header (20 bytes)
    //   - Optional header (up to 240 bytes for PE32+)
    //   - Section table (~40 bytes × N sections, typically < 400 bytes)
    //   - First data directory entries

    let mut header_buf = vec![0u8; HEADER_BUFFER_SIZE];
    file.seek(SeekFrom::Start(0))
        .map_err(|e| format!("seek: {e}"))?;
    let header_len = file
        .read(&mut header_buf)
        .map_err(|e| format!("read: {e}"))?;
    if header_len < MINIMUM_PE_HEADER_SIZE {
        return Err("file too small for PE header".into());
    }
    let bytes = &header_buf[..header_len];

    // DOS header offset 0x3C holds e_lfanew — a 4-byte little-endian pointer
    // to the PE signature ("PE\0\0").  This is the entry point for all PE parsing.
    let pe_offset = u32::from_le_bytes([
        bytes[DOS_ELFANEW_OFFSET],
        bytes[DOS_ELFANEW_OFFSET + 1],
        bytes[DOS_ELFANEW_OFFSET + 2],
        bytes[DOS_ELFANEW_OFFSET + 3],
    ]) as usize;
    if pe_offset == 0 || pe_offset + PE_SIGNATURE_SIZE > bytes.len() {
        return Err("invalid PE offset".into());
    }
    if &bytes[pe_offset..pe_offset + PE_SIGNATURE_SIZE] != b"PE\0\0" {
        return Err("not a valid PE file".into());
    }

    // COFF header immediately follows the PE signature (4 bytes after pe_offset).
    let coff_header = pe_offset + PE_SIGNATURE_SIZE;
    // NumberOfSections at COFF+2: count of section table entries.
    let number_of_sections = u16::from_le_bytes([
        bytes[coff_header + COFF_NUMBER_OF_SECTIONS_OFFSET],
        bytes[coff_header + COFF_NUMBER_OF_SECTIONS_OFFSET + 1],
    ]) as usize;
    // SizeOfOptionalHeader at COFF+16: size of the optional header that follows.
    // PE32 (32-bit) = 224, PE32+ (64-bit) = 240.
    let size_of_optional_header = u16::from_le_bytes([
        bytes[coff_header + COFF_SIZE_OF_OPTIONAL_HEADER_OFFSET],
        bytes[coff_header + COFF_SIZE_OF_OPTIONAL_HEADER_OFFSET + 1],
    ]) as usize;

    // Optional header starts right after the 20-byte COFF header.
    let optional_header = coff_header + COFF_HEADER_SIZE;
    // Section table starts after: COFF header (20) + optional header (variable).
    let section_table_offset = coff_header + COFF_HEADER_SIZE + size_of_optional_header;

    // The PE magic number at the start of the optional header tells us whether
    // this is PE32 (0x10B) or PE32+ (0x20B).  The data directory offset within
    // the optional header differs between the two formats.
    let magic = u16::from_le_bytes([bytes[optional_header], bytes[optional_header + 1]]);
    let data_directory_offset = match magic {
        PE32_MAGIC => PE32_DATA_DIRECTORY_OFFSET,
        PE32_PLUS_MAGIC => PE32_PLUS_DATA_DIRECTORY_OFFSET,
        _ => return Err(format!("unknown PE magic: 0x{magic:04X}")),
    };

    // ────── Step 2: Build RVA→file-offset converter ──────
    //
    // PE uses RVAs (Relative Virtual Addresses) everywhere — addresses that
    // are relative to the image base when the DLL is loaded into memory.
    // To read data from the file on disk, we must convert RVA → file offset
    // by walking the section table.
    //
    // Each section table entry is exactly 40 bytes:
    //   Offset  Size  Field
    //   +0      8     Name (UTF-8, null-padded to 8 bytes)
    //   +8      4     VirtualSize (size of section when loaded in memory)
    //   +12     4     VirtualAddress (RVA where section starts in memory)
    //   +16     4     SizeOfRawData (size of section data in the file)
    //   +20     4     PointerToRawData (file offset where section data begins)
    //   +24     4     PointerToRelocations (deprecated, usually 0)
    //   +28     4     PointerToLinenumbers (deprecated, usually 0)
    //   +32     2     NumberOfRelocations
    //   +34     2     NumberOfLinenumbers
    //   +36     4     Characteristics (section flags: code, data, readable, etc.)

    let rva_to_offset = |rva: usize| -> Result<usize, String> {
        for section_index in 0..number_of_sections {
            let section_offset = section_table_offset + section_index * SECTION_ENTRY_SIZE;
            if section_offset + SECTION_ENTRY_SIZE > bytes.len() {
                break; // section entry would extend past our header buffer
            }
            let virtual_size = u32::from_le_bytes([
                bytes[section_offset + SECTION_VIRTUAL_SIZE_OFFSET],
                bytes[section_offset + SECTION_VIRTUAL_SIZE_OFFSET + 1],
                bytes[section_offset + SECTION_VIRTUAL_SIZE_OFFSET + 2],
                bytes[section_offset + SECTION_VIRTUAL_SIZE_OFFSET + 3],
            ]) as usize;
            let virtual_address = u32::from_le_bytes([
                bytes[section_offset + SECTION_VIRTUAL_ADDRESS_OFFSET],
                bytes[section_offset + SECTION_VIRTUAL_ADDRESS_OFFSET + 1],
                bytes[section_offset + SECTION_VIRTUAL_ADDRESS_OFFSET + 2],
                bytes[section_offset + SECTION_VIRTUAL_ADDRESS_OFFSET + 3],
            ]) as usize;
            let raw_data_offset = u32::from_le_bytes([
                bytes[section_offset + SECTION_POINTER_TO_RAW_DATA_OFFSET],
                bytes[section_offset + SECTION_POINTER_TO_RAW_DATA_OFFSET + 1],
                bytes[section_offset + SECTION_POINTER_TO_RAW_DATA_OFFSET + 2],
                bytes[section_offset + SECTION_POINTER_TO_RAW_DATA_OFFSET + 3],
            ]) as usize;
            // An RVA belongs to this section if it falls within
            // [VirtualAddress, VirtualAddress + VirtualSize).
            if rva >= virtual_address && rva < virtual_address + virtual_size {
                // file_offset = section_file_start + (rva - section_rva_start)
                return Ok(raw_data_offset + (rva - virtual_address));
            }
        }
        Err(format!("RVA 0x{rva:08X} not found in any section"))
    };

    // ────── Step 3: Find .text section size ──────
    //
    // The .text section contains all executable code — every compiled function
    // body lives here.  Its size is roughly proportional to the total amount
    // of code in the crate.  For a debug build with 5,000 trivial exported
    // functions, this can be 80-120 KB.
    //
    // We scan the section table (already in our header buffer) for the entry
    // whose 8-byte name starts with ".text".

    let mut text_section_kb: Option<f64> = None;
    for section_index in 0..number_of_sections {
        let section_offset = section_table_offset + section_index * SECTION_ENTRY_SIZE;
        if section_offset + SECTION_ENTRY_SIZE > bytes.len() {
            break;
        }
        // Section names are null-padded UTF-8, up to 8 bytes.
        let name_end = bytes[section_offset..section_offset + SECTION_NAME_MAX_LENGTH]
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(SECTION_NAME_MAX_LENGTH);
        let name =
            std::str::from_utf8(&bytes[section_offset..section_offset + name_end]).unwrap_or("");
        if name == ".text" {
            // SizeOfRawData at offset +16 in the section entry tells us how
            // many bytes of code are stored in the file.
            let raw_size = u32::from_le_bytes([
                bytes[section_offset + SECTION_SIZE_OF_RAW_DATA_OFFSET],
                bytes[section_offset + SECTION_SIZE_OF_RAW_DATA_OFFSET + 1],
                bytes[section_offset + SECTION_SIZE_OF_RAW_DATA_OFFSET + 2],
                bytes[section_offset + SECTION_SIZE_OF_RAW_DATA_OFFSET + 3],
            ]) as f64;
            text_section_kb = Some(raw_size / BYTES_PER_KILOBYTE);
            break;
        }
    }

    // ────── Step 4: Count base relocations ──────
    //
    // Base relocations are stored in the .reloc section.  They tell the OS
    // loader which absolute addresses in the code/data need to be patched
    // when the DLL is loaded at a different base address than expected
    // (ASLR — Address Space Layout Randomization).
    //
    // The data directory entry for the relocation table is at index 5.
    // Each data directory entry is 8 bytes: {RVA (u32), Size (u32)}.
    //
    // .reloc section structure:
    //   The section consists of a series of relocation blocks:
    //     +0  PageRVA    (u32) — base RVA for all entries in this block
    //     +4  BlockSize  (u32) — total bytes in this block (header + entries)
    //     +8  Entries[]  (u16 each) — (BlockSize - 8) / 2 relocation entries
    //
    // Each 16-bit entry encodes:
    //   bits 15-12: relocation type
    //     3 = IMAGE_REL_BASED_DIR64  (64-bit absolute, for x86-64 code)
    //    10 = IMAGE_REL_BASED_DIR64  (same, alternate name)
    //   bits 11-0:  12-bit offset within the 4 KB page

    let mut relocation_count: usize = 0;
    // Index 5 in the data directory = base relocation table.
    // Skip past the first BASE_RELOCATION_DATA_DIRECTORY_INDEX entries.
    let reloc_dir_offset = optional_header
        + data_directory_offset
        + BASE_RELOCATION_DATA_DIRECTORY_INDEX * DATA_DIRECTORY_ENTRY_SIZE;
    if reloc_dir_offset + DATA_DIRECTORY_ENTRY_SIZE <= bytes.len() {
        let reloc_rva = u32::from_le_bytes([
            bytes[reloc_dir_offset],
            bytes[reloc_dir_offset + 1],
            bytes[reloc_dir_offset + 2],
            bytes[reloc_dir_offset + 3],
        ]) as usize;
        let reloc_size = u32::from_le_bytes([
            bytes[reloc_dir_offset + 4],
            bytes[reloc_dir_offset + 5],
            bytes[reloc_dir_offset + 6],
            bytes[reloc_dir_offset + 7],
        ]) as usize;

        if reloc_rva != 0 && reloc_size != 0 {
            if let Ok(base_offset) = rva_to_offset(reloc_rva) {
                // Read the entire .reloc section into memory.
                // For our DLLs this is typically 5-20 KB.
                let mut reloc_buf = vec![0u8; reloc_size];
                file.seek(SeekFrom::Start(base_offset as u64))
                    .map_err(|e| format!("seek reloc: {e}"))?;
                let _ = file
                    .read(&mut reloc_buf)
                    .map_err(|e| format!("read reloc: {e}"))?;

                // Walk the relocation blocks sequentially.
                let mut pos = 0;
                while pos + 8 <= reloc_buf.len() {
                    let block_size = u32::from_le_bytes([
                        reloc_buf[pos + 4],
                        reloc_buf[pos + 5],
                        reloc_buf[pos + 6],
                        reloc_buf[pos + 7],
                    ]) as usize;

                    if block_size < 8 || pos + block_size > reloc_buf.len() {
                        break; // malformed block or out of bounds
                    }

                    // Block header is 8 bytes (PageRVA + BlockSize).
                    // Remaining bytes are 2-byte relocation entries.
                    relocation_count += (block_size - 8) / 2;
                    pos += block_size;
                }
            }
        }
    }

    // ────── Step 5: Parse export table ──────
    //
    // The export directory (IMAGE_EXPORT_DIRECTORY) is at data directory
    // entry 0.  It describes all symbols exported by the DLL — the functions
    // that GetProcAddress can look up.
    //
    // IMAGE_EXPORT_DIRECTORY structure (40 bytes):
    //   Offset  Size  Field
    //   +0      4     Characteristics (reserved, usually 0)
    //   +4      4     TimeDateStamp (Unix timestamp of build)
    //   +8      2     MajorVersion
    //   +10     2     MinorVersion
    //   +12     4     Name (RVA to the DLL name string)
    //   +16     4     Base (starting ordinal number, typically 1)
    //   +20     4     NumberOfFunctions (total exports, including ordinals)
    //   +24     4     NumberOfNames (named exports — what we care about)
    //   +28     4     AddressOfFunctions (RVA to array of function RVAs)
    //   +32     4     AddressOfNames (RVA to array of name string RVAs)
    //   +36     4     AddressOfNameOrdinals (RVA to array of ordinal u16s)
    //
    // We only need NumberOfNames and AddressOfNames to collect export names.

    let exports = {
        // Data directory entry 0 is at the very start of the data directory
        // array within the optional header.
        let export_dir_rva_offset = optional_header
            + data_directory_offset
            + EXPORT_DATA_DIRECTORY_INDEX * DATA_DIRECTORY_ENTRY_SIZE;
        if export_dir_rva_offset + DATA_DIRECTORY_ENTRY_SIZE > bytes.len() {
            return Err("data directory out of bounds".into());
        }
        let export_rva = u32::from_le_bytes([
            bytes[export_dir_rva_offset],
            bytes[export_dir_rva_offset + 1],
            bytes[export_dir_rva_offset + 2],
            bytes[export_dir_rva_offset + 3],
        ]) as usize;
        let export_size = u32::from_le_bytes([
            bytes[export_dir_rva_offset + 4],
            bytes[export_dir_rva_offset + 5],
            bytes[export_dir_rva_offset + 6],
            bytes[export_dir_rva_offset + 7],
        ]) as usize;

        // No exports section present (RVA and size are both zero).
        if export_rva == 0 || export_size == 0 {
            Vec::new()
        } else {
            // Convert RVA to file offset and read the export directory header.
            let export_offset = rva_to_offset(export_rva)?;
            let mut dir_buf = [0u8; EXPORT_DIRECTORY_SIZE];
            file.seek(SeekFrom::Start(export_offset as u64))
                .map_err(|e| format!("seek export: {e}"))?;
            file.read_exact(&mut dir_buf)
                .map_err(|e| format!("read export dir: {e}"))?;

            let number_of_names = u32::from_le_bytes([
                dir_buf[EXPORT_NUMBER_OF_NAMES_OFFSET],
                dir_buf[EXPORT_NUMBER_OF_NAMES_OFFSET + 1],
                dir_buf[EXPORT_NUMBER_OF_NAMES_OFFSET + 2],
                dir_buf[EXPORT_NUMBER_OF_NAMES_OFFSET + 3],
            ]) as usize;
            let address_of_names_rva = u32::from_le_bytes([
                dir_buf[EXPORT_ADDRESS_OF_NAMES_OFFSET],
                dir_buf[EXPORT_ADDRESS_OF_NAMES_OFFSET + 1],
                dir_buf[EXPORT_ADDRESS_OF_NAMES_OFFSET + 2],
                dir_buf[EXPORT_ADDRESS_OF_NAMES_OFFSET + 3],
            ]) as usize;

            if number_of_names == 0 || address_of_names_rva == 0 {
                Vec::new()
            } else {
                // The name pointer array at AddressOfNames contains
                // `number_of_names` RVAs (4 bytes each).  Each RVA points
                // to a null-terminated ASCII string — the export name.
                let names_offset = rva_to_offset(address_of_names_rva)?;
                let names_array_size = number_of_names * 4;
                let mut names_array = vec![0u8; names_array_size];
                file.seek(SeekFrom::Start(names_offset as u64))
                    .map_err(|e| format!("seek names: {e}"))?;
                file.read_exact(&mut names_array)
                    .map_err(|e| format!("read names: {e}"))?;

                let mut names: Vec<String> = Vec::with_capacity(number_of_names);
                // Reuse a stack buffer for reading each name.
                // Export names are short (e.g. "update", "export_01234").
                let mut name_buf = [0u8; EXPORT_NAME_BUFFER_SIZE];
                for index in 0..number_of_names {
                    let entry = index * 4;
                    let name_rva = u32::from_le_bytes([
                        names_array[entry],
                        names_array[entry + 1],
                        names_array[entry + 2],
                        names_array[entry + 3],
                    ]) as usize;
                    if let Ok(name_file_offset) = rva_to_offset(name_rva) {
                        file.seek(SeekFrom::Start(name_file_offset as u64))
                            .map_err(|e| format!("seek name: {e}"))?;
                        let n = file
                            .read(&mut name_buf)
                            .map_err(|e| format!("read name: {e}"))?;
                        // Null-terminated string: find the first zero byte.
                        let end = name_buf[..n].iter().position(|&b| b == 0).unwrap_or(n);
                        if let Ok(s) = std::str::from_utf8(&name_buf[..end]) {
                            names.push(s.to_string());
                        }
                    }
                }
                names
            }
        }
    };

    Ok(PeInfo {
        exports,
        text_section_kb,
        relocation_count,
    })
}
