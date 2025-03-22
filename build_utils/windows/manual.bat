powershell -ExecutionPolicy Bypass -File "build_utils\windows\1setup_winget.ps1"

@echo ">>>> Installing dependencies"
start /wait cmd /c "build_utils\windows\2setup_dependencies.bat"

@echo ">>>> Setting up msys"
start /wait cmd /c "build_utils\windows\3setup_msys.bat"

@echo ">>>> Updating path"
start /wait cmd /c "build_utils\windows\4update_path.bat"

@echo ">>>> Setting up rust"
start /wait cmd /c "build_utils\windows\5setup_rust.ps1"

