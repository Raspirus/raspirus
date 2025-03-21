# install vclibs
Add-AppxPackage 'https://aka.ms/Microsoft.VCLibs.x64.14.00.Desktop.appx'
# install dependencies
Invoke-WebRequest -Uri https://github.com/microsoft/winget-cli/releases/latest/download/DesktopAppInstaller_Dependencies.zip -OutFile .\DesktopAppInstaller_Dependencies.zip
Expand-Archive .\DesktopAppInstaller_Dependencies.zip
Add-AppxPackage .\DesktopAppInstaller_Dependencies\x64\*
# install winget
Add-AppxPackage 'https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle'
