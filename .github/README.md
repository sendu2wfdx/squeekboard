# GitHub Actions CI/CD

This directory contains GitHub Actions workflows for automated building, testing, and releasing of squeekboard.

## Workflows

### 1. CI (`ci.yml`)
- **Trigger**: On every push and pull request
- **Purpose**: Quick build and test to verify code changes
- **Actions**:
  - Build with meson/ninja
  - Run tests (with allowed failures)
  - Verify build artifacts

### 2. Build and Release (`build-and-release.yml`)
- **Trigger**: 
  - Push to main/master branch
  - Version tags (v*)
  - Manual dispatch
- **Purpose**: Build packages for multiple architectures and create releases
- **Actions**:
  - Build .deb packages for amd64
  - Build .deb packages for arm64 (using QEMU)
  - Upload build artifacts
  - Create GitHub releases on tags

## Building Packages

### AMD64 Packages
The workflow builds native amd64 packages on Ubuntu runners using the standard Debian build tools:
- `meson` for configuration
- `ninja` for building
- `debuild` for packaging

### ARM64 Packages
ARM64 packages are built using Docker with QEMU emulation:
- Uses Debian Bookworm base image
- Runs on arm64 architecture via QEMU
- Full native build inside emulated environment

## Creating a Release

To create a new release with packaged artifacts:

1. **Tag your commit**:
   ```bash
   git tag -a v1.44.0 -m "Release version 1.44.0"
   git push origin v1.44.0
   ```

2. **GitHub Actions will automatically**:
   - Build .deb packages for both amd64 and arm64
   - Create a GitHub Release
   - Upload the .deb files as release assets
   - Generate release notes

## Artifacts

Build artifacts are available for download after each workflow run:
- **squeekboard-deb-amd64**: AMD64 .deb packages
- **squeekboard-deb-arm64**: ARM64 .deb packages
- **build-info-***: Build information and changes files

## Manual Workflow Dispatch

You can manually trigger the build and release workflow:
1. Go to Actions tab in GitHub
2. Select "Build and Release" workflow
3. Click "Run workflow"
4. Select branch and run

## Dependencies

The workflows install all required build dependencies automatically:
- Meson build system
- Ninja build tool
- Rust toolchain
- GTK3 development libraries
- Wayland development libraries
- Debian packaging tools

## Caching

The workflows use GitHub Actions cache to speed up builds:
- Cargo registry cache
- Cargo git cache
- Build dependency cache

This significantly reduces build times for subsequent runs.

## Troubleshooting

### Build Failures
- Check the workflow logs in the Actions tab
- Verify all dependencies are correctly specified
- Ensure Cargo.lock is up to date

### ARM64 Build Issues
- ARM64 builds may take longer due to QEMU emulation
- Check Docker and QEMU setup steps
- Verify platform specification is correct

### Release Creation Issues
- Ensure you have proper permissions (contents: write)
- Verify tag format matches `v*` pattern
- Check GITHUB_TOKEN has necessary scopes
