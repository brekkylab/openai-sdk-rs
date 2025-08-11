#!/usr/bin/env bash

# Quick version release tool
# Usage: ./scripts/quick-release.sh [patch|minor|major]

set -euo pipefail

# Color definitions
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

# Get current version
get_current_version() {
    grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'
}

# Calculate next version
bump_version() {
    local current="$1"
    local bump_type="$2"
    
    # Parse version number (major.minor.patch)
    local major=$(echo "$current" | cut -d. -f1)
    local minor=$(echo "$current" | cut -d. -f2)
    local patch=$(echo "$current" | cut -d. -f3)
    
    case "$bump_type" in
        "patch")
            patch=$((patch + 1))
            ;;
        "minor")
            minor=$((minor + 1))
            patch=0
            ;;
        "major")
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        *)
            echo "Error: Invalid version type. Use patch, minor, or major"
            exit 1
            ;;
    esac
    
    echo "$major.$minor.$patch"
}

# Update CHANGELOG
update_changelog() {
    local version="$1"
    local date=$(date +%Y-%m-%d)
    
    # Create temporary file
    local temp_file=$(mktemp)
    
    # Add new version to CHANGELOG
    {
        echo "# Changelog"
        echo ""
        echo "All notable changes to this project will be documented in this file."
        echo ""
        echo "The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),"
        echo "and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)."
        echo ""
        echo "## [$version] - $date"
        echo ""
        echo "### Added"
        echo "- [Please add new features here]"
        echo ""
        echo "### Changed"
        echo "- [Please add changes here]"
        echo ""
        echo "### Fixed"
        echo "- [Please add fixes here]"
        echo ""
        # Skip first few header lines of original file, then add remaining content
        tail -n +9 CHANGELOG.md
    } > "$temp_file"
    
    mv "$temp_file" CHANGELOG.md
}

# Main function
main() {
    local bump_type="${1:-patch}"
    
    log_info "Starting quick version release process..."
    
    # 1. Check git status
    if [[ -n $(git status --porcelain) ]]; then
        log_warning "Working directory has uncommitted changes"
        log_info "Please commit or stage your changes first"
        exit 1
    fi
    
    # 2. Get current version and calculate new version
    local current_version=$(get_current_version)
    local new_version=$(bump_version "$current_version" "$bump_type")
    
    log_info "Current version: $current_version"
    log_info "New version: $new_version"
    
    # 3. Confirm release
    echo -n "Confirm release version $new_version? (y/N): "
    read -r confirm
    if [[ ! "$confirm" =~ ^[Yy]$ ]]; then
        log_info "Release cancelled"
        exit 0
    fi
    
    # 4. Run quick check
    log_info "Running quick check..."
    ./scripts/dev-workflow.sh quick
    
    # 5. Update version number
    log_info "Updating version number..."
    sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    rm Cargo.toml.bak
    
    # 6. Update CHANGELOG
    log_info "Updating CHANGELOG..."
    update_changelog "$new_version"
    
    # 7. Open editor for user to edit CHANGELOG
    log_info "Please edit CHANGELOG.md to add version update notes..."
    ${EDITOR:-vim} CHANGELOG.md
    
    # 8. Commit changes
    log_info "Committing version update..."
    git add Cargo.toml CHANGELOG.md
    git commit -m "Bump version to $new_version

Release notes:
- Please check CHANGELOG.md for detailed update content"
    
    # 9. Create tag
    log_info "Creating git tag..."
    git tag "v$new_version"
    
    # 10. Push
    echo -n "Push to remote repository immediately? (y/N): "
    read -r push_confirm
    if [[ "$push_confirm" =~ ^[Yy]$ ]]; then
        log_info "Pushing to remote repository..."
        git push origin main --tags
        log_success "Version $new_version released!"
        log_info "GitHub Actions will automatically build and publish to crates.io"
    else
        log_success "Version $new_version is ready!"
        log_info "Run the following command to complete release:"
        echo "  git push origin main --tags"
    fi
}

# Show help
if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
    cat << 'EOF'
Quick Version Release Tool

Usage:
  ./scripts/quick-release.sh [patch|minor|major]

Parameters:
  patch   Increment patch version (0.1.0 -> 0.1.1)
  minor   Increment minor version (0.1.0 -> 0.2.0)  
  major   Increment major version (0.1.0 -> 1.0.0)

Examples:
  ./scripts/quick-release.sh patch   # Release patch version
  ./scripts/quick-release.sh minor   # Release feature version
  ./scripts/quick-release.sh major   # Release major version

EOF
    exit 0
fi

main "$@"
