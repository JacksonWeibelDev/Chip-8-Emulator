# CHIP-8 Emulator

A CHIP-8 emulator written in Rust, with a raylib desktop frontend.

## Build and run on Windows

Install the stable Rust MSVC toolchain, Visual Studio Build Tools with the C++ workload, CMake, and LLVM/Clang (including libclang). See the [raylib-rs Windows setup guide](https://raylib-rs.github.io/raylib-rs/book/getting-started/install-windows.html) for details. The Rust raylib crate builds its own native library; an existing raylib installation is not required.

From the repository root, run a CHIP-8 ROM with:

```powershell
cargo run --manifest-path desktop/Cargo.toml -- path\to\game.ch8
```

The emulator runs at a target of 60 frames per second with ten instructions per frame. Escape or the window close button exits. The sound timer runs, but audio playback is not yet implemented.

## Controls

| Keyboard | CHIP-8 |
| --- | --- |
| 1 2 3 4 | 1 2 3 C |
| Q W E R | 4 5 6 D |
| A S D F | 7 8 9 E |
| Z X C V | A 0 B F |

Run core tests with `cargo test --manifest-path chip8_core/Cargo.toml`.
