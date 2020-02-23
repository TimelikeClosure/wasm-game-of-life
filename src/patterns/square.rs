use super::pattern::Pattern;
use crate::Cell;

pub struct Square {}

impl Square {
    pub fn new() -> Pattern {
        let mut pattern = Pattern::rect(6, 6);

        pattern.set_cell(2, 2, Cell::Alive);
        pattern.set_cell(3, 2, Cell::Alive);
        pattern.set_cell(2, 3, Cell::Alive);
        pattern.set_cell(3, 3, Cell::Alive);

        pattern
    }
}
