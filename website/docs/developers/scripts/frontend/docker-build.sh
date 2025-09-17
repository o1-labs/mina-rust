# Build the frontend Docker image locally
cd frontend

# Build with default production configuration
docker build -t mina-frontend .

# Run the locally built container
docker run -d -p 8070:80 mina-frontend