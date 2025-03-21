# install vclibs
Add-AppxPackage 'https://aka.ms/Microsoft.VCLibs.x64.14.00.Desktop.appx'
# install dependencies
Invoke-WebRequest -Uri https://github.com/microsoft/winget-cli/releases/latest/download/DesktopAppInstaller_Dependencies.zip -OutFile .\DesktopAppInstaller_Dependencies.zip
Expand-Archive .\DesktopAppInstaller_Dependencies.zip
Add-AppxPackage .\DesktopAppInstaller_Dependencies\x64\*
# install winget
Invoke-WebRequest -Uri 'https://github.com/microsoft/winget-cli/releases/latest/download/4df037184d634a28b13051a797a25a16_License1.xml' -OutFile winget_license.xml -UseBasicParsing
Invoke-WebRequest -Uri 'https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle' -OutFile winget.appx -UseBasicParsing
Add-AppxProvisionedPackage -Online -PackagePath winget.appx -LicensePath winget_license.xml || Add-AppxPackage winget.appx
