.PHONY: help test fmt clippy check examples docs clean audit coverage dev status quick-release

# Default target
help:
.PHONY: help test fmt clippy check examples docs clean audit coverage dev status quick security setup release

# AI Assistant Optimized Development Workflow
# Use these commands for automated development tasks

# Default target - show AI-friendly help
help:
	@echo "🤖 AI Assistant Optimized Commands:"
	@echo ""
	@echo "🚀 Quick Development:"
	@echo "  make quick        - Quick checks (format, lint, compile) ⭐"
	@echo "  make test         - Full test suite with coverage"
	@echo "  make dev          - Development status overview"
	@echo ""
	@echo "🔒 Quality & Security:"
	@echo "  make security     - Security audits and vulnerability checks"
	@echo "  make coverage     - Generate test coverage report"
	@echo ""
	@echo "📦 Release Management:"
	@echo "  make release      - Interactive patch release"
	@echo "  make setup        - Setup development environment"
	@echo ""
	@echo "🎯 Individual Commands:"
	@echo "  make fmt          - Format code"
	@echo "  make clippy       - Lint code"
	@echo "  make check        - Type check (fast)"
	@echo "  make clean        - Clean build artifacts"
	@echo ""
	@echo "💡 Advanced Usage:"
	@echo "  ./scripts/quick-release.sh [patch|minor|major]"
	@echo "  ./scripts/tasks.sh add 'Task description'"
	@echo "  ./scripts/dev-workflow.sh [setup|quick|test|security|coverage]"
	@echo ""
	@echo "📋 VS Code Integration:"
	@echo "  Ctrl+Shift+P → 'Tasks: Run Task' for GUI access"

# 🚀 Quick development check (AI Assistant Recommended)
quick:
	@echo "🤖 Running AI-optimized quick check..."
	@./scripts/simple-check.sh

# 🧪 Full test suite (AI Assistant Recommended)  
test:
	@echo "🤖 Running comprehensive test suite..."
	@./scripts/dev-workflow.sh test

# 🔒 Security checks (AI Assistant Recommended)
security:
	@echo "🤖 Running security audits..."
	@./scripts/dev-workflow.sh security

# 📊 Coverage report (AI Assistant Recommended)
coverage:
	@echo "🤖 Generating coverage report..."
	@./scripts/dev-workflow.sh coverage

# 📦 Release management (AI Assistant Recommended)
release:
	@echo "🤖 Starting automated release process..."
	@./scripts/quick-release.sh patch

# ⚙️ Development environment setup (AI Assistant Recommended)
setup:
	@echo "🤖 Setting up development environment..."
	@./scripts/dev-workflow.sh setup

# 📋 Development status overview (AI Assistant Recommended)
dev:
	@echo "🤖 Checking development status..."
	@./scripts/tasks.sh status

status: dev

# 🔧 Individual component commands (for specific needs)
fmt:
	@echo "Formatting code..."
	@cargo fmt --all

clippy:
	@echo "Running clippy lints..."
	@cargo clippy --all-targets --all-features -- -D warnings

check:
	@echo "Type checking..."
	@cargo check --all-targets --all-features

examples:
	@echo "Checking examples..."
	@cargo check --examples

docs:
	@echo "Generating documentation..."
	@cargo doc --no-deps --open

clean:
	@echo "Cleaning build artifacts..."
	@cargo clean

# 🛠️ Tool installation (for setup)
install-tools:
	@echo "Installing development tools..."
	@cargo install cargo-audit cargo-llvm-cov cargo-outdated cargo-deny
