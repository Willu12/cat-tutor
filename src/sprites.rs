use wasm_bindgen::JsValue;
use web_sys::HtmlImageElement;

pub struct Sprites {
    idle: HtmlImageElement,
    shouting: HtmlImageElement,
    lying: HtmlImageElement,
    sleeping: HtmlImageElement,
}

impl Sprites {
    pub async fn new(sprite_dir_path: &str) -> Result<Self, JsValue> {
        Ok(Sprites {
            idle: load_image(sprite_dir_path, Frame::Idle).await?,
            shouting: load_image(sprite_dir_path, Frame::Shouting).await?,
            lying: load_image(sprite_dir_path, Frame::Lying).await?,
            sleeping: load_image(sprite_dir_path, Frame::Sleeping).await?,
        })
    }

    pub fn get(&self, frame: Frame) -> &HtmlImageElement {
        match frame {
            Frame::Idle => &self.idle,
            Frame::Lying => &self.lying,
            Frame::Shouting => &self.shouting,
            Frame::Sleeping => &self.sleeping,
        }
    }
}

async fn load_image(sprite_dir_path: &str, frame: Frame) -> Result<HtmlImageElement, JsValue> {
    let image = HtmlImageElement::new()?;
    let file_name = frame.file_name();
    let url = format!("{sprite_dir_path}{file_name}");
    image.set_src(&url);
    image
        .decode()
        .await
        .map_err(|_| format!("failed to decode {url}"))?;
    Ok(image)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Frame {
    Idle,
    Shouting,
    Lying,
    Sleeping,
}

impl Frame {
    pub fn file_name(self) -> &'static str {
        match self {
            Frame::Idle => "idle.png",
            Frame::Lying => "lying.png",
            Frame::Shouting => "shouting.png",
            Frame::Sleeping => "sleeping.png",
        }
    }
}
