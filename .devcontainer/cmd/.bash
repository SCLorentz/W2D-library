# Alias for ls -la
alias ll='ls -la'

# Change prompt color (for Bash, using ANSI colors)
PS1='\[\e[38;5;41m\]\w\[\e[0m\]\\$ '

run() {
    # rust wasm build
    echo "Building rust..."
    wasm-pack build --target web --out-dir ./src/frontend/script
    #wasm-opt -Os ./public/script/chrome_dino_game_bg.wasm -o ./public/script/chrome_dino_game_bg.wasm
    echo "Done!"

    cd ./src/backend/selfhost

    # go build
    echo "Building server..."
    go mod tidy
    GOOS=linux GOARCH=amd64 go build -o dino main.go

    # server startup
    echo "Starting server..."
    ./dino
}