# classic_full_hot_reload

Classic full DLL hot-reload: edit game code, recompile the entire crate to a new DLL,
load it, and swap — no restart needed.

Each rebuild produces a new `game_v1.dll`, `game_v2.dll`, etc. Old DLLs stay loaded
in the process (Windows reference counting). The host simply switches to calling the
newest one.

## Running

```powershell
cargo run -p host
```

The host watches `game_lib/src/lib.rs`. Edit any function, save, and it auto-rebuilds
and reloads within seconds.

## Architecture

```
host.exe                 watches source, rebuilds crate, loads newest DLL
game_lib.dll             cdylib — the game code
game_v1.dll              first loaded version
game_v2.dll              second loaded version (after first edit)
game_v3.dll              ...
```

## Pipeline

1. **Startup** — host builds `game_lib`, copies to `game_v1.dll`, loads it, resolves
   `update` and `compute` via `GetProcAddress`.

2. **Tick** — every 800ms the host calls `update(tick)` and `compute(tick, 3)` from the
   currently loaded DLL.

3. **Edit** — you change a function in `game_lib/src/lib.rs` and save.

4. **Detect** — the host polls `file_modified_time()` and sees the change.

5. **Rebuild** — `cargo build -p game_lib --target-dir target_reload` recompiles the
   entire crate to a fresh DLL. A separate target directory avoids locking the
   currently loaded DLL.

6. **Copy** — the new DLL is copied to `game_v{N}.dll` with an incrementing version
   number.

7. **Load** — `LoadLibraryW("game_v{N}.dll")` maps it into the process.

8. **Resolve** — `GetProcAddress` gets the new `update` and `compute` addresses.

9. **Swap** — the host's function pointers are updated to point at the new DLL.
   The old `game_v{N-1}.dll` stays loaded but is no longer called.

10. **Next tick** — the host calls the new functions from the latest DLL.

