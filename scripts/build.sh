#!/bin/bash
# Build script for QRD-SDK

set -e

echo "Building QRD-SDK..."

# Format check
echo "Checking formatting..."
cargo fmt --all -- --check

# Lint
echo "Running clippy..."
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Build
echo "Building release..."
cargo build --workspace --release

# Test
echo "Running tests..."
cargo test --workspace --all-features

echo "✓ Build successful!"
