# Quick Start Guide - GitHub Actions CI/CD

## 快速开始指南 (中文版在下方)

### For Repository Maintainers

#### Create a New Release
```bash
# 1. Ensure all changes are committed
git add .
git commit -m "Prepare for release v1.44.0"

# 2. Create and push a version tag
git tag -a v1.44.0 -m "Release version 1.44.0"
git push origin v1.44.0

# 3. Wait for GitHub Actions to complete
# - Build will start automatically
# - .deb packages will be generated
# - GitHub Release will be created
# - Packages will be uploaded as assets
```

#### Trigger a Manual Build
1. Go to repository page on GitHub
2. Click on "Actions" tab
3. Select "Build and Release" workflow
4. Click "Run workflow" button
5. Choose branch and click "Run workflow"

#### Check Build Status
- Build status badges are shown in README
- Click on badges to view detailed logs
- Go to Actions tab to see all workflow runs

### For Users

#### Download Pre-built Packages

**Option 1: From Releases (Recommended)**
1. Go to repository's Releases page
2. Click on the latest release
3. Download the appropriate .deb file:
   - `*_amd64.deb` - For regular PCs (Intel/AMD)
   - `*_arm64.deb` - For Raspberry Pi 5, ARM64 devices

**Option 2: From Actions (Latest Builds)**
1. Go to repository's Actions tab
2. Click on a successful workflow run
3. Scroll to "Artifacts" section
4. Download the package:
   - `squeekboard-deb-amd64` - AMD64 package
   - `squeekboard-deb-arm64` - ARM64 package

#### Install Package

**On Debian/Ubuntu (AMD64):**
```bash
sudo dpkg -i squeekboard_*_amd64.deb
sudo apt-get install -f  # Fix dependencies if needed
```

**On Raspberry Pi 5 (ARM64):**
```bash
sudo dpkg -i squeekboard_*_arm64.deb
sudo apt-get install -f  # Fix dependencies if needed
```

---

## 中文快速指南

### 仓库维护者

#### 创建新版本发布
```bash
# 1. 确保所有改动已提交
git add .
git commit -m "准备发布 v1.44.0"

# 2. 创建并推送版本标签
git tag -a v1.44.0 -m "发布版本 1.44.0"
git push origin v1.44.0

# 3. 等待 GitHub Actions 完成
# - 构建会自动开始
# - 生成 .deb 安装包
# - 自动创建 GitHub Release
# - 上传安装包到 Release
```

#### 手动触发构建
1. 打开 GitHub 仓库页面
2. 点击 "Actions" 标签
3. 选择 "Build and Release" 工作流
4. 点击 "Run workflow" 按钮
5. 选择分支后点击 "Run workflow"

#### 查看构建状态
- README 中显示构建状态徽章
- 点击徽章可查看详细日志
- 访问 Actions 标签查看所有构建记录

### 普通用户

#### 下载预编译的安装包

**方式一：从 Releases 下载（推荐）**
1. 访问仓库的 Releases 页面
2. 点击最新的 release
3. 下载对应的 .deb 文件：
   - `*_amd64.deb` - 普通电脑（Intel/AMD 处理器）
   - `*_arm64.deb` - 树莓派5、ARM64 设备

**方式二：从 Actions 下载（最新构建）**
1. 访问仓库的 Actions 标签
2. 点击一个成功的工作流运行
3. 滚动到 "Artifacts" 部分
4. 下载安装包：
   - `squeekboard-deb-amd64` - AMD64 安装包
   - `squeekboard-deb-arm64` - ARM64 安装包

#### 安装软件包

**在 Debian/Ubuntu 上（AMD64）：**
```bash
sudo dpkg -i squeekboard_*_amd64.deb
sudo apt-get install -f  # 如果有依赖问题，运行此命令修复
```

**在树莓派5上（ARM64）：**
```bash
sudo dpkg -i squeekboard_*_arm64.deb
sudo apt-get install -f  # 如果有依赖问题，运行此命令修复
```

---

## Workflow Files

### CI Workflow (`.github/workflows/ci.yml`)
- Runs on every push and PR
- Quick build and test
- No package generation
- Fast feedback

### Build and Release Workflow (`.github/workflows/build-and-release.yml`)
- Runs on main/master push and tags
- Generates .deb packages for AMD64 and ARM64
- Creates GitHub Releases on tags
- Uploads artifacts

## Common Issues

### Build Fails
**Check:**
1. View detailed logs in Actions tab
2. Verify all dependencies are available
3. Check if Cargo.lock is up to date

### ARM64 Build Slow
**Normal:** ARM64 builds use QEMU emulation and take longer (15-30 minutes)

### Release Not Created
**Verify:**
1. Tag format is correct (`v*` pattern, e.g., `v1.44.0`)
2. Tag was pushed to GitHub
3. Repository has proper permissions

### Package Installation Fails
**Fix dependencies:**
```bash
sudo apt-get update
sudo apt-get install -f
```

## Advanced Usage

### Skip CI for a Commit
```bash
git commit -m "docs: update README [skip ci]"
```

### Build Only One Architecture
Edit workflow file and comment out unwanted job

### Custom Build Options
Modify workflow files in `.github/workflows/` directory

## Support

For more information, see:
- [GitHub Actions Documentation](.github/README.md)
- [中文文档](.github/README.zh-CN.md)
- [Cross-Compilation Guide](CROSS_COMPILE_README.md)
