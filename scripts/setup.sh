#!/usr/bin/env bash
set -euo pipefail

echo "==> Setting up development environment..."

# Check for Rust toolchain
if ! command -v rustup &> /dev/null; then
    echo "rustup not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    # shellcheck source=/dev/null
    source "$HOME/.cargo/env"
fi

echo "    Rust toolchain: $(rustc --version)"

# Install required components
rustup component add rustfmt clippy

echo "==> Fetching dependencies..."
cargo fetch

echo "==> Setup complete."
