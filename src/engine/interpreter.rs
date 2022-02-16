use crate::parser::Expr;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::error::{AppError, AppResult};

#[derive(Debug, PartialEq)]
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
    MakeFunction { nargs: i32 },
}

#[derive(Debug, PartialEq)]
pub struct Env {
    /// Variable assignments within the env
    table: HashMap<String, Value>,
    parent: Option<Rc<Env>>,
}

impl Env {
    fn define(&mut self, name: &str, value: Value) {
        self.table.insert(name.to_owned(), value);
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f32),
    String(String),
    Symbol(String),
    Keyword(String),
    List(Vec<Value>),
    CompiledCode(Vec<Instruction>),
    Function {
        /// List of names
        params: Vec<String>,
        body: Vec<Instruction>,
        env: Env,
    },
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

pub struct Program {
    instructions: Vec<Instruction>,
}

fn compile_to_instruction_vec(expr: &Expr) -> AppResult<Vec<Instruction>> {
    let mut instructions = Vec::new();
    compile_to_instructions(expr, &mut instructions)?;
    Ok(instructions)
}

fn compile_to_instructions(expr: &Expr, instructions: &mut Vec<Instruction>) -> AppResult<()> {
    match expr {
        Expr::Number(_) | Expr::String(_) | Expr::Keyword(_) => {
            instructions.push(Instruction::LoadConst(Box::new(Value::from_expr(&expr))));
        }
        Expr::Symbol(sym) => {
            instructions.push(Instruction::LoadName(sym.clone()));
        }
        Expr::List(elems) => match elems.as_slice() {
            [Expr::Symbol(head_sym), cond, if_true, if_false] if head_sym == "if" => {
                let true_instructions = compile_to_instruction_vec(if_true)?;
                let false_instructions = compile_to_instruction_vec(if_false)?;

                let true_instruction_count = true_instructions.len() as i32;
                let false_instruction_count = false_instructions.len() as i32;

                compile_to_instructions(cond, instructions)?;
                instructions.push(Instruction::RelativeJumpIfTrue {
                    offset: false_instruction_count,
                });
                instructions.extend(false_instructions.into_iter());
                instructions.push(Instruction::RelativeJump {
                    offset: true_instruction_count,
                });
                instructions.extend(true_instructions.into_iter());
            }
            [function_expr, args @ ..] => {
                // Function application
                compile_to_instructions(function_expr, instructions)?;
                for arg in args {
                    compile_to_instructions(arg, instructions)?
                }
                let nargs = (elems.len() - 1) as i32;
                instructions.push(Instruction::CallFunction { nargs });
            }
            [] => return Err(AppError::new("Expected non-empty list")),
            [form_name, ..] => {
                return Err(AppError::new(format!(
                    "Invalid format for form {:?}",
                    form_name
                )))
            }
        },
    }
    Ok(())
}

pub fn evaluate(program: Program) -> AppResult<Value> {
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_from_expr() {
        assert_eq!(Value::from_expr(&Expr::Number(2.0)), Value::Number(2.0));
    }

    #[test]
    fn test_compile_simple() {
        let instructions =
            compile_to_instruction_vec(&Expr::from_string("(+ 1 2)").unwrap()).unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::LoadName("+".to_string()),
                Instruction::LoadConst(Box::new(Value::Number(1.0))),
                Instruction::LoadConst(Box::new(Value::Number(2.0))),
                Instruction::CallFunction { nargs: 2 },
            ]
        );
    }
}
