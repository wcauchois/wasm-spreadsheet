use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::{AppError, AppResult};

use super::compiler::Program;
use super::env::Env;
use super::model::{Instruction, Value};

pub trait KeywordResolver {
    fn resolve_keyword(&self, kw: &str) -> AppResult<Value>;
}

pub struct EmptyKeywordResolver;

impl KeywordResolver for EmptyKeywordResolver {
    fn resolve_keyword(&self, _kw: &str) -> AppResult<Value> {
        Ok(Value::Nil)
    }
}

fn eval_instructions<R: KeywordResolver>(
    instructions: &Vec<Instruction>,
    env: Rc<RefCell<Env>>,
    kw_resolver: &R,
) -> AppResult<Value> {
    let mut pc: usize = 0;
    let mut stack: Vec<Value> = Vec::new();
    while pc < instructions.len() {
        let instruction = &instructions[pc];
        pc += 1;
        match instruction {
            Instruction::LoadConst(value) => {
                stack.push(*value.clone());
            }
            Instruction::StoreName(name) => {
                let value = stack.pop().unwrap();
                env.borrow_mut().define(name, value);
            }
            Instruction::LoadName(name) => {
                stack.push(env.borrow().lookup(name)?);
            }
            Instruction::LoadKeyword(kw) => {
                stack.push(kw_resolver.resolve_keyword(&kw)?);
            }
            Instruction::DiscardValue => {
                stack.pop().unwrap();
            }
            Instruction::CallFunction { .. } | Instruction::ApplyFunction => {
                let args = match instruction {
                    Instruction::CallFunction { nargs } => {
                        let mut args = Vec::with_capacity(*nargs as usize);
                        for _ in 0..*nargs {
                            args.push(stack.pop().unwrap());
                        }
                        args.reverse();
                        args
                    }
                    Instruction::ApplyFunction => {
                        let arg_list = stack.pop().unwrap();
                        match arg_list {
                            Value::List(args) => args,
                            _ => {
                                return Err(AppError::new(
                                    "The second argument to `apply` must be a list",
                                ))
                            }
                        }
                    }
                    _ => unreachable!(),
                };
                let func = stack.pop().unwrap();
                match func {
                    Value::BuiltinFunction(builtin_func) => {
                        stack.push(builtin_func.call(args)?);
                    }
                    Value::UserFunction {
                        params,
                        body,
                        env: function_env,
                    } => {
                        let child_env = Rc::new(RefCell::new(Env::child(function_env)));
                        for (param, arg) in params.iter().zip(args) {
                            child_env.borrow_mut().define(param, arg);
                        }
                        stack.push(eval_instructions(&body, child_env, kw_resolver)?);
                    }
                    _ => {
                        return Err(AppError::new(format!(
                            "Expression {:?} is not callable!",
                            func
                        )))
                    }
                }
            }
            Instruction::MakeFunction => {
                if let (Value::CompiledCode(func_instructions), Value::List(param_syms)) =
                    (stack.pop().unwrap(), stack.pop().unwrap())
                {
                    stack.push(Value::UserFunction {
                        params: param_syms
                            .iter()
                            .map(|sym| match sym {
                                Value::Symbol(s) => s.clone(),
                                _ => panic!(),
                            })
                            .collect(),
                        body: func_instructions,
                        env: env.clone(),
                    });
                } else {
                    panic!("Unexpected stack for MakeFunction");
                }
            }
            Instruction::RelativeJump { offset } => {
                pc += *offset as usize;
            }
            Instruction::RelativeJumpIfTrue { offset } => {
                let cond = stack.pop().unwrap();
                let should_jump = match cond {
                    Value::Boolean(true) => true,
                    _ => false,
                };
                if should_jump {
                    pc += *offset as usize;
                }
            }
        }
    }
    Ok(stack.pop().unwrap())
}

pub fn eval<R: KeywordResolver>(program: &Program, env: Env, kw_resolver: &R) -> AppResult<Value> {
    let env_rc = Rc::new(RefCell::new(env));
    eval_instructions(&program.instructions, env_rc, kw_resolver)
}

#[cfg(test)]
mod tests {
    use super::{super::compiler::compile, *};
    use crate::parser::Expr;

    #[test]
    fn test_full_eval() {
        let env = Env::with_builtins();
        let program = compile(&Expr::from_string("(+ 1 2)").unwrap()).unwrap();
        let res = eval(&program, env, &EmptyKeywordResolver).unwrap();
        assert_eq!(res, Value::Number(3.0));
    }

    #[test]
    fn test_begin() {
        let env = Env::with_builtins();
        let program = compile(&Expr::from_string("(begin 1 2)").unwrap()).unwrap();
        let res = eval(&program, env, &EmptyKeywordResolver).unwrap();
        assert_eq!(res, Value::Number(2.0));
    }
}
