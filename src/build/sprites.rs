use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use web_sys::HtmlImageElement;
use std::rc::Rc;

pub use crate::values::{Texture, Text};

impl Texture {
    //
    pub fn new(value: Texture) -> Self {
        value
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!("Player {{ texture: {}, size: {:?}, y: {}, x: {} }}", self.texture, self.size, self.y, self.x)
    }
    
    pub fn create(&mut self) -> Result<Texture, JsValue> {
        // get the canvas and context
        let (context, _canvas) = self.clone().canvas;

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
            self.size.unwrap_or(0.0),
            self.angle.unwrap_or(100.0)
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

    pub fn new(value: Text) -> Self {
        value
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        format!("Text {{ text: {}, size: {}, y: {}, x: {} }}", self.text, self.size, self.y, self.x)
    }
    
    pub fn create(&mut self) -> Result<Text, JsValue> {
        // get the canvas and context
        let (context, _) = self.clone().canvas;
        // style
        context.set_font(format!("{}px {}", self.size, self.font).as_str());
        context.set_fill_style(&JsValue::from_str(self.color.clone().as_str()));

        context.begin_path();
        context.fill_text(&self.text, self.x, self.y).unwrap();
        //
        return Ok(self.clone());
    }
}