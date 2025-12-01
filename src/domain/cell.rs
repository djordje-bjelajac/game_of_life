#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn is_alive_reflects_state() {
        let alive = Cell::Alive;
        assert!(alive.is_alive());

        let dead = Cell::Dead;
        assert!(!dead.is_alive());
    }

    #[test]
    fn next_state_live_cell_underpopulation() {
        let cell = Cell::Alive;
        assert_eq!(cell.next_state(0), Cell::Dead);
        assert_eq!(cell.next_state(1), Cell::Dead);
    }

    #[test]
    fn next_state_live_cell_survives_with_two_or_three_neighbors() {
        let cell = Cell::Alive;
        assert_eq!(cell.next_state(2), Cell::Alive);
        assert_eq!(cell.next_state(3), Cell::Alive);
    }

    #[test]
    fn next_state_live_cell_overpopulation() {
        let cell = Cell::Alive;
        for neighbors in 4..=8 {
            assert_eq!(cell.next_state(neighbors), Cell::Dead);
        }
    }

    #[test]
    fn next_state_dead_cell_reproduction() {
        let cell = Cell::Dead;
        assert_eq!(cell.next_state(3), Cell::Alive);
    }

    #[test]
    fn next_state_dead_cell_stays_dead_otherwise() {
        let cell = Cell::Dead;
        for neighbors in 0..=8 {
            if neighbors == 3 {
                continue;
            }
            assert_eq!(cell.next_state(neighbors), Cell::Dead);
        }
    }
}
