// REQUIREMENTS
//   Rust stable (edition 2024).  Windows only.  Run from the live_coding/
//   directory:
//     cargo build
//     cargo run -p live_patch -- loop
//
// DESCRIPTION
//   Live Coding host — a Live++ / UE-style hot reload loop.  The base DLL
//   (patch_dll.dll) is loaded ONCE and its exported `update`/`compute` are
//   called through typed HotFn wrappers — no jump table.  When you edit code
//   and save:
//
//     - Leaf functions (compute) are recompiled to a tiny DLL, then their
//       PROLOGUE in the loaded base DLL is patched with a `jmp rel32` to the
//       new code.  Existing callers are redirected in place (Live Coding!).
//     - Anything else (update, structs, helpers, test.rs, new modules, a
//       leaf that references internals) triggers a full cargo build with a
//       LOAD-ALONGSIDE reload: the fresh DLL is loaded while the old one
//       stays running, pointers are swapped, then the old DLL is freed — no
//       unload gap.
//     - A brand-new function can be compiled, loaded, and REGISTERED by name
//       at runtime (the "add a new function live" capability).
//     - Host-owned state (via HostServices) survives every patch and reload.
//
// USAGE
//   cargo run -p live_patch -- loop        # hot-reload loop
//   cargo run -p live_patch                 # static demos
//
// --- SCRIPT ---

use live_patch as live_coding;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

// ---------------------------------------------------------------------------
// Direct function pointers to the loaded base DLL — no indirection.
// ---------------------------------------------------------------------------

type UpdateFn = unsafe extern "C" fn(tick: i32);
type ComputeFn = unsafe extern "C" fn(x: i32, y: i32) -> i32;
type ExtraFn = unsafe extern "C" fn(tick: i32) -> i32;

// ---------------------------------------------------------------------------
// main — CLI dispatch
// ---------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "loop" {
        run_hot_reload_loop();
    } else {
        println!("=== Live Coding (Live++-style) ===\n");
        println!("  Run with 'cargo run -p live_patch -- loop' for hot-reload mode.\n");

        demo_prologue_patch();
        demo_new_function();

        println!("\n=== Demos complete ===");
    }
}

// ---------------------------------------------------------------------------
// Demo 1 — prologue patching an existing function in place
// ---------------------------------------------------------------------------

fn demo_prologue_patch() {
    println!("--- Demo: prologue-patch an existing function in place ---");

    // Compile a replacement for a "compute" that returns 9999 instead of the
    // normal formula, load it, and patch the ORIGINAL function's prologue.
    let original_address: u64 = demo_compute_original as *const () as usize as u64;
    let params = "x: i32, y: i32";
    let body = "9999";
    let return_type = "i32";

    let start = Instant::now();
    let result = unsafe {
        live_coding::compile_and_patch_prologue(
            "demo_compute",
            params,
            body,
            return_type,
            original_address,
        )
    };
    match result {
        Ok(_) => {
            println!(
                "  Patched demo_compute prologue in {:?}.  Original address now jumps to new code.",
                start.elapsed()
            );
            let before = demo_compute_original(1, 2);
            // NOTE: after prologue patching, even the ORIGINAL pointer returns 9999!
            println!(
                "  Calling the ORIGINAL pointer: demo_compute(1,2) = {before}  (should be 9999!)"
            );
        }
        Err(e) => println!("  Failed: {e}"),
    }

    println!();
}

/// A stand-in "existing" function that will be prologue-patched.
fn demo_compute_original(_x: i32, _y: i32) -> i32 {
    42
}

// ---------------------------------------------------------------------------
// Demo 2 — compile + register a NEW function at runtime
// ---------------------------------------------------------------------------

fn demo_new_function() {
    println!("--- Demo: add a NEW function at runtime ---");

    let start = Instant::now();
    let address = unsafe {
        live_coding::compile_and_register_new("extra_new", "tick: i32", "tick * 100 + 7", "i32")
    };
    match address {
        Ok(addr) => {
            println!(
                "  Registered new function 'extra_new' at {addr:#x} in {:?}",
                start.elapsed()
            );
            // Resolve by name and call it — this function did not exist at startup.
            if let Some(resolved) = live_coding::resolve_symbol("extra_new") {
                let extra_fn: live_coding::HotFn<ExtraFn> =
                    unsafe { live_coding::HotFn::from_address(resolved) };
                let value = unsafe { extra_fn.call(3) };
                println!("  Called registered 'extra_new'(3) = {value}  (expected 307)");
            } else {
                println!("  Could not resolve 'extra_new' by name");
            }
        }
        Err(e) => println!("  Failed: {e}"),
    }

    println!();
}

// ---------------------------------------------------------------------------
// Build helpers — capture compiler output so errors can be shown and handled.
// ---------------------------------------------------------------------------

/// Run `cargo build -p patch_dll --target-dir <dir>` and capture its output.
///
/// Returns `Ok(())` on success, or `Err` with the captured stderr (which
/// contains the rustc diagnostics) when the build fails.
fn build_patch_dll(workspace_root: &std::path::Path) -> Result<(), String> {
    let output = std::process::Command::new("cargo")
        .args(["build", "-p", "patch_dll", "--target-dir", "target_reload"])
        .current_dir(workspace_root)
        .output()
        .map_err(|e| format!("failed to run cargo build: {e}"))?;

    if output.status.success() {
        Ok(())
    } else {
        // stderr holds the rustc diagnostics; strip trailing whitespace.
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        Err(format!("{stdout}{stderr}").trim().to_string())
    }
}

/// A loaded base DLL and everything needed to use and later release it.
struct BaseDll {
    handle: *mut std::ffi::c_void,
    temp_path: PathBuf,
    update_address: u64,
    compute_address: u64,
}

/// Unique temp path for a base-DLL copy (PID + counter avoids collisions).
fn unique_temp_path(kind: &str) -> PathBuf {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let counter = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    std::env::temp_dir().join(format!("{kind}_{}_{counter}.dll", std::process::id()))
}

/// Load a freshly built base DLL the LOAD-ALONGSIDE way: copy it to a unique
/// temp path, load the copy, hand it the host services, and resolve
/// `update`/`compute`.  Loading the copy means the original file is never
/// locked, so cargo can overwrite it on the next build.
unsafe fn load_base_dll_copy(base_dll: &std::path::Path) -> Result<BaseDll, String> {
    let temp_path = unique_temp_path("live_base");
    std::fs::copy(base_dll, &temp_path).map_err(|e| {
        format!(
            "cannot copy {} to {}: {e}",
            base_dll.display(),
            temp_path.display()
        )
    })?;

    let handle = match unsafe { live_coding::load_library(&temp_path) } {
        Ok(handle) => handle,
        Err(e) => {
            let _ = std::fs::remove_file(&temp_path);
            return Err(e);
        }
    };

    // Hand the DLL the host services so its code can reach host-owned state.
    if let Err(e) = unsafe { live_coding::provide_services_to_dll(handle) } {
        unsafe { live_coding::unload_library(handle) };
        let _ = std::fs::remove_file(&temp_path);
        return Err(e);
    }

    let update_address = match unsafe { live_coding::resolve_export(handle, "update") } {
        Ok(address) => address,
        Err(e) => {
            unsafe { live_coding::unload_library(handle) };
            let _ = std::fs::remove_file(&temp_path);
            return Err(e);
        }
    };
    let compute_address = match unsafe { live_coding::resolve_export(handle, "compute") } {
        Ok(address) => address,
        Err(e) => {
            unsafe { live_coding::unload_library(handle) };
            let _ = std::fs::remove_file(&temp_path);
            return Err(e);
        }
    };

    Ok(BaseDll {
        handle,
        temp_path,
        update_address,
        compute_address,
    })
}

// ---------------------------------------------------------------------------
// Live-reload loop
// ---------------------------------------------------------------------------

fn run_hot_reload_loop() {
    let exe_dir = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let workspace_root = exe_dir.parent().unwrap().parent().unwrap();
    let patch_src = workspace_root.join("patch_dll").join("src").join("lib.rs");

    // 1. Cold build + initial load of the BASE DLL.
    println!("  Building patch_dll.dll (cold)...");
    let build_start = Instant::now();
    match build_patch_dll(&workspace_root) {
        Ok(()) => println!("  Cold build: {:?}", build_start.elapsed()),
        Err(errors) => {
            println!("  ? cold build failed.  Compiler output:\n{errors}");
            return;
        }
    }

    let base_dll = workspace_root
        .join("target_reload")
        .join("debug")
        .join("patch_dll.dll");

    // Load the base DLL (as a temp copy — the original file stays unlocked)
    // and resolve the exported entry points into typed HotFn wrappers.  No
    // wrapper, no jump table: after a prologue patch these same pointers
    // transparently call the new code.
    print!("  Loading base DLL (temp copy)... ");
    let mut base_dll_state = match unsafe { load_base_dll_copy(&base_dll) } {
        Ok(state) => state,
        Err(e) => {
            println!("failed: {e}");
            return;
        }
    };
    println!(
        "OK (update @ {:#x}, compute @ {:#x})",
        base_dll_state.update_address, base_dll_state.compute_address
    );

    // Typed callable wrappers — the ONLY way we call the game functions.
    // `compute_address` is kept mutable because it is the prologue-patch
    // target and changes on every full reload.
    let mut compute_address = base_dll_state.compute_address;
    let mut update_fn: live_coding::HotFn<UpdateFn> =
        unsafe { live_coding::HotFn::from_address(base_dll_state.update_address) };
    let mut compute_fn: live_coding::HotFn<ComputeFn> =
        unsafe { live_coding::HotFn::from_address(base_dll_state.compute_address) };

    // 2. Watch state — snapshot every .rs under patch_dll/src/ plus
    //    Cargo.toml and build.rs, so ANY change (structs, helpers, test.rs,
    //    new modules, dependencies, ...) is detected.
    let patch_src_root = workspace_root.join("patch_dll").join("src");
    let patch_dll_root = workspace_root.join("patch_dll");
    let mut source_snapshots: HashMap<PathBuf, Vec<u8>> = HashMap::new();
    for source_file in collect_watch_files(&patch_src_root, &patch_dll_root) {
        let content = std::fs::read(&source_file).unwrap_or_default();
        source_snapshots.insert(source_file, content);
    }
    // lib.rs with all extern bodies stripped — used to tell "only a function
    // body changed" (fast prologue-patch) from "structs/helpers/other files
    // changed" (full rebuild).
    let mut stripped_lib_rs =
        strip_extern_bodies(&std::fs::read_to_string(&patch_src).unwrap_or_default());

    // Cache of previous function bodies: (name, params, body, ret_type).
    let mut cached_funcs: Vec<(String, String, String, String)> = Vec::new();
    let mut iteration: u64 = 0;

    println!();
    println!("  +--------------------------------------------------------------+");
    println!("  |   LIVE CODING LOOP -- edit any .rs under patch_dll/src, save |");
    println!("  |                                                              |");
    println!("  |   Leaf functions (compute) -> prologue-patched in place      |");
    println!("  |   Dep functions  (update)  -> cargo build + reload           |");
    println!("  |   NEW functions            -> compiled + registered by name  |");
    println!("  |   Structs / helpers /      -> full cargo build + reload      |");
    println!("  |   test.rs / new modules                                       |");
    println!("  |                                                              |");
    println!("  |   Base DLL stays loaded.  Reloads are load-alongside.        |");
    println!("  |   Press Ctrl+C to exit.                                      |");
    println!("  +--------------------------------------------------------------+");
    println!();

    loop {
        iteration += 1;
        game_tick(iteration, update_fn, compute_fn);
        std::thread::sleep(std::time::Duration::from_millis(800));

        // Diff the on-disk source against the last snapshot — ANY file change.
        let source_files = collect_watch_files(&patch_src_root, &patch_dll_root);
        let mut changed_files: Vec<PathBuf> = Vec::new();
        for source_file in &source_files {
            let current = std::fs::read(source_file).unwrap_or_default();
            let previous = source_snapshots.get(source_file);
            if previous != Some(&current) {
                changed_files.push(source_file.clone());
            }
            source_snapshots.insert(source_file.clone(), current);
        }

        if changed_files.is_empty() {
            continue;
        }

        println!("\n  -- source changed (iter {iteration}) --");
        for source_file in &changed_files {
            let relative = source_file
                .strip_prefix(&workspace_root)
                .unwrap_or(source_file);
            println!("      {}", relative.display());
        }

        let source = match std::fs::read_to_string(&patch_src) {
            Ok(s) => s,
            Err(e) => {
                println!("  ? Cannot read {patch_src:?}: {e}");
                continue;
            }
        };

        let funcs = extract_function_bodies(&source);

        // A full rebuild is needed when ANYTHING outside the extern function
        // bodies changed — structs, internal helpers, attributes, `mod` decls
        // — or when another file (test.rs, a new module) changed.  Compare
        // lib.rs with its extern bodies stripped: differing stripped text
        // means non-body content changed.
        let new_stripped_lib_rs = strip_extern_bodies(&source);
        let lib_rs_changed = changed_files
            .iter()
            .any(|source_file| *source_file == patch_src);
        let mut full_rebuild = !lib_rs_changed || new_stripped_lib_rs != stripped_lib_rs;
        stripped_lib_rs = new_stripped_lib_rs;

        // If `update` (a function with internal deps) changed, a full rebuild
        // is needed too — detect it up front so leaf patches are skipped.
        if funcs.iter().any(|(name, _, body, _)| {
            name == "update"
                && cached_funcs
                    .iter()
                    .find(|(cached_name, _, cached_body, _)| {
                        cached_name == name && cached_body == body
                    })
                    .is_none()
        }) {
            full_rebuild = true;
        }

        let total_start = Instant::now();
        let mut patched = 0usize;
        let mut skipped = 0usize;
        let mut absorbed_by_rebuild = 0usize;
        // Functions whose changes were applied successfully this round.
        // Only these get committed to the cache (so failures retry later).
        let mut applied_funcs: Vec<(String, String, String, String)> = Vec::new();

        for (name, params, body, ret_type) in &funcs {
            // Check if this function's body actually changed.
            let changed = cached_funcs
                .iter()
                .find(|(n, _, b, _)| n == name && b == body)
                .is_none();

            if !changed {
                skipped += 1;
                continue;
            }

            if full_rebuild {
                // The full rebuild recompiles everything from source, so any
                // per-function patching here would be discarded — skip it.
                absorbed_by_rebuild += 1;
                continue;
            }

            match name.as_str() {
                // Leaf function — prologue-patch in place (Live Coding!).
                "compute" => {
                    print!("    {name} (leaf, prologue-patch)... ");
                    let func_start = Instant::now();
                    let result = unsafe {
                        live_coding::compile_and_patch_prologue(
                            name,
                            params,
                            body,
                            ret_type,
                            compute_address,
                        )
                    };
                    match result {
                        Ok(_) => {
                            println!("? patched in {:?}", func_start.elapsed());
                            patched += 1;
                            applied_funcs.push((
                                name.clone(),
                                params.clone(),
                                body.clone(),
                                ret_type.clone(),
                            ));
                        }
                        Err(e) => println!("? {e}"),
                    }
                }
                // Function with internal deps — full cargo rebuild + reload.
                "update" => {
                    full_rebuild = true;
                }
                // Any OTHER name is treated as a NEW function: compile,
                // load, and register it by name at runtime.
                other => {
                    print!("    {other} (NEW, register by name)... ");
                    let func_start = Instant::now();
                    let result = unsafe {
                        live_coding::compile_and_register_new(other, params, body, ret_type)
                    };
                    match result {
                        Ok(addr) => {
                            println!("? registered @ {addr:#x} in {:?}", func_start.elapsed());
                            patched += 1;
                            applied_funcs.push((
                                name.clone(),
                                params.clone(),
                                body.clone(),
                                ret_type.clone(),
                            ));
                        }
                        Err(e) => println!("? {e}"),
                    }
                }
            }
        }

        // Merge the newly applied changes into the cache, keeping entries
        // for functions that were unchanged this round and replacing the
        // entries for functions whose changes were applied successfully.
        // Failed changes are NOT committed, so the next save retries them.
        for applied in &applied_funcs {
            upsert_cached_func(&mut cached_funcs, applied.clone());
        }

        // A full DLL reload is needed for non-extern changes or when a
        // dependent function (update) changed.
        if full_rebuild {
            println!("    full cargo build + load-alongside...");

            let build_start = Instant::now();
            match build_patch_dll(&workspace_root) {
                Ok(()) => {
                    // Load the fresh DLL ALONGSIDE the old one (no unload gap):
                    // copy to a unique temp path so cargo can overwrite the
                    // original file next time, load the copy, swap the host's
                    // pointers, then free the old DLL.
                    match unsafe { load_base_dll_copy(&base_dll) } {
                        Ok(new_state) => {
                            // Swap the callables and the prologue-patch target
                            // to the freshly built DLL.
                            compute_address = new_state.compute_address;
                            update_fn = unsafe {
                                live_coding::HotFn::from_address(new_state.update_address)
                            };
                            compute_fn = unsafe {
                                live_coding::HotFn::from_address(new_state.compute_address)
                            };

                            // Nothing references the old DLL any more — free it
                            // and clean up its temp file.
                            unsafe { live_coding::unload_library(base_dll_state.handle) };
                            let _ = std::fs::remove_file(&base_dll_state.temp_path);
                            base_dll_state = new_state;

                            println!(
                                "    Loaded fresh DLL, swapped pointers, freed old in {:?}",
                                build_start.elapsed()
                            );
                            patched += 1;

                            // The rebuilt DLL now matches the current source for
                            // EVERY extracted function — commit them all to the
                            // cache so a re-save of the same bodies is treated
                            // as unchanged.
                            for entry in &funcs {
                                upsert_cached_func(&mut cached_funcs, entry.clone());
                            }
                        }
                        Err(e) => {
                            // The old DLL was never unloaded — keep running it.
                            println!("    ? load-alongside failed: {e}");
                            println!("    Kept previous DLL running (nothing was unloaded).");
                        }
                    }
                }
                Err(errors) => {
                    // The build failed and the old DLL was never unloaded —
                    // the game keeps running the last good code untouched.
                    println!("    ? cargo build FAILED.  Compiler output:\n{errors}");
                    println!("    Kept previous DLL running (nothing was unloaded).");
                }
            }
        }

        println!(
            "  -- {patched} patched, {skipped} unchanged, {absorbed_by_rebuild} handled by rebuild -- total {:?} --\n",
            total_start.elapsed()
        );
    }
}

/// One iteration of game logic — calls update and compute through PLAIN
/// function pointers.  No wrapper: if a prologue was patched, the same
/// pointers transparently execute the new code.
fn game_tick(
    iteration: u64,
    update_fn: live_coding::HotFn<UpdateFn>,
    compute_fn: live_coding::HotFn<ComputeFn>,
) {
    let tick_val = iteration as i32;

    unsafe { update_fn.call(tick_val) };

    let result = unsafe { compute_fn.call(tick_val, 3) };
    println!("    compute({tick_val}, 3) = {result}");
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Recursively collect every `.rs` file under `root` (e.g. patch_dll/src/).
fn collect_source_files(root: &std::path::Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(directory) = stack.pop() {
        if let Ok(entries) = std::fs::read_dir(&directory) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    files.push(path);
                }
            }
        }
    }
    files.sort();
    files
}

/// All files whose changes can affect the build: every `.rs` under
/// `patch_src_root` plus `Cargo.toml` and `build.rs` in `patch_dll_root`.
fn collect_watch_files(
    patch_src_root: &std::path::Path,
    patch_dll_root: &std::path::Path,
) -> Vec<PathBuf> {
    let mut files = collect_source_files(patch_src_root);
    for extra in [
        patch_dll_root.join("Cargo.toml"),
        patch_dll_root.join("build.rs"),
    ] {
        if extra.exists() {
            files.push(extra);
        }
    }
    files.sort();
    files
}

/// Return `source` with the BODY of every `pub extern "C" fn` removed (the
/// signatures are kept).  Diffing two versions of this stripped text tells us
/// whether only function bodies changed (fast prologue-patch) or whether
/// non-body content — structs, internal helpers, attributes, `mod` decls —
/// also changed (which requires a full rebuild).
fn strip_extern_bodies(source: &str) -> String {
    let mask = code_mask(source);
    let bytes = source.as_bytes();
    let mut result = String::new();
    let mut i = 0usize;
    let n = bytes.len();

    while let Some(start) = find_extern_marker(bytes, &mask, i) {
        // Keep everything before the function.
        result.push_str(&source[i..start]);

        // Find the body opening brace (code only — skips strings/comments).
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
            // No body found — keep the remainder as-is.
            result.push_str(&source[start..]);
            return result;
        };

        // Keep the signature through the opening brace.
        result.push_str(&source[start..=open]);

        // Drop the body up to the matching closing brace (code only).
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
            None => return result, // unterminated; keep what we have
        }
    }

    result.push_str(&source[i..]);
    result
}

/// Insert or replace a function entry in the body cache.
///
/// The cache maps function name → (params, body, ret_type).  Only entries
/// whose changes were applied successfully are committed; this ensures a
/// failed change is re-attempted on the next save.
fn upsert_cached_func(
    cache: &mut Vec<(String, String, String, String)>,
    entry: (String, String, String, String),
) {
    if let Some(slot) = cache.iter_mut().find(|(name, _, _, _)| *name == entry.0) {
        *slot = entry;
    } else {
        cache.push(entry);
    }
}

/// Lexical mask: `true` where `source` is real code, `false` inside string
/// literals, char literals, or comments (con 6: mini-lexer).  Brace counting
/// and marker searches only look at code positions, so braces inside strings
/// or comments can never confuse the extractor.
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
                // Line comment `//`.
                if b == b'/' && i + 1 < n && bytes[i + 1] == b'/' {
                    mask[i] = false;
                    mask[i + 1] = false;
                    state = State::LineComment;
                    i += 2;
                    continue;
                }
                // Block comment `/* ... */` (nesting-aware).
                if b == b'/' && i + 1 < n && bytes[i + 1] == b'*' {
                    mask[i] = false;
                    mask[i + 1] = false;
                    state = State::BlockComment(1);
                    i += 2;
                    continue;
                }
                // String literal `"..."` (handles backslash escapes).
                if b == b'"' {
                    mask[i] = false;
                    state = State::StringLit;
                    i += 1;
                    continue;
                }
                // Raw / byte string: r"...", r#"..."#, b"...", br#"..."#.
                if (b == b'r' || b == b'b') && i + 1 < n {
                    let mut j = i + 1;
                    let mut hashes = 0usize;
                    while j < n && bytes[j] == b'#' {
                        hashes += 1;
                        j += 1;
                    }
                    if j < n && bytes[j] == b'"' {
                        // Mask the prefix (r / b, #'s, opening quote).
                        for k in i..=j {
                            mask[k] = false;
                        }
                        // Find closing `"` followed by `hashes` #'s.
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

/// Find `pub extern "C" fn ` at or after `from`, where the leading `pub extern `
/// is real code (the quoted `"C"` part is lexed as a string literal, so it is
/// matched verbatim rather than via the code mask).
fn find_extern_marker(haystack: &[u8], mask: &[bool], from: usize) -> Option<usize> {
    let code_part = b"pub extern ";
    let quoted_part = b"\"C\" fn ";
    let total = code_part.len() + quoted_part.len();
    let n = haystack.len();
    let mut i = from;
    while i + total <= n {
        if &haystack[i..i + code_part.len()] == code_part
            && mask[i..i + code_part.len()].iter().all(|&is_code| is_code)
            && &haystack[i + code_part.len()..i + total] == quoted_part
        {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Lexically-aware extractor: `pub extern "C" fn NAME(params) [-> Ret] { body }`.
/// Returns `Vec<(function_name, params, body_text, return_type)>`.  Braces and
/// markers inside strings/comments are ignored via [`code_mask`].
fn extract_function_bodies(source: &str) -> Vec<(String, String, String, String)> {
    let mask = code_mask(source);
    let bytes = source.as_bytes();
    let marker_total = b"pub extern ".len() + b"\"C\" fn ".len();
    let mut results = Vec::new();
    let mut i = 0usize;
    let n = bytes.len();

    while let Some(start) = find_extern_marker(bytes, &mask, i) {
        // Name: up to the first `(`.
        let name_start = start + marker_total;
        let mut name_end = name_start;
        while name_end < n && bytes[name_end] != b'(' {
            name_end += 1;
        }
        let name = source[name_start..name_end].trim().to_string();

        // Params: balanced parens (code only).
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

        // Return type: text between `)` and the body `{`.
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

        // Body: balanced braces (code only).
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
