//! Shared FFI types used by both the host and plugin DLLs.
//!
//! These types use `#[repr(C)]` for a stable ABI across compilation units.
//! The plugin DLLs include an identical copy of `BirdDefinition` (see the
//! generated `plugin_XXX/src/lib.rs` files).

/// Definition of a bird rendered on the stress-test menu screen.
///
/// This struct is returned by each plugin's `create_bird()` function.
/// It must be identical in layout between the host and every plugin DLL.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BirdDefinition {
    /// Null-terminated ASCII name, max 31 characters + null terminator.
    pub name: [u8; 32],
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
    /// Display size multiplier (0.5 – 2.0).
    pub size: f32,
}

impl BirdDefinition {
    /// Decode the name buffer into a Rust `String`.
    pub fn name_string(&self) -> String {
        let end = self
            .name
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.name.len());
        String::from_utf8_lossy(&self.name[..end]).into_owned()
    }

    /// Convert the RGB bytes into a macroquad `Color`.
    pub fn color(&self) -> (f32, f32, f32, f32) {
        (
            self.color_r as f32 / 255.0,
            self.color_g as f32 / 255.0,
            self.color_b as f32 / 255.0,
            1.0,
        )
    }
}

/// Function pointer type for `create_bird`.
///
/// Each plugin DLL exports exactly one function with this signature.
pub type CreateBirdFn = unsafe extern "C" fn() -> BirdDefinition;

/// Metadata about a loaded plugin.
#[derive(Debug, Clone)]
pub struct PluginInfo {
    /// Plugin index (000–299).
    pub index: u32,
    /// Plugin crate name, e.g. `"plugin_042"`.
    pub name: String,
    /// The bird definition this plugin produces.
    pub bird: BirdDefinition,
    /// Whether this plugin was loaded successfully.
    pub loaded: bool,
}
