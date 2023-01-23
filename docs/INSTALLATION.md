# Clone and compile
⚠️ WORK IN PROGRESS ⚠️

## Introduction
For people that just want a working app, they can just head over to the [Release page](github.com/Benji377/Raspirus/releases/latest) 
and download the executable for the correct platform. But if you are on a different Linux distribution, unsupported OS, or just want to
compile the project on your own, this step-by-step guide will guide you.

## Limitations
- Glibc can cause problems on Linux: https://tauri.app/v1/guides/building/linux#limitations
- You need to use 64 bit systems, else the app might crash because its using memory improvements
- The app is meant to be run as a "I'm the only app running on this system" app. This is important regarding RAM usage,
because if you have much RAM, it will use much RAM. And if you for some reason try to limit the initially available RAM,
the app might crash because it doesn't have the promised amount of RAM. (A future version might have a toggle for this)

## Step by step guide
1. Clone repository
2. Install Rust
3. Install NPM
4. Install Nextjs
5. Install Tauri
6. Install all needed dependencies
7. Check that Rust works by executing `cargo build` inside the `app/src-tauri/` folder
8. Build the project by executing `cargo tauri build` inside the `app/` folder 
9. You should see a line indicating the location of your executable in the terminal
10. Make sure that you have a graphical overlay where the app can run. SSH might not work for this purpose

## Conclusion
