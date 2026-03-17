.PHONY: build run test lint fmt clean docker docker-up docker-down setup help

## Default target
help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Build in release mode
	cargo build --release

run: ## Run the service (release)
	cargo run --release

test: ## Run all tests
	cargo test --all-features

lint: ## Run clippy and format checks
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

fmt: ## Auto-format source code
	cargo fmt --all

clean: ## Remove build artifacts
	cargo clean

docker: ## Build Docker image
	docker build -t loka-zk-middleware .

docker-up: ## Start services with docker compose
	docker compose up -d

docker-down: ## Stop services with docker compose
	docker compose down

setup: ## Set up the development environment
	bash scripts/setup.sh
