#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_NAME="pro-clock"
BIN_SRC="$ROOT_DIR/target/release/$APP_NAME"

export PATH="$HOME/.cargo/bin:$PATH"

if [[ "${1:-}" == "--system" ]]; then
    echo "==> Building and installing system-wide .deb package (requires sudo)..."
    VERSION="$(sed -nE 's/^version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' "$ROOT_DIR/Cargo.toml" | head -n 1)"
    if [[ -z "$VERSION" ]]; then
        echo "Error: Could not parse version from Cargo.toml" >&2
        exit 1
    fi
    bash "$ROOT_DIR/packaging/build-deb.sh"
    ARCH="$(dpkg --print-architecture)"
    DEB_FILE="$ROOT_DIR/dist/${APP_NAME}_${VERSION}_${ARCH}.deb"
    sudo apt install -y "$DEB_FILE"
    echo "==> Pro Clock installed system-wide! Launch it from your Ubuntu Apps menu or run 'pro-clock'."
    exit 0
fi

echo "==> Installing Pro Clock for current user (~/.local)..."
if [[ ! -x "$BIN_SRC" ]]; then
    echo "==> Release binary not found; compiling with cargo..."
    (cd "$ROOT_DIR" && cargo build --release)
fi

mkdir -p "$HOME/.local/bin"
mkdir -p "$HOME/.local/share/applications"
mkdir -p "$HOME/.local/share/icons/hicolor/scalable/apps"
mkdir -p "$HOME/.local/share/doc/$APP_NAME/docs/tutorials"

install -m 755 "$BIN_SRC" "$HOME/.local/bin/$APP_NAME"
install -m 644 "$ROOT_DIR/assets/pro-clock.svg" "$HOME/.local/share/icons/hicolor/scalable/apps/pro-clock.svg"
install -m 644 "$ROOT_DIR/README.md" "$HOME/.local/share/doc/$APP_NAME/README.md"
install -m 644 "$ROOT_DIR/docs/HELP.md" "$HOME/.local/share/doc/$APP_NAME/docs/HELP.md"
install -m 644 "$ROOT_DIR/docs/tutorials/"*.md "$HOME/.local/share/doc/$APP_NAME/docs/tutorials/"

sed "s|^Exec=pro-clock|Exec=$HOME/.local/bin/$APP_NAME|" \
    "$ROOT_DIR/packaging/pro-clock.desktop" \
    > "$HOME/.local/share/applications/pro-clock.desktop"

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$HOME/.local/share/applications" >/dev/null 2>&1 || true
fi
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -q -t -f "$HOME/.local/share/icons/hicolor" >/dev/null 2>&1 || true
fi

echo ""
echo "==> Pro Clock installed successfully!"
echo "    Binary:  $HOME/.local/bin/$APP_NAME"
echo "    Desktop: $HOME/.local/share/applications/pro-clock.desktop"
echo "    Docs:    $HOME/.local/share/doc/$APP_NAME"
echo ""
echo "You can now launch 'Pro Clock' from your Ubuntu Applications menu,"
echo "or run '$HOME/.local/bin/$APP_NAME' from a terminal."
echo "(Tip: Run './install.sh --system' if you prefer installing the .deb system-wide.)"
