// thinlink.rs — rustc-direct recompilation (ThinLink concept)
//
// Instead of running full `cargo build -p patch_plugin`, this module invokes
// rustc directly on the changed source file. This demonstrates the key insight
// behind Subsecond's ThinLink: replay the captured compiler invocation, produce
// only the changed codegen unit, and link against the pre-existing dependency
// rlibs — avoiding the full Cargo dependency resolution and metadata rebuild.
//
// In production Subsecond, the CLI captures every crate's rustc args via
// RUSTC_WORKSPACE_WRAPPER and replays them with a custom linker (ThinLink).
// Here we do a simplified version that invokes rustc directly.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Rebuild the patch DLL using rustc directly instead of cargo.
///
/// Returns the path to the rebuilt DLL on success.
pub fn rebuild_patch_direct(
    workspace_root: &Path,
    patch_number: u32,
) -> Result<PathBuf, String> {
    let build_dir = workspace_root.join("target").join("debug");
    let output_name = format!("patch_plugin_v{patch_number}.dll");
    let output_path = build_dir.join(&output_name);

    // Resolve paths relative to the workspace
    let patch_src = workspace_root.join("patch_plugin").join("src").join("lib.rs");
    let deps_dir = build_dir.clone(); // rlibs and dependency artifacts

    // ── 1. Compile the changed source to an object file ──────────────────
    // In production ThinLink, this step replays the captured rustc args for
    // only the codegen units affected by the change. Here we recompile the
    // entire crate as a simplification.
    let object_file = build_dir.join(format!("patch_v{patch_number}.o"));
    let mut compile = Command::new("rustc");
    compile
        .arg(&patch_src)
        .arg("--crate-name").arg("patch_plugin")
        .arg("--crate-type").arg("cdylib")
        .arg("--edition").arg("2024")
        .arg("--emit").arg("obj")
        .arg("-o").arg(&object_file)
        .arg("-L").arg(&deps_dir)             // search path for dependencies
        .arg("--out-dir").arg(&build_dir)
        .current_dir(workspace_root);

    // Add the original plugin as an extern (so patch can reference its symbols)
    compile.arg("--extern").arg(format!("plugin={}", build_dir.join("plugin.dll").display()));

    let status = compile.status().map_err(|e| format!("rustc spawn failed: {e}"))?;
    if !status.success() {
        return Err(format!("rustc compilation failed (exit {status})"));
    }
    println!("[thinlink] compiled {patch_number} → {}", object_file.display());

    // ── 2. Link the object into a DLL ────────────────────────────────────
    // ThinLink would use the captured linker args here. We use a minimal
    // link invocation for demonstration.
    let mut link = Command::new("rustc");
    link
        .arg(&object_file)
        .arg("--crate-name").arg("patch_plugin")
        .arg("--crate-type").arg("cdylib")
        .arg("--edition").arg("2024")
        .arg("-o").arg(&output_path)
        .arg("-L").arg(&deps_dir)
        .current_dir(workspace_root);

    let status = link.status().map_err(|e| format!("link spawn failed: {e}"))?;
    if !status.success() {
        return Err(format!("linking failed (exit {status})"));
    }

    // Also generate a PDB (rustc does this automatically in debug mode)
    println!("[thinlink] linked → {}", output_name);

    Ok(output_path)
}
