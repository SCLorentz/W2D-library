# rust wasm build
echo "Building rust..."
wasm-pack build --target web --out-dir ./public/script
wasm-opt -Os ./public/script/chrome_dino_game_bg.wasm -o ./public/script/chrome_dino_game_bg.wasm
echo "Done!"

# go build
echo "Building server..."
go build -o dino

# server startup
echo "Starting server..."
./dino