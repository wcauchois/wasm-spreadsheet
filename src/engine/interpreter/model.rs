use std::cell::RefCell;
use std::fmt;
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
    LoadKeyword(String),
    /// Call a function (built-in or user-defined)
    CallFunction {
        nargs: i32,
    },
    /// Apply a function. The stack should consist of <function>, <list of arguments>
    ApplyFunction,
    /// Jumps if the value on top of the stack is true
    RelativeJumpIfTrue {
        offset: i32,
    },
    /// Jumps
    RelativeJump {
        offset: i32,
    },
    /// Creates a function object from a code object on the stack and pushes it on the stack
    MakeFunction,
    /// Pop a value from the stack and do nothing with it.
    DiscardValue,
}

#[derive(PartialEq, Clone)]
pub enum Value {
    Number(f32),
    String(String),
    Boolean(bool),
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

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "Number({})", n),
            Value::String(s) => write!(f, "String({:?})", s),
            Value::Boolean(b) => {
                if *b {
                    write!(f, "#t")
                } else {
                    write!(f, "#f")
                }
            }
            Value::Symbol(sym) => write!(f, "Symbol({})", sym),
            Value::Keyword(kw) => write!(f, "Keyword({})", kw),
            Value::List(elems) => write!(f, "List({:?}", elems),
            Value::CompiledCode(code) => write!(f, "<compiled code>"),
            Value::UserFunction { params, .. } => write!(f, "<func: {:?}>", params),
            Value::BuiltinFunction(func) => func.fmt(f),
            Value::Nil => write!(f, "Nil"),
        }
    }
}

impl Value {
    pub fn from_expr(expr: &Expr) -> Self {
        match expr {
            Expr::Number(n) => Value::Number(n.clone()),
            Expr::String(s) => Value::String(s.clone()),
            Expr::Boolean(b) => Value::Boolean(b.clone()),
            Expr::Symbol(sym) => Value::Symbol(sym.clone()),
            Expr::Keyword(kw) => Value::Symbol(kw.clone()),
            Expr::List(exprs) => Value::List(exprs.iter().map(|e| Value::from_expr(e)).collect()),
        }
    }

    pub fn type_string(&self) -> &str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Boolean(_) => "bool",
            Value::Symbol(_) => "symbol",
            Value::Keyword(_) => "keyword",
            Value::List(_) => "list",
            Value::CompiledCode(code) => "code",
            Value::UserFunction { .. } => "function",
            Value::BuiltinFunction(func) => "builtin",
            Value::Nil => "nil",
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
