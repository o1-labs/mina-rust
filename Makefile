# OpenMina Makefile

# Rust
NIGHTLY_RUST_VERSION = "nightly"

# Docker
DOCKER_ORG ?= openmina

# PostgreSQL configuration for archive node
OPEN_ARCHIVE_ADDRESS ?= http://localhost:3007
PG_USER ?= openmina
PG_PW 	?= openminaopenmina
PG_DB 	?= openmina_archive
PG_HOST	?= localhost
PG_PORT	?= 5432

# Utilities
NETWORK ?= devnet
GIT_COMMIT := $(shell git rev-parse --short=8 HEAD)

.PHONY: help
help: ## Ask for help!
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: build
build: ## Build the project in debug mode
	cargo build

.PHONY: build-ledger
build-ledger: download-circuits ## Build the ledger binary and library, requires nightly Rust
	@cd ledger && cargo +nightly build --release --tests

.PHONY: build-release
build-release: ## Build the project in release mode
	cargo build --release --package=cli --bin openmina

.PHONY: build-testing
build-testing: ## Build the testing binary with scenario generators
	cargo build --release --features scenario-generators --bin openmina-node-testing

.PHONY: build-tests-webrtc
build-tests-webrtc: ## Build tests for WebRTC
	@mkdir -p target/release/tests
	@cargo build --release --tests \
		--package=openmina-node-testing \
		--package=cli
# Update ./.gitignore accordingly if cargo-build-test.json is changed
	@cargo build --release \
		--features=scenario-generators,p2p-webrtc \
		--package=openmina-node-testing \
		--tests \
		--message-format=json \
		> cargo-build-test.json
# Update ./.gitignore accordingly if tests.json is changed
	@jq -r '. | select(.executable != null and (.target.kind | (contains(["test"])))) | [.target.name, .executable ] | @tsv' cargo-build-test.json > tests.tsv
	@while read NAME FILE; do \
		cp -a $$FILE target/release/tests/webrtc_$$NAME; \
	done < tests.tsv

.PHONY: build-vrf
build-vrf: ## Build the VRF package
	@cd vrf && cargo +nightly build --release --tests

.PHONY: build-wasm
build-wasm: ## Build WebAssembly node
	@cd node/web && cargo +nightly build \
		--release --target wasm32-unknown-unknown
# Update ./.gitignore accordingly if the out-dir is changed
	@wasm-bindgen --keep-debug --web \
		--out-dir pkg \
		target/wasm32-unknown-unknown/release/openmina_node_web.wasm

.PHONY: check
check: ## Check code for compilation errors
	cargo check --all-targets

.PHONY: check-tx-fuzzing
check-tx-fuzzing: ## Check the transaction fuzzing tools, requires nightly Rust
	@cd tools/fuzzing && cargo +nightly check

.PHONY: check-format
check-format: ## Check code formatting
	cargo +nightly fmt -- --check
	taplo format --check

.PHONY: check-md
check-md: ## Check if markdown files are properly formatted
	@echo "Checking markdown formatting..."
	npx prettier --check "**/*.md"
	@echo "Markdown format check completed."

.PHONY: clean
clean: ## Clean build artifacts
	cargo clean

.PHONY: download-circuits
download-circuits: ## Download the circuits used by Mina from GitHub
	@if [ ! -d "circuit-blobs" ]; then \
	  git clone --depth 1 https://github.com/openmina/circuit-blobs.git; \
	  ln -s -b "$$PWD"/circuit-blobs/* ledger/; \
	else \
	  echo "circuit-blobs already exists, skipping download."; \
	fi

.PHONY: format
format: ## Format code using rustfmt and taplo
	cargo +nightly fmt
	taplo format

.PHONY: format-md
format-md: ## Format all markdown files to wrap at 80 characters
	@echo "Formatting markdown files..."
	npx prettier --write "**/*.md"
	@echo "Markdown files have been formatted to 80 characters."

.PHONY: lint
lint: ## Run linter (clippy)
	cargo clippy --all-targets -- -D warnings --allow clippy::mutable_key_type

.PHONY: lint-dockerfiles
lint-dockerfiles: ## Check all Dockerfiles using hadolint
	@if [ "$$GITHUB_ACTIONS" = "true" ]; then \
		OUTPUT=$$(find . -name "Dockerfile*" -type f -exec hadolint {} \;); \
		if [ -n "$$OUTPUT" ]; then \
			echo "$$OUTPUT"; \
			exit 1; \
		fi; \
	else \
		OUTPUT=$$(find . -name "Dockerfile*" -type f -exec sh -c 'docker run --rm -i hadolint/hadolint < "$$1"' _ {} \;); \
		if [ -n "$$OUTPUT" ]; then \
			echo "$$OUTPUT"; \
			exit 1; \
		fi; \
	fi

.PHONY: setup-wasm-toolchain
setup-wasm-toolchain: ## Setup the WebAssembly toolchain, using nightly
		@ARCH=$$(uname -m); \
		OS=$$(uname -s | tr A-Z a-z); \
		case $$OS in \
			linux) OS_PART="unknown-linux-gnu" ;; \
			darwin) OS_PART="apple-darwin" ;; \
			*) echo "Unsupported OS: $$OS" && exit 1 ;; \
		esac; \
		case $$ARCH in \
			x86_64) ARCH_PART="x86_64" ;; \
			aarch64) ARCH_PART="aarch64" ;; \
			arm64) ARCH_PART="aarch64" ;; \
			*) echo "Unsupported architecture: $$ARCH" && exit 1 ;; \
		esac; \
		TARGET="$$ARCH_PART-$$OS_PART"; \
		echo "Installing rust-src and rustfmt for ${NIGHTLY_RUST_VERSION}-$$TARGET with wasm32 target"; \
		rustup target add wasm32-unknown-unknown --toolchain ${NIGHTLY_RUST_VERSION}-$$TARGET

.PHONY: test
test: ## Run tests
	cargo test

.PHONY: test-ledger
test-ledger: build-ledger ## Run ledger tests in release mode, requires nightly Rust
	@cd ledger && cargo +nightly test --release -- -Z unstable-options --report-time

.PHONY: test-p2p
test-p2p: ## Run P2P tests
	cargo test -p p2p --tests

.PHONY: test-release
test-release: ## Run tests in release mode
	cargo test --release

.PHONY: test-vrf
test-vrf: ## Run VRF tests, requires nightly Rust
	@cd vrf && cargo +nightly test --release -- -Z unstable-options --report-time

# Docker build targets

.PHONY: docker-build-all
docker-build-all: docker-build-bootstrap-sandbox docker-build-debugger \
	docker-build-frontend docker-build-fuzzing docker-build-heartbeats-processor \
	docker-build-light docker-build-light-focal docker-build-openmina \
	docker-build-openmina-testing docker-build-producer-dashboard \
	docker-build-test ## Build all Docker images

.PHONY: docker-build-bootstrap-sandbox
docker-build-bootstrap-sandbox: ## Build bootstrap sandbox Docker image
	docker build -t $(DOCKER_ORG)/openmina-bootstrap-sandbox:$(GIT_COMMIT) \
		tools/bootstrap-sandbox/

.PHONY: docker-build-debugger
docker-build-debugger: ## Build debugger Docker image
	docker build -t $(DOCKER_ORG)/openmina-debugger:$(GIT_COMMIT) \
		-f node/testing/docker/Dockerfile.debugger node/testing/docker/

.PHONY: docker-build-frontend
docker-build-frontend: ## Build frontend Docker image
	docker build -t $(DOCKER_ORG)/openmina-frontend:$(GIT_COMMIT) frontend/

.PHONY: docker-build-fuzzing
docker-build-fuzzing: ## Build fuzzing Docker image
	docker build -t $(DOCKER_ORG)/openmina-fuzzing:$(GIT_COMMIT) tools/fuzzing/

.PHONY: docker-build-heartbeats-processor
docker-build-heartbeats-processor: ## Build heartbeats processor Docker image
	docker build -t $(DOCKER_ORG)/openmina-heartbeats-processor:$(GIT_COMMIT) \
		tools/heartbeats-processor/

.PHONY: docker-build-light
docker-build-light: ## Build light Docker image
	docker build -t $(DOCKER_ORG)/openmina-light:$(GIT_COMMIT) \
		-f node/testing/docker/Dockerfile.light node/testing/docker/

.PHONY: docker-build-light-focal
docker-build-light-focal: ## Build light focal Docker image
	docker build -t $(DOCKER_ORG)/openmina-light-focal:$(GIT_COMMIT) \
		-f node/testing/docker/Dockerfile.light.focal node/testing/docker/

.PHONY: docker-build-openmina
docker-build-openmina: ## Build main OpenMina Docker image
	docker build -t $(DOCKER_ORG)/openmina:$(GIT_COMMIT) .

.PHONY: docker-build-openmina-testing
docker-build-openmina-testing: ## Build OpenMina testing Docker image
	docker build -t $(DOCKER_ORG)/openmina-testing:$(GIT_COMMIT) \
		-f node/testing/docker/Dockerfile.openmina node/testing/docker/

.PHONY: docker-build-producer-dashboard
docker-build-producer-dashboard: ## Build producer dashboard Docker image
	docker build -t $(DOCKER_ORG)/openmina-producer-dashboard:$(GIT_COMMIT) \
		-f docker/producer-dashboard/Dockerfile .

.PHONY: docker-build-test
docker-build-test: ## Build test Docker image
	docker build -t $(DOCKER_ORG)/openmina-test:$(GIT_COMMIT) \
		-f node/testing/docker/Dockerfile.test node/testing/docker/

# Postgres related targets + archive node
.PHONY: run-archive
run-archive: build-release ## Run an archive node with local storage
	OPENMINA_ARCHIVE_ADDRESS=$(OPENMINA_ARCHIVE_ADDRESS) \
		cargo run --bin openmina \
		--release -- \
		node \
		--archive-archiver-process \
		--archive-local-storage
		--network $(NETWORK)

.PHONY: postgres-clean
postgres-clean:
	@echo "Dropping DB: ${PG_DB} and user: ${PG_USER}"
	@sudo -u postgres psql -c "DROP DATABASE IF EXISTS ${PG_DB}"
	@sudo -u postgres psql -c "DROP DATABASE IF EXISTS ${PG_USER}"
	@sudo -u postgres psql -c "DROP ROLE IF EXISTS ${PG_USER}"
	@echo "Cleanup complete."

.PHONY: postgres-setup
postgres-setup: ## Set up PostgreSQL database for archive node
	@echo "Setting up PostgreSQL database: ${PG_DB} with user: ${PG_USER}"
	@sudo -u postgres createuser -d -r -s $(PG_USER) 2>/dev/null || true
	@sudo -u postgres psql -c "ALTER USER $(PG_USER) PASSWORD '$(PG_PW)'" 2>/dev/null || true
	@sudo -u postgres createdb -O $(PG_USER) $(PG_DB) 2>/dev/null || true
	@sudo -u postgres createdb -O $(PG_USER) $(PG_USER) 2>/dev/null || true
