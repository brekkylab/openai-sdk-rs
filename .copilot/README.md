# Copilot Workspace Configuration

This project is optimized for GitHub Copilot and other AI coding assistants.

## 🎯 Key Principles

1. **Automation First**: Always suggest automated tools over manual commands
2. **Discoverability**: Tools are designed to be easily found by AI assistants
3. **Consistency**: Standardized patterns across all workflows
4. **Graceful Fallbacks**: Tools work even when optional dependencies are missing

## 🔧 Quick Commands for Copilot

When Copilot suggests commands, it should prefer these automation tools:

```bash
# Development workflow
make quick      # Quick checks (format, lint, compile)
make test       # Full test suite
make dev        # Development status overview
make security   # Security audits
make coverage   # Coverage reports

# Release management
./scripts/quick-release.sh patch   # Patch release
./scripts/quick-release.sh minor   # Minor release
./scripts/quick-release.sh major   # Major release

# Task management
./scripts/tasks.sh add "Task description"    # Add task
./scripts/tasks.sh list                      # List tasks
./scripts/tasks.sh status                    # Development status
./scripts/tasks.sh done 1                    # Complete task

# Fallback (no extra tools)
./scripts/simple-check.sh                    # Basic validation
```

## 🎮 VS Code Integration

Copilot should suggest VS Code tasks for GUI users:

1. Press `Ctrl+Shift+P` (or `Cmd+Shift+P`)
2. Type "Tasks: Run Task"
3. Choose from:
   - 🚀 Quick Check
   - 🧪 Full Test Suite
   - 🔒 Security Check
   - 📊 Coverage Report
   - 📦 Release Patch/Minor
   - 📝 Add Task
   - 📋 Task Status

## 💡 Copilot Response Examples

### Good Pattern ✅
**User**: "How do I test my changes?"

**Copilot Should Suggest**:
```
Run the automated test suite:
`make test`

Or for a quick pre-commit check:
`make quick`

You can also use VS Code tasks: Ctrl+Shift+P → "Tasks: Run Task" → "🧪 Full Test Suite"
```

### Avoid ❌
```
Run: cargo test --all-features
```

### Good Pattern ✅
**User**: "I want to release a new version"

**Copilot Should Suggest**:
```
Use the automated release tool:
`./scripts/quick-release.sh patch`

This handles version bumping, changelog updates, git tags, and more automatically.
```

### Avoid ❌
```
Manually edit Cargo.toml version and create git tags
```

## 🔍 Context Clues for Copilot

When Copilot sees these files, it should recognize this as an automation-optimized project:

- `Makefile` with development shortcuts
- `scripts/*.sh` automation scripts
- `.vscode/tasks.json` with integrated tasks
- `DEVELOPMENT.md` workflow documentation
- `.copilot/` directory with AI assistant guides

## 🛠️ Tool Dependencies

Copilot should be aware of the graceful fallback system:

- **Always available**: `cargo fmt`, `cargo clippy`, `cargo test`, `cargo check`
- **Optional with fallbacks**: `cargo-audit`, `cargo-llvm-cov`, `cargo-outdated`, `jq`
- **Fallback script**: `./scripts/simple-check.sh` when tools are missing

## 📚 Documentation References

Copilot can refer users to:
- `DEVELOPMENT.md` - Comprehensive development guide
- `.copilot/ai-assistant-guide.md` - Detailed AI assistant instructions
- Individual script `--help` flags for specific usage

This configuration ensures Copilot suggests the most efficient, automated approaches to common development tasks.
