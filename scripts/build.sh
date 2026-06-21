#!/bin/bash
# 跨平台构建脚本 - 为 Windows、macOS、Linux 编译 uvp

set -e

VERSION="2026.6.0"
BUILD_DIR="build/rs-v${VERSION}"

echo "🔨 Building uvp v${VERSION}"
echo ""

# 创建构建目录
mkdir -p ${BUILD_DIR}

# 定义目标平台
declare -A TARGETS=(
    ["x86_64-pc-windows-msvc"]="uvp.exe"
    ["x86_64-apple-darwin"]="uvp-macos-x64"
    ["aarch64-apple-darwin"]="uvp-macos-arm64"
    ["x86_64-unknown-linux-gnu"]="uvp-linux-x64"
    ["aarch64-unknown-linux-gnu"]="uvp-linux-arm64"
)

# 检查是否安装了目标平台的支持
check_target() {
    local target=$1
    if ! rustup target list | grep -q "${target} (installed)"; then
        echo "⚠️  Target ${target} not installed. Installing..."
        rustup target add ${target}
    fi
}

# 编译指定平台
build_target() {
    local target=$1
    local output_name=$2
    
    echo "📦 Building for ${target}..."
    check_target ${target}
    
    cargo build --release --target ${target}
    
    # 复制二进制文件
    local src="target/${target}/release/${output_name}"
    local dst="${BUILD_DIR}/uvp-${target}"
    
    if [ -f "${src}" ]; then
        cp "${src}" "${dst}"
        echo "✓ Built: ${dst}"
    else
        echo "✗ Failed to build ${target}"
        return 1
    fi
}

# 构建当前平台
build_current() {
    echo "📦 Building for current platform..."
    cargo build --release
    
    local output="target/release/uvp"
    if [ -f "${output}.exe" ]; then
        cp "${output}.exe" "${BUILD_DIR}/uvp.exe"
        echo "✓ Built: ${BUILD_DIR}/uvp.exe"
    else
        cp "${output}" "${BUILD_DIR}/uvp"
        echo "✓ Built: ${BUILD_DIR}/uvp"
    fi
}

# 主逻辑
case "${1}" in
    "all")
        echo "🌍 Building for all platforms..."
        for target in "${!TARGETS[@]}"; do
            build_target "${target}" "${TARGETS[$target]}" || true
        done
        ;;
    "current")
        build_current
        ;;
    *)
        echo "Usage: $0 {all|current}"
        echo ""
        echo "  all     - Build for all supported platforms"
        echo "  current - Build for current platform only"
        exit 1
        ;;
esac

echo ""
echo "✅ Build completed!"
echo "📁 Binaries are in ${BUILD_DIR}/"
