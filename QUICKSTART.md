# 快速开始指南

## 🚀 5 分钟快速发布

### 第一步：准备仓库

1. **创建 GitHub 仓库**
   - 访问 https://github.com/new
   - 仓库名称：`readme-summarizer`
   - 设置为公开（Public）
   - 不要初始化 README（我们已经有了）

2. **推送代码到 GitHub**
   ```bash
   cd /Users/qwerdey/work/git-readmes
   
   # 初始化 git（如果还没有）
   git init
   git add .
   git commit -m "Initial commit: README Summarizer v1.0.0"
   
   # 添加远程仓库（替换 YOUR_USERNAME）
   git remote add origin https://github.com/YOUR_USERNAME/readme-summarizer.git
   git branch -M main
   git push -u origin main
   ```

### 第二步：更新配置

1. **更新 Cargo.toml**
   - 将 `YOUR_USERNAME` 替换为你的 GitHub 用户名
   - 更新 `authors` 字段为你的名字和邮箱

2. **更新 README.md**
   - 将所有 `YOUR_USERNAME` 替换为你的 GitHub 用户名

### 第三步：创建第一个发布

```bash
# 使用自动化脚本
./scripts/prepare-release.sh 1.0.0

# 推送到 GitHub
git push origin main
git push origin v1.0.0
```

### 第四步：等待构建完成

1. 访问 GitHub 仓库的 Actions 页面
2. 等待构建完成（约 5-10 分钟）
3. 访问 Releases 页面查看自动创建的发布

### 第五步：完善 Release 说明

1. 编辑 Release
2. 从 `CHANGELOG.md` 复制相关内容
3. 添加安装说明
4. 发布！

## 📦 测试安装

发布后，测试用户是否能正常安装：

```bash
# macOS/Linux
curl -L https://github.com/YOUR_USERNAME/readme-summarizer/releases/latest/download/readme-sum-macos -o readme-sum
chmod +x readme-sum
./readme-sum --version
```

## 🎯 下一步

- 在 README 中添加徽章（版本、下载量等）
- 创建 GitHub Discussions 供用户交流
- 添加更多示例和文档
- 收集用户反馈，规划下一个版本

## 💡 提示

- 每次发布前运行 `./scripts/build-release.sh` 确保一切正常
- 使用 `PUBLISH_CHECKLIST.md` 确保不遗漏任何步骤
- 遵循语义化版本规范
- 及时回复用户的 Issues 和 Pull Requests

## 🆘 遇到问题？

查看详细文档：
- `RELEASE.md` - 完整发布流程
- `PUBLISH_CHECKLIST.md` - 发布检查清单
- `CONTRIBUTING.md` - 贡献指南
