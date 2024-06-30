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
	sudo apt-get update && sudo apt-get upgrade -y
	sudo apt-get autoremove -y
	@printf "$(TEXT)🌒 >>>> Installing curl$(RESET)"
	sudo apt-get -y install curl
	@printf "$(TEXT)🌓 >>>> Installing Rust for Linux$(RESET)"
	sudo curl https://sh.rustup.rs -sSf | sh -s -- -y
	@printf "$(TEXT)🌓 >>>> Adding variables to bashrc$(RESET)"
	@if ! grep -q "export PATH=\$$HOME/.cargo/bin:\$$PATH" $(HOME)/.bashrc; then \
	    echo "export PATH=\$$HOME/.cargo/bin:\$$PATH" >> $(HOME)/.bashrc; \
	fi
	@if ! grep -q "source \$$HOME/.cargo/env" $(HOME)/.bashrc; then \
	    echo "source \$$HOME/.cargo/env" >> $(HOME)/.bashrc; \
	    echo "Appended source to ~/.bashrc"; \
	fi
	@printf "$(TEXT)🌔 >>>> Installing Tauri deps$(RESET)"
	sudo apt-get install -y libwebkit2gtk-4.1-dev \
                              build-essential \
                              curl \
                              wget \
                              file \
                              libxdo-dev \
                              libssl-dev \
                              libayatana-appindicator3-dev \
                              librsvg2-dev
	@printf "$(TEXT)🌕 >>>> Installing Tauri$(RESET)"
	source $(HOME)/.cargo/env && cargo install tauri-cli
	@printf "$(TEXT)🌖 >>>> Adding WASM target$(RESET)"
	rustup target add wasm32-unknown-unknown
	@printf "$(TEXT)🌗 >>>> Installing Trunk$(RESET)"
    source $(HOME)/.cargo/env && cargo install trunk
	@printf "$(TEXT)🌗 >>>> Setting up required folders$(RESET)"
	mkdir -p dist
	@printf "$(TEXT)🌘 >>>> Compiling src-tauri module$(RESET)"
    source $(HOME)/.cargo/env && cargo install --path src-tauri/
	@printf "$(TEXT)🎉 >>>> Done!$(RESET)"

build:
	@clear
	@printf "$(TEXT)>>>> Building release$(RESET)"
	cargo tauri build
	@printf "$(TEXT)>>>> Done!$(RESET)"

test:
	@printf "$(TEXT)>>>> Executing cargo tests$(RESET)"
	cargo test
	@printf "$(TEXT)>>>> Done!$(RESET)"
check:
	@printf "$(TEXT)>>>> Checking codebase$(RESET)"
	cargo clippy --all-features
	@printf "$(TEXT)>>>> Done!$(RESET)"
clean:
	@printf "$(TEXT)>>>> Cleaning cwd$(RESET)"
	cargo clean -v -v
	@printf "$(TEXT)>>>> Done!$(RESET)"
docs:
    @printf "$(TEXT)>>>> Generating docs$(RESET)"
    cargo doc --no-deps --workspace --open
    @printf "$(TEXT)>>>> Done!$(RESET)"
