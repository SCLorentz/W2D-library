use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use serde::Serialize;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, Serialize)]
pub struct Image {
    pub path: String,
}

#[derive(Clone, Serialize)]
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

#[derive(Clone, Serialize)]
pub enum Kind {
    Image(Image),
    Text(Text),
}

#[derive(Clone, Serialize)]
pub struct ReturnSprite {
    pub x: f64,
    pub y: f64,
    pub size: Option<f64>,
    pub angle: Option<f64>,
    pub kind: Kind
}

#[derive(Debug)]
pub enum ErrorTypes {
    //EmptyName,
    //DuplicateName,
    //UnexpectedOverwrite,
    //NoMatch,
    NoGlobalWindow,
}

impl Display for ErrorTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            //ErrorTypes::EmptyName => write!(f, "Name cannot be empty"),
            //ErrorTypes::DuplicateName => write!(f, "Duplicate name found"),
            //ErrorTypes::UnexpectedOverwrite => write!(f, "Unexpected overwrite"),
            //ErrorTypes::NoMatch => write!(f, "a window was expected!"),
            ErrorTypes::NoGlobalWindow => write!(f, "no global window found!"),
        }
    }
}