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
