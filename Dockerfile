FROM rust:bullseye AS build
# hadolint ignore=DL3008
RUN apt-get update && \
    apt-get install -y --no-install-recommends protobuf-compiler ocaml && \
    apt-get clean

WORKDIR /mina

COPY rust-toolchain.toml .

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN RUST_VERSION=$(grep 'channel = ' rust-toolchain.toml | \
        sed 's/channel = "\(.*\)"/\1/') && \
    rustup default "$RUST_VERSION" && \
    rustup component add rustfmt

COPY . .

RUN make build-release && \
    mkdir -p /mina/release-bin && \
    cp /mina/target/release/mina /mina/release-bin/mina

RUN make build-testing && \
    mkdir -p /mina/testing-release-bin && \
    cp /mina/target/release/mina-node-testing \
        /mina/testing-release-bin/mina-node-testing

# necessary for proof generation when running a block producer.
RUN make download-circuits && \
    rm -rf circuit-blobs/berkeley_rc1 circuit-blobs/*/tests

FROM debian:bullseye
# hadolint ignore=DL3008
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        libjemalloc2 \
        libssl1.1 \
        libpq5 \
        jq \
        procps && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /mina/release-bin/mina /usr/local/bin/
COPY --from=build /mina/testing-release-bin/mina-node-testing \
    /usr/local/bin/

RUN mkdir -p /usr/local/lib/mina/circuit-blobs
COPY --from=build /mina/circuit-blobs/ \
    /usr/local/lib/mina/circuit-blobs/

EXPOSE 3000
EXPOSE 8302

ENTRYPOINT [ "mina" ]
