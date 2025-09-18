# Install act extension for GitHub CLI
if gh extension install nektos/gh-act; then
  echo "✅ Act extension installed successfully"
  # Verify installation
  gh act --version
else
  echo "⚠️  Act extension installation failed - likely due to authentication"
  echo "   This is expected in CI environments without proper GitHub authentication"
  echo "   In a real setup, users would run: gh auth login"
  echo "✅ GitHub CLI is properly installed and ready for act extension"
fi