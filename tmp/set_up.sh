# rust dependencies
echo "Installing rust dependencies..."
cargo install wasm-pack                 # wasm-pack is a tool for compiling rust to wasm
cargo install wasm-opt                  # wasm-opt is a tool for optimizing wasm files
echo "Done!"

# pkl extension download
echo "download pickle extension..."     # pickle is a programming language for configuration files developed by apple
wget "https://github.com/apple/pkl-vscode/releases/download/0.17.0/pkl-vscode-0.17.0.vsix" -O /tmp/pkl.vsix
# pkl extension install
echo "installing pickle extension..."
code --install-extension /tmp/pkl.vsix  # install the extension in vscode
# pkl extension cleanup
rm /tmp/pkl.vsix
echo "Done!"

echo "configuring some stuff..."
# set this repo as safe
git config --global --add safe.directory /workspaces/chrome-dino-game
# set the run.sh as a safe file
chmod +x run.sh
# configure git to not rebase
git config pull.rebase false
echo "Done!"