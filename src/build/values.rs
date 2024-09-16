use std::collections::HashMap;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};

// https://medium.com/@mikecode/rust-how-to-store-values-of-different-types-in-a-vector-cf1b62120aa1
#[derive(Clone)]
pub struct CanvasFactory {
    pub element: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
    pub window: Window,
}

#[derive(Clone)]
pub struct Game {
    pub score: u32,
    pub canvas: CanvasFactory,
    pub bg_color: String,
    pub fg_color: String,
    pub sprites: HashMap<String, Result<Sprite, wasm_bindgen::JsValue>>
}

#[derive(Clone)]
pub struct Sprite {
    pub x: f64,
    pub y: f64,
    pub texture: String,
    pub size: f64,
}