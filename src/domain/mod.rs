pub mod cell;
pub mod constants;
pub mod grid;
pub mod patterns;
pub mod rules;

pub use cell::Cell;
pub use constants::{MAX_GRID_SIZE, MAX_UPS, MIN_GRID_SIZE, MIN_UPS};
pub use grid::Grid;
pub use patterns::PATTERNS;
pub use rules::next_generation;
