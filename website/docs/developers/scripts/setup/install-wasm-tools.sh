# Install wasm-pack for WebAssembly builds
cargo install wasm-pack

# Install wasm-bindgen CLI tool for generating WebAssembly bindings
cargo install -f wasm-bindgen-cli --version 0.2.99

# Add WebAssembly target
rustup target add wasm32-unknown-unknown
