# Conway's Game of Life

A terminal-based implementation of Conway's Game of Life written in Rust.

## Rules

Conway's Game of Life follows these simple rules:

1. Any live cell with 2-3 neighbors survives
2. Any dead cell with exactly 3 neighbors becomes alive
3. All other cells die or stay dead

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

## Controls

| Input         | Action              |
|---------------|---------------------|
| `Space`       | Pause/Resume        |
| `R`           | Randomize grid      |
| `C`           | Clear grid          |
| `Q`           | Quit                |
| Left Click    | Draw cells (alive)  |
| Left Drag     | Draw while dragging |
| Right Click   | Erase cells (dead)  |

## Configuration

Default settings in `src/main.rs`:

- Grid size: 40x20
- Tick speed: 100ms

