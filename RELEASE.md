# 发布指南

本文档说明如何发布 README Summarizer 的新版本。

## 发布前准备

### 1. 更新版本号

在 `Cargo.toml` 中更新版本号：

```toml
[package]
name = "readme-summarizer"
version = "1.0.0"  # 更新这里
```

### 2. 更新 CHANGELOG.md

在 `CHANGELOG.md` 中添加新版本的变更记录：

```markdown
## [1.0.0] - 2026-03-19

### Added
- 新功能列表

### Changed
- 修改的功能

### Fixed
- 修复的 bug
```

### 3. 测试构建

确保所有平台都能成功构建：

```bash
# 本地测试
cargo build --release
cargo test
cargo clippy

# 测试运行
./target/release/readme-sum --version
./target/release/readme-sum --help
```

## 发布流程

### 方法 1: 使用 GitHub Actions 自动发布（推荐）

1. **提交所有更改**
   ```bash
   git add .
   git commit -m "chore: prepare for v1.0.0 release"
   git push origin main
   ```

2. **创建并推送标签**
   ```bash
   git tag -a v1.0.0 -m "Release version 1.0.0"
   git push origin v1.0.0
   ```

3. **GitHub Actions 自动构建**
   - 推送标签后，GitHub Actions 会自动：
     - 为 Linux、macOS、Windows 构建二进制文件
     - 创建 GitHub Release
     - 上传所有平台的二进制文件

4. **编辑 Release 说明**
   - 访问 GitHub Releases 页面
   - 编辑自动创建的 release
   - 从 CHANGELOG.md 复制相关内容
   - 添加安装说明

### 方法 2: 手动发布

如果不使用 GitHub Actions，可以手动构建和发布：

#### Linux 构建

```bash
# 在 Linux 机器上
cargo build --release --target x86_64-unknown-linux-gnu
strip target/x86_64-unknown-linux-gnu/release/readme-sum
mv target/x86_64-unknown-linux-gnu/release/readme-sum readme-sum-linux
```

#### macOS 构建

```bash
# 在 macOS 机器上
cargo build --release --target x86_64-apple-darwin
strip target/x86_64-apple-darwin/release/readme-sum
mv target/x86_64-apple-darwin/release/readme-sum readme-sum-macos
```

#### Windows 构建

```bash
# 在 Windows 机器上
cargo build --release --target x86_64-pc-windows-msvc
move target\x86_64-pc-windows-msvc\release\readme-sum.exe readme-sum-windows.exe
```

#### 创建 GitHub Release

1. 访问 GitHub 仓库的 Releases 页面
2. 点击 "Draft a new release"
3. 选择标签或创建新标签（如 `v1.0.0`）
4. 填写 Release 标题和说明
5. 上传构建的二进制文件
6. 发布 Release

## 发布到 crates.io（可选）

如果想让用户通过 `cargo install` 安装：

1. **登录 crates.io**
   ```bash
   cargo login
   ```

2. **发布**
   ```bash
   cargo publish
   ```

3. **验证**
   ```bash
   cargo install readme-summarizer
   ```

## 发布后

1. **更新 README.md**
   - 将 `YOUR_USERNAME` 替换为实际的 GitHub 用户名
   - 更新下载链接

2. **通知用户**
   - 在 GitHub Discussions 或社交媒体上宣布新版本
   - 说明新功能和改进

3. **监控反馈**
   - 关注 GitHub Issues
   - 及时回复用户问题

## 版本号规范

遵循 [语义化版本](https://semver.org/lang/zh-CN/)：

- **主版本号 (MAJOR)**: 不兼容的 API 修改
- **次版本号 (MINOR)**: 向下兼容的功能性新增
- **修订号 (PATCH)**: 向下兼容的问题修正

示例：
- `1.0.0` - 首个稳定版本
- `1.1.0` - 添加新功能
- `1.1.1` - 修复 bug
- `2.0.0` - 重大更新，可能不兼容旧版本

## 回滚发布

如果发现严重问题需要回滚：

1. **删除 GitHub Release**
   - 在 Releases 页面删除有问题的版本

2. **删除标签**
   ```bash
   git tag -d v1.0.0
   git push origin :refs/tags/v1.0.0
   ```

3. **发布修复版本**
   - 修复问题
   - 发布新的修订版本（如 v1.0.1）
