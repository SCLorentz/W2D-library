// Todo: create a struct for the game, sprites, etc
use wasm_bindgen::prelude::*;
use std::f64;
use web_sys::console;
use wasm_bindgen::closure::Closure;
use web_sys::{HtmlCanvasElement, HtmlImageElement};
use std::rc::Rc;
use std::collections::HashMap;

mod values;
use values::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

impl Sprite {
    fn new(x: f64, y: f64, texture: String, size: Option<f64>) -> Self {
        // return value
        let size = match size {
            Some(size) => size,
            None => 100.0,
        };
        return Self {
            x,
            y,
            texture,
            size,
        }
    }

    fn to_string(&self) -> String {
        format!("Player {{ texture: {}, size: {}, y: {}, x: {} }}", self.texture, self.size, self.y, self.x)
    }
    
    fn create(&mut self) -> Result<HtmlCanvasElement, JsValue> {
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

        context.begin_path();

        // create a new image (I used gemini for this)
        let image = Rc::new(HtmlImageElement::new().unwrap());
        let image_clone = image.clone();
        image.set_src(&String::from(self.texture.clone()));

        //
        let (img_h, img_w) = ( image.height() as f64, image.width() as f64 );
        let (dx, dy) = (self.x, self.y);
        //
        let size = self.size;
        let width = size * img_w / img_h;

        // Esperar o carregamento da imagem
        let closure = Closure::wrap(Box::new(move || {
            //context.draw_image_with_html_image_element(&image_clone, dx, dy).unwrap();
            context.draw_image_with_html_image_element_and_dw_and_dh(&image_clone, dx, dy, width, size).unwrap();
        }) as Box<dyn FnMut()>);
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
}

impl CanvasFactory {
    fn canvas() -> Result<CanvasFactory, JsValue> {
        // create a new canvas
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        // Manufacture the element we're gonna append
        let e = document.create_element("canvas")?;
        e.set_id("game-canvas");

        let _ = body.append_child(&e);

        // draw the canvas
        let canvas: web_sys::HtmlCanvasElement = e.clone()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        //return Ok(context);
        return Ok(CanvasFactory {
            context,
            element: canvas,
            window,
        });
    }
}

impl Game {
    fn new() -> Self {
        let canvas = CanvasFactory::canvas().unwrap();
        // get from user theme
        let bg_color = String::from("black");
        let fg_color = String::from("white");
        let sprites = HashMap::new();
        //
        let mut game = Self {
            score: 1,
            canvas,
            bg_color,
            fg_color,
            sprites
        };
        Self::draw(&mut game);
        //
        return game;
    }

    fn sprite(&mut self, name: &str, x: f64, y: f64, texture: String, size: Option<f64>) -> Result<Sprite, JsValue> {
        let mut sprite = Sprite::new(x, y, texture, size);
        let mut _sprite = Sprite::create(&mut sprite).unwrap();
        //
        let return_value = Ok(sprite).clone();
        //
        self.sprites.insert(name.to_string(), return_value.clone());
        return return_value;
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
        for (_, v) in self.sprites.clone() {
            // get the sprite values
            let texture = v.clone().unwrap().texture;
            let size = v.clone().unwrap().size;
            let x = v.clone().unwrap().x;
            let y = v.clone().unwrap().y;
            // create the sprite
            let mut sprite = Sprite::new(x, y, texture.clone(), Some(size));
            let mut _sprite = Sprite::create(&mut sprite).unwrap();
            //
            console::log_1(&JsValue::from_str(&texture.to_string()));
        }
        //
        context.set_fill_style(&JsValue::from_str(self.bg_color.clone().as_str()));

        // Set a new background color
        context.fill_rect(0.0, 0.0, width, height);

        // use the roboto font in /assets/font
        context.set_font("20px Arial");
        context.set_fill_style(&JsValue::from_str(self.fg_color.clone().as_str()));
        // draw the score
        let value = String::from("score: ") + &self.score.clone().to_string();
        context.fill_text(&value.to_string(), 10.0, 50.0).unwrap();
    }

    fn get_sprite_by_name(&mut self, name: &str) -> Option<&Result<Sprite, wasm_bindgen::JsValue>> {
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

    fn get_score(&mut self) {
        let string = format!("the current score is: {}", &self.score.clone());
        let value = JsValue::from_str(&string);
        //
        console::log_1(&value);
    }

    fn update_value(&mut self, name: &str) {
        // get the sprite
        if let Some(estrutura) = self.sprites.get_mut(name) {
            estrutura.clone().unwrap().x = 11.0;
            return;
        }
        console::log_1(&JsValue::from_str("sprite not found!"));
    }

    fn update_score(&mut self, value: u32) {
        self.score = value;
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
        Self::draw(self);
    }
}

#[wasm_bindgen]
pub fn start_game() -> Result<HtmlCanvasElement, JsValue> {
    let mut game = Game::new();
    let _ = game.sprite("cactus-2", 500.0, 500.0, String::from("/assets/template/cactus-2.png"), None);
    // methods
    game.print_sprite_info("cactus-2");     // prints a especific sprite by name
    game.list_all_sprites();                // print all the sprites in the game
    game.get_score();                       // print the current score of the game
    // Review: this method is not working
    game.update_value("cactus-2");          // update the value of a sprite
    game.print_sprite_info("cactus-2");     // print the new value of the sprite
    //
    game.update_score(10);                  // update the score
    game.redraw();                          // redraw the game
    // canvas html element
    //console::log_1(&game.canvas.clone().into());
    return Ok(game.canvas.element);
}