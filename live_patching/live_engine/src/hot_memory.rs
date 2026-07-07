//! # Hot Memory — executable page allocator
//!
//! Provides `alloc_rwx(size)` and `make_executable(ptr, size)` using
//! `VirtualAlloc` / `VirtualProtect` on Windows.
//!
//! ## Simulation note
//!
//! In a **true function-level live-patching** system you would:
//! 1. Compile individual functions to position-independent machine code
//! 2. Copy that code into these RWX pages
//! 3. Relocate any absolute addresses
//! 4. Flush the instruction cache
//!
//! In this **prototype** we:
//! - Use the allocator infrastructure (real VirtualAlloc calls)
//! - But load whole cdylibs via `libloading` instead of copying individual
//!   function bytes (the DLL loader handles relocation for us)
//! - The hot memory pages are available for future JIT-style patching
//!
//! The infrastructure is real; the current usage is through DLL loading.

use std::ptr;

/// Size of an individual executable page allocation.
const PAGE_SIZE: usize = 4096;

/// A block of executable memory.
pub struct ExecutableMemory {
    ptr: *mut u8,
    size: usize,
}

// SAFETY: The memory is process-local; sending across threads is fine.
unsafe impl Send for ExecutableMemory {}
unsafe impl Sync for ExecutableMemory {}

impl ExecutableMemory {
    /// Allocate `size` bytes of memory with Read + Write + Execute permissions.
    ///
    /// Rounds up to the nearest page boundary.
    pub fn alloc_rwx(size: usize) -> Option<Self> {
        let aligned = align_up(size, PAGE_SIZE);

        #[cfg(windows)]
        {
            // VirtualAlloc with PAGE_EXECUTE_READWRITE
            let ptr = unsafe {
                VirtualAlloc(
                    ptr::null_mut(),
                    aligned,
                    0x3000, // MEM_COMMIT | MEM_RESERVE
                    0x40,   // PAGE_EXECUTE_READWRITE
                )
            };
            if ptr.is_null() {
                return None;
            }
            Some(Self {
                ptr: ptr as *mut u8,
                size: aligned,
            })
        }

        #[cfg(not(windows))]
        {
            // POSIX: mmap not yet implemented in this prototype.
            // Would use: libc::mmap with PROT_READ|PROT_WRITE|PROT_EXEC
            return None;
        }
    }

    /// Returns a mutable pointer to the start of the memory.
    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    /// Size of the allocation in bytes.
    pub fn size(&self) -> usize {
        self.size
    }
}

impl Drop for ExecutableMemory {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }
        #[cfg(windows)]
        unsafe {
            VirtualFree(self.ptr as *mut std::ffi::c_void, 0, 0x8000); // MEM_RELEASE
        }
        #[cfg(not(windows))]
        {
            // POSIX munmap not yet implemented in this prototype.
            let _ = (self.ptr, self.size);
        }
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

// ---------------------------------------------------------------------------
// Windows API bindings (minimal, no external crate needed)
// ---------------------------------------------------------------------------

#[cfg(windows)]
extern "system" {
    fn VirtualAlloc(
        lpAddress: *const std::ffi::c_void,
        dwSize: usize,
        flAllocationType: u32,
        flProtect: u32,
    ) -> *mut std::ffi::c_void;

    fn VirtualFree(lpAddress: *mut std::ffi::c_void, dwSize: usize, dwFreeType: u32) -> i32;
}
