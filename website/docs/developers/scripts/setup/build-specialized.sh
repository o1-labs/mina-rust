# Build in release mode (slower compilation, faster runtime)
echo "Building in release mode..."
make build-release

# Build ledger components (requires nightly Rust)
echo "Building ledger components..."
make build-ledger

# Build WebAssembly node for browser
echo "Building WebAssembly node..."
make build-wasm

# Build testing framework with scenario generators
echo "Building testing framework..."
make build-testing

# Build VRF (Verifiable Random Function) components
echo "Building VRF components..."
make build-vrf
