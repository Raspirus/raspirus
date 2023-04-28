# Tested on WSL: Debian GNU/Linux 11 (bullseye) on Windows 10 x86_64

install:
	@echo ">>>> Updating system and installing curl"
	sudo apt update && sudo apt upgrade -y 
	sudo apt install curl
	@echo ">>>> Installing Rust for Linux"
	curl https://sh.rustup.rs -sSf | sh -s -- -y
	export PATH="$HOME/.cargo/bin:$PATH"
	@echo ">>>> Installing Nodejs"
	curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash
	sudo apt install -y nodejs
	@echo ">>>> Installing Nextjs and EsLint"
	npm install next@latest react@latest react-dom@latest eslint-config-next@latest
	@echo ">>>> Installing Tauri deps"
	sudo apt install libwebkit2gtk-4.0-dev build-essential wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
	@echo ">>>> Installing Tauri"
	cargo install tauri-cli
	@echo ">>>> Installing npm deps"
	npm install
	@echo ">>>> Done!"

build: install
	@echo ">>>>  Building release"
	cargo tauri build
	@echo ">>>> Done!"

test: install
	@echo ">>>>  Executing cargo tests"
	cd /src-tauri/ && \
	cargo test
