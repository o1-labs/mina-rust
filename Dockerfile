FROM rust:bullseye AS build
# hadolint ignore=DL3008
RUN apt-get update && \
    apt-get install -y --no-install-recommends protobuf-compiler && \
    apt-get clean

WORKDIR /openmina

COPY rust-toolchain.toml .

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN RUST_VERSION=$(grep 'channel = ' rust-toolchain.toml | \
        sed 's/channel = "\(.*\)"/\1/') && \
    rustup default "$RUST_VERSION" && \
    rustup component add rustfmt

COPY . .

RUN make build-release && \
    mkdir -p /openmina/release-bin && \
    cp /openmina/target/release/openmina /openmina/release-bin/openmina

RUN make build-testing && \
    mkdir -p /openmina/testing-release-bin && \
    cp /openmina/target/release/openmina-node-testing \
        /openmina/testing-release-bin/openmina-node-testing

# necessary for proof generation when running a block producer.
RUN make download-circuits && \
    rm -rf circuit-blobs/berkeley_rc1 circuit-blobs/*/tests

FROM debian:bullseye
# hadolint ignore=DL3008
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libjemalloc2 libssl1.1 libpq5 curl jq procps && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /openmina/release-bin/openmina /usr/local/bin/
COPY --from=build /openmina/testing-release-bin/openmina-node-testing \
    /usr/local/bin/

RUN mkdir -p /usr/local/lib/openmina/circuit-blobs
COPY --from=build /openmina/circuit-blobs/ \
    /usr/local/lib/openmina/circuit-blobs/

EXPOSE 3000
EXPOSE 8302

ENTRYPOINT [ "openmina" ]
