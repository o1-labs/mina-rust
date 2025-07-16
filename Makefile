# OpenMina Makefile

.PHONY: help
help: ## Ask for help!
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: build
build: ## Build the project in debug mode
	cargo build

.PHONY: build-release
build-release: ## Build the project in release mode
	cargo build --release --bin openmina

.PHONY: build-tests
build-tests: ## Build test binaries
	cargo build --release --tests --package=openmina-node-testing --package=cli

.PHONY: build-wasm
build-wasm: ## Build WebAssembly node
	cd node/web && cargo +nightly build --release --target wasm32-unknown-unknown

.PHONY: check
check: ## Check code for compilation errors
	cargo check --all-targets

.PHONY: check-format
check-format: ## Check code formatting
	cargo fmt --all -- --check

.PHONY: clean
clean: ## Clean build artifacts
	cargo clean

.PHONY: format
format: ## Format code using rustfmt
	cargo fmt --all

.PHONY: lint
lint: ## Run linter (clippy)
	cargo clippy --all-targets -- -D warnings --allow clippy::mutable_key_type

.PHONY: test
test: ## Run tests
	cargo test

.PHONY: test-ledger
test-ledger: ## Run ledger tests
	cd ledger && cargo test --release

.PHONY: test-p2p
test-p2p: ## Run P2P tests
	cargo test -p p2p --tests

.PHONY: test-release
test-release: ## Run tests in release mode
	cargo test --release

.PHONY: test-vrf
test-vrf: ## Run VRF tests
	cd vrf && cargo test --release