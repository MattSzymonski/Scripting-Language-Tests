//! Shared FFI types used by both the host and every generated module DLL.
//!
//! `ModuleIdentity`'s layout (`#[repr(C)]`) must exactly match the identical
//! definition emitted into every generated crate's `src/lib.rs` by
//! `generate_aaa_modules.py`.

/// Identity/visual info returned by every module's `create_module_instance`
/// export — the "CreateInstance" phase, mirroring instantiating a module's
/// `IModuleInterface` in Unreal.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ModuleIdentity {
    /// Null-terminated ASCII name, max 31 characters + null terminator.
    pub name: [u8; 32],
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
    /// 0 = core, 1 = subsystem, 2 = leaf.
    pub tier: u8,
    /// Display size multiplier (0.5 - 2.0). Always 1.0 for core/subsystem.
    pub size: f32,
}

impl ModuleIdentity {
    pub fn name_string(&self) -> String {
        let end = self.name.iter().position(|&b| b == 0).unwrap_or(self.name.len());
        String::from_utf8_lossy(&self.name[..end]).into_owned()
    }

    pub fn color(&self) -> (f32, f32, f32, f32) {
        (
            self.color_r as f32 / 255.0,
            self.color_g as f32 / 255.0,
            self.color_b as f32 / 255.0,
            1.0,
        )
    }

    pub fn tier(&self) -> ModuleTier {
        match self.tier {
            0 => ModuleTier::Core,
            1 => ModuleTier::Subsystem,
            _ => ModuleTier::Leaf,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleTier {
    Core,
    Subsystem,
    Leaf,
}

impl ModuleTier {
    pub fn from_str(s: &str) -> Self {
        match s {
            "core" => ModuleTier::Core,
            "subsystem" => ModuleTier::Subsystem,
            _ => ModuleTier::Leaf,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ModuleTier::Core => "core",
            ModuleTier::Subsystem => "subsystem",
            ModuleTier::Leaf => "leaf",
        }
    }
}

impl std::fmt::Display for ModuleTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

/// Function pointer types for every module's exported entry points.
pub type CreateInstanceFn = unsafe extern "C" fn() -> ModuleIdentity;
pub type ModuleStartupFn = unsafe extern "C" fn() -> u32;

/// Per-module load metrics, mirroring `AAA-Scenario.csv`'s column layout so
/// a run of this harness can be diffed directly against the real Unreal
/// reference data in that file.
#[derive(Debug, Clone)]
pub struct ModuleMetrics {
    pub name: String,
    pub tier: ModuleTier,
    pub total_ms: f64,
    pub load_library_ms: f64,
    pub get_symbol_ms: f64,
    pub create_instance_ms: f64,
    pub startup_module_ms: f64,
    pub status: &'static str,
    pub file_size_bytes: u64,
    pub memory_used_bytes: i64,
    pub num_exported_functions: u32,
    pub num_imported_dlls: u32,
    /// Dependencies (from the manifest's transitive closure) that were NOT
    /// yet resident before this module's `LoadLibrary` call, but were
    /// resident immediately after — i.e. modules this load transitively
    /// pulled in. Empty most of the time; non-empty rows are exactly the
    /// "unlucky first loader" outliers described in the write-up's section 6.
    pub newly_loaded_deps: Vec<String>,
}
