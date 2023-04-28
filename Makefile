# ifeq ($(OS),Windows_NT) 
#    detected_OS := Windows
#else
#    detected_OS := $(shell sh -c 'uname 2>/dev/null || echo Unknown')
#endif

# THIS Makefile IS PROBABLY NOT WORKING. 
# IT HAS NOT BEEN TESTED YET. 
# ITS ONLY PURPOSE IS TO GROUP TOGETHER THE INSTALL COMMANDS FOR DEBIAN SYSTEMS

install:
	@echo "Updating system and installing curl"
	sudo apt update && sudo apt upgrade -y 
	sudo apt install curl
	@echo "Installing Rust for Linux"
	curl https://sh.rustup.rs -sSf | sh
	source "$HOME/.cargo/env"
	@echo "Installing Nodejs"
	curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash
	sudo apt install -y nodejs
	@echo "Installing Nextjs and EsLint"
	npm install next@latest react@latest react-dom@latest eslint-config-next@latest
	@echo "Installing Tauri deps"
	sudo apt install libwebkit2gtk-4.0-dev build-essential wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
	@echo "Installing Tauri"
	cargo install tauri-cli
	@echo "Installing npm deps"
	npm install
	@echo "Building release"
	cargo tauri build
	@echo "Done!"
