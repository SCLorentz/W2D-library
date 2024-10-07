use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::collections::HashMap;
//use web_sys::console;

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
    custom_values: HashMap<String, String>                             // This is where the custom values created by the user of the lib will be saved
}

// Todo: create a different list containing only the sprites that are beeing rendered and exclude the ones that aren't visible

#[wasm_bindgen]
impl Game
{
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game
    { Game {
        html_element: None,
        canvas_context: None,
        bg_color: String::from("white"),
        data: HashMap::new(),
        custom_values: HashMap::new()
    }}

    #[wasm_bindgen]
    pub fn inicialize(&mut self) -> Result<Game, JsValue>
    {
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

        self.canvas_context = Some(context);
        
        Ok(self.to_owned())
    }

    fn update(&mut self) -> Result<(), JsValue>
    {
        // set the bg color
        self.get_canvas_context().save();
        // redraw sprites
        let v = self.data.clone().into_values().collect();
        Self::reload_sprites(self, v)?;

        Self::set_bg_color(&mut self.clone(), self.bg_color.clone())?;
        //
        Ok(())
    }

    fn reload_sprites(&mut self, mut vec: Vec<Sprite>) -> Result<(), JsValue>
    {
        if vec.is_empty() {
            return Ok(());
        }
        //
        let first = vec.remove(0);
        // create the sprite
        let mut sprite = Sprite::new(first);
        sprite.render()?;
        //
        Self::reload_sprites(self, vec)?;
        //
        Ok(())
    }

    pub fn force_update(&mut self) -> Result<Game, JsValue> 
    {
        Self::update(&mut self.clone())?;
        Ok(self.to_owned())
    }

    pub fn get_html_element(&mut self) -> HtmlCanvasElement
    {
        self.html_element.to_owned().unwrap()
    }

    pub fn get_canvas_context(&mut self) -> CanvasRenderingContext2d
    {
        self.canvas_context.to_owned().unwrap()
    }

    fn get_window_proportions() -> (u32, u32)
    {(
        web_sys::window().expect("no global `window` exists").inner_width().unwrap().as_f64().unwrap() as u32,
        web_sys::window().expect("no global `window` exists").inner_height().unwrap().as_f64().unwrap() as u32,
    )}

    pub fn resize_canvas(&mut self) -> Result<Game, JsValue>
    {
        let canvas = Self::get_html_element(self);
        // Get the window proportions
        let (window_width, window_height) = Self::get_window_proportions();
        
        // Resize the canvas
        canvas.set_width(window_width);
        canvas.set_height(window_height);

        Self::update(&mut self.clone())?;
        //
        Ok(self.to_owned())
    }

    pub fn set_bg_color(&mut self, bg_color: String) -> Result<Game, JsValue>
    {
        let element = Self::get_html_element(self);
        let context = Self::get_canvas_context(self);
        //
        let (width, height) = (element.width() as f64, element.height() as f64);

        context.set_fill_style(&JsValue::from_str(bg_color.as_str()));

        // Set a new background color
        context.fill_rect(0.0, 0.0, width, height);

        self.bg_color = bg_color;
        //
        Ok(self.to_owned())
    }

    pub fn new_image(&mut self, id: String, x: f64, y: f64, path: String, size: Option<f64>, angle: Option<f64>) -> Result<Game, JsValue>
    {
        let canvas = (self.get_canvas_context(), self.get_html_element());
        let kind = Kind::Image(Image { path });
        //
        // create sprite
        let mut sprite = Sprite::new( Sprite {
            kind,
            pos: (x, y),
            size,
            angle,
            canvas
        });
        sprite.render()?;
        //
        self.data.insert(id, sprite);
        //
        Ok(self.to_owned())
    }

    pub fn new_text(&mut self, id: String, x: f64, y: f64, value: String, color: String, font: String) -> Result<Game, JsValue>
    {
        let canvas = (self.get_canvas_context(), self.get_html_element());
        let kind = Kind::Text( Text { value, color, font });
        // create sprite
        let mut sprite = Sprite::new( Sprite {
            kind,
            pos: (x, y),
            size: None,
            angle: None,
            canvas
        });
        sprite.render()?;
        //
        self.data.insert(id, sprite);
        Ok(self.to_owned())
    }

    pub fn get_canvas_size(&mut self) -> Vec<u32>
    {
        let canvas = self.get_html_element();
        //
        return vec![canvas.width(), canvas.height()]
    }

    pub fn update_sprite_value(&mut self, name: &str, x: f64, y: f64) -> Result<Game, JsValue>
    {
        // get the sprite
        if let Some(estrutura) = self.data.get_mut(name) {
            //console::log_1(&JsValue::from_str(estrutura.x.to_string().as_str()));
            //
            estrutura.pos = (x, y);
            return Ok(self.to_owned())
        }
        Err(JsValue::from_str("sprite not found!"))
    }

    pub fn get_sprite_by_id(&mut self, name: String) -> JsValue
    {
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
    
    pub fn create_custom_value(&mut self, name: String, value: String) -> Result<Game, JsValue>
    {
        self.custom_values.insert(name, value);
        //
        Ok(self.to_owned())
    }

    pub fn get_custom_value(&mut self, name: String) -> Result<String, JsValue>
    {
        Ok(self.custom_values.get(&name).unwrap().to_string())
    }

    pub fn modify_custom_value(&mut self, _name: String, _value: String) -> Result<(), JsValue>
    {
        Err(JsValue::from_str("not implemented yet!"))
    }

    pub fn delete_custom_value(&mut self, name: String) -> Result<Game, JsValue>
    {
        self.custom_values.remove(&name);
        //
        Ok(self.to_owned())
    }
}