use signals2::*;
use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};
use std::fmt;

use crate::console_log::*;
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
            interpreter::Value::Boolean(b) => {
                SheetCellComputedValue::Text((if b { "TRUE" } else { "FALSE" }).into())
            }
            interpreter::Value::Nil => SheetCellComputedValue::Text("<nil>".into()),
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
    source: String,
}

pub struct SheetCellInfo {
    pub value: SheetCellComputedValue,
    pub source: String,
}

impl SheetCell {
    pub fn to_interpreter_value(&self) -> interpreter::Value {
        self.computed_value.to_interpreter_value()
    }
}

pub struct Sheet {
    cells: HashMap<SheetAddress, SheetCell>,
    dep_graph: DepGraph<SheetAddress>,
    // Not stored in SheetCell itself so that clients can subscribe to cells
    // which haven't been created yet.
    signals: HashMap<SheetAddress, Signal<()>>,
}

pub struct CellSubscription {
    address: SheetAddress,
    connection: Connection,
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

const MAX_ITERS: usize = 10_000;

impl Sheet {
    pub fn set_cell(&mut self, address: &SheetAddress, contents: String) -> AppResult<()> {
        let interpreted_cell = interpret_cell(&contents)?;
        let (computed_value, formula) = match interpreted_cell {
            InterpretCellResult::Number(n) => (SheetCellComputedValue::Number(n), None),
            InterpretCellResult::Text(s) => (SheetCellComputedValue::Text(s), None),
            InterpretCellResult::Expr(expr) => {
                let program = interpreter::compile_with_prelude(&expr)?;
                let env = interpreter::Env::with_builtins();
                let result = interpreter::eval(&program, env, &*self)?;
                let references = get_references_for_expr(&expr)?;
                let computed_value = SheetCellComputedValue::Invalid {
                    message: "<pending>".to_string(),
                };
                let formula = SheetFormula {
                    address: address.clone(),
                    program,
                    references,
                };
                (computed_value, Some(formula))
            }
        };

        let new_cell = SheetCell {
            computed_value,
            formula,
            source: contents.clone(),
        };

        self.dep_graph = match &new_cell.formula {
            Some(formula) => self.dep_graph.update_node(formula),
            None => self.dep_graph.clear_id(&address),
        };
        self.cells.insert(address.clone(), new_cell);

        self.emit_cell_update(&address);

        let mut handled_addresses = HashSet::new();
        let mut computation_queue: VecDeque<SheetAddress> = VecDeque::from([address.clone()]);
        let mut num_iters = 0;
        while let Some(address_to_compute) = computation_queue.pop_front() {
            num_iters += 1;
            if num_iters > MAX_ITERS {
                break;
            }

            if handled_addresses.contains(&address_to_compute) {
                continue;
            } else {
                handled_addresses.insert(address_to_compute.clone());
            }

            if let Some(cell) = self.cells.get(&address_to_compute) {
                if let Some(formula) = &cell.formula {
                    let env = interpreter::Env::with_builtins();
                    let result = interpreter::eval(&formula.program, env, &*self)?;
                    {
                        self.cells
                            .get_mut(&address_to_compute)
                            .unwrap()
                            .computed_value =
                            SheetCellComputedValue::from_interpreter_value(result);
                    }
                    self.emit_cell_update(&address_to_compute);
                }
            }

            for dependent in self
                .dep_graph
                .get_direct_dependents(address_to_compute.clone())
            {
                computation_queue.push_back(dependent);
            }
        }

        Ok(())
    }

    fn emit_cell_update(&self, address: &SheetAddress) {
        if let Some(signal) = self.signals.get(address) {
            signal.emit();
        }
    }

    pub fn get_cell(&self, address: &SheetAddress) -> SheetCellInfo {
        match self.cells.get(&address) {
            Some(cell) => SheetCellInfo {
                value: cell.computed_value.clone(),
                source: cell.source.clone(),
            },
            None => SheetCellInfo {
                value: SheetCellComputedValue::empty(),
                source: "".to_string(),
            },
        }
    }

    pub fn subscribe_to_cell<F: Fn() -> () + Send + Sync + 'static>(
        &mut self,
        address: SheetAddress,
        f: F,
    ) -> CellSubscription {
        let signal = self
            .signals
            .entry(address.clone())
            .or_insert_with(|| Signal::new());
        CellSubscription {
            address: address.clone(),
            connection: signal.connect(f),
        }
    }

    pub fn unsubscribe(&mut self, subscription: &CellSubscription) {
        subscription.connection.disconnect();
        if let Some(signal) = self.signals.get(&subscription.address) {
            if signal.count() == 0 {
                self.signals.remove(&subscription.address);
            }
        }
    }

    pub fn debug_graphviz(&self) -> String {
        self.dep_graph.to_graphviz()
    }

    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            dep_graph: DepGraph::empty(),
            signals: HashMap::new(),
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
