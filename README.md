# compsoc-game-jam

# building

- `cargo build --features bevy/dynamic`

## Make sure you're using the nightly toolchain:
- rustup toolchain install nightly
- rustup override set nightly

## if on windows

- `cargo install -f cargo-binutils`
- `rustup component add llvm-tools-preview`

# running
- `cargo run --features bevy/dynamic`
