(Thsi file is TEMPORARY)

## Generate Docs from the /app/src-tauri/ folder:
`cargo doc --no-deps --release --target-dir=../../docs/generated/`
- `--no-deps`: Remove unused docs
- `--release`: It is generally better than a debug
- `--target-dir`: Where to output the docs