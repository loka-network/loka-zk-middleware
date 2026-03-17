#!/usr/bin/env bash
set -euo pipefail

echo "==> Building loka-zk-middleware (release)..."
cargo build --release
echo "==> Build complete."
