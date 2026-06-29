@echo off
chcp 65001 >nul 2>&1
setlocal enabledelayedexpansion

REM Build script for uvp (Windows + macOS cross-compile)

set SCRIPT_DIR=%~dp0
set PROJECT_ROOT=%SCRIPT_DIR%..
cd /d "%PROJECT_ROOT%"

set VERSION=2026.6.0
set BUILD_DIR=build\rs-v%VERSION%
set CARGO_DIR=src\ruvp

set MAC_TARGET=aarch64-apple-darwin

echo [BUILD] uvp v%VERSION%
echo.

if not exist "%BUILD_DIR%" mkdir "%BUILD_DIR%"

REM === Windows (native) ===
echo [BUILD] Compiling for Windows (native)...
cargo build --release --manifest-path "%CARGO_DIR%\Cargo.toml"

if errorlevel 1 (
    echo [FAIL] Windows build failed
    pause
    exit /b 1
)

if exist "%CARGO_DIR%\target\release\uvp.exe" (
    copy "%CARGO_DIR%\target\release\uvp.exe" "%BUILD_DIR%\uvp-windows-x86_64.exe" >nul
    echo [OK] Built: %BUILD_DIR%\uvp-windows-x86_64.exe
) else (
    echo [FAIL] Windows binary not found
    pause
    exit /b 1
)

echo.

REM === macOS Apple Silicon (cross-compile) ===
echo [BUILD] Compiling for macOS Apple Silicon (%MAC_TARGET%)...
echo [INFO] Requires: rustup target add %MAC_TARGET%
echo [INFO] Requires: macOS cross-compile toolchain (osxcross or run on macOS)
echo.

cargo build --release --manifest-path "%CARGO_DIR%\Cargo.toml" --target %MAC_TARGET%

if errorlevel 1 (
    echo [WARN] macOS build failed - this is expected on Windows without osxcross.
    echo [WARN] To build macOS binary, run build.sh on a macOS machine.
    echo.
    goto :done
)

if exist "%CARGO_DIR%\target\%MAC_TARGET%\release\uvp" (
    copy "%CARGO_DIR%\target\%MAC_TARGET%\release\uvp" "%BUILD_DIR%\uvp-macos-aarch64" >nul
    echo [OK] Built: %BUILD_DIR%\uvp-macos-aarch64
)

:done
echo.
echo [DONE] Build completed! Binaries are in %BUILD_DIR%\
pause

endlocal
