#!/usr/bin/env bash

# Simplified quick check script - no additional tools required
# Usage: ./scripts/simple-check.sh

set -euo pipefail

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

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

# Simplified quick check
simple_check() {
    log_info "Running simplified quick check..."
    
    log_info "1. Checking code format..."
    if cargo fmt --all -- --check; then
        log_success "Code format is correct"
    else
        log_warning "Code format issues found, auto-fixing..."
        cargo fmt --all
        log_success "Code format fixed"
    fi
    
    log_info "2. Running clippy checks..."
    if cargo clippy --all-targets --all-features -- -D warnings; then
        log_success "Clippy checks passed"
    else
        log_error "Clippy checks failed, please fix warnings"
        return 1
    fi
    
    log_info "3. Quick compilation check..."
    if cargo check --all-targets --all-features; then
        log_success "Compilation check passed"
    else
        log_error "Compilation failed"
        return 1
    fi
    
    log_info "4. Running tests..."
    if cargo test --all-features; then
        log_success "Tests passed"
    else
        log_error "Tests failed"
        return 1
    fi
    
    log_success "All checks passed!"
}

# Show help
if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
    cat << 'EOF'
Simplified Quick Check Tool

This script doesn't require installing additional cargo tools, using only Rust built-in features:
- cargo fmt (code formatting)
- cargo clippy (code linting)  
- cargo check (compilation check)
- cargo test (run tests)

Usage:
  ./scripts/simple-check.sh

EOF
    exit 0
fi

simple_check
