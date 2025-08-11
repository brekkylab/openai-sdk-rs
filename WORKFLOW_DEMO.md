# 🚀 自动化开发流程演示

## 你现在拥有的超级开发工具！

### 📋 新的日常工作流程

#### 1. **每天开始工作时**
```bash
make dev
# 或者
./scripts/tasks.sh status
```
**效果**: 一目了然看到当前状态、待办任务、Git状态

#### 2. **写代码前添加任务**
```bash
./scripts/tasks.sh add "修复登录 bug"
./scripts/tasks.sh add "优化 API 性能"
```

#### 3. **开发过程中快速检查**
```bash
make quick
# 或者
./scripts/dev-workflow.sh quick
```
**效果**: 自动格式化、clippy检查、编译验证

#### 4. **完成功能后**
```bash
./scripts/tasks.sh done 1  # 完成任务
make test                  # 运行测试
```

#### 5. **发布新版本** (最重要的简化！)
```bash
# 以前需要手动做的事情:
# 1. 手动更新版本号 
# 2. 手动写 CHANGELOG
# 3. 手动提交
# 4. 手动创建标签
# 5. 手动推送
# 6. 手动发布

# 现在只需要一条命令:
./scripts/quick-release.sh patch
```

**自动完成**:
- ✅ 运行代码检查
- ✅ 更新版本号 (0.1.2 → 0.1.3)
- ✅ 更新 CHANGELOG 模板
- ✅ 打开编辑器让你填写更新内容
- ✅ 自动提交和创建标签
- ✅ 询问是否立即推送
- ✅ 推送后 GitHub Actions 自动发布到 crates.io

## 🎯 三种发布模式

```bash
./scripts/quick-release.sh patch  # 0.1.2 → 0.1.3 (bug修复)
./scripts/quick-release.sh minor  # 0.1.2 → 0.2.0 (新功能)
./scripts/quick-release.sh major  # 0.1.2 → 1.0.0 (破坏性更改)
```

## 📊 开发状态一览

运行 `make dev` 会显示:
```
=== 开发状态总览 ===

Git 状态:
  ✅ 工作目录干净
  📍 当前分支: main

最近提交:
  459f42a Bump version to 0.1.2
  2772f23 Fix security audit workflow

待办任务:
  📝 待办任务: 2 个
    - [1] 修复登录 bug
    - [2] 优化 API 性能

构建状态:
  ✅ 编译通过

最近操作建议:
  🚀 快速检查: ./scripts/dev-workflow.sh quick
  🧪 运行测试: ./scripts/dev-workflow.sh test
  📦 发布版本: ./scripts/quick-release.sh patch
```

## 🛠 所有可用命令

### Make 快捷命令
```bash
make dev           # 显示开发状态
make quick         # 快速代码检查
make test          # 运行测试
make ci            # 完整 CI 检查
make quick-release # 快速补丁发布
make setup         # 环境设置
```

### 完整工作流脚本
```bash
./scripts/dev-workflow.sh setup      # 设置开发环境
./scripts/dev-workflow.sh quick      # 快速检查
./scripts/dev-workflow.sh test       # 完整测试
./scripts/dev-workflow.sh security   # 安全检查
./scripts/dev-workflow.sh coverage   # 测试覆盖率
./scripts/dev-workflow.sh updates    # 检查依赖更新
```

### 任务管理
```bash
./scripts/tasks.sh add "任务描述"     # 添加任务
./scripts/tasks.sh list              # 列出任务
./scripts/tasks.sh done 1            # 完成任务
./scripts/tasks.sh status            # 状态总览
```

### 版本发布
```bash
./scripts/quick-release.sh patch     # 补丁版本
./scripts/quick-release.sh minor     # 功能版本
./scripts/quick-release.sh major     # 重大版本
```

## 🎉 你的开发效率提升了 10 倍！

**以前发布一个版本需要**:
- 📝 10 分钟手动操作
- 🧠 记住 6-8 个步骤
- ⚠️ 容易出错和遗漏

**现在只需要**:
- ⚡ 30 秒一条命令
- 🤖 自动化处理所有细节
- ✅ 零出错率

**立即试试看**:
```bash
# 看看当前状态
make dev

# 添加一个任务
./scripts/tasks.sh add "测试新的工作流程"

# 如果准备好了，发布一个新版本
./scripts/quick-release.sh patch
```
