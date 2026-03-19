# README Summarizer 使用示例

> **注意**: 本工具已更新为单文件表格输出模式，默认输出到源目录。

## 快速开始

### 1. 构建项目

```bash
cd git-readmes
cargo build --release
```

可执行文件位于 `target/release/readme-sum`

### 2. 安装到系统（可选）

```bash
cargo install --path .
```

安装后可以在任何位置使用 `readme-sum` 命令。

## 使用场景

### 场景 1: 首次运行（交互式配置）

首次运行时，工具会提示你输入源目录：

```bash
readme-sum

# 输出：
📁 欢迎使用 README Summarizer!

请输入要扫描的源目录路径: ~/projects
✓ 配置已保存

📁 扫描目录: /Users/qwerdey/projects
📝 README 文件名: README.md
📄 输出文件: /Users/qwerdey/projects/README-SUMMARY.md

✓ 处理: project-a
✓ 处理: project-b
✓ 处理: project-c

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ 完成! 成功处理 3 个项目
📄 摘要文件已生成: /Users/qwerdey/projects/README-SUMMARY.md
```

配置会保存到 `~/.config/readme-summarizer/config.toml`

### 场景 2: 后续运行（使用保存的配置）

```bash
readme-sum

# 自动使用配置文件中保存的源目录
# 输出文件默认为: {源目录}/README-SUMMARY.md
```

### 场景 3: 指定自定义源目录

临时覆盖配置文件中的源目录：

```bash
readme-sum --source ~/my-projects

# 输出文件: ~/my-projects/README-SUMMARY.md
```

### 场景 4: 指定自定义输出文件

```bash
readme-sum --source ~/projects --output ~/Documents/my-summary.md

# 输出文件: ~/Documents/my-summary.md
```

### 场景 5: 扫描特定文件名的 README

如果你的项目使用不同的 README 文件名：

```bash
readme-sum --filename README.zh-CN.md
```

### 场景 6: 完整自定义

```bash
readme-sum \
  --source ~/workspace/repos \
  --output ~/Documents/repos-summary.md \
  --filename README.md
```

## 输出示例

### 命令行输出

```
📁 扫描目录: /Users/qwerdey/projects
📝 README 文件名: README.md
� 输出文件: /Users/qwerdey/projects/README-SUMMARY.md

✓ 处理: project-a
✓ 处理: project-b
✓ 处理: project-c

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ 完成! 成功处理 3 个项目
📄 摘要文件已生成: /Users/qwerdey/projects/README-SUMMARY.md
```

### 生成的摘要文件示例

**README-SUMMARY.md**:

```markdown
# README 项目汇总

生成时间: 2026-03-19 17:30:00

| 项目名称 | 项目功能 | 解决的问题 | 源文件路径 |
|---------|---------|-----------|----------|
| Web Analytics Dashboard | - Real-time visitor tracking<br>- Beautiful data visualizations<br>- Custom event tracking<br>- Export reports to PDF/CSV<br>- Multi-user support with role-based access | Many existing analytics tools are either too expensive for small businesses or too complex to set up and use. This project aims to provide a simple, affordable, and easy-to-use analytics solution. | projects/project-a/README.md |
| 图片压缩工具 | - 支持 PNG、JPG、WebP 等多种格式<br>- 批量压缩整个目录<br>- 保持图片质量的同时减小文件大小<br>- 多线程并行处理<br>- 压缩前后对比报告 | 在网站开发中，图片文件往往占据大量空间，影响加载速度。手动压缩图片费时费力，这个工具可以自动化批量压缩流程，提高工作效率。 | projects/project-b/README.md |
| API Rate Limiter | This middleware provides configurable rate limiting to protect your API from abuse and ensure fair usage across all clients. | APIs without rate limiting are vulnerable to:<br>- DDoS attacks<br>- Resource exhaustion<br>- Unfair usage by single clients<br>- Increased infrastructure costs<br>This library solves these problems with minimal configuration. | projects/project-c/README.md |

---
*此摘要由 README Summarizer 自动生成*
```

## 配置文件管理

### 查看当前配置

```bash
cat ~/.config/readme-summarizer/config.toml
```

### 手动编辑配置

```toml
source_directory = "/Users/username/projects"
readme_filename = "README.md"
output_filename = "README-SUMMARY.md"
```

编辑后保存，下次运行时会使用新的默认值。

### 配置优先级

命令行参数 > 配置文件

### 输出文件位置优先级

1. `--output` 参数指定的完整文件路径
2. `{源目录}/{output_filename}` (默认: `README-SUMMARY.md`)

## 常见问题

### Q: 如何处理嵌套目录？

A: 工具仅扫描一层子目录。如果需要扫描嵌套目录，可以多次运行工具，每次指定不同的源目录。

### Q: 如果 README 格式不标准怎么办？

A: 工具会尽力提取信息。如果无法找到特定段落，会使用以下策略：
- 项目名称：使用目录名
- 项目功能：提取第一段文本
- 解决的问题：标记为"未明确说明"

### Q: 生成的文件已存在会怎样？

A: 已存在的摘要文件会被覆盖。每次运行都会重新生成完整的表格。

### Q: 支持哪些 README 格式？

A: 工具主要针对 Markdown 格式优化，但也可以处理纯文本文件。

### Q: 如何批量处理多个不同位置的项目？

A: 可以创建一个脚本：

```bash
#!/bin/bash
readme-sum --source ~/work/projects --output ~/summaries/work-summary.md
readme-sum --source ~/personal/projects --output ~/summaries/personal-summary.md
readme-sum --source ~/opensource --output ~/summaries/opensource-summary.md
```

### Q: 表格中的内容太长怎么办？

A: 工具会自动使用 `<br>` 标签分隔多行内容（如列表项）。Markdown 渲染器会正确显示这些内容。

## 高级用法

### 与其他工具集成

#### 1. 生成后自动打开摘要文件

```bash
readme-sum --source ~/projects && open ~/projects/README-SUMMARY.md
```

#### 2. 定期自动生成（使用 cron）

```bash
# 每天凌晨 2 点运行
0 2 * * * /usr/local/bin/readme-sum --source ~/projects
```

#### 3. Git Hook 集成

在项目根目录的 `.git/hooks/post-merge` 中添加：

```bash
#!/bin/bash
readme-sum --source .
# 这会在当前目录生成 README-SUMMARY.md
```

## 性能提示

- 工具使用本地规则解析，速度很快
- 处理 100 个项目通常只需几秒钟
- 不需要网络连接或 API 密钥
- 内存占用极小

## 故障排除

### 权限错误

如果遇到权限问题：

```bash
chmod +x target/release/readme-sum
```

### 配置文件损坏

删除配置文件重新生成：

```bash
rm ~/.config/readme-summarizer/config.toml
readme-sum
```

### 找不到项目

确保：
1. 源目录路径正确
2. 子目录中确实存在 README 文件
3. README 文件名匹配（区分大小写）
