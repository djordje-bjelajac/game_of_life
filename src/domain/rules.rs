use super::grid::Grid;

pub fn count_neighbors(grid: &Grid, x: usize, y: usize) -> u8 {
    let mut count = 0u8;

    for dy in -1i32..=1 {
        for dx in -1i32..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && ny >= 0 {
                let nx = nx as usize;
                let ny = ny as usize;

                if nx < grid.width() && ny < grid.height() && grid.get(nx, ny).is_alive() {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn next_generation(grid: &Grid) -> Grid {
    let mut new_grid = Grid::new(grid.width(), grid.height());

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let neighbors = count_neighbors(grid, x, y);
            let cell = grid.get(x, y);
            let new_cell = cell.next_state(neighbors);
            new_grid.set(x, y, new_cell);
        }
    }

    new_grid
}

#[cfg(test)]
mod tests {
    use super::{count_neighbors, next_generation, Grid};
    use crate::domain::cell::Cell;

    #[test]
    fn count_neighbors_handles_corner_cells() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 1, Cell::Alive);
        grid.set(1, 0, Cell::Alive);
        grid.set(1, 1, Cell::Alive);

        assert_eq!(count_neighbors(&grid, 0, 0), 3);
    }

    #[test]
    fn count_neighbors_handles_edge_cells() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0, Cell::Alive);
        grid.set(2, 0, Cell::Alive);
        grid.set(0, 1, Cell::Alive);
        grid.set(1, 1, Cell::Alive);
        grid.set(2, 1, Cell::Alive);

        assert_eq!(count_neighbors(&grid, 1, 0), 5);
    }

    #[test]
    fn count_neighbors_handles_center_cells() {
        let mut grid = Grid::new(3, 3);
        for y in 0..3 {
            for x in 0..3 {
                if x == 1 && y == 1 {
                    continue;
                }
                grid.set(x, y, Cell::Alive);
            }
        }

        assert_eq!(count_neighbors(&grid, 1, 1), 8);
    }

    #[test]
    fn next_generation_blinker_oscillates() {
        let mut grid = Grid::new(5, 5);
        grid.set(1, 2, Cell::Alive);
        grid.set(2, 2, Cell::Alive);
        grid.set(3, 2, Cell::Alive);

        let next = next_generation(&grid);
        assert_eq!(next.get(2, 1), Cell::Alive);
        assert_eq!(next.get(2, 2), Cell::Alive);
        assert_eq!(next.get(2, 3), Cell::Alive);
        assert_eq!(next.get(1, 2), Cell::Dead);
        assert_eq!(next.get(3, 2), Cell::Dead);
    }

    #[test]
    fn next_generation_block_is_stable() {
        let mut grid = Grid::new(4, 4);
        grid.set(1, 1, Cell::Alive);
        grid.set(1, 2, Cell::Alive);
        grid.set(2, 1, Cell::Alive);
        grid.set(2, 2, Cell::Alive);

        let next = next_generation(&grid);
        for y in 1..=2 {
            for x in 1..=2 {
                assert_eq!(next.get(x, y), Cell::Alive);
            }
        }
    }

    #[test]
    fn next_generation_glider_moves_diagonally() {
        let mut grid = Grid::new(10, 10);
        grid.set(1, 0, Cell::Alive);
        grid.set(2, 1, Cell::Alive);
        grid.set(0, 2, Cell::Alive);
        grid.set(1, 2, Cell::Alive);
        grid.set(2, 2, Cell::Alive);

        let mut current = grid.clone();
        for _ in 0..4 {
            current = next_generation(&current);
        }

        assert_eq!(current.get(2, 1), Cell::Alive);
        assert_eq!(current.get(3, 2), Cell::Alive);
        assert_eq!(current.get(1, 3), Cell::Alive);
        assert_eq!(current.get(2, 3), Cell::Alive);
        assert_eq!(current.get(3, 3), Cell::Alive);
    }
}
