#!/bin/bash
# 构建脚本 - Windows x86_64 + macOS Apple Silicon (M3)
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

VERSION="2026.6.0"
BUILD_DIR="build/rs-v${VERSION}"
CARGO_DIR="src/ruvp"

MAC_TARGET="aarch64-apple-darwin"

echo "[BUILD] uvp v${VERSION}"
echo ""

mkdir -p "${BUILD_DIR}"

check_target() {
    local target=$1
    if ! rustup target list --installed | grep -q "^${target}$"; then
        echo "[INFO] Installing target ${target}..."
        rustup target add "${target}"
    fi
}

build_native() {
    echo "[BUILD] Compiling for current platform (native)..."
    cargo build --release --manifest-path "${CARGO_DIR}/Cargo.toml"

    if [ -f "${CARGO_DIR}/target/release/uvp.exe" ]; then
        cp "${CARGO_DIR}/target/release/uvp.exe" "${BUILD_DIR}/uvp-windows-x86_64.exe"
        echo "[OK] Built: ${BUILD_DIR}/uvp-windows-x86_64.exe"
    elif [ -f "${CARGO_DIR}/target/release/uvp" ]; then
        cp "${CARGO_DIR}/target/release/uvp" "${BUILD_DIR}/uvp-$(uname -m)"
        echo "[OK] Built: ${BUILD_DIR}/uvp-$(uname -m)"
    else
        echo "[FAIL] Native binary not found"
        return 1
    fi
}

build_macos() {
    echo "[BUILD] Compiling for macOS Apple Silicon (${MAC_TARGET})..."
    check_target "${MAC_TARGET}"

    cargo build --release --manifest-path "${CARGO_DIR}/Cargo.toml" --target "${MAC_TARGET}"

    local src="${CARGO_DIR}/target/${MAC_TARGET}/release/uvp"
    if [ -f "${src}" ]; then
        cp "${src}" "${BUILD_DIR}/uvp-macos-aarch64"
        echo "[OK] Built: ${BUILD_DIR}/uvp-macos-aarch64"
    else
        echo "[FAIL] macOS binary not found: ${src}"
        return 1
    fi
}

case "${1}" in
    "windows")
        build_native
        ;;
    "macos")
        build_macos
        ;;
    "all"|"")
        build_native || echo "[WARN] Native build failed"
        echo ""
        build_macos || echo "[WARN] macOS build failed (expected on non-macOS without cross toolchain)"
        ;;
    *)
        echo "Usage: $0 [windows|macos|all]"
        echo ""
        echo "  windows - Build for current platform (native)"
        echo "  macos   - Build for macOS Apple Silicon (M3)"
        echo "  all     - Build both (default)"
        exit 1
        ;;
esac

echo ""
echo "[DONE] Build completed! Binaries are in ${BUILD_DIR}/"
