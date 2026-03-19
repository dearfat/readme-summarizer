#!/bin/bash

echo "=== 完整配置文件工作流测试 ==="
echo ""

# 清理旧配置
echo "1. 清理旧配置..."
rm -rf ~/.config/readme-summarizer
echo "✓ 配置目录已清理"
echo ""

# 第一次运行 - 使用 --source 参数
echo "2. 第一次运行（使用 --source 参数）..."
./target/release/readme-sum --source test-projects
echo ""

# 检查配置文件是否创建
echo "3. 检查配置文件..."
if [ -f ~/.config/readme-summarizer/config.toml ]; then
    echo "✓ 配置文件已创建"
    echo "配置内容："
    cat ~/.config/readme-summarizer/config.toml
    echo ""
else
    echo "✗ 配置文件未创建"
    echo ""
fi

# 第二次运行 - 不使用任何参数，应该使用保存的配置
echo "4. 第二次运行（不使用参数，应该使用保存的配置）..."
./target/release/readme-sum
echo ""

echo "=== 测试完成 ==="
