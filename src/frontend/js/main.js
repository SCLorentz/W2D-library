import init, { Game } from "/script/plain_render_lib.js";
await init()

const obj = new Game();
//
obj.inicialize();
obj.resize_canvas();
obj.set_bg_color("black");
//obj.set_bg_image("https://i.pinimg.com/originals/be/14/4b/be144b24d59ecc058888bc1da2ef8ef4.jpg");  < -- not working
// render elements
obj.new_image(
    "buzz",
    "500.0",
    "250.0",
    "https://i.pinimg.com/474x/be/14/4b/be144b24d59ecc058888bc1da2ef8ef4.jpg",
    "250.0",
    "45.0"
)
obj.new_image(
    "dino",     // id
    "500.0",    // x
    "100.0",    // y
    "../static/assets/base/player.png",  // path
    "200.0",    // size
    "180.0"     // angle
)

// Todo: Add a function that makes pixelated render less blurry

obj.new_text("hw", "600.0", "800.0", "Hello world", "red", "Roboto", "50.0");
//id: String, x: f64, y: f64, value: String, color: String, font: String

//console.log(obj.get_sprite_by_id("dino")) < -- not working

window.addEventListener("click", () => {
    obj.update_sprite_value("buzz", "650.0", "250.0");
    obj.force_update();
})

window.addEventListener("resize", () => obj.resize_canvas());