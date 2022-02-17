use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::{AppError, AppResult};

use super::builtins::BUILTINS_ENVIRONMENT;
use super::model::Value;

#[derive(Debug, PartialEq)]
pub struct Env {
    /// Variable assignments within the env
    pub(super) table: HashMap<String, Value>,
    pub(super) parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    /// Map a name to a value in the current env
    pub(super) fn define(&mut self, name: &str, value: Value) {
        self.table.insert(name.to_owned(), value);
    }

    pub fn lookup(&self, name: &str) -> AppResult<Value> {
        if self.table.contains_key(name) {
            return Ok(self.table.get(name).unwrap().clone());
        } else {
            let mut current_parent = self.parent.clone();
            while let Some(parent) = current_parent {
                if let Some(value) = parent.borrow().table.get(name) {
                    return Ok(value.clone());
                }
                current_parent = parent.borrow().parent.clone();
            }
        }
        Err(AppError::new(format!("Variable is not defined: {}", name)))
    }

    pub(super) fn child(parent: Rc<RefCell<Env>>) -> Self {
        Self {
            table: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn with_builtins() -> Self {
        BUILTINS_ENVIRONMENT.with(|builtins_environment| Self {
            table: HashMap::new(),
            parent: Some(builtins_environment.clone()),
        })
    }
}
