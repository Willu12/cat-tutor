use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Miau, {name}!")
}

#[wasm_bindgen]
pub fn log() {
    let word = JsValue::from_str(&greet("Borys"));
    log_1(&word);
}
