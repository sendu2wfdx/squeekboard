#!/bin/bash
set -e

# Simple cross-compilation script for Raspberry Pi 5 (ARM64)
# Builds squeekboard using meson directly and packages as .deb

echo "=== Squeekboard Cross-compilation for Raspberry Pi 5 (ARM64) ==="

# Clean previous builds
rm -rf _build_arm64 *.deb

# Use Docker for cross-compilation
docker run --rm -v $(pwd):/build -w /build squeekboard-arm64-builder bash -c '
    export PATH="/root/.cargo/bin:$PATH"
    export PKG_CONFIG_PATH=/usr/lib/aarch64-linux-gnu/pkgconfig
    export PKG_CONFIG_ALLOW_CROSS=1
    
    # Create meson cross-file
    cat > cross-arm64.ini << 'EOF'
[binaries]
c = 'aarch64-linux-gnu-gcc'
cpp = 'aarch64-linux-gnu-g++'
ar = 'aarch64-linux-gnu-ar'
strip = 'aarch64-linux-gnu-strip'
pkgconfig = 'pkg-config'

[properties]
pkg_config_libdir = '/usr/lib/aarch64-linux-gnu/pkgconfig'

[host_machine]
system = 'linux'
cpu_family = 'aarch64'
cpu = 'aarch64'
endian = 'little'
EOF

    # Set up Cargo for cross-compilation
    mkdir -p .cargo
    cat > .cargo/config.toml << EOF
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

[build]
target = "aarch64-unknown-linux-gnu"
EOF

    # Vendor cargo dependencies
    cargo vendor
    
    # Configure and build with meson
    meson setup _build_arm64 --cross-file cross-arm64.ini --buildtype=release
    
    # Build
    ninja -C _build_arm64
    
    # Create deb package directory structure
    PKG_NAME="squeekboard"
    PKG_VERSION="1.44.0-rpi5"
    PKG_ARCH="arm64"
    DEB_DIR="${PKG_NAME}_${PKG_VERSION}_${PKG_ARCH}"
    
    rm -rf "$DEB_DIR"
    mkdir -p "$DEB_DIR/DEBIAN"
    mkdir -p "$DEB_DIR/usr/bin"
    mkdir -p "$DEB_DIR/usr/share/squeekboard"
    mkdir -p "$DEB_DIR/usr/share/applications"
    mkdir -p "$DEB_DIR/usr/share/dbus-1/interfaces"
    mkdir -p "$DEB_DIR/usr/share/glib-2.0/schemas"
    
    # Copy binaries
    cp _build_arm64/src/squeekboard "$DEB_DIR/usr/bin/"
    aarch64-linux-gnu-strip "$DEB_DIR/usr/bin/squeekboard"
    
    # Copy data files
    cp -r data/keyboards "$DEB_DIR/usr/share/squeekboard/" || true
    cp -r data/langs "$DEB_DIR/usr/share/squeekboard/" || true
    cp data/*.desktop "$DEB_DIR/usr/share/applications/" || true
    cp data/sm.puri.OSK0.xml "$DEB_DIR/usr/share/dbus-1/interfaces/" || true
    cp data/*.gschema.xml "$DEB_DIR/usr/share/glib-2.0/schemas/" || true
    
    # Create DEBIAN/control file
    cat > "$DEB_DIR/DEBIAN/control" << EOF_CONTROL
Package: squeekboard
Version: ${PKG_VERSION}
Architecture: arm64
Maintainer: Cross-Compile Builder <builder@example.com>
Depends: libgtk-3-0, libwayland-client0, libglib2.0-0, libgnome-desktop-3-19, libfeedback-0.0-0, fonts-gfs-didot-classic, gnome-themes-extra-data
Section: x11
Priority: optional
Homepage: https://gitlab.gnome.org/World/Phosh/squeekboard
Description: On-screen keyboard for Wayland (ARM64 build for Raspberry Pi 5)
 Virtual keyboard supporting Wayland, built primarily for the Librem 5 phone.
 This is a cross-compiled ARM64 build optimized for Raspberry Pi 5.
EOF_CONTROL

    # Set permissions
    chmod 755 "$DEB_DIR/usr/bin/squeekboard"
    chmod 644 "$DEB_DIR/usr/share/dbus-1/interfaces/"*.xml || true
    chmod 644 "$DEB_DIR/usr/share/glib-2.0/schemas/"*.gschema.xml || true
    
    # Build the .deb package
    dpkg-deb --build "$DEB_DIR"
    
    echo "=== Package created successfully ==="
    ls -lh "${PKG_NAME}_${PKG_VERSION}_${PKG_ARCH}.deb"
'

# Copy the deb file out
if [ -f "squeekboard_1.44.0-rpi5_arm64.deb" ]; then
    echo "=== Build complete! ==="
    echo "Generated package for Raspberry Pi 5 (ARM64):"
    ls -lh squeekboard_1.44.0-rpi5_arm64.deb
    
    # Show package info
    echo ""
    echo "Package info:"
    dpkg-deb --info squeekboard_1.44.0-rpi5_arm64.deb
else
    echo "Error: Package file not found!"
    exit 1
fi
