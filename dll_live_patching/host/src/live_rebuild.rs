// live_rebuild.rs — Live source-change patching (Path B)
//
// Watches plugin/src/lib.rs for changes. On each save, rebuilds patch_plugin
// via cargo, reloads the new DLL, and re-applies the jump table — all without
// restarting the host process.

use crate::jump_table::{apply_dll_patch, call_hot_add};
use crate::thinlink;
use crate::types::{AddressMap, LocalJumpTable, RegisteredModule};
use crate::windows_ffi;
use std::ffi::{c_void, CString};
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

pub fn run(
    original_mod: &RegisteredModule,
    add_v1_rva: u64,
    sentinel_rva_original: u64,
    sentinel_rva_patch: u64,
) {
    use notify::event::EventKind;
    use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

    let plugin_src = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().join("plugin").join("src");
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
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

    watcher.watch(&plugin_src, RecursiveMode::NonRecursive).expect("watch plugin/src");
    println!("[watch] monitoring {}\n", plugin_src.display());

    let mut patch_number = 1u32;

    for () in rx {
        std::thread::sleep(Duration::from_millis(300));
        println!("[rebuild #{patch_number}] source change detected, rebuilding patch_plugin...");

        // 1. Rebuild — use thinlink if enabled, otherwise cargo
        let use_thinlink = std::env::var("DLL_THINLINK").is_ok();
        let new_name;
        let mut new_path;

        if use_thinlink {
            new_name = format!("patch_plugin_v{patch_number}.dll");
            match thinlink::rebuild_patch_direct(&workspace_root, patch_number) {
                Ok(path) => {
                    new_path = path;
                    println!("[thinlink #{patch_number}] rebuilt via rustc-direct");
                }
                Err(e) => {
                    eprintln!("[thinlink #{patch_number}] failed: {e} — falling back to cargo");
                    let status = Command::new("cargo").args(["build", "-p", "patch_plugin"])
                        .current_dir(&workspace_root).status().expect("cargo build");
                    if !status.success() { eprintln!("[rebuild #{patch_number}] build failed"); continue; }
                    new_path = build_dir.join(format!("patch_plugin.dll"));
                    std::fs::copy(&new_path, build_dir.join(&new_name)).expect("copy");
                    new_path = build_dir.join(&new_name);
                }
            }
        } else {
            let status = Command::new("cargo").args(["build", "-p", "patch_plugin"])
                .current_dir(&workspace_root).status().expect("cargo build");
            if !status.success() { eprintln!("[rebuild #{patch_number}] build failed"); continue; }
            new_name = format!("patch_plugin_v{patch_number}.dll");
            new_path = build_dir.join(&new_name);
            std::fs::copy(build_dir.join("patch_plugin.dll"), &new_path).expect("copy DLL");
            let _ = std::fs::copy(build_dir.join("patch_plugin.pdb"), build_dir.join(format!("patch_plugin_v{patch_number}.pdb")));
            println!("[rebuild #{patch_number}] copied to {new_name}");
        }

        // 3. Load the new patch DLL
        let patch_lib = unsafe { libloading::Library::new(&new_path).expect("load rebuilt DLL") };
        let patch_lib = Box::leak(Box::new(patch_lib));

        // 4. Wire trampoline
        let set_trampoline: libloading::Symbol<extern "C" fn(*const ())> = unsafe {
            patch_lib.get(b"set_get_call_count_trampoline").unwrap()
        };
        let inc_cstr = CString::new("increment_call_count").unwrap();
        let original_lib_cstr = CString::new("plugin.dll").unwrap();
        let original_hmod = unsafe { windows_ffi::GetModuleHandleA(original_lib_cstr.as_ptr() as *const u8) };
        let inc_ptr = unsafe { windows_ffi::GetProcAddress(original_hmod, inc_cstr.as_ptr() as *const u8) };
        set_trampoline(inc_ptr as *const ());

        // 5. Resolve new function addresses
        let add_v2: libloading::Symbol<extern "C" fn(i32, i32) -> i32> = unsafe { patch_lib.get(b"add_v2").unwrap() };
        let patch_name_cstr = CString::new(new_name.as_str()).unwrap();
        let patch_hmod = unsafe { windows_ffi::GetModuleHandleA(patch_name_cstr.as_ptr() as *const u8) };
        let mut modinfo: windows_ffi::MODULEINFO = unsafe { std::mem::zeroed() };
        unsafe {
            windows_ffi::GetModuleInformation(
                windows_ffi::GetCurrentProcess(), patch_hmod,
                &mut modinfo, std::mem::size_of::<windows_ffi::MODULEINFO>() as u32,
            );
        }
        let add_v2_ptr = *add_v2 as *const c_void;
        let add_v2_rva_new = (add_v2_ptr as usize - modinfo.lpBaseOfDll as usize) as u64;

        // 6. Build and apply new JumpTable
        let mut table = LocalJumpTable {
            map: { let mut m = AddressMap::default(); m.insert(add_v1_rva, add_v2_rva_new); m },
            sentinel_rva_original,
            sentinel_rva_patch,
        };
        apply_dll_patch(original_mod, &mut table).expect("live apply_dll_patch");

        // 7. Verify
        let result = unsafe { call_hot_add(10, 20) };
        println!("[rebuild #{patch_number}] ✓ applied — call_hot_add(10, 20) = {result}\n");
        patch_number += 1;
    }
}
