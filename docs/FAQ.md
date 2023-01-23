# Frequently Asked Questions

## How do I generate the documentation for this repository?
You can find the generated documentation in the [docs folder](https://github.com/Benji377/Raspirus/tree/main/docs/generated) and in case you want to generate
your own, you can do so by using the `cargo doc` command. Here are some parameters you might want to use with it:
- `--no-deps`: Ignores dependencies, only documents the code itself
- `--release`: It is generally better than a debug
- `--target-dir`: Where to output the docs
All together, the command might look something like this: \
`cargo doc --no-deps --release --target-dir=/docs/generated/`

## In VSCode, how do I set up Rust analyzer to work in non-standard directory structure?
The Rust analyzer plugin in Visual Studio Code tries to search for a Cargo.toml file in the current directory, or parent directory. But since we packed the entire 
application in the `app` directory, it's unable to find the file and therefore might not work. This is a big lost, as it doesn't tell you if your Rust files
have correct syntax or not. To solve this issue, you can add an option to the plugin and specify the location of your Cargo.toml file.
As stated [in this comment](https://github.com/rust-lang/rust-analyzer/issues/2649#issuecomment-691582605), you need to add the following lines to the end
of your plugin settings' JSON. Afterward, you will also need to restart the Analyzer for the modification to take effect.
```json
{
    "rust-analyzer.linkedProjects": [
        "/home/matklad/tmp/hello/Cargo.toml"
    ]
}
```

## More coming soon
...
