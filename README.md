# compsoc-game-jam

# building (dev)
- `cargo build --features bevy/dynamic`

# building (release)
- `cargo build --release`

## Make sure you're using the nightly toolchain:
- rustup toolchain install nightly
- rustup override set nightly

## if on windows

- `cargo install -f cargo-binutils`
- `rustup component add llvm-tools-preview`

# running (dev)
- `cargo run --features bevy/dynamic`

# running (release)
- `cargo run --release`
