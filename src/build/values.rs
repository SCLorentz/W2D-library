use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Clone, Debug)]
pub struct Image {
    pub path: String,
}

#[derive(Clone)]
pub struct Text {
    pub value: String,
    pub color: String,
    pub font: String,
}

#[derive(Clone)]
pub struct Sprite {
    pub kind: Kind,
    //
    pub pos: (f64, f64),
    pub size: Option<f64>,
    pub angle: Option<f64>,
    //
    pub canvas: (CanvasRenderingContext2d, HtmlCanvasElement)
}

#[derive(Clone)]
pub enum Kind {
    Image(Image),
    Text(Text),
}