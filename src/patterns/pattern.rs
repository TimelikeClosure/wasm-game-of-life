use crate::Cell;

pub struct PatternCell {
    pub x: u32,
    pub y: u32,
    pub cell: Cell,
}

impl PatternCell {
    pub fn new(x: u32, y: u32, cell: Cell) -> PatternCell {
        PatternCell { x, y, cell }
    }
}

pub struct Pattern {
    pub cells: Vec<PatternCell>,
}

impl Pattern {
    pub fn new() -> Pattern {
        Pattern { cells: Vec::new() }
    }

    pub fn rect(width: u32, height: u32) -> Pattern {
        let mut pattern = Pattern::new();

        for row in 0..height {
            for column in 0..width {
                pattern
                    .cells
                    .push(PatternCell::new(column, row, Cell::Dead));
            }
        }

        pattern
    }

    pub fn size(&self) -> (u32, u32) {
        let mut height = 0;
        let mut width = 0;
        for cell in &self.cells {
            if cell.x > width {
                width = cell.x;
            }
            if cell.y > height {
                height = cell.y;
            }
        }
        (width, height)
    }

    pub fn set_cell(&mut self, x: u32, y: u32, cell: Cell) {
        for pattern_cell in &mut self.cells {
            if pattern_cell.x == x && pattern_cell.y == y {
                pattern_cell.cell = cell;
                return;
            }
        }

        self.cells.push(PatternCell::new(x, y, cell));
    }

    pub fn remove_cell(&mut self, x: u32, y: u32) {
        for index in 0..self.cells.len() {
            let cell = &self.cells[index];
            if cell.x == x && cell.y == y {
                self.cells.remove(index);
                return;
            }
        }
    }
}
