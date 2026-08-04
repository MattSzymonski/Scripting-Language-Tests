// rebuild.rs — Live rebuild support (optional feature)
//
// File watcher + cargo/rustc rebuild loop for development workflows.

use crate::engine::{apply_dll_patch_named, call_hot};
use crate::registry::{get_module_handle, windows_ffi};
use crate::types::{AddressMap, JumpTable, RegisteredModule};
use notify::event::EventKind;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// Run a file-watching rebuild loop. Monitors a source directory for changes,
/// rebuilds the patch DLL, reloads it, and re-applies the jump table.
///
/// When `DLL_THINLINK=1` is set, uses rustc-direct compilation instead of
/// full `cargo build` (demonstrating the ThinLink concept).
pub fn run(
    watch_dir: &Path,
    workspace_root: &Path,
    patch_crate_name: &str,
    original_mod: &RegisteredModule,
    function_id: u64,
    sentinel_rva_original: u64,
    sentinel_rva_patch: u64,
) {
    let build_dir = workspace_root.join("target").join("debug");
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                if matches!(event.kind, EventKind::Modify(_)) {
                    let _ = tx.send(());
                }
            }
        },
        Config::default(),
    )
    .expect("file watcher");

    watcher
        .watch(watch_dir, RecursiveMode::NonRecursive)
        .expect("watch directory");
    println!("[watch] monitoring {}\n", watch_dir.display());

    let mut patch_number = 1u32;
    let use_thinlink = std::env::var("DLL_THINLINK").is_ok();

    for () in rx {
        std::thread::sleep(Duration::from_millis(300));
        println!("[rebuild #{patch_number}] change detected, rebuilding...");

        let mut new_name;
        let mut new_path;
        let patch_dll_name;

        if use_thinlink {
            new_name = format!("{patch_crate_name}_v{patch_number}.dll");
            match rebuild_direct(workspace_root, patch_crate_name, patch_number) {
                Ok(path) => {
                    new_path = path;
                    patch_dll_name = new_name.clone();
                    println!("[thinlink #{patch_number}] rebuilt via rustc-direct");
                }
                Err(e) => {
                    eprintln!("[thinlink #{patch_number}] {e} — falling back to cargo");
                    if !cargo_build(workspace_root, patch_crate_name) {
                        continue;
                    }
                    new_path = build_dir.join(format!("{patch_crate_name}.dll"));
                    new_name = format!("{patch_crate_name}_v{patch_number}.dll");
                    let dest = build_dir.join(&new_name);
                    std::fs::copy(&new_path, &dest).expect("copy");
                    new_path = dest;
                    patch_dll_name = new_name.clone();
                }
            }
        } else {
            if !cargo_build(workspace_root, patch_crate_name) {
                continue;
            }
            new_name = format!("{patch_crate_name}_v{patch_number}.dll");
            new_path = build_dir.join(&new_name);
            let src = build_dir.join(format!("{patch_crate_name}.dll"));
            std::fs::copy(&src, &new_path).expect("copy DLL");
            let _ = std::fs::copy(
                build_dir.join(format!("{patch_crate_name}.pdb")),
                build_dir.join(format!("{patch_crate_name}_v{patch_number}.pdb")),
            );
            patch_dll_name = new_name.clone();
            println!("[rebuild #{patch_number}] copied to {new_name}");
        }

        // Load the new patch DLL
        let patch_lib =
            unsafe { libloading::Library::new(&new_path).expect("load rebuilt DLL") };
        let patch_lib = Box::leak(Box::new(patch_lib));

        // Wire trampoline if supported
        if let Ok(set_trampoline) =
            unsafe { patch_lib.get::<extern "C" fn(*const ())>(b"set_get_call_count_trampoline") }
        {
            let inc_cstr = CString::new("increment_call_count").unwrap();
            let orig_hmod = get_module_handle("plugin.dll").unwrap();
            let inc_ptr = unsafe {
                windows_ffi::GetProcAddress(orig_hmod, inc_cstr.as_ptr() as *const u8)
            };
            set_trampoline(inc_ptr as *const ());
        }

        // Resolve new function address
        let add_v2 = unsafe {
            patch_lib
                .get::<extern "C" fn(i32, i32) -> i32>(b"add_v2")
                .unwrap()
        };
        let patch_hmod = get_module_handle(&patch_dll_name).unwrap();
        let mut modinfo: windows_ffi::MODULEINFO = unsafe { std::mem::zeroed() };
        let size = std::mem::size_of::<windows_ffi::MODULEINFO>() as u32;
        unsafe {
            windows_ffi::GetModuleInformation(
                windows_ffi::GetCurrentProcess(),
                patch_hmod,
                &mut modinfo,
                size,
            );
        }
        let new_base = modinfo.lpBaseOfDll as usize;
        let add_v2_rva = (*add_v2 as *const () as usize - new_base) as u64;

        // Apply
        let mut table = JumpTable {
            map: {
                let mut m = AddressMap::default();
                m.insert(function_id, add_v2_rva);
                m
            },
            sentinel_rva_original,
            sentinel_rva_patch,
        };
        apply_dll_patch_named(original_mod, &mut table, &patch_dll_name)
            .expect("live apply_dll_patch");

        let result: i32 = unsafe { call_hot(function_id, (10i32, 20i32)) };
        println!("[rebuild #{patch_number}] ✓ applied — call_hot(10, 20) = {result}\n");
        patch_number += 1;
    }
}

fn cargo_build(workspace_root: &Path, crate_name: &str) -> bool {
    let status = Command::new("cargo")
        .args(["build", "-p", crate_name])
        .current_dir(workspace_root)
        .status()
        .expect("cargo build");
    if !status.success() {
        eprintln!("[rebuild] cargo build failed");
    }
    status.success()
}

fn rebuild_direct(
    workspace_root: &Path,
    crate_name: &str,
    patch_number: u32,
) -> Result<PathBuf, String> {
    let build_dir = workspace_root.join("target").join("debug");
    let output_name = format!("{crate_name}_v{patch_number}.dll");
    let output_path = build_dir.join(&output_name);
    let patch_src = workspace_root
        .join(crate_name)
        .join("src")
        .join("lib.rs");

    // Compile to object
    let object_file = build_dir.join(format!("{crate_name}_v{patch_number}.o"));
    let mut compile = Command::new("rustc");
    compile
        .arg(&patch_src)
        .arg("--crate-name")
        .arg(crate_name)
        .arg("--crate-type")
        .arg("cdylib")
        .arg("--edition")
        .arg("2024")
        .arg("--emit")
        .arg("obj")
        .arg("-o")
        .arg(&object_file)
        .arg("-L")
        .arg(&build_dir)
        .arg("--out-dir")
        .arg(&build_dir)
        .current_dir(workspace_root);

    let status = compile
        .status()
        .map_err(|e| format!("rustc spawn failed: {e}"))?;
    if !status.success() {
        return Err(format!("compilation failed"));
    }

    // Link to DLL
    let mut link = Command::new("rustc");
    link.arg(&object_file)
        .arg("--crate-name")
        .arg(crate_name)
        .arg("--crate-type")
        .arg("cdylib")
        .arg("--edition")
        .arg("2024")
        .arg("-o")
        .arg(&output_path)
        .arg("-L")
        .arg(&build_dir)
        .current_dir(workspace_root);

    let status = link.status().map_err(|e| format!("link failed: {e}"))?;
    if !status.success() {
        return Err(format!("linking failed"));
    }

    Ok(output_path)
}
