git clone https://github.com/o1-labs/openmina.git
cd openmina

# Download required circuits
echo "Downloading required circuits..."
make download-circuits

# Build in debug mode (faster compilation, slower runtime)
echo "Building the Mina Rust Node in debug mode..."
make build
