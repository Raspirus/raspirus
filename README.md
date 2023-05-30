[![Rust Build & Test](https://github.com/Benji377/Raspirus/actions/workflows/rust.yml/badge.svg)](https://github.com/Benji377/Raspirus/actions/workflows/rust.yml)
[![Codecov](https://codecov.io/gh/Raspirus/Raspirus/branch/main/graph/badge.svg?token=0GFFWY7YBP)](https://codecov.io/gh/Raspirus/Raspirus)
![GitHub downloads](https://img.shields.io/github/downloads/Raspirus/Raspirus/total?label=Downloads)
![Lines of code](https://img.shields.io/tokei/lines/github/Raspirus/Raspirus?label=Lines%20of%20code)
# :rocket: Raspirus
![banner_logo](https://user-images.githubusercontent.com/50681275/223684389-ed0f104f-c183-4223-9723-c268e7cc5268.png)

## Introduction
Sometimes you work on a computer that is not connected to the internet. It even has no antivirus installed and you guess you are safe. But it could be used by a lot of people that upload and download files using their personal USB sticks. This bothers you, and it bothered me too. That's where Raspirus enters the game, an application to be used on the Raspberry Pi, but also for Windows and other Linux systems. Raspirus will scan all files on your USB key and warn you in case of possible threats. It's far from being a fully-fledged antivirus, as that would consume way too much RAM on a normal Raspberry Pi 3. It just computes the hash of each file and checks for a match in a signature database.

## Contributors
<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## Future enhancements:
- Add webpage that explains each type of possible malware to user
- Ship with OS directly for Raspberry Pi installation
