# Pro Clock

A free, open-source, **100% offline** Clockify replacement built in Rust for Linux and Windows.

No accounts, no subscriptions, no cloud lock-in, and no telemetry — all of your time tracking data stays in a single local SQLite database on your machine:
- **Linux**: `~/.local/share/pro-clock/pro_clock.db`
- **Windows**: `%APPDATA%\proclock\pro-clock\data\pro_clock.db`

## Features

- **Live Timer & Manual Time Entry**: Start/stop a live stopwatch (crash-safe — survives app restarts) or switch to Manual mode to log past sessions with custom date/start/end times.
- **Weekly Timesheet Grid**: Classic Clockify-style project × day weekly matrix with daily/weekly totals, week navigation, and one-click quick-logging (`1:30`, `1.5`, `45m`, etc.).
- **Pomodoro Timer & Ubuntu Desktop Notifications**: Built-in Focus / Short Break / Long Break timer with circular progress ring, cycle tracking, automatic time-entry logging, and native Ubuntu desktop notifications via `notify-send`.
- **Full Entry Editing & Continue**: Edit descriptions, projects, tags, start/end times, billable status, or custom hourly rates; click **▶** on any past entry to immediately continue tracking it.
- **Projects, Clients & Tags**: Color-coded projects linked to clients, default billable flags, per-project hourly rates, archive/restore support, and multi-select tags.
- **Reports & Analytics**: Filter by date presets (`Today`, `This Week`, `Last Week`, `This Month`, `Last Month`, `All Time`, or `Custom`), project, client, or billable status; view KPI summary cards, a daily hours bar chart, and project breakdown bars.
- **Offline Export & Backup**: Export filtered reports to Clockify-compatible **CSV** or **JSON**, and create one-click local SQLite backups.
- **Ubuntu-Native Feel**: Clean Dark and Light themes inspired by Ubuntu Yaru / GNOME Adwaita, with a native `.desktop` launcher and scalable SVG icon.

## Installing on Ubuntu

### Option 1: Install the `.deb` package (recommended for system-wide install)
```bash
./packaging/build-deb.sh
sudo apt install ./dist/pro-clock_0.2.0_amd64.deb
```

### Option 2: User-local install (no `sudo` required)
```bash
./install.sh
```
This installs the binary to `~/.local/bin/pro-clock` and adds **Pro Clock** to your Ubuntu Applications menu.

To uninstall:
```bash
./uninstall.sh
```

## Installing on Windows

Prebuilt Windows binaries are not currently published, so Windows users can build the installer directly from source in a few steps.

### Prerequisites
1. **[Rust](https://rustup.rs/)** (installs `cargo` — when prompted by `rustup-init.exe`, also install the **Visual Studio C++ Build Tools**).
2. **[Git](https://git-scm.com/download/win)** (or download and extract the repository ZIP from GitHub).
3. **[Inno Setup 6](https://jrsoftware.org/isinfo.php)** *(only needed for Option 1's `.exe` setup wizard)*:
   ```powershell
   winget install JRSoftware.InnoSetup
   ```

### Step 1: Clone the repository
```powershell
git clone https://github.com/andyswings/Pro_Clock.git
cd Pro_Clock
```

### Option 1: Build the Windows Setup Wizard (`.exe` installer, recommended)
```powershell
.\packaging\windows\build-installer.ps1
```
*(If PowerShell blocks running scripts, run `powershell -ExecutionPolicy Bypass -File .\packaging\windows\build-installer.ps1` instead.)*

This creates two files in `dist\`:
- `dist\ProClock-Setup-0.2.0-x64.exe` — double-click to run the graphical Windows installer (Start Menu shortcut, optional Desktop shortcut, Add/Remove Programs uninstaller; works without admin rights or system-wide).
- `dist\ProClock-0.2.0-windows-x64-portable.zip` — portable no-install archive (pass `-PortableOnly` if you don't have Inno Setup installed).

### Option 2: Direct user-local install (no Inno Setup required)
```powershell
.\install.ps1
```
This compiles `pro-clock.exe`, installs it to `%LOCALAPPDATA%\ProClock`, and adds **Pro Clock** to your Start Menu (pass `-DesktopShortcut` for a Desktop icon).

To uninstall (preserves your SQLite database):
```powershell
.\uninstall.ps1
```

## Building from Source

```bash
cargo build --release
cargo run --release
```

Run tests:
```bash
cargo test
```

## License

MIT License — see [LICENSE](LICENSE) for details.
