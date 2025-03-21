install:
	@echo ">>>> Starting installation..."

	@echo ">>>> Installing winget"
	powershell -ExecutionPolicy Bypass -File "build_utils\windows\1setup_winget.ps1"

	@echo ">>>> Installing dependencies"
	start /wait cmd /c "build_utils\windows\2setup_dependencies.bat"

	@echo ">>>> Installing rust"
	start /wait cmd /c "build_utils\windows\3setup_rust.bat"

	@echo ">>>> Installing pipx"
	start /wait cmd /c "build_utils\windows\4setup_pipx.bat"

	@echo ">>>> Installing gvsbuild"
	start /wait cmd /c "build_utils\windows\5setup_gvsbuild.bat"

	@echo ">>>> Building gtk4"
	start /wait cmd /c "build_utils\windows\6build_gtk.bat"

	@echo ">>>> Setting up environment variables"
	start /wait cmd /c "build_utils\windows\7setup_env.bat"

	@echo ">>>> Cleanup"
	rmdir /s /q "C:\Python313"

	@echo ">>>> Done! You should reopen your terminal for changes to apply"

build:
	@echo ">>>> Building release"
	cargo build --release --verbose --no-default-features
	@echo ">>>> Done!"

run:
	@echo ">>>> Running release"
	cargo run --release --verbose --no-default-features
	@echo ">>>> Done!"

test:
	@echo ">>>> Executing cargo tests"
	cargo test
	@echo ">>>> Done!"

check:
	@echo ">>>> Checking codebase"
	cargo clippy --all-features
	@echo ">>>> Done!"

clean:
	@echo ">>>> Cleaning cwd"
	cargo clean -v -v
	@echo ">>>> Done!"

docs:
	@echo ">>>> Generating docs"
	cargo doc --no-deps --workspace --open
	@echo ">>>> Done!"
