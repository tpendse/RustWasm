mod utils;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead  = 0,
    Alive = 1
}

#[wasm_bindgen]
pub struct Universe {
    width  : u32,
    height : u32,
    cells  : Vec<Cell>
}

// NOTE Not really random right now -- `rand` crate wasn't being downloaded
fn random_cells(width:u32, height:u32) -> Vec<Cell> {
    let cells = (0..width * height).map(|i| {
        if i % 2 == 0 || i % 7 == 0 {
            Cell::Alive
        } else {
            Cell::Dead
        }
    }).collect();

    cells
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width  = 64;
        let height = 64;

        utils::set_panic_hook();

        log!("Creating a new WASM Universe ...");

        let cells = random_cells(width, height);

        Universe { width, height, cells }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..self.width * self.height).map(|_i| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * self.height).map(|_i| Cell::Dead).collect();
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn reset(&mut self) {
        let cells = (0..self.width * self.height).map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        }).collect();

        self.cells = cells;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    // The universe wraps around vertically and horizontally
    /*
    * The modulo operation on the height and width will only give remainders outside the universe size
    * Hence when count goes beyond, it effectively wraps to the other side
     */
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row    + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2           => Cell::Dead,  // Rule 1 : underpopulation
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive, // Rule 2 : continuation
                    (Cell::Alive, x) if x > 3           => Cell::Dead,  // Rule 2 : overpopulation
                    (Cell::Dead, 3)                     => Cell::Alive, // Rule 3 : reproduction
                    (otherwise, _)                      => otherwise,   // Default
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '□' } else { '■' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

// Impl for tests -- no wasm bindings
impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}