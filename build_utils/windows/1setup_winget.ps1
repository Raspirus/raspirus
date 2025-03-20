if (-not (Get-Command winget -ErrorAction SilentlyContinue)) {
Add-AppxPackage -Path "build_utils\windows\xaml.appx"
Add-AppxPackage -Path "build_utils\windows\vctools.appx"
$URL = "https://api.github.com/repos/microsoft/winget-cli/releases/latest"
$URL = (Invoke-WebRequest -Uri $URL -UseBasicParsing).Content | ConvertFrom-Json |
        Select-Object -ExpandProperty "assets" |
        Where-Object "browser_download_url" -Match '.msixbundle' |
        Select-Object -ExpandProperty "browser_download_url"
# Remove-Item "build_utils\windows\winget.appx"
Invoke-WebRequest -Uri $URL -OutFile "build_utils\windows\winget.appx" -UseBasicParsing
Add-AppxPackage -Path "build_utils\windows\winget.appx"
}
