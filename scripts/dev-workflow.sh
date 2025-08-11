#!/usr/bin/env bash

# OpenAI SDK Rust - Development Workflow Automation Script
# Usage: ./scripts/dev-workflow.sh [command]

set -euo pipefail

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 日志函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check dependencies
check_dependencies() {
    log_info "Checking development dependencies..."
    
    if ! command -v cargo &> /dev/null; then
        log_error "cargo not found, please install Rust"
        exit 1
    fi
    
    # Check essential cargo tools
    tools=("cargo-audit" "cargo-llvm-cov")
    optional_tools=("cargo-outdated" "cargo-deny")
    missing_tools=()
    missing_optional=()
    
    for tool in "${tools[@]}"; do
        if ! cargo install --list | grep -q "^$tool"; then
            missing_tools+=("$tool")
        fi
    done
    
    for tool in "${optional_tools[@]}"; do
        if ! cargo install --list | grep -q "^$tool"; then
            missing_optional+=("$tool")
        fi
    done
    
    # Install essential tools
    if [ ${#missing_tools[@]} -ne 0 ]; then
        log_warning "Missing essential tools: ${missing_tools[*]}"
        log_info "Installing essential tools..."
        for tool in "${missing_tools[@]}"; do
            if ! cargo install "$tool"; then
                log_error "Failed to install $tool"
                exit 1
            fi
        done
    fi
    
    # Try to install optional tools (don't exit on failure)
    if [ ${#missing_optional[@]} -ne 0 ]; then
        log_warning "Missing optional tools: ${missing_optional[*]}"
        log_info "Attempting to install optional tools (failures won't affect main workflow)..."
        for tool in "${missing_optional[@]}"; do
            if ! cargo install "$tool" 2>/dev/null; then
                log_warning "Skipping $tool installation (may require newer Rust version)"
            fi
        done
    fi
    
    log_success "Dependency check completed"
}

# Quick development check
quick_check() {
    log_info "Running quick development check..."
    
    log_info "1. Checking code formatting..."
    cargo fmt --all -- --check || {
        log_warning "Code formatting issues found, auto-fixing..."
        cargo fmt --all
        log_success "Code formatting fixed"
    }
    
    log_info "2. Running clippy checks..."
    cargo clippy --all-targets --all-features -- -D warnings
    
    log_info "3. Fast compilation check..."
    cargo check --all-targets --all-features
    
    log_success "Quick check completed"
}

# Full test suite
full_test() {
    log_info "Running full test suite..."
    
    log_info "1. Unit tests..."
    cargo test --all-features
    
    log_info "2. Doc tests..."
    cargo test --doc
    
    log_info "3. Example compilation check..."
    cargo check --examples
    
    log_success "All tests passed"
}

# Security checks
security_check() {
    log_info "Running security checks..."
    
    log_info "1. Dependency security audit..."
    if command -v cargo-audit &> /dev/null; then
        cargo audit
    else
        log_warning "cargo-audit not installed, skipping security audit"
    fi
    
    log_info "2. License check..."
    if command -v cargo-deny &> /dev/null; then
        cargo deny check licenses 2>/dev/null || log_warning "License check failed or configuration incomplete"
    else
        log_warning "cargo-deny not installed, skipping license check"
    fi
    
    log_info "3. Dependency check..."
    if command -v cargo-deny &> /dev/null; then
        cargo deny check bans 2>/dev/null || log_warning "Dependency check failed or configuration incomplete"
    else
        log_warning "cargo-deny not installed, skipping dependency check"
    fi
    
    log_success "Security checks completed"
}

# Generate test coverage report
coverage_report() {
    log_info "Generating test coverage report..."
    
    if command -v cargo-llvm-cov &> /dev/null; then
        cargo llvm-cov --all-features --workspace --html
        
        log_success "Coverage report generated: target/llvm-cov/html/index.html"
        
        # Auto-open report on macOS
        if [[ "$OSTYPE" == "darwin"* ]]; then
            open target/llvm-cov/html/index.html
        fi
    else
        log_warning "cargo-llvm-cov not installed, using basic test coverage"
        log_info "Running tests with basic information..."
        cargo test --all-features
        log_info "For detailed coverage report, install: cargo install cargo-llvm-cov"
    fi
}

# Release preparation
prepare_release() {
    local version="$1"
    
    if [ -z "$version" ]; then
        log_error "Please provide version number, example: ./scripts/dev-workflow.sh release 0.1.3"
        exit 1
    fi
    
    log_info "Preparing release version $version..."
    
    # 1. Run complete CI checks
    log_info "1. Running complete CI checks..."
    quick_check
    full_test
    security_check
    
    # 2. Update version number
    log_info "2. Updating version number..."
    sed -i.bak "s/^version = \".*\"/version = \"$version\"/" Cargo.toml
    rm Cargo.toml.bak
    
    # 3. Update CHANGELOG
    log_info "3. Please manually update CHANGELOG.md, then press any key to continue..."
    read -n 1 -s
    
    # 4. Commit changes
    log_info "4. Committing version update..."
    git add Cargo.toml CHANGELOG.md
    git commit -m "Bump version to $version"
    
    # 5. Create tag
    log_info "5. Creating git tag..."
    git tag "v$version"
    
    log_success "Version $version prepared successfully!"
    log_info "Run the following command to publish:"
    echo "  git push origin main --tags"
}

# Publish to crates.io (manual trigger)
publish_crate() {
    log_info "Publishing to crates.io..."
    
    log_warning "Make sure you have publish permissions for crates.io"
    log_info "Press y to continue, any other key to cancel..."
    read -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cargo publish --dry-run
        log_info "Dry-run completed, press y to continue with actual publish..."
        read -n 1 -r
        echo
        
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            cargo publish
            log_success "Successfully published to crates.io!"
        else
            log_info "Publish cancelled"
        fi
    else
        log_info "Publish cancelled"
    fi
}

# Check dependency updates
check_updates() {
    log_info "Checking dependency updates..."
    
    log_info "1. Checking outdated dependencies..."
    if command -v cargo-outdated &> /dev/null; then
        cargo outdated
    else
        log_warning "cargo-outdated not installed, manual check: cargo tree --duplicates"
        cargo tree --duplicates || true
    fi
    
    log_info "2. Checking security issues..."
    if command -v cargo-audit &> /dev/null; then
        cargo audit
    else
        log_warning "cargo-audit not installed, skipping security check"
    fi
    
    log_info "Tip: Use 'cargo update' to update dependencies"
}

# Development environment setup
setup_dev() {
    log_info "Setting up development environment..."
    
    # Install development tools
    check_dependencies
    
    # Create necessary directories
    mkdir -p scripts
    mkdir -p .vscode
    
    # Create VS Code settings
    if [ ! -f .vscode/settings.json ]; then
        cat > .vscode/settings.json << 'EOF'
{
    "rust-analyzer.cargo.allFeatures": true,
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.checkOnSave.allFeatures": true,
    "editor.formatOnSave": true,
    "editor.rulers": [100],
    "files.insertFinalNewline": true,
    "files.trimTrailingWhitespace": true
}
EOF
        log_success "VS Code configuration created"
    fi
    
    # Create Git hooks
    if [ ! -f .git/hooks/pre-commit ]; then
        cat > .git/hooks/pre-commit << 'EOF'
#!/bin/sh
# Run quick check before commit
exec ./scripts/dev-workflow.sh quick
EOF
        chmod +x .git/hooks/pre-commit
        log_success "Git pre-commit hook created"
    fi
    
    log_success "Development environment setup completed"
}

# Show help information
show_help() {
    cat << 'EOF'
OpenAI SDK Rust - Development Workflow Automation

Usage:
  ./scripts/dev-workflow.sh <command> [options]

Commands:
  setup              Setup development environment
  quick              Quick check (format, clippy, compile)
  test               Run full test suite
  security           Security checks (audit, licenses)
  coverage           Generate test coverage report
  updates            Check dependency updates
  release <version>  Prepare version release
  publish            Publish to crates.io
  help               Show this help information

Examples:
  ./scripts/dev-workflow.sh quick
  ./scripts/dev-workflow.sh release 0.1.3
  ./scripts/dev-workflow.sh coverage

EOF
}

# Main function
main() {
    local command="${1:-help}"
    
    case "$command" in
        "setup")
            setup_dev
            ;;
        "quick")
            # Skip dependency check, run quick check directly
            quick_check
            ;;
        "test")
            check_dependencies
            full_test
            ;;
        "security")
            check_dependencies
            security_check
            ;;
        "coverage")
            check_dependencies
            coverage_report
            ;;
        "updates")
            check_dependencies
            check_updates
            ;;
        "release")
            check_dependencies
            prepare_release "${2:-}"
            ;;
        "publish")
            check_dependencies
            publish_crate
            ;;
        "help"|"--help"|"-h")
            show_help
            ;;
        *)
            log_error "Unknown command: $command"
            echo
            show_help
            exit 1
            ;;
    esac
}

# Run main function
main "$@"
