// Todo: create a struct for the game, sprites, etc
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::collections::HashMap;
/*use std::f64;
use js_sys::Reflect;*/
//use web_sys::console;

//use serde::{Serialize, Deserialize};

mod values;
use values::*;

mod sprites;
use sprites::Sprite;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Game {
    html_element: Option<HtmlCanvasElement>,
    canvas_context: Option<CanvasRenderingContext2d>,
    bg_color: String,
    // Todo: transform this in a map
    data: HashMap<String, Sprite>,                                     // this is where the sprites and texts will be saved
    #[allow(dead_code)]
    custom_values: HashMap<String, String>                             // This is where the custom values created by the user of the lib will be saved
}

// Todo: create a different list containing only the sprites that are beeing rendered and exclude the ones that aren't visible

#[wasm_bindgen]
impl Game {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        //
        let (html_element, canvas_context, bg_color) = (None, None, String::from("white"));
        let data = HashMap::new();
        let custom_values = HashMap::new();

        Game {
            html_element,
            canvas_context,
            bg_color,
            data,
            custom_values,
        }
    }

    #[wasm_bindgen]
    pub fn inicialize(&mut self) -> Result<Game, JsValue> {
        //
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let e = document.create_element("canvas")?;

        body.append_child(&e)?;

        // draw the canvas
        let element: web_sys::HtmlCanvasElement = e.clone()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        self.html_element = Some(element.clone());

        let context = element
            .get_context("2d")
            .map_err(|_| JsValue::from_str("Failed to get context"))?
            .ok_or_else(|| JsValue::from_str("Context is null"))?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .map_err(|_| JsValue::from_str("Failed to cast context"))?;

        self.canvas_context = Some(context.clone());
        
        Ok(self.to_owned())
    }

    fn update(&mut self) -> Result<(), JsValue> {
        // set the bg color
        self.get_canvas_context().unwrap().save();
        // redraw sprites
        let v = self.data.clone().into_values().collect();
        Self::reload_sprites(self, v)?;

        Self::set_bg_color(&mut self.clone(), self.bg_color.clone())?;
        //
        Ok(())
    }

    fn reload_sprites(&mut self, mut vec: Vec<Sprite>) -> Result<(), JsValue> {
        //
        if vec.is_empty() {
            return Ok(());
        }
        //
        let first = vec.remove(0);
        // create the sprite
        let mut sprite = Sprite::new(first);
        let _ = sprite.render();
        //
        Self::reload_sprites(self, vec)?;
        //
        Ok(())
    }

    pub fn force_update(&mut self) -> Result<Game, JsValue> {
        //
        Self::update(&mut self.clone())?;
        Ok(self.to_owned())
    }

    pub fn get_html_element(&mut self) -> Option<HtmlCanvasElement> {
        self.html_element.clone()
    }

    pub fn get_canvas_context(&mut self) -> Option<CanvasRenderingContext2d> {
        self.canvas_context.clone()
    }

    pub fn resize_canvas(&mut self) -> Result<Game, JsValue> {
        // change the size of the canvas
        let (window, canvas) = (
            web_sys::window().expect("no global `window` exists"),
            Self::get_html_element(self).unwrap()
        );

        // Get the window proportions
        let (window_width, window_height) = (
            window.inner_width().unwrap().as_f64().unwrap() as u32,
            window.inner_height().unwrap().as_f64().unwrap() as u32,
        );
        
        // Resize the canvas
        canvas.set_width(window_width);
        canvas.set_height(window_height);

        Self::update(&mut self.clone())?;
        //
        return Ok(self.to_owned())
    }

    pub fn set_bg_color(&mut self, bg_color: String) -> Result<Game, JsValue> {
        //
        let element = Self::get_html_element(self).unwrap();
        let context = Self::get_canvas_context(self).unwrap();
        //
        let (width, height) = (element.width() as f64, element.height() as f64);

        context.set_fill_style(&JsValue::from_str(bg_color.as_str()));

        // Set a new background color
        context.fill_rect(0.0, 0.0, width, height);

        self.bg_color = bg_color;
        //
        return Ok(self.to_owned())
    }

    pub fn new_image(&mut self, id: String, x: f64, y: f64, path: String, size: Option<f64>, angle: Option<f64>) -> Result<Game, JsValue> {
        // get canvas
        let canvas = (self.get_canvas_context().unwrap(), self.get_html_element().unwrap());
        //
        let texture = Sprite {
            kind: Kind::Image(Image { path }),
            pos: (x, y),
            size,
            angle,
            canvas
        };
        // create sprite
        let mut sprite = Sprite::new(texture);
        let _ = sprite.render();
        //
        self.data.insert(id, sprite);
        //
        return Ok(self.to_owned())
    }

    pub fn new_text(&mut self, id: String, x: f64, y: f64, value: String, color: String, font: String) -> Result<Game, JsValue> {
        // get canvas
        let canvas = (self.get_canvas_context().unwrap(), self.get_html_element().unwrap());
        //
        let texture = Sprite {
            kind: Kind::Text(Text { value, color, font }),
            pos: (x, y),
            size: None,
            angle: None,
            canvas
        };
        // create sprite
        let mut sprite = Sprite::new(texture);
        let _ = sprite.render();
        //
        self.data.insert(id, sprite);
        return Ok(self.to_owned())
    }

    pub fn get_canvas_size(&mut self) -> Vec<u32> {
        let canvas = self.get_html_element().unwrap();
        //
        return vec![canvas.width(), canvas.height()]
    }

    pub fn update_sprite_value(&mut self, name: &str, x: f64, y: f64) -> Result<Game, JsValue> {
        // get the sprite
        if let Some(estrutura) = self.data.get_mut(name) {
            //console::log_1(&JsValue::from_str(estrutura.x.to_string().as_str()));
            //
            estrutura.pos = (x, y);
            return Ok(self.to_owned())
        }
        return Err(JsValue::from_str("sprite not found!"))
    }

    pub fn get_sprite_by_id(&mut self, name: String) -> JsValue {
        let sprite = self.data.get(&name).unwrap();
        //
        let value = ReturnSprite {
            x: sprite.pos.0,
            y: sprite.pos.1,
            size: sprite.size,
            angle: sprite.angle,
            kind: sprite.kind.clone()
        };
        // convert the sprite to a json
        return serde_wasm_bindgen::to_value(&value).unwrap();
    }
    
    /*pub fn create_custom_value(&mut self, name: String, value: String) -> Result<(), JsValue> {
        self.custom_values.insert(name, value);
        //
        return Ok(());
    }

    pub fn get_custom_value(&mut self, name: String) -> Result<String, JsValue> {
        self.custom_values.get(&name)
    }

    pub fn modify_custom_value(&mut self, name: String, value: String) -> Result<(), JsValue> {
        self.custom_values.insert(name, value);
        //
        return Ok(());
    }

    pub fn delete_custom_value(&mut self, name: String) -> Result<(), JsValue> {
        self.custom_values.remove(&name);
        //
        return Ok(());
    }*/
}