# Build and serve the leaderboard application for custom hosting
cd frontend

# Build the application with leaderboard configuration
make build-leaderboard

# Serve the static files
npx http-server dist/frontend/browser -p 8080