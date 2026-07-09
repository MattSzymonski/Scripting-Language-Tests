//! Heavy common — simulates an engine core module that all plugins depend on.
//!
//! Depends on real crates (`image`, `serde`, `serde_json`) to create a
//! realistic dependency tree that the Windows loader must resolve per DLL.

use image::{DynamicImage, RgbaImage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

// ── Bird definition types (shared between host and plugins) ──

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirdDefinition {
    pub name: [u8; 32],
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
    pub size: f32,
}

// ── Simulated engine subsystems (bloat for dependency resolution) ──

static REGISTRY_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Simulated type registry — like Unreal's UObject registration table.
pub fn register_type(name: &str) -> u32 {
    let hash = name.bytes().fold(0u64, |h, b| h.wrapping_mul(31).wrapping_add(b as u64));
    let mut map: HashMap<u64, String> = HashMap::new();
    for i in 0..200 {
        map.insert(hash.wrapping_add(i), name.to_string());
    }
    // Force serde serialization of the map — exercises the dep tree
    let json = serde_json::to_string(&map).unwrap_or_default();
    let _ = std::hint::black_box(json.len());
    REGISTRY_COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// Simulated reflection data — like Unreal's FProperty system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDescriptor {
    pub name: String,
    pub type_name: String,
    pub offset: usize,
    pub size: usize,
    pub flags: u32,
}

/// Simulated class descriptor — like Unreal's UClass.
#[derive(Serialize, Deserialize)]
pub struct ClassDescriptor {
    pub class_name: String,
    pub properties: Vec<PropertyDescriptor>,
    pub class_flags: u32,
}

impl ClassDescriptor {
    pub fn for_bird() -> Self {
        Self {
            class_name: "Bird".to_string(),
            properties: vec![
                PropertyDescriptor { name: "Name".into(), type_name: "FString".into(), offset: 0, size: 32, flags: 0x0001 },
                PropertyDescriptor { name: "Color".into(), type_name: "FColor".into(), offset: 32, size: 4, flags: 0x0001 },
                PropertyDescriptor { name: "Size".into(), type_name: "float".into(), offset: 36, size: 4, flags: 0x0001 },
                PropertyDescriptor { name: "Sprite".into(), type_name: "FTexture2D".into(), offset: 40, size: 8, flags: 0x0002 },
                PropertyDescriptor { name: "Animation".into(), type_name: "UAnimSequence".into(), offset: 48, size: 8, flags: 0x0004 },
                PropertyDescriptor { name: "CollisionRadius".into(), type_name: "float".into(), offset: 56, size: 4, flags: 0x0001 },
                PropertyDescriptor { name: "MaxHealth".into(), type_name: "float".into(), offset: 60, size: 4, flags: 0x0001 },
            ],
            class_flags: 0x0100,
        }
    }
}

/// Simulate heavy static initialization.
pub fn initialize_plugin_module(plugin_name: &str) {
    register_type(plugin_name);
    let class = ClassDescriptor::for_bird();

    // Simulate work: serialize to JSON (forces serde_json codegen), create a test image
    let json = serde_json::to_string_pretty(&class).unwrap_or_default();
    let _ = std::hint::black_box(json.len());

    // Create a small test image — forces image crate code into the dependency tree
    let img = RgbaImage::new(16, 16);
    let _ = std::hint::black_box(img.width());

    // Simulate texture asset loading
    let dynamic = DynamicImage::new_rgba8(16, 16);
    let _ = std::hint::black_box(dynamic.color());
}

// ── Static data bloat ───────────────────────────────────

pub const STATIC_LOOKUP: [u32; 4096] = generate_lookup();

const fn generate_lookup() -> [u32; 4096] {
    let mut table = [0u32; 4096];
    let mut i = 0;
    while i < 4096 {
        table[i] = (i as u32).wrapping_mul(2654435761);
        i += 1;
    }
    table
}
