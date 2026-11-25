<div align="center">

<img src="assets/pro-clock.svg" alt="Pro Clock Logo" width="84" height="84" />

# Pro Clock

**A fast, free, open-source, 100% offline alternative to Clockify — built in Rust for Linux and Windows.**

[![Version](https://img.shields.io/badge/version-0.2.0-E95420?style=flat-square)](Cargo.toml)
[![Built With Rust](https://img.shields.io/badge/built%20with-Rust%20%2B%20egui-000000?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-2ECC71?style=flat-square)](LICENSE)
[![Offline First](https://img.shields.io/badge/100%25%20offline-SQLite-03A9F4?style=flat-square)](docs/HELP.md#data-storage-sqlite-schema--backups)
[![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20Windows-5C6BC0?style=flat-square)](#installation)

[Why Pro Clock?](#why-pro-clock-a-true-offline-clockify-alternative) · [Pro Clock vs. Clockify](#pro-clock-vs-clockify) · [Features](#features) · [Tutorials & Help](docs/tutorials/README.md) · [Installation](#installation)

</div>

---

## Why Pro Clock? A True Offline Clockify Alternative

Clockify popularized clean project-based time tracking, weekly timesheets, and billable reporting—but it still lives in the cloud, requires an account, locks advanced billing and customization features behind paid plans, and puts your work history on someone else's server.

**Pro Clock gives you the familiar Clockify workflow without the cloud, the browser tab, or the paywall:**

- **100% Offline & Local-First**: No sign-up, no internet connection required, no telemetry, and no cloud lock-in. Every project, client, tag, and time entry lives in a single local SQLite file (`pro_clock.db`).
- **Free Forever, Zero Paywalls**: Unlimited projects, clients, tags, hourly rates, per-entry custom rate overrides, visual reports, Pomodoro focus tracking, and CSV/JSON exports are all included out of the box under the [MIT License](LICENSE).
- **Native Rust Speed & Crash Safety**: Compiles to a lightweight native desktop binary (`eframe` / `egui`) that launches instantaneously—no heavy Electron bundle. Running timers persist their `start_time` straight to SQLite so crashes or reboots never lose your clock.
- **Full Data Ownership & Clockify-Compatible Export**: Query your SQLite database anytime, take one-click timestamped backups, or export 13-column Clockify-compatible CSV and JSON reports ready for client invoicing.

---

## Pro Clock vs. Clockify

| Capability | **Pro Clock** | **Clockify** |
|---|---|---|
| **Live Stopwatch & Manual Entry** | ✅ Instant native timer + manual date/time entry | ✅ Web / desktop app |
| **Crash-Safe Running Timer** | ✅ Persisted locally in SQLite | ☁️ Requires server sync |
| **Weekly Timesheet Grid** | ✅ Project × 7-day grid with `1:30`, `1.5`, `45m` quick-log | ✅ Included |
| **Pomodoro Timer + OS Notifications** | ✅ Built-in ring UI, cycles & auto-logging | ⚠️ Separate browser extension / settings |
| **Clients, Projects & Tags** | ✅ Unlimited, color-coded, archivable | ✅ Included (some features gated by tier) |
| **Per-Entry Custom Hourly Rate Overrides** | ✅ Free & built-in | 🔒 Paid plans |
| **Reports & Daily Hours Charts** | ✅ Presets, client/project/billable filters, charts | ✅ Basic free / advanced paid |
| **CSV & JSON Exports** | ✅ Clockify-compatible 13-column CSV + JSON | ✅ Included |
| **100% Offline Operation** | ✅ Works completely air-gapped | ❌ Cloud-first (sync required) |
| **Account / Login Required** | ✅ None | ❌ Account required |
| **Data Ownership & Privacy** | ✅ Single local `pro_clock.db` + 1-click backup | ☁️ Vendor-hosted servers |
| **Price** | ✅ **Free & Open Source (MIT)** | 💲 Freemium / per-seat subscription |
| **App Footprint** | ⚡ Native Rust binary | 🌐 Web browser / Electron |
| **Multi-User Cloud Team Sync** | ❌ Designed for solo / local-first workflows | ✅ Built for cloud teams |

> **Honest fit check:** If you need real-time multi-employee team approvals and cloud seat management, Clockify is built for teams. If you are a freelancer, developer, consultant, or maker who wants **fast, private, zero-subscription personal time tracking and client billing**, Pro Clock is built for you.

---

## Features

- **⏱ Live Time Tracker & Manual Mode**: Start/stop a crash-safe stopwatch, update description/project/tags/billable status *while* a timer is running, click **▶** on any past entry to continue it in one click, or switch to **✏ Manual** mode to log past sessions.
- **📅 Weekly Timesheet Grid**: Clockify-style Project × 7-day matrix with `◀ Prev` / `This Week` / `Next ▶` navigation, configurable Monday/Sunday week start, `+ Add Row`, daily/weekly totals, and rapid cell quick-logging (`1:30`, `0:45`, `1:30:00`, `1.5`, `2`, `1.5h`, `2h`, `45m`).
- **🍅 Pomodoro Timer & Native Notifications**: Focus, Short Break, and Long Break countdown ring with cycle indicator dots, background ticking across all tabs, automatic SQLite time-entry logging, and native OS notifications (`notify-send` on Linux, PowerShell balloon tips on Windows, `osascript` on macOS).
- **📊 Reports, Analytics & Offline Export**: Filter by `Today`, `This Week`, `Last Week`, `This Month`, `Last Month`, `All Time`, or `Custom` date range, plus Project, Client, and `Billable only`. View 4 KPI summary cards, a Daily Hours bar chart, Breakdown by Project bars, and export to **CSV** or **JSON**.
- **💰 Clients, Projects, Tags & 3-Tier Billing**: Color-coded projects (10 preset colors), client notes, archive/restore support, multi-select tags, and automatic 3-tier rate resolution (**Entry Custom Rate** → **Project Hourly Rate** → **Default Hourly Rate**).
- **🎨 Dark & Light Themes + Built-in Help**: Ubuntu Yaru (Dark & Light) and Windows 11 Fluent (Dark & Light) themes, plus a searchable offline **❓ Help & Guide** tab built right into the sidebar.

---

## Documentation & Tutorials

Pro Clock includes both an **in-app offline Help & Guide** (**❓ Help & Guide** in the sidebar) and full Markdown guides in [`docs/`](docs/HELP.md):

- **[Complete Help & Reference Manual](docs/HELP.md)** — Full reference covering every tab, button, duration format, CSV column, SQLite table, and troubleshooting tip.
- **[Step-by-Step Tutorials Hub](docs/tutorials/README.md)**:
  1. [5-Minute Quick Start](docs/tutorials/01-getting-started.md) — Settings, first client/project/tags, first live timer, and Today's report.
  2. [Mastering Daily Time Tracking](docs/tutorials/02-time-tracker-mastery.md) — Live timer edits, seamless task switching, crash recovery, Manual mode, Edit modal, and search filters.
  3. [Fast Weekly Logging with the Timesheet Grid](docs/tutorials/03-weekly-timesheet.md) — Navigating weeks, adding rows, quick-log duration shortcuts, and weekly totals.
  4. [Deep Work & Focus Sessions with Pomodoro](docs/tutorials/04-pomodoro-deep-work.md) — Intervals, desktop notifications, cycle tracking, and auto-logging.
  5. [Freelance Billing, Reports & Client Invoicing](docs/tutorials/05-freelance-billing-invoicing.md) — 3-tier hourly rate hierarchy, filtering reports, CSV/JSON export, and invoice prep.
  6. [Offline Data Ownership, Backups & Switching from Clockify](docs/tutorials/06-backups-sql-and-clockify-migration.md) — One-click backups, safe restores, cross-OS transfers, `sqlite3` queries, and Clockify concept mapping.

---

## Installation

### Installing on Ubuntu / Debian Linux

#### Option 1: Install the `.deb` package (recommended for system-wide install)
```bash
./packaging/build-deb.sh
sudo apt install ./dist/pro-clock_0.2.0_amd64.deb
```
*(Or run `./install.sh --system` to build and install the `.deb` in one command.)*

#### Option 2: User-local install (no `sudo` required)
```bash
./install.sh
```
This installs the binary to `~/.local/bin/pro-clock` and adds **Pro Clock** to your Ubuntu Applications menu.

To uninstall (preserves your SQLite database):
```bash
./uninstall.sh
```

> **Tip for Ubuntu notifications:** Ensure `libnotify-bin` is installed (`sudo apt install libnotify-bin`) for desktop notifications on timer stop and Pomodoro phase transitions.

---

### Installing on Windows

Prebuilt Windows binaries are not currently published, so Windows users can build the installer directly from source in a few steps.

#### Prerequisites
1. **[Rust](https://rustup.rs/)** (installs `cargo` — when prompted by `rustup-init.exe`, also install the **Visual Studio C++ Build Tools**).
2. **[Git](https://git-scm.com/download/win)** (or download and extract the repository ZIP from GitHub).
3. **[Inno Setup 6](https://jrsoftware.org/isinfo.php)** *(only needed for Option 1's `.exe` setup wizard)*:
   ```powershell
   winget install JRSoftware.InnoSetup
   ```

#### Step 1: Clone the repository
```powershell
git clone https://github.com/andyswings/Pro_Clock.git
cd Pro_Clock
```

#### Option 1: Build the Windows Setup Wizard (`.exe` installer, recommended)
```powershell
.\packaging\windows\build-installer.ps1
```
*(If PowerShell blocks running scripts, run `powershell -ExecutionPolicy Bypass -File .\packaging\windows\build-installer.ps1` instead.)*

This creates two files in `dist\`:
- `dist\ProClock-Setup-0.2.0-x64.exe` — double-click to run the graphical Windows installer (Start Menu shortcut, optional Desktop shortcut, Add/Remove Programs uninstaller; works without admin rights or system-wide).
- `dist\ProClock-0.2.0-windows-x64-portable.zip` — portable no-install archive (pass `-PortableOnly` if you don't have Inno Setup installed).

#### Option 2: Direct user-local install (no Inno Setup required)
```powershell
.\install.ps1
```
This compiles `pro-clock.exe`, installs it to `%LOCALAPPDATA%\ProClock`, and adds **Pro Clock** to your Start Menu (pass `-DesktopShortcut` for a Desktop icon).

To uninstall (preserves your SQLite database):
```powershell
.\uninstall.ps1
```

---

### Building from Source (Linux / Windows / macOS)

```bash
cargo build --release
cargo run --release
```

Run the unit test suite:
```bash
cargo test
```

---

## Where Your Data Lives

All of your time tracking data stays in a single local SQLite database on your machine:

| Operating System | SQLite Database Path |
|---|---|
| **Linux** | `~/.local/share/pro-clock/pro_clock.db` |
| **Windows** | `%APPDATA%\proclock\pro-clock\data\pro_clock.db` |
| **macOS** | `~/Library/Application Support/org.proclock.pro-clock/pro_clock.db` |

You can create a timestamped backup copy anytime via **⚙ Settings → Create Backup Copy**, or open your data folder directly with **📂 Open Data Folder**. See [Tutorial 6](docs/tutorials/06-backups-sql-and-clockify-migration.md) for backup, restore, and read-only `sqlite3` query examples.

---

## Project Structure

```text
Pro_Clock/
├── src/
│   ├── main.rs          # App entry point, viewport config, generated window icon
│   ├── models.rs        # Data models, rate calculation, duration parsing/formatting
│   ├── db.rs            # Local SQLite persistence (WAL mode) & CRUD queries
│   ├── export.rs        # Clockify-compatible CSV and JSON export
│   ├── notify.rs        # Cross-platform desktop notifications & folder opener
│   ├── theme.rs         # Ubuntu Yaru & Windows 11 (Dark/Light) palettes and reusable UI widgets
│   └── ui/              # Tracker, Timesheet, Pomodoro, Reports, Manage & Help views
├── docs/
│   ├── HELP.md          # Complete offline reference manual
│   └── tutorials/       # 6 step-by-step tutorials + learning paths hub
├── packaging/           # Debian (.deb), .desktop launcher, and Windows Inno Setup scripts
└── assets/              # Scalable SVG & ICO app icons
```

---

## License

MIT License — see [LICENSE](LICENSE) for details.
