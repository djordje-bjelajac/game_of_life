use std::io::{self, Write};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    style::Print,
    terminal::{self, Clear, ClearType},
};

use crate::grid::Grid;

const ALIVE_CHAR: &str = "█";
const DEAD_CHAR: &str = "·";

pub struct Renderer {
    stdout: io::Stdout,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
        }
    }

    pub fn init(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(self.stdout, Hide, EnableMouseCapture, Clear(ClearType::All))?;
        Ok(())
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        execute!(self.stdout, Show, DisableMouseCapture, Clear(ClearType::All))?;
        Ok(())
    }

    pub fn render(&mut self, grid: &Grid, generation: u64, paused: bool) -> io::Result<()> {
        execute!(self.stdout, MoveTo(0, 0))?;

        // Render grid
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let char = if grid.get(x, y).is_alive() { ALIVE_CHAR } else { DEAD_CHAR };
                execute!(self.stdout, Print(char))?;
            }
            execute!(self.stdout, Print("\r\n"))?;
        }

        // Render status line
        let status = if paused { "PAUSED" } else { "RUNNING" };
        execute!(
            self.stdout,
            Print(format!(
                "\r\nGeneration: {} | Status: {}\r\n",
                generation, status
            ))
        )?;

        // Render controls
        execute!(
            self.stdout,
            Print("Controls: [Space] Pause/Resume | [R] Randomize | [C] Clear | [Q] Quit | [Click] Draw\r\n")
        )?;

        self.stdout.flush()?;
        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}
