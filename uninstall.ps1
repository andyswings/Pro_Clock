#Requires -Version 5.1
<#
.SYNOPSIS
    Uninstalls Pro Clock installed via install.ps1 while keeping your SQLite database safe.
#>
$ErrorActionPreference = "Stop"

$InstallDir = Join-Path $env:LOCALAPPDATA "ProClock"
$StartMenuShortcut = Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs\Pro Clock.lnk"
$DesktopShortcut = Join-Path ([Environment]::GetFolderPath("Desktop")) "Pro Clock.lnk"

Write-Host "==> Removing Pro Clock from $InstallDir..." -ForegroundColor Cyan

if (Test-Path $InstallDir) {
    Remove-Item -Recurse -Force $InstallDir
}
if (Test-Path $StartMenuShortcut) {
    Remove-Item -Force $StartMenuShortcut
}
if (Test-Path $DesktopShortcut) {
    Remove-Item -Force $DesktopShortcut
}

Write-Host "==> Pro Clock uninstalled." -ForegroundColor Green
Write-Host "    Note: Your time-tracking database at '$env:APPDATA\proclock\pro-clock\data\pro_clock.db' was preserved."
Write-Host "    Delete that folder manually if you want to erase all tracked data."
