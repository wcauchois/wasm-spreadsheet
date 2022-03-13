use crate::error::{AppError, AppResult};
use crate::parser::Expr;

use super::model::{Instruction, Value};

pub struct Program {
    pub(super) instructions: Vec<Instruction>,
}

fn compile_to_instruction_vec(expr: &Expr) -> AppResult<Vec<Instruction>> {
    let mut instructions = Vec::new();
    compile_to_instructions(expr, &mut instructions)?;
    Ok(instructions)
}

fn compile_to_instructions(expr: &Expr, instructions: &mut Vec<Instruction>) -> AppResult<()> {
    match expr {
        Expr::Number(_) | Expr::String(_) => {
            instructions.push(Instruction::LoadConst(Box::new(Value::from_expr(&expr))));
        }
        Expr::Keyword(kw) => {
            instructions.push(Instruction::LoadKeyword(kw.clone()));
        }
        Expr::Symbol(sym) => {
            instructions.push(Instruction::LoadName(sym.clone()));
        }
        Expr::List(elems) => match elems.as_slice() {
            [Expr::Symbol(head_sym), name_sym, arg_spec, body] if head_sym == "defun" => {
                // Desugar: (defun foo (x y) body) --> (def foo (fn (x y) body))
                compile_to_instructions(
                    &Expr::List(vec![
                        Expr::Symbol("def".into()),
                        name_sym.clone(),
                        Expr::List(vec![
                            Expr::Symbol("fn".into()),
                            arg_spec.clone(),
                            body.clone(),
                        ]),
                    ]),
                    instructions,
                )?;
            }
            [Expr::Symbol(head_sym), Expr::Symbol(name), body] if head_sym == "def" => {
                compile_to_instructions(body, instructions)?;
                instructions.push(Instruction::StoreName(name.clone()));
                // The result of a def is just nil.
                instructions.push(Instruction::LoadConst(Box::new(Value::Nil)));
            }
            [Expr::Symbol(head_sym), rest @ ..] if head_sym == "begin" => {
                for (idx, sub_expr) in rest.iter().enumerate() {
                    compile_to_instructions(sub_expr, instructions)?;
                    let is_last = idx == rest.len() - 1;
                    if !is_last {
                        instructions.push(Instruction::DiscardValue);
                    }
                }
            }
            [Expr::Symbol(head_sym), value] if head_sym == "quote" => {
                instructions.push(Instruction::LoadConst(Box::new(Value::from_expr(&value))));
            }
            [Expr::Symbol(head_sym), Expr::List(params), body] if head_sym == "lambda" => {
                instructions.push(Instruction::LoadConst(Box::new(Value::List(
                    params
                        .iter()
                        .map(|param| -> AppResult<Value> {
                            match param {
                                Expr::Symbol(sym) => Ok(Value::Symbol(sym.clone())),
                                _ => Err(AppError::new("Expected symbol in parameter list")),
                            }
                        })
                        .collect::<AppResult<Vec<Value>>>()?,
                ))));
                instructions.push(Instruction::LoadConst(Box::new(Value::CompiledCode(
                    compile_to_instruction_vec(body)?,
                ))));
                instructions.push(Instruction::MakeFunction);
            }
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
            [Expr::Symbol(head_sym), function_expr, arg_expr] if head_sym == "apply" => {
                compile_to_instructions(function_expr, instructions)?;
                compile_to_instructions(arg_expr, instructions)?;
                instructions.push(Instruction::ApplyFunction);
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
            [] => return Err(AppError::new("Cannot evaluate an empty list")),
        },
    }
    Ok(())
}

pub fn compile(expr: &Expr) -> AppResult<Program> {
    Ok(Program {
        instructions: compile_to_instruction_vec(&expr)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
