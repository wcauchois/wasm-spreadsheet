use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn double(i: i32) -> i32 {
    i * 2
}
