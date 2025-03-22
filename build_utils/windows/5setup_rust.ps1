%USERPROFILE%\.cargo\bin\rustup default stable-gnu
[System.Environment]::SetEnvironmentVariable("Path", $env:Path + ";$env:USERPROFILE\.cargo\bin", [System.EnvironmentVariableTarget]::User)
