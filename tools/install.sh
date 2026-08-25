#!/usr/bin/env bash
set -e

echo "🌸 Installing Sorayunara Programming Language Toolchain..."

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
  linux)
    if [ "$ARCH" = "x86_64" ]; then
      TARGET="x86_64-unknown-linux-gnu"
    elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
      TARGET="aarch64-unknown-linux-gnu"
    else
      echo "Unsupported architecture: $ARCH"
      exit 1
    fi
    EXT="tar.gz"
    ;;
  darwin)
    if [ "$ARCH" = "arm64" ]; then
      TARGET="aarch64-apple-darwin"
    else
      TARGET="x86_64-apple-darwin"
    fi
    EXT="tar.gz"
    ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

LATEST_TAG=$(curl -s https://api.github.com/repos/Sorayunara/sorayunara/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
if [ -z "$LATEST_TAG" ]; then
  LATEST_TAG="v0.1.0"
fi

URL="https://github.com/Sorayunara/sorayunara/releases/download/${LATEST_TAG}/sora-${LATEST_TAG}-${TARGET}.${EXT}"
INSTALL_DIR="$HOME/.sorayunara/bin"
mkdir -p "$INSTALL_DIR"

echo "Downloading Sorayunara ${LATEST_TAG} (${TARGET})..."
TMP_DIR=$(mktemp -d)
curl -fsSL "$URL" -o "$TMP_DIR/sora.${EXT}"
tar -xzf "$TMP_DIR/sora.${EXT}" -C "$TMP_DIR"

cp "$TMP_DIR/sora-${LATEST_TAG}-${TARGET}/sorayunara" "$INSTALL_DIR/sora"
chmod +x "$INSTALL_DIR/sora"
rm -rf "$TMP_DIR"

echo "✅ Sorayunara successfully installed to $INSTALL_DIR/sora!"
echo ""
echo "To add Sorayunara to your PATH, add this to your ~/.bashrc or ~/.zshrc:"
echo "  export PATH=\"\$HOME/.sorayunara/bin:\$PATH\""
