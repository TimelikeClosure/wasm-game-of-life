use super::pattern::Pattern;
use crate::Cell;

pub struct Circle {}

impl Circle {
    pub fn new() -> Pattern {
        let mut pattern = Pattern::rect(8, 8);

        pattern.set_cell(3, 2, Cell::Alive);
        pattern.set_cell(4, 2, Cell::Alive);
        pattern.set_cell(2, 3, Cell::Alive);
        pattern.set_cell(5, 3, Cell::Alive);
        pattern.set_cell(2, 4, Cell::Alive);
        pattern.set_cell(5, 4, Cell::Alive);
        pattern.set_cell(3, 5, Cell::Alive);
        pattern.set_cell(4, 5, Cell::Alive);

        pattern
    }
}
