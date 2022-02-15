extern crate console_error_panic_hook;
extern crate nom;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;

mod parser;

#[derive(Eq, PartialEq, Hash)]
struct SheetAddress {
    row: i32,
    col: i32,
}

struct SheetCell {
    value: i32,
}

#[wasm_bindgen]
pub struct Sheet {
    cells: HashMap<SheetAddress, SheetCell>,
}

#[wasm_bindgen]
impl Sheet {
    pub fn get_cell(&mut self, row: i32, col: i32) -> i32 {
        self.cells
            .get(&SheetAddress { row, col })
            .map(|cell| cell.value)
            .unwrap_or(0)
    }

    pub fn set_cell(&mut self, row: i32, col: i32, value: i32) {
        self.cells
            .insert(SheetAddress { row, col }, SheetCell { value });
    }

    pub fn debug_parse_expr(&mut self, input: &str) -> String {
        (match parser::parse(input) {
            Ok(expr) => format!("{:#?}", expr),
            Err(msg) => format!("ERROR: {}", msg),
        })
        .into()
    }

    pub fn new() -> Self {
        console_error_panic_hook::set_once();

        Self {
            cells: HashMap::new(),
        }
    }
}
