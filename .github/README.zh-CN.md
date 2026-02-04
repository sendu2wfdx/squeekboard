# GitHub Actions 自动编译和发布说明

本仓库已配置 GitHub Actions 工作流，可以自动编译、打包 .deb 文件并创建 GitHub Releases。

## 功能特性

### 1. 自动构建（CI）
- **触发条件**：每次代码推送和 Pull Request
- **功能**：快速编译和测试代码
- **支持架构**：AMD64

### 2. 打包和发布
- **触发条件**：
  - 推送到 main/master 分支
  - 创建版本标签（v*格式）
  - 手动触发
- **功能**：
  - 编译 AMD64 架构的 .deb 包
  - 编译 ARM64 架构的 .deb 包（树莓派5可用）
  - 自动上传构建产物
  - 自动创建 GitHub Release

## 如何使用

### 方式一：自动发布（推荐）

1. **准备发布版本**
   ```bash
   # 确保代码已提交
   git add .
   git commit -m "准备发布 v1.44.0"
   ```

2. **创建版本标签**
   ```bash
   # 创建带注释的标签
   git tag -a v1.44.0 -m "发布版本 1.44.0"
   
   # 推送标签到 GitHub
   git push origin v1.44.0
   ```

3. **等待自动构建**
   - GitHub Actions 会自动开始构建
   - 编译 AMD64 和 ARM64 两个架构的 .deb 包
   - 构建完成后自动创建 Release

4. **查看发布**
   - 访问仓库的 Releases 页面
   - 下载对应架构的 .deb 文件

### 方式二：手动触发构建

1. 打开 GitHub 仓库页面
2. 点击 "Actions" 标签
3. 选择 "Build and Release" 工作流
4. 点击 "Run workflow" 按钮
5. 选择要构建的分支
6. 点击 "Run workflow" 开始构建

### 方式三：每次推送自动构建

- 推送代码到 main/master 分支会自动触发构建
- 构建产物可以在 Actions 页面下载
- 不会自动创建 Release（只有标签才会创建）

## 下载构建产物

### 从 GitHub Actions 下载

1. 打开仓库的 Actions 标签
2. 选择一个工作流运行
3. 滚动到页面底部的 "Artifacts" 部分
4. 下载需要的文件：
   - `squeekboard-deb-amd64`：AMD64 架构的 .deb 包
   - `squeekboard-deb-arm64`：ARM64 架构的 .deb 包

### 从 Releases 下载

1. 打开仓库的 Releases 标签
2. 选择需要的版本
3. 在 Assets 部分下载 .deb 文件

## 安装说明

### Debian/Ubuntu (AMD64)
```bash
# 下载 amd64 版本的 .deb 文件
# 例如：squeekboard_1.44.0+github123.abc123_amd64.deb

# 安装
sudo dpkg -i squeekboard_*.deb

# 如果有依赖问题，运行：
sudo apt-get install -f
```

### 树莓派 5 (ARM64)
```bash
# 下载 arm64 版本的 .deb 文件
# 例如：squeekboard_1.44.0+github-arm64.abc123_arm64.deb

# 安装
sudo dpkg -i squeekboard_*_arm64.deb

# 如果有依赖问题，运行：
sudo apt-get install -f
```

## 版本命名规则

构建的包会自动添加版本后缀：
- **AMD64**：`版本号+github构建号.提交哈希`
  - 例如：`1.44.0+github123.abc1234`
- **ARM64**：`版本号+github-arm64.提交哈希`
  - 例如：`1.44.0+github-arm64.abc1234`

## 查看构建状态

### 在 GitHub 页面查看
1. 打开仓库主页
2. 查看 README 顶部的构建徽章
3. 点击徽章可以查看详细的构建日志

### 在 Actions 页面查看
1. 打开 Actions 标签
2. 查看最近的工作流运行
3. 点击具体的运行可以查看详细日志
4. 每个步骤都有独立的日志可以展开查看

## 常见问题

### Q: 为什么 ARM64 构建需要更长时间？
A: ARM64 构建使用 QEMU 模拟器在 AMD64 机器上运行，速度会比原生构建慢。

### Q: 如何查看构建失败的原因？
A: 
1. 打开 Actions 标签
2. 点击失败的工作流运行
3. 展开失败的步骤查看错误日志

### Q: 可以只构建一个架构吗？
A: 可以修改工作流文件来禁用某个架构的构建，但不推荐。

### Q: 如何添加更多架构支持？
A: 可以参考 ARM64 构建的配置，添加其他架构的 Docker 构建步骤。

## 技术细节

### AMD64 构建
- 运行环境：Ubuntu latest
- 构建工具：Meson + Ninja
- 打包工具：debuild
- 原生编译，速度快

### ARM64 构建
- 运行环境：Docker (Debian Bookworm)
- 模拟器：QEMU
- 平台：linux/arm64
- 全功能原生构建（在模拟环境中）

### 缓存策略
- Cargo registry 缓存
- Cargo git 缓存
- 构建依赖缓存
- 加速后续构建

## 需要帮助？

如果遇到问题，可以：
1. 查看 `.github/README.md` 英文文档
2. 查看 Actions 日志中的错误信息
3. 在仓库中创建 Issue 报告问题
4. 参考 `.github/workflows/` 目录下的工作流文件
