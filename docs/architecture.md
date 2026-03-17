# Architecture

## Overview

Loka ZK Middleware is a Rust-based microservice providing RESTful APIs for Zero-Knowledge proof generation and verification. It is designed for privacy-enhanced Agentic payment flows.

## Components

```
src/
├── main.rs          # Application entry point and server bootstrap
├── config.rs        # Environment-based configuration
├── errors.rs        # Unified error types with HTTP mapping
├── api/
│   ├── mod.rs       # Route configuration
│   ├── handlers.rs  # Request handlers
│   └── models.rs    # Request/response data models
└── zk/
    ├── mod.rs       # Module exports
    ├── circuits.rs  # ZK circuit definitions (Square, Sum)
    └── service.rs   # Proof generation & verification service
```

## Request Flow

1. Client sends HTTP request to an API endpoint.
2. Actix-web routes the request to the appropriate handler in `src/api/handlers.rs`.
3. The handler validates input, then delegates to `ZkService` in `src/zk/service.rs`.
4. `ZkService` constructs the appropriate circuit from `src/zk/circuits.rs`, runs the Groth16 trusted setup, generates or verifies the proof, and returns the result.
5. The handler serializes the response and sends it back to the client.

## Proof System

- **Groth16** — a succinct non-interactive argument of knowledge (SNARK).
- **BN254** — the pairing-friendly elliptic curve used for proof computation.
- **arkworks** — the Rust library suite powering circuit definition, proving, and verification.

## Configuration

All configuration is loaded from environment variables at startup (see `src/config.rs`):

| Variable       | Default              | Description          |
|---------------|----------------------|----------------------|
| `HOST`        | `0.0.0.0`           | Bind address         |
| `PORT`        | `8080`               | Listen port          |
| `SERVICE_NAME`| `loka-zk-middleware` | Service identifier   |
| `RUST_LOG`    | —                    | Log level            |
