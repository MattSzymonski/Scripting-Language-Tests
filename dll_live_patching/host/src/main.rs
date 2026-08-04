// host/src/main.rs — DLL Live-Patching MVP
//
// Demonstrates: cross-module redirection, PDB-based symbol discovery,
// Subsecond runtime integration, stub trampolines, multi-function jump table,
// and live source-change rebuild — all on Windows with ASLR.

mod windows_ffi;
mod types;
mod registry;
mod jump_table;

#[cfg(feature = "pdb")]
mod pdb_resolver;
#[cfg(feature = "subsecond")]
mod subsecond_integration;

mod live_rebuild;
mod thinlink;

use std::ffi::c_void;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use types::{AddressMap, LocalJumpTable};

// ── Helpers ──────────────────────────────────────────────────────────────

fn find_dll(name: &str) -> PathBuf {
    let exe_dir = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
    let candidates = [
        exe_dir.join(name),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("target").join("debug").join(name),
    ];
    for c in &candidates { if c.exists() { return c.clone(); } }
    panic!("{name} not found. Build: cargo build -p {}", name.trim_end_matches(".dll"));
}

// ── Main Demonstration ───────────────────────────────────────────────────

fn main() {
    println!("=== DLL Live-Patching MVP ===\n");

    let plugin_path = find_dll("plugin.dll");
    let patch_path  = find_dll("patch_plugin.dll");
    println!("[init] original DLL : {}", plugin_path.display());
    println!("[init] patch DLL    : {}", patch_path.display());

    // Load both DLLs
    let original_lib = Box::leak(Box::new(unsafe { libloading::Library::new(&plugin_path).expect("load plugin.dll") }));
    let patch_lib    = Box::leak(Box::new(unsafe { libloading::Library::new(&patch_path).expect("load patch_plugin.dll") }));

    // Register both modules
    let original_mod = registry::register_library("plugin.dll", "__subsecond_anchor").expect("register plugin.dll");
    let patch_mod    = registry::register_library("patch_plugin.dll", "__subsecond_anchor").expect("register patch_plugin.dll");
    println!("[registry] plugin.dll       base {:#x}, sentinel {:#x}", original_mod.base_address, original_mod.sentinel_address);
    println!("[registry] patch_plugin.dll base {:#x}, sentinel {:#x}", patch_mod.base_address, patch_mod.sentinel_address);

    // Resolve exported functions
    let add_v1: libloading::Symbol<extern "C" fn(i32, i32) -> i32> = unsafe { original_lib.get(b"add_v1").expect("add_v1") };
    let get_call_count: libloading::Symbol<extern "C" fn() -> u32> = unsafe { original_lib.get(b"get_call_count").expect("get_call_count") };
    let add_v2: libloading::Symbol<extern "C" fn(i32, i32) -> i32> = unsafe { patch_lib.get(b"add_v2").expect("add_v2") };
    let add_v3: libloading::Symbol<extern "C" fn(i32, i32) -> i32> = unsafe { patch_lib.get(b"add_v3").expect("add_v3") };

    let add_v1_ptr = *add_v1 as *const c_void;
    let add_v2_ptr = *add_v2 as *const c_void;
    let add_v3_ptr = *add_v3 as *const c_void;

    // Wire trampoline: patch DLL → original DLL increment_call_count
    let set_trampoline: libloading::Symbol<extern "C" fn(*const ())> = unsafe { patch_lib.get(b"set_get_call_count_trampoline").unwrap() };
    let increment_call_count: libloading::Symbol<extern "C" fn() -> u32> = unsafe { original_lib.get(b"increment_call_count").unwrap() };
    set_trampoline(*increment_call_count as *const ());
    println!("[trampoline] wired: patch DLL → original increment_call_count @ {:#x}", *increment_call_count as *const () as usize);

    println!("\n[symbols] add_v1 @ {:#x}  (plugin.dll)", add_v1_ptr as usize);
    println!("[symbols] add_v2 @ {:#x}  (patch_plugin.dll)", add_v2_ptr as usize);
    println!("[symbols] add_v3 @ {:#x}  (patch_plugin.dll)", add_v3_ptr as usize);

    // Compute ASLR slides
    let sentinel_rva_original = registry::resolve_rva("plugin.dll", "__subsecond_anchor", original_mod.base_address).unwrap();
    let sentinel_rva_patch    = registry::resolve_rva("patch_plugin.dll", "__subsecond_anchor", patch_mod.base_address).unwrap();
    println!("\n[aslr] plugin.dll       slide = {:#x}", registry::compute_aslr_slide(&original_mod, sentinel_rva_original));
    println!("[aslr] patch_plugin.dll slide = {:#x}", registry::compute_aslr_slide(&patch_mod, sentinel_rva_patch));

    // Compute RVAs
    let add_v1_rva = (add_v1_ptr as usize - original_mod.base_address) as u64;
    let add_v2_rva = (add_v2_ptr as usize - patch_mod.base_address) as u64;
    let add_v3_rva = (add_v3_ptr as usize - patch_mod.base_address) as u64;

    // ── Step 1: Cross-Module Redirection ────────────────────────────────
    println!("\n━━━ Step 1: Cross-Module Redirection ━━━\n");

    jump_table::init_hot_function(add_v1_ptr);

    let before = unsafe { jump_table::call_hot_add(2, 3) };
    let count_before = get_call_count();
    println!("[before patch] call_hot_add(2, 3) = {before}  (expected: 5), call_count = {count_before}");
    assert_eq!(before, 5);

    // Patch 1: add_v1 → add_v2
    let mut table = LocalJumpTable {
        map: { let mut m = AddressMap::default(); m.insert(add_v1_rva, add_v2_rva); m },
        sentinel_rva_original, sentinel_rva_patch,
    };
    println!("\n[jump_table] add_v1 (RVA {add_v1_rva:#x}) → add_v2 (RVA {add_v2_rva:#x})");
    jump_table::apply_dll_patch(&original_mod, &mut table).expect("patch 1");

    let after = unsafe { jump_table::call_hot_add(2, 3) };
    let count_after = get_call_count();
    println!("\n[after patch 1] call_hot_add(2, 3) = {after}  (expected: 6), call_count = {count_after}");
    assert_eq!(after, 6);
    assert!(count_after > count_before, "trampoline should increment original DLL's call_count");

    // Patch 2: add_v1 → add_v3
    let mut table2 = LocalJumpTable {
        map: { let mut m = AddressMap::default(); m.insert(add_v1_rva, add_v3_rva); m },
        sentinel_rva_original, sentinel_rva_patch,
    };
    jump_table::apply_dll_patch(&original_mod, &mut table2).expect("patch 2");

    let after2 = unsafe { jump_table::call_hot_add(2, 3) };
    let count_after2 = get_call_count();
    println!("\n[after patch 2] call_hot_add(2, 3) = {after2}  (expected: 7), call_count = {count_after2}");
    assert_eq!(after2, 7);
    assert!(count_after2 > count_after);
    println!("\n  ✅ Step 1 complete.");

    // ── Path C: Multi-Function Jump Table ──────────────────────────────
    println!("\n━━━ Path C: Multi-Function Jump Table ━━━\n");

    let greet: libloading::Symbol<extern "C" fn(*const std::ffi::c_char) -> i32> = unsafe { original_lib.get(b"greet").expect("greet") };
    let greet_v2: libloading::Symbol<extern "C" fn(*const std::ffi::c_char) -> i32> = unsafe { patch_lib.get(b"greet_v2").expect("greet_v2") };
    let greet_rva = (*greet as *const c_void as usize - original_mod.base_address) as u64;
    let greet_v2_rva = (*greet_v2 as *const c_void as usize - patch_mod.base_address) as u64;

    let mut multi_table = LocalJumpTable {
        map: { let mut m = AddressMap::default(); m.insert(add_v1_rva, add_v2_rva); m.insert(greet_rva, greet_v2_rva); m },
        sentinel_rva_original, sentinel_rva_patch,
    };
    jump_table::apply_dll_patch(&original_mod, &mut multi_table).expect("multi-fn patch");

    let add_result = unsafe { jump_table::call_hot_add(2, 3) };
    let greet_orig_addr = *greet as *const c_void as u64;
    let greet_result = {
        let table_ptr = jump_table::APP_JUMP_TABLE.load(Ordering::Relaxed);
        let table = unsafe { &*table_ptr };
        if let Some(new_addr) = table.map.get(&greet_orig_addr) {
            let func: extern "C" fn(*const std::ffi::c_char) -> i32 = unsafe { std::mem::transmute(*new_addr as *const ()) };
            func(std::ffi::CString::new("hello").unwrap().as_ptr())
        } else { greet(std::ffi::CString::new("hello").unwrap().as_ptr()) }
    };
    println!("[multi-fn] add(2,3) = {add_result} (expected: 6), greet(\"hello\") = {greet_result} (expected: 50)");
    assert_eq!(add_result, 6);
    assert_eq!(greet_result, 50);
    println!("  ✅ Path C complete — 2 functions, 1 jump table.");

    // ── Step 2: PDB-Based Symbol Resolution ─────────────────────────────
    println!("\n━━━ Step 2: PDB-Based Symbol Resolution ━━━\n");

    #[cfg(feature = "pdb")]
    {
        use pdb_resolver::*;
        let original_symbols = collect_public_symbols(&plugin_path).expect("PDB original");
        let patch_symbols = collect_public_symbols(&patch_path).expect("PDB patch");
        println!("[pdb] {} original symbols, {} patch symbols", original_symbols.len(), patch_symbols.len());
        assert!(original_symbols.contains_key("__subsecond_anchor"));
        assert!(original_symbols.contains_key("add_v1"));
        assert!(patch_symbols.contains_key("add_v2"));
        println!("[pdb] sentinel + functions confirmed in both PDBs ✓");

        let mut pdb_table = LocalJumpTable {
            map: { let mut m = AddressMap::default(); m.insert(original_symbols["add_v1"], patch_symbols["add_v2"]); m },
            sentinel_rva_original: original_symbols["__subsecond_anchor"],
            sentinel_rva_patch: patch_symbols["__subsecond_anchor"],
        };
        jump_table::apply_dll_patch(&original_mod, &mut pdb_table).expect("PDB patch");
        let pdb_after = unsafe { jump_table::call_hot_add(2, 3) };
        println!("\n[pdb] after PDB-informed patch: call_hot_add(2, 3) = {pdb_after} (expected: 6)");
        assert_eq!(pdb_after, 6);
        println!("  ✅ Step 2 complete.");
    }
    #[cfg(not(feature = "pdb"))] { println!("  ⚠ PDB support: cargo build -p host --features pdb"); }

    // ── Step 3: Subsecond Runtime Integration ────────────────────────────
    println!("\n━━━ Step 3: Subsecond Runtime Integration ━━━\n");

    #[cfg(feature = "subsecond")]
    {
        use subsecond_integration::*;
        let mut ss_table = LocalJumpTable {
            map: { let mut m = AddressMap::default(); m.insert(add_v1_rva, add_v2_rva); m },
            sentinel_rva_original, sentinel_rva_patch,
        };
        jump_table::apply_dll_patch(&original_mod, &mut ss_table).expect("subsecond patch");
        let _ss_jt = to_subsecond_jump_table(&ss_table);
        let ss_after = call_via_subsecond(2, 3);
        println!("[subsecond] call_via_subsecond(2, 3) = {ss_after} (expected: 6)");
        assert_eq!(ss_after, 6);
        println!("  ✅ Step 3 complete — subsecond::call() dispatch verified.");
    }
    #[cfg(not(feature = "subsecond"))] { println!("  ⚠ Subsecond support: cargo build -p host --features subsecond"); }

    // ── Step 4: Live Source-Change Patching ─────────────────────────────
    println!("\n━━━ Step 4: Live Source-Change Patching ━━━");
    println!("  Watching plugin/src/lib.rs for changes...");
    println!("  Edit, save, and the patch rebuilds + reloads automatically.");
    println!("  Press Ctrl+C to exit.\n");

    live_rebuild::run(&original_mod, add_v1_rva, sentinel_rva_original, sentinel_rva_patch);
}
