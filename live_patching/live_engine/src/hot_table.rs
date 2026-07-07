//! # Hot Function Table
//!
//! Stores current function pointers for all hot-patchable game entry points.
//! The engine calls through this table **exclusively** — never directly to the
//! game library.
//!
//! ## Patching protocol
//!
//! 1. Compile new game code → new cdylib
//! 2. Load new library via `libloading`
//! 3. Resolve new function symbols
//! 4. Call [`HotFnTable::patch_update`] / [`patch_render`] — these atomically
//!    swap the pointer using `Ordering::Release`
//! 5. The engine picks up the new pointer on the next frame via
//!    `Ordering::Acquire`
//! 6. The old library is **kept loaded** (not dropped) so in-flight calls
//!    on other threads complete safely
//!
//! ## Why this works
//!
//! - `extern "C"` functions have a stable ABI — the engine doesn't care which
//!   library version they came from.
//! - Atomic pointer swaps guarantee that the engine either sees the old
//!   pointer or the new one, never a torn write.
//! - Each function slot is independent — you could patch `update` without
//!   touching `render`.

use std::sync::atomic::{AtomicPtr, Ordering};

use crate::api::{GameInitFn, GameRenderFn, GameUpdateFn, GameWorldSizeFn};

/// The live-patchable function table.
///
/// Every game logic call from the engine goes through this struct.
pub struct HotFnTable {
    /// Called once at startup (rarely patched, but possible).
    pub init: AtomicPtr<()>,

    /// Called every frame: game_update(api, world, dt).
    pub update: AtomicPtr<()>,

    /// Called every frame: game_render(api, world).
    pub render: AtomicPtr<()>,

    /// Called once before init: returns needed world state byte size.
    pub world_size: AtomicPtr<()>,
}

impl HotFnTable {
    /// Create a new empty table (all slots null).
    pub fn new() -> Self {
        Self {
            init: AtomicPtr::new(std::ptr::null_mut()),
            update: AtomicPtr::new(std::ptr::null_mut()),
            render: AtomicPtr::new(std::ptr::null_mut()),
            world_size: AtomicPtr::new(std::ptr::null_mut()),
        }
    }

    // -----------------------------------------------------------------------
    // Atomic patching operations
    // -----------------------------------------------------------------------

    /// Atomically swap the init function pointer.
    ///
    /// Returns the **old** pointer (caller should keep the old library alive).
    pub fn patch_init(&self, new_fn: GameInitFn) -> GameInitFn {
        let old = self.init.swap(new_fn as *mut (), Ordering::Release);
        unsafe { std::mem::transmute(old) }
    }

    /// Atomically swap the update function pointer.
    ///
    /// Returns the **old** pointer (caller should keep the old library alive).
    pub fn patch_update(&self, new_fn: GameUpdateFn) -> GameUpdateFn {
        let old = self.update.swap(new_fn as *mut (), Ordering::Release);
        unsafe { std::mem::transmute(old) }
    }

    /// Atomically swap the render function pointer.
    ///
    /// Returns the **old** pointer (caller should keep the old library alive).
    pub fn patch_render(&self, new_fn: GameRenderFn) -> GameRenderFn {
        let old = self.render.swap(new_fn as *mut (), Ordering::Release);
        unsafe { std::mem::transmute(old) }
    }

    /// Atomically swap the world_size function pointer.
    pub fn patch_world_size(&self, new_fn: GameWorldSizeFn) -> GameWorldSizeFn {
        let old = self.world_size.swap(new_fn as *mut (), Ordering::Release);
        unsafe { std::mem::transmute(old) }
    }

    // -----------------------------------------------------------------------
    // Reading (engine uses these every frame)
    // -----------------------------------------------------------------------

    /// Read the current update function pointer (Acquire ordering).
    pub fn read_update(&self) -> GameUpdateFn {
        let ptr = self.update.load(Ordering::Acquire);
        if ptr.is_null() {
            // No game function loaded yet — return a no-op stub.
            return stub_update;
        }
        unsafe { std::mem::transmute(ptr) }
    }

    /// Read the current render function pointer (Acquire ordering).
    pub fn read_render(&self) -> GameRenderFn {
        let ptr = self.render.load(Ordering::Acquire);
        if ptr.is_null() {
            return stub_render;
        }
        unsafe { std::mem::transmute(ptr) }
    }

    /// Read the current init function pointer (Acquire ordering).
    pub fn read_init(&self) -> GameInitFn {
        let ptr = self.init.load(Ordering::Acquire);
        if ptr.is_null() {
            return stub_init;
        }
        unsafe { std::mem::transmute(ptr) }
    }

    /// Read the current world_size function pointer (Acquire ordering).
    pub fn read_world_size(&self) -> GameWorldSizeFn {
        let ptr = self.world_size.load(Ordering::Acquire);
        if ptr.is_null() {
            return stub_world_size;
        }
        unsafe { std::mem::transmute(ptr) }
    }
}

// ---------------------------------------------------------------------------
// Fallback stubs — used when no game functions are loaded yet
// ---------------------------------------------------------------------------

extern "C" fn stub_update(
    _api: *const crate::api::EngineAPI,
    _engine: crate::api::EngineHandle,
    _world: crate::api::WorldHandle,
    _dt: f32,
) {
    // No game loaded — nothing to update.
}

extern "C" fn stub_render(
    _api: *const crate::api::EngineAPI,
    _engine: crate::api::EngineHandle,
    _world: crate::api::WorldHandle,
) {
    // No game loaded — nothing to render.
}

extern "C" fn stub_init(
    _api: *const crate::api::EngineAPI,
    _engine: crate::api::EngineHandle,
    _world: crate::api::WorldHandle,
) {
    // No game loaded.
}

extern "C" fn stub_world_size() -> usize {
    0
}
