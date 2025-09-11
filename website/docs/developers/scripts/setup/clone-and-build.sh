git clone https://github.com/o1-labs/mina-rust.git
cd mina-rust

# Download required circuits
echo "Downloading required circuits..."
make download-circuits

# Build in debug mode (faster compilation, slower runtime)
echo "Building the Mina Rust Node in debug mode..."
make build
