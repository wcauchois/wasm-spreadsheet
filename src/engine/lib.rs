extern crate console_error_panic_hook;
extern crate nom;

#[macro_use]
extern crate lazy_static;

use std::cell::Cell;
use std::collections::HashMap;
use std::fmt;
use wasm_bindgen::prelude::*;

mod dep_graph;
mod error;
mod interpreter;
mod parser;

use dep_graph::DepGraph;
use error::AppResult;
use interpreter::{eval, Env};
use parser::{interpret_cell, InterpretCellResult};

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct SheetAddress {
    row: i32,
    col: i32,
}

pub struct SheetFormula {
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

#[derive(Clone, Debug)]
pub enum SheetCellComputedValue {
    Number(f32),
    Text(String),
    Invalid { message: String },
}

impl SheetCellComputedValue {
    pub fn from_interpreter_value(ivalue: interpreter::Value) -> Self {
        match ivalue {
            interpreter::Value::Number(n) => SheetCellComputedValue::Number(n),
            interpreter::Value::String(n) => SheetCellComputedValue::Text(n),
            _ => SheetCellComputedValue::Invalid {
                message: format!("Expression is not representable in a cell: {:?}", ivalue),
            },
        }
    }

    pub fn empty() -> Self {
        Self::Text("".to_string())
    }
}

impl fmt::Display for SheetCellComputedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Text(s) => write!(f, "{}", s),
            Self::Invalid { message } => write!(f, "!INVALID: {}", message),
        }
    }
}

struct SheetCell {
    computed_value: SheetCellComputedValue,
    formula: Option<SheetFormula>,
}

pub struct Sheet {
    cells: HashMap<SheetAddress, SheetCell>,
    // dep_graph: Cell<DepGraph<SheetAddress>>,
}

impl Sheet {
    pub fn set_cell(&mut self, address: SheetAddress, contents: String) -> AppResult<()> {
        let interpreted_cell = interpret_cell(&contents)?;
        let new_cell = match interpreted_cell {
            InterpretCellResult::Number(n) => SheetCell {
                computed_value: SheetCellComputedValue::Number(n),
                formula: None,
            },
            InterpretCellResult::Text(s) => SheetCell {
                computed_value: SheetCellComputedValue::Text(s),
                formula: None,
            },
            InterpretCellResult::Expr(expr) => {
                let program = interpreter::compile(&expr)?;
                let env = Env::with_builtins();
                let res = eval(&program, env)?;
                SheetCell {
                    computed_value: SheetCellComputedValue::from_interpreter_value(res),
                    formula: Some(SheetFormula {
                        address: address.clone(),
                        program,
                        references: Vec::new(), // TODO
                    }),
                }
            }
        };
        self.cells.insert(address, new_cell);
        Ok(())
    }

    pub fn get_cell(&self, address: SheetAddress) -> SheetCellComputedValue {
        self.cells
            .get(&address)
            .map(|cell| cell.computed_value.clone())
            .unwrap_or(SheetCellComputedValue::empty())
    }

    pub fn new() -> Self {
        let mut cells = HashMap::new();
        cells.insert(
            SheetAddress { row: 3, col: 3 },
            SheetCell {
                computed_value: SheetCellComputedValue::Text("foo".to_string()),
                formula: None,
            },
        );
        Self { cells } // TEMP
                       // Self {
                       //     cells: HashMap::new(),
                       // }
    }
}

#[wasm_bindgen]
pub struct JsSheet(Sheet);

#[wasm_bindgen]
impl JsSheet {
    pub fn get_cell(&mut self, row: i32, col: i32) -> String {
        self.0.get_cell(SheetAddress { row, col }).to_string()
    }

    pub fn set_cell(&mut self, row: i32, col: i32, contents: &str) -> Result<(), JsValue> {
        self.0
            .set_cell(SheetAddress { row, col }, contents.to_string())
            .map_err(|err| JsValue::from_str(format!("{}", err).as_str()))
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
        JsSheet(Sheet::new())
    }
}
