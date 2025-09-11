# Build stage
FROM rust:1.84-slim-bookworm AS builder

WORKDIR /usr/src/app
# hadolint ignore=DL3008
RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY . .
RUN ls -la tools/heartbeats-processor && \
    cargo build --release -p heartbeats-processor

# Runtime stage
FROM debian:bookworm-slim

# hadolint ignore=DL3008
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libsqlite3-0 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/heartbeats-processor .
COPY tools/heartbeats-processor/schema.sql .

ENV DATABASE_PATH=/app/data/heartbeats.db

ENTRYPOINT ["./heartbeats-processor"]
CMD ["process-loop"]
