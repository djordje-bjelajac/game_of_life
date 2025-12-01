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

#[path = "rules_test.rs"]
#[cfg(test)]
mod rules_test;
