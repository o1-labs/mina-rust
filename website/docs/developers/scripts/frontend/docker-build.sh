# Build the frontend Docker image locally
# Note: Must be run from project root as Docker build context requires files from root

# Build with default production configuration
docker build -t mina-frontend . -f frontend/Dockerfile

# Run the locally built container
docker run -d -p 8070:80 mina-frontend