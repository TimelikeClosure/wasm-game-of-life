use super::pattern::Pattern;
use crate::Cell;

pub struct Acorn {}

impl Acorn {
    pub fn new() -> Pattern {
        let mut pattern = Pattern::rect(11, 7);

        pattern.set_cell(2, 2, Cell::Alive);
        pattern.set_cell(3, 2, Cell::Alive);
        pattern.set_cell(4, 2, Cell::Alive);
        pattern.set_cell(7, 2, Cell::Alive);
        pattern.set_cell(8, 2, Cell::Alive);
        pattern.set_cell(5, 3, Cell::Alive);
        pattern.set_cell(7, 4, Cell::Alive);

        pattern
    }
}
