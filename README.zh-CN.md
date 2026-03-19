# README Summarizer

[English](README.md) | 中文

一个用 Rust 编写的命令行工具，用于自动扫描目录下的项目并生成 README 文件摘要。

## 功能特性

- 🔍 自动扫描指定目录下的所有子项目
- 📝 提取 README 文件中的关键信息（项目名称、功能、解决的问题）
- 💾 支持配置持久化，保存默认设置
- 📊 生成单个 Markdown 表格文件汇总所有项目
- 📁 表格包含可点击的项目目录链接
- 🎯 默认输出到源目录，便于查看
- 💬 首次运行交互式配置
- 👀 监控模式：自动检测文件变化并重新生成
- 📚 Obsidian 集成：一键同步到 Obsidian vault
- ⚡ 快速高效的本地解析，无需 API 调用

## 安装

### 方法 1: 从 GitHub Releases 下载（推荐）

访问 [Releases 页面](https://github.com/YOUR_USERNAME/readme-summarizer/releases) 下载适合你操作系统的预编译二进制文件：

**macOS**
```bash
# 下载并安装
curl -L https://github.com/YOUR_USERNAME/readme-summarizer/releases/latest/download/readme-sum-macos -o readme-sum
chmod +x readme-sum
sudo mv readme-sum /usr/local/bin/
```

**Linux**
```bash
# 下载并安装
curl -L https://github.com/YOUR_USERNAME/readme-summarizer/releases/latest/download/readme-sum-linux -o readme-sum
chmod +x readme-sum
sudo mv readme-sum /usr/local/bin/
```

**Windows**

下载 `readme-sum-windows.exe`，将其重命名为 `readme-sum.exe` 并添加到 PATH 环境变量中。

### 方法 2: 使用 Homebrew（macOS/Linux）

```bash
# 添加 tap
brew tap YOUR_USERNAME/tap

# 安装
brew install readme-summarizer
```

### 方法 3: 使用 Cargo 安装

如果你已经安装了 Rust 工具链：

```bash
# 从 GitHub 安装
cargo install --git https://github.com/YOUR_USERNAME/readme-summarizer

# 从 crates.io 安装（发布后）
cargo install readme-summarizer
```

### 方法 4: 从源码构建

```bash
# 克隆仓库
git clone https://github.com/YOUR_USERNAME/readme-summarizer
cd readme-summarizer

# 构建发布版本
cargo build --release

# 二进制文件位于 target/release/readme-sum
# 可以将其复制到 PATH 中的目录
sudo cp target/release/readme-sum /usr/local/bin/
```

### 验证安装

```bash
readme-sum --version
```

## 使用方法

### 首次使用

首次运行时，工具会提示你输入要扫描的源目录：

```bash
readme-sum

# 输出：
# 📁 欢迎使用 README Summarizer!
# 
# 请输入要扫描的源目录路径: ~/projects
# ✓ 配置已保存
```

### 基本用法

```bash
# 使用配置的默认目录扫描
readme-sum

# 指定源目录
readme-sum --source ~/projects

# 指定输出文件路径
readme-sum --source ~/projects --output ~/Documents/summary.md

# 指定自定义 README 文件名
readme-sum --filename README.zh-CN.md
```

### 监控模式

自动监控源目录变化，当检测到 README 文件变化或新项目时自动重新生成摘要：

```bash
# 使用配置的默认目录
readme-sum watch

# 指定源目录
readme-sum watch --source ~/projects
```

### Obsidian 同步

将生成的摘要文件同步到 Obsidian vault：

```bash
# 首次使用会提示输入 vault 路径
readme-sum sync-obsidian

# 指定 vault 路径
readme-sum sync-obsidian --vault ~/Documents/ObsidianVault
```

### 配置管理

使用 `config` 子命令管理配置文件：

```bash
# 查看当前配置
readme-sum config view

# 设置源目录
readme-sum config set source ~/projects

# 设置 README 文件名
readme-sum config set readme-filename README.zh-CN.md

# 设置输出文件名
readme-sum config set output-filename SUMMARY.md

# 设置 Obsidian vault 路径
readme-sum config set obsidian-vault ~/Documents/ObsidianVault

# 在默认编辑器中打开配置文件
readme-sum config edit

# 显示配置文件路径
readme-sum config path
```

### 命令行参数

```
命令:
  watch          监控源目录变化并自动重新生成摘要
  sync-obsidian  同步摘要文件到 Obsidian vault
  config         管理配置文件
  help           显示帮助信息

选项:
  -s, --source <PATH>      扫描的源目录（覆盖配置文件）
  -o, --output <FILE>      输出文件路径（默认: {源目录}/README-SUMMARY.md）
  -f, --filename <NAME>    README 文件名（默认: README.md）
  -h, --help              显示帮助信息
  -V, --version           显示版本信息

config 子命令:
  view  查看当前配置
  set   设置配置项 (source|readme-filename|output-filename|obsidian-vault)
  edit  在默认编辑器中打开配置文件
  path  显示配置文件路径
```

### 配置文件

配置文件位置根据操作系统自动确定：

- **Linux**: `~/.config/readme-summarizer/config.toml`
- **macOS**: `~/Library/Application Support/readme-summarizer/config.toml`
- **Windows**: `%APPDATA%\readme-summarizer\config.toml`

配置文件格式：

```toml
source_directory = "/Users/username/projects"
readme_filename = "README.md"
output_filename = "README-SUMMARY.md"
obsidian_vault_path = "/Users/username/Documents/ObsidianVault"
```

首次运行时会交互式提示输入源目录并自动创建配置文件。你可以手动编辑此文件来修改默认值。

## 工作原理

1. **交互式配置**: 首次运行时提示输入源目录并保存
2. **扫描目录**: 遍历源目录下的一层子目录
3. **查找 README**: 在每个子目录中查找指定的 README 文件
4. **解析内容**: 使用规则提取项目信息：
   - **项目名称**: 提取第一个 `#` 标题，否则使用目录名
   - **项目功能**: 查找 "Features"、"功能"、"Description" 等关键词段落
   - **解决的问题**: 查找 "Problem"、"问题"、"Why"、"Motivation" 等关键词段落
5. **生成表格**: 将所有项目信息汇总到单个 Markdown 表格文件

## 输出示例

运行命令后的输出：

```
📁 扫描目录: /Users/qwerdey/projects
📝 README 文件名: README.md
� 输出文件: /Users/qwerdey/projects/README-SUMMARY.md

✓ 处理: my-web-app
✓ 处理: rust-cli-tool
✓ 处理: python-script

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ 完成! 成功处理 3 个项目
📄 摘要文件已生成: /Users/qwerdey/projects/README-SUMMARY.md
```

生成的摘要文件格式：

```markdown
# README 项目汇总

生成时间: 2026-03-19 17:30:00

| 项目名称 | 项目目录 | 项目功能 | 解决的问题 | 源文件路径 |
|---------|---------|---------|-----------|----------|
| my-web-app | [📁](/path/to/my-web-app/) | Real-time analytics<br>Beautiful charts<br>... | Expensive tools<br>Complex setup<br>... | projects/my-web-app/README.md |
| rust-cli-tool | [📁](/path/to/rust-cli-tool/) | Fast execution<br>Cross-platform<br>... | Manual processes<br>Slow scripts<br>... | projects/rust-cli-tool/README.md |

---
*此摘要由 README Summarizer 自动生成*
```

**项目目录列**包含可点击的文件夹图标链接，点击可直接在文件管理器中打开项目目录。

## 技术栈

- **Rust 2021 Edition**
- **clap**: 命令行参数解析
- **serde + toml**: 配置序列化
- **regex**: 文本模式匹配
- **chrono**: 时间戳生成
- **walkdir**: 目录遍历

## 项目结构

```
git-readmes/
├── Cargo.toml
├── src/
│   ├── main.rs           # 程序入口和 CLI 参数解析
│   ├── config.rs         # 配置管理
│   ├── scanner.rs        # 目录扫描
│   ├── parser.rs         # README 内容解析
│   ├── generator.rs      # 摘要文件生成
│   └── error.rs          # 错误类型定义
└── README.md
```

## 注意事项

- 工具仅扫描一层子目录，不会递归扫描
- 解析规则基于常见 README 结构，可能无法覆盖所有格式
- 默认输出文件位于源目录内，文件名为 `README-SUMMARY.md`
- 已存在的摘要文件会被覆盖
- 表格中的长文本使用 `<br>` 分隔多行内容
- 监控模式会忽略输出文件本身的变化，避免无限循环
- Obsidian 同步会覆盖 vault 中的同名文件

## 许可证

MIT License
