use super::{Cell, Grid};

#[test]
fn new_initializes_dead_cells() {
    let width = 4;
    let height = 3;
    let grid = Grid::new(width, height);

    assert_eq!(grid.width(), width);
    assert_eq!(grid.height(), height);

    for y in 0..height {
        for x in 0..width {
            assert_eq!(grid.get(x, y), Cell::Dead);
        }
    }
}

#[test]
fn get_returns_dead_out_of_bounds() {
    let grid = Grid::new(2, 2);
    assert_eq!(grid.get(5, 5), Cell::Dead);
    assert_eq!(grid.get(0, 5), Cell::Dead);
    assert_eq!(grid.get(5, 0), Cell::Dead);
}

#[test]
fn set_and_get_persist_value() {
    let mut grid = Grid::new(3, 3);
    grid.set(1, 1, Cell::Alive);
    assert_eq!(grid.get(1, 1), Cell::Alive);

    grid.set(2, 0, Cell::Alive);
    assert_eq!(grid.get(2, 0), Cell::Alive);
}

#[test]
fn clear_resets_cells_to_dead() {
    let mut grid = Grid::new(3, 3);
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if (x + y) % 2 == 0 {
                grid.set(x, y, Cell::Alive);
            }
        }
    }

    grid.clear();

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            assert_eq!(grid.get(x, y), Cell::Dead);
        }
    }
}
