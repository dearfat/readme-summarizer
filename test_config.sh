#!/bin/bash

echo "=== 测试配置文件创建和使用 ==="
echo ""

# 清理旧配置
echo "1. 清理旧配置..."
rm -rf ~/.config/readme-summarizer
echo "✓ 配置目录已清理"
echo ""

# 第一次运行 - 应该提示输入源目录
echo "2. 第一次运行（输入源目录）..."
echo "test-projects" | ./target/release/readme-sum
echo ""

# 检查配置文件是否创建
echo "3. 检查配置文件..."
if [ -f ~/.config/readme-summarizer/config.toml ]; then
    echo "✓ 配置文件已创建"
    echo "配置内容："
    cat ~/.config/readme-summarizer/config.toml
else
    echo "✗ 配置文件未创建"
fi
echo ""

# 第二次运行 - 应该直接使用保存的配置
echo "4. 第二次运行（应该使用保存的配置）..."
./target/release/readme-sum
echo ""

echo "=== 测试完成 ==="
