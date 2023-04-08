[![Rust Build & Test](https://github.com/Benji377/Raspirus/actions/workflows/rust.yml/badge.svg)](https://github.com/Benji377/Raspirus/actions/workflows/rust.yml)
# :rocket: Raspirus
![banner_logo](https://user-images.githubusercontent.com/50681275/223684389-ed0f104f-c183-4223-9723-c268e7cc5268.png)

## Introduction
Sometimes you work on a computer that is not connected to the internet. It even has no antivirus installed and you guess you are safe. But it could be used by a lot of people that upload and download files using their personal USB sticks. This bothers you, and it bothered me too. That's where Raspirus enters the game, an application to be used on the Raspberry Pi, but also for Windows and other Linux systems. Raspirus will scan all files on your USB key and warn you in case of possible threats. It's far from being a fully-fledged antivirus, as that would consume way too much RAM on a normal Raspberry Pi 3. It just computes the hash of each file and checks for a match in a signature database.

## Installation
Please follow [this guide](https://github.com/Raspirus/docs) to build the app on your own machine. Or if you run Windows, Ubuntu or MacOS, you can try to use the pre-built installers on the [Releases page](https://github.com/Raspirus/Raspirus/releases/latest). If you encounter any difficulties following the instructions, please open an issue and we will be happy to get back to you.

## Questions
For any questions, please first check out the [FAQ docs](https://github.com/Raspirus/docs) and see if you can find an answer to your problem. If not, please open an issue on this repository.

## Frameworks and Tools used:
- NPM: https://www.npmjs.com
- NextJS: https://nextjs.org
- Rust: https://www.rust-lang.org
- TAURI: https://tauri.app/v1/guides/getting-started/setup/next-js
- SweetAlertv2: https://sweetalert2.github.io

## More stuff:
- Tests: https://jonaskruckenberg.github.io/tauri-docs-wip/development/testing.html
- Translate: https://stefaniq.com/translate-your-next-js-project-with-next-translate/