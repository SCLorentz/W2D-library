use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use web_sys::HtmlImageElement;
use std::rc::Rc;

pub use crate::values::Sprite;
use crate::{Image, Kind, Text};

impl Sprite
{
    pub fn new(value: Sprite) -> Self { value }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        //
        let (x, y) = self.pos;
        match &self.kind {
            Kind::Image(image) => format!("Image {{ texture: {:?}, size: {:?}, x: {}, y: {} }}", image.path, self.size, x, y),
            Kind::Text(text) => format!("Text {{ value: {:?}, color: {:?}, font: {:?}, x: {}, y: {} }}", text.value, text.color, text.font, x, y),
        }
    }

    pub fn render(&mut self) -> Result<(), JsValue>
    {
        match &self.kind {
            Kind::Image(image) => {
                Self::create_image(self, image.clone())?;
                Ok(())
            }
            Kind::Text(text) => {
                Self::create_text(self, text.clone())?;
                Ok(())
            }
        }
    }
    
    pub fn create_image(&mut self, image: Image) -> Result<Sprite, JsValue>
    {
        let (context, _) = self.clone().canvas;
        let (x, y) = self.pos;
        let image_path = image.path;

        // create a new image
        let image = Rc::new(HtmlImageElement::new().unwrap());
        let image_clone = image.clone();
        //
        image.set_src(&String::from(image_path));
        // some values. This are needed cause the closure requests them
        let (img_h, img_w, dx, dy, size, angle) = ( 
            image.height() as f64,
            image.width() as f64,
            x,
            y,
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

        Ok(self.clone())
    }

    pub fn create_text(&mut self, text: Text) -> Result<Sprite, JsValue>
    {
        let (x, y) = self.pos;
        let (context, _) = self.clone().canvas;
        // style
        context.set_font(format!("{}px {}", self.size.unwrap_or(100.0), text.font).as_str());
        context.set_fill_style(&JsValue::from_str(text.color.as_str()));

        context.begin_path();
        context.fill_text(&text.value, x, y).unwrap();
        //
        Ok(self.clone())
    }
}