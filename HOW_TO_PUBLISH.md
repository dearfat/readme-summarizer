# 完整发布指南

本文档提供了将 README Summarizer 发布给其他用户使用的完整步骤。

## 📦 支持的安装方式总览

1. **GitHub Releases** - 预编译二进制文件（自动化）
2. **Homebrew** - macOS/Linux 包管理器
3. **Cargo** - Rust 包管理器
4. **源码构建** - 从源代码编译

---

## 🚀 快速发布流程（5分钟）

### 1. 准备 GitHub 仓库

```bash
cd /Users/qwerdey/work/git-readmes

# 初始化 git（如果还没有）
git init
git add .
git commit -m "Initial commit: README Summarizer v1.0.0"

# 创建 GitHub 仓库并推送
# 访问 https://github.com/new 创建仓库
git remote add origin https://github.com/YOUR_USERNAME/readme-summarizer.git
git branch -M main
git push -u origin main
```

### 2. 更新配置

**重要：** 将所有文件中的 `YOUR_USERNAME` 替换为你的 GitHub 用户名：

```bash
# 批量替换（macOS/Linux）
find . -type f \( -name "*.md" -o -name "*.toml" -o -name "*.rb" \) -exec sed -i '' 's/YOUR_USERNAME/你的用户名/g' {} +

# 或者手动编辑以下文件：
# - Cargo.toml
# - README.md
# - homebrew/readme-summarizer.rb
# - HOMEBREW_SETUP.md
```

### 3. 创建第一个发布

```bash
# 使用自动化脚本
./scripts/prepare-release.sh 1.0.0

# 推送到 GitHub
git push origin main
git push origin v1.0.0
```

### 4. 等待自动构建

- 访问 `https://github.com/YOUR_USERNAME/readme-summarizer/actions`
- 等待 GitHub Actions 完成构建（约 5-10 分钟）
- 构建完成后会自动创建 Release

### 5. 完善 Release 说明

1. 访问 `https://github.com/YOUR_USERNAME/readme-summarizer/releases`
2. 编辑自动创建的 Release
3. 从 `CHANGELOG.md` 复制内容
4. 发布！

---

## 📋 详细发布步骤

### 方式 1: GitHub Releases（已自动化）✅

**优点：**
- 自动构建多平台二进制文件
- 用户无需安装 Rust
- 下载即用

**配置状态：** ✅ 已完成
- `.github/workflows/release.yml` 已配置
- 支持 Linux、macOS、Windows

**用户安装：**
```bash
# macOS
curl -L https://github.com/YOUR_USERNAME/readme-summarizer/releases/latest/download/readme-sum-macos -o readme-sum
chmod +x readme-sum
sudo mv readme-sum /usr/local/bin/

# Linux
curl -L https://github.com/YOUR_USERNAME/readme-summarizer/releases/latest/download/readme-sum-linux -o readme-sum
chmod +x readme-sum
sudo mv readme-sum /usr/local/bin/
```

---

### 方式 2: Homebrew 安装 🍺

**优点：**
- macOS/Linux 用户熟悉的安装方式
- 自动管理依赖和更新
- 一条命令安装

**步骤：**

#### 2.1 创建 Homebrew Tap 仓库

```bash
# 1. 在 GitHub 创建新仓库，命名为 homebrew-tap
# 2. 克隆仓库
git clone https://github.com/YOUR_USERNAME/homebrew-tap
cd homebrew-tap

# 3. 创建 Formula 目录
mkdir -p Formula

# 4. 复制 formula 文件
cp /Users/qwerdey/work/git-readmes/homebrew/readme-summarizer.rb Formula/
```

#### 2.2 更新 Formula

发布版本后，获取 SHA256：

```bash
# 获取 tarball 的 SHA256
curl -L https://github.com/YOUR_USERNAME/readme-summarizer/archive/refs/tags/v1.0.0.tar.gz | shasum -a 256
```

编辑 `Formula/readme-summarizer.rb`：
- 替换 `YOUR_USERNAME`
- 替换 `REPLACE_WITH_ACTUAL_SHA256`
- 确认版本号

#### 2.3 提交并发布

```bash
git add Formula/readme-summarizer.rb
git commit -m "Add readme-summarizer formula"
git push origin main
```

**用户安装：**
```bash
brew tap YOUR_USERNAME/tap
brew install readme-summarizer
```

**详细文档：** 查看 `HOMEBREW_SETUP.md`

---

### 方式 3: Cargo 安装 🦀

**优点：**
- Rust 用户的标准安装方式
- 自动处理依赖
- 跨平台支持

**步骤：**

#### 3.1 发布到 crates.io

```bash
# 1. 注册 crates.io 账号
# 访问 https://crates.io/ 并用 GitHub 登录

# 2. 获取 API token
# 访问 https://crates.io/me

# 3. 登录
cargo login

# 4. 发布
cargo publish
```

#### 3.2 验证发布

```bash
# 等待几分钟后
cargo install readme-summarizer
readme-sum --version
```

**用户安装：**
```bash
# 从 crates.io
cargo install readme-summarizer

# 从 GitHub
cargo install --git https://github.com/YOUR_USERNAME/readme-summarizer
```

**注意事项：**
- 包名必须唯一
- 版本号不能重复
- 遵循 crates.io 政策

---

### 方式 4: 源码构建 🔧

**优点：**
- 完全控制构建过程
- 可以自定义编译选项
- 适合开发者

**用户安装：**
```bash
git clone https://github.com/YOUR_USERNAME/readme-summarizer
cd readme-summarizer
cargo build --release
sudo cp target/release/readme-sum /usr/local/bin/
```

---

## 📊 安装方式对比

| 方式 | 难度 | 速度 | 适用人群 | 推荐度 |
|------|------|------|----------|--------|
| GitHub Releases | ⭐ | ⚡⚡⚡ | 所有用户 | ⭐⭐⭐⭐⭐ |
| Homebrew | ⭐⭐ | ⚡⚡ | macOS/Linux 用户 | ⭐⭐⭐⭐ |
| Cargo | ⭐⭐ | ⚡⚡ | Rust 开发者 | ⭐⭐⭐⭐ |
| 源码构建 | ⭐⭐⭐ | ⚡ | 开发者 | ⭐⭐⭐ |

---

## 🔄 版本更新流程

### 发布新版本

```bash
# 1. 更新版本号
# 编辑 Cargo.toml: version = "1.1.0"

# 2. 更新 CHANGELOG.md
# 添加新版本的变更记录

# 3. 提交更改
git add .
git commit -m "chore: bump version to 1.1.0"

# 4. 创建标签
git tag -a v1.1.0 -m "Release version 1.1.0"

# 5. 推送
git push origin main
git push origin v1.1.0

# 6. GitHub Actions 自动构建
# 7. 更新 Homebrew formula（如果使用）
# 8. 发布到 crates.io（如果使用）
cargo publish
```

---

## ✅ 发布检查清单

使用前请查看 `PUBLISH_CHECKLIST.md` 确保：

- [ ] 所有测试通过
- [ ] 代码已格式化
- [ ] 文档已更新
- [ ] 版本号已更新
- [ ] CHANGELOG 已更新
- [ ] 所有 `YOUR_USERNAME` 已替换
- [ ] GitHub Actions 配置正确

---

## 📚 相关文档

- **QUICKSTART.md** - 5分钟快速发布
- **RELEASE.md** - 详细发布流程
- **PUBLISH_CHECKLIST.md** - 发布检查清单
- **HOMEBREW_SETUP.md** - Homebrew 详细配置
- **CONTRIBUTING.md** - 贡献指南
- **CHANGELOG.md** - 版本历史

---

## 🆘 常见问题

### Q: GitHub Actions 构建失败怎么办？
A: 查看 Actions 日志，通常是依赖问题。确保 `Cargo.toml` 中的依赖都来自 crates.io。

### Q: crates.io 发布失败？
A: 检查包名是否已被占用，版本号是否重复，是否有未提交的更改。

### Q: Homebrew formula 不工作？
A: 确保 SHA256 正确，URL 可访问，版本号匹配。

### Q: 如何回滚发布？
A: 删除 GitHub Release 和标签，修复问题后重新发布。

---

## 🎯 推荐发布策略

**对于新项目：**
1. 先发布到 GitHub Releases（最简单）
2. 创建 Homebrew Tap（提升用户体验）
3. 发布到 crates.io（覆盖 Rust 用户）
4. 积累用户后考虑提交到 Homebrew Core

**版本规划：**
- v0.x.x - 开发版本
- v1.0.0 - 首个稳定版本
- v1.x.x - 功能更新
- v2.0.0 - 重大更新

---

## 📞 需要帮助？

- 查看 GitHub Issues
- 阅读 Rust 发布指南
- 参考 Homebrew 文档
- 查看 crates.io 指南
