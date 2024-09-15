// Todo: create a struct for the game, sprites, etc
use wasm_bindgen::prelude::*;
use std::f64;
use std::collections::HashMap;
//use web_sys::console;
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, Window};
use std::rc::Rc;


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[derive(Clone)]
struct Game {
    score: u32,
    canvas: CanvasFactory,
    bg_color: String,
    fg_color: String,
    sprites: HashMap<String, Result<Sprite, wasm_bindgen::JsValue>>
}

#[derive(Clone)]
#[derive(Default)]
struct Sprite {
    x: f64,
    y: f64,
    texture: String,
    size: f64,
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

// https://medium.com/@mikecode/rust-how-to-store-values-of-different-types-in-a-vector-cf1b62120aa1
#[derive(Clone)]
struct CanvasFactory {
    context: CanvasRenderingContext2d,
    element: HtmlCanvasElement,
    window: Window,
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
        let (context, canvas, window) = (
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

        // create sprites
        let _ = self.sprite("Player", x, y, String::from("/assets/base/player.png"), Some(100.0));
        let _ = self.sprite("cactus-6", 300.0, 100.0, String::from("/assets/template/cactus-6.png"), None);
        let _ = self.sprite("cactus-5", 600.0, 100.0, String::from("/assets/template/cactus-5.png"), None);
        // get sprite again with
        let _get_player = self.get_sprite_by_name("Player");
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

    fn resize_canvas(&mut self) {
        // change the size of the canvas
        let window = self.canvas.window.clone();
        let canvas = self.canvas.element.clone();

        // Get the window proportions
        let (window_width, window_height) = (
            window.inner_width().unwrap().as_f64().unwrap() as u32,
            window.inner_height().unwrap().as_f64().unwrap() as u32,
        );
        
        // Resize the canvas
        canvas.set_width(window_width);
        canvas.set_height(window_height);
    }
}

#[wasm_bindgen]
pub fn start_game() -> Result<HtmlCanvasElement, JsValue> {
    let game = Game::new();
    // canvas html element
    /*console::log_1(&game.canvas.clone().into());
    // score
    console::log_1(&game.score.clone().into());*/
    return Ok(game.canvas.element);
}