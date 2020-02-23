use super::pattern::Pattern;
use crate::Cell;

pub struct Oval {}

impl Oval {
    pub fn new() -> Pattern {
        let mut pattern = Pattern::rect(8, 7);

        pattern.set_cell(3, 2, Cell::Alive);
        pattern.set_cell(4, 2, Cell::Alive);
        pattern.set_cell(2, 3, Cell::Alive);
        pattern.set_cell(5, 3, Cell::Alive);
        pattern.set_cell(3, 4, Cell::Alive);
        pattern.set_cell(4, 4, Cell::Alive);

        pattern
    }
}
