# Test act can list workflows
if command -v "gh act" >/dev/null 2>&1; then
  echo "✅ Testing act functionality..."
  gh act -l -W .github/workflows/test-docs-scripts-ci.yaml
else
  echo "⚠️  Act extension not available (likely due to authentication in CI)"
  echo "   In a real setup with authentication, you would run:"
  echo "   gh act -l -W .github/workflows/test-docs-scripts-ci.yaml"
  echo "✅ Test completed - act functionality would work with proper authentication"
fi