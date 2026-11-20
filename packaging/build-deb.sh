#!/usr/bin/env bash
set -euo pipefail

APP_NAME="pro-clock"
VERSION="0.2.0"
ARCH="$(dpkg --print-architecture)"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$ROOT_DIR/dist"
PKG_DIR="$DIST_DIR/${APP_NAME}_${VERSION}_${ARCH}"

export PATH="$HOME/.cargo/bin:$PATH"

echo "==> Building release binary..."
cd "$ROOT_DIR"
cargo build --release

echo "==> Staging Debian package structure..."
rm -rf "$PKG_DIR"
mkdir -p "$PKG_DIR/DEBIAN"
mkdir -p "$PKG_DIR/usr/bin"
mkdir -p "$PKG_DIR/usr/share/applications"
mkdir -p "$PKG_DIR/usr/share/icons/hicolor/scalable/apps"
mkdir -p "$PKG_DIR/usr/share/doc/$APP_NAME"

install -m 755 "$ROOT_DIR/target/release/$APP_NAME" "$PKG_DIR/usr/bin/$APP_NAME"
install -m 644 "$ROOT_DIR/packaging/pro-clock.desktop" "$PKG_DIR/usr/share/applications/pro-clock.desktop"
install -m 644 "$ROOT_DIR/assets/pro-clock.svg" "$PKG_DIR/usr/share/icons/hicolor/scalable/apps/pro-clock.svg"
install -m 644 "$ROOT_DIR/LICENSE" "$PKG_DIR/usr/share/doc/$APP_NAME/copyright"

INSTALLED_SIZE="$(du -sk "$PKG_DIR/usr" | awk '{print $1}')"

cat > "$PKG_DIR/DEBIAN/control" <<EOF
Package: $APP_NAME
Version: $VERSION
Section: utils
Priority: optional
Architecture: $ARCH
Installed-Size: $INSTALLED_SIZE
Depends: libc6, libgcc-s1, libgl1, libx11-6 | libwayland-client0
Maintainer: Pro Clock Contributors <pro-clock@localhost>
Homepage: https://github.com/pro-clock/pro-clock
Description: Free, open-source, 100% offline Clockify replacement
 Pro Clock is a fast, native Linux time tracker written in Rust.
 It features a live stopwatch timer, manual time entry, projects,
 clients, tags, billable rates, visual reports, and CSV/JSON export,
 all stored locally in SQLite with zero cloud dependencies.
EOF

cat > "$PKG_DIR/DEBIAN/postinst" <<'EOF'
#!/bin/sh
set -e
if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database -q /usr/share/applications || true
fi
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor || true
fi
EOF
chmod 755 "$PKG_DIR/DEBIAN/postinst"

DEB_FILE="$DIST_DIR/${APP_NAME}_${VERSION}_${ARCH}.deb"
echo "==> Building $DEB_FILE..."
dpkg-deb --build --root-owner-group "$PKG_DIR" "$DEB_FILE"

echo "==> Done! Package created at:"
echo "    $DEB_FILE"
echo ""
echo "Install on Ubuntu with:"
echo "    sudo apt install \"$DEB_FILE\""
