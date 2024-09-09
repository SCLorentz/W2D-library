# rust wasm build
echo "Building rust..."
wasm-pack build --target web --out-dir ./src/frontend/script
#wasm-opt -Os ./public/script/chrome_dino_game_bg.wasm -o ./public/script/chrome_dino_game_bg.wasm
echo "Done!"

cd ./src

# go build
echo "Building server..."
go mod tidy
GOOS=linux GOARCH=amd64 go build -o dino main.go

# server startup
echo "Starting server..."
./dino