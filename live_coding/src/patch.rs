//! Prologue-patching primitives for Live Coding (Windows x86/x64).
//!
//! The core Live++/UE-style mechanism: overwrite the first 5 bytes of a
//! function with `E9 <rel32>`, a direct jump to freshly compiled code.  Every
//! existing caller keeps its original function pointer and is transparently
//! redirected - the pointer never changes.
//!
//! If the target is further than ±2 GB away, a **trampoline** is allocated
//! near the original function (holding an absolute `mov rax, imm64; jmp rax`)
//! and the prologue jumps to that.
//!
//! Patching must happen on the main thread, between frames, so there is no
//! concurrent executor inside the first 5 bytes of the patched function and
//! no thread suspension is needed.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use crate::style::{paint_cyan as cyan, paint_dim as dim};
use crate::error::LiveCodeError;

// ---------------------------------------------------------------------------
// Memory-protection constants (WinNT.h)
// ---------------------------------------------------------------------------

/// PAGE_EXECUTE_READWRITE - code page made writable so we can patch it.
const PAGE_EXECUTE_READWRITE: u32 = 0x40;
/// MEM_COMMIT - commit the allocated region (VirtualAlloc).
const MEM_COMMIT: u32 = 0x1000;
/// MEM_RESERVE - reserve the region (VirtualAlloc).
const MEM_RESERVE: u32 = 0x2000;
/// The `jmp rel32` opcode (x86 / x64 near relative jump).
const JMP_REL32_OPCODE: u8 = 0xE9;
/// Number of bytes a `jmp rel32` instruction occupies.
const JMP_REL32_SIZE: usize = 5;
/// Size of an absolute trampoline: `mov rax, imm64; jmp rax` = 10 + 2 bytes.
const TRAMPOLINE_SIZE: usize = 12;
/// `mov rax, imm64` - REX.W + B8 + 8-byte immediate.
const MOV_RAX_IMM64_OPCODE: u8 = 0xB8;
/// REX.W prefix byte for `mov rax, imm64`.
const REX_W_PREFIX: u8 = 0x48;
/// `jmp rax` opcode (FF /4).
const JMP_RAX_OPCODE: u8 = 0xFF;
/// `jmp rax` ModRM byte (mod=11, reg=4, rm=0).
const JMP_RAX_MODRM: u8 = 0xE0;
/// Search stride when looking for trampoline memory (±1 GB window, 16 KB steps).
const TRAMPOLINE_SEARCH_STRIDE: usize = 0x4000;

// ---------------------------------------------------------------------------
// Windows FFI - raw bindings to kernel32.dll
// ---------------------------------------------------------------------------

#[cfg(windows)]
unsafe extern "system" {
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
}

/// Trampoline cache keyed by the jump target, so repeated far patches to the
/// same function reuse one trampoline instead of leaking a new one.
static TRAMPOLINE_CACHE: LazyLock<Mutex<HashMap<usize, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

// ---------------------------------------------------------------------------
// Patching
// ---------------------------------------------------------------------------

/// Redirect the function at `old_address` so it jumps to `new_address`, by
/// rewriting its prologue (first 5 bytes) with `E9 <rel32>`.
///
/// # Safety
///
/// - `old_address` must point at the start of a real function with at least 5
///   patchable prologue bytes, and must not be concurrently executed.
/// - `new_address` must point at a function with a compatible ABI.
///
/// The caller is responsible for the single-threaded patching contract: the
/// function being patched must not be running on another thread while the
/// prologue bytes are overwritten.
#[cfg(windows)]
pub unsafe fn patch_prologue(old_address: usize, new_address: usize) -> crate::error::Result<()> {
    // The jump destination is old_address + 5 (after the rel32 instruction).
    let jump_source: i64 = old_address as i64 + JMP_REL32_SIZE as i64;
    let relative: i64 = new_address as i64 - jump_source;

    println!(
        "      {} {}",
        dim("patch_prologue:"),
        dim(&format!(
            "{old_address:#x} -> {new_address:#x} (delta {relative:+} bytes)"
        ))
    );

    // Direct jump if the target is within rel32 reach (±2 GB).
    if relative >= i32::MIN as i64 && relative <= i32::MAX as i64 {
        println!(
            "      {}",
            dim("target within rel32 range - writing direct `E9 <rel32>`")
        );
        return write_jmp_rel32(old_address, relative as i32);
    }

    // Otherwise build a trampoline near the original function.
    println!(
        "      {}",
        dim(&format!(
            "target {relative:+} bytes away exceeds rel32 range - building trampoline"
        ))
    );
    let trampoline_address = build_trampoline(old_address, new_address)?;
    let trampoline_relative = trampoline_address as i64 - jump_source;
    if trampoline_relative < i32::MIN as i64 || trampoline_relative > i32::MAX as i64 {
        return Err(LiveCodeError::PatchFailed {
            address: old_address,
            message: "trampoline still out of rel32 range".to_string(),
        });
    }
    println!(
        "      {} {}",
        dim("writing `E9 <rel32>` -> trampoline @"),
        cyan(&format!("{trampoline_address:#x}"))
    );
    write_jmp_rel32(old_address, trampoline_relative as i32)
}

#[cfg(not(windows))]
pub unsafe fn patch_prologue(_old_address: usize, _new_address: usize) -> crate::error::Result<()> {
    Err(LiveCodeError::PatchFailed {
        address: _old_address,
        message: "prologue patching is only supported on Windows x86/x64".to_string(),
    })
}

/// Write a 5-byte `jmp rel32` at `function_address` targeting
/// `function_address + 5 + relative`.
///
/// # Safety
///
/// `function_address` must be the start of a writable-after-`VirtualProtect`
/// code page and must not be concurrently executed.
#[cfg(windows)]
unsafe fn write_jmp_rel32(function_address: usize, relative: i32) -> crate::error::Result<()> {
    let function_start = function_address as *mut u8;

    // Make the code page writable, remembering the previous protection.
    let mut old_protection: u32 = 0;
    // SAFETY: function_start points at real executable code; VirtualProtect
    // only touches the page protection, never the contents.
    let protect_result = unsafe {
        VirtualProtect(
            function_start as *mut std::ffi::c_void,
            JMP_REL32_SIZE,
            PAGE_EXECUTE_READWRITE,
            &mut old_protection,
        )
    };
    if protect_result == 0 {
        return Err(LiveCodeError::PatchFailed {
            address: function_address,
            message: format!(
                "VirtualProtect failed: {}",
                std::io::Error::last_os_error()
            ),
        });
    }

    // Write the jmp rel32: E9 <little-endian rel32>.
    // SAFETY: the page is now writable; we write exactly the 5 prologue bytes.
    unsafe {
        *function_start = JMP_REL32_OPCODE;
        let rel_bytes = relative.to_le_bytes();
        std::ptr::copy_nonoverlapping(rel_bytes.as_ptr(), function_start.add(1), 4);
    }

    println!(
        "      {} {}",
        dim("wrote 5 bytes:"),
        cyan(&format!(
            "E9 {:02X} {:02X} {:02X} {:02X} at {function_address:#x}",
            (relative as u32 & 0xff),
            ((relative as u32 >> 8) & 0xff),
            ((relative as u32 >> 16) & 0xff),
            ((relative as u32 >> 24) & 0xff),
        ))
    );

    // Flush the instruction cache so the CPU executes the new bytes.
    // SAFETY: the patched page contains the new bytes; flushing is required
    // before execution on x86.
    unsafe {
        FlushInstructionCache(
            GetCurrentProcess(),
            function_start as *const std::ffi::c_void,
            JMP_REL32_SIZE,
        );
    }

    // Restore the previous page protection.
    let mut ignored_protection: u32 = 0;
    // SAFETY: restoring protection on the same page we made writable above.
    unsafe {
        VirtualProtect(
            function_start as *mut std::ffi::c_void,
            JMP_REL32_SIZE,
            old_protection,
            &mut ignored_protection,
        );
    }

    Ok(())
}

/// Allocate a trampoline near `old_address` holding `mov rax, imm64; jmp rax`
/// to `new_address`, searching ±1 GB for free memory in 16 KB strides.
///
/// # Safety
///
/// `old_address` must be a valid code address; the returned allocation is
/// executable scratch memory.
#[cfg(windows)]
unsafe fn build_trampoline(old_address: usize, new_address: usize) -> crate::error::Result<usize> {
    // Reuse a cached trampoline for this target if it is reachable.
    if let Some(existing) = TRAMPOLINE_CACHE.lock().unwrap().get(&new_address).copied() {
        let existing_relative = existing as i64 - (old_address as i64 + JMP_REL32_SIZE as i64);
        if existing_relative >= i32::MIN as i64 && existing_relative <= i32::MAX as i64 {
            println!(
                "      {} {}",
                dim("reusing cached trampoline @"),
                cyan(&format!("{existing:#x}"))
            );
            return Ok(existing);
        }
    }

    let low_bound = (old_address as i64 - 0x4000_0000).max(0) as usize; // −1 GB
    let high_bound = old_address as i64 + 0x4000_0000; // +1 GB

    let mut direction: i64 = 1;
    let mut offset: usize = 0;

    loop {
        let candidate = if direction > 0 {
            old_address.wrapping_add(offset)
        } else {
            old_address.wrapping_sub(offset)
        };

        if candidate >= low_bound && (candidate as i64) <= high_bound && candidate != 0 {
            // SAFETY: VirtualAlloc reserves executable scratch memory at a
            // candidate address; a null return means failure (handled below).
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
                // SAFETY: the freshly allocated page is writable; we write the
                // full 12-byte trampoline before it is ever executed.
                unsafe {
                    // mov rax, imm64
                    *trampoline_start = REX_W_PREFIX;
                    *trampoline_start.add(1) = MOV_RAX_IMM64_OPCODE;
                    std::ptr::copy_nonoverlapping(
                        (new_address as u64).to_le_bytes().as_ptr(),
                        trampoline_start.add(2),
                        8,
                    );
                    // jmp rax
                    *trampoline_start.add(10) = JMP_RAX_OPCODE;
                    *trampoline_start.add(11) = JMP_RAX_MODRM;
                }
                // Flush the instruction cache for the trampoline.
                // SAFETY: the trampoline is executable scratch we just wrote.
                unsafe {
                    FlushInstructionCache(GetCurrentProcess(), allocation, TRAMPOLINE_SIZE);
                }
                // Cache it so repeated far patches to this target reuse it.
                TRAMPOLINE_CACHE
                    .lock()
                    .unwrap()
                    .insert(new_address, allocation as usize);
                println!(
                    "      {} {}",
                    dim("allocated trampoline @"),
                    cyan(&format!(
                        "{:#x} near base {old_address:#x} (12 bytes: 48 B8 imm64 FF E0)",
                        allocation as usize
                    ))
                );
                return Ok(allocation as usize);
            }
        }

        direction = -direction;
        if direction > 0 {
            offset += TRAMPOLINE_SEARCH_STRIDE;
        }

        if offset > 0x4000_0000 {
            break;
        }
    }

    Err(LiveCodeError::PatchFailed {
        address: old_address,
        message: "could not allocate a trampoline".to_string(),
    })
}

// ---------------------------------------------------------------------------
// Single-function compilation via rustc
// ---------------------------------------------------------------------------

/// Write `source_code` to a fixed temp `.rs` and compile it to a cdylib with
/// `rustc` directly (bypassing cargo).  Returns the compiled library path.
///
/// The SOURCE path and the incremental cache directory are fixed per crate
/// name, so repeated leaf compiles reuse rustc's incremental state (only the
/// changed body gets re-codegened).  The output library stays unique per
/// compile, because a previously loaded patch DLL locks its file on Windows.
///
/// `edition` selects the Rust edition (e.g. "2021") and `artifact_extension`
/// the shared-library suffix ("dll", "so", "dylib").
pub fn compile_rust_source(
    source_code: &str,
    crate_name: &str,
    edition: &str,
    artifact_extension: &str,
) -> crate::error::Result<std::path::PathBuf> {
    use std::io::Write;

    let temp_dir = std::env::temp_dir();
    let pid = std::process::id();
    // Monotonic counter so every compilation gets a unique DLL filename even
    // within the same process (PID alone isn't enough for repeated reloads).
    static COMPILE_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let counter = COMPILE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    // Fixed source path + persistent incremental dir (keyed by crate name) so
    // rustc's incremental cache is reused across compiles.
    let source_path = temp_dir.join(format!("{crate_name}_leaf.rs"));
    let incremental_dir = temp_dir.join(format!("{crate_name}_leaf_incr"));
    let lib_path = temp_dir.join(format!(
        "{crate_name}_{pid}_{counter}.{artifact_extension}"
    ));

    let mut file = std::fs::File::create(&source_path).map_err(|source| LiveCodeError::SourceIo {
        path: source_path.clone(),
        source,
    })?;
    file.write_all(source_code.as_bytes()).map_err(|source| LiveCodeError::SourceIo {
        path: source_path.clone(),
        source,
    })?;
    drop(file);

    let output = std::process::Command::new("rustc")
        .args(["--crate-type=cdylib", "--crate-name"])
        .arg(crate_name)
        .args(["--edition", edition, "-o"])
        .arg(&lib_path)
        .args(["-C", &format!("incremental={}", incremental_dir.display())])
        .arg(&source_path)
        .output()
        .map_err(|source| LiveCodeError::SourceIo {
            path: source_path.clone(),
            source,
        })?;

    let _ = std::fs::remove_file(&source_path);

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(LiveCodeError::RustcFailed {
            stderr: stderr.to_string(),
        });
    }

    Ok(lib_path)
}
