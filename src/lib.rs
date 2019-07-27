mod utils;

use std::fmt;
use std::fmt::Formatter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}


macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

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

        let x_deltas = [self.width - 1, 0, 1];
        let y_deltas = [self.height - 1, 0, 1];

        for dx in x_deltas.iter() {
            for dy in y_deltas.iter() {
                if *dx == 0 && *dy == 0 {
                    continue;
                }

                let x = (row + *dx) % self.width;
                let y = (column + *dy) % self.height;
                let index = self.get_index(x , y);

                let edge = *dx > 1 && row == 0 || *dy > 1 && column == 0 // -1 delta && getting 0
                                || *dx == 1 && x < row || *dy == 1 && y < column; // +1 delta && got 0

//                console_log!("{} ({}+{}, {}+{}) => ({})", edge, x, *dx, y, *dy, index);

                let cell_status = match self.edge_mode {
                    UniverseEdgeMode::FixedAlive => {
                        if edge {
                            Cell::Alive
                        } else {
                            self.cells[index]
                        }
                    },
                    UniverseEdgeMode::FixedDead => {
                        if edge {
                            Cell::Dead
                        } else {
                            self.cells[index]
                        }
                    },
                    _ => {
                        self.cells[index]
                    }
                };

                count += cell_status as u8;
            }
        }

        count
    }

    pub fn new() -> Universe {
        let width: u32 = 128;
        let height: u32 = 128;

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
            edge_mode: UniverseEdgeMode::Wrap,
        }
    }


    pub fn tick(&mut self) {

        let mut next_cells = self.cells.clone();

        match self.edge_mode {
            UniverseEdgeMode::Expand => {
                unimplemented!();
                // if there are 3 or more alive along an edge, we need to expand one cell in that direction.
                // if there are none alive along an edge, we can shrink that edge.
            },
            _ => {}
        }

        // for each cell
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

                let me_symbol = match me {
                    Cell::Dead => "dead",
                    Cell::Alive => "alive"
                };
                let next_symbol = match next {
                    Cell::Dead => "dead",
                    Cell::Alive => "alive"
                };

//                console_log!("({}, {}) -> ({}) {} => {}", x, y, live_neighbors, me_symbol, next_symbol);

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
