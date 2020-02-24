mod patterns;
mod utils;

use js_sys::Math::random;
use patterns::f_pent::FPentomino;
use patterns::glider::Glider;
use patterns::space_ship::SpaceShip;
use patterns::Pattern;
use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    next: Vec<Cell>,
    direction_deltas: Box<[(u32, u32)]>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        ((row % self.height) * self.width + (column % self.width)) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for (delta_column, delta_row) in self.direction_deltas.iter() {
            let neighbor_row = (row + delta_row) % self.height;
            let neighbor_column = (column + delta_column) % self.width;
            let index = self.get_index(neighbor_row, neighbor_column);
            count += self.cells[index] as u8;
        }

        count
    }

    fn generate_pattern(&mut self, pattern: Pattern, x_base: u32, y_base: u32) {
        for pattern_cell in pattern.cells {
            let x = x_base + pattern_cell.x;
            let y = y_base + pattern_cell.y;
            let index = self.get_index(y, x);
            self.cells[index] = pattern_cell.cell;
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = match cell {
                    Cell::Alive => '◼',
                    Cell::Dead => '◻',
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..height * width)
            // .map(|index| {
            //     if index % 2 == 0 || index % 7 == 0 {
            //         Cell::Alive
            //     } else {
            //         Cell::Dead
            //     }
            // })
            .map(|_| {
                if (random() * 2.) as u8 != 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            // .map(|_| Cell::Dead)
            .collect::<Vec<Cell>>();

        let next = (0..height * width)
            .map(|_| Cell::Dead)
            .collect::<Vec<Cell>>();

        let up = height - 1;
        let left = width - 1;
        let direction_deltas = vec![
            (1, 0),
            (1, 1),
            (0, 1),
            (left, 1),
            (left, 0),
            (left, up),
            (0, up),
            (1, up),
        ]
        .into_boxed_slice();

        let mut universe = Universe {
            width,
            height,
            cells,
            next,
            direction_deltas,
        };

        for (x, y) in [
            (0, 0),
            (24, 8),
            (48, 16),
            (8, 24),
            (32, 32),
            (56, 40),
            (16, 48),
            (40, 56),
        ]
        .iter()
        {
            universe.generate_pattern(SpaceShip::new(), *x, *y)
        }

        for (x, y) in [
            (40, 0),
            (0, 8),
            (24, 16),
            (48, 24),
            (8, 32),
            (32, 40),
            (56, 48),
            (16, 56),
        ]
        .iter()
        {
            universe.generate_pattern(Glider::new(), *x, *y)
        }

        universe
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.get_index(row, column);
                let cell = self.cells[index];
                let neighbors = self.live_neighbor_count(row, column);

                let next_cell = match (cell, neighbors) {
                    (Cell::Dead, 3) => Cell::Alive,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    _ => Cell::Dead,
                };

                self.next[index] = next_cell;
            }
        }

        std::mem::swap(&mut self.cells, &mut self.next);
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn prev(&self) -> *const Cell {
        self.next.as_ptr()
    }
}
