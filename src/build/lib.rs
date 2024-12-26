use js_sys;
use std::{
    collections::HashMap,
    rc::Rc,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    CanvasRenderingContext2d,
    Document,
    HtmlCanvasElement,
    HtmlElement,
    HtmlImageElement,
    Window,
};


mod values;
use values::*;

mod sprites;
use sprites::Sprite;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Game
{
    // fps
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

// Todo: create a different list containing only the sprites that are being rendered and exclude the ones that aren't visible

#[wasm_bindgen]
impl Game
{
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self
    {
        let expect_window = "No global window".to_string();
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
        if self.html_element.is_some()
        {
            return Ok(self.to_owned());
        }
    
        let canvas = self.document.create_element("canvas")?;
        self.body.append_child(&canvas)?;
    
        let element: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| JsValue::from_str("Failed to cast canvas"))?;
    
        self.html_element = Some(element.clone());
    
        let context = element.get_context("2d")
            .map_err(|_| JsValue::from_str("Failed to get context"))?
            .ok_or_else(|| JsValue::from_str("Context is null"))?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .map_err(|_| JsValue::from_str("Failed to cast context"))?;
    
        self.canvas_context = Some(context);
    
        Ok(self.to_owned())
    }

    pub async fn update(&mut self) -> Result<(), JsValue>
    {
        // set the bg color
        self.get_canvas_context().save();
        // background appears in the first layer 
        Self::set_bg_color(&mut self.clone(), self.background.clone())?;
        // redraw sprites, sprites appears in the second layer
        let v = self.data.clone().into_values().collect();
        Self::reload_sprites(self, v).await?;
        //
        Ok(())
    }

    pub async fn reload_sprites(&mut self, vec: Vec<Sprite>) -> Result<(), JsValue>
    {
        if vec.is_empty()
        {
            return Ok(());
        }
    
        // Process each sprite asynchronously
        for sprite in vec
        {
            let mut sprite = Sprite::new(sprite);
            sprite.render()?;
        }
    
        Ok(())
    }

    pub async fn force_update(&mut self) -> Result<Game, JsValue>
    {
        Self::update(&mut self.clone()).await?;
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

    pub async fn resize_canvas(&mut self) -> Result<Game, JsValue>
    {
        let canvas = Self::get_html_element(self);
        // Get the window proportions
        let (window_width, window_height) = Self::get_window_proportions(self);
    
        // Set the desired aspect ratio
        let aspect_ratio = 16.0 / 9.0;
        let (new_width, new_height);
    
        // Calculate new dimensions while maintaining aspect ratio
        if window_width / window_height > aspect_ratio
        {
            new_width = (window_height * aspect_ratio).min(window_width);
            new_height = window_height;
        }
        else
        {
            new_width = window_width;
            new_height = (window_width / aspect_ratio).min(window_height);
        }
        
        // Resize the canvas
        canvas.set_width(new_width as u32);
        canvas.set_height(new_height as u32);
    
        // Get the 2D context of the canvas
        let context = canvas.get_context("2d")?.unwrap();
        let context_2d = context.dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    
        // Fill the canvas background to avoid white bands
        context_2d.set_fill_style(&JsValue::from_str("black")); // In the future that color will probably have to change...
        context_2d.fill_rect(0.0, 0.0, new_width, new_height);
    
        // Update game after resizing
        Self::update(&mut self.clone()).await?; // Adicione .await aqui
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

        Ok(self.to_owned())
    }

    pub async fn set_bg_image(&mut self, path: String) -> Result<Game, JsValue>
    {
        let context = Self::get_canvas_context(self);
        let (width, height) = Self::get_window_proportions(self);
    
        // Create the image
        let image = Rc::new(HtmlImageElement::new().unwrap());
        image.set_src(&path);

        let onload_promise = js_sys::Promise::new(&mut |resolve, reject| {
            let closure = Closure::wrap(Box::new(move || {
                resolve.call0(&JsValue::NULL).unwrap();
            }) as Box<dyn FnMut()>);
    
            image.set_onload(Some(closure.as_ref().unchecked_ref()));
            closure.forget(); // The promise will be resolved when the onload event is fired
        });

        let future = JsFuture::from(onload_promise);
    
        // Wait for the image to load
        future.await.unwrap();
    
        // Once loaded, draw the image  
        context.translate(width / 2.0, height / 2.0).unwrap();
        context.draw_image_with_html_image_element_and_dw_and_dh(
            &image, 
            -width / 2.0, 
            -height / 2.0, 
            width, 
            height
        ).unwrap();
    
        // Update the background
        self.background = path;
    
        Ok(self.to_owned())
    }    

    pub fn new_image(&mut self, id: String, x: f64, y: f64, path: String, size: Option<f64>, angle: Option<f64>) -> Result<Game, JsValue>
    {
        let canvas = (self.get_canvas_context(), self.get_html_element());
        let kind = Kind::Image(Image { path });

        // create sprite
        let mut sprite = Sprite::new( Sprite {
            kind,
            pos: (x, y),
            size,
            angle,
            canvas
        });
        sprite.render()?;
        self.data.insert(id, sprite);
        //
        Ok(self.to_owned())
    }

    pub async fn new_text(&mut self, id: String, x: f64, y: f64, value: String, color: String, font: String, size: Option<f64>) -> Result<Game, JsValue>
    {
        let canvas = (self.get_canvas_context(), self.get_html_element());
        let font_with_size = format!("{}px {}", size.unwrap_or(16.0), font.clone());

        // Load the font using a Fonts API
        // It allows you to load fonts asynchronously and check if they are available
        let font_face = format!("{} {}", size.unwrap_or(16.0), font);
        let load_promise = js_sys::Promise::resolve(&JsValue::from_str(&font_face));

        // Create a closure for then
        let resolve_closure = Closure::wrap(Box::new(move |_: JsValue| {
            // Additional logic if necessary - for more customizable texts IDK
            // Don't return anything, just execute the logic
        }) as Box<dyn FnMut(JsValue)>);

        // Create a closure for catch
        let reject_closure = Closure::wrap(Box::new(move |err: JsValue| {
            // You can add logic to handle errors if necessary
            // Don't return anything, just execute the logic
        }) as Box<dyn FnMut(JsValue)>);

        // Use closures with promise
        load_promise.then(&resolve_closure).catch(&reject_closure);

        // Wait for the font to load
        let future = JsFuture::from(load_promise);
        future.await?;

        // Set the font style
        let context = self.get_canvas_context();
        context.set_font(&font_with_size);
        context.set_fill_style(&color.clone().into());

        let kind = Kind::Text(Text { value: value.clone(), color: color.clone(), font: font_with_size.clone() });

        let mut sprite = Sprite::new(Sprite {
            kind,
            pos: (x, y),
            size,
            angle: None,
            canvas
        });

        sprite.render()?;
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
        if let Some(estrutura) = self.data.get_mut(name)
        {
            //console::log_1(&JsValue::from_str(estrutura.x.to_string().as_str()));
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