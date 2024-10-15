//use serde::de::value::Error;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlElement, HtmlImageElement, Window};
use std::{collections::HashMap, rc::Rc};
//use web_sys::console;
//use std::time::{Instant, Duration};

mod values;
use values::*;

mod sprites;
use sprites::Sprite;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Game {
    // fps
    #[allow(dead_code)]
    fps: u32,
    //last_update: Instant,
    // html canvas
    html_element: Option<HtmlCanvasElement>,
    canvas_context: Option<CanvasRenderingContext2d>,
    background: String,
    // values
    data: HashMap<String, Sprite>,                                     // this is where the sprites and texts will be saved
    custom_values: HashMap<String, String>,                            // This is where the custom values created by the user of the lib will be saved
    // html
    window: Window,
    document: Document,
    body: HtmlElement
}

// Todo: create a different list containing only the sprites that are beeing rendered and exclude the ones that aren't visible

#[wasm_bindgen]
impl Game
{
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self
    {
        let expect_window = ErrorTypes::NoGlobalWindow.to_string();
        //
        let window = web_sys::window().expect(&expect_window);
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");
        //
        Self {
            html_element: None,
            canvas_context: None,
            background: String::from("white"),
            data: HashMap::new(),
            custom_values: HashMap::new(),
            //
            fps: 60,
            //last_update: Instant::now(),
            //
            window,
            document,
            body
        }
    }

    #[wasm_bindgen]
    pub fn inicialize(&mut self) -> Result<Game, JsValue>
    {
        // check if the canvas was already created
        if self.html_element.is_some() {
            return Ok(self.to_owned())
        }

        let canvas = self.document.create_element("canvas")?;

        self.body.append_child(&canvas)?;

        // draw the canvas
        let element: web_sys::HtmlCanvasElement = canvas
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
    // I need to create a way of update this with fps, ideas on 'broken.rs'
    {
        // set the bg color
        self.get_canvas_context().save();
        // background appears in the first layer 
        Self::set_bg_color(&mut self.clone(), self.background.clone())?;
        // redraw sprites, sprites appears in the second layer
        let v = self.data.clone().into_values().collect();
        Self::reload_sprites(self, v)?;
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

    fn get_window_proportions(&mut self) -> (f64, f64)
    {(
        self.window.inner_width().unwrap_or_default().as_f64().unwrap(),
        self.window.inner_height().unwrap_or_default().as_f64().unwrap()
    )}
    
    pub fn resize_canvas(&mut self) -> Result<Game, JsValue>
    {
        let canvas = Self::get_html_element(self);
        // Get the window proportions
        let (window_width, window_height) = Self::get_window_proportions(self);
        
        // Resize the canvas
        canvas.set_width(window_width as u32);
        canvas.set_height(window_height as u32);

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

        self.background = bg_color;
        //
        Ok(self.to_owned())
    }

    pub fn set_bg_image(&mut self, path: String) -> Result<Game, JsValue>
    {
        let context = Self::get_canvas_context(self);
        //
        let (width, height) = Self::get_window_proportions(self);

        let image = Rc::new(HtmlImageElement::new().unwrap());
        let image_clone = image.clone();
        //
        image.set_src(&String::from(path.clone()));
        // some values. This are needed cause the closure requests them
        let (dx, dy) = ( 
            width / 2.0,
            height / 2.0,
        );

        // Esperar o carregamento da imagem
        let closure = Closure::wrap(Box::new(move || {
            // Translate to the center of where the image will be
            context.translate(dx + width / 2.0, dy + height / 2.0).unwrap();
            // Draw the image centered at (0, 0)
            context.draw_image_with_html_image_element_and_dw_and_dh(
                &image_clone, 
                -width / 2.0, 
                -height / 2.0, 
                width, 
                height
            ).unwrap();
            
        }) as Box<dyn FnMut()>);
        //
        image.set_onload(Some(closure.as_ref().unchecked_ref()));

        self.background = path;
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

    pub fn new_text(&mut self, id: String, x: f64, y: f64, value: String, color: String, font: String, size: Option<f64>) -> Result<Game, JsValue>
    {
        let canvas = (self.get_canvas_context(), self.get_html_element());
        let kind = Kind::Text( Text { value, color, font });
        // create sprite
        let mut sprite = Sprite::new( Sprite {
            kind,
            pos: (x, y),
            size,
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