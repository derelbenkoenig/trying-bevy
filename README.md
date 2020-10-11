# my-amethyst-project
game development using the Amethyst game engine in Rust

## Purpose
Create a focused, opinionated fighting game engine from Amethyst.

Initial support goals for the project are.

- Rollback
    - Predict user inputs
    - Game state should be serialize-able
    - Rewindable
- Easy to create games from
    - Add character assets
    - Add move sets for characters
- Proof of concept
    - A fighting game using the engine designed above

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
