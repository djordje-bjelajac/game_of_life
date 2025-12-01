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
