#!/bin/bash
# 构建发布版本的脚本

set -e

echo "🔨 开始构建 README Summarizer..."
echo ""

# 清理旧的构建
echo "📦 清理旧的构建文件..."
cargo clean
echo ""

# 运行测试
echo "🧪 运行测试..."
cargo test
echo ""

# 运行代码检查
echo "🔍 运行代码检查..."
cargo clippy -- -D warnings
echo ""

# 格式化代码
echo "✨ 格式化代码..."
cargo fmt
echo ""

# 构建发布版本
echo "🚀 构建发布版本..."
cargo build --release
echo ""

# 显示二进制文件信息
echo "📊 二进制文件信息:"
ls -lh target/release/readme-sum
echo ""

# 测试二进制文件
echo "✅ 测试二进制文件..."
./target/release/readme-sum --version
./target/release/readme-sum --help
echo ""

echo "🎉 构建完成！"
echo "二进制文件位置: target/release/readme-sum"
echo ""
echo "下一步："
echo "1. 测试功能: ./target/release/readme-sum --source test-projects"
echo "2. 安装到系统: sudo cp target/release/readme-sum /usr/local/bin/"
