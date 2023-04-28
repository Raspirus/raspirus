# ifeq ($(OS),Windows_NT) 
#    detected_OS := Windows
#else
#    detected_OS := $(shell sh -c 'uname 2>/dev/null || echo Unknown')
#endif

# THIS Makefile IS PROBABLY NOT WORKING. 
# IT HAS NOT BEEN TESTED YET. 
# ITS ONLY PURPOSE IS TO GROUP TOGETHER THE INSTALL COMMANDS FOR DEBIAN SYSTEMS

install:
	$(info Updating system and installing curl)
	sudo apt update && sudo apt upgrade -y 
	sudo apt install curl
	$(info Installing Rust for Linux)
	curl https://sh.rustup.rs -sSf | sh
	export PATH="$HOME/.cargo/bin:$PATH"
	$(info Installing Nodejs)
	curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash
	sudo apt install -y nodejs
	$(info Installing Nextjs and EsLint)
	npm install next@latest react@latest react-dom@latest eslint-config-next@latest
	$(info Installing Tauri deps)
	sudo apt install libwebkit2gtk-4.0-dev build-essential wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
	$(info Installing Tauri)
	cargo install tauri-cli
	$(info Installing npm deps)
	npm install
	$(info Building release)
	cargo tauri build
	$(info Done!)
