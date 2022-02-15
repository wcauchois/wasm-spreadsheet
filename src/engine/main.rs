extern crate console_error_panic_hook;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Eq, PartialEq, Hash)]
struct SheetCoordinate {
    row: i32,
    col: i32,
}

struct SheetCell {
    value: i32,
}

#[wasm_bindgen]
pub struct Sheet {
    cells: HashMap<SheetCoordinate, SheetCell>,
}

#[wasm_bindgen]
impl Sheet {
    pub fn get_cell(&mut self, row: i32, col: i32) -> i32 {
        self.cells
            .get(&SheetCoordinate { row, col })
            .map(|cell| cell.value)
            .unwrap_or(0)
    }

    pub fn set_cell(&mut self, row: i32, col: i32, value: i32) {
        self.cells
            .insert(SheetCoordinate { row, col }, SheetCell { value })
            .unwrap();
    }

    pub fn new() -> Self {
        console_error_panic_hook::set_once();

        Self {
            cells: HashMap::new(),
        }
    }
}

#[wasm_bindgen]
pub fn double(i: i32) -> i32 {
    i * 2
}
