#!/bin/bash
# 准备发布的脚本

set -e

# 检查参数
if [ -z "$1" ]; then
    echo "用法: $0 <version>"
    echo "示例: $0 1.0.0"
    exit 1
fi

VERSION=$1
TAG="v${VERSION}"

echo "🚀 准备发布版本 ${VERSION}..."
echo ""

# 检查是否有未提交的更改
if [ -n "$(git status --porcelain)" ]; then
    echo "❌ 错误: 有未提交的更改，请先提交所有更改"
    git status --short
    exit 1
fi

# 更新 Cargo.toml 中的版本号
echo "📝 更新 Cargo.toml 中的版本号..."
sed -i.bak "s/^version = \".*\"/version = \"${VERSION}\"/" Cargo.toml
rm Cargo.toml.bak
echo ""

# 构建并测试
echo "🔨 构建并测试..."
cargo build --release
cargo test
echo ""

# 提交版本更改
echo "💾 提交版本更改..."
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to ${VERSION}"
echo ""

# 创建标签
echo "🏷️  创建标签 ${TAG}..."
git tag -a "${TAG}" -m "Release version ${VERSION}"
echo ""

echo "✅ 准备完成！"
echo ""
echo "下一步："
echo "1. 推送提交: git push origin main"
echo "2. 推送标签: git push origin ${TAG}"
echo "3. GitHub Actions 将自动构建并创建 Release"
echo ""
echo "如果需要回滚:"
echo "  git tag -d ${TAG}"
echo "  git reset --hard HEAD~1"
