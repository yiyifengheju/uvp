# uvp installer for Windows (PowerShell)
# Usage: irm https://github.com/yiyifengheju/uvp/releases/latest/download/install.ps1 | iex

$ErrorActionPreference = "Stop"

$Repo = "yiyifengheju/uvp"
$InstallDir = if ($env:UVP_INSTALL_DIR) { $env:UVP_INSTALL_DIR } else { "$env:USERPROFILE\.uvp\bin" }

function Get-Asset {
    $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
    switch ($arch) {
        "X64"  { return "uvp-windows-x86_64.exe" }
        default {
            Write-Error "[FAIL] Unsupported architecture: $arch"
            exit 1
        }
    }
}

function Get-LatestVersion {
    $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest" -UseBasicParsing
    return $response.tag_name
}

function Add-ToPath {
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($currentPath -split ";" | Where-Object { $_ -eq $InstallDir }) {
        Write-Host "[INFO] $InstallDir is already in PATH"
        return
    }
    $newPath = "$InstallDir;$currentPath"
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    $env:Path = "$InstallDir;$env:Path"
    Write-Host "[OK] Added $InstallDir to user PATH (permanent)"
}

function Main {
    Write-Host "[INSTALL] uvp installer"
    Write-Host ""

    $asset = Get-Asset
    Write-Host "[INFO] Platform: $asset"

    $version = Get-LatestVersion
    if (-not $version) {
        Write-Error "[FAIL] Could not determine latest version"
        exit 1
    }
    Write-Host "[INFO] Latest version: $version"

    $url = "https://github.com/$Repo/releases/download/$version/$asset"
    Write-Host "[INFO] Downloading from: $url"
    Write-Host ""

    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }

    $dest = Join-Path $InstallDir "uvp.exe"
    Invoke-WebRequest -Uri $url -OutFile $dest -UseBasicParsing
    Write-Host "[OK] Installed: $dest"
    Write-Host ""

    Add-ToPath
    Write-Host ""

    Write-Host "[DONE] Run 'uvp --version' to verify."
    Write-Host "       You may need to restart your terminal for PATH changes to take effect."
}

Main
