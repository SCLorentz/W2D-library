use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use web_sys::{HtmlCanvasElement, HtmlImageElement};
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;

pub use crate::values::*;

fn values() -> (CanvasRenderingContext2d, HtmlCanvasElement) {
    // get the half of the window size
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("expecting a document on window");

    // key down event
    //#[cfg(any(doc, feature = "keypress"))]

    // Todo: substitute this method to get the canvas as well
    let canvas = document.get_element_by_id("game-canvas").unwrap();

    // draw the canvas
    let canvas: web_sys::HtmlCanvasElement = canvas.clone()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    return (context, canvas);
}

impl Texture {
    pub fn new(x: f64, y: f64, texture: String, size: Option<f64>) -> Self {
        // return value data
        let size = match size {
            Some(size) => size,
            None => 100.0,
        };
        //
        return Self {
            x,
            y,
            texture,
            size,
        }
    }

    pub fn to_string(&self) -> String {
        format!("Player {{ texture: {}, size: {}, y: {}, x: {} }}", self.texture, self.size, self.y, self.x)
    }
    
    pub fn create(&mut self) -> Result<HtmlCanvasElement, JsValue> {
        // get the canvas and context
        let (context, canvas) = values();

        context.begin_path();

        // create a new image (I used gemini for this)
        let image = Rc::new(HtmlImageElement::new().unwrap());
        let image_clone = image.clone();
        image.set_src(&String::from(self.texture.clone()));
        // some values. This are needed cause the closure requests them
        let (img_h, img_w, dx, dy, size) = ( 
            image.height() as f64,
            image.width() as f64,
            self.x,
            self.y,
            self.size
        );
        let width = size * img_w / img_h;

        // Esperar o carregamento da imagem
        let closure = Closure::wrap(Box::new(move || {
            context.draw_image_with_html_image_element_and_dw_and_dh(&image_clone, dx, dy, width, size).unwrap();
        }) as Box<dyn FnMut()>);
        //
        image.set_onload(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        return Ok(canvas);
    }

    /*fn rotate(&mut self, angle: f64) {
        self.x += angle.sin() * 10.0;
        self.y += angle.cos() * 10.0;
    }

    fn resize(&mut self, width: f64, height: f64) {
        self.width = (width / 2.0) - (self.width / 2.0);
        self.height = (height / 2.0) - (self.height / 2.0);
    }*/

    /*fn update_sprite_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }*/
}

impl Text {
    pub fn new(x: f64, y: f64, text: String, size: Option<f64>, font: Option<String>, color: Option<String>) -> Self {
        // values and their defaults
        let size = match size {
            Some(size) => size,
            None => 100.0,
        };
        let font = match font {
            Some(font) => font,
            None => String::from("Arial"),
        };
        let color = match color {
            Some(color) => color,
            None => String::from("black"),
        };
        //
        return Self {
            x,
            y,
            text,
            size,
            font,
            color,
        }
    }

    /*pub fn to_string(&self) -> String {
        format!("Text {{ text: {}, size: {}, y: {}, x: {} }}", self.text, self.size, self.y, self.x)
    }*/
    
    pub fn create(&mut self) -> Result<HtmlCanvasElement, JsValue> {
        // get the canvas and context
        let (context, canvas) = values();
        // style
        context.set_font(format!("{}px {}", self.size, self.font).as_str());
        context.set_fill_style(&JsValue::from_str(self.color.clone().as_str()));

        context.begin_path();
        context.fill_text(&self.text, self.x, self.y).unwrap();
        return Ok(canvas);
    }
}