# Deploy WebNode to a web server
cd frontend

# Build for production
make build-webnode

# Deploy to static hosting
# Note: Adjust the destination path as needed for your hosting setup
cp -r dist/frontend/browser/* /var/www/webnode/