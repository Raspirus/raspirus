SHELL := /bin/bash
COLOR := \033[38;2;255;51;102m
TEXT  := \033[38;2;53;192;145m
RESET := \033[0;39m\n

PKG_MANAGER := $(shell \
		if command -v apt >/dev/null 2>&1; then echo "Detected apt"; \
    elif command -v pacman >/dev/null 2>&1; then echo "Detected pacman"; \
    elif command -v dnf >/dev/null 2>&1; then echo "Detected dnf"; \
    else echo "unknown";fi)

install:
	@clear
	@printf "$(COLOR)██████╗  █████╗ ███████╗██████╗ ██╗██████╗ ██╗   ██╗███████╗$(RESET)"
	@printf "$(COLOR)██╔══██╗██╔══██╗██╔════╝██╔══██╗██║██╔══██╗██║   ██║██╔════╝$(RESET)"
	@printf "$(COLOR)██████╔╝███████║███████╗██████╔╝██║██████╔╝██║   ██║███████╗$(RESET)"
	@printf "$(COLOR)██╔══██╗██╔══██║╚════██║██╔═══╝ ██║██╔══██╗██║   ██║╚════██║$(RESET)"
	@printf "$(COLOR)██║  ██║██║  ██║███████║██║     ██║██║  ██║╚██████╔╝███████║$(RESET)"
	@printf "$(COLOR)╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝     ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝$(RESET)"
	@printf "$(TEXT)🚀 Starting installation...$(RESET)"
	
	@printf "$(TEXT)🌑 >>>> Installing dependencies$(RESET)"
	@if command -v apt >/dev/null 2>&1; then sudo apt update && sudo apt install -y curl libgtk-4-dev build-essential libssl-dev; \
	elif command -v pacman >/dev/null 2>&1; then sudo pacman -Syu --noconfirm gtk4 base-devel openssl; \
	elif command -v dnf >/dev/null 2>&1; then sudo dnf install -y curl gtk4-devel @development-tools openssl-devel; \
	elif command -v brew >/dev/null 2>&1; then sudo brew install curl gtk4 openssl; \
	else echo "No supported package manager found!"; exit 1; fi
	
	@printf "$(TEXT)🌓 >>>> Installing Rust for Unix$(RESET)"
	#sudo curl https://sh.rustup.rs -sSf | sh -s -- -y

	@printf "$(TEXT)🌓 >>>> Adding variables to bashrc$(RESET)"
	#@if ! grep -q "export PATH=\$$HOME/.cargo/bin:\$$PATH" $(HOME)/.bashrc; then \
	#  echo "export PATH=\$$HOME/.cargo/bin:\$$PATH" >> $(HOME)/.bashrc; \
	#fi

	#@if ! grep -q "source \$$HOME/.cargo/env" $(HOME)/.bashrc; then \
	#  echo "source \$$HOME/.cargo/env" >> $(HOME)/.bashrc; \
	#  echo "Appended source to ~/.bashrc"; \
	#fi

	@printf "$(TEXT)🌕 >>>> Checking project$(RESET)"
		cd ../src
		cargo check
	@printf "$(TEXT)🎉 >>>> Done!$(RESET)"

build:
	@clear
	@printf "$(TEXT)>>>> Building release$(RESET)"
	cargo build --release --verbose --no-default-features
	@printf "$(TEXT)>>>> Done!$(RESET)"

run:
	@clear
	@printf "$(TEXT)>>>> Running release$(RESET)"
	cargo run --release --verbose --no-default-features
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
