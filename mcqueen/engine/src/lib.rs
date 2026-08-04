// mini_engine/src/lib.rs — Minimal game engine with dll_patch dispatch
//
// REQUIREMENTS
//   macroquad 0.4, libloading 0.8, dll_patch (workspace dep)
//
// DESCRIPTION
//   Owns the PhysicsState and dispatches game function calls through
//   dll_patch's jump table. When a patch_game.dll is applied, calls
//   are automatically redirected to the new implementations without
//   restarting or reloading the original game.dll.
//
// --- SCRIPT ---

use macroquad::prelude::*;
use std::path::PathBuf;

// ── Renderer ──────────────────────────────────────────────────────────────

pub struct Renderer;

impl Renderer {
    pub fn clear_background(&self, color: Color) {
        clear_background(color);
    }
    pub fn draw_circle(&self, center_x: f32, center_y: f32, radius: f32, color: Color) {
        draw_circle(center_x, center_y, radius, color);
    }
    pub fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        draw_text(text, x, y, font_size, color);
    }
}

// ── Physics State (owned by engine, passed as *mut to game functions) ─────

#[repr(C)]
pub struct PhysicsState {
    pub delta_time: f32,
    pub position_x: f32,
    pub position_y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub radius: f32,
    pub active: bool,
}

impl Default for PhysicsState {
    fn default() -> Self {
        Self {
            delta_time: 0.0,
            position_x: 400.0,
            position_y: 500.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            radius: 20.0,
            active: false,
        }
    }
}

// ── Ball State ────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BallState {
    pub position_x: f32,
    pub position_y: f32,
    pub radius: f32,
    pub color_r: f32,
    pub color_g: f32,
    pub color_b: f32,
    pub color_a: f32,
}

// ── Engine ────────────────────────────────────────────────────────────────

pub struct Engine {
    renderer: Renderer,
    physics_state: PhysicsState,
    /// Handle to game.dll (kept alive for the process lifetime).
    #[allow(dead_code)]
    game_library: libloading::Library,
    /// RVAs of the three hot functions in game.dll.
    function_rvas: FunctionRvas,
    /// dll_patch module info for game.dll.
    original_module: dll_patch::RegisteredModule,
    /// Currently loaded patch DLL (kept alive).
    #[allow(dead_code)]
    patch_library: Option<libloading::Library>,
}

struct FunctionRvas {
    start: u64,
    update: u64,
    get_state: u64,
}

impl Engine {
    /// Load game.dll, register it with dll_patch, and initialize
    /// hot-function dispatch for all three game functions.
    pub fn new(game_dll_path: impl Into<PathBuf>) -> Self {
        let game_dll_path: PathBuf = game_dll_path.into();

        // Load the original game DLL
        let game_library =
            unsafe { libloading::Library::new(&game_dll_path).expect("failed to load game.dll") };

        // Resolve sentinel and register the module
        let sentinel: libloading::Symbol<unsafe extern "C" fn()> = unsafe {
            game_library
                .get(b"__subsecond_anchor")
                .expect("sentinel not found in game.dll")
        };
        let sentinel_ptr = *sentinel as *const ();

        // Use the actual filename (not "game.dll") since the DLL may be
        // loaded from a versioned copy like "game_base.dll" or "game_v0.dll".
        let module_name = game_dll_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("game.dll");

        let original_module = unsafe {
            dll_patch::register_library(module_name, sentinel_ptr as *const std::ffi::c_void)
                .expect("register game.dll with dll_patch")
        };

        // Resolve game functions and compute their RVAs
        let start_sym: libloading::Symbol<unsafe extern "C" fn(*mut PhysicsState)> =
            unsafe { game_library.get(b"start").expect("symbol start") };
        let start_ptr = *start_sym as *const ();
        let start_rva = (start_ptr as usize - original_module.base_address) as u64;

        let update_sym: libloading::Symbol<unsafe extern "C" fn(*mut PhysicsState)> =
            unsafe { game_library.get(b"update").expect("symbol update") };
        let update_ptr = *update_sym as *const ();
        let update_rva = (update_ptr as usize - original_module.base_address) as u64;

        let get_state_sym: libloading::Symbol<
            unsafe extern "C" fn(*const PhysicsState) -> BallState,
        > = unsafe { game_library.get(b"get_state").expect("symbol get_state") };
        let get_state_ptr = *get_state_sym as *const ();
        let get_state_rva = (get_state_ptr as usize - original_module.base_address) as u64;

        // Register each function with dll_patch for dispatch
        dll_patch::init_function(start_rva, start_ptr);
        dll_patch::init_function(update_rva, update_ptr);
        dll_patch::init_function(get_state_rva, get_state_ptr);

        Self {
            renderer: Renderer,
            physics_state: PhysicsState::default(),
            game_library,
            function_rvas: FunctionRvas {
                start: start_rva,
                update: update_rva,
                get_state: get_state_rva,
            },
            original_module,
            patch_library: None,
        }
    }

    /// Call game.start() through the dll_patch jump table.
    pub fn start(&mut self) {
        let ptr: *mut PhysicsState = &mut self.physics_state;
        unsafe { dll_patch::call_hot::<*mut PhysicsState, ()>(self.function_rvas.start, ptr) };
    }

    /// Advance one frame: call game.update(), read game.get_state(), render.
    pub fn update(&mut self, delta_time: f32) {
        self.physics_state.delta_time = delta_time;
        let state_ptr: *mut PhysicsState = &mut self.physics_state;

        // Step physics via jump table
        unsafe {
            dll_patch::call_hot::<*mut PhysicsState, ()>(self.function_rvas.update, state_ptr);
        }

        // Read visual state via jump table
        let ball = unsafe {
            dll_patch::call_hot::<*const PhysicsState, BallState>(
                self.function_rvas.get_state,
                state_ptr as *const PhysicsState,
            )
        };

        // Render
        self.renderer.clear_background(BLACK);
        let ball_color = Color::new(ball.color_r, ball.color_g, ball.color_b, ball.color_a);
        self.renderer
            .draw_circle(ball.position_x, ball.position_y, ball.radius, ball_color);

        let fps_text = format!("FPS: {:.0}", macroquad::time::get_fps());
        self.renderer.draw_text(&fps_text, 10.0, 25.0, 22.0, GRAY);
        self.renderer.draw_text(
            "SPACE=bounce | Edit patch_game/src/lib.rs to hot-patch",
            10.0,
            50.0,
            16.0,
            GRAY,
        );
    }

    /// Apply a function-level patch from a freshly-built patch DLL.
    pub fn apply_patch(&mut self, patch_dll_path: impl Into<PathBuf>) -> Result<(), String> {
        let patch_path: PathBuf = patch_dll_path.into();
        let patch_name = patch_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("patch_game.dll");

        // Load the patch DLL
        let patch_lib = unsafe {
            libloading::Library::new(&patch_path).map_err(|e| format!("load patch DLL: {e}"))?
        };

        // Resolve patch DLL's sentinel
        let patch_sentinel: libloading::Symbol<unsafe extern "C" fn()> = unsafe {
            patch_lib
                .get(b"__subsecond_anchor")
                .map_err(|e| format!("patch sentinel: {e}"))?
        };
        let patch_sentinel_addr = *patch_sentinel as *const () as usize;

        // Original sentinel RVA (link-time offset)
        let original_sentinel_rva =
            self.original_module.sentinel_address - self.original_module.base_address;

        // Helper: compute the RVA of a symbol relative to the original's
        // link-time layout, given its address in the patch DLL.
        let compute_patch_rva = |patch_fn_ptr: usize| -> u64 {
            (patch_fn_ptr.wrapping_sub(patch_sentinel_addr)).wrapping_add(original_sentinel_rva)
                as u64
        };

        // Resolve replacement function pointers in the patch DLL
        let start_new: libloading::Symbol<unsafe extern "C" fn(*mut PhysicsState)> =
            unsafe { patch_lib.get(b"start").map_err(|e| format!("start: {e}"))? };
        let start_new_rva = compute_patch_rva(*start_new as *const () as usize);

        let update_new: libloading::Symbol<unsafe extern "C" fn(*mut PhysicsState)> = unsafe {
            patch_lib
                .get(b"update")
                .map_err(|e| format!("update: {e}"))?
        };
        let update_new_rva = compute_patch_rva(*update_new as *const () as usize);

        let get_state_new: libloading::Symbol<
            unsafe extern "C" fn(*const PhysicsState) -> BallState,
        > = unsafe {
            patch_lib
                .get(b"get_state")
                .map_err(|e| format!("get_state: {e}"))?
        };
        let get_state_new_rva = compute_patch_rva(*get_state_new as *const () as usize);

        // Build the jump table: original_RVA → patch_RVA (pre-rebase)
        let mut table = dll_patch::JumpTable {
            map: {
                let mut m = dll_patch::AddressMap::default();
                m.insert(self.function_rvas.start, start_new_rva);
                m.insert(self.function_rvas.update, update_new_rva);
                m.insert(self.function_rvas.get_state, get_state_new_rva);
                m
            },
            sentinel_rva_original: original_sentinel_rva as u64,
            sentinel_rva_patch: original_sentinel_rva as u64,
        };

        // Apply: rebases with ASLR slides and atomically swaps the global table
        dll_patch::apply_dll_patch_named(&self.original_module, &mut table, patch_name)
            .map_err(|e| format!("apply_dll_patch_named: {e}"))?;

        // Keep the new patch DLL alive; old one is intentionally leaked
        let _old = self.patch_library.replace(patch_lib);
        Ok(())
    }
}
