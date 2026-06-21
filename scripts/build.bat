@echo off
REM 跨平台构建脚本 - 为 Windows 编译 uvp

setlocal enabledelayedexpansion

set VERSION=2026.6.0
set BUILD_DIR=build\rs-v%VERSION%

echo 🔨 Building uvp v%VERSION%
echo.

REM 创建构建目录
if not exist %BUILD_DIR% mkdir %BUILD_DIR%

REM 编译当前平台
echo 📦 Building for Windows...
cargo build --release

if errorlevel 1 (
    echo ✗ Failed to build
    exit /b 1
)

REM 复制二进制文件
if exist target\release\uvp.exe (
    copy target\release\uvp.exe %BUILD_DIR%\uvp.exe >nul
    echo ✓ Built: %BUILD_DIR%\uvp.exe
) else (
    echo ✗ Binary not found
    exit /b 1
)

echo.
echo ✅ Build completed!
echo 📁 Binaries are in %BUILD_DIR%\

endlocal
