#!/usr/bin/env bash
set -euo pipefail

APP_NAME="pro-clock"

if dpkg -s "$APP_NAME" >/dev/null 2>&1; then
    echo "==> Removing system-wide $APP_NAME package (requires sudo)..."
    sudo apt remove -y "$APP_NAME"
fi

rm -f "$HOME/.local/bin/$APP_NAME"
rm -f "$HOME/.local/share/applications/pro-clock.desktop"
rm -f "$HOME/.local/share/icons/hicolor/scalable/apps/pro-clock.svg"
rm -rf "$HOME/.local/share/doc/$APP_NAME"

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$HOME/.local/share/applications" >/dev/null 2>&1 || true
fi

echo "==> Pro Clock uninstalled."
echo "Note: Your local database in ~/.local/share/pro-clock/ was preserved so you don't lose data."
