extern crate console_error_panic_hook;
extern crate nom;

#[macro_use]
extern crate lazy_static;

use std::cell::Cell;
use std::collections::{hash_map::Entry, HashMap, VecDeque};
use std::fmt;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

mod console_log;
mod dep_graph;
mod error;
mod interpreter;
mod parser;
mod sheet;

use interpreter::EmptyKeywordResolver;
use sheet::{CellSubscription, Sheet, SheetAddress, SheetCellInfo};

#[wasm_bindgen]
pub struct JsSheet {
    sheet: Sheet,
    sheet_update_queue: Arc<Mutex<VecDeque<SheetAddress>>>,
    listener_map: HashMap<SheetAddress, (Vec<js_sys::Function>, CellSubscription)>,
}

#[wasm_bindgen]
pub struct JsSheetCellInfo {
    underlying: SheetCellInfo,
}

#[wasm_bindgen]
impl JsSheetCellInfo {
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> String {
        self.underlying.value.to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn source(&self) -> String {
        self.underlying.source.clone()
    }

    fn from(underlying: SheetCellInfo) -> Self {
        Self { underlying }
    }
}

#[wasm_bindgen]
impl JsSheet {
    pub fn get_cell(&mut self, row: i32, col: i32) -> JsSheetCellInfo {
        JsSheetCellInfo::from(self.sheet.get_cell(&SheetAddress { row, col }))
    }

    pub fn set_cell(&mut self, row: i32, col: i32, contents: &str) -> Result<(), JsValue> {
        let result = self
            .sheet
            .set_cell(&SheetAddress { row, col }, contents.to_string())
            .map_err(|err| JsValue::from_str(format!("{}", err).as_str()))?;
        self.flush_update_queue();
        Ok(result)
    }

    pub fn add_listener(&mut self, row: i32, col: i32, func: js_sys::Function) {
        let address = SheetAddress { row, col };

        let (listeners, _) = match self.listener_map.entry(address.clone()) {
            Entry::Vacant(entry) => {
                let my_sheet_update_queue = self.sheet_update_queue.clone();
                let my_address = address.clone();
                let subscription = self.sheet.subscribe_to_cell(address.clone(), move || {
                    my_sheet_update_queue
                        .lock()
                        .unwrap()
                        .push_back(my_address.clone());
                });
                entry.insert((Vec::new(), subscription))
            }
            Entry::Occupied(entry) => entry.into_mut(),
        };

        listeners.push(func);
    }

    pub fn remove_listener(&mut self, row: i32, col: i32, input_func: &js_sys::Function) -> bool {
        let mut was_removed = false;
        let address = SheetAddress { row, col };
        if let Some((listeners, subscription)) = self.listener_map.get_mut(&address) {
            if listeners.iter().any(|func| func == input_func) {
                was_removed = true;
                listeners.retain(|func| func != input_func);
                if listeners.is_empty() {
                    self.sheet.unsubscribe(subscription);
                    self.listener_map.remove(&address);
                }
            }
        };
        was_removed
    }

    fn flush_update_queue(&self) {
        let mut sheet_update_queue = self.sheet_update_queue.lock().unwrap();
        let js_this = JsValue::null();
        while let Some(address) = sheet_update_queue.pop_front() {
            match self.listener_map.get(&address) {
                Some((listeners, _)) => {
                    for func in listeners {
                        let arg: JsValue =
                            JsSheetCellInfo::from(self.sheet.get_cell(&address)).into();
                        // Ignores errors. Should we try to propagate them?
                        let _ = func.call1(&js_this, &arg);
                    }
                }
                None => (),
            }
        }
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
            let res = interpreter::eval(&program, env, &EmptyKeywordResolver)?;
            Ok(format!("{:?}", res))
        }()
        .map_err(|err| JsValue::from_str(format!("{}", err).as_str()))
    }

    pub fn debug_graphviz(&self) -> String {
        self.sheet.debug_graphviz()
    }

    pub fn debug_call_me(&self, func: &js_sys::Function) -> Result<(), JsValue> {
        let this = JsValue::null();
        func.call0(&this)?;
        Ok(())
    }

    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        JsSheet {
            sheet: Sheet::new(),
            sheet_update_queue: Arc::new(Mutex::new(VecDeque::new())),
            listener_map: HashMap::new(),
        }
    }
}
