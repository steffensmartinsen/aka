#!/usr/bin/env bash
set -euo pipefail

# aka installer — builds a release binary and installs it to ~/.local/bin

INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="aka"

# Print the rc file for the user's shell — used only for the PATH hint.
rc_file() {
    case "$(basename "${SHELL:-}")" in
        zsh) echo "$HOME/.zshrc" ;;
        *)   echo "$HOME/.bashrc" ;;
    esac
}

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
        rc="$(rc_file)"
        echo ""
        echo "WARNING: $INSTALL_DIR is not on your PATH."
        echo "Add this line to $rc:"
        echo ""
        echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
        echo ""
        echo "Then restart your shell."
        ;;
esac