# 贡献指南

感谢你对 README Summarizer 的关注！我们欢迎任何形式的贡献。

## 如何贡献

### 报告 Bug

如果你发现了 bug，请在 GitHub Issues 中创建一个新的 issue，并包含以下信息：

- 问题的详细描述
- 复现步骤
- 预期行为
- 实际行为
- 你的操作系统和版本
- 工具的版本（`readme-sum --version`）

### 提出新功能

如果你有新功能的想法，请先在 GitHub Issues 中创建一个 feature request，描述：

- 功能的用途
- 为什么需要这个功能
- 如何使用这个功能

### 提交代码

1. Fork 这个仓库
2. 创建你的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交你的修改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建一个 Pull Request

### 代码规范

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 确保所有测试通过 (`cargo test`)
- 为新功能添加适当的文档

### 开发环境设置

```bash
# 克隆仓库
git clone https://github.com/YOUR_USERNAME/readme-summarizer
cd readme-summarizer

# 安装依赖并构建
cargo build

# 运行测试
cargo test

# 运行程序
cargo run -- --help
```

## 行为准则

请保持友好和尊重。我们致力于为所有人提供一个无骚扰的体验。

## 问题？

如果你有任何问题，请随时在 Issues 中提问。
