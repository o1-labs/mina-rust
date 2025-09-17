# Pull and run the frontend Docker image
docker pull o1labs/mina-rust-frontend:latest
docker run -d --name mina-frontend -p 8070:80 o1labs/mina-rust-frontend:latest

# Access the dashboard at http://localhost:8070