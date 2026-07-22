#!/usr/bin/env bash
set -euo pipefail

# aka installer — builds a release binary and installs it to ~/.local/bin

INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="aka"

echo "Building release binary..."
cargo build --release

echo "Installing to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"
cp "target/release/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"

echo "Installed $BINARY_NAME to $INSTALL_DIR/$BINARY_NAME"

# Warn if the install dir isn't on PATH — the binary won't be found otherwise
case ":$PATH:" in
    *":$INSTALL_DIR:"*)
        echo "You can now run: aka --help"
        ;;
    *)
        echo ""
        echo "WARNING: $INSTALL_DIR is not on your PATH."
        echo "Add this line to your ~/.bashrc (or ~/.zshrc on macOS):"
        echo ""
        echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
        echo ""
        echo "Then run: source ~/.bashrc"
        ;;
esac