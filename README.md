![Discord](https://img.shields.io/discord/1132753101485514774?logo=discord&label=Join%20the%20Discord)
[![Test](https://github.com/Raspirus/Raspirus/actions/workflows/testproject.yml/badge.svg)](https://github.com/Raspirus/Raspirus/actions/workflows/testproject.yml)
![GitHub downloads](https://img.shields.io/github/downloads/Raspirus/Raspirus/total?label=Downloads)
[![Gmail](https://img.shields.io/badge/Contact_Us-Gmail-red?style=flat&logo=gmail)](mailto:raspirus.dev@gmail.com)

# :rocket: Raspirus
<div align="center">
  <img src="https://raw.githubusercontent.com/raspirus/media/refs/heads/main/logo/logo-final.svg" alt="Logo" style="width: 45%; max-width: 400px; vertical-align: middle; margin-right: 5%;">
  <img src="https://raw.githubusercontent.com/raspirus/media/refs/heads/main/logo/usb-final.svg" alt="USB" style="width: 45%; max-width: 400px; vertical-align: middle;">
</div>


<!-- https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#alerts -->

## Disclaimer

This branch is currently under development and thus subject to heavy change. Expect nothing to be final. If bugs are found, please still report them as solving them before stabilizing this branch will be beneficial.
Progress can be inspected here: [Release Roadmap](https://github.com/orgs/Raspirus/projects/7)

Plans for this update include:
- Functional and useful cli only mode for headless users
- Stable gtk based frontend to avoid issues on non standard systems
- More reliable and faster backend functionality
- Better code organization


## Introduction

**Raspirus: Empowering Your Malware Protection**

Welcome to Raspirus, your lightweight rules-based malware scanner. Originally designed to scan attached USB drives using a Raspberry Pi, Raspirus has evolved into a versatile tool capable of scanning local files and folders as well. Some of its standout features include:

- ~~**Comprehensive File Scans:** Raspirus can efficiently scan compressed files, ensuring no threat goes undetected~~ (Currently WIP)
- **Privacy Prioritized:** Offering a privacy-friendly option, Raspirus keeps your personal information secure by simply not touching it
- **Cross-Platform Convenience:** Enjoy the benefits of Raspirus protection on a variety of operating systems, or compile it yourself on many more
- **Swift and Dependable:** Count on Raspirus for fast and malware detection
- **Simple to use UI:** Raspirus boasts a simple to use and easy to understand UI, making usage easy (Also currently WIP)

## Getting Started

### For Regular Users

#### Installation

Getting started with Raspirus is a breeze. Follow these simple steps:

1. Visit our [website](https://raspirus.deno.dev) or head to the [GitHub release page](https://github.com/Raspirus/Raspirus/releases/latest).
2. Download the executable that matches your operating system.

**Alterantives:**

- [Snap Store (Linux)](https://snapcraft.io/raspirus/)
- [Flathub (Linux)](https://flathub.org/apps/details/io.github.raspirus.raspirus)
- [SourceForge](https://sourceforge.net/projects/raspirus/files/latest/download)
- [Chocolatey (Windows)](https://community.chocolatey.org/packages/raspirus/)

#### Usage

Raspirus has two main modes: Graphical and CLI. By default, the application will attempt to launch with the GTK frontend as its user interface, which provides an easy to use graphical way of utilizing Raspirus. Secondly, there is also various CLI arguments, which let the user execute various functions without having to use a graphical interface.

##### Arguments

Here is a full list of the currently offered CLI arguments:

  - *-h / --help*: Shows a list with available arguments and their expected inputs
  - *-n / --nogui*: Prevents raspiurs from opening the graphical frontend. This will make it only execute the commands passed via the CLI
  - *-f / --fullscreen*: Attempts to open the graphical frontend in fullscreen mode. This is ideal for scenarios where Raspirus is the main application running on the machine
  - *-u / --update*: Attempts to fetch an update package from configured remote url
  - *-d / --debug*: Sets the application to log everything, even if irrelevant for the ordinary user. This will output large amounts of text to the terminal and log files, so use with care on slower machines
  - *-q / --quiet*: Silences all log output except for error messages
  - *-s / --scan*: Attempts to run a scan on the specified path
  - *-j / --json*: Outputs the scan results in json format for easy parsing by other utilities
  - *-t / --threads*: Sets the parallel threads used for scanning
  - *-x / --max*: Sets how many matches a file can produce at max, before further matches get ignored
  - *-i / --min*: Sets how many matches per file are required for it to get flagged
  - *-r / --remote*: Overrides the remote url set in the config which gets used to fetch updates. Only useful when utilized in combination with --update

### For Developers

Are you a developer looking to set up Raspirus? We've got you covered. Check out our comprehensive guides for various operating systems in the [Developers section](https://github.com/Raspirus/raspirus/wiki/Developers) on our docs.

## Questions?

Got questions about Raspirus? We're here to help!

- Visit our [FAQ section](https://github.com/Raspirus/raspirus/wiki/FAQ) for answers to common queries.
- Join our thriving community on the [Discord server](https://discord.gg/Vx7fW9PA8B) to engage with fellow users.
- If you've encountered a bug, browse the GitHub issues to see if it's already reported.


## Support the Project
If you find value in this project, consider sponsoring it through [GitHub Sponsors](https://github.com/sponsors/Raspirus) to contribute to its continued development and maintenance. Your support is greatly appreciated!

### Our sponsors:
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/tuchaVshortah"><img src="https://avatars.githubusercontent.com/u/71591558?v=4" width="100px;" alt="Profile picture"/><br /><sub><b>Nurkanat Baysenkul</b></sub></a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/dylanchxx"><img src="https://avatars.githubusercontent.com/u/127700913?v=4" width="100px;" alt="Profile picture"/><br /><sub><b>Dylanchxx</b></sub></a></td>
    </tr>
  </tbody>
</table>

Check out all the contributors on [our Website](https://raspirus.deno.dev/)

## Star History

<a href="https://star-history.com/#Raspirus/Raspirus&Date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=Raspirus/Raspirus&type=Date&theme=dark" />
    <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=Raspirus/Raspirus&type=Date" />
    <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=Raspirus/Raspirus&type=Date" />
  </picture>
</a>

## Awards
<div style="display: flex; flex-direction: row;">
    <img src="https://raw.githubusercontent.com/Raspirus/media/main/awards/oss-rising-star-white.svg" alt="Sourceforge award 1" width="200" style="max-width:100%;">
    <img src="https://raw.githubusercontent.com/Raspirus/media/main/awards/oss-users-love-us-white.svg" alt="Sourceforge award 2" width="200" style="max-width:100%;">
</div>




Thank you for choosing Raspirus for your malware protection needs. Together, we're making the digital world safer for everyone.
