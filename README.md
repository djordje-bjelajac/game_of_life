# Conway's Game of Life

A desktop Conway's Game of Life simulator written in Rust on top of `eframe`/`egui`.  
The project is organized using a lightweight domain/application layering: the `domain`
module contains pure simulation logic, while `application` owns the GUI state and
interaction loop.

## Current Status

- GPU-accelerated window rendered through `eframe::run_native`
- Adjustable grid (width and height sliders, 10×10 up to 200×200)
- Simulation speed control (1–60 updates/second) with real-time pause/resume
- Interactive drawing with the mouse (left = alive, right = dead, drag supported)
- Pattern library (Glider, Blinker, Pulsar, Gosper Glider Gun) centered into the grid
- Color customization for alive, dead, background and grid-line colors
- Live statistics panel (generation counter, alive cells, grid and UPS summary)

## Requirements

- Rust toolchain (stable) with `cargo`
- A desktop environment capable of running `egui`/`winit` applications (macOS, Linux, Windows)

## Build & Run

```bash
cargo run --release
```

Use `cargo run` without `--release` for faster iterative builds. Release mode is
recommended for smooth animation on larger grids.

## Controls & Interaction

| Input / Action | Result |
| --- | --- |
| `Space` | Toggle pause / resume |
| `R` | Randomize the entire grid |
| `C` | Clear (set all cells to dead) |
| Mouse left click / drag | Paint cells alive |
| Mouse right click / drag | Paint cells dead |
| Pattern dropdown + “Insert Pattern” | Spawn the selected pattern at grid center |
| Grid sliders | Resize grid immediately while preserving overlapping cells |
| UPS slider | Change simulation speed (updates per second) |
| Color pickers | Update palette in real time |

## Architecture Overview

- `src/main.rs` sets up the `eframe` window and instantiates `GameApp`.
- `application::game_app::GameApp` implements `eframe::App`, manages UI state,
  user input, rendering, and delegates pure logic to the domain layer.
- `domain` exposes:
  - `Cell` and `Grid` primitives
  - `rules::next_generation` (and supporting neighbor counting)
  - `patterns::PATTERNS` with predefined offsets
  - `constants.rs` defining safe bounds for sliders

This separation keeps the simulation deterministic and testable while letting
the UI evolve independently.

## Development Notes

- Format: `cargo fmt`
- Lints: `cargo clippy --all-targets --all-features`
- Tests (logic only for now): `cargo test`
- Test layout: each module keeps its unit tests in a sibling `*_test.rs` file (e.g.
  `cell.rs` ↔ `cell_test.rs`) loaded via `#[path = \"...\"]` so code and tests stay co-located.

Feel free to extend the pattern catalogue or add new UI panels by following the
existing domain/application split.

