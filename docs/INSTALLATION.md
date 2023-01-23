# Clone and compile
⚠️ WORK IN PROGRESS ⚠️

## Introduction
For people that just want a working app, they can just head over to the [Release page](github.com/Benji377/Raspirus/releases/latest) 
and download the executable for the correct platform. But if you are on a different Linux distribution, unsupported OS, or just want to
compile the project on your own, this step-by-step guide will guide you.

## Limitations
- Glibc can cause problems on Linux: https://tauri.app/v1/guides/building/linux#limitations
- You need to use 64 bit systems, else the app might crash because its using memory improvements that only work there
- The app is meant to be run as a "I'm the only app running on this system" app. This is important regarding RAM usage,
because if you have much RAM, it will use much RAM. And if you, for some reason, try to limit the initially available RAM,
the app might crash because it doesn't have the promised amount of RAM. (A future version might have a toggle for this)

## Step by step guide
Please read the whole guide once before starting to execute each step!

### 1. Downlaod the repository
This step is very straightforward, just download the whole repository by clicking the green button on the homepage of this repository.
Optionally you can also download code spcific to a Release by visiting the Release page and download the `.zip` file in the assets.
Another thing you might want to do is [clone this repository](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository)

### 2. Install Rust
One of the requirements to compile the project is to have Rust installed. You can check if you have Rust installed on your machine with the command `rustc --version`,
if this command fails, head over to the [Rust website](https://www.rust-lang.org/tools/install) and follow the instructions for your OS.
Also make sure that your Rust installation is up-to-date with the command `rustup update`.

### 3. Install NPM
npm is needed because the frontend of the app works on Javascript and is basically a website. To check if you already have Nodejs installed, try executing the commands: `node -v` and `npm -v`. If any of them fail, or you find out you have an older version, head over to the [NPM Website](https://docs.npmjs.com/cli/v7/configuring-npm/install) to install the latest version for your OS. If you are using a WSL, [this guide](https://learn.microsoft.com/en-us/windows/dev-environment/javascript/nodejs-on-wsl) might be useful to you.

### 4. Install Nextjs
The frontend is built on Javascript using a well-known framework named [Nextjs](https://nextjs.org), it makes website development faster and more efficient. You will need to install this tool too to be able to build the application. But don't worry, you can do this easily with NPM: `npm install next@latest react@latest react-dom@latest eslint-config-next@latest`. This will install Nextjs, React (which Nextjs is based on) and Eslint. You can learn more about the installation process [here](https://beta.nextjs.org/docs/installation).

### 5. Install Tauri
Tauri is the framework that connects the Rust backend with the Nextjs frontend. 

6. Install Tauri
7. Install all needed dependencies
8. Check that Rust works by executing `cargo build` inside the `app/src-tauri/` folder
9. Build the project by executing `cargo tauri build` inside the `app/` folder 
10. You should see a line indicating the location of your executable in the terminal
11. Make sure that you have a graphical overlay where the app can run. SSH might not work for this purpose

## Conclusion
