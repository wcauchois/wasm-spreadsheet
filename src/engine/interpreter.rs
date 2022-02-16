use crate::parser::Expr;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;
use std::thread_local;

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
    /// Map a name to a value in the current env
    fn define(&mut self, name: &str, value: Value) {
        self.table.insert(name.to_owned(), value);
    }

    fn lookup<'a>(&'a self, name: &str) -> AppResult<&'a Value> {
        if let Some(env) = self.resolve(name) {
            Ok(env.table.get(name).unwrap())
        } else {
            Err(AppError::new(format!("Variable is not defined: {}", name)))
        }
    }

    /// Find the env in which the name is bound
    fn resolve<'a>(&'a self, name: &str) -> Option<&'a Env> {
        if self.table.contains_key(name) {
            Some(self)
        } else if let Some(parent) = &self.parent {
            parent.resolve(name)
        } else {
            None
        }
    }

    fn with_builtins() -> Self {
        BUILTINS_ENVIRONMENT.with(|builtins_environment| Self {
            table: HashMap::new(),
            parent: Some(builtins_environment.clone()),
        })
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
    UserFunction {
        /// List of names
        params: Vec<String>,
        body: Vec<Instruction>,
        env: Env,
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

trait BuiltinFunction: Sync {
    fn name(&self) -> String;
    fn call(&self, args: Vec<Value>) -> AppResult<Value>;
}

impl fmt::Debug for dyn BuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<builtin {}>", self.name())
    }
}

impl std::cmp::PartialEq for dyn BuiltinFunction {
    fn eq(&self, other: &dyn BuiltinFunction) -> bool {
        std::ptr::eq(self, other)
    }
}

macro_rules! define_builtin_function {
    ($struct_name:ident, $string_name:expr, $args:ident => $body:expr) => {
        struct $struct_name;
        impl BuiltinFunction for $struct_name {
            fn name(&self) -> String {
                String::from($string_name)
            }

            fn call(&self, $args: Vec<Value>) -> AppResult<Value> {
                $body
            }
        }
    };
}

define_builtin_function!(Plus, "+", args => {
    let mut accum: f32 = 0.0;
    for arg in args {
        match arg {
            Value::Number(n) => {
                accum += n;
            }
            _ => return Err(AppError::new("Bad arguments for `+`"))
        }
    }
    Ok(Value::Number(accum))
});

lazy_static! {
    static ref BUILTIN_FUNCTIONS: Vec<&'static dyn BuiltinFunction> = vec![&Plus];
    static ref BUILTIN_FUNCTIONS_BY_NAME: HashMap<String, &'static dyn BuiltinFunction> =
        BUILTIN_FUNCTIONS
            .iter()
            .map(|builtin| (builtin.name(), builtin.clone()))
            .collect();
}

thread_local! {
    pub static BUILTINS_ENVIRONMENT: Rc<Env> = Rc::new(Env {
        table: BUILTIN_FUNCTIONS_BY_NAME
            .iter()
            .map(|(name, func)| (name.clone(), Value::BuiltinFunction(func.clone())))
            .collect(),
        parent: None,
    });
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

pub fn compile(expr: &Expr) -> AppResult<Program> {
    Ok(Program {
        instructions: compile_to_instruction_vec(&expr)?,
    })
}

pub fn evaluate(program: &Program, env: &mut Env) -> AppResult<Value> {
    let mut pc: usize = 0;
    let instructions = &program.instructions;
    while pc < instructions.len() {
        let instruction = &instructions[pc];
        pc += 1;
        match instruction {
            _ => panic!(),
        }
    }
    Ok(Value::Nil)
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

    #[test]
    fn test_plus_builtin() {
        assert_eq!(
            Plus.call(vec![Value::Number(3.0), Value::Number(4.0)])
                .unwrap(),
            Value::Number(7.0)
        );
        assert_eq!(Plus.name(), "+");
    }

    #[test]
    fn test_env_with_builtins() {
        let env = Env::with_builtins();
        assert_eq!(env.lookup("+").unwrap(), &Value::BuiltinFunction(&Plus));
    }
}
