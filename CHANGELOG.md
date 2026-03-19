# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-03-19

### Added
- 初始版本发布
- 自动扫描目录并提取 README 文件信息
- 生成单个 Markdown 表格文件汇总所有项目
- 表格包含可点击的项目目录链接
- 交互式配置，首次运行自动创建配置文件
- 跨平台配置文件支持（Linux、macOS、Windows）
- `watch` 命令：监控源目录变化并自动重新生成摘要
- `sync-obsidian` 命令：同步摘要文件到 Obsidian vault
- `config` 命令：管理配置文件
  - `config view`：查看当前配置
  - `config set`：设置配置项
  - `config edit`：在编辑器中打开配置文件
  - `config path`：显示配置文件路径
- 支持自定义 README 文件名
- 支持自定义输出文件名和路径
- 配置持久化到系统配置目录

### Features
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

### Technical Details
- Rust 2021 Edition
- 跨平台支持：Linux、macOS、Windows
- 配置文件自动保存到系统标准配置目录
- 文件监控使用防抖机制，避免频繁触发
- 支持路径展开（`~` 自动展开为用户主目录）
