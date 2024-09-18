// Todo: create a struct for the game, sprites, etc
use wasm_bindgen::prelude::*;
use std::f64;
use web_sys::console;
use web_sys::HtmlCanvasElement;
use std::collections::HashMap;

mod values;
use values::*;

mod sprites;
use sprites::Texture;

impl CanvasFactory {
    fn canvas() -> Result<CanvasFactory, JsValue> {
        // create a new canvas
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        // Manufacture the element we're gonna append
        let e = document.create_element("canvas")?;
        e.set_id("game-canvas");

        body.append_child(&e)?;

        // draw the canvas
        let element: web_sys::HtmlCanvasElement = e.clone()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = element
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        //return Ok(context);
        return Ok(CanvasFactory {
            context,
            element,
            window,
        });
    }
}

impl Game {
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
            score: 1,                // remove this later, cause it's not a game feature
            canvas,                  // all canvas data, the context, element and window
            default_bg_color,        // if no color is provided, use black to background
            default_fg_color,        // if no color is provided, use white to foreground
            sprites,                 // a hashmap of all the sprites in the game created
            //values: HashMap::new(),
        };
        // draw the first frame of the game
        Self::draw(&mut game);
        //
        return game;
    }

    /*fn create_custom_value(&mut self, name: &str, value: Values) {
        self.values.insert(name.to_string(), value);
        // return success or error
    }*/

    /*fn return_custom_value(&mut self, value: Result<Texture, JsValue>) -> Result<Texture, JsValue> {
        // get the value in the hashmap
        // return the value
    }*/
    
    /*fn modify_custom_value(&mut self, name: &str, value: Result<Texture, JsValue>) {
        // get the value in the hashmap
        // change it's value
        // return success or error
    }*/

    fn sprite(&mut self, name: &str, x: f64, y: f64, texture: String, size: Option<f64>) -> Result<Texture, JsValue> {
        let mut sprite = Texture::new(x, y, texture, size);
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
            let mut sprite = Texture::new(x, y, texture.clone(), Some(size));
            Texture::create(&mut sprite).unwrap(); // this is the sprite html element, if you need to use it, use the variable "sprite_element"
            //
            console::log_1(&JsValue::from_str(&texture.to_string()));
        }
        //
        context.set_fill_style(&JsValue::from_str(self.default_bg_color.clone().as_str()));

        // Set a new background color
        context.fill_rect(0.0, 0.0, width, height);

        // move this out of the game struct
        let _ = Self::draw_text(self, format!("score: {}", self.score).as_str(), 10.0, 50.0, "Arial");
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

    // remove score related methods, cause it's not a game feature
    // this should be created in the js side by the user of this lib
    fn get_score(&mut self) {
        let val = JsValue::from(format!("the current score is: {}", &self.score.clone()));
        //
        console::log_1(&val);
    }

    fn update_score(&mut self, value: u32) {
        self.score = value;
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

#[wasm_bindgen]
pub fn start_game() -> Result<HtmlCanvasElement, JsValue> {
    let mut game = Game::new();
    {
        // game settings
        game.sprite("cactus-2", 500.0, 500.0, String::from("/assets/template/cactus-2.png"), None)?;
        //
        //game.draw_text(format!("score: {}", game.score).as_str(), 10.0, 50.0, "Arial"); draw the score text here
        // methods
        game.print_sprite_info("cactus-2");     // prints a especific sprite by name
        game.list_all_sprites();                // print all the sprites in the game
        game.get_score();                       // print the current score of the game
        // Review: this method is not working
        game.update_sprite_value("cactus-2");   // update the value of a sprite
        game.print_sprite_info("cactus-2");     // print the new value of the sprite
        //
        game.update_score(10);                  // update the score
        game.redraw();                          // redraw the game
    }
    // canvas html element
    //console::log_1(&game.canvas.clone().into());
    return Ok(game.canvas.element);
}