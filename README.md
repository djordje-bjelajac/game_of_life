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

The codebase follows a lightweight Domain-Driven Design/hexagonal architecture split:

- `src/main.rs` is the outermost adapter, wiring the application into `eframe`.
- `application::game_app::GameApp` is the primary driving adapter/port implementation.
  It owns UI state, translates user input into application commands, and orchestrates
  rendering. This layer never mutates simulation state directly; instead it invokes
  domain services.
- `domain` is the core and remains framework-free. It exposes:
  - `Cell` and `Grid` entities/value objects
  - `rules::next_generation` (pure simulation service + neighbor counting)
  - `patterns::PATTERNS` with predefined offsets
  - `constants.rs` defining safe bounds for sliders

By keeping adapters (`application`, UI) at the edges and the pure domain in the center,
the project stays testable, deterministic, and open to new delivery mechanisms (CLI,
network service, etc.) without modifying domain code.

## Development Notes

- Format: `cargo fmt`
- Lints: `cargo clippy --all-targets --all-features`
- Tests (logic only for now): `cargo test`
- Test layout: each module keeps its unit tests in a sibling `*_test.rs` file (e.g.
  `cell.rs` ↔ `cell_test.rs`) loaded via `#[path = \"...\"]` so code and tests stay co-located.

Feel free to extend the pattern catalogue or add new UI panels by following the
existing domain/application split.

