# Voxel Terrain Generator

A Godot voxel terrain generator written in Rust using [godot-rust (gdext)](https://github.com/godot-rust/gdext).

This project follows and translates the [voxel terrain tutorial series](https://www.youtube.com/watch?v=wuusq4tc9iI&list=PLMQtM2GgbPEW8ElbOfc49QWV0eOquQiic&index=3) from GDScript to Rust.

## Getting Started

### Prerequisites

- Rust
- Godot 4.5.1

### Building

The Rust library must be compiled before running the Godot project:

```bash
cd rust
cargo build
```

### Running

1. Open the Godot project:
```bash
cd godot
godot
```

2. The Rust extension is configured in `godot/rs.gdextension`
3. Press `F5` or `Cmd+B` (macOS) to run the game, or go to **Run** > **Play**

The voxel terrain will be generated and loaded when the scene starts.
