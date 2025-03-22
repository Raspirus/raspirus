%USERPROFILE%\.cargo\bin\rustup default stable-gnu
powershell -Command "[System.Environment]::SetEnvironmentVariable("Path", $env:Path + ";$env:USERPROFILE\.cargo\bin", [System.EnvironmentVariableTarget]::User)"
