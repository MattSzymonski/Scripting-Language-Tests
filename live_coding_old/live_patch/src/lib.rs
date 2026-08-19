// REQUIREMENTS
//   Rust stable (edition 2021+).  Windows only (uses Win32 FFI).  No external
//   crate dependencies.
//
// DESCRIPTION
//   Live Coding runtime for Rust — a Live++ / Unreal-Engine-style hot patch
//   layer.  Instead of routing calls through a jump table (subsecond style),
//   this runtime patches the FUNCTION PROLOGUE in place: it overwrites the
//   first 5 bytes of the original exported function with a `jmp rel32`
//   instruction pointing at freshly compiled code.  Existing direct callers
//   keep their original function pointer and are transparently redirected —
//   no wrapper, no lookup, no runtime overhead after the patch.
//
//   A name-keyed symbol registry also lets NEW functions be compiled, loaded,
//   and registered at runtime, then resolved by name — the Rust equivalent of
//   UE Live Coding's ability to add new functions live.
//
// USAGE
//   See main.rs for the runnable host loop.  Library consumers:
//
//     let old_addr = resolve_export(base_handle, "compute")?;
//     let dll = compile_single_function("compute", "x: i32, y: i32", "...", "i32")?;
//     let handle = load_library(&dll)?;
//     let new_addr = resolve_export(handle, "compute")?;
//     unsafe { patch_prologue(old_addr, new_addr)? };   // redirect in place
//
// EXAMPLE USAGE
//   cargo run -p live_patch -- loop      # live reload loop
//   cargo run -p live_patch              # static demos incl. new-function demo
//
// --- SCRIPT ---

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

// ---------------------------------------------------------------------------
// Global state — name-keyed symbol registry
// ---------------------------------------------------------------------------

/// Name → (address, ABI signature) registry.  New functions compiled at
/// runtime are registered here and can be resolved by name, mirroring UE's
/// runtime class registry.  The signature is stored so ABI mismatches can be
/// caught at patch time (con 9).
static SYMBOL_REGISTRY: LazyLock<Mutex<HashMap<String, (u64, String)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Loaded patch-DLL (handle-as-usize, temp path) per function name.  Lets us
/// free the previous patch DLL when a function is re-patched (con 8: resource
/// growth).  Handles are stored as `usize` so the map is `Send`-safe.
static PATCH_DLL_INFO: LazyLock<Mutex<HashMap<String, (usize, std::path::PathBuf)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Trampoline cache keyed by jump target, so repeated far patches to the same
/// function reuse one trampoline instead of leaking a new one (con 8).
static TRAMPOLINE_CACHE: LazyLock<Mutex<HashMap<u64, u64>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Register a function with its runtime address and ABI signature.
pub fn register_function(symbol_name: &str, address: u64, signature: &str) {
    let mut registry = SYMBOL_REGISTRY.lock().unwrap();
    registry.insert(symbol_name.to_string(), (address, signature.to_string()));
}

/// Verify that `signature` matches the previously registered signature for
/// `symbol_name`.  Returns an error on mismatch so ABI mistakes are caught at
/// patch time instead of crashing the next call (con 2 / 9).
pub fn verify_signature(symbol_name: &str, signature: &str) -> Result<(), String> {
    let registry = SYMBOL_REGISTRY.lock().unwrap();
    if let Some((_, registered)) = registry.get(symbol_name) {
        if registered != signature {
            return Err(format!(
                "ABI mismatch for '{symbol_name}': registered '{registered}', new '{signature}'"
            ));
        }
    }
    Ok(())
}

/// Register a symbol (name → address) without a signature (compat shim).
pub fn register_symbol(symbol_name: &str, address: u64) {
    register_function(symbol_name, address, "");
}

/// Resolve a previously registered symbol by name.  Returns `None` if the
/// name was never registered.
pub fn resolve_symbol(symbol_name: &str) -> Option<u64> {
    SYMBOL_REGISTRY
        .lock()
        .unwrap()
        .get(symbol_name)
        .map(|(address, _)| *address)
}

/// Resolve a symbol together with its registered ABI signature.
pub fn resolve_function(symbol_name: &str) -> Option<(u64, String)> {
    SYMBOL_REGISTRY.lock().unwrap().get(symbol_name).cloned()
}

// ---------------------------------------------------------------------------
// Typed function-pointer wrapper — contains the unsafe transmute in one place
// ---------------------------------------------------------------------------

/// A typed wrapper around a raw function address.  The signature is encoded
/// in the type parameter, so the host can't accidentally call a hot function
/// with the wrong ABI.  All `transmute`s happen here, in one audited spot
/// (con 9).
#[derive(Clone, Copy)]
pub struct HotFn<Signature> {
    address: u64,
    marker: std::marker::PhantomData<Signature>,
}

impl<Signature> HotFn<Signature> {
    /// Wrap a raw function address.
    ///
    /// # Safety
    ///
    /// `address` must point to a function whose ABI matches `Signature`, and
    /// the module it lives in must remain loaded while this value is used.
    pub unsafe fn from_address(address: u64) -> Self {
        Self {
            address,
            marker: std::marker::PhantomData,
        }
    }

    /// The raw address this wrapper points at.
    pub fn address(&self) -> u64 {
        self.address
    }
}

// NOTE: multi-parameter fn types do not unify with `fn(Args)` where `Args` is
// a tuple in impl heads, so we provide concrete `call` impls for the exact
// signatures the host uses.

impl HotFn<unsafe extern "C" fn(i32)> {
    /// Call a `fn(i32)` with one argument.
    ///
    /// # Safety
    ///
    /// Same invariants as [`HotFn::from_address`].
    pub unsafe fn call(&self, tick: i32) {
        let function: unsafe extern "C" fn(i32) =
            unsafe { std::mem::transmute(self.address as usize) };
        unsafe { function(tick) };
    }
}

impl HotFn<unsafe extern "C" fn(i32) -> i32> {
    /// Call a `fn(i32) -> i32` with one argument.
    ///
    /// # Safety
    ///
    /// Same invariants as [`HotFn::from_address`].
    pub unsafe fn call(&self, tick: i32) -> i32 {
        let function: unsafe extern "C" fn(i32) -> i32 =
            unsafe { std::mem::transmute(self.address as usize) };
        unsafe { function(tick) }
    }
}

impl HotFn<unsafe extern "C" fn(i32, i32) -> i32> {
    /// Call a `fn(i32, i32) -> i32` with two arguments.
    ///
    /// # Safety
    ///
    /// Same invariants as [`HotFn::from_address`].
    pub unsafe fn call(&self, x: i32, y: i32) -> i32 {
        let function: unsafe extern "C" fn(i32, i32) -> i32 =
            unsafe { std::mem::transmute(self.address as usize) };
        unsafe { function(x, y) }
    }
}

// ---------------------------------------------------------------------------
// Host-owned state — survives every patch and reload (con 5)
// ---------------------------------------------------------------------------

/// Mutable game state owned by the HOST process.  Because it lives here, it
/// survives every leaf patch AND every full DLL reload — the game DLL only
/// ever holds a pointer to it (via [`HostServices`]).
#[derive(Default)]
#[repr(C)]
pub struct HostGameState {
    /// Ticks executed since the process started (never reset by reloads).
    pub total_ticks: i64,
    /// A running score the game code can mutate freely.
    pub score: i64,
}

/// Host-owned game state.
pub static HOST_STATE: LazyLock<Mutex<HostGameState>> =
    LazyLock::new(|| Mutex::new(HostGameState::default()));

unsafe extern "C" fn host_get_state() -> *mut std::ffi::c_void {
    let mut guard = HOST_STATE.lock().unwrap();
    let state_pointer: *mut HostGameState = &mut *guard;
    state_pointer as *mut std::ffi::c_void
}

/// C-ABI table of services the host hands to every loaded game DLL via its
/// `set_services` export.  The game DLL stores this pointer and calls through
/// it to reach host-owned state.
#[repr(C)]
pub struct HostServices {
    /// Returns a pointer to the host-owned [`HostGameState`].
    pub get_state: unsafe extern "C" fn() -> *mut std::ffi::c_void,
}

/// The service table handed to every loaded DLL.
pub static HOST_SERVICES: HostServices = HostServices {
    get_state: host_get_state,
};

/// Hand the host's service table to a freshly loaded DLL so its code can
/// reach host-owned state.  DLLs that do not export `set_services` (older
/// builds) are simply skipped.
///
/// # Safety
///
/// `handle` must be a valid module handle from [`load_library`].
#[cfg(windows)]
pub unsafe fn provide_services_to_dll(handle: *mut std::ffi::c_void) -> Result<(), String> {
    let Ok(set_services_address) = (unsafe { resolve_export(handle, "set_services") }) else {
        return Ok(());
    };
    let set_services: unsafe extern "C" fn(*const HostServices) =
        unsafe { std::mem::transmute(set_services_address as usize) };
    unsafe { set_services(&HOST_SERVICES) };
    Ok(())
}

// ---------------------------------------------------------------------------
// Memory-protection constants (WinNT.h)
// ---------------------------------------------------------------------------

/// PAGE_EXECUTE_READWRITE — code page made writable so we can patch it.
const PAGE_EXECUTE_READWRITE: u32 = 0x40;
/// MEM_COMMIT — commit the allocated region (VirtualAlloc).
const MEM_COMMIT: u32 = 0x1000;
/// MEM_RESERVE — reserve the region (VirtualAlloc).
const MEM_RESERVE: u32 = 0x2000;
/// The `jmp rel32` opcode (x86 / x64 near relative jump).
const JMP_REL32_OPCODE: u8 = 0xE9;
/// Number of bytes a `jmp rel32` instruction occupies.
const JMP_REL32_SIZE: usize = 5;
/// Size of an absolute trampoline: `mov rax, imm64; jmp rax` = 10 + 2 bytes.
const TRAMPOLINE_SIZE: usize = 12;
/// `mov rax, imm64` — REX.W + B8 + 8-byte immediate.
const MOV_RAX_IMM64_OPCODE: u8 = 0xB8;
/// REX.W prefix byte for `mov rax, imm64`.
const REX_W_PREFIX: u8 = 0x48;
/// `jmp rax` opcode (FF /4).
const JMP_RAX_OPCODE: u8 = 0xFF;
/// `jmp rax` ModRM byte (mod=11, reg=4, rm=0).
const JMP_RAX_MODRM: u8 = 0xE0;
/// Size of the region we try to allocate the trampoline in.
/// Memory range to search for a trampoline home: ±1 GB keeps rel32 reachable.
const TRAMPOLINE_SEARCH_STRIDE: usize = 0x4000; // 16 KB steps

// ---------------------------------------------------------------------------
// Windows FFI — raw bindings to kernel32.dll
// ---------------------------------------------------------------------------

#[cfg(windows)]
unsafe extern "system" {
    fn LoadLibraryW(lpFileName: *const u16) -> *mut std::ffi::c_void;
    fn GetProcAddress(
        hModule: *mut std::ffi::c_void,
        lpProcName: *const u8,
    ) -> *mut std::ffi::c_void;
    fn FreeLibrary(hModule: *mut std::ffi::c_void) -> i32;
    fn VirtualAlloc(
        lpAddress: *mut std::ffi::c_void,
        dwSize: usize,
        flAllocationType: u32,
        flProtect: u32,
    ) -> *mut std::ffi::c_void;
    fn VirtualProtect(
        lpAddress: *mut std::ffi::c_void,
        dwSize: usize,
        flNewProtect: u32,
        lpflOldProtect: *mut u32,
    ) -> i32;
    fn FlushInstructionCache(
        hProcess: *mut std::ffi::c_void,
        lpBaseAddress: *const std::ffi::c_void,
        dwSize: usize,
    ) -> i32;
    fn GetCurrentProcess() -> *mut std::ffi::c_void;
    // Thread enumeration (for safe patching — con 7).
    fn CreateToolhelp32Snapshot(dwFlags: u32, th32ProcessID: u32) -> *mut std::ffi::c_void;
    fn Thread32First(hSnapshot: *mut std::ffi::c_void, lpte: *mut ThreadEntry32) -> i32;
    fn Thread32Next(hSnapshot: *mut std::ffi::c_void, lpte: *mut ThreadEntry32) -> i32;
    fn OpenThread(
        dwDesiredAccess: u32,
        bInheritHandle: i32,
        dwThreadId: u32,
    ) -> *mut std::ffi::c_void;
    fn SuspendThread(hThread: *mut std::ffi::c_void) -> u32;
    fn ResumeThread(hThread: *mut std::ffi::c_void) -> u32;
    fn CloseHandle(hObject: *mut std::ffi::c_void) -> i32;
    fn GetCurrentThreadId() -> u32;
    fn GetCurrentProcessId() -> u32;
}

/// TH32CS_SNAPTHREAD — include all threads in the toolhelp snapshot.
const TH32CS_SNAPTHREAD: u32 = 0x0000_0004;
/// THREAD_SUSPEND_RESUME — access right needed to suspend/resume a thread.
const THREAD_SUSPEND_RESUME: u32 = 0x0000_0002;

/// `THREADENTRY32` from TlHelp32.h — one thread entry in a toolhelp snapshot.
#[repr(C)]
#[allow(non_snake_case)]
struct ThreadEntry32 {
    dwSize: u32,
    cntUsage: u32,
    th32ThreadID: u32,
    th32OwnerProcessID: u32,
    tpBasePri: i32,
    tpDeltaPri: i32,
    dwFlags: u32,
}

/// Handle to the currently-loaded base DLL, if any.
#[cfg(windows)]
static mut CURRENT_DLL_HANDLE: *mut std::ffi::c_void = std::ptr::null_mut();

// ---------------------------------------------------------------------------
// DLL loading / symbol resolution
// ---------------------------------------------------------------------------

/// Load a DLL into this process and return its module handle.
///
/// # Safety
///
/// The returned handle is valid until [`unload_current_dll`] is called.
#[cfg(windows)]
pub unsafe fn load_library(dll_path: &std::path::Path) -> Result<*mut std::ffi::c_void, String> {
    use std::os::windows::ffi::OsStrExt;

    let wide_path: Vec<u16> = dll_path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let module_handle = unsafe { LoadLibraryW(wide_path.as_ptr()) };
    if module_handle.is_null() {
        return Err(format!(
            "LoadLibraryW failed for '{}': {}",
            dll_path.display(),
            std::io::Error::last_os_error()
        ));
    }

    // Remember the handle so unload_current_dll can free it later.
    unsafe { CURRENT_DLL_HANDLE = module_handle };

    Ok(module_handle)
}

#[cfg(not(windows))]
pub unsafe fn load_library(_dll_path: &std::path::Path) -> Result<*mut std::ffi::c_void, String> {
    Err("load_library is only supported on Windows".to_string())
}

/// Resolve the address of an exported symbol in a loaded module.
///
/// # Safety
///
/// `module_handle` must be a valid handle from [`load_library`] and must
/// remain loaded while the returned address is used.
#[cfg(windows)]
pub unsafe fn resolve_export(
    module_handle: *mut std::ffi::c_void,
    symbol_name: &str,
) -> Result<u64, String> {
    let c_string =
        std::ffi::CString::new(symbol_name).map_err(|e| format!("Invalid symbol name: {e}"))?;
    let procedure_address =
        unsafe { GetProcAddress(module_handle, c_string.as_ptr() as *const u8) };

    if procedure_address.is_null() {
        return Err(format!("symbol '{symbol_name}' not found"));
    }

    Ok(procedure_address as u64)
}

/// Unload the currently-loaded DLL (frees the handle stored by [`load_library`]).
///
/// # Safety
///
/// Must not be called while any thread is executing code from the DLL.
#[cfg(windows)]
pub unsafe fn unload_current_dll() {
    let handle = unsafe { CURRENT_DLL_HANDLE };
    if !handle.is_null() {
        unsafe { FreeLibrary(handle) };
        unsafe { CURRENT_DLL_HANDLE = std::ptr::null_mut() };
    }
}

/// Unload a SPECIFIC DLL handle (e.g. the base DLL) without touching the
/// global handle.  This is used by the host to release the base DLL before a
/// full rebuild — the global `CURRENT_DLL_HANDLE` may have been overwritten
/// by intermediate patch-DLL loads, so the host must free the base handle it
/// captured directly.
///
/// # Safety
///
/// `handle` must be a valid handle from [`load_library`] and must not be
/// referenced by any executing thread.
#[cfg(windows)]
pub unsafe fn unload_library(handle: *mut std::ffi::c_void) {
    if !handle.is_null() {
        unsafe { FreeLibrary(handle) };
    }
}

#[cfg(not(windows))]
pub unsafe fn unload_library(_handle: *mut std::ffi::c_void) {
    // no-op on non-Windows
}

#[cfg(not(windows))]
pub unsafe fn unload_current_dll() {
    // no-op on non-Windows
}

// ---------------------------------------------------------------------------
// Thread suspension — safe patching (con 7)
// ---------------------------------------------------------------------------

/// Suspend every thread in this process except the current one, so code pages
/// can be patched without another thread executing a torn instruction.
/// Returns thread handles that must be passed to [`resume_threads`].
///
/// # Safety
///
/// Caller must not hold a lock that a suspended thread needs (deadlock), and
/// must call [`resume_threads`] with the returned handles exactly once.
#[cfg(windows)]
unsafe fn suspend_other_threads() -> Vec<*mut std::ffi::c_void> {
    let mut suspended = Vec::new();
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) };
    if snapshot.is_null() {
        return suspended;
    }

    let current_process = unsafe { GetCurrentProcessId() };
    let current_thread = unsafe { GetCurrentThreadId() };

    let mut entry: ThreadEntry32 = unsafe { std::mem::zeroed() };
    entry.dwSize = std::mem::size_of::<ThreadEntry32>() as u32;

    if unsafe { Thread32First(snapshot, &mut entry) } != 0 {
        loop {
            if entry.th32OwnerProcessID == current_process && entry.th32ThreadID != current_thread {
                let thread_handle =
                    unsafe { OpenThread(THREAD_SUSPEND_RESUME, 0, entry.th32ThreadID) };
                if !thread_handle.is_null() {
                    unsafe { SuspendThread(thread_handle) };
                    suspended.push(thread_handle);
                }
            }
            if unsafe { Thread32Next(snapshot, &mut entry) } == 0 {
                break;
            }
        }
    }

    unsafe { CloseHandle(snapshot) };
    suspended
}

/// Resume every thread suspended by [`suspend_other_threads`].
///
/// # Safety
///
/// `suspended` must be the handles returned by [`suspend_other_threads`], and
/// must not have been resumed already.
#[cfg(windows)]
unsafe fn resume_threads(suspended: &[*mut std::ffi::c_void]) {
    for &handle in suspended {
        unsafe { ResumeThread(handle) };
        unsafe { CloseHandle(handle) };
    }
}

// ---------------------------------------------------------------------------
// Core Live Coding primitive — prologue patching
// ---------------------------------------------------------------------------

/// Patch the prologue of the function at `old_address` so it jumps to the
/// code at `new_address`.  This is the Live++ / UE Live Coding mechanism:
/// the first 5 bytes of the original function become `E9 <rel32>`, a direct
/// jump to the replacement.  Every existing caller keeps its original
/// function pointer and is redirected automatically — no jump table, no
/// wrapper, no runtime overhead after the patch.
///
/// If `new_address` is further than ±2 GB away (a common case when the new
/// code lives in a separately loaded DLL), a **trampoline** is allocated in
/// memory near `old_address`: the trampoline holds an absolute `jmp rax`
/// to the real target, and the prologue jumps (rel32) to the trampoline.
///
/// # Safety
///
/// - `old_address` must point to the start of a real function with at least
///   5 bytes of executable prologue.
/// - `new_address` must point to a function with a signature compatible with
///   the original (a mismatch is undefined behaviour).
/// - Must not be called while another thread is actively executing inside the
///   first 5 bytes of `old_address` (the demo is single-threaded for patching).
#[cfg(windows)]
pub unsafe fn patch_prologue(old_address: u64, new_address: u64) -> Result<(), String> {
    // The jump destination is `old_address + 5` (after the rel32 instruction).
    let jump_source: i64 = (old_address as i64) + (JMP_REL32_SIZE as i64);
    let relative: i64 = (new_address as i64) - jump_source;

    // If the target is within rel32 reach, write a direct `jmp rel32`.
    if relative >= i32::MIN as i64 && relative <= i32::MAX as i64 {
        unsafe { write_jmp_rel32(old_address, relative as i32) }?;
        return Ok(());
    }

    // Otherwise build a trampoline near the original function.
    let trampoline_address = unsafe { build_trampoline(old_address, new_address)? };
    let trampoline_relative: i64 = (trampoline_address as i64) - jump_source;
    if trampoline_relative < i32::MIN as i64 || trampoline_relative > i32::MAX as i64 {
        return Err("trampoline still out of rel32 range".to_string());
    }
    unsafe { write_jmp_rel32(old_address, trampoline_relative as i32) }?;

    Ok(())
}

/// Write a 5-byte `jmp rel32` at `function_address` targeting
/// `function_address + 5 + relative`.
///
/// # Safety
///
/// `function_address` must be the start of a function with ≥5 patchable
/// prologue bytes and must not be concurrently executed.
#[cfg(windows)]
unsafe fn write_jmp_rel32(function_address: u64, relative: i32) -> Result<(), String> {
    let function_start = function_address as *mut u8;

    // Suspend every other thread so no one executes the first 5 bytes while
    // they are being overwritten (con 7: thread-safe patching).
    let suspended = unsafe { suspend_other_threads() };

    let patch_result = (|| -> Result<(), String> {
        // Make the code page writable, remembering the previous protection.
        let mut old_protection: u32 = 0;
        let protect_result = unsafe {
            VirtualProtect(
                function_start as *mut std::ffi::c_void,
                JMP_REL32_SIZE,
                PAGE_EXECUTE_READWRITE,
                &mut old_protection,
            )
        };
        if protect_result == 0 {
            return Err(format!(
                "VirtualProtect failed at {function_address:#x}: {}",
                std::io::Error::last_os_error()
            ));
        }

        // Write the jmp rel32: E9 <little-endian rel32>.
        unsafe {
            *function_start = JMP_REL32_OPCODE;
            let rel_bytes = relative.to_le_bytes();
            std::ptr::copy_nonoverlapping(rel_bytes.as_ptr(), function_start.add(1), 4);
        }

        // Flush the instruction cache so the CPU executes the new bytes.
        unsafe {
            FlushInstructionCache(
                GetCurrentProcess(),
                function_start as *const std::ffi::c_void,
                JMP_REL32_SIZE,
            );
        }

        // Restore the previous page protection.
        let mut ignored_protection: u32 = 0;
        unsafe {
            VirtualProtect(
                function_start as *mut std::ffi::c_void,
                JMP_REL32_SIZE,
                old_protection,
                &mut ignored_protection,
            );
        }

        Ok(())
    })();

    // Always resume, even if the patch failed.
    unsafe { resume_threads(&suspended) };

    patch_result
}

/// Allocate a trampoline near `old_address` that performs an absolute jump
/// to `new_address`.  The trampoline is:
///
///   mov rax, imm64     ; 48 B8 <8-byte address>
///   jmp rax            ; FF E0
///
/// Returns the trampoline's address.  Searches for free memory within ±1 GB
/// of the original function so the prologue's rel32 jump can reach it.
///
/// # Safety
///
/// The returned trampoline stays committed for the lifetime of the process
/// (leaked by design — it is executable code that may be jumped to forever).
#[cfg(windows)]
unsafe fn build_trampoline(old_address: u64, new_address: u64) -> Result<u64, String> {
    // Reuse a cached trampoline for this target if it is reachable from the
    // original function (con 8: resource growth).
    if let Some(existing) = TRAMPOLINE_CACHE.lock().unwrap().get(&new_address).copied() {
        let existing_relative = (existing as i64) - (old_address as i64 + JMP_REL32_SIZE as i64);
        if existing_relative >= i32::MIN as i64 && existing_relative <= i32::MAX as i64 {
            return Ok(existing);
        }
    }

    let low_bound = (old_address as i64 - 0x4000_0000).max(0) as u64; // −1 GB
    let high_bound = old_address as i64 + 0x4000_0000; // +1 GB

    // Probe candidate addresses from the original function outward.
    let mut candidate: u64;
    let mut direction: i64 = 1;
    let mut offset: u64 = 0;

    loop {
        if direction > 0 {
            candidate = old_address.wrapping_add(offset);
        } else {
            candidate = old_address.wrapping_sub(offset);
        }

        // Only try candidates within the ±1 GB window.
        if candidate >= low_bound && (candidate as i64) <= high_bound && candidate != 0 {
            let allocation = unsafe {
                VirtualAlloc(
                    candidate as *mut std::ffi::c_void,
                    TRAMPOLINE_SIZE,
                    MEM_COMMIT | MEM_RESERVE,
                    PAGE_EXECUTE_READWRITE,
                )
            };
            if !allocation.is_null() {
                let trampoline_start = allocation as *mut u8;
                unsafe {
                    // mov rax, imm64
                    *trampoline_start = REX_W_PREFIX;
                    *trampoline_start.add(1) = MOV_RAX_IMM64_OPCODE;
                    std::ptr::copy_nonoverlapping(
                        new_address.to_le_bytes().as_ptr(),
                        trampoline_start.add(2),
                        8,
                    );
                    // jmp rax
                    *trampoline_start.add(10) = JMP_RAX_OPCODE;
                    *trampoline_start.add(11) = JMP_RAX_MODRM;
                }
                // Flush the instruction cache for the trampoline.
                unsafe {
                    FlushInstructionCache(GetCurrentProcess(), allocation, TRAMPOLINE_SIZE);
                }
                // Cache it so repeated far patches to this target reuse it.
                TRAMPOLINE_CACHE
                    .lock()
                    .unwrap()
                    .insert(new_address, allocation as u64);
                return Ok(allocation as u64);
            }
        }

        // Alternate direction and grow the search distance.
        direction = -direction;
        if direction > 0 {
            offset += TRAMPOLINE_SEARCH_STRIDE as u64;
        }

        // Bail out if we've swept the whole ±1 GB window.
        if offset > 0x4000_0000 {
            break;
        }
    }

    Err(format!(
        "could not allocate a trampoline near {old_address:#x}"
    ))
}

#[cfg(not(windows))]
pub unsafe fn patch_prologue(_old_address: u64, _new_address: u64) -> Result<(), String> {
    Err("patch_prologue is only supported on Windows".to_string())
}

// ---------------------------------------------------------------------------
// Single-function compilation via rustc — bypasses cargo's linker.
// ---------------------------------------------------------------------------

/// Compile a single `extern "C"` function to a `.dll` using `rustc` directly.
/// Writes to the system temp dir with a unique PID-based name — no file-lock
/// conflicts on repeated compilations.
/// `params` is the parameter list, e.g. `"x: i32, y: i32"` or `""`.
#[cfg(windows)]
pub fn compile_single_function(
    function_name: &str,
    params: &str,
    function_body: &str,
    return_type: &str,
) -> Result<std::path::PathBuf, String> {
    use std::io::Write;

    let return_decl = if return_type.is_empty() {
        String::new()
    } else {
        format!(" -> {return_type}")
    };

    // The generated module includes a `set_services` export (so the host can
    // hand it the service table, making host-owned state reachable from
    // patched code) plus a `host_state()` helper the body can call directly.
    let source_code = format!(
        "#[repr(C)]
struct HostServices {{
    get_state: unsafe extern \"C\" fn() -> *mut std::ffi::c_void,
}}

#[allow(dead_code)]
static mut HOST_SERVICES_PTR: *const HostServices = core::ptr::null();

#[unsafe(no_mangle)]
pub extern \"C\" fn set_services(services: *const HostServices) {{
    unsafe {{ HOST_SERVICES_PTR = services; }}
}}

#[allow(dead_code)]
fn host_state() -> *mut std::ffi::c_void {{
    unsafe {{ ((*HOST_SERVICES_PTR).get_state)() }}
}}

#[unsafe(no_mangle)]
pub extern \"C\" fn {function_name}({params}){return_decl} {{
    {function_body}
}}
"
    );

    let temp_dir = std::env::temp_dir();
    let pid = std::process::id();
    // Monotonic counter so every compilation gets a unique filename even
    // within the same process (PID alone isn't enough for repeated reloads).
    static COMPILE_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let counter = COMPILE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let source_path = temp_dir.join(format!("live_code_{function_name}_{pid}_{counter}.rs"));
    let dll_path = temp_dir.join(format!("live_code_{function_name}_{pid}_{counter}.dll"));

    let mut file = std::fs::File::create(&source_path)
        .map_err(|e| format!("cannot create {source_path:?}: {e}"))?;
    file.write_all(source_code.as_bytes())
        .map_err(|e| format!("cannot write {source_path:?}: {e}"))?;
    drop(file);

    let output = std::process::Command::new("rustc")
        .args([
            "--crate-type=cdylib",
            "--crate-name",
            "live_coding_func",
            "-o",
        ])
        .arg(&dll_path)
        .args(["--edition", "2024"])
        .arg(&source_path)
        .output()
        .map_err(|e| format!("failed to run rustc: {e}"))?;

    let _ = std::fs::remove_file(&source_path);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("rustc failed:\n{stderr}"));
    }

    Ok(dll_path)
}

#[cfg(not(windows))]
pub fn compile_single_function(
    _function_name: &str,
    _params: &str,
    _function_body: &str,
    _return_type: &str,
) -> Result<std::path::PathBuf, String> {
    Err("compile_single_function is only supported on Windows".to_string())
}

// ---------------------------------------------------------------------------
// High-level patch helpers
// ---------------------------------------------------------------------------

/// Free the previous patch DLL registered for `function_name` (if any) and
/// delete its temp file.  Called AFTER the prologue is re-patched, when the
/// old patch DLL is no longer referenced (con 8: resource growth).
fn free_previous_patch_dll(function_name: &str) {
    if let Some((handle_as_usize, path)) = PATCH_DLL_INFO.lock().unwrap().remove(function_name) {
        unsafe { unload_library(handle_as_usize as *mut std::ffi::c_void) };
        let _ = std::fs::remove_file(&path);
    }
}

/// Remember the currently-loaded patch DLL for `function_name`.
fn record_patch_dll(function_name: &str, handle: *mut std::ffi::c_void, path: &std::path::Path) {
    PATCH_DLL_INFO.lock().unwrap().insert(
        function_name.to_string(),
        (handle as usize, path.to_path_buf()),
    );
}

/// Compile a replacement for an EXISTING function and patch its prologue in
/// place.  Returns the path of the compiled patch DLL on success.
///
/// # Safety
///
/// The replacement must have a signature compatible with the original at
/// `original_address`.
#[cfg(windows)]
pub unsafe fn compile_and_patch_prologue(
    function_name: &str,
    params: &str,
    function_body: &str,
    return_type: &str,
    original_address: u64,
) -> Result<std::path::PathBuf, String> {
    let signature = format!("({params}) -> {return_type}");
    // ABI guard: refuse to patch over a different signature (con 2 / 9).
    verify_signature(function_name, &signature)?;

    let dll_path = compile_single_function(function_name, params, function_body, return_type)?;
    let handle = unsafe { load_library(&dll_path)? };

    // Hand the host service table to the patched code (host-owned state).
    unsafe { provide_services_to_dll(handle)? };

    let new_address = unsafe { resolve_export(handle, function_name)? };

    // Redirect the original function to the new code — in place, no wrapper.
    unsafe { patch_prologue(original_address, new_address)? };

    // The old patch DLL is now unreferenced — free it, then record the new one.
    free_previous_patch_dll(function_name);
    record_patch_dll(function_name, handle, &dll_path);

    // Remember the new address + signature under the function name.
    register_function(function_name, new_address, &signature);

    Ok(dll_path)
}

/// Compile a NEW function (one that did not exist in the base DLL), load it,
/// and register it in the name-keyed registry.  Returns the new address.
///
/// This is the Rust equivalent of UE Live Coding's "add a new function live":
/// the function never existed before, so there is no prologue to patch — it is
/// simply registered and becomes resolvable by name.
#[cfg(windows)]
pub unsafe fn compile_and_register_new(
    function_name: &str,
    params: &str,
    function_body: &str,
    return_type: &str,
) -> Result<u64, String> {
    let signature = format!("({params}) -> {return_type}");
    verify_signature(function_name, &signature)?;

    let dll_path = compile_single_function(function_name, params, function_body, return_type)?;
    let handle = unsafe { load_library(&dll_path)? };
    unsafe { provide_services_to_dll(handle)? };
    let new_address = unsafe { resolve_export(handle, function_name)? };

    free_previous_patch_dll(function_name);
    record_patch_dll(function_name, handle, &dll_path);
    register_function(function_name, new_address, &signature);

    Ok(new_address)
}
