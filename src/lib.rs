use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

#[wasm_bindgen(start)]
fn on_init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn run() {
    hello_from_rust();
}

fn greet(name: &str) -> String {
    format!("Miau, {name}!")
}

fn hello_from_rust() {
    log(&greet("Borys"));
}

fn log(msg: &str) {
    let formatted_msg = JsValue::from_str(&format!("[cat_tutor] {msg}"));
    log_1(&formatted_msg);
}
