# Bevy Hex Board 2D

This repository demonstrates how to build a 2D hexagon game board with Rust
and Bevy. The project starts with a minimal Bevy application and will grow into
an interactive board sandbox with generated hex tiles, hover/selection, camera
controls, and deterministic board logic.

## Requirements

- Rust toolchain with Cargo
- System dependencies required by Bevy for your platform

## Development

Run the local Bevy app:

```sh
cargo run
```

Type-check the project:

```sh
cargo check
```

Run tests:

```sh
cargo test
```

Format all Rust code:

```sh
cargo fmt --all
```

Run Clippy with warnings treated as errors:

```sh
cargo clippy --all-targets --all-features -- -D warnings
```

## Project Structure

- `Cargo.toml` defines the Rust package and Bevy dependency.
- `src/main.rs` starts the Bevy app and spawns the 2D camera.
- `src/lib.rs` contains reusable logic and constants that future board modules
  can build on.
- `assets/` will hold runtime assets if the demo later needs textures, fonts,
  shaders, maps, or other Bevy-loaded files.
