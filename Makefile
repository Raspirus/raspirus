SHELL := /bin/bash
COLOR := \033[38;2;255;51;102m
TEXT  := \033[38;2;53;192;145m
RESET := \033[0;39m\n

.PHONY: install build test

install:
	@clear
	@printf "$(COLOR)██████╗  █████╗ ███████╗██████╗ ██╗██████╗ ██╗   ██╗███████╗$(RESET)"
	@printf "$(COLOR)██╔══██╗██╔══██╗██╔════╝██╔══██╗██║██╔══██╗██║   ██║██╔════╝$(RESET)"
	@printf "$(COLOR)██████╔╝███████║███████╗██████╔╝██║██████╔╝██║   ██║███████╗$(RESET)"
	@printf "$(COLOR)██╔══██╗██╔══██║╚════██║██╔═══╝ ██║██╔══██╗██║   ██║╚════██║$(RESET)"
	@printf "$(COLOR)██║  ██║██║  ██║███████║██║     ██║██║  ██║╚██████╔╝███████║$(RESET)"
	@printf "$(COLOR)╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝     ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝$(RESET)"
	@printf "$(TEXT)🚀 Starting installation...$(RESET)"
	@sleep 3
	@printf "$(TEXT)🌑 >>>> Updating system and removing temps$(RESET)"
	sudo apt-get update && apt-get upgrade -y
	sudo apt-get autoremove -y
	@printf "$(TEXT)🌒 >>>> Installing curl$(RESET)"
	sudo apt-get -y install curl
	@printf "$(TEXT)🌓 >>>> Installing Rust for Linux$(RESET)"
	curl https://sh.rustup.rs -sSf | sh -s -- -y
	export PATH="$HOME/.cargo/bin:$PATH"
	-source "$HOME/.cargo/env"
	@printf "$(TEXT)🌔 >>>> Installing Nodejs$(RESET)"
	curl -fsSL https://deb.nodesource.com/setup_lts.x | bash
	sudo apt-get install -y nodejs
	@printf "$(TEXT)🌕 >>>> Installing Nextjs and EsLint$(RESET)"
	npm install next@latest react@latest react-dom@latest eslint-config-next@latest
	@printf "$(TEXT)🌖 >>>> Installing Tauri deps$(RESET)"
	sudo apt-get install -y libwebkit2gtk-4.0-dev build-essential wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
	@printf "$(TEXT)🌗 >>>> Installing Tauri$(RESET)"
	cargo install tauri-cli
	@printf "$(TEXT)🌘 >>>> Installing npm deps$(RESET)"
	npm install
	@printf "$(TEXT)🎉 >>>> Done!$(RESET)"

build: install
	@clear
	@printf "$(TEXT)>>>> Building release$(RESET)"
	cargo tauri build
	@printf "$(TEXT)>>>> Done!$(RESET)"

test: install
	@printf "$(TEXT)>>>> Executing cargo tests$(RESET)"
	cd /src-tauri/ && \
	cargo test
