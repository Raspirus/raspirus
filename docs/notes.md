(Thsi file is TEMPORARY)

## Generate Docs from the /app/src-tauri/ folder:
`cargo doc --no-deps --release --target-dir=../../docs/generated/`
- `--no-deps`: Remove unused docs
- `--release`: It is generally better than a debug
- `--target-dir`: Where to output the docs

## Setup Rust analyzer to work in non-standard directory structure:
Link: https://github.com/rust-lang/rust-analyzer/issues/2649#issuecomment-691582605


## Build steps

### Warnings!
- Glibc can cause problems on Linux: https://tauri.app/v1/guides/building/linux#limitations
- You need to use 64 bit systems, else the app might crash because its using memory improvements
- The app is meant to be run as a "I'm the only app running on this system" app. This is important regarding RAM usage,
because if you have much RAM, it will use much RAM. And if you for some reason try to limit the initially available RAM,
the app might crash because it doesn't have the promised amount of RAM. (A future version might have a toggle for this)


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