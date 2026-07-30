#!/bin/bash
# uvp installer — download from GitHub Releases, install to ~/.uvp/bin, add to PATH
set -e

REPO="yiyifengheju/uvp"
INSTALL_DIR="${UVP_INSTALL_DIR:-$HOME/.uvp/bin}"

get_target() {
    local os arch
    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Darwin)
            case "$arch" in
                arm64|aarch64) echo "uvp-macos-aarch64" ;;
                *) echo "[FAIL] Unsupported macOS architecture: $arch" >&2; exit 1 ;;
            esac
            ;;
        Linux)
            case "$arch" in
                x86_64|amd64) echo "uvp-linux-x86_64" ;;
                aarch64|arm64) echo "uvp-linux-aarch64" ;;
                *) echo "[FAIL] Unsupported Linux architecture: $arch" >&2; exit 1 ;;
            esac
            ;;
        *) echo "[FAIL] Unsupported OS: $os" >&2; exit 1 ;;
    esac
}

get_latest_version() {
    curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name"' \
        | sed 's/.*"tag_name": *"//;s/".*//'
}

main() {
    echo "[INSTALL] uvp installer"
    echo ""

    local target version url

    target="$(get_target)"
    echo "[INFO] Platform: ${target}"

    version="$(get_latest_version)"
    if [ -z "$version" ]; then
        echo "[FAIL] Could not determine latest version" >&2
        exit 1
    fi
    echo "[INFO] Latest version: ${version}"

    url="https://github.com/${REPO}/releases/download/${version}/${target}"
    echo "[INFO] Downloading from: ${url}"
    echo ""

    mkdir -p "$INSTALL_DIR"

    curl -fsSL "$url" -o "${INSTALL_DIR}/uvp"
    chmod +x "${INSTALL_DIR}/uvp"

    # macOS: remove quarantine attribute to avoid Gatekeeper block
    if [ "$(uname -s)" = "Darwin" ]; then
        xattr -cr "${INSTALL_DIR}/uvp" 2>/dev/null || true
    fi

    echo "[OK] Installed: ${INSTALL_DIR}/uvp"
    echo ""

    add_to_path
    echo ""

    echo "[DONE] Run 'uvp --version' to verify."
    echo "       You may need to restart your shell or run: source ~/.bashrc"
}

add_to_path() {
    local line="export PATH=\"${INSTALL_DIR}:\$PATH\""

    # already in PATH
    if echo "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
        echo "[INFO] ${INSTALL_DIR} is already in PATH"
        return
    fi

    local added=false

    for rc in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
        if [ -f "$rc" ]; then
            if ! grep -qF "$INSTALL_DIR" "$rc"; then
                echo "" >> "$rc"
                echo "# uvp" >> "$rc"
                echo "$line" >> "$rc"
                echo "[OK] Added to PATH in ${rc}"
                added=true
            fi
        fi
    done

    # if zsh is default but no .zshrc existed, create it
    if [ "$added" = false ] && [ "$(basename "$SHELL")" = "zsh" ]; then
        echo "" >> "$HOME/.zshrc"
        echo "# uvp" >> "$HOME/.zshrc"
        echo "$line" >> "$HOME/.zshrc"
        echo "[OK] Added to PATH in ~/.zshrc"
        added=true
    fi

    if [ "$added" = false ]; then
        echo "" >> "$HOME/.bashrc"
        echo "# uvp" >> "$HOME/.bashrc"
        echo "$line" >> "$HOME/.bashrc"
        echo "[OK] Added to PATH in ~/.bashrc"
    fi

    export PATH="${INSTALL_DIR}:$PATH"
}

main
