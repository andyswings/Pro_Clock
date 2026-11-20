#Requires -Version 5.1
<#
.SYNOPSIS
    Installs Pro Clock for the current Windows user (%LOCALAPPDATA%\ProClock) with Start Menu shortcut.
.DESCRIPTION
    Mirrors install.sh on Windows. No administrator privileges required.
    Pass -DesktopShortcut to also create a Desktop shortcut.
#>
[CmdletBinding()]
param(
    [switch]$DesktopShortcut
)

$ErrorActionPreference = "Stop"

$RootDir = $PSScriptRoot
$BinSrc = Join-Path $RootDir "target\release\pro-clock.exe"
$InstallDir = Join-Path $env:LOCALAPPDATA "ProClock"
$InstalledExe = Join-Path $InstallDir "pro-clock.exe"
$InstalledIcon = Join-Path $InstallDir "pro-clock.ico"

Write-Host "==> Installing Pro Clock for current user ($InstallDir)..." -ForegroundColor Cyan

if (-not (Test-Path $BinSrc)) {
    Write-Host "==> Release binary not found; compiling with cargo..." -ForegroundColor Cyan
    Push-Location $RootDir
    try {
        cargo build --release
        if ($LASTEXITCODE -ne 0) { throw "cargo build --release failed." }
    } finally {
        Pop-Location
    }
}

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Copy-Item -Force $BinSrc $InstalledExe
Copy-Item -Force (Join-Path $RootDir "assets\pro-clock.ico") $InstalledIcon
Copy-Item -Force (Join-Path $RootDir "LICENSE") (Join-Path $InstallDir "LICENSE")

# Create Start Menu shortcut
$WshShell = New-Object -ComObject WScript.Shell
$StartMenuDir = Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs"
$StartMenuShortcut = Join-Path $StartMenuDir "Pro Clock.lnk"

$Shortcut = $WshShell.CreateShortcut($StartMenuShortcut)
$Shortcut.TargetPath = $InstalledExe
$Shortcut.WorkingDirectory = $InstallDir
$Shortcut.IconLocation = "$InstalledIcon,0"
$Shortcut.Description = "Pro Clock — Free, 100% offline time tracker"
$Shortcut.Save()

if ($DesktopShortcut) {
    $DesktopPath = [Environment]::GetFolderPath("Desktop")
    $DesktopLink = Join-Path $DesktopPath "Pro Clock.lnk"
    $DShortcut = $WshShell.CreateShortcut($DesktopLink)
    $DShortcut.TargetPath = $InstalledExe
    $DShortcut.WorkingDirectory = $InstallDir
    $DShortcut.IconLocation = "$InstalledIcon,0"
    $DShortcut.Description = "Pro Clock — Free, 100% offline time tracker"
    $DShortcut.Save()
}

Write-Host ""
Write-Host "==> Pro Clock installed successfully!" -ForegroundColor Green
Write-Host "    Binary:     $InstalledExe"
Write-Host "    Start Menu: $StartMenuShortcut"
if ($DesktopShortcut) {
    Write-Host "    Desktop:    $DesktopLink"
}
Write-Host ""
Write-Host "You can now launch 'Pro Clock' from the Windows Start Menu."
Write-Host "(Tip: Run '.\packaging\windows\build-installer.ps1' to build a standalone Windows Setup .exe.)"
