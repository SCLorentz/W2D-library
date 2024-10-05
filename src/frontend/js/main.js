import init, { Game } from "/script/chrome_dino_game.js";
await init()
//
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
    "",
    "500.0",
    "100.0",
    "https://play-lh.googleusercontent.com/iiIJq5JmLFYNI1bVz4IBHyoXs508JcEzHhOgau69bnveF9Wat51-ax9LMPVOlneKwqg",
    "200.0",    // size
    "180.0"     // angle
)

window.addEventListener("resize", () => obj.resize_canvas())