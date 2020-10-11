# my-amethyst-project
game development using the Amethyst game engine in Rust

## Dependencies
This project relies on [amethyst](https://github.com/amethyst/amethyst#dependencies).

Ensure you follow its Dependencies installation guide.

## Running locally
Due to a workaround since conditional features aren't supported yet (https://github.com/rust-lang/cargo/issues/7914)
features are used to choose which OS you want this to compile on, and the appropriate dependencies will be used.

```bash
# Linux, this is the default so nothing extra is needed
cargo run --release
```

```bash
# Windows
cargo run --release --features windows --no-default-features
```

```bash
# Mac OS
cargo run --release --features mac --no-default-features
```
