# Install Node.js
brew install node@23

# Link the installed version
brew link --overwrite node@23

# Verify installation
echo "Node.js version: $(node --version)"
echo "npm version: $(npm --version)"

echo "Node.js installation complete!"
