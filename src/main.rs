mod cell;
mod grid;
mod renderer;
mod rules;

use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind};

use cell::Cell;
use grid::Grid;
use renderer::Renderer;
use rules::next_generation;

const GRID_WIDTH: usize = 80;
const GRID_HEIGHT: usize = 40;
const TICK_DURATION: Duration = Duration::from_millis(100);

fn main() -> std::io::Result<()> {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    grid.randomize();

    let mut renderer = Renderer::new();
    renderer.init()?;

    let mut generation: u64 = 0;
    let mut paused = false;

    loop {
        // Render current state
        renderer.render(&grid, generation, paused)?;

        // Poll for keyboard and mouse events with timeout
        if event::poll(TICK_DURATION)? {
            match event::read()? {
                Event::Key(key_event) => {
                    if key_event.kind == KeyEventKind::Press {
                        match key_event.code {
                            KeyCode::Char('q') | KeyCode::Char('Q') => {
                                break;
                            }
                            KeyCode::Char(' ') => {
                                paused = !paused;
                            }
                            KeyCode::Char('r') | KeyCode::Char('R') => {
                                grid.randomize();
                                generation = 0;
                            }
                            KeyCode::Char('c') | KeyCode::Char('C') => {
                                grid.clear();
                                generation = 0;
                            }
                            _ => {}
                        }
                    }
                }
                Event::Mouse(mouse_event) => {
                    if let MouseEventKind::Down(MouseButton::Left) | MouseEventKind::Drag(MouseButton::Left) = mouse_event.kind {
                        let x = mouse_event.column as usize;
                        let y = mouse_event.row as usize;
                        if x < GRID_WIDTH && y < GRID_HEIGHT {
                            grid.set(x, y, Cell::Alive);
                        }
                    } else if let MouseEventKind::Down(MouseButton::Right) = mouse_event.kind {
                        let x = mouse_event.column as usize;
                        let y = mouse_event.row as usize;
                        if x < GRID_WIDTH && y < GRID_HEIGHT {
                            grid.set(x, y, Cell::Dead);
                        }
                    }
                }
                _ => {}
            }
        }

        // Update simulation if not paused
        if !paused {
            grid = next_generation(&grid);
            generation += 1;
        }
    }

    renderer.cleanup()?;
    Ok(())
}
