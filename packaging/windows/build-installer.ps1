#Requires -Version 5.1
<#
.SYNOPSIS
    Builds the Pro Clock release binary and Windows installer (Inno Setup .exe) + portable .zip.
.DESCRIPTION
    Mirrors packaging/build-deb.sh on Windows.
    Requires Rust (cargo) and Inno Setup 6 (ISCC.exe) for the .exe installer.
    If Inno Setup is not installed, use -PortableOnly to build just the portable .zip.
#>
[CmdletBinding()]
param(
    [switch]$SkipBuild,
    [switch]$PortableOnly
)

$ErrorActionPreference = "Stop"

$RootDir = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$DistDir = Join-Path $RootDir "dist"
$CargoToml = Get-Content (Join-Path $RootDir "Cargo.toml") -Raw
if ($CargoToml -match '(?m)^version\s*=\s*"([^"]+)"') {
    $Version = $Matches[1]
} else {
    throw "Could not parse version from Cargo.toml"
}

if (-not $SkipBuild) {
    Write-Host "==> Building Pro Clock v$Version (release)..." -ForegroundColor Cyan
    Push-Location $RootDir
    try {
        cargo build --release
        if ($LASTEXITCODE -ne 0) { throw "cargo build --release failed." }
    } finally {
        Pop-Location
    }
}

$ExePath = Join-Path $RootDir "target\release\pro-clock.exe"
if (-not (Test-Path $ExePath)) {
    throw "Release binary not found at $ExePath"
}

New-Item -ItemType Directory -Force -Path $DistDir | Out-Null

# 1. Build portable ZIP archive
$ZipPath = Join-Path $DistDir "ProClock-$Version-windows-x64-portable.zip"
$StageDir = Join-Path $DistDir "ProClock-$Version-windows-x64"
if (Test-Path $StageDir) { Remove-Item -Recurse -Force $StageDir }
if (Test-Path $ZipPath) { Remove-Item -Force $ZipPath }
New-Item -ItemType Directory -Force -Path $StageDir | Out-Null

Copy-Item $ExePath (Join-Path $StageDir "pro-clock.exe")
Copy-Item (Join-Path $RootDir "LICENSE") (Join-Path $StageDir "LICENSE")
Copy-Item (Join-Path $RootDir "README.md") (Join-Path $StageDir "README.md")
Copy-Item -Recurse -Force (Join-Path $RootDir "docs") (Join-Path $StageDir "docs")

Write-Host "==> Creating portable archive: $ZipPath" -ForegroundColor Cyan
Compress-Archive -Path "$StageDir\*" -DestinationPath $ZipPath -Force
Remove-Item -Recurse -Force $StageDir

if ($PortableOnly) {
    Write-Host "==> Done! Portable archive created at:" -ForegroundColor Green
    Write-Host "    $ZipPath"
    exit 0
}

# 2. Locate Inno Setup Compiler (ISCC.exe)
$IsccCandidates = @(
    (Get-Command iscc -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source -ErrorAction SilentlyContinue),
    "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe",
    "$env:ProgramFiles\Inno Setup 6\ISCC.exe",
    "$env:LOCALAPPDATA\Programs\Inno Setup 6\ISCC.exe"
) | Where-Object { $_ -and (Test-Path $_) }

$Iscc = $IsccCandidates | Select-Object -First 1
if (-not $Iscc) {
    Write-Warning "Inno Setup 6 (ISCC.exe) was not found."
    Write-Host "Install Inno Setup 6 with:  winget install JRSoftware.InnoSetup" -ForegroundColor Yellow
    Write-Host "Or re-run with -PortableOnly to skip the setup wizard." -ForegroundColor Yellow
    Write-Host "Portable ZIP is still ready at: $ZipPath" -ForegroundColor Green
    exit 1
}

$IssPath = Join-Path $PSScriptRoot "pro-clock.iss"
Write-Host "==> Compiling Windows installer with Inno Setup ($Iscc)..." -ForegroundColor Cyan
& $Iscc "/DAppVersion=$Version" $IssPath
if ($LASTEXITCODE -ne 0) {
    throw "ISCC.exe failed with exit code $LASTEXITCODE"
}

$InstallerPath = Join-Path $DistDir "ProClock-Setup-$Version-x64.exe"
Write-Host ""
Write-Host "==> Done! Windows artifacts created:" -ForegroundColor Green
Write-Host "    Installer: $InstallerPath"
Write-Host "    Portable:  $ZipPath"
