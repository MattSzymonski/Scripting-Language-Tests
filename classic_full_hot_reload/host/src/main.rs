// host — Classic full hot-reload host.
//
// Watches game_lib/src/lib.rs for changes.  When a change is detected:
//   1. cargo build -p game_lib --target-dir target_reload   (recompile)
//   2. Copy target_reload/debug/game_lib.dll → game_v{N}.dll
//   3. Print DLL diagnostics (size, export count, top exports)
//   4. LoadLibraryW(game_v{N}.dll)
//   5. GetProcAddress for update + compute
//   6. Call the new functions on every tick
//
// The old DLL stays loaded (Windows reference counting).
// DLL names are incremental: game_v1.dll, game_v2.dll, ...

use std::os::windows::ffi::OsStrExt;
use std::time::Instant;

mod dll_analysis;
use dll_analysis::inspect_dll;

// ---------------------------------------------------------------------------
// Windows FFI
// ---------------------------------------------------------------------------

unsafe extern "system" {
    fn LoadLibraryW(lpFileName: *const u16) -> *mut std::ffi::c_void;
    fn GetProcAddress(
        hModule: *mut std::ffi::c_void,
        lpProcName: *const u8,
    ) -> *mut std::ffi::c_void;
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn file_modified_time(path: &std::path::Path) -> Option<std::time::SystemTime> {
    std::fs::metadata(path).ok().and_then(|m| m.modified().ok())
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn main() {
    let workspace_root = std::env::current_dir().unwrap();
    let game_source = workspace_root.join("game_lib").join("src").join("lib.rs");
    let test_source = workspace_root.join("game_lib").join("src").join("test.rs");
    let exe_dir = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let target_dir = workspace_root.join("target_reload");
    let built_dll = target_dir.join("debug").join("game_lib.dll");

    println!("=== Classic Full Hot Reload ===\n");
    println!("  Watching: {}", game_source.display());
    println!("  Watching: {}", test_source.display());
    println!("  DLLs go to: {}\\game_vN.dll\n", exe_dir.display());
    println!("  Edit game_lib/src/lib.rs (or test.rs), save, watch it reload.\n");

    let mut last_source_time = file_modified_time(&game_source);
    let mut last_test_source_time = file_modified_time(&test_source);
    let mut dll_counter: u64 = 0;
    let mut tick: u32 = 0;

    // Function pointers to the currently loaded DLL.
    let mut current_update: Option<unsafe extern "C" fn(i32)> = None;
    let mut current_compute: Option<unsafe extern "C" fn(i32, i32) -> i32> = None;

    // Initial build and load.
    print!("  Building game_lib... ");
    rebuild_game_lib(&target_dir);
    dll_counter += 1;
    let dll_path = copy_dll(&built_dll, &exe_dir, dll_counter);
    // --- DLL diagnostics (initial load) ---
    println!("{}", inspect_dll(&dll_path, &target_dir));
    match load_game_functions(&dll_path) {
        Some((update, compute)) => {
            current_update = Some(update);
            current_compute = Some(compute);
            println!("loaded {}\n", dll_path.display());
        }
        None => println!("failed to load\n"),
    }

    loop {
        tick += 1;

        // --- Call the currently loaded game functions ---
        if let Some(update_fn) = current_update {
            unsafe { update_fn(tick as i32) };
        }
        if let Some(compute_fn) = current_compute {
            let result = unsafe { compute_fn(tick as i32, 3) };
            println!("        compute({}, 3) = {}", tick, result);
        }

        std::thread::sleep(std::time::Duration::from_millis(2000));

        // --- Check for source changes ---
        let current_time = file_modified_time(&game_source);
        let current_test_time = file_modified_time(&test_source);
        let source_changed = current_time != last_source_time;
        let test_changed = current_test_time != last_test_source_time;

        if source_changed || test_changed {
            last_source_time = current_time;
            last_test_source_time = current_test_time;

            let which = if source_changed && test_changed {
                "lib.rs & test.rs"
            } else if source_changed {
                "lib.rs"
            } else {
                "test.rs"
            };
            println!("\n  ── Source changed ({which}, tick {tick}) — rebuilding game_lib... ──");

            let start = Instant::now();
            rebuild_game_lib(&target_dir);
            let build_time = start.elapsed();

            dll_counter += 1;
            let dll_path = copy_dll(&built_dll, &exe_dir, dll_counter);

            // --- DLL diagnostics ---
            println!("{}", inspect_dll(&dll_path, &target_dir));

            print!("  Loading {}... ", dll_path.display());
            let load_start = Instant::now();
            match load_game_functions(&dll_path) {
                Some((update, compute)) => {
                    current_update = Some(update);
                    current_compute = Some(compute);
                    println!(
                        "✓ build {build_time:?} + load {:?}  (game_v{dll_counter}.dll)\n",
                        load_start.elapsed()
                    );
                }
                None => println!("✗ failed\n"),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Build, copy, load
// ---------------------------------------------------------------------------

/// Run `cargo build -p game_lib --target-dir <dir>`.
fn rebuild_game_lib(target_dir: &std::path::Path) {
    let output = std::process::Command::new("cargo")
        .args(["build", "-p", "game_lib", "--target-dir"])
        .arg(target_dir)
        .output()
        .expect("failed to run cargo build");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("  cargo build failed:\n{stderr}");
    }
}

/// Copy `source.dll` → `dest_dir/game_v{counter}.dll`.
fn copy_dll(
    source: &std::path::Path,
    dest_dir: &std::path::Path,
    counter: u64,
) -> std::path::PathBuf {
    let dest = dest_dir.join(format!("game_v{counter}.dll"));
    std::fs::copy(source, &dest).expect("failed to copy DLL");
    dest
}

/// Load `game_vN.dll` and resolve `update` + `compute`.
fn load_game_functions(
    dll_path: &std::path::Path,
) -> Option<(
    unsafe extern "C" fn(i32),
    unsafe extern "C" fn(i32, i32) -> i32,
)> {
    let wide: Vec<u16> = dll_path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let handle = unsafe { LoadLibraryW(wide.as_ptr()) };
    if handle.is_null() {
        return None;
    }

    let update_name = std::ffi::CString::new("update").ok()?;
    let compute_name = std::ffi::CString::new("compute").ok()?;

    let update_ptr = unsafe { GetProcAddress(handle, update_name.as_ptr() as *const u8) };
    let compute_ptr = unsafe { GetProcAddress(handle, compute_name.as_ptr() as *const u8) };

    if update_ptr.is_null() || compute_ptr.is_null() {
        return None;
    }

    let update_fn: unsafe extern "C" fn(i32) = unsafe { std::mem::transmute(update_ptr) };
    let compute_fn: unsafe extern "C" fn(i32, i32) -> i32 =
        unsafe { std::mem::transmute(compute_ptr) };

    Some((update_fn, compute_fn))
}
