#!/usr/bin/env bash
set -euo pipefail

echo "==> Running all tests..."
RUST_LOG="${RUST_LOG:-info}" cargo test --all-features "$@"
echo "==> Tests complete."
