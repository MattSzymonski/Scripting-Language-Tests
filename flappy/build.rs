//! Build the C# game (`game_cs`) before the host, so `cargo run -p flappy`
//! is a single command. Best-effort: if `dotnet` is missing the host still
//! builds and reports a friendly error at runtime.

use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let project = manifest.join("..").join("game_cs");

    // Rebuild whenever the C# project or any of its sources change.
    println!(
        "cargo:rerun-if-changed={}",
        project.join("game_cs.csproj").display()
    );
    if let Ok(entries) = std::fs::read_dir(project.join("src")) {
        for entry in entries.flatten() {
            println!("cargo:rerun-if-changed={}", entry.path().display());
        }
    }

    let Some(project_str) = project.to_str() else {
        println!("cargo:warning=game_cs path is not valid UTF-8; build it manually");
        return;
    };

    match Command::new("dotnet")
        .args([
            "build",
            project_str,
            "-c",
            "Release",
            "--nologo",
            "-v",
            "quiet",
        ])
        .status()
    {
        Ok(status) if status.success() => {}
        Ok(status) => println!(
            "cargo:warning=`dotnet build game_cs` failed ({status}); \
             run it manually before launching"
        ),
        Err(err) => println!(
            "cargo:warning=could not run dotnet ({err}); \
             build game_cs with `dotnet build game_cs -c Release`"
        ),
    }
}
