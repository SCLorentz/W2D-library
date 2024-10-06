import init, { Game } from "/script/chrome_dino_game.js";
await init()

const obj = new Game();
//
obj.inicialize();
obj.resize_canvas();
obj.set_bg_color("black");
// render elements
obj.create_sprite(
    "buzz",
    "500.0",
    "250.0",
    "https://i.pinimg.com/474x/be/14/4b/be144b24d59ecc058888bc1da2ef8ef4.jpg",
    "250.0",
    "45.0"
)
obj.create_sprite(
    "dino",
    "500.0",
    "100.0",
    "https://play-lh.googleusercontent.com/iiIJq5JmLFYNI1bVz4IBHyoXs508JcEzHhOgau69bnveF9Wat51-ax9LMPVOlneKwqg",
    "200.0",    // size
    "180.0"     // angle
)

// for now, texts aren't beeing redrawed when the canvas reload, I could resolve this creating a new list and fn separated for them, but I want to merge it with the 'sprite' one
obj.draw_text("myHelloWorld", "Hello world", "600.0", "400.0", "100.0")

window.addEventListener("click", () => {
    obj.update_sprite_value("buzz", "650.0");
    obj.force_update();
})

window.addEventListener("resize", () => obj.resize_canvas())