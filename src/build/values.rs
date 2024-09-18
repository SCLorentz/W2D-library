use std::collections::HashMap;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};

// https://medium.com/@mikecode/rust-how-to-store-values-of-different-types-in-a-vector-cf1b62120aa1
#[derive(Clone)]
pub struct CanvasFactory {
    pub element: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
    pub window: Window,
}

/*#[derive(Clone)]
pub enum Values {
    String(String),
    Int(i32),
}*/

#[derive(Clone)]
pub struct Game {
    pub score: u32,                                                         // instead we can use a HashMap to create custom values
    //pub values: HashMap<String, Values>,
    //
    pub canvas: CanvasFactory,
    pub default_bg_color: String,
    pub default_fg_color: String,
    //
    pub sprites: HashMap<String, Result<Texture, wasm_bindgen::JsValue>>
}

#[derive(Clone)]
pub struct Texture {
    pub x: f64,
    pub y: f64,
    pub texture: String,
    pub size: f64,
    pub angle: f64,
}

#[derive(Clone)]
pub struct Text {
    pub x: f64,
    pub y: f64,
    pub text: String,
    pub size: f64,
    pub font: String,
    pub color: String,
}