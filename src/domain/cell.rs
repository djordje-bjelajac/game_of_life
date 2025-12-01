#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Alive,
    Dead,
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        matches!(self, Cell::Alive)
    }

    /// Computes next state based on Game of Life rules:
    /// - Live cell with 2-3 neighbors survives
    /// - Dead cell with exactly 3 neighbors becomes alive
    /// - All other cells die or stay dead
    pub fn next_state(&self, neighbor_count: u8) -> Cell {
        match (self, neighbor_count) {
            (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
            (Cell::Dead, 3) => Cell::Alive,
            _ => Cell::Dead,
        }
    }
}

