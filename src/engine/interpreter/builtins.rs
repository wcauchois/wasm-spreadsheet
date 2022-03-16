use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::iter::Extend;
use std::rc::Rc;
use std::thread_local;

use crate::error::{AppError, AppResult};

use super::env::Env;
use super::model::Value;

use crate::parser;

pub trait BuiltinFunction: Sync {
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

define_builtin_function!(Mult, "*", args => {
    let mut accum: f32 = 1.0;
    for arg in args {
        match arg {
            Value::Number(n) => {
                accum *= n;
            }
            _ => return Err(AppError::new("Bad arguments for `*`"))
        }
    }
    Ok(Value::Number(accum))
});

define_builtin_function!(Show, "show", args => {
    let arg = args.first().ok_or(AppError::new("Bad arguments for `show`"))?;
    Ok(Value::String(format!("{:?}", arg)))
});

define_builtin_function!(Type, "type", args => {
    let arg = args.first().ok_or(AppError::new("Bad arguments for `type`"))?;
    Ok(Value::String(arg.type_string().to_string()))
});

define_builtin_function!(Cons, "cons", args => {
    match args.as_slice() {
        [head, tail] => {
            let mut result = vec![head.clone()];
            match tail {
                Value::List(list) => {
                    result.extend(list.iter().cloned());
                }
                Value::Nil => {}
                _ => {
                    return Err(AppError::new(
                        "Bad arguments for `cons`: expected nil or a list for second arg",
                    ));
                }
            }
            Ok(Value::List(result))
        }
        _ => Err(AppError::new(
            "Bad arguments for `cons`: expected 2 arguments",
        )),
    }
});

define_builtin_function!(Car, "car", args => {
    let arg = args.first().ok_or(AppError::new(
        "Bad arguments for `car`: expected 1 argument",
    ))?;
    match arg {
        Value::List(list) => match list.first() {
            Some(head) => Ok(head.clone()),
            None => Err(AppError::new("Cannot take the car of an empty list")),
        },
        Value::Nil => Err(AppError::new("Cannot take the car of an empty list")),
        _ => Err(AppError::new(
            "Bad arguments for `car`: expected a list as the first argument",
        )),
    }
});

define_builtin_function!(Cdr, "cdr", args => {
    let arg = args.first().ok_or(AppError::new(
        "Bad arguments for `cdr`: expected 1 argument",
    ))?;
    match arg {
        Value::List(list) => {
            if list.len() > 0 {
                Ok(Value::List(list[1..].to_vec()))
            } else {
                Err(AppError::new("Cannot take the cdr of an empty list"))
            }
        }
        Value::Nil => Err(AppError::new("Cannot take the cdr of an empty list")),
        _ => Err(AppError::new(
            "Bad arguments for `cdr`: expected a list as the first argument",
        )),
    }
});

define_builtin_function!(NilQ, "nil?", args => {
    let arg = args.first().ok_or(AppError::new("Bad arguments for `nil?`: expected 1 argument"))?;
    Ok(Value::Boolean(match arg {
        Value::Nil => true,
        Value::List(list) if list.len() == 0 => true,
        _ => false,
    }))
});

lazy_static! {
    static ref BUILTIN_FUNCTIONS: Vec<&'static dyn BuiltinFunction> =
        vec![&Plus, &Mult, &Show, &Cons, &Car, &Type, &NilQ, &Cdr];
    static ref BUILTIN_FUNCTIONS_BY_NAME: HashMap<String, &'static dyn BuiltinFunction> =
        BUILTIN_FUNCTIONS
            .iter()
            .map(|builtin| (builtin.name(), builtin.clone()))
            .collect();
}

thread_local! {
    pub static BUILTINS_ENVIRONMENT: Rc<RefCell<Env>> = Rc::new(RefCell::new(Env {
        table: BUILTIN_FUNCTIONS_BY_NAME
            .iter()
            .map(|(name, func)| (name.clone(), Value::BuiltinFunction(func.clone())))
            .chain(std::iter::once(("nil".to_string(), Value::Nil)))
            .collect(),
        parent: None,
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_builtin() {
        assert_eq!(
            Plus.call(vec![Value::Number(3.0), Value::Number(4.0)])
                .unwrap(),
            Value::Number(7.0)
        );
        assert_eq!(Plus.name(), "+");
    }
}

pub const prelude: &str = r#"
(
    (defun map (fun lst)
        (if (nil? lst)
            nil
            (cons (fun (car lst)) (map fun (cdr lst)))))
    (defun filter (fun lst)
        (if (nil? lst)
            nil
            (if (fun (car lst))
                (cons (car lst) (filter fun (cdr lst)))
                (filter fun (cdr lst)))))
)
"#;

thread_local! {
    pub static PARSED_PRELUDE: parser::Expr = parser::parse(prelude).unwrap();
}
