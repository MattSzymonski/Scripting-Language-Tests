//! Pure-Rust build by default. When the `cs` feature is enabled
//! (`cargo run --features cs`), `dotnet build` is invoked for both `game_cs`
//! (the game logic) and `game_cs_loader` (the stable, never-reloaded loader
//! that makes `game_cs` hot-reloadable) before the host runs, so both
//! assemblies are ready at runtime.

use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // Only build the C# projects when the `cs` feature is active.
    if std::env::var("CARGO_FEATURE_CS").is_err() {
        return;
    }

    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");

    build_dotnet_project(&workspace, "game_cs");
    build_dotnet_project(&workspace, "game_cs_loader");
}

/// Rebuild `{workspace}/{name}` in Release whenever its project file or
/// sources change. Failures are reported as Cargo warnings rather than hard
/// errors, so the host still builds and can print a clear runtime message.
fn build_dotnet_project(workspace: &Path, name: &str) {
    let project = workspace.join(name);

    println!(
        "cargo:rerun-if-changed={}",
        project.join(format!("{name}.csproj")).display()
    );
    if let Ok(entries) = std::fs::read_dir(project.join("src")) {
        for entry in entries.flatten() {
            println!("cargo:rerun-if-changed={}", entry.path().display());
        }
    }

    let Some(project_str) = project.to_str() else {
        println!("cargo:warning={name} path is not valid UTF-8; build it manually");
        return;
    };

    match Command::new("dotnet")
        .args(["build", project_str, "-c", "Release", "--nologo", "-v", "quiet"])
        .status()
    {
        Ok(status) if status.success() => {}
        Ok(status) => println!(
            "cargo:warning=`dotnet build {name}` failed ({status}); \
             run it manually before launching"
        ),
        Err(err) => println!(
            "cargo:warning=could not run dotnet ({err}); \
             build {name} with `dotnet build {name} -c Release`"
        ),
    }
}
