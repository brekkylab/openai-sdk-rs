# AI Assistant Quick Reference

## 🎯 Context Recognition Patterns

When working with this project, AI assistants should recognize these patterns:

### Files that indicate automation-first project:
- `Makefile` with emoji-decorated commands
- `scripts/*.sh` automation scripts  
- `.vscode/tasks.json` with task definitions
- `DEVELOPMENT.md` comprehensive guide
- `.copilot/` directory with AI assistant guides

### Key automation commands to suggest:
```bash
make quick      # Instead of: cargo fmt && cargo clippy
make test       # Instead of: cargo test --all-features  
make security   # Instead of: cargo audit
make dev        # Instead of: git status (shows much more)
```

## 🤖 Copilot Response Templates

### User: "Run tests"
```
I'll run the comprehensive test suite using the project's automation:
`make test`

This runs all tests with proper error handling and reporting.
```

### User: "Check code quality"  
```
Let me run the automated quality checks:
`make quick`

This handles formatting, linting, and compilation checks automatically.
```

### User: "How do I release?"
```
This project has automated releases:
`make release`

Or for specific version types:
`./scripts/quick-release.sh [patch|minor|major]`
```

### User: "Add a TODO"
```
Use the task management system:
`./scripts/tasks.sh add "Your task description"`

Check status with: `make dev`
```

## 🔧 VS Code Integration

Suggest VS Code tasks for GUI users:
- Ctrl+Shift+P → "Tasks: Run Task" → Select automation
- Point out the 🤖 emoji indicates AI-optimized commands

## 🎪 Project Philosophy

This project prioritizes:
1. **Automation over manual commands**
2. **AI assistant discoverability** 
3. **Consistent workflows**
4. **Graceful fallbacks**

Always suggest the automated approach first!
