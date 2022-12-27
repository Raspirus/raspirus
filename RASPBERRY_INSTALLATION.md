# How to cross-compile Raspirus for the Raspberry Pi
WARNING! Ubuntu 20.04 WSL has issues with the webkit dependencies and should therefore not be used!

## Prerequisites:
- A Linux machine that supports GLBIBC of the Raspberry Oi you want to use

## Initial setup
Here we are going to install Rust, NodeJS, Tauri and their dependencies. These are important to build the project later on.

1. Update the distro: \
  `sudo apt-get update && sudo apt-get upgrade -y`

2. Add Tauri [system requirements](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux):
  ```
  sudo apt install libwebkit2gtk-4.0-dev \
      build-essential \
      curl \
      wget \
      libssl-dev \
      libgtk-3-dev \
      libayatana-appindicator3-dev \
      librsvg2-dev
  ```

3. Install Rust: \
  `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh` \
  And check that it's up-to-date: \
  `rustup update`

4. Install [Tauri using the Rust package manager Cargo](https://tauri.app/v1/guides/getting-started/setup/next-js/#create-the-rust-project): \
  `cargo install tauri-cli`

5. Install NodeJS using your prefered method, in my case I'm using the [guide for Windows WSL](https://learn.microsoft.com/en-us/windows/dev-environment/javascript/nodejs-on-wsl#install-nvm-nodejs-and-npm):
	- Install NVM: \
	`curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh | bash`

	- Install NodeJS using NVM: \
	`nvm install --lts`

6. Remove possible node_modules, if there are any (In the project folder): \
  `sudo rm -rf node_modules`

7. Install the new ones: \
  `npm install`

8. Important! On Linux WSL you need to add a dependency to the Cargo.toml file. You can find the file in <project-root>/src-tauri/Cargo.toml: \
  Add this line in the [dependencies] section: \
  `openssl-sys = {version = "0.9.66", features = ["vendored"]}`

## Cross-compiling:
How to compile for Raspberry Pi on x86_64 linux hosts

1. install Rust target: \
  `rustup target add armv7-unknown-linux-gnueabihf`

2. install a linker for arm: \
  `sudo apt install gcc-arm-linux-gnueabihf`

3. Open or create the file <projectroot>/.cargo/config.toml and add the following:
  ```
  [target.armv7-unknown-linux-gnueabihf]
  linker = "arm-linux-gnueabihf-gcc"
  ```

4. Enable armhf in the package manager: \
  `sudo dpkg --add-architecture armhf`

5. Add deb sources by opening the file /etc/apt/sources-list and adding (Not all might be necessary and some will generate errors that can safely be ignored):
  ```
  deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports jammy main restricted
  deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports jammy-updates main restricted
  deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports jammy universe
  deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports jammy-updates universe
  deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports jammy multiverse
  deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports jammy-updates multiverse
  deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports jammy-backports main restricted universe multiverse
  deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports jammy-security main restricted
  deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports jammy-security universe
  deb [arch=armhf] http://ports.ubuntu.com/ubuntu-ports jammy-security multiverse
  ```

6. Check that the armhf architecture is still enabled: \
  `sudo dpkg --add-architecture armhf`

7. Update the new package info: \
  `sudo apt-get update && sudo apt-get upgrade -y`

8. Install webkitgtk: \
  `sudo apt install libwebkit2gtk-4.0-dev:armhf`

9. Tell where pkgconfig can find the libs for that arch: \
  `export PKG_CONFIG_SYSROOT_DIR=/usr/arm-linux-gnueabihf/`

10. Finally build the app: \
  `cargo tauri build --target armv7-unknown-linux-gnueabihf`

## Conclusion
- The static website can be found in the out folder
- Any issues saysing that it couldn't run the project can be ignored, as you probably aren't on a Raspberry Pi and therefore obviously can't run it
