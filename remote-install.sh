#!/usr/bin/env bash
set -euo pipefail

main() {
    local REPO="steffensmartinsen/aka"
    local INSTALL_DIR="$HOME/.local/bin"
    local BINARY_NAME="aka"

    rc_file() {
        case "$(basename "${SHELL:-}")" in
            zsh) echo "$HOME/.zshrc" ;;
            *)   echo "$HOME/.bashrc" ;;
        esac
    }

    # Detect platform → map to the release target triple
    local os arch target
    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os-$arch" in
        Linux-x86_64)   target="x86_64-unknown-linux-gnu" ;;
        Darwin-x86_64)  target="x86_64-apple-darwin" ;;
        Darwin-arm64)   target="aarch64-apple-darwin" ;;
        *)
            echo "Error: unsupported platform $os-$arch" >&2
            echo "You can build from source instead: https://github.com/$REPO" >&2
            exit 1
            ;;
    esac

    local url="https://github.com/$REPO/releases/latest/download/aka-$target.tar.gz"

    # Temp dir with guaranteed cleanup
    local TMP
    TMP="$(mktemp -d)"
    trap 'rm -rf "${TMP:-}"' EXIT
    
    echo "Downloading aka for $target..."
    curl -fsSL "$url" -o "$TMP/aka.tar.gz"

    echo "Extracting..."
    tar xzf "$TMP/aka.tar.gz" -C "$TMP"

    echo "Installing to $INSTALL_DIR..."
    mkdir -p "$INSTALL_DIR"
    cp "$TMP/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"

    echo "Installed $BINARY_NAME to $INSTALL_DIR/$BINARY_NAME"

    # Wire up shell integration automatically. Call by full path since
    # INSTALL_DIR may not be on PATH yet in this session.
    echo "Setting up shell integration..."
    "$INSTALL_DIR/$BINARY_NAME" install || {
        echo "Shell integration step failed — you can run 'aka install' manually later." >&2
    }

    case ":$PATH:" in
        *":$INSTALL_DIR:"*)
            echo "Done. Run 'aka --help' to get started."
            ;;
        *)
            rc="$(rc_file)"
            echo ""
            echo "WARNING: $INSTALL_DIR is not on your PATH."
            echo "Add this line to $rc:"
            echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
            echo "Then restart your shell."
            ;;
    esac
}

main "$@"