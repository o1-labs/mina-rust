# OpenMina Makefile

NIGHTLY_RUST_VERSION = "nightly"

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
	cargo build --release --bin openmina

.PHONY: build-tests-webrtc
build-tests-webrtc: ## Build tests for WebRTC
	@mkdir -p target/release/tests
	@cargo build --release --tests \
		--package=openmina-node-testing \
		--package=cli
	@cargo build --release \
		--features=scenario-generators,p2p-webrtc \
		--package=openmina-node-testing \
		--tests \
		--message-format=json \
		> cargo-build-test.json
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
format: ## Format code using rustfmt
	cargo +nightly fmt

.PHONY: format-md
format-md: ## Format all markdown files to wrap at 80 characters
	@echo "Formatting markdown files..."
	npx prettier --write "**/*.md"
	@echo "Markdown files have been formatted to 80 characters."

.PHONY: lint
lint: ## Run linter (clippy)
	cargo clippy --all-targets -- -D warnings --allow clippy::mutable_key_type

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
