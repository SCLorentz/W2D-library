pub fn animate(mut self) {
    let now = Instant::now();
    let elapsed = now.duration_since(self.last_update);
    let desired_frame_time = Duration::from_secs(1) / self.fps;
    let canvas = self.clone().html_element.unwrap();

    if elapsed >= desired_frame_time {
        self.last_update = now;

        // Limpar o canvas e desenhar os elementos
        self.clone().canvas_context.unwrap().clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        Self::update(&mut self);

        self.clone().window.request_animation_frame(Closure::wrap(Box::new(move || self.animate()) as Box<dyn FnMut()>).as_ref().unchecked_ref());
    } else {
        self.clone().window.request_animation_frame(Closure::wrap(Box::new(move || self.animate()) as Box<dyn FnMut()>).as_ref().unchecked_ref());
    }
}