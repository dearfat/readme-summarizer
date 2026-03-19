# 发布检查清单

在发布新版本之前，请确保完成以下所有步骤：

## 📋 发布前检查

### 代码质量
- [ ] 所有测试通过 (`cargo test`)
- [ ] 代码检查无警告 (`cargo clippy`)
- [ ] 代码已格式化 (`cargo fmt`)
- [ ] 没有未使用的依赖
- [ ] 文档是最新的

### 版本信息
- [ ] 更新 `Cargo.toml` 中的版本号
- [ ] 更新 `CHANGELOG.md` 添加新版本的变更记录
- [ ] 确认版本号遵循语义化版本规范

### 文档更新
- [ ] README.md 中的示例是最新的
- [ ] 所有新功能都有文档说明
- [ ] 安装说明是准确的
- [ ] 更新 GitHub 用户名（将 `YOUR_USERNAME` 替换为实际用户名）

### 功能测试
- [ ] 基本功能测试通过
- [ ] `watch` 命令正常工作
- [ ] `sync-obsidian` 命令正常工作
- [ ] `config` 命令所有子命令正常工作
- [ ] 配置文件正确创建和读取
- [ ] 跨平台路径处理正确

### Git 准备
- [ ] 所有更改已提交
- [ ] 工作目录干净 (`git status`)
- [ ] 在正确的分支上（通常是 `main` 或 `master`）

## 🚀 发布步骤

### 使用自动化脚本（推荐）

```bash
# 1. 构建和测试
./scripts/build-release.sh

# 2. 准备发布（自动更新版本号、创建标签）
./scripts/prepare-release.sh 1.0.0

# 3. 推送到 GitHub
git push origin main
git push origin v1.0.0

# 4. GitHub Actions 会自动构建并创建 Release
```

### 手动发布

```bash
# 1. 更新版本号
# 编辑 Cargo.toml，修改 version = "1.0.0"

# 2. 构建和测试
cargo build --release
cargo test

# 3. 提交更改
git add .
git commit -m "chore: prepare for v1.0.0 release"

# 4. 创建标签
git tag -a v1.0.0 -m "Release version 1.0.0"

# 5. 推送
git push origin main
git push origin v1.0.0
```

## 📦 发布后检查

- [ ] GitHub Release 已自动创建
- [ ] 所有平台的二进制文件已上传（Linux、macOS、Windows）
- [ ] Release 说明已编辑（从 CHANGELOG 复制）
- [ ] 下载链接可以正常工作
- [ ] 安装说明已测试

## 🔄 可选：发布到 crates.io

如果要发布到 crates.io：

```bash
# 1. 登录
cargo login

# 2. 发布
cargo publish

# 3. 验证
cargo install readme-summarizer
readme-sum --version
```

检查清单：
- [ ] crates.io 账号已设置
- [ ] 包名可用（未被占用）
- [ ] 所有依赖都来自 crates.io
- [ ] README.md 在 crates.io 上显示正确

## 📢 发布后任务

- [ ] 在 GitHub Discussions 宣布新版本
- [ ] 更新项目主页（如果有）
- [ ] 在社交媒体分享（可选）
- [ ] 监控 GitHub Issues 的反馈
- [ ] 准备下一个版本的规划

## ⚠️ 常见问题

### GitHub Actions 构建失败
- 检查 `.github/workflows/release.yml` 配置
- 查看 Actions 日志找出错误原因
- 确保所有依赖都能在 CI 环境中构建

### 二进制文件无法运行
- 确保目标平台正确
- 检查是否需要额外的系统依赖
- 验证文件权限（Linux/macOS 需要执行权限）

### 版本号冲突
- 确保新版本号大于当前版本
- 检查 crates.io 上是否已存在该版本
- 遵循语义化版本规范

## 📝 版本号规范

- **1.0.0** - 首个稳定版本
- **1.1.0** - 添加新功能（向下兼容）
- **1.1.1** - 修复 bug（向下兼容）
- **2.0.0** - 重大更新（可能不兼容）

## 🆘 需要帮助？

如果遇到问题：
1. 查看 `RELEASE.md` 获取详细说明
2. 检查 GitHub Actions 日志
3. 在 GitHub Issues 提问
