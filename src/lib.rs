mod utils;

use std::fmt;
use std::fmt::Formatter;
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
    Alive = 1
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    edge_mode: UniverseEdgeMode,
}

#[wasm_bindgen]
pub enum UniverseEdgeMode {
    FixedAlive,
    FixedDead,
    Wrap, // little stretch
    Expand, // big stretch
}


#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.height + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for x in row-1..row+1 {
            for y in column-1..column+1 {
                if x == 0 && y == 0 {
                    // that's me
                    continue;
                } else if x < 0 || y < 0 || x >= self.width || y >=self.height  {
                    // an edge
                    count += match self.edge_mode {
                        UniverseEdgeMode::FixedAlive => Cell::Alive as u8,
                        UniverseEdgeMode::FixedDead => Cell::Dead as u8,
                        _ => { unimplemented!() }
                    };

                } else {
                    // a real neighbor
                    let index = self.get_index(x, y);
                    count += self.cells[index] as u8;
                }
            }
        }

        count
    }

    pub fn new() -> Universe {
        let width: u32 = 64;
        let height: u32 = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
            edge_mode: UniverseEdgeMode::FixedAlive,
        }
    }


    pub fn tick(&mut self) {

        let mut next_cells = self.cells.clone();

        // for each cell,
        for x in 0..self.width {
            for y in 0..self.height {
                let index = self.get_index(x, y);
                let me = self.cells[index];
                let live_neighbors = self.live_neighbor_count(x, y);

                // Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
                // Any live cell with two or three live neighbours lives on to the next generation.
                // Any live cell with more than three live neighbours dies, as if by overpopulation.
                // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

                let next = match live_neighbors {
                    0...1 => Cell::Dead,
                    2 => me,
                    3 => Cell::Alive,
                    _ => Cell::Dead
                };

                next_cells[index] = next;
            }
        }

        self.cells = next_cells;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = match cell {
                    Cell::Dead => '◻',
                    Cell::Alive => '◼'
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
