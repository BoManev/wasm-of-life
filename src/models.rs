extern crate fixedbitset;
extern crate js_sys;

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        let width = 128;
        let height = 128;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for x in 0..size {
            cells.set(x, js_sys::Math::random() < 0.5)
        }

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    pub fn live_neighbot_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        // height - 1 == - 1 (row above)
        for drow in [self.height - 1, 0, 1].iter().cloned() {
            for dcol in [self.width - 1, 0, 1].iter().cloned() {
                if drow == 0 && dcol == 0 {
                    continue;
                }

                let nrow = (row + drow) % self.height;
                let ncol = (col + dcol) % self.width;
                let idx = self.get_index(nrow, ncol);
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
                let cell: Cell = self.cells[idx].into();
                let live_count = self.live_neighbot_count(row, col);

                next.set(
                    idx,
                    match (cell, live_count) {
                        (Cell::Alive, x) if x < 2 => Cell::Dead.into(),
                        (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive.into(),
                        (Cell::Alive, x) if x > 3 => Cell::Dead.into(),
                        (Cell::Dead, 3) => Cell::Alive.into(),
                        (any, _) => any.into(),
                    },
                )
            }
        }
        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let size = (width * self.height) as usize;
        self.cells = FixedBitSet::with_capacity(size);

        for x in 0..size {
            self.cells.set(x, Cell::Dead.into());
        }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = (self.width * height) as usize;
        self.cells = FixedBitSet::with_capacity(size);

        for x in 0..size {
            self.cells.set(x, Cell::Dead.into());
        }
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}

impl From<Cell> for bool {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::Alive => true,
            Cell::Dead => false,
        }
    }
}

impl From<bool> for Cell {
    fn from(bool: bool) -> Self {
        match bool {
            true => Cell::Alive,
            false => Cell::Dead,
        }
    }
}

impl From<Cell> for u32 {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::Alive => 1,
            Cell::Dead => 0,
        }
    }
}

impl From<u32> for Cell {
    fn from(value: u32) -> Self {
        match value {
            1 => Cell::Alive,
            _ => Cell::Dead,
        }
    }
}

impl Cell {
    pub fn to_bool(&self) -> bool {
        match *self {
            Cell::Dead => false,
            Cell::Alive => true,
        }
    }
}

impl Universe {
    pub fn get_cells(&self) -> Vec<Cell> {
        self.cells.as_slice().iter().map(|c| (*c).into()).collect()
    }

    pub fn set_cells(&mut self, cell: &[(u32, u32)]) {
        cell.iter().for_each(|(row, col)| {
            let idx = self.get_index(*row, *col);
            self.cells.set(idx, Cell::Alive.into());
        });
    }
}
