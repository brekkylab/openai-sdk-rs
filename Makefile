.PHONY: help test fmt clippy check examples docs clean audit coverage

# Default target
help:
	@echo "Available targets:"
	@echo "  test      - Run all tests"
	@echo "  fmt       - Format code"
	@echo "  clippy    - Run clippy lints"
	@echo "  check     - Type check (fast)"
	@echo "  examples  - Check examples compile"
	@echo "  docs      - Generate documentation"
	@echo "  clean     - Clean build artifacts"
	@echo "  audit     - Security audit"
	@echo "  coverage  - Generate test coverage"
	@echo "  ci        - Run full CI checks locally"

# Run tests
test:
	cargo test --all-features
	cargo test --doc

# Format code
fmt:
	cargo fmt --all

# Run clippy
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Type check (fast)
check:
	cargo check --all-targets --all-features

# Check examples compile
examples:
	cargo check --examples

# Generate documentation
docs:
	cargo doc --no-deps --open

# Clean build artifacts
clean:
	cargo clean

# Security audit
audit:
	cargo audit

# Generate test coverage (requires cargo-llvm-cov)
coverage:
	cargo llvm-cov --all-features --workspace --html
	@echo "Coverage report generated in target/llvm-cov/html/index.html"

# Run full CI checks locally
ci: fmt clippy test examples docs audit
	@echo "All CI checks passed!"

# Install development tools
install-tools:
	cargo install cargo-audit
	cargo install cargo-llvm-cov
	cargo install cargo-outdated
	cargo install cargo-deny
