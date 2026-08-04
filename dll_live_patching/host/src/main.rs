// host/src/main.rs — DLL Live-Patching MVP
//
// Demonstrates the core mechanism from Subsecond's DLL-support design:
//   1. Load a cdylib plugin
//   2. Register the module with Windows APIs (base address, sentinel)
//   3. Compute the ASLR slide from the sentinel symbol
//   4. Build a JumpTable mapping old function RVA → new function RVA
//   5. Atomically swap the active function pointer
//   6. Verify the patched implementation is called
//
// Architecture:
//   Global JumpTable (AtomicPtr) → call sites dereference this to get the
//   current function address. Patching means swapping the pointer, never
//   modifying existing code.

use std::collections::HashMap;
use std::ffi::{c_void, CString};
use std::path::PathBuf;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::{LazyLock, Mutex};

// ---------------------------------------------------------------------------
// Windows FFI declarations (windows-sys crate)
// ---------------------------------------------------------------------------
#[cfg(windows)]
mod windows_ffi {
    use std::ffi::c_void;

    #[link(name = "kernel32")]
    unsafe extern "system" {
        pub fn GetModuleHandleA(lpModuleName: *const u8) -> *mut c_void;
        pub fn GetProcAddress(hModule: *mut c_void, lpProcName: *const u8) -> *mut c_void;
        pub fn GetCurrentProcess() -> *mut c_void;
    }

    #[link(name = "psapi")]
    unsafe extern "system" {
        pub fn GetModuleInformation(
            hProcess: *mut c_void,
            hModule: *mut c_void,
            lpmodinfo: *mut MODULEINFO,
            cb: u32,
        ) -> i32;
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    #[allow(non_snake_case)]
    pub struct MODULEINFO {
        pub lpBaseOfDll: *mut c_void,
        pub SizeOfImage: u32,
        pub EntryPoint: *mut c_void,
    }
}

// ---------------------------------------------------------------------------
// Data structures
// ---------------------------------------------------------------------------

/// Maps a function's compile-time (link-time) RVA to its replacement's RVA.
/// In Subsecond proper, this is built by diffing PDB symbol tables. Here we
/// construct it manually for the MVP.
type AddressMap = HashMap<u64, u64>;

/// A replica of Subsecond's `JumpTable` — the data structure that describes
/// a patch. It is shipped from the build system to the runtime.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct JumpTable {
    /// Path to the patch DLL (for the MVP, same as the original DLL since
    /// all function versions coexist in one module).
    patch_library_path: PathBuf,
    /// Old function RVA → new function RVA (relative to module base).
    map: AddressMap,
    /// The link-time address (RVA) of the sentinel in the original binary.
    sentinel_rva_original: u64,
    /// The link-time address (RVA) of the sentinel in the patch binary.
    sentinel_rva_patch: u64,
}

/// Information about a loaded module, stored at registration time.
#[derive(Clone, Debug)]
struct RegisteredModule {
    /// Filesystem name, e.g. "plugin.dll".
    name: String,
    /// Base address of the module image (HMODULE on Windows).
    base_address: usize,
    /// Runtime address of the sentinel function (post-ASLR).
    sentinel_address: usize,
    /// Total size of the module image in bytes.
    image_size: usize,
}

/// Errors that can occur during library registration or patching.
#[derive(Debug)]
enum PatchingError {
    LibraryNotLoaded(String),
    SentinelNotInLibrary { sentinel: usize, name: String },
    AlreadyRegistered(String),
    ModuleInfoFailed(String),
    SymbolNotFound(String),
    SentinelNotExported(String),
}

impl std::fmt::Display for PatchingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LibraryNotLoaded(name) => {
                write!(f, "library '{}' is not loaded in the current process", name)
            }
            Self::SentinelNotInLibrary { sentinel, name } => {
                write!(
                    f,
                    "sentinel address {sentinel:#x} does not belong to library '{name}'"
                )
            }
            Self::AlreadyRegistered(name) => {
                write!(f, "library '{name}' is already registered")
            }
            Self::ModuleInfoFailed(name) => {
                write!(f, "failed to query module information for '{name}'")
            }
            Self::SymbolNotFound(name) => {
                write!(f, "symbol '{name}' not found in the loaded library")
            }
            Self::SentinelNotExported(name) => {
                write!(
                    f,
                    "sentinel '{name}' not exported from the library — \
                     add #[no_mangle] pub extern \"C\" fn {name}() {{}}"
                )
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Module registry — global state tracking loaded DLLs
// ---------------------------------------------------------------------------

static MODULE_REGISTRY: LazyLock<Mutex<HashMap<String, RegisteredModule>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Register a dynamically loaded library for hot-patching.
///
/// The sentinel function must be exported with `#[no_mangle] pub extern "C"`.
/// It serves as the ASLR reference point — the same role `main` plays for
/// the host executable in Subsecond's existing design.
#[cfg(windows)]
fn register_library(
    library_name: &str,
    sentinel_symbol_name: &str,
) -> Result<RegisteredModule, PatchingError> {
    use windows_ffi::*;

    // ── Step 1: confirm the DLL is loaded ──────────────────────────────
    let name_cstr = CString::new(library_name)
        .map_err(|_| PatchingError::LibraryNotLoaded(library_name.to_string()))?;
    let hmod = unsafe { GetModuleHandleA(name_cstr.as_ptr() as *const u8) };
    if hmod.is_null() {
        return Err(PatchingError::LibraryNotLoaded(library_name.to_string()));
    }

    // ── Step 2: get module bounds ──────────────────────────────────────
    let mut modinfo: MODULEINFO = unsafe { std::mem::zeroed() };
    let ok = unsafe {
        GetModuleInformation(
            GetCurrentProcess(),
            hmod,
            &mut modinfo,
            std::mem::size_of::<MODULEINFO>() as u32,
        )
    };
    if ok == 0 {
        return Err(PatchingError::ModuleInfoFailed(library_name.to_string()));
    }

    let base_address = modinfo.lpBaseOfDll as usize;
    let image_size = modinfo.SizeOfImage as usize;

    // ── Step 3: resolve the sentinel symbol ────────────────────────────
    let sentinel_cstr = CString::new(sentinel_symbol_name)
        .map_err(|_| PatchingError::SentinelNotExported(sentinel_symbol_name.to_string()))?;
    let sentinel_ptr = unsafe { GetProcAddress(hmod, sentinel_cstr.as_ptr() as *const u8) };
    if sentinel_ptr.is_null() {
        return Err(PatchingError::SentinelNotExported(
            sentinel_symbol_name.to_string(),
        ));
    }
    let sentinel_address = sentinel_ptr as usize;

    // ── Step 4: validate sentinel belongs to this module ───────────────
    if sentinel_address < base_address || sentinel_address >= base_address + image_size {
        return Err(PatchingError::SentinelNotInLibrary {
            sentinel: sentinel_address,
            name: library_name.to_string(),
        });
    }

    // ── Step 5: store in registry ──────────────────────────────────────
    let mut registry = MODULE_REGISTRY.lock().unwrap();
    if registry.contains_key(library_name) {
        return Err(PatchingError::AlreadyRegistered(library_name.to_string()));
    }

    let module = RegisteredModule {
        name: library_name.to_string(),
        base_address,
        sentinel_address,
        image_size,
    };
    registry.insert(library_name.to_string(), module.clone());
    Ok(module)
}

/// Compute the ASLR slide for a registered module.
///
/// ASLR slide = runtime_sentinel_address - link_time_sentinel_rva
///
/// The link-time RVA is what the compiler/linker emitted (what you'd see
/// in the PDB or PE headers). The runtime address is what `GetProcAddress`
/// returns after the OS loader has randomized the module's base.
fn compute_aslr_slide(module: &RegisteredModule, link_time_sentinel_rva: u64) -> usize {
    module.sentinel_address - link_time_sentinel_rva as usize
}

/// Resolve an exported symbol's runtime address via `GetProcAddress`.
#[cfg(windows)]
fn resolve_export(
    library_name: &str,
    symbol_name: &str,
    base_address: usize,
) -> Result<usize, PatchingError> {
    use windows_ffi::*;

    let name_cstr = CString::new(library_name)
        .map_err(|_| PatchingError::LibraryNotLoaded(library_name.to_string()))?;
    let hmod = unsafe { GetModuleHandleA(name_cstr.as_ptr() as *const u8) };
    if hmod.is_null() {
        return Err(PatchingError::LibraryNotLoaded(library_name.to_string()));
    }

    let sym_cstr = CString::new(symbol_name)
        .map_err(|_| PatchingError::SymbolNotFound(symbol_name.to_string()))?;
    let ptr = unsafe { GetProcAddress(hmod, sym_cstr.as_ptr() as *const u8) };
    if ptr.is_null() {
        return Err(PatchingError::SymbolNotFound(symbol_name.to_string()));
    }

    // Return the RVA (offset from base) so we can construct the JumpTable
    // in link-time-relative terms, matching Subsecond's approach.
    Ok(ptr as usize - base_address)
}

// ---------------------------------------------------------------------------
// Global jump table — the single atomic pointer that call sites dereference
// ---------------------------------------------------------------------------

/// Holds the *current* function pointer for the hot-reloadable function.
/// On a patch, this pointer is atomically swapped to the new implementation.
///
/// In Subsecond, this is a full `HashMap<u64, u64>` (the `APP_JUMP_TABLE`).
/// For the MVP we use a single `AtomicPtr` for clarity; the concept is the
/// same — call sites read this pointer and jump to whatever address it holds.
static HOT_FUNCTION_PTR: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

/// Call the currently-active version of the hot-reloadable function.
///
/// # Safety
/// The pointer must have been initialised (via `set_initial_function`) before
/// the first call. After a patch, the pointer refers to the new implementation.
unsafe fn call_hot_add(left: i32, right: i32) -> i32 {
    let ptr = HOT_FUNCTION_PTR.load(Ordering::Relaxed);
    assert!(
        !ptr.is_null(),
        "HOT_FUNCTION_PTR was never initialised — call set_initial_function first"
    );
    let func: extern "C" fn(i32, i32) -> i32 = unsafe { std::mem::transmute(ptr) };
    func(left, right)
}

/// Set the initial function pointer (called once after loading the DLL).
fn set_initial_function(ptr: *const c_void) {
    HOT_FUNCTION_PTR.store(ptr as *mut c_void, Ordering::Relaxed);
}

/// Apply a patch — atomically swap the function pointer to the new implementation.
fn apply_patch(new_ptr: *const c_void) {
    let old = HOT_FUNCTION_PTR.swap(new_ptr as *mut c_void, Ordering::Relaxed);
    let old_addr = old as usize;
    let new_addr = new_ptr as usize;
    println!("[patch] atomic swap: {old_addr:#x} → {new_addr:#x}");
}

// ---------------------------------------------------------------------------
// Demonstration
// ---------------------------------------------------------------------------

fn main() {
    println!("=== DLL Live-Patching MVP ===\n");

    // ── 1. Locate the plugin DLL ───────────────────────────────────────
    //
    // Cargo builds workspace members into target/debug/ (or target/release/).
    // The cdylib is named `plugin.dll` on Windows.
    let plugin_path = find_plugin_dll();
    println!("[init] loading plugin from: {}", plugin_path.display());

    // ── 2. Load the DLL via libloading ─────────────────────────────────
    //
    // `libloading::Library::new` calls `LoadLibraryW` on Windows. The DLL
    // stays loaded until `Library` is dropped (which we leak intentionally
    // so function pointers remain valid).
    let library =
        unsafe { libloading::Library::new(&plugin_path).expect("failed to load plugin.dll") };
    let library = Box::leak(Box::new(library)); // keep loaded forever

    // ── 3. Register the library with Subsecond's module registry ────────
    //
    // This stores the base address and sentinel address so we can compute
    // the ASLR slide when building jump tables.
    let module_name = "plugin.dll";
    let sentinel_name = "__subsecond_anchor";

    #[cfg(windows)]
    let registered =
        register_library(module_name, sentinel_name).expect("failed to register library");

    println!(
        "[registry] {} loaded at base {:#x}, size {} bytes",
        registered.name, registered.base_address, registered.image_size
    );
    println!(
        "[registry] sentinel '{}' at runtime address {:#x}",
        sentinel_name, registered.sentinel_address
    );

    // ── 4. Resolve exported function addresses ─────────────────────────
    //
    // We use `libloading` to get typed function pointers. This internally
    // calls `GetProcAddress` on Windows.
    let add_v1: libloading::Symbol<extern "C" fn(i32, i32) -> i32> = unsafe {
        library
            .get(b"add_v1")
            .expect("add_v1 not exported from plugin")
    };
    let add_v2: libloading::Symbol<extern "C" fn(i32, i32) -> i32> = unsafe {
        library
            .get(b"add_v2")
            .expect("add_v2 not exported from plugin")
    };
    let add_v3: libloading::Symbol<extern "C" fn(i32, i32) -> i32> = unsafe {
        library
            .get(b"add_v3")
            .expect("add_v3 not exported from plugin")
    };
    let get_call_count: libloading::Symbol<extern "C" fn() -> u32> = unsafe {
        library
            .get(b"get_call_count")
            .expect("get_call_count not exported")
    };

    let add_v1_ptr = *add_v1 as *const c_void;
    let add_v2_ptr = *add_v2 as *const c_void;
    let add_v3_ptr = *add_v3 as *const c_void;

    println!(
        "[symbols] add_v1 @ {:#x}, add_v2 @ {:#x}, add_v3 @ {:#x}",
        add_v1_ptr as usize, add_v2_ptr as usize, add_v3_ptr as usize
    );

    // ── 5. Compute ASLR slide from the sentinel ────────────────────────
    //
    // We resolve the sentinel's RVA (offset from base) via GetProcAddress,
    // then compute: slide = runtime_sentinel - sentinel_rva.
    // For the MVP, sentinel_rva is the same as the resolved offset since
    // there's only one build — in production, this comes from the PDB.
    #[cfg(windows)]
    let sentinel_rva = resolve_export(module_name, sentinel_name, registered.base_address)
        .expect("failed to resolve sentinel RVA") as u64;

    let aslr_slide = compute_aslr_slide(&registered, sentinel_rva);
    println!(
        "[aslr] sentinel RVA = {sentinel_rva:#x}, runtime sentinel = {:#x}, slide = {:#x}",
        registered.sentinel_address, aslr_slide
    );

    // ── 6. Call the original function (no patch yet) ───────────────────
    //
    // Initialise the hot-function pointer so call_hot_add works.
    set_initial_function(add_v1_ptr);

    let before = unsafe { call_hot_add(2, 3) };
    let count_before = get_call_count();
    println!(
        "\n[before patch] call_hot_add(2, 3) = {before}  (expected: 5)\n  call_count = {count_before}"
    );
    assert_eq!(before, 5, "original add_v1 should return a + b = 5");

    // ── 7. Simulate building a JumpTable ───────────────────────────────
    //
    // In Subsecond, the CLI diffs the original PDB against the patch PDB
    // to build this map. Here we construct it manually.
    let add_v1_rva = (add_v1_ptr as usize - registered.base_address) as u64;
    let add_v2_rva = (add_v2_ptr as usize - registered.base_address) as u64;

    let jump_table = JumpTable {
        patch_library_path: plugin_path.clone(),
        map: {
            let mut map = AddressMap::default();
            map.insert(add_v1_rva, add_v2_rva);
            map
        },
        sentinel_rva_original: sentinel_rva,
        sentinel_rva_patch: sentinel_rva, // same DLL, same sentinel RVA
    };

    println!("\n[jump_table] mapping add_v1 RVA {add_v1_rva:#x} → add_v2 RVA {add_v2_rva:#x}");
    println!(
        "[jump_table] sentinel RVA (original): {:#x}, sentinel RVA (patch): {:#x}",
        jump_table.sentinel_rva_original, jump_table.sentinel_rva_patch
    );

    // ── 8. Apply the first patch ───────────────────────────────────────
    //
    // In a real system, `apply_patch` would rebase all addresses in the
    // JumpTable by the ASLR slide before committing. Since both functions
    // are in the same DLL, the slide cancels out — but we still demonstrate
    // the address computation.
    apply_patch(add_v2_ptr);

    let after = unsafe { call_hot_add(2, 3) };
    let count_after = get_call_count();
    println!(
        "\n[after patch 1] call_hot_add(2, 3) = {after}  (expected: 6)\n  call_count = {count_after}"
    );
    assert_eq!(after, 6, "patched add_v2 should return a + b + 1 = 6");
    assert!(
        count_after > count_before,
        "call_count should have increased (DLL state preserved)"
    );

    // ── 9. Apply a second patch ────────────────────────────────────────
    //
    // Demonstrates that patches can be applied sequentially without
    // restarting the process.
    apply_patch(add_v3_ptr);

    let after2 = unsafe { call_hot_add(2, 3) };
    let count_after2 = get_call_count();
    println!(
        "\n[after patch 2] call_hot_add(2, 3) = {after2}  (expected: 7)\n  call_count = {count_after2}"
    );
    assert_eq!(after2, 7, "second patch add_v3 should return a + b + 2 = 7");
    assert!(
        count_after2 > count_after,
        "call_count should have increased (DLL state preserved across multiple patches)"
    );

    // ── 10. Summary ────────────────────────────────────────────────────
    println!("\n=== All assertions passed ===");
    println!("DLL state (CALL_COUNT = {count_after2}) preserved across 3 function calls.");
    println!("The hot-reloadable function was redirected twice without restarting.");
    println!("ASLR slide computed from sentinel: {aslr_slide:#x}");
    println!(
        "Module base: {:#x}, sentinel: {:#x}",
        registered.base_address, registered.sentinel_address
    );
}

/// Locate the plugin DLL relative to the host executable.
///
/// Cargo places workspace artifacts in `target/debug/` (or `target/release/`).
/// The DLL is named `plugin.dll` on Windows.
fn find_plugin_dll() -> PathBuf {
    // Try the same directory as the host executable first (for `cargo run`).
    let exe_dir = std::env::current_exe()
        .expect("failed to get exe path")
        .parent()
        .unwrap()
        .to_path_buf();

    let candidates = [
        exe_dir.join("plugin.dll"),
        // Also check the manifest dir (for `cargo run -p host` from workspace root).
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("target")
            .join("debug")
            .join("plugin.dll"),
    ];

    for candidate in &candidates {
        if candidate.exists() {
            return candidate.clone();
        }
    }

    panic!(
        "plugin.dll not found. Searched:\n  {}\n  {}\n\n\
         Build the plugin first: cargo build -p plugin",
        candidates[0].display(),
        candidates[1].display()
    );
}
