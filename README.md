[![Rust Build & Test](https://github.com/Benji377/Raspirus/actions/workflows/rust.yml/badge.svg)](https://github.com/Benji377/Raspirus/actions/workflows/rust.yml)
# :rocket: Raspirus
![DALL·E 2022-12-22 17 39 15 - an angry raspberry killing bad green viruses](https://user-images.githubusercontent.com/50681275/209184400-ede538b0-ac56-41d2-aaaf-dda0fe93bc5d.png)

## Introduction
Sometimes you have a computer that is not connected to the internet and has no antivirus on it. The computer gets used by a lot of people that upload and download files using their personal USB stick. This bothers you, and it bothered me too. That's where Raspirus comes into play, its an application amde to be used on the Raspberry Pi, but it works on Windows and other Linux systems too. Raspirus will scan all files on your USB key and tell you if it finds anything suspicious. Its not a full-fletched antivirus, as that would consume too much RAM for a normal Raspberry Pi 3 for example. The app simply builds the hash of each file and checks if it finds that hash in a signatures database.

## Installation
Please follow [this guide](https://github.com/Raspirus/docs) to build the app on your own machine. Or if you are on Windows, Ubuntu or MacOS, you can try to use the pre-built installers in the [Releases page](https://github.com/Raspirus/Raspirus/releases/latest). If you encounter nay difficulties while following the instructions, please open an issue and we will be happy to help you out.

## Questions
For any questions, check out the [FAQ docs](https://github.com/Raspirus/docs) and see if you can find an answer to your problem. If not, please open an Issue on this repository.

## Frameworks and Tools used:
- NPM: https://www.npmjs.com
- NextJS: https://nextjs.org
- Rust: https://www.rust-lang.org
- TAURI: https://tauri.app/v1/guides/getting-started/setup/next-js
- SweetAlertv2: https://sweetalert2.github.io
