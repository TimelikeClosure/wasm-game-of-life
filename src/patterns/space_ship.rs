use super::pattern::Pattern;
use crate::Cell;

pub struct SpaceShip {}

impl SpaceShip {
    pub fn new() -> Pattern {
        let mut pattern = Pattern::rect(9, 8);

        pattern.set_cell(2, 2, Cell::Alive);
        pattern.set_cell(5, 2, Cell::Alive);
        pattern.set_cell(6, 3, Cell::Alive);
        pattern.set_cell(2, 4, Cell::Alive);
        pattern.set_cell(6, 4, Cell::Alive);
        pattern.set_cell(3, 5, Cell::Alive);
        pattern.set_cell(4, 5, Cell::Alive);
        pattern.set_cell(5, 5, Cell::Alive);
        pattern.set_cell(6, 5, Cell::Alive);

        pattern
    }
}
