# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-01-01

### Added

- Initial project scaffold with actix-web server.
- Groth16 proof generation for the Square circuit (`x² = y`).
- Groth16 proof generation for the Sum circuit (`a + b = sum`).
- Generic Groth16 proof verification endpoint.
- Health-check endpoint returning supported schemes and curves.
- Environment-based configuration (`HOST`, `PORT`, `SERVICE_NAME`).
- Unit tests for circuit constraint satisfaction.
- Integration tests for end-to-end proof generation and verification.
