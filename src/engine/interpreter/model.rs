use std::cell::RefCell;
use std::rc::Rc;

use crate::parser::Expr;

use super::builtins::BuiltinFunction;
use super::env::Env;

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    /// Push a constant onto the stack
    LoadConst(Box<Value>),
    /// Store a value into the environment
    StoreName(String),
    /// Read a value from the environment
    LoadName(String),
    /// Call a function (built-in or user-defined)
    CallFunction { nargs: i32 },
    /// Jumps if the value on top of the stack is true
    RelativeJumpIfTrue { offset: i32 },
    /// Jumps
    RelativeJump { offset: i32 },
    /// Creates a function object from a code object on the stack and pushes it on the stack
    MakeFunction,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f32),
    String(String),
    Symbol(String),
    // XXX: Keywords should be resolved to a cell value
    Keyword(String),
    List(Vec<Value>),
    CompiledCode(Vec<Instruction>),
    UserFunction {
        /// List of names
        params: Vec<String>,
        body: Vec<Instruction>,
        env: Rc<RefCell<Env>>,
    },
    BuiltinFunction(&'static dyn BuiltinFunction),
    Nil,
}

impl Value {
    pub fn from_expr(expr: &Expr) -> Self {
        match expr {
            Expr::Number(n) => Value::Number(n.clone()),
            Expr::String(s) => Value::String(s.clone()),
            Expr::Symbol(sym) => Value::Symbol(sym.clone()),
            Expr::Keyword(kw) => Value::Symbol(kw.clone()),
            Expr::List(exprs) => Value::List(exprs.iter().map(|e| Value::from_expr(e)).collect()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_from_expr() {
        assert_eq!(Value::from_expr(&Expr::Number(2.0)), Value::Number(2.0));
    }
}
