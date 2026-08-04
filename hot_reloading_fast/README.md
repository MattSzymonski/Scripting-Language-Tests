cargo run

Attempt 1.
Hot reload speed is around 800ms

Attempt 2.
with ldd linker hot reload speed is around 820ms
```
[target.x86_64-pc-windows-msvc]
linker = "C:/Users/mateusz.szymonski/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/x86_64-pc-windows-msvc/bin/rust-lld.exe"
rustflags = ["-C", "linker-flavor=lld-link"]
```