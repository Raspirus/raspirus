install:
	@echo ">>>> Starting installation..."

	@echo ">>>> Installing winget"
	powershell -ExecutionPolicy Bypass -File "build_utils\windows\1setup_winget.ps1"

	@echo ">>>> Installing dependencies"
	start /wait cmd /c "build_utils\windows\2setup_dependencies.bat"

	@echo ">>>> Setting up msys"
	start /wait cmd /c "build_utils\windows\3setup_msys.bat"

	@echo ">>>> Updating path"
	start /wait cmd /c "build_utils\windows\4update_path.bat"

	@echo ">>>> Setting up rust"
	start /wait cmd /c "build_utils\windows\5setup_rust.ps1"

	@echo ">>>> Done! You should reopen your terminal for changes to apply"

build:
	@echo ">>>> Building release"
	%USERPROFILE%\.cargo\bin\cargo build --release --verbose --no-default-features
	@echo ">>>> Done!"

run:
	@echo ">>>> Running release"
	%USERPROFILE%\.cargo\bin\cargo run --release --verbose --no-default-features
	@echo ">>>> Done!"

test:
	@echo ">>>> Executing cargo tests"
	%USERPROFILE%\.cargo\bin\cargo test
	@echo ">>>> Done!"

check:
	@echo ">>>> Checking codebase"
	%USERPROFILE%\.cargo\bin\cargo clippy --all-features
	@echo ">>>> Done!"

clean:
	@echo ">>>> Cleaning cwd"
	%USERPROFILE%\.cargo\bin\cargo clean -v -v
	@echo ">>>> Done!"

docs:
	@echo ">>>> Generating docs"
	%USERPROFILE%\.cargo\bin\cargo doc --no-deps --workspace --open
	@echo ">>>> Done!"
