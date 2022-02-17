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
        Expr::Number(_) | Expr::String(_) | Expr::Keyword(_) => {
            instructions.push(Instruction::LoadConst(Box::new(Value::from_expr(&expr))));
        }
        Expr::Symbol(sym) => {
            instructions.push(Instruction::LoadName(sym.clone()));
        }
        Expr::List(elems) => match elems.as_slice() {
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
