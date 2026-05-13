.PHONY: help build test bench check fmt lint clean doc install-hooks fuzz

help:
	@echo "QRD-SDK Development Commands:"
	@echo "  make build          Build all crates in release mode"
	@echo "  make test           Run all tests"
	@echo "  make test-all       Run all tests with all features"
	@echo "  make bench          Run benchmarks"
	@echo "  make check          Run cargo check with all features"
	@echo "  make fmt            Format code with rustfmt"
	@echo "  make fmt-check      Check formatting without changes"
	@echo "  make lint           Run clippy linter"
	@echo "  make doc            Generate documentation"
	@echo "  make doc-open       Generate and open documentation"
	@echo "  make fuzz           Run fuzzing tests"
	@echo "  make clean          Clean build artifacts"
	@echo "  make install-hooks  Install git pre-commit hooks"

build:
	cargo build --workspace --release

test:
	cargo test --workspace --lib --bins

test-all:
	cargo test --workspace --all-features

bench:
	cargo bench --workspace

check:
	cargo check --workspace --all-features

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

doc:
	cargo doc --workspace --no-deps --release

doc-open:
	cargo doc --workspace --no-deps --release --open

fuzz:
	cargo +nightly fuzz run --manifest-path tests/fuzz/Cargo.toml

clean:
	cargo clean

install-hooks:
	@echo "Installing git pre-commit hooks..."
	@mkdir -p .git/hooks
	@echo '#!/bin/sh\nmake fmt-check && make lint' > .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "✓ Git hooks installed"

ci: fmt-check lint test doc
	@echo "✓ All CI checks passed"
