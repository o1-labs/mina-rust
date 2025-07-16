FROM rust:1.84.0-bullseye AS builder

RUN mkdir -p ~/.ssh && chmod 0700 ~/.ssh && ssh-keyscan github.com >> ~/.ssh/known_hosts

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

RUN cargo install --git https://github.com/openmina/openmina.git --branch fix/bootstrap-replay openmina-bootstrap-sandbox --locked

FROM debian:bullseye

# hadolint ignore=DL3008
RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/openmina-bootstrap-sandbox \
    /usr/local/bin/openmina-bootstrap-sandbox

COPY . /target/record

ENV RUST_LOG=info

WORKDIR /
