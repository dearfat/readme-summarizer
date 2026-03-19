# Homebrew 安装支持

## 方法 1: 创建自己的 Homebrew Tap（推荐）

### 步骤 1: 创建 Homebrew Tap 仓库

```bash
# 创建一个新的 GitHub 仓库，命名为 homebrew-tap
# 仓库名必须以 homebrew- 开头
```

### 步骤 2: 添加 Formula

```bash
# 克隆你的 tap 仓库
git clone https://github.com/YOUR_USERNAME/homebrew-tap
cd homebrew-tap

# 创建 Formula 目录
mkdir -p Formula

# 复制 formula 文件
cp /path/to/readme-summarizer/homebrew/readme-summarizer.rb Formula/
```

### 步骤 3: 更新 Formula

1. **获取 tarball 的 SHA256**

发布版本后，运行：
```bash
curl -L https://github.com/YOUR_USERNAME/readme-summarizer/archive/refs/tags/v1.0.0.tar.gz | shasum -a 256
```

2. **更新 Formula 文件**

将 `readme-summarizer.rb` 中的：
- `YOUR_USERNAME` 替换为你的 GitHub 用户名
- `REPLACE_WITH_ACTUAL_SHA256` 替换为实际的 SHA256 值
- 确保版本号正确

3. **提交并推送**
```bash
git add Formula/readme-summarizer.rb
git commit -m "Add readme-summarizer formula"
git push origin main
```

### 步骤 4: 用户安装

用户现在可以通过以下方式安装：

```bash
# 添加你的 tap
brew tap YOUR_USERNAME/tap

# 安装
brew install readme-summarizer

# 使用
readme-sum --version
```

### 步骤 5: 更新 Formula（发布新版本时）

```bash
cd homebrew-tap

# 编辑 Formula/readme-summarizer.rb
# 更新版本号和 SHA256

git add Formula/readme-summarizer.rb
git commit -m "Update readme-summarizer to v1.1.0"
git push origin main

# 用户更新
brew update
brew upgrade readme-summarizer
```

## 方法 2: 提交到官方 Homebrew Core（高级）

要提交到官方 Homebrew，需要满足以下条件：

1. **项目要求**
   - 项目稳定且维护良好
   - 有一定的用户基础（通常需要 75+ GitHub stars）
   - 遵循 Homebrew 的质量标准

2. **提交流程**

```bash
# 1. Fork homebrew-core
git clone https://github.com/Homebrew/homebrew-core

# 2. 创建新分支
cd homebrew-core
git checkout -b readme-summarizer

# 3. 创建 formula
brew create https://github.com/YOUR_USERNAME/readme-summarizer/archive/refs/tags/v1.0.0.tar.gz

# 4. 编辑 formula
# 编辑 Formula/readme-summarizer.rb

# 5. 测试 formula
brew install --build-from-source Formula/readme-summarizer.rb
brew test readme-summarizer
brew audit --new-formula readme-summarizer

# 6. 提交 PR
git add Formula/readme-summarizer.rb
git commit -m "readme-summarizer 1.0.0 (new formula)"
git push origin readme-summarizer
```

然后在 GitHub 上创建 Pull Request 到 homebrew-core。

## 方法 3: 使用 Homebrew Binary Formula（最简单）

如果你已经在 GitHub Releases 中提供了预编译的二进制文件：

```ruby
class ReadmeSummarizer < Formula
  desc "CLI tool to scan directories and generate summaries of README files"
  homepage "https://github.com/YOUR_USERNAME/readme-summarizer"
  version "1.0.0"
  license "MIT"

  if OS.mac?
    url "https://github.com/YOUR_USERNAME/readme-summarizer/releases/download/v1.0.0/readme-sum-macos"
    sha256 "REPLACE_WITH_MACOS_SHA256"
  elsif OS.linux?
    url "https://github.com/YOUR_USERNAME/readme-summarizer/releases/download/v1.0.0/readme-sum-linux"
    sha256 "REPLACE_WITH_LINUX_SHA256"
  end

  def install
    bin.install "readme-sum-macos" => "readme-sum" if OS.mac?
    bin.install "readme-sum-linux" => "readme-sum" if OS.linux?
  end

  test do
    system "#{bin}/readme-sum", "--version"
  end
end
```

## 自动化 Formula 更新

创建一个脚本来自动更新 formula：

```bash
#!/bin/bash
# scripts/update-homebrew-formula.sh

VERSION=$1
MACOS_SHA=$(curl -L https://github.com/YOUR_USERNAME/readme-summarizer/releases/download/v${VERSION}/readme-sum-macos | shasum -a 256 | cut -d' ' -f1)
LINUX_SHA=$(curl -L https://github.com/YOUR_USERNAME/readme-summarizer/releases/download/v${VERSION}/readme-sum-linux | shasum -a 256 | cut -d' ' -f1)

cat > Formula/readme-summarizer.rb <<EOF
class ReadmeSummarizer < Formula
  desc "CLI tool to scan directories and generate summaries of README files"
  homepage "https://github.com/YOUR_USERNAME/readme-summarizer"
  version "${VERSION}"
  license "MIT"

  if OS.mac?
    url "https://github.com/YOUR_USERNAME/readme-summarizer/releases/download/v${VERSION}/readme-sum-macos"
    sha256 "${MACOS_SHA}"
  elsif OS.linux?
    url "https://github.com/YOUR_USERNAME/readme-summarizer/releases/download/v${VERSION}/readme-sum-linux"
    sha256 "${LINUX_SHA}"
  end

  def install
    bin.install "readme-sum-macos" => "readme-sum" if OS.mac?
    bin.install "readme-sum-linux" => "readme-sum" if OS.linux?
  end

  test do
    system "#{bin}/readme-sum", "--version"
  end
end
EOF

echo "Formula updated for version ${VERSION}"
```

## 推荐方案

**对于新项目，推荐使用方法 1（创建自己的 Tap）**：

优点：
- 完全控制发布流程
- 无需等待审核
- 可以快速迭代
- 适合早期项目

步骤：
1. 创建 `homebrew-tap` 仓库
2. 添加 formula
3. 在 README 中告诉用户如何安装

用户安装：
```bash
brew tap YOUR_USERNAME/tap
brew install readme-summarizer
```

## 更新 README

在项目 README 中添加 Homebrew 安装说明：

```markdown
### 方法 4: 使用 Homebrew（macOS/Linux）

\`\`\`bash
brew tap YOUR_USERNAME/tap
brew install readme-summarizer
\`\`\`
```
