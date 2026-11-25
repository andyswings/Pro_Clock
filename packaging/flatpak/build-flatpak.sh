#!/usr/bin/env bash
set -euo pipefail

APP_NAME="pro-clock"
APP_ID="io.github.andyswings.ProClock"
RUNTIME_BRANCH="25.08"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
VERSION="$(sed -nE 's/^version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' "$ROOT_DIR/Cargo.toml" | head -n 1)"
if [[ -z "$VERSION" ]]; then
    echo "Error: Could not parse version from Cargo.toml" >&2
    exit 1
fi

export PATH="$HOME/.cargo/bin:$PATH"

for cmd in cargo flatpak flatpak-builder; do
    if ! command -v "$cmd" >/dev/null 2>&1; then
        echo "Error: Required tool '$cmd' not found in PATH." >&2
        exit 1
    fi
done

ARCH="$(flatpak --default-arch)"
DIST_DIR="$ROOT_DIR/dist"
VENDOR_DIR="$DIST_DIR/flatpak-vendor"
WORK_DIR="$DIST_DIR/flatpak-build"
BUNDLE_FILE="$DIST_DIR/${APP_NAME}_${VERSION}_${ARCH}.flatpak"

cleanup() {
    rm -rf "$VENDOR_DIR" "$WORK_DIR"
}
trap cleanup EXIT

echo "==> Ensuring Flathub user remote and Freedesktop $RUNTIME_BRANCH runtime/SDK..."
flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo

REQUIRED_REFS=(
    "org.freedesktop.Platform//$RUNTIME_BRANCH"
    "org.freedesktop.Sdk//$RUNTIME_BRANCH"
    "org.freedesktop.Sdk.Extension.rust-stable//$RUNTIME_BRANCH"
)
MISSING_REFS=()
for ref in "${REQUIRED_REFS[@]}"; do
    if ! flatpak info --user "$ref" >/dev/null 2>&1 && ! flatpak info --system "$ref" >/dev/null 2>&1; then
        MISSING_REFS+=("$ref")
    fi
done

if (( ${#MISSING_REFS[@]} > 0 )); then
    echo "==> Installing missing Flatpak refs: ${MISSING_REFS[*]}"
    flatpak install --user -y flathub "${MISSING_REFS[@]}"
fi

echo "==> Validating desktop entry and AppStream metainfo..."
flatpak run --filesystem="$SCRIPT_DIR:ro" --command=desktop-file-validate "org.freedesktop.Sdk//$RUNTIME_BRANCH" \
    "$SCRIPT_DIR/$APP_ID.desktop"
flatpak run --filesystem="$SCRIPT_DIR:ro" --command=appstreamcli "org.freedesktop.Sdk//$RUNTIME_BRANCH" \
    validate --no-net "$SCRIPT_DIR/$APP_ID.metainfo.xml"

echo "==> Vendoring Cargo crates into $VENDOR_DIR..."
rm -rf "$VENDOR_DIR" "$WORK_DIR"
mkdir -p "$VENDOR_DIR" "$WORK_DIR"
(
    cd "$ROOT_DIR"
    cargo vendor --locked "$VENDOR_DIR/vendor" >/dev/null
)

cat > "$VENDOR_DIR/config.toml" <<'EOF'
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
EOF

echo "==> Building Flatpak inside Freedesktop SDK $RUNTIME_BRANCH..."
flatpak-builder \
    --force-clean \
    --user \
    --state-dir="$WORK_DIR/state" \
    --repo="$WORK_DIR/repo" \
    "$WORK_DIR/build-dir" \
    "$SCRIPT_DIR/$APP_ID.yml"

echo "==> Verifying built Flatpak binary inside runtime..."
flatpak-builder --run "$WORK_DIR/build-dir" "$SCRIPT_DIR/$APP_ID.yml" \
    sh -c 'ldd /app/bin/pro-clock && if ldd /app/bin/pro-clock | grep -q "not found"; then exit 1; fi && grep -a -m1 "Windows 11" /app/bin/pro-clock >/dev/null'

echo "==> Creating single-file bundle $BUNDLE_FILE..."
flatpak build-bundle \
    --runtime-repo=https://flathub.org/repo/flathub.flatpakrepo \
    "$WORK_DIR/repo" \
    "$BUNDLE_FILE" \
    "$APP_ID"

echo "==> Done! Flatpak bundle created at:"
echo "    $BUNDLE_FILE"
echo ""
echo "Install with:"
echo "    flatpak install --user \"$BUNDLE_FILE\""
echo "Run with:"
echo "    flatpak run $APP_ID"
