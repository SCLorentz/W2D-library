/*use std::collections::HashMap;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};

// https://medium.com/@mikecode/rust-how-to-store-values-of-different-types-in-a-vector-cf1b62120aa1
#[derive(Clone)]
pub struct CanvasFactory {
    pub element: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
    pub window: Window,
}

#[derive(Clone)]
pub enum Values {
    String(String),
    Number(i32),
}
#[derive(Clone)]
pub struct Game {
    pub values: HashMap<String, Values>,
    //
    pub canvas: CanvasFactory,
    pub default_bg_color: String,
    pub default_fg_color: String,
    //
    pub sprites: HashMap<String, Result<Texture, wasm_bindgen::JsValue>>
}

#[derive(Debug)]
pub enum CustomValueError {
    EmptyName,
    DuplicateName,
    UnexpectedOverwrite,
    NoMatch,
}*/

use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Clone)]
pub struct Texture {
    pub x: f64,
    pub y: f64,
    pub texture: String,
    pub size: Option<f64>,
    pub angle: Option<f64>,
    pub canvas: (CanvasRenderingContext2d, HtmlCanvasElement)
}
#[derive(Clone)]
pub struct Text {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    #[allow(dead_code)] // this is anoying allow it for now
    pub angle: Option<f64>,
    //
    pub text: String,
    pub font: String,
    pub color: String,
    //
    pub canvas: (CanvasRenderingContext2d, HtmlCanvasElement)
}