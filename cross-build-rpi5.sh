#!/bin/bash
set -e

# Cross-compilation script for Raspberry Pi 5 (ARM64)
# This script builds squeekboard for ARM64 and generates a .deb package

echo "=== Squeekboard Cross-compilation for Raspberry Pi 5 (ARM64) ==="

# Use Docker for clean cross-compilation environment
echo "Using Docker for cross-compilation..."

# Create Dockerfile for cross-compilation
cat > Dockerfile.arm64 << 'DOCKERFILE_END'
FROM debian:bookworm

# Install cross-compilation tools
RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install -y \
        crossbuild-essential-arm64 \
        gcc-aarch64-linux-gnu \
        g++-aarch64-linux-gnu \
        pkg-config \
        meson \
        ninja-build \
        cargo \
        rustc \
        debhelper-compat \
        devscripts \
        lsb-release \
        python3 \
        python3-ruamel.yaml \
        curl \
        git \
        ca-certificates

# Configure apt sources for both architectures
RUN sed -i 's/^deb /deb [arch=amd64] /g' /etc/apt/sources.list.d/debian.sources && \
    cp /etc/apt/sources.list.d/debian.sources /etc/apt/sources.list.d/debian-arm64.sources && \
    sed -i 's/arch=amd64/arch=arm64/g' /etc/apt/sources.list.d/debian-arm64.sources && \
    apt-get update

# Install ARM64 libraries from Debian repositories
# Rust dependencies will be vendored by cargo
RUN apt-get install -y \
        libglib2.0-dev:arm64 \
        libgtk-3-dev:arm64 \
        libwayland-dev:arm64 \
        libgnome-desktop-3-dev:arm64 \
        libbsd-dev:arm64 \
        libfeedback-dev:arm64 \
        wayland-protocols

# Set up Rust for ARM64
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . /root/.cargo/env && \
    rustup target add aarch64-unknown-linux-gnu

# Set up environment for cross-compilation
ENV PATH="/root/.cargo/bin:${PATH}"
ENV PKG_CONFIG_PATH=/usr/lib/aarch64-linux-gnu/pkgconfig
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc

WORKDIR /build

CMD ["/bin/bash"]
DOCKERFILE_END

echo "Building Docker image for cross-compilation..."
docker build -t squeekboard-arm64-builder -f Dockerfile.arm64 .

echo "Running cross-compilation in Docker..."
docker run --rm -v $(pwd):/build squeekboard-arm64-builder bash -c '
    cd /build
    export PATH="/root/.cargo/bin:/usr/local/bin:$PATH"
    export DEB_BUILD_OPTIONS="nocheck"
    export CARGO_HOME=/build/debian/cargo
    export DEB_HOST_RUST_TYPE=aarch64-unknown-linux-gnu
    export DEB_HOST_GNU_TYPE=aarch64-linux-gnu
    export DEB_BUILD_ARCH=amd64
    export DEB_HOST_ARCH=arm64
    
    # Create a wrapper for rustc that includes the target
    cat > /usr/local/bin/rustc-aarch64 << '"'"'EOF_RUSTC'"'"'
#!/bin/bash
exec /root/.cargo/bin/rustc --target aarch64-unknown-linux-gnu "$@"
EOF_RUSTC
    chmod +x /usr/local/bin/rustc-aarch64
    
    # Verify the wrapper works
    /usr/local/bin/rustc-aarch64 --version || {
        echo "Error: rustc wrapper failed"
        exit 1
    }
    
    # Set up Cargo config for cross-compilation
    mkdir -p .cargo
    cat > .cargo/config.toml << "CARGOCONFIG_EOF"
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

[build]
target = "aarch64-unknown-linux-gnu"
CARGOCONFIG_EOF

    # Build using dpkg-buildpackage for ARM64
    # Use -d to ignore build dependencies since cargo vendors everything
    dpkg-buildpackage -d -aarm64 -B -uc -us || {
        echo "Build failed, checking logs..."
        cat _build/meson-logs/meson-log.txt 2>/dev/null || true
        exit 1
    }
'

echo "Copying .deb files from Docker build..."
# The .deb files will be in the parent directory
if ls ../*.deb 1> /dev/null 2>&1; then
    cp ../*.deb .
    echo "=== Build complete ==="
    echo "Generated .deb packages for ARM64 (Raspberry Pi 5):"
    ls -lh *.deb
else
    echo "Warning: No .deb files found"
fi
