# Run WebNode with webnode configuration
# This enables WASM node functionality and downloads circuit files
docker run -p 4200:80 \
  -e MINA_FRONTEND_ENVIRONMENT=webnode \
  mina-frontend