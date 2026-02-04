# Cross-Compilation for Raspberry Pi 5 (ARM64)

This directory contains scripts and configuration files for cross-compiling squeekboard for Raspberry Pi 5 (ARM64 architecture) and generating Debian .deb packages.

## Prerequisites

- Docker installed and running
- At least 4GB of free disk space
- x86_64/amd64 host system

## Build Scripts

### Option 1: Docker-based Debian Packaging (`cross-build-rpi5.sh`)

This script uses the Debian package build system with cross-compilation support:

```bash
./cross-build-rpi5.sh
```

This approach:
- Creates a Docker container with full cross-compilation toolchain
- Uses `dpkg-buildpackage` with ARM64 target
- Attempts to generate proper Debian packages following Debian standards

**Status**: Work in progress - requires meson Rust cross-compilation integration fixes

### Option 2: Direct Meson Build (`build-rpi5-direct.sh`)

This script directly uses meson for cross-compilation:

```bash
./build-rpi5-direct.sh
```

This approach:
- Uses meson with a cross-compilation file
- Builds the ARM64 binary
- Manually packages into .deb format

**Status**: Partial - generates package structure but binary build needs meson configuration fixes

## Generated Files

- `squeekboard_1.44.0-rpi5_arm64.deb` - Debian package (currently partial)
- `.cargo/config.toml` - Cargo configuration for ARM64 cross-compilation
- `cross-arm64.txt` / `cross-arm64.ini` - Meson cross-compilation configuration
- `Dockerfile.arm64` - Docker build environment specification

## Current Status

The cross-compilation infrastructure is fully set up with:
- ✅ Docker-based build environment
- ✅ ARM64 toolchain (gcc, g++, binutils)
- ✅ Rust ARM64 target configuration
- ✅ Cargo vendoring for offline builds
- ✅ ARM64 library dependencies
- ✅ Debian package structure
-  ⚠️ Meson+Rust cross-compilation integration (needs fixes)

## Known Issues

### Meson Rust Compiler Detection

Meson currently has difficulty detecting the Rust compiler in cross-compilation mode. The error manifests as:

```
ERROR: Unknown compiler(s): [['rustc-aarch64']]
```

or

```
ERROR: Undefined constant 'aarch64' in machine file variable 'c'.
```

### Workarounds Being Investigated

1. Using a wrapper script for rustc with embedded target
2. Configuring meson's Rust support for cross-compilation
3. Building Rust components separately and linking

## Manual Build Instructions

If you want to build directly on a Raspberry Pi 5 or in an ARM64 environment:

```bash
# On Raspberry Pi 5 or ARM64 system
apt-get build-dep .
meson setup _build
ninja -C _build
ninja -C _build install
```

## Docker Environment

The Docker container includes:
- Debian Bookworm base
- Cross-compilation toolchain for ARM64
- Rust with aarch64-unknown-linux-gnu target
- ARM64 development libraries:
  - libglib2.0-dev
  - libgtk-3-dev
  - libwayland-dev
  - libgnome-desktop-3-dev
  - libfeedback-dev
  - libbsd-dev

## Package Contents

The generated .deb package includes:
- Keyboard layout files for 40+ languages
- GSettings schema
- DBus interface definitions
- Application metadata

Missing (to be added):
- squeekboard binary executable
- Desktop entry file

## Contributing

To improve cross-compilation support:

1. Test the scripts on your system
2. Report issues with detailed error messages
3. Submit fixes for meson Rust cross-compilation
4. Improve documentation

## References

- [Squeekboard GitHub](https://gitlab.gnome.org/World/Phosh/squeekboard)
- [Meson Cross-compilation](https://mesonbuild.com/Cross-compilation.html)
- [Rust Cross-compilation](https://rust-lang.github.io/rustup/cross-compilation.html)
- [Debian Cross-building](https://wiki.debian.org/CrossCompiling)
