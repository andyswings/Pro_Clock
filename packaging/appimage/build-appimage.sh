#!/usr/bin/env bash
set -euo pipefail

APP_NAME="pro-clock"
APP_ID="io.github.andyswings.ProClock"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
VERSION="$(sed -nE 's/^version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' "$ROOT_DIR/Cargo.toml" | head -n 1)"
if [[ -z "$VERSION" ]]; then
    echo "Error: Could not parse version from Cargo.toml" >&2
    exit 1
fi

ARCH="$(uname -m)"
DIST_DIR="$ROOT_DIR/dist"
WORK_DIR="$DIST_DIR/appimage-build"
APPDIR="$WORK_DIR/AppDir"
OUTPUT_FILE="$DIST_DIR/${APP_NAME}_${VERSION}_${ARCH}.AppImage"

export PATH="$HOME/.cargo/bin:$PATH"

cleanup() {
    rm -rf "$WORK_DIR"
}
trap cleanup EXIT

for cmd in cargo mksquashfs; do
    if ! command -v "$cmd" >/dev/null 2>&1; then
        echo "Error: Required tool '$cmd' not found in PATH." >&2
        exit 1
    fi
done

if command -v appimagetool >/dev/null 2>&1; then
    APPIMAGETOOL="$(command -v appimagetool)"
else
    CACHE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/pro-clock-build"
    APPIMAGETOOL="$CACHE_DIR/appimagetool-${ARCH}.AppImage"
    if [[ ! -x "$APPIMAGETOOL" ]]; then
        if ! command -v curl >/dev/null 2>&1; then
            echo "Error: 'appimagetool' not found and 'curl' is missing to download it." >&2
            exit 1
        fi
        mkdir -p "$CACHE_DIR"
        TOOL_URL="https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-${ARCH}.AppImage"
        echo "==> Downloading appimagetool from $TOOL_URL..."
        curl -fSL "$TOOL_URL" -o "${APPIMAGETOOL}.tmp"
        chmod +x "${APPIMAGETOOL}.tmp"
        mv "${APPIMAGETOOL}.tmp" "$APPIMAGETOOL"
    fi
fi

echo "==> Building release binary..."
cd "$ROOT_DIR"
cargo build --release

BIN_PATH="$ROOT_DIR/target/release/$APP_NAME"
if ldd "$BIN_PATH" | grep -q "not found"; then
    echo "Error: Release binary has missing shared library dependencies:" >&2
    ldd "$BIN_PATH" >&2
    exit 1
fi

echo "==> Staging AppDir at $APPDIR..."
rm -rf "$WORK_DIR"
mkdir -p \
    "$APPDIR/usr/bin" \
    "$APPDIR/usr/share/applications" \
    "$APPDIR/usr/share/icons/hicolor/scalable/apps" \
    "$APPDIR/usr/share/icons/hicolor/256x256/apps" \
    "$APPDIR/usr/share/metainfo" \
    "$APPDIR/usr/share/doc/$APP_NAME/docs/tutorials"

install -m 755 "$BIN_PATH" "$APPDIR/usr/bin/$APP_NAME"

DESKTOP_SRC="$ROOT_DIR/packaging/flatpak/${APP_ID}.desktop"
install -m 644 "$DESKTOP_SRC" "$APPDIR/usr/share/applications/${APP_ID}.desktop"
install -m 644 "$DESKTOP_SRC" "$APPDIR/${APP_ID}.desktop"
if command -v desktop-file-validate >/dev/null 2>&1; then
    echo "==> Validating desktop entry..."
    desktop-file-validate "$APPDIR/${APP_ID}.desktop"
fi

METAINFO_SRC="$ROOT_DIR/packaging/flatpak/${APP_ID}.metainfo.xml"
install -m 644 "$METAINFO_SRC" "$APPDIR/usr/share/metainfo/${APP_ID}.appdata.xml"
if command -v appstreamcli >/dev/null 2>&1; then
    echo "==> Validating AppStream metadata..."
    appstreamcli validate --no-net "$APPDIR/usr/share/metainfo/${APP_ID}.appdata.xml"
fi

SVG_SRC="$ROOT_DIR/assets/pro-clock.svg"
PNG_DEST="$APPDIR/usr/share/icons/hicolor/256x256/apps/${APP_ID}.png"
install -m 644 "$SVG_SRC" "$APPDIR/usr/share/icons/hicolor/scalable/apps/${APP_ID}.svg"

if command -v rsvg-convert >/dev/null 2>&1; then
    rsvg-convert -w 256 -h 256 "$SVG_SRC" -o "$PNG_DEST" || true
elif command -v python3 >/dev/null 2>&1; then
    python3 - "$SVG_SRC" "$PNG_DEST" <<'PY' || true
import sys
import gi
gi.require_version("GdkPixbuf", "2.0")
from gi.repository import GdkPixbuf
pb = GdkPixbuf.Pixbuf.new_from_file_at_size(sys.argv[1], 256, 256)
pb.savev(sys.argv[2], "png", [], [])
PY
elif command -v inkscape >/dev/null 2>&1; then
    inkscape "$SVG_SRC" --export-type=png --export-filename="$PNG_DEST" -w 256 -h 256 || true
fi

if [[ -s "$PNG_DEST" ]]; then
    install -m 644 "$PNG_DEST" "$APPDIR/${APP_ID}.png"
    ln -sf "${APP_ID}.png" "$APPDIR/.DirIcon"
else
    install -m 644 "$SVG_SRC" "$APPDIR/${APP_ID}.svg"
    ln -sf "${APP_ID}.svg" "$APPDIR/.DirIcon"
fi

install -m 644 "$ROOT_DIR/LICENSE" "$APPDIR/usr/share/doc/$APP_NAME/copyright"
install -m 644 "$ROOT_DIR/README.md" "$APPDIR/usr/share/doc/$APP_NAME/README.md"
install -m 644 "$ROOT_DIR/docs/HELP.md" "$APPDIR/usr/share/doc/$APP_NAME/docs/HELP.md"
install -m 644 "$ROOT_DIR/docs/tutorials/"*.md "$APPDIR/usr/share/doc/$APP_NAME/docs/tutorials/"

cat > "$APPDIR/AppRun" <<'EOF'
#!/bin/sh
set -eu
HERE="$(dirname "$(readlink -f "$0")")"
export PATH="$HERE/usr/bin:$PATH"
exec "$HERE/usr/bin/pro-clock" "$@"
EOF
chmod 755 "$APPDIR/AppRun"

echo "==> Packaging AppImage $OUTPUT_FILE..."
mkdir -p "$DIST_DIR"
rm -f "$OUTPUT_FILE"
APPIMAGE_EXTRACT_AND_RUN=1 ARCH="$ARCH" "$APPIMAGETOOL" --no-appstream "$APPDIR" "$OUTPUT_FILE"
chmod 755 "$OUTPUT_FILE"

echo "==> Done! AppImage created at:"
echo "    $OUTPUT_FILE"
echo ""
echo "Run directly with:"
echo "    \"$OUTPUT_FILE\""
