# Install rustup (Rust toolchain installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Source cargo environment
source ~/.cargo/env

# Install Rust 1.84 (as specified in rust-toolchain.toml)
rustup install 1.84
rustup default 1.84

# Install nightly toolchain (required for some components)
rustup install nightly

# Add required components for Rust 1.84
rustup component add rustfmt clippy --toolchain 1.84

# Add required components for nightly
rustup component add rustfmt clippy rust-src --toolchain nightly
