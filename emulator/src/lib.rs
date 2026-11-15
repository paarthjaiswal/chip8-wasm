use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn ping() -> String {
    String::from("CHIP-8 WASM core is connected!")
}

