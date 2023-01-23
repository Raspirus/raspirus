# Custom build guide
This guide will help you in building the project on your own machine.

## Table of contents
- [Introduction](#introduction)
- [Limitations](#limitations)
- [Step-by-step guide](#step-by-step-guide)
  - [1. Download the repository](#1-download-the-repository)
  - [2. Install Rust](#2-install-rust)
  - [3. Install NPM](#3-install-npm)
  - [4. Install Next.js](#4-install-nextjs)
  - [5. Install Tauri](#5-install-tauri)
  - [6. Install project dependencies](#6-install-project-dependencies)
  - [7. Build the project](#7-build-the-project)
- [Conclusion](#conclusion)

## Introduction
For people that just want a working app, they can just head over to the [Release page](github.com/Benji377/Raspirus/releases/latest) 
and download the executable for the correct platform. But if you are on a different Linux distribution, unsupported OS, or just want to
compile the project on your own, this step-by-step guide will guide you.

## Limitations
- Glibc can cause problems on Linux: https://tauri.app/v1/guides/building/linux#limitations
- You need to use 64-bit systems, else the app might crash because it's using memory improvements that only work there
- The app is meant to be run as a "I'm the only app running on this system" app. This is important regarding RAM usage,
because if you have much RAM, it will use much RAM. And if you, for some reason, try to limit the initially available RAM,
the app might crash because it doesn't have the promised amount of RAM. (A future version might have a toggle for this)

## Step-by-step guide
Please read the whole guide once before starting to execute each step!

### 1. Download the repository
This step is very straightforward, just download the whole repository by clicking the green button on the homepage of this repository.
Optionally, you can also download code specific to a Release by visiting the Release page and download the `.zip` file in the assets.
Another thing you might want to do is [clone this repository](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository)

### 2. Install Rust
One of the requirements to compile the project is to have Rust installed. You can check if you have Rust installed on your machine with the command `rustc --version`,
if this command fails, head over to the [Rust website](https://www.rust-lang.org/tools/install) and follow the instructions for your OS.
Also make sure that your Rust installation is up-to-date with the command `rustup update`.

### 3. Install NPM
NPM is needed because the frontend of the app works on JavaScript and is basically a website. To check if you already have Node.js installed, try executing the commands: `node -v` and `npm -v`. If any of them fail, or you find out you have an older version, head over to the [NPM Website](https://docs.npmjs.com/cli/v7/configuring-npm/install) to install the latest version for your OS. If you are using a WSL, [this guide](https://learn.microsoft.com/en-us/windows/dev-environment/javascript/nodejs-on-wsl) might be useful to you.

### 4. Install Next.js
The frontend is built on JavaScript using a well-known framework named [Next.js](https://nextjs.org), it makes website development faster and more efficient. You will need to install this tool too to be able to build the application. But don't worry, you can do this easily with NPM: `npm install next@latest react@latest react-dom@latest eslint-config-next@latest`. This will install Next.js, React (which Next.js is based on) and ESLint. You can learn more about the installation process [here](https://beta.nextjs.org/docs/installation).

### 5. Install Tauri
Tauri is the framework that connects the Rust backend with the Next.js frontend. It is an open-source project made by very friendly and welcoming people. Unfortunately,
installing Tauri is not as straightforward as other processes. It is very OS dependent, and you will therefore make sure that you meet the [Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) before you start. Afterward, you can install Tauri suing cargo: `cargo install tauri-cli`. You could also use NPM to install it, but we will mainly work with cargo in this short guide. Check out [their FAQ section](https://tauri.app/v1/guides/faq#node-or-cargo) to learn about why NPM might be better for you.

### 6. Install project dependencies
Firstly, we will install the node modules. To do this, head over to the directory that contains all the Raspirus code. Open the `app` directory, open a terminal in this location and execute the command: `npm install`. This might take a while, but it will download all the necessary modules.
**Warning!:** On WSL you might get an `OpenSSL is missing` error, to fix this you need to edit the file `app/src-tauri/Cargo.toml` and add the following line:
`openssl-sys = {version = "0.9.66", features = ["vendored"]}` in the `[dependencies]` section.

### 7. Build the project
Before you can completely build the project, there is one more thing you might want to check. To make sure that the Rust part of the project works fine, open the
folder `app/src-tauri/` and execute the command `cargo build`. If this command succeeds, you can go back to the `app/` directory. If this command fails, please [open an issue](https://github.com/Benji377/Raspirus/issues/new) on this repository with as much information about the error as possible.
If everything went well, you are now in the `app` directory, and you can safely execute the command `cargo tauri build`. This command will build the entire application and display a path at the end of the process showing you where the executable is located. By default, you should be able to find it in the `app\src-tauri\target\release` folder.


## Conclusion
This application is basically a website attached to some Rust code and packaged with the Tauri framework. It will therefore need a graphical overlay to start and display the website. This project is in constant development and therefore, if you find anything unusual, have some good ideas or find some errors, don't be afraid to open an issue on this repository and I will be happy to help you out.
