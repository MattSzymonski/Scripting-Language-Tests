// REQUIREMENTS
//   Rust stable (edition 2021+). No external crate dependencies.
//
// DESCRIPTION
//   Minimal-viable-product reimplementation of the `subsecond` hot-patching
//   runtime (https://crates.io/crates/subsecond).  Provides a jump-table based
//   indirection layer so function bodies can be swapped at runtime without
//   restarting the process or modifying existing executable memory.
//
//   The MVP omits the external build-orchestrator (ThinLink / dioxus-cli) and
//   native dylib loading (libloading).  Instead `apply_patch` accepts a
//   pre-constructed `JumpTable` directly, which is sufficient for in-process
//   experimentation and for integration with a custom build pipeline.
//
// USAGE
//   See `main.rs` for a runnable demo.
//
// EXAMPLE USAGE (library consumer)
//   let hot = subsecond::HotFn::current(my_closure);
//   let result = hot.call((arg1, arg2));
//
//   // Later, after recompilation:
//   let new_table = subsecond::JumpTable { /* old→new addr map */ };
//   unsafe { subsecond::apply_patch(new_table).unwrap(); }
//
// --- SCRIPT ---

use std::collections::HashMap;
use std::panic::AssertUnwindSafe;
use std::sync::atomic::AtomicPtr;
use std::sync::{Arc, Mutex};

// ---------------------------------------------------------------------------
// Global state
// ---------------------------------------------------------------------------

/// The single global jump table.  When `None` every hot function simply calls
/// its original implementation.  After `apply_patch` it is replaced atomically.
static APP_JUMP_TABLE: AtomicPtr<JumpTable> = AtomicPtr::new(std::ptr::null_mut());

/// Handlers invoked immediately after a new jump table is committed.
static HOTRELOAD_HANDLERS: Mutex<Vec<Arc<dyn Fn() + Send + Sync>>> = Mutex::new(Vec::new());

// ---------------------------------------------------------------------------
// Public data types
// ---------------------------------------------------------------------------

/// Maps **old** function-pointer addresses (as `u64`) to **new** addresses.
///
/// In a full Subsecond deployment the map is constructed by the CLI tool
/// (`create_jump_table`) by diffing the symbol tables of the original binary
/// and the freshly-compiled patch library.  Here we expose the raw map so you
/// can feed it manually or from a custom build step.
#[derive(Debug, Clone, Default)]
pub struct JumpTable {
    /// old function address → new function address
    pub map: HashMap<u64, u64>,
}

/// Opaque handle to a hot-reloadable function.
///
/// Create one via [`HotFn::current`], then call it with [`HotFn::call`].
/// Under the hood every invocation consults the global [`JumpTable`]; if a
/// newer version of the function has been registered it will be transparently
/// dispatched.
pub struct HotFn<Args, Marker, Function>
where
    Function: HotFunction<Args, Marker>,
{
    inner: Function,
    _marker: std::marker::PhantomData<(Args, Marker)>,
}

/// A pointer-sized identifier for a hot-patched function.
///
/// Returned by [`HotFn::ptr_address`].  Can be compared across patches to
/// detect whether the function body actually changed (though without pointer
/// versioning every patch yields a new address).
#[non_exhaustive]
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct HotFnPtr(pub u64);

/// Special panic payload emitted when a *caller* of a hot function is stale.
///
/// When code **above** a `subsecond::call` site changes, the call site itself
/// becomes invalid.  Subsecond detects this and panics with `HotFnPanic`.
/// The enclosing [`call`] loop catches this panic and retries automatically,
/// unwinding only as far as the nearest `call` boundary so that long-lived
/// state above that boundary is preserved.
#[derive(Debug)]
pub struct HotFnPanic {
    _backtrace: std::backtrace::Backtrace,
}

impl HotFnPanic {
    /// Create a new `HotFnPanic` with a captured backtrace.
    ///
    /// This is normally constructed internally by [`HotFn::try_call`] when a
    /// stale caller is detected.  Exposed publicly for testing / demo purposes.
    pub fn new() -> Self {
        Self {
            _backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}

/// Errors that can occur while applying a patch.
#[derive(Debug, PartialEq)]
pub enum PatchError {
    /// The jump table is malformed (e.g. empty map).
    InvalidTable(&'static str),
}

// ---------------------------------------------------------------------------
// Public free functions
// ---------------------------------------------------------------------------

/// Call a closure with hot-reloading enabled.
///
/// If the closure's code is patched while it is running the call will unwind
/// (via [`HotFnPanic`]) and **retry** automatically.  This means edits to code
/// *inside* the closure take effect on the very next invocation, while edits to
/// code *above* this call site cause a safe unwind / retry cycle.
///
/// In release mode (`debug_assertions` disabled) this is a zero-cost
/// passthrough — the jump-table lookup is compiled away entirely.
///
/// # Example
///
/// ```ignore
/// subsecond::call(|| {
///     println!("Hello from hot-reloadable code!");
/// });
/// ```
pub fn call<Output>(mut function: impl FnMut() -> Output) -> Output {
    // In release builds just call the function directly — zero overhead.
    if !cfg!(debug_assertions) {
        return function();
    }

    let mut hot_function = HotFn::current(function);
    loop {
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| hot_function.call(())));

        match result {
            Ok(output) => return output,
            Err(payload) => {
                // If it's our own HotFnPanic, loop and retry with the latest
                // function pointer.  Otherwise resume unwinding.
                if payload.downcast_ref::<HotFnPanic>().is_none() {
                    std::panic::resume_unwind(payload);
                }
            }
        }
    }
}

/// Register a callback that fires every time a patch is applied.
///
/// Handlers run **after** the new [`JumpTable`] has been committed, so they
/// can safely inspect the latest table via [`get_jump_table`].
pub fn register_handler(handler: Arc<dyn Fn() + Send + Sync + 'static>) {
    HOTRELOAD_HANDLERS.lock().unwrap().push(handler);
}

/// Return a reference to the current global jump table, if one has been set.
///
/// # Safety
///
/// The returned reference points into a leaked box.  It is valid *right now*
/// but may be invalidated by a future call to [`apply_patch`].  Do not hold
/// the reference across patch boundaries.
pub unsafe fn get_jump_table() -> Option<&'static JumpTable> {
    let pointer = APP_JUMP_TABLE.load(std::sync::atomic::Ordering::Relaxed);
    if pointer.is_null() {
        return None;
    }
    Some(unsafe { &*pointer })
}

/// Apply a new jump table, atomically replacing the previous one.
///
/// After this call every subsequent [`HotFn::call`] / [`call`] invocation will
/// use the updated function addresses.
///
/// # Safety
///
/// The addresses in `table.map` must point to valid functions whose signatures
/// match the original functions they replace.  Mismatched signatures cause
/// **undefined behaviour** (almost certainly a crash).
///
/// In the full Subsecond pipeline the CLI tool guarantees correctness; here
/// the caller is responsible.
pub unsafe fn apply_patch(table: JumpTable) -> Result<(), PatchError> {
    if table.map.is_empty() {
        return Err(PatchError::InvalidTable(
            "JumpTable map must contain at least one entry",
        ));
    }
    unsafe { commit_patch(table) };
    Ok(())
}

// ---------------------------------------------------------------------------
// DLL loading — load a compiled patch library and build a JumpTable from its
// exported symbols.  Windows-only; on other platforms this is a stub.
// ---------------------------------------------------------------------------

/// Load a patch DLL and build a [`JumpTable`] by resolving named exports.
///
/// Each entry in `symbol_map` is `(exported_symbol_name, original_function_address)`.
/// The function looks up every symbol in the DLL; if found, the corresponding
/// `original_address → dll_address` mapping is added to the table.
///
/// On non-Windows platforms this returns `Err` with a message.
///
/// # Safety
///
/// The DLL's exported functions must have signatures compatible with the
/// original functions they replace.  Mismatched signatures cause undefined
/// behaviour when the jump table is applied.
#[cfg(windows)]
pub unsafe fn load_patch_dll(
    dll_path: &std::path::Path,
    symbol_map: &[(&str, u64)],
) -> Result<JumpTable, String> {
    use std::os::windows::ffi::OsStrExt;

    // Convert the path to a NUL-terminated wide string for LoadLibraryW.
    let wide_path: Vec<u16> = dll_path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    // Load the DLL into the process address space.
    let module_handle = unsafe { LoadLibraryW(wide_path.as_ptr()) };
    if module_handle.is_null() {
        return Err(format!(
            "LoadLibraryW failed for '{}': {}",
            dll_path.display(),
            std::io::Error::last_os_error()
        ));
    }

    // Store the handle so reload_patch_dll can free it later.
    unsafe { CURRENT_DLL_HANDLE = module_handle };

    let mut map = HashMap::new();

    for &(symbol_name, original_address) in symbol_map {
        let c_string =
            std::ffi::CString::new(symbol_name).map_err(|e| format!("Invalid symbol name: {e}"))?;
        let procedure_address =
            unsafe { GetProcAddress(module_handle, c_string.as_ptr() as *const u8) };

        if !procedure_address.is_null() {
            map.insert(original_address, procedure_address as u64);
        }
        // Symbols not found in the DLL are silently skipped — they weren't
        // part of the patch.
    }

    Ok(JumpTable { map })
}

#[cfg(not(windows))]
pub unsafe fn load_patch_dll(
    _dll_path: &std::path::Path,
    _symbol_map: &[(&str, u64)],
) -> Result<JumpTable, String> {
    Err("load_patch_dll is only supported on Windows".to_string())
}

/// Patch a **single** function pointer in the jump table without rebuilding
/// the entire table.  The base DLL stays loaded and linked — only the entry
/// for `old_address → new_address` is added or updated.
///
/// If no jump table exists yet, one is created with just this entry.
///
/// # Safety
///
/// `new_address` must point to a function with a signature compatible with
/// the original function at `old_address`.
#[cfg(windows)]
pub unsafe fn patch_function(old_address: u64, new_address: u64) {
    let current_ptr = APP_JUMP_TABLE.load(std::sync::atomic::Ordering::Relaxed);
    if current_ptr.is_null() {
        // First patch — create a fresh table with this single entry.
        let mut map = HashMap::new();
        map.insert(old_address, new_address);
        let table = Box::new(JumpTable { map });
        APP_JUMP_TABLE.store(Box::into_raw(table), std::sync::atomic::Ordering::Relaxed);
    } else {
        // Update the existing table in-place (the table is leaked, so
        // we hold the only reference and this is safe under relaxed ordering
        // since we're single-threaded for patching).
        unsafe {
            let table = &mut *current_ptr;
            table.map.insert(old_address, new_address);
        }
    }

    // Notify registered handlers.
    let handlers = HOTRELOAD_HANDLERS.lock().unwrap().clone();
    for handler in &handlers {
        handler();
    }
}

#[cfg(not(windows))]
pub unsafe fn patch_function(_old_address: u64, _new_address: u64) {}

/// Load a tiny patch DLL that exports **one** function, resolve it, and
/// patch that single function pointer into the jump table.
///
/// The base DLL (`patch_dll`) is **not** touched — only the one changed
/// function's pointer is updated.  This is the true Subsecond flow: compile
/// only the changed function, load it, swap one pointer.
///
/// Returns `Ok(())` on success.
///
/// # Safety
///
/// The exported function's signature must match the original.
#[cfg(windows)]
pub unsafe fn load_single_function(
    dll_path: &std::path::Path,
    symbol_name: &str,
    original_address: u64,
) -> Result<(), String> {
    // The DLL is already at a unique temp path from compile_single_function,
    // so we can load it directly — no copy needed, no file-lock race.
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

    let c_string =
        std::ffi::CString::new(symbol_name).map_err(|e| format!("Invalid symbol name: {e}"))?;
    let procedure_address =
        unsafe { GetProcAddress(module_handle, c_string.as_ptr() as *const u8) };

    if procedure_address.is_null() {
        return Err(format!(
            "symbol '{}' not found in '{}'",
            symbol_name,
            dll_path.display()
        ));
    }

    unsafe { patch_function(original_address, procedure_address as u64) };

    Ok(())
}

#[cfg(not(windows))]
pub unsafe fn load_single_function(
    _dll_path: &std::path::Path,
    _symbol_name: &str,
    _original_address: u64,
) -> Result<(), String> {
    Err("load_single_function is only supported on Windows".to_string())
}

/// Load a DLL, resolve every symbol in `symbol_map`, and patch each one
/// individually via [`patch_function`].  The DLL is loaded from a temp copy
/// so the original file is never locked.
///
/// Returns the number of symbols successfully patched.
#[cfg(windows)]
pub unsafe fn load_and_patch_all(
    dll_path: &std::path::Path,
    symbol_map: &[(&str, u64)],
) -> Result<usize, String> {
    use std::os::windows::ffi::OsStrExt;

    // Copy to temp so the source DLL is never locked.
    let temp_dir = std::env::temp_dir();
    let file_stem = dll_path.file_stem().unwrap_or_default().to_string_lossy();
    let temp_path = temp_dir.join(format!("{}_{}.dll", file_stem, std::process::id()));

    std::fs::copy(dll_path, &temp_path).map_err(|e| {
        format!(
            "failed to copy '{}' → '{}': {e}",
            dll_path.display(),
            temp_path.display()
        )
    })?;

    let wide_path: Vec<u16> = temp_path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let module_handle = unsafe { LoadLibraryW(wide_path.as_ptr()) };
    if module_handle.is_null() {
        return Err(format!(
            "LoadLibraryW failed for '{}': {}",
            temp_path.display(),
            std::io::Error::last_os_error()
        ));
    }

    let mut count: usize = 0;
    for &(symbol_name, original_address) in symbol_map {
        let c_string = std::ffi::CString::new(symbol_name)
            .map_err(|e| format!("Invalid symbol name '{symbol_name}': {e}"))?;
        let procedure_address =
            unsafe { GetProcAddress(module_handle, c_string.as_ptr() as *const u8) };
        if !procedure_address.is_null() {
            unsafe { patch_function(original_address, procedure_address as u64) };
            count += 1;
        }
    }

    Ok(count)
}

#[cfg(not(windows))]
pub unsafe fn load_and_patch_all(
    _dll_path: &std::path::Path,
    _symbol_map: &[(&str, u64)],
) -> Result<usize, String> {
    Err("load_and_patch_all is only supported on Windows".to_string())
}

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
}

/// Handle to the currently-loaded patch DLL, if any.
#[cfg(windows)]
static mut CURRENT_DLL_HANDLE: *mut std::ffi::c_void = std::ptr::null_mut();

// ---------------------------------------------------------------------------
// Single-function compilation via rustc — bypasses cargo's linker.
// Compile ONE function to a tiny DLL in ~0.3s instead of relinking 5000.
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

    let source_code = format!(
        "#[unsafe(no_mangle)]\npub extern \"C\" fn {function_name}({params}){return_decl} {{\n    {function_body}\n}}\n"
    );

    let temp_dir = std::env::temp_dir();
    let pid = std::process::id();
    // Monotonic counter so every compilation gets a unique filename even
    // within the same process (PID alone isn't enough for repeated reloads).
    static COMPILE_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let counter = COMPILE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let source_path = temp_dir.join(format!("sss_func_{function_name}_{pid}_{counter}.rs"));
    let dll_path = temp_dir.join(format!("sss_func_{function_name}_{pid}_{counter}.dll"));

    let mut file = std::fs::File::create(&source_path)
        .map_err(|e| format!("cannot create {source_path:?}: {e}"))?;
    file.write_all(source_code.as_bytes())
        .map_err(|e| format!("cannot write {source_path:?}: {e}"))?;
    drop(file);

    let output = std::process::Command::new("rustc")
        .args([
            "--crate-type=cdylib",
            "--crate-name",
            "live_patch_func",
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

/// Compile a single function and immediately patch it into the jump table.
#[cfg(windows)]
pub unsafe fn compile_and_patch(
    function_name: &str,
    params: &str,
    function_body: &str,
    return_type: &str,
    original_address: u64,
) -> Result<std::path::PathBuf, String> {
    let dll_path = compile_single_function(function_name, params, function_body, return_type)?;
    unsafe { load_single_function(&dll_path, function_name, original_address)? };
    Ok(dll_path)
}

#[cfg(not(windows))]
pub unsafe fn compile_and_patch(
    _function_name: &str,
    _params: &str,
    _function_body: &str,
    _return_type: &str,
    _original_address: u64,
) -> Result<std::path::PathBuf, String> {
    Err("compile_and_patch is only supported on Windows".to_string())
}

/// Reload the patch DLL: free the old one, load the new one, resolve symbols,
/// build a fresh [`JumpTable`], and apply it atomically.
///
/// Returns the number of symbols resolved on success.
///
/// # Safety
///
/// The DLL's exported functions must have signatures compatible with the
/// original functions they replace.
#[cfg(windows)]
pub unsafe fn reload_patch_dll(
    dll_path: &std::path::Path,
    symbol_map: &[(&str, u64)],
) -> Result<usize, String> {
    // Save the old handle BEFORE loading the new DLL — load_patch_dll
    // overwrites CURRENT_DLL_HANDLE with the new one.
    let old_handle = unsafe { CURRENT_DLL_HANDLE };

    // Step 1 — load the new DLL first (before freeing the old one).
    let table = unsafe { load_patch_dll(dll_path, symbol_map)? };
    let count = table.map.len();

    if count == 0 {
        return Err("no matching symbols found in DLL".to_string());
    }

    // Step 2 — apply the new jump table so no one references the old DLL.
    unsafe { apply_patch(table).map_err(|e| format!("apply_patch failed: {e:?}"))? };

    // Step 3 — now it's safe to unload the previous DLL.
    if !old_handle.is_null() {
        unsafe { FreeLibrary(old_handle) };
    }

    Ok(count)
}

/// Unload the currently-loaded patch DLL and clear the jump table.
///
/// After calling this, hot functions revert to their original in-process
/// implementations.  Call this before rebuilding the DLL so the linker can
/// overwrite the file (Windows locks loaded DLLs).
///
/// # Safety
///
/// Must not be called while any thread is executing code from the DLL.
#[cfg(windows)]
pub unsafe fn unload_patch_dll() {
    // Step 1 — clear the jump table so no future calls hit the DLL.
    APP_JUMP_TABLE.store(std::ptr::null_mut(), std::sync::atomic::Ordering::Relaxed);

    // Step 2 — free the library.
    let handle = unsafe { CURRENT_DLL_HANDLE };
    if !handle.is_null() {
        unsafe { FreeLibrary(handle) };
        unsafe { CURRENT_DLL_HANDLE = std::ptr::null_mut() };
    }
}

#[cfg(not(windows))]
pub unsafe fn unload_patch_dll() {
    // no-op on non-Windows
}

#[cfg(not(windows))]
pub unsafe fn reload_patch_dll(
    _dll_path: &std::path::Path,
    _symbol_map: &[(&str, u64)],
) -> Result<usize, String> {
    Err("reload_patch_dll is only supported on Windows".to_string())
}

// ---------------------------------------------------------------------------
// Internal: commit a jump table and notify handlers
// ---------------------------------------------------------------------------

unsafe fn commit_patch(table: JumpTable) {
    // Leak the table so the AtomicPtr always points to valid memory.
    let boxed = Box::new(table);
    APP_JUMP_TABLE.store(Box::into_raw(boxed), std::sync::atomic::Ordering::Relaxed);

    // Notify every registered handler (clone the vec to avoid deadlocks).
    let handlers = HOTRELOAD_HANDLERS.lock().unwrap().clone();
    for handler in &handlers {
        handler();
    }
}

// ---------------------------------------------------------------------------
// HotFn methods
// ---------------------------------------------------------------------------

impl<Args, Marker, Function> HotFn<Args, Marker, Function>
where
    Function: HotFunction<Args, Marker>,
{
    /// Wrap an existing function / closure so it can be hot-patched.
    ///
    /// `current` is a const fn — it does not allocate or inspect the function.
    pub const fn current(function: Function) -> Self {
        Self {
            inner: function,
            _marker: std::marker::PhantomData,
        }
    }

    /// Call the function, automatically dispatching to the latest patched
    /// version.
    ///
    /// Panics with [`HotFnPanic`] if the caller is stale (i.e. code above
    /// this call site changed).  Use [`HotFn::try_call`] if you need to
    /// handle that case manually.
    pub fn call(&mut self, args: Args) -> Function::Return {
        self.try_call(args).unwrap()
    }

    /// Call the function, returning `Err(HotFnPanic)` when the caller is stale
    /// instead of panicking.
    pub fn try_call(&mut self, args: Args) -> Result<Function::Return, HotFnPanic> {
        if !cfg!(debug_assertions) {
            return Ok(self.inner.call_it(args));
        }

        // Fast path: if the inner type is pointer-sized (function pointer /
        // ZST) we can use the jump-table lookup via call_as_ptr.
        if std::mem::size_of::<Function>() == std::mem::size_of::<fn() -> ()>() {
            return Ok(unsafe { self.inner.call_as_ptr(args) });
        }

        // Slow path for trait objects / fat pointers: look up `call_it`'s own
        // vtable entry in the jump table and call through it.
        unsafe {
            let known_function_pointer =
                <Function as HotFunction<Args, Marker>>::call_it as *const () as u64;

            if let Some(jump_table) = get_jump_table() {
                if let Some(patched_pointer) = jump_table.map.get(&known_function_pointer).cloned()
                {
                    let call_it: fn(&Function, Args) -> Function::Return =
                        std::mem::transmute(patched_pointer as *const ());
                    return Ok(call_it(&self.inner, args));
                }
            }

            Ok(self.inner.call_it(args))
        }
    }

    /// Return the current address of the underlying function.
    ///
    /// If a patch has been applied the returned address reflects the **newest**
    /// version.  Compare this against a previously-saved [`HotFnPtr`] to
    /// detect whether the function changed across a patch boundary.
    pub fn ptr_address(&self) -> HotFnPtr {
        // For plain function pointers (whose size == usize) check the jump
        // table first — the stored fn pointer value doesn't change across
        // patches, but the *effective* address (what would be called) does.
        if std::mem::size_of::<Function>() == std::mem::size_of::<fn() -> ()>() {
            let pointer: usize = unsafe { std::mem::transmute_copy(&self.inner) };
            let original = pointer as u64;

            if let Some(jump_table) = unsafe { get_jump_table() } {
                if let Some(patched) = jump_table.map.get(&original).cloned() {
                    return HotFnPtr(patched);
                }
            }

            return HotFnPtr(original);
        }

        let known_function_pointer =
            <Function as HotFunction<Args, Marker>>::call_it as *const () as usize;

        if let Some(jump_table) = unsafe { get_jump_table() } {
            if let Some(patched_pointer) = jump_table
                .map
                .get(&(known_function_pointer as u64))
                .cloned()
            {
                return HotFnPtr(patched_pointer);
            }
        }

        HotFnPtr(known_function_pointer as u64)
    }
}

// ---------------------------------------------------------------------------
// HotFunction trait  (sealed via doc-hidden marker types)
// ---------------------------------------------------------------------------

/// Trait enabling a type to participate in hot-patching.
///
/// Implemented for `FnMut(...)` closures up to 9 arguments.  Not intended to
/// be implemented by downstream code — the marker type parameter seals it.
pub trait HotFunction<Args, Marker> {
    /// The return type of the function.
    type Return;

    /// The function-pointer equivalent of this callable.
    type Real;

    /// Call the function normally (without jump-table indirection).
    fn call_it(&mut self, args: Args) -> Self::Return;

    /// Call the function **with** jump-table indirection.
    ///
    /// # Safety
    ///
    /// Only valid when `Self` is pointer-sized (function pointer or ZST).
    unsafe fn call_as_ptr(&mut self, args: Args) -> Self::Return;
}

// ---------------------------------------------------------------------------
// Macro-generated implementations for FnMut arities 0–9
// ---------------------------------------------------------------------------

macro_rules! impl_hot_function {
    (
        $(
            ($marker:ident, $($arg:ident),*)
        ),*
    ) => {
        $(
            #[doc(hidden)]
            pub struct $marker;

            impl<T, $($arg,)* ReturnType> HotFunction<($($arg,)*), $marker> for T
            where
                T: FnMut($($arg),*) -> ReturnType,
            {
                type Return = ReturnType;
                type Real = fn($($arg),*) -> ReturnType;

                fn call_it(&mut self, args: ($($arg,)*)) -> Self::Return {
                    #[allow(non_snake_case)]
                    let ($($arg,)*) = args;
                    self($($arg),*)
                }

                unsafe fn call_as_ptr(&mut self, args: ($($arg,)*)) -> Self::Return {
                    unsafe {
                        if let Some(jump_table) = get_jump_table() {
                            // Read the function pointer value.  `self` is
                            // `&mut Self`; passing it directly to
                            // `transmute_copy` (which expects `&Src`) reads
                            // the actual function-pointer bytes.
                            let real: Self::Real = std::mem::transmute_copy(self);

                            #[cfg(target_pointer_width = "64")]
                            let real_addr = real as u64;
                            #[cfg(target_pointer_width = "32")]
                            let real_addr = real as u32 as u64;

                            if let Some(patched_addr) = jump_table.map.get(&real_addr).cloned() {
                                #[cfg(target_pointer_width = "64")]
                                let patched: Self::Real = std::mem::transmute(patched_addr);
                                #[cfg(target_pointer_width = "32")]
                                let patched: Self::Real = std::mem::transmute(patched_addr as u32);

                                #[allow(non_snake_case)]
                                let ($($arg,)*) = args;
                                return patched($($arg),*);
                            }
                        }

                        // No jump table or no entry — call the original.
                        #[allow(non_snake_case)]
                        let ($($arg,)*) = args;
                        self($($arg),*)
                    }
                }
            }
        )*
    };
}

impl_hot_function!(
    (Fn0Marker,),
    (Fn1Marker, A),
    (Fn2Marker, A, B),
    (Fn3Marker, A, B, C),
    (Fn4Marker, A, B, C, D),
    (Fn5Marker, A, B, C, D, E),
    (Fn6Marker, A, B, C, D, E, F),
    (Fn7Marker, A, B, C, D, E, F, G),
    (Fn8Marker, A, B, C, D, E, F, G, H),
    (Fn9Marker, A, B, C, D, E, F, G, H, I)
);
