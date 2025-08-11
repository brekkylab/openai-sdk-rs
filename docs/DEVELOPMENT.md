# 开发流程自动化指南

这个项目包含了完整的开发流程自动化工具，让日常开发变得更加高效。

## 🚀 快速开始

### 1. 设置开发环境
```bash
# 安装开发工具和配置环境
make setup
# 或者
./scripts/dev-workflow.sh setup
```

### 2. 日常开发循环

```bash
# 查看当前开发状态
make dev

# 快速检查代码 (格式、clippy、编译)
make quick

# 运行完整测试
make test

# 添加开发任务
./scripts/tasks.sh add "实现新功能"

# 查看任务列表
./scripts/tasks.sh list

# 完成任务
./scripts/tasks.sh done 1
```

### 3. 版本发布

```bash
# 快速发布补丁版本 (0.1.2 -> 0.1.3)
make quick-release
# 或者
./scripts/quick-release.sh patch

# 发布功能版本 (0.1.2 -> 0.2.0)  
./scripts/quick-release.sh minor

# 发布重大版本 (0.1.2 -> 1.0.0)
./scripts/quick-release.sh major
```

## 📜 脚本详解

### `scripts/dev-workflow.sh` - 主要开发工作流

这是核心的开发工作流脚本，提供以下功能：

```bash
# 设置开发环境（安装工具、创建配置等）
./scripts/dev-workflow.sh setup

# 快速检查（格式、clippy、编译）
./scripts/dev-workflow.sh quick

# 完整测试套件
./scripts/dev-workflow.sh test

# 安全检查
./scripts/dev-workflow.sh security

# 生成测试覆盖率报告
./scripts/dev-workflow.sh coverage

# 检查依赖更新
./scripts/dev-workflow.sh updates

# 准备版本发布
./scripts/dev-workflow.sh release 0.1.3

# 手动发布到 crates.io
./scripts/dev-workflow.sh publish
```

### `scripts/quick-release.sh` - 快速版本发布

简化版本发布流程：

```bash
# 发布补丁版本（bug 修复）
./scripts/quick-release.sh patch

# 发布功能版本（新功能，向后兼容）
./scripts/quick-release.sh minor

# 发布重大版本（破坏性更改）
./scripts/quick-release.sh major
```

脚本会自动：
1. 运行快速检查确保代码质量
2. 更新版本号
3. 更新 CHANGELOG
4. 提交更改并创建 git 标签
5. 可选择立即推送触发 CI/CD

### `scripts/tasks.sh` - 任务管理

简单的任务管理工具：

```bash
# 添加任务
./scripts/tasks.sh add "实现 WebSocket 支持"

# 列出所有任务
./scripts/tasks.sh list

# 完成任务
./scripts/tasks.sh done 1

# 查看开发状态总览
./scripts/tasks.sh status
```

## 🔄 完整的开发流程

### 日常开发

1. **开始工作**
   ```bash
   make dev  # 查看当前状态
   ```

2. **添加任务**
   ```bash
   ./scripts/tasks.sh add "修复登录错误"
   ```

3. **快速检查**
   ```bash
   make quick  # 在编写代码前后运行
   ```

4. **运行测试**
   ```bash
   make test  # 确保没有破坏现有功能
   ```

5. **完成任务**
   ```bash
   ./scripts/tasks.sh done 1
   ```

### 发布新版本

1. **确保所有测试通过**
   ```bash
   make ci  # 运行完整 CI 检查
   ```

2. **发布版本**
   ```bash
   ./scripts/quick-release.sh patch  # 或 minor/major
   ```

3. **脚本会自动处理**：
   - 更新版本号
   - 更新 CHANGELOG  
   - 提交更改
   - 创建 git 标签
   - 推送到远程仓库

4. **GitHub Actions 自动执行**：
   - 运行完整测试套件
   - 发布到 crates.io
   - 创建 GitHub Release
   - 部署文档

## 🛠 自定义配置

### Git Hooks

脚本会自动创建 pre-commit hook，在每次提交前运行快速检查：

```bash
# .git/hooks/pre-commit
#!/bin/sh
exec ./scripts/dev-workflow.sh quick
```

### VS Code 配置

自动创建 `.vscode/settings.json` 配置：

```json
{
    "rust-analyzer.cargo.allFeatures": true,
    "rust-analyzer.checkOnSave.command": "clippy",
    "editor.formatOnSave": true,
    "files.insertFinalNewline": true
}
```

### 环境变量

可以通过环境变量自定义行为：

```bash
# 自定义编辑器（用于编辑 CHANGELOG）
export EDITOR=code  # 使用 VS Code
export EDITOR=vim   # 使用 Vim

# 跳过某些检查（仅在必要时使用）
export SKIP_TESTS=1
```

## 📊 监控和报告

### 测试覆盖率

```bash
make coverage
# 生成 HTML 报告并自动打开
```

### 依赖检查

```bash
./scripts/dev-workflow.sh updates
# 检查过期依赖和安全问题
```

### 开发状态

```bash
make dev
# 显示：
# - Git 状态
# - 待办任务  
# - 构建状态
# - 推荐操作
```

## 🚨 故障排除

### 常见问题

1. **脚本权限错误**
   ```bash
   chmod +x scripts/*.sh
   ```

2. **缺少 jq 工具**
   ```bash
   brew install jq  # macOS
   apt install jq   # Ubuntu
   ```

3. **Rust 工具缺失**
   ```bash
   make install-tools
   ```

### 手动恢复

如果自动化出现问题，可以手动执行：

```bash
# 重置到上一个提交
git reset --hard HEAD~1

# 删除标签
git tag -d v0.1.3
git push origin :refs/tags/v0.1.3

# 重新开始发布流程
./scripts/quick-release.sh patch
```

## 🎯 最佳实践

1. **频繁运行快速检查**
   ```bash
   make quick  # 在编写代码时经常运行
   ```

2. **使用任务管理**
   ```bash
   # 开始工作前添加任务
   ./scripts/tasks.sh add "优化性能"
   ```

3. **定期检查状态**
   ```bash
   make dev  # 每天开始工作时查看
   ```

4. **小版本频繁发布**
   ```bash
   # 优先使用 patch 版本
   ./scripts/quick-release.sh patch
   ```

5. **保持 CHANGELOG 更新**
   - 每次发布时认真填写变更内容
   - 遵循 [Keep a Changelog](https://keepachangelog.com/) 格式

这套工具链将让你的 Rust 项目开发变得更加高效和规范！
