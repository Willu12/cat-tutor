use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement};

const CAT_ELEMENT_ID: &str = "cat-tutor-root";
const SPRITE_SIZE: u32 = 32;

#[wasm_bindgen(start)]
fn on_init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn run(sprite_dir_path: &str) -> Result<(), JsValue> {
    hello_from_rust(sprite_dir_path);
    create_overlay()
}

fn greet(name: &str) -> String {
    format!("Miau, {name}!")
}

fn hello_from_rust(sprite_dir_path: &str) {
    log(&greet("Borys"));
    log(&format!("{}idle.png", sprite_dir_path));
}

fn log(msg: &str) {
    let formatted_msg = JsValue::from_str(&format!("[cat-tutor] {msg}"));
    log_1(&formatted_msg);
}

fn create_overlay() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("Failed to get window.")?;
    let document = window.document().ok_or("Failed to get document")?;
    let body = document.body().ok_or("Failed to get body from document.")?;

    if document.get_element_by_id(CAT_ELEMENT_ID).is_some() {
        log("overlay already exists,skipping");
        return Ok(());
    }

    let div_element = create_div(&document)?;
    let canvas = create_canvas(&document)?;
    div_element.append_child(&canvas)?;
    body.append_child(&div_element)?;

    Ok(())
}

fn create_div(document: &Document) -> Result<Element, JsValue> {
    let div_element = document.create_element("div")?;
    div_element.set_id(CAT_ELEMENT_ID);
    Ok(div_element)
}

fn create_canvas(document: &Document) -> Result<HtmlCanvasElement, JsValue> {
    let canvas_element = document.create_element("canvas")?;
    let canvas = canvas_element
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| "creted element is not a canvas")?;

    canvas.set_width(SPRITE_SIZE);
    canvas.set_height(SPRITE_SIZE);
    let ctx = canvas
        .get_context("2d")?
        .ok_or("Failed to get context.")?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|_| "Canvas context is not 2d")?;

    ctx.set_fill_style_str("blue");
    ctx.fill_rect(0., 0., f64::from(SPRITE_SIZE), f64::from(SPRITE_SIZE));

    Ok(canvas)
}
