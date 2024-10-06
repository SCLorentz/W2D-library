// Todo: create a struct for the game, sprites, etc
use wasm_bindgen::prelude::*;
use web_sys::{/*js_sys::Object,*/ CanvasRenderingContext2d, HtmlCanvasElement};
use std::collections::HashMap;
/*use web_sys::CanvasRenderingContext2d;
use std::f64;
use web_sys::HtmlCanvasElement;
use std::collections::HashMap;
use js_sys::Reflect;*/
use web_sys::console;

//use js_sys::Object;

mod values;
use values::*;

mod sprites;
use sprites::Sprite;

/*impl Game {
    fn new() -> Self {
        // game data
        let (canvas, default_bg_color, default_fg_color, sprites) = (
            CanvasFactory::canvas().unwrap(),
            String::from("black"),
            String::from("white"),
            HashMap::new()
        );
        //
        let mut game = Self {
            canvas,                  // all canvas data, the context, element and window
            default_bg_color,        // if no color is provided, use black to background
            default_fg_color,        // if no color is provided, use white to foreground
            sprites,                 // a hashmap of all the sprites in the game created
            values: HashMap::new(),  // a hashmap of all the values created by the user
        };
        // draw the first frame of the game
        Self::draw(&mut game);
        //
        return game;
    }

    fn create_custom_value(&mut self, name: &str, value: Values) -> Result<(), String> {
        if self.values.contains_key(name) {
            //return Err(CustomValueError::DuplicateName);
            return Err(String::from("Duplicate name"));
        }
        //
        let _ = match self.values.insert(name.to_string(), value) {
            //Some(_) => Err(CustomValueError::UnexpectedOverwrite),
            Some(_) => Err(String::from("Unexpected overwrite")),
            None => Ok(()),
        };
        // return success or error
        return Ok(());
    }

    fn return_custom_value(&mut self, name: &str) -> Result<&Values, JsValue> {
        //return self.values.get(name).unwrap_or(CustomValueError::NoMatch);
        return Ok(self.values.get(name).unwrap());//.unwrap_or(JsValue::from_str("No match")));
    }
    
    /*fn modify_custom_value(&mut self, name: &str, value: Result<Texture, JsValue>) {
        // get the value in the hashmap
        // change it's value
        // return success or error
    }*/

    fn sprite(&mut self, name: &str, x: f64, y: f64, texture: String, size: Option<f64>) -> Result<Texture, JsValue> {
        let mut sprite = Texture::new(x, y, texture, size, None);
        let mut _sprite = Texture::create(&mut sprite).unwrap();
        //
        let return_value = Ok(sprite).clone();
        //
        self.sprites.insert(name.to_string(), return_value.clone());
        return return_value;
    }

    fn draw_text(&mut self, text: &str, x: f64, y: f64, font: &str) -> Result<Text, JsValue> {
        let mut text = Text::new(x, y, text.to_string(), Some(20.0), Some(font.to_string()), Some(self.default_fg_color.clone()));
        let _text = text.create().unwrap();
        //
        return Ok(text);
    }

    fn draw(&mut self) {
        Self::resize_canvas(self);
        // get the values
        let (context, canvas, window)= (
            self.canvas.context.clone(),
            self.canvas.element.clone(),
            self.canvas.window.clone(),
        );
        // window size
        let (x, y) = (
            window.inner_width().unwrap().as_f64().unwrap() / 2.0,
            window.inner_height().unwrap().as_f64().unwrap() / 2.0
        );
        // canvas size
        let (width, height) = (canvas.width() as f64, canvas.height() as f64);

        // create player (remove this later)
        let _ = self.sprite("Player", x, y, String::from("/assets/base/player.png"), Some(100.0));
        //
        for (_, val) in self.sprites.clone() {
            // get the sprite values
            let (texture, size, x, y) = (
                val.clone().unwrap().texture,
                val.clone().unwrap().size,
                val.clone().unwrap().x,
                val.clone().unwrap().y,
            );
            // create the sprite
            let mut sprite = Texture::new(x, y, texture.clone(), Some(size), Some(90.0));
            Texture::create(&mut sprite).unwrap(); // this is the sprite html element, if you need to use it, use the variable "sprite_element"
            //
            console::log_1(&JsValue::from_str(&texture.to_string()));
        }
        //
        context.set_fill_style(&JsValue::from_str(self.default_bg_color.clone().as_str()));

        // Set a new background color
        context.fill_rect(0.0, 0.0, width, height);

        // move this out of the game struct
        //let _ = Self::draw_text(self, format!("score: {}", self.score).as_str(), 10.0, 50.0, "Arial");
        let _ = self.draw_text("hello world", 10.0, 50.0, "Arial");
    }

    // now, I'm going in a path where text and sprites are differents things, but I should rethink this. See the pros and cons
    fn get_sprite_by_name(&mut self, name: &str) -> Option<&Result<Texture, JsValue>> {
        self.sprites.get(&name.to_string())
    }

    fn print_sprite_info(&mut self, name: &str) {
        let get_sprite = self.get_sprite_by_name(name);
        //
        let sprite_string = get_sprite.unwrap().clone().unwrap().to_string();
        let js_value = JsValue::from_str(&sprite_string);
        //
        console::log_1(&js_value);
    }

    fn list_all_sprites(&mut self) {
        for (key, _) in self.sprites.clone() {
            let get_sprite = self.get_sprite_by_name(&key);
            //
            let sprite_string = get_sprite.unwrap().clone().unwrap().to_string();
            let js_value = JsValue::from_str(&sprite_string);
            //
            console::log_1(&js_value);
        }
    }

    // use the sprite struct to update the sprite values instead of the game struct to keep them organizated, share the hashmap between them
    fn update_sprite_value(&mut self, name: &str) {
        // get the sprite
        if let Some(estrutura) = self.sprites.get_mut(name) {
            estrutura.clone().unwrap().x = 11.0;
            return;
        }
        console::log_1(&JsValue::from_str("sprite not found!"));
    }

    fn resize_canvas(&mut self) {
        // change the size of the canvas
        let (window, canvas) = (
            self.canvas.window.clone(),
            self.canvas.element.clone()
        );

        // Get the window proportions
        let (window_width, window_height) = (
            window.inner_width().unwrap().as_f64().unwrap() as u32,
            window.inner_height().unwrap().as_f64().unwrap() as u32,
        );
        
        // Resize the canvas
        canvas.set_width(window_width);
        canvas.set_height(window_height);
    }

    fn _get_canvas_size(&mut self) -> (f64, f64) {
        let canvas = self.canvas.element.clone();
        return (canvas.width() as f64, canvas.height() as f64);
    }

    fn redraw(&mut self) {
        // clear the canvas
        let (context, canvas) = (
            self.canvas.context.clone(),
            self.canvas.element.clone()
        );
        //
        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        //
        Self::draw(self);
    }
}

// extra. This is not a game feature

fn get_score(&mut self) {
    let val = JsValue::from(format!("the current score is: {}", &self.score.clone()));
    //
    console::log_1(&val);
}

fn update_score(&mut self, value: u32) {
    self.score = value;
}*/

#[wasm_bindgen]
#[derive(Clone)]
pub struct Game {
    html_element: Option<HtmlCanvasElement>,
    canvas_context: Option<CanvasRenderingContext2d>,
    bg_color: String,
    // Todo: transform this in a map
    data: HashMap<String, Sprite>,                                     // this is where the sprites and texts will be saved
    _custom_value: HashMap<String, String>                              // This is where the custom values created by the user of the lib will be saved
}

// Todo: create a different list containing only the sprites that are beeing rendered and exclude the ones that aren't visible

#[wasm_bindgen]
impl Game {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        //
        let (html_element, canvas_context, bg_color) = (None, None, String::from("white"));
        let data = HashMap::new();
        let _custom_value = HashMap::new();

        Game {
            html_element,
            canvas_context,
            bg_color,
            data,
            _custom_value,
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
        
        Ok(self.clone())
    }

    fn update(&mut self) {
        // set the bg color
        self.get_canvas_context().unwrap().save();
        // redraw sprites
        let v = self.data.clone().into_values().collect();
        let _ = Self::reload_sprites(self, v);

        Self::set_bg_color(&mut self.clone(), self.bg_color.clone());
        //
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

    pub fn force_update(&mut self) {
        //
        Self::update(&mut self.clone());
    }

    pub fn get_html_element(&mut self) -> Option<HtmlCanvasElement> {
        self.html_element.clone()
    }

    pub fn get_canvas_context(&mut self) -> Option<CanvasRenderingContext2d> {
        self.canvas_context.clone()
    }

    pub fn resize_canvas(&mut self) -> Game {
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

        Self::update(&mut self.clone());
        //
        return self.clone()
    }

    pub fn set_bg_color(&mut self, bg_color: String) {
        //
        let element = Self::get_html_element(self).unwrap();
        let context = Self::get_canvas_context(self).unwrap();
        //
        let (width, height) = (element.width() as f64, element.height() as f64);

        context.set_fill_style(&JsValue::from_str(bg_color.as_str()));

        // Set a new background color
        context.fill_rect(0.0, 0.0, width, height);

        self.bg_color = bg_color;
    }

    pub fn new_image(&mut self, id: String, x: f64, y: f64, path: String, size: Option<f64>, angle: Option<f64>) -> Game {
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
        return self.clone()
    }

    pub fn new_text(&mut self, id: String, x: f64, y: f64, value: String, color: String, font: String) -> Game {
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
        return self.clone()
    }

    pub fn get_canvas_size(&mut self) -> Vec<u32> {
        let canvas = self.get_html_element().unwrap();
        //
        return vec![canvas.width(), canvas.height()]
    }

    pub fn update_sprite_value(&mut self, name: &str, x: f64, y: f64) {
        //
        console::log_1(&JsValue::from_str(&x.to_string()));
        // get the sprite
        if let Some(estrutura) = self.data.get_mut(name) {
            //console::log_1(&JsValue::from_str(estrutura.x.to_string().as_str()));
            //
            estrutura.pos = (x, y);
            return;
        }
        console::log_1(&JsValue::from_str("sprite not found!"));
    }
}