use std::collections::HashMap;
use std::fmt;

use crate::dep_graph;
use crate::dep_graph::DepGraph;
use crate::error::{AppError, AppResult};
use crate::interpreter;
use crate::parser::{interpret_cell, Expr, ExprVisitor, InterpretCellResult};

use super::sheet_range::{SheetRange, SheetRangeShapedAddresses};
use super::SheetAddress;

pub struct SheetFormula {
    address: SheetAddress,
    program: interpreter::Program,
    references: Vec<SheetAddress>,
    source: String,
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
            interpreter::Value::String(s) => SheetCellComputedValue::Text(s),
            _ => SheetCellComputedValue::Invalid {
                message: format!("Expression is not representable in a cell: {:?}", ivalue),
            },
        }
    }

    pub fn to_interpreter_value(&self) -> interpreter::Value {
        match self {
            SheetCellComputedValue::Number(n) => interpreter::Value::Number(*n),
            SheetCellComputedValue::Text(s) => interpreter::Value::String(s.into()),
            SheetCellComputedValue::Invalid { .. } => interpreter::Value::Nil,
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

impl SheetCell {
    pub fn to_interpreter_value(&self) -> interpreter::Value {
        self.computed_value.to_interpreter_value()
    }
}

pub struct Sheet {
    cells: HashMap<SheetAddress, SheetCell>,
    dep_graph: DepGraph<SheetAddress>,
}

impl Sheet {
    fn resolve_address(&self, address: &SheetAddress) -> interpreter::Value {
        self.cells
            .get(address)
            .map(|cell| cell.to_interpreter_value())
            .unwrap_or(interpreter::Value::Nil)
    }
}

impl interpreter::KeywordResolver for Sheet {
    fn resolve_keyword(&self, kw: &str) -> AppResult<interpreter::Value> {
        let range = SheetRange::parse(kw)?;

        Ok(match range.addresses_shaped() {
            SheetRangeShapedAddresses::Single { address } => self.resolve_address(&address),
            SheetRangeShapedAddresses::Addresses1D(iter) => interpreter::Value::List(
                iter.map(|address| self.resolve_address(&address))
                    .collect::<Vec<_>>(),
            ),
            SheetRangeShapedAddresses::Addresses2D(iter) => interpreter::Value::List(
                iter.map(|row| {
                    interpreter::Value::List(
                        row.map(|address| self.resolve_address(&address))
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
            ),
        })
    }
}

struct ExprReferencesVisitor {
    references: Vec<SheetAddress>,
    errors: Vec<AppError>,
}

impl ExprVisitor for ExprReferencesVisitor {
    fn visit_keyword(&mut self, kw: &String) {
        match SheetRange::parse(kw) {
            Err(error) => self.errors.push(error),
            Ok(range) => self.references.extend(range.addresses_flat()),
        }
    }
}

fn get_references_for_expr(expr: &Expr) -> AppResult<Vec<SheetAddress>> {
    let mut visitor = ExprReferencesVisitor {
        references: Vec::new(),
        errors: Vec::new(),
    };
    expr.walk(&mut visitor);
    if visitor.errors.len() > 0 {
        // TODO: Include actual error messages
        Err(AppError::new("One or more ranges failed to parse"))
    } else {
        Ok(visitor.references)
    }
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
                let result = interpreter::eval(&program, env, &*self)?;
                let references = get_references_for_expr(&expr)?;
                SheetCell {
                    computed_value: SheetCellComputedValue::from_interpreter_value(result),
                    formula: Some(SheetFormula {
                        address: address.clone(),
                        program,
                        references,
                        source: contents.clone(),
                    }),
                }
            }
        };
        self.dep_graph = match &new_cell.formula {
            Some(formula) => self.dep_graph.update_node(formula),
            None => self.dep_graph.clear_id(&address),
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

    pub fn debug_graphviz(&self) -> String {
        self.dep_graph.to_graphviz()
    }

    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            dep_graph: DepGraph::empty(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_references_for_expr() {
        let result = get_references_for_expr(&Expr::List(vec![
            Expr::Keyword("a1".to_string()),
            Expr::Keyword("b4".to_string()),
        ]));
        assert_eq!(
            result,
            Ok(vec![
                SheetAddress { col: 0, row: 0 },
                SheetAddress { col: 1, row: 3 }
            ])
        );

        let result = get_references_for_expr(&Expr::Keyword("a1".to_string()));
        assert_eq!(result, Ok(vec![SheetAddress { col: 0, row: 0 },]));
    }
}
