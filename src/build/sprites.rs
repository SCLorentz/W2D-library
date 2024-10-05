/*use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use web_sys::{HtmlCanvasElement, HtmlImageElement};
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;

pub use crate::values::*;

fn values() -> (CanvasRenderingContext2d, HtmlCanvasElement) {
    // get the half of the window size
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("expecting a document on window");

    // Todo: substitute this method to get the canvas as well. Getting the canvas using the element ID isn't the best way
    // draw the canvas
    let canvas: web_sys::HtmlCanvasElement = document.get_element_by_id("game-canvas").unwrap()
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
    pub fn new(x: f64, y: f64, texture: String, size: Option<f64>, angle: Option<f64>) -> Self {
        Self {
            x,
            y,
            texture,
            size: size.unwrap_or(100.0),
            angle: angle.unwrap_or(0.0),
        }
    }

    pub fn to_string(&self) -> String {
        format!("Player {{ texture: {}, size: {}, y: {}, x: {} }}", self.texture, self.size, self.y, self.x)
    }
    
    pub fn create(&mut self) -> Result<Texture, JsValue> {
        // get the canvas and context
        let (context, _canvas) = values();

        // create a new image
        let image = Rc::new(HtmlImageElement::new().unwrap());
        let image_clone = image.clone();
        //
        image.set_src(&String::from(self.texture.clone()));
        // some values. This are needed cause the closure requests them
        let (img_h, img_w, dx, dy, size, angle) = ( 
            image.height() as f64,
            image.width() as f64,
            self.x,
            self.y,
            self.size,
            self.angle
        );
        let width = size * img_w / img_h;

        // Esperar o carregamento da imagem
        let closure = Closure::wrap(Box::new(move || {
            // I used AI to help me to rotate the image
            context.save();
            // Translate to the center of where the image will be
            context.translate(dx + width / 2.0, dy + size / 2.0).unwrap();
            // Rotate
            context.rotate(angle.to_radians()).unwrap();
            // Draw the image centered at (0, 0)
            context.draw_image_with_html_image_element_and_dw_and_dh(
                &image_clone, 
                -width / 2.0, 
                -size / 2.0, 
                width, 
                size
            ).unwrap();
            
            context.restore();
        }) as Box<dyn FnMut()>);
        //
        image.set_onload(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        return Ok(self.clone());
    }

    /*pub fn _rotate(&mut self,new_angle: f64) -> Result<Texture, JsValue> {
        let (context, _) = values();
        self.angle = new_angle;
        //
        context.save();
        // Translate to the center of where the image will be
        context.translate(dx + width / 2.0, dy + size / 2.0).unwrap();
        // Rotate
        context.rotate(angle.to_radians()).unwrap();
        //
        return Ok(self.clone());
    }*/

    /*fn resize(&mut self, width: f64, height: f64) {
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
        //
        return Self {
            x,
            y,
            text,
            size: size.unwrap_or(100.0),
            font: font.unwrap_or(String::from("Arial")),
            color: color.unwrap_or(String::from("black")),
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
}*/