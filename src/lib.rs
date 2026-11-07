use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

#[wasm_bindgen(start)]
fn on_init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    hello_from_rust();
    create_square()
}

fn greet(name: &str) -> String {
    format!("Miau, {name}!")
}

fn hello_from_rust() {
    log(&greet("Borys"));
}

fn log(msg: &str) {
    let formatted_msg = JsValue::from_str(&format!("[cat-tutor] {msg}"));
    log_1(&formatted_msg);
}

const CAT_ELEMENT_ID: &str = "cat-tutor-root";

fn create_square() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("Failed to get window.")?;
    let document = window.document().ok_or("Failed to get document")?;
    let body = document.body().ok_or("Failed to get body from document.")?;

    if document.get_element_by_id(CAT_ELEMENT_ID).is_some() {
        return Ok(());
    }

    let element = document.create_element("div")?;
    element.set_id(CAT_ELEMENT_ID);
    element.set_text_content(Some("MIAU"));
    body.append_child(&element)?;

    let body_children_count = body.child_element_count();
    log(&format!("body children: {body_children_count}"));

    Ok(())
}
