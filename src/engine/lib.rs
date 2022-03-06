extern crate console_error_panic_hook;
extern crate nom;

#[macro_use]
extern crate lazy_static;

use std::cell::Cell;
use std::collections::HashMap;
use std::fmt;
use wasm_bindgen::prelude::*;

mod dep_graph;
mod error;
mod interpreter;
mod parser;
mod sheet;

use sheet::{Sheet, SheetAddress};

#[wasm_bindgen]
pub struct JsSheet(Sheet);

#[wasm_bindgen]
impl JsSheet {
    pub fn get_cell(&mut self, row: i32, col: i32) -> String {
        self.0.get_cell(SheetAddress { row, col }).to_string()
    }

    pub fn set_cell(&mut self, row: i32, col: i32, contents: &str) -> Result<(), JsValue> {
        self.0
            .set_cell(SheetAddress { row, col }, contents.to_string())
            .map_err(|err| JsValue::from_str(format!("{}", err).as_str()))
    }

    pub fn debug_parse_expr(&mut self, input: &str) -> String {
        (match parser::parse(input) {
            Ok(expr) => format!("{:#?}", expr),
            Err(msg) => format!("ERROR: {}", msg),
        })
        .into()
    }

    pub fn debug_eval_expr(&mut self, input: &str) -> Result<String, JsValue> {
        || -> error::AppResult<String> {
            let expr = parser::parse(input)?;
            let program = interpreter::compile(&expr)?;
            let env = interpreter::Env::with_builtins();
            let res = interpreter::eval(&program, env, &interpreter::KeywordResolver::empty())?;
            Ok(format!("{:?}", res))
        }()
        .map_err(|err| JsValue::from_str(format!("{}", err).as_str()))
    }

    pub fn debug_graphviz(&self) -> String {
        self.0.debug_graphviz()
    }

    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        JsSheet(Sheet::new())
    }
}
