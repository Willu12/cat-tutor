use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;
use web_sys::console::log_1;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement};

const CAT_ELEMENT_ID: &str = "cat-tutor-root";
const SPRITE_SIZE: u32 = 32;

#[wasm_bindgen(start)]
fn on_init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub async fn run(sprite_dir_path: &str) -> Result<(), JsValue> {
    create_overlay(sprite_dir_path).await
}

fn log(msg: &str) {
    let formatted_msg = JsValue::from_str(&format!("[cat-tutor] {msg}"));
    log_1(&formatted_msg);
}

async fn create_overlay(sprite_dir_path: &str) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("Failed to get window.")?;
    let document = window.document().ok_or("Failed to get document")?;
    let body = document.body().ok_or("Failed to get body from document.")?;

    if document.get_element_by_id(CAT_ELEMENT_ID).is_some() {
        log("overlay already exists,skipping");
        return Ok(());
    }

    let div_element = create_div(&document)?;
    let (canvas, rendering_ctx) = create_canvas(&document)?;

    div_element.append_child(&canvas)?;
    body.append_child(&div_element)?;

    // Draw cat image
    let image = load_image(&format!("{sprite_dir_path}idle.png")).await?;
    rendering_ctx.draw_image_with_html_image_element(&image, 0., 0.)?;

    Ok(())
}

fn create_div(document: &Document) -> Result<Element, JsValue> {
    let div_element = document.create_element("div")?;
    div_element.set_id(CAT_ELEMENT_ID);
    Ok(div_element)
}

async fn load_image(url: &str) -> Result<HtmlImageElement, JsValue> {
    let image = HtmlImageElement::new()?;
    image.set_src(url);
    image
        .decode()
        .await
        .map_err(|_| format!("failed to decode {url}"))?;
    Ok(image)
}

fn create_canvas(
    document: &Document,
) -> Result<(HtmlCanvasElement, CanvasRenderingContext2d), JsValue> {
    let canvas_element = document.create_element("canvas")?;
    let canvas = canvas_element
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| "created element is not a canvas")?;

    canvas.set_width(SPRITE_SIZE);
    canvas.set_height(SPRITE_SIZE);
    let ctx = canvas
        .get_context("2d")?
        .ok_or("Failed to get context.")?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|_| "Canvas context is not 2d")?;

    Ok((canvas, ctx))
}
