# Agents

## Overview

This document describes the AI coding agents configuration for the loka-zk-middleware project.

## Project Context

Loka ZK Middleware is a Rust microservice built with **actix-web** that provides Zero-Knowledge proof generation and verification via RESTful APIs, powered by the **arkworks** library suite (Groth16 on BN254).

## Build & Test

```bash
# Build
cargo build --release

# Run tests
cargo test --all-features

# Lint
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Project Structure

```
src/
├── main.rs          # Entry point — actix-web server bootstrap
├── config.rs        # Environment-based configuration (HOST, PORT, etc.)
├── errors.rs        # ZkError enum with actix ResponseError impl
├── api/
│   ├── mod.rs       # Route configuration
│   ├── handlers.rs  # HTTP request handlers
│   └── models.rs    # Request/response serde models
└── zk/
    ├── mod.rs       # Module re-exports
    ├── circuits.rs  # arkworks R1CS circuit definitions
    └── service.rs   # Proof generation & verification logic
```

## Conventions

- All public API errors use the `ZkError` enum from `src/errors.rs`.
- Routes are registered in `src/api/mod.rs` under the `/api/v1` scope.
- ZK circuits implement `ark_relations::r1cs::ConstraintSynthesizer<Fr>`.
- Unit tests live alongside source files in `#[cfg(test)]` modules.
- Integration / E2E test scripts are in the `tests/` directory.
