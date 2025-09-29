# Install nvm (if not already installed)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh | bash

# Source nvm
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

# Install Node.js
nvm install 23.1.0
nvm use 23.1.0
nvm alias default 23.1.0

# Verify installation
echo "Node.js version: $(node --version)"
echo "npm version: $(npm --version)"

echo "Node.js installation complete!"
