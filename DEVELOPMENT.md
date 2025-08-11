# OpenAI SDK Rust - Development Automation

This project includes comprehensive development automation tools to streamline the development workflow.

## 🤖 AI Assistant Integration

This project is optimized for GitHub Copilot and other coding assistants. The automation tools are designed to be easily discoverable and usable by AI agents.

## 🛠️ Available Automation Tools

### Quick Commands

| Command         | Description             | Usage                            |
| --------------- | ----------------------- | -------------------------------- |
| `make quick`    | Quick development check | Format, clippy, compile          |
| `make test`     | Full test suite         | All tests with coverage          |
| `make security` | Security checks         | Audit, licenses, vulnerabilities |
| `make coverage` | Coverage report         | Generate HTML coverage report    |
| `make release`  | Prepare release         | Interactive version bump         |

### Development Scripts

#### 1. Main Workflow Script (`./scripts/dev-workflow.sh`)
```bash
# Quick checks (recommended for pre-commit)
./scripts/dev-workflow.sh quick

# Full test suite
./scripts/dev-workflow.sh test

# Security audits
./scripts/dev-workflow.sh security

# Coverage report
./scripts/dev-workflow.sh coverage

# Prepare release
./scripts/dev-workflow.sh release 0.2.1
```

#### 2. Quick Release Tool (`./scripts/quick-release.sh`)
```bash
# Patch release (0.1.0 -> 0.1.1)
./scripts/quick-release.sh patch

# Minor release (0.1.0 -> 0.2.0)
./scripts/quick-release.sh minor

# Major release (0.1.0 -> 1.0.0)
./scripts/quick-release.sh major
```

#### 3. Task Management (`./scripts/tasks.sh`)
```bash
# Add development task
./scripts/tasks.sh add "Implement new API endpoint"

# List all tasks
./scripts/tasks.sh list

# Mark task as complete
./scripts/tasks.sh done 1

# Show development status
./scripts/tasks.sh status
```

#### 4. Simple Check (`./scripts/simple-check.sh`)
```bash
# Tool-free validation (when cargo tools unavailable)
./scripts/simple-check.sh
```

## 🎯 VS Code Integration

### Tasks (Ctrl+Shift+P → "Tasks: Run Task")
- 🚀 Quick Check
- 🧪 Full Test Suite
- 🔒 Security Check
- 📊 Coverage Report
- 📦 Release Patch/Minor
- 📝 Add Task
- 📋 Task Status

### Recommended Workflow

1. **Before coding**: `make quick` or use VS Code task "🚀 Quick Check"
2. **Before commit**: `./scripts/dev-workflow.sh quick`
3. **Before push**: `make test` or "🧪 Full Test Suite"
4. **Weekly**: `make security` or "🔒 Security Check"
5. **Release**: `./scripts/quick-release.sh patch`

## 🤖 For AI Assistants

When working with this project, AI assistants should:

1. **Use automation tools** instead of manual commands
2. **Suggest appropriate scripts** based on context
3. **Recommend VS Code tasks** for common operations
4. **Follow the established workflow** patterns

### Common AI Assistant Patterns

```bash
# Instead of manual testing:
# ❌ cargo test --all-features
# ✅ make test  (or ./scripts/dev-workflow.sh test)

# Instead of manual checks:
# ❌ cargo fmt && cargo clippy
# ✅ make quick  (or ./scripts/dev-workflow.sh quick)

# Instead of manual releases:
# ❌ Manual version bumping + git tags
# ✅ ./scripts/quick-release.sh patch

# Instead of ad-hoc task tracking:
# ❌ Comments in code
# ✅ ./scripts/tasks.sh add "Task description"
```

## 🔧 Tool Dependencies

### Essential (always available)
- `cargo fmt` - Code formatting
- `cargo clippy` - Linting
- `cargo test` - Testing
- `cargo check` - Compilation check

### Optional (graceful fallbacks)
- `cargo-audit` - Security auditing
- `cargo-llvm-cov` - Coverage reporting
- `cargo-outdated` - Dependency updates
- `cargo-deny` - License/ban checking
- `jq` - Enhanced task management

## 📁 Project Structure

```
scripts/
├── dev-workflow.sh     # Main automation script
├── quick-release.sh    # Version release automation
├── tasks.sh           # Task management
└── simple-check.sh    # Tool-free validation

.vscode/
├── settings.json      # AI-optimized VS Code settings
└── tasks.json        # Integrated task definitions

Makefile              # Quick command shortcuts
.tasks.json          # Task storage (auto-created)
```

## 🎖️ Best Practices for AI Integration

1. **Always suggest automation** over manual commands
2. **Use descriptive task names** with emojis for clarity
3. **Prefer make shortcuts** for common operations
4. **Suggest VS Code tasks** for GUI users
5. **Follow semantic versioning** for releases
6. **Use task management** for tracking work
7. **Run security checks** before releases

This automation framework ensures consistent, efficient development while being easily discoverable by AI coding assistants.
