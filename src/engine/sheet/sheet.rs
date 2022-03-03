use std::collections::HashMap;
use std::fmt;

use crate::dep_graph;
use crate::error::AppResult;
use crate::interpreter;
use crate::parser::{interpret_cell, InterpretCellResult};

use super::SheetAddress;

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
                let env = interpreter::Env::with_builtins();
                let res = interpreter::eval(&program, env)?;
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
        Self {
            cells: HashMap::new(),
        }
    }
}
