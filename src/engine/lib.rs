extern crate console_error_panic_hook;
extern crate nom;

#[macro_use]
extern crate lazy_static;

use std::cell::Cell;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

mod dep_graph;
mod error;
mod interpreter;
mod parser;

#[derive(Eq, PartialEq, Hash, Clone)]
struct SheetAddress {
    row: i32,
    col: i32,
}

struct SheetFormula {
    address: SheetAddress,
    program: interpreter::Program,
    references: Vec<SheetAddress>,
}

impl dep_graph::Node<SheetAddress> for SheetFormula {
    fn get_id(&self) -> SheetAddress {
        self.address.clone()
    }

    fn get_deps<'a>(&'a self) -> &'a Vec<SheetAddress> {
        &self.references
    }
}

// on cell change: get dependents, recompute the computed value..

enum SheetCellComputedValue {
    Number(f32),
    Text(String),
    Invalid { message: String },
}

impl SheetCellComputedValue {
    fn from_interpreted_value(ivalue: interpreter::Value) -> Self {
        match ivalue {
            interpreter::Value::Number(n) => SheetCellComputedValue::Number(n),
            interpreter::Value::String(n) => SheetCellComputedValue::Text(n),
            _ => SheetCellComputedValue::Invalid {
                message: format!("Expression is not representable in a cell: {:?}", ivalue),
            },
        }
    }
}

struct SheetCellValue {
    computed_value: Cell<SheetCellComputedValue>,
    formula: Option<SheetFormula>,
}

struct SheetCell {
    value: SheetCellValue,
}

#[wasm_bindgen]
pub struct Sheet {
    cells: HashMap<SheetAddress, SheetCell>,
}

#[wasm_bindgen]
impl Sheet {
    pub fn get_cell(&mut self, row: i32, col: i32) -> i32 {
        panic!()
        // self.cells
        //     .get(&SheetAddress { row, col })
        //     .map(|cell| cell.value)
        //     .unwrap_or(0)
    }

    pub fn set_cell(&mut self, row: i32, col: i32, value: i32) {
        panic!();
        // self.cells
        //     .insert(SheetAddress { row, col }, SheetCell { value });
    }

    pub fn debug_parse_expr(&mut self, input: &str) -> String {
        (match parser::parse(input) {
            Ok(expr) => format!("{:#?}", expr),
            Err(msg) => format!("ERROR: {}", msg),
        })
        .into()
    }

    pub fn debug_eval_expr(&mut self, input: &str) -> Result<String, JsValue> {
        || -> error::AppResult<String> {
            let expr = parser::parse(input)?;
            let program = interpreter::compile(&expr)?;
            let env = interpreter::Env::with_builtins();
            let res = interpreter::eval(&program, env)?;
            Ok(format!("{:?}", res))
        }()
        .map_err(|err| JsValue::from_str(format!("{}", err).as_str()))
    }

    pub fn new() -> Self {
        console_error_panic_hook::set_once();

        Self {
            cells: HashMap::new(),
        }
    }
}
