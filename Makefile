SHELL := /bin/bash
.PHONY: install build test
# Tested on WSL: Debian GNU/Linux 11 (bullseye) on Windows 10 x86_64

COLOR := \033[38;2;255;51;102m
TEXT  := \033[38;2;53;192;145m
RESET := \033[0;39m\n

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
	@printf "$(TEXT)1️⃣ >>>> Updating system and removing temps$(RESET)"
	sudo apt update && sudo apt upgrade -y
	sudo apt autoremove -y
	@printf "$(TEXT)2️⃣>>>> Installing curl$(RESET)"
	sudo apt install curl
	@printf "$(TEXT)3️⃣ >>>> Installing Rust for Linux$(RESET)"
	curl https://sh.rustup.rs -sSf | sh -s -- -y
	export PATH="$HOME/.cargo/bin:$PATH"
	-source "$HOME/.cargo/env"
	@printf "$(TEXT)4️⃣ >>>> Installing Nodejs$(RESET)"
	curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash
	sudo apt install -y nodejs
	@printf "$(TEXT)5️⃣ >>>> Installing Nextjs and EsLint$(RESET)"
	npm install next@latest react@latest react-dom@latest eslint-config-next@latest
	@printf "$(TEXT)6️⃣ >>>> Installing Tauri deps$(RESET)"
	sudo apt install -y libwebkit2gtk-4.0-dev build-essential wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
	@printf "$(TEXT)7️⃣ >>>> Installing Tauri$(RESET)"
	cargo install tauri-cli
	@printf "$(TEXT)8️⃣ >>>> Installing npm deps$(RESET)"
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
