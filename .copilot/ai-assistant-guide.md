# AI Assistant Integration Guide

This document serves as a reference for AI assistants (like GitHub Copilot) to understand and effectively use the automation tools in this project.

## 🎯 Project Context

This is a Rust OpenAI SDK with comprehensive automation tools. When suggesting commands or workflows, always prefer the automated tools over manual commands.

## 🚀 Quick Reference for AI Assistants

### Instead of Manual Commands, Use Automation:

| ❌ Manual Approach           | ✅ Automated Approach               | Context           |
| --------------------------- | ---------------------------------- | ----------------- |
| `cargo fmt && cargo clippy` | `make quick`                       | Pre-commit checks |
| `cargo test --all-features` | `make test`                        | Running tests     |
| Manual version bumping      | `./scripts/quick-release.sh patch` | Releases          |
| Ad-hoc TODO comments        | `./scripts/tasks.sh add "task"`    | Task tracking     |
| Manual security checks      | `make security`                    | Security audits   |
| Manual coverage             | `make coverage`                    | Test coverage     |

### Common AI Assistant Scenarios

#### 1. When User Asks to "Run Tests"
```bash
# ✅ Prefer this:
make test

# Or if they want just quick validation:
make quick

# ❌ Instead of:
cargo test --all-features
```

#### 2. When User Wants to "Check Code Quality"
```bash
# ✅ Comprehensive check:
make quick

# ✅ Or via script:
./scripts/dev-workflow.sh quick

# ❌ Instead of:
cargo fmt --check && cargo clippy
```

#### 3. When User Wants to "Release New Version"
```bash
# ✅ Automated release:
./scripts/quick-release.sh patch

# ✅ For different types:
./scripts/quick-release.sh minor  # Feature release
./scripts/quick-release.sh major  # Breaking changes

# ❌ Instead of:
# Manual Cargo.toml editing + git tags
```

#### 4. When User Asks About "Project Status"
```bash
# ✅ Development overview:
./scripts/tasks.sh status

# ✅ Or via make:
make dev
```

#### 5. When User Wants to "Add a TODO"
```bash
# ✅ Tracked task:
./scripts/tasks.sh add "Implement error handling for API timeout"

# ❌ Instead of:
# Adding TODO comments in code
```

## 🔧 VS Code Integration

When suggesting VS Code workflows, recommend tasks over terminal commands:

### Accessing VS Code Tasks
1. Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on Mac)
2. Type "Tasks: Run Task"
3. Select from available tasks:
   - 🚀 Quick Check
   - 🧪 Full Test Suite
   - 🔒 Security Check
   - 📊 Coverage Report
   - 📦 Release Patch/Minor
   - 📝 Add Task
   - 📋 Task Status

## 🤖 AI Response Patterns

### When User Asks: "How do I test this?"
```
I'll help you run the tests using the project's automation tools:

For a quick check (recommended before commits):
`make quick`

For the full test suite:
`make test`

You can also use VS Code tasks by pressing Ctrl+Shift+P → "Tasks: Run Task" → "🧪 Full Test Suite"
```

### When User Asks: "How do I release a new version?"
```
This project has automated release tools. For a patch release:
`./scripts/quick-release.sh patch`

This will:
- Run all quality checks
- Update version numbers
- Update CHANGELOG.md
- Create git tags
- Guide you through the release process

For minor or major releases, use `minor` or `major` instead of `patch`.
```

### When User Asks: "How do I check code quality?"
```
Use the quick check automation:
`make quick`

This runs:
- Code formatting (cargo fmt)
- Linting (cargo clippy)
- Compilation checks
- Basic validation

Or use the VS Code task "🚀 Quick Check" for a GUI approach.
```

## 🛡️ Error Handling

The automation tools include graceful fallbacks:

### When cargo tools are missing:
```bash
# ✅ Always available:
./scripts/simple-check.sh

# ✅ Fallback mode in main script:
./scripts/dev-workflow.sh quick
```

### When jq is missing (for task management):
```bash
# Tasks fall back to simple markdown files
./scripts/tasks.sh add "task"  # Still works without jq
```

## 📝 Best Practices for AI Responses

1. **Always suggest automation first**
2. **Provide both make shortcuts and full script paths**
3. **Mention VS Code tasks for GUI users**
4. **Explain what the automation does**
5. **Use the project's established patterns**

### Example Good AI Response:
```
I'll help you set up the development environment. This project has automation tools:

1. First, set up the development environment:
   `make setup`

2. For daily development, use:
   `make quick` - Quick quality checks before commits
   `make test` - Full test suite

3. You can also use VS Code tasks (Ctrl+Shift+P → "Tasks: Run Task") for GUI access to these tools.

The automation handles formatting, linting, testing, and even releases automatically.
```

## 🔍 Project Structure Recognition

When analyzing the project, note these indicators of automation:

- `Makefile` with development shortcuts
- `scripts/` directory with `.sh` files
- `.vscode/tasks.json` with integrated tasks
- `DEVELOPMENT.md` with workflow documentation
- `.tasks.json` for task management (auto-created)

These indicate a project designed for automation-first development.
