# Pro Clock — Complete Help & Reference Manual

> **Pro Clock v0.2.0** — A free, open-source, 100% offline Clockify alternative written in Rust.  
> Looking for guided walkthroughs? See the [Tutorials Hub](tutorials/README.md).  
> Looking for the project overview and installation instructions? See the [main README](../README.md).

---

## Table of Contents

1. [Overview & Core Principles](#overview--core-principles)
2. [Interface Overview](#interface-overview)
3. [Time Tracker (`⏱ Time Tracker`)](#time-tracker--time-tracker)
4. [Weekly Timesheet (`📅 Timesheet`)](#weekly-timesheet--timesheet)
5. [Pomodoro Timer (`🍅 Pomodoro`)](#pomodoro-timer--pomodoro)
6. [Reports & Export (`📊 Reports & Export`)](#reports--export--reports--export)
7. [Projects, Clients & Tags](#projects-clients--tags)
8. [Settings (`⚙ Settings`)](#settings--settings)
9. [Billing & Hourly Rate Precedence](#billing--hourly-rate-precedence)
10. [Data Storage, SQLite Schema & Backups](#data-storage-sqlite-schema--backups)
11. [Desktop Notifications Reference](#desktop-notifications-reference)
12. [FAQ & Troubleshooting](#faq--troubleshooting)

---

## Overview & Core Principles

Pro Clock is designed for freelancers, developers, consultants, and privacy-conscious individuals who want Clockify-style time tracking without cloud accounts, paywalls, browser tabs, or subscription fees.

- **100% Offline & Local-First**: Every client, project, tag, time entry, and setting lives in a single local SQLite file (`pro_clock.db`).
- **Zero Accounts & Zero Telemetry**: No sign-up wall, no background network calls, and no analytics.
- **Crash-Safe Timers**: Running timers persist their `start_time` in SQLite immediately so closing the app or rebooting never loses elapsed time.
- **Built-In Offline Help**: You can also read a searchable version of this guide inside the app at any time via **❓ Help & Guide** in the sidebar.

---

## Interface Overview

Pro Clock opens with a left navigation sidebar and a main workspace panel (default window size `1120 × 740`, minimum `840 × 560`):

### Sidebar Sections

| Group | Item | Purpose |
|---|---|---|
| **TRACK** | **⏱ Time Tracker** | Live stopwatch timer, manual time entry, search/filtering, and daily grouped entry list |
| **TRACK** | **📅 Timesheet** | Clockify-style Project × 7-day weekly grid with quick-log cells |
| **TRACK** | **🍅 Pomodoro** | Focus / Short Break / Long Break countdown ring with auto-logging |
| **TRACK** | **📊 Reports & Export** | Date/client/project/billable filters, KPI cards, daily chart, CSV & JSON export |
| **MANAGE** | **📁 Projects** | Create, color-code, rate-configure, edit, archive/restore, or delete projects |
| **MANAGE** | **🏢 Clients** | Manage clients and notes, archive/restore, or delete |
| **MANAGE** | **🏷 Tags** | Create, color-code, edit, or delete multi-select tags |
| **SYSTEM** | **⚙ Settings** | Theme, week start, currency, default rate, notifications, Pomodoro intervals, backups |
| **SYSTEM** | **❓ Help & Guide** | Built-in offline searchable manual and quick links to every section |

### Persistent Sidebar Indicators & Controls

- **Live `TRACKING` Badge**: Whenever a Time Tracker timer is running, an orange badge in the sidebar displays the live `HH:MM:SS` elapsed clock and task description regardless of which tab you are viewing.
- **Live Pomodoro Badge**: Whenever a Pomodoro session is running, a color-coded badge displays the active phase (`FOCUS SESSION`, `SHORT BREAK`, or `LONG BREAK`) and `MM:SS` countdown.
- **Theme Switcher**: Click **☀ Switch to Light** / **🌙 Switch to Dark** at the bottom-left to toggle between Ubuntu Yaru Dark and Light themes immediately.
- **Status Toast Banners**: Confirmations and validation errors appear at the top of the workspace for 4.5 seconds (or dismiss manually with `✕`).

---

## Time Tracker (`⏱ Time Tracker`)

### Timer Mode (`⏱ Timer`)

1. Type what you are working on in **What are you working on?**.
2. Optionally select a **Project** (`+ Project`), attach one or more **🏷 Tags**, and toggle the billable currency button (`$` or your configured currency symbol).
   - Selecting a project automatically sets the billable toggle to that project's **Billable by default** preference.
3. Click **▶ START** to start the live stopwatch.
4. Click **■ STOP** when finished to save the end timestamp and trigger an optional desktop notification.

**Key Timer Behaviors:**
- **Live Edits Persist Automatically**: While a timer is running, changing the project, tags, billable toggle, or editing the description (saved when the text box loses focus) updates the active database record immediately.
- **Single Active Timer Rule**: Only one timer runs at a time. Starting a new timer—or clicking **▶** (Continue) on an existing entry—automatically stops any currently running timer at that exact second.
- **Crash-Safe Persistence**: Running timers are stored in SQLite with `end_time = NULL`. If Pro Clock closes unexpectedly, reopening it automatically reloads and resumes counting from the original `start_time`.

### Manual Mode (`✏ Manual`)

Switch to **✏ Manual** at the top-right of the Time Tracker view to log past work:

1. Fill in description, project, tags, and billable toggle.
2. Enter:
   - **Date (`YYYY-MM-DD`)** — defaults to today
   - **Start (`HH:MM`)** — defaults to 1 hour ago (also accepts `HH:MM:SS`)
   - **End (`HH:MM`)** — defaults to now (also accepts `HH:MM:SS`)
3. Confirm the live duration preview (shown in blue, or an error if End is not after Start).
4. Click **+ ADD ENTRY**.

### Daily Entry List, Editing & Filtering

- **Search & Filters**:
  - Search box matches both **entry description** and **project name** (case-insensitive).
  - **Project filter dropdown** restricts the list to a specific project.
  - **Billable only** hides non-billable entries.
  - Click **Clear filters** to reset all three filters.
- **Day Groups**: Entries are grouped by local calendar date (`Today`, `Yesterday`, or `Day, Mon DD, YYYY`) with daily total duration and total billable earnings in the header.
- **Row Actions**:
  - **▶ (Continue)**: Starts a new live timer copying the entry's description, project, billable flag, and tags.
  - **✏ (Edit)**: Opens the **Edit Time Entry** modal where you can modify description, project, **Custom Rate** override, **Billable** flag, Date, Start (`HH:MM:SS`), End (`HH:MM:SS`), and tags.
  - **🗑 (Delete)**: Permanently deletes the entry from SQLite.

---

## Weekly Timesheet (`📅 Timesheet`)

The Weekly Timesheet displays a Clockify-style **Project × 7-Day** matrix for rapid batch logging and weekly hour reviews.

### Navigating & Adding Rows

- Use **◀ Prev**, **This Week**, and **Next ▶** to move across weeks.
- Today's column header is highlighted in orange.
- Whether a week starts on **Monday** or **Sunday** is controlled by **Week starts on Monday** in **⚙ Settings**.
- Any project with entries in the selected week appears automatically. To log against a project with no entries yet this week, select it in **Select project to add row...** and click **+ Add Row**.

### Quick-Logging a Cell

1. Click any `HH:MM` cell in the grid to highlight it and open the **Quick Log** card below the table.
2. Enter a **Duration** and an optional **Note**.
3. Press **Enter** or click **+ Add Time**.

#### Supported Duration Formats

| Input Example | Parsed Duration | Notes |
|---|---|---|
| `1:30` | `01:30:00` (1.5 hours) | Standard `HH:MM` format |
| `0:45` | `00:45:00` (45 minutes) | Minutes-only `HH:MM` |
| `1:30:00` | `01:30:00` | Exact `HH:MM:SS` format |
| `1.5` | `01:30:00` | Decimal hours |
| `2` | `02:00:00` | Whole hours |
| `1.5h` / `2h` | `01:30:00` / `02:00:00` | Explicit hours suffix |
| `45m` | `00:45:00` | Explicit minutes suffix |

> **Note:** Quick-logged entries are saved starting at **`09:00:00` local time** on the selected day and inherit the selected project's **Billable by default** setting (`false` for `No Project`). To adjust exact timestamps, tags, or a custom rate afterward, open **⏱ Time Tracker** and click **✏** on the entry.

---

## Pomodoro Timer (`🍅 Pomodoro`)

Pro Clock includes an integrated Pomodoro timer built for uninterrupted focus blocks:

- **Three Phases**: **Focus Session** (orange), **Short Break** (blue), and **Long Break** (green).
- **Task Binding**: Enter a description, pick a **Project**, and toggle **Billable** so your focus blocks are categorized cleanly.
- **Auto-log to Tracker**: When checked (enabled by default), each naturally completed **Focus Session** automatically writes a time entry into SQLite using your bound description (or `"Pomodoro Focus Session"` if left blank), project, and billable flag.
- **Background Ticking**: Switch to any other tab while a Pomodoro runs—the timer continues ticking and remains visible in the sidebar.
- **Controls**:
  - **▶ START**: Starts or resumes the countdown.
  - **⏸ PAUSE**: Pauses the current countdown without resetting remaining time.
  - **↺ Reset**: Resets the current phase back to its full configured duration.
  - **⏭ Skip**: Advances immediately to the next phase **without** auto-logging or triggering a completion notification.
- **Cycle Tracking**: Filled orange dots show completed focus sessions in the current cycle. After `pomodoro_cycles_before_long` focus sessions (default `4`), Pro Clock switches to **Long Break** instead of **Short Break**.

---

## Reports & Export (`📊 Reports & Export`)

### Filtering & Analytics

Filter your tracked history using any combination of:
- **Date Presets**: `Today`, `This Week`, `Last Week`, `This Month`, `Last Month`, `All Time`, or `Custom` (`From YYYY-MM-DD` to `To YYYY-MM-DD`).
- **Project**: `All Projects` or a specific project (including archived projects).
- **Client**: `All Clients` or a specific client.
- **Billable only**: Restrict KPIs, charts, and exports to billable entries.

The report displays:
1. **4 KPI Summary Cards**: `TOTAL TIME`, `BILLABLE TIME`, `BILLABLE AMOUNT`, and `ENTRIES`.
2. **Daily Hours Bar Chart**: Visual bar per active day in the filtered range with hours labeled above each bar.
3. **Breakdown by Project**: Ranked list of projects by duration with percentage share, total time, billable amount, and color-coded progress bar.

### Exporting to CSV & JSON (100% Offline)

Enter your target folder in **Folder:** (defaults to your system `Documents` directory, or home directory if unavailable) and click:
- **Export CSV** → writes `pro-clock-report-YYYY-MM-DD_HHMMSS.csv`
- **Export JSON** → writes `pro-clock-report-YYYY-MM-DD_HHMMSS.json`
- **📂 Open Folder** → opens the export directory in your OS file manager (`xdg-open` on Linux, `explorer` on Windows, `open` on macOS).

#### Export Schema (Clockify-Compatible CSV & JSON)

| CSV Header | JSON Field | Description / Format |
|---|---|---|
| `Project` | `project` | Project name (empty string if unassigned) |
| `Client` | `client` | Linked client name (empty string if unassigned) |
| `Description` | `description` | Entry description |
| `Tags` | `tags` | Comma-separated tag names |
| `Billable` | `billable` | `"Yes"` or `"No"` |
| `Start Date` | `start_date` | Local start date (`YYYY-MM-DD`) |
| `Start Time` | `start_time` | Local start time (`HH:MM:SS`) |
| `End Date` | `end_date` | Local end date (`YYYY-MM-DD`) or `"Running"` |
| `End Time` | `end_time` | Local end time (`HH:MM:SS`) or `"Running"` |
| `Duration (h)` | `duration_hms` | Duration formatted as `HH:MM:SS` |
| `Duration (decimal)` | `duration_decimal` | Decimal hours rounded to 2 decimal places |
| `Billable Rate` | `hourly_rate` | Effective hourly rate applied (2 decimals) |
| `Billable Amount` | `billable_amount` | Calculated billable total (2 decimals; `0.00` if non-billable) |

---

## Projects, Clients & Tags

### Clients (`🏢 Clients`)
- Create clients with a unique name and optional notes.
- Edit, **Archive** / **Restore** (use **Show archived** to view archived clients), or **Delete** (`🗑`).

### Projects (`📁 Projects`)
- Link each project to an optional **Client**, assign a **Rate**, set **Billable by default**, and choose from 10 preset colors (`Ubuntu Orange`, `Ocean Blue`, `Emerald`, `Amethyst`, `Amber`, `Crimson`, `Teal`, `Indigo`, `Rose`, `Slate`).
- **Archive vs. Delete**:
  - **Archiving** hides a project from active timer/timesheet selectors while keeping it visible in historical entries and reports.
  - **Deleting** removes the project record; existing time entries are **preserved** with `project_id` set to `NULL` (`ON DELETE SET NULL`).

### Tags (`🏷 Tags`)
- Create color-coded tags (such as `Deep Work`, `Meeting`, `Bugfix`, `Support`) and attach multiple tags to any time entry in **Time Tracker** or the **Edit Time Entry** modal.

---

## Settings (`⚙ Settings`)

- **Appearance & Workspace**:
  - **Dark Mode (Ubuntu Yaru Dark)**: Toggles dark/light theme immediately.
  - **Week starts on Monday**: Controls both **Weekly Timesheet** columns and **Reports** (`This Week` / `Last Week`).
  - **Currency Symbol**: Single or multi-character currency prefix (e.g., `$`, `€`, `£`, `CHF `).
  - **Default Hourly Rate**: Global fallback hourly rate. Click **Save Settings** to persist currency and default rate.
- **Pomodoro & Desktop Notifications**:
  - **Enable desktop notifications** + **🔔 Send Test Notification** button.
  - **Automatically log completed Pomodoro focus sessions to Time Tracker**.
  - **Focus (min)** (`1–180`), **Short Break (min)** (`1–60`), **Long Break (min)** (`1–90`), **Cycles before Long Break** (`1–12`). Changes save immediately and reset the Pomodoro display if not currently running.
- **Offline Database & Backup**:
  - Shows the full local path to `pro_clock.db`.
  - **Create Backup Copy**: Copies `pro_clock.db` to `pro-clock-backup-YYYY-MM-DD_HHMMSS.db` inside your export directory.
  - **📂 Open Data Folder**: Opens the folder containing `pro_clock.db`.

---

## Billing & Hourly Rate Precedence

For every **billable** time entry, Pro Clock resolves the effective hourly rate using a strict 3-tier waterfall:

1. **Entry Custom Rate** (set in the **✏ Edit Time Entry** modal) — if present, this always wins.
2. **Project Hourly Rate** — used if no entry custom rate is set and the project's hourly rate is `> 0`.
3. **Default Hourly Rate** — used from **⚙ Settings** if neither of the above applies.

**Calculation:**
```text
Billable Amount = (duration_seconds / 3600.0) × effective_hourly_rate
```
If an entry's **Billable** flag is unchecked, its `Billable Amount` is always `0.00`.

---

## Data Storage, SQLite Schema & Backups

### Database File Locations

| OS | Default Database Path |
|---|---|
| **Linux** | `~/.local/share/pro-clock/pro_clock.db` |
| **Windows** | `%APPDATA%\proclock\pro-clock\data\pro_clock.db` |
| **macOS** | `~/Library/Application Support/org.proclock.pro-clock/pro_clock.db` |

### SQLite Tables

Pro Clock uses an open, straightforward SQLite schema (`PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;`):

- `clients` — `id`, `name`, `notes`, `archived`, `created_at`
- `projects` — `id`, `name`, `client_id`, `color`, `billable_default`, `hourly_rate`, `archived`, `created_at`
- `tags` — `id`, `name`, `color`, `created_at`
- `time_entries` — `id`, `description`, `project_id`, `start_time`, `end_time`, `billable`, `hourly_rate`, `created_at`
- `time_entry_tags` — `time_entry_id`, `tag_id`
- `settings` — `key`, `value`

> **Important WAL Mode Note:** Pro Clock opens SQLite in Write-Ahead Logging (`WAL`) mode, which may create `pro_clock.db-wal` and `pro_clock.db-shm` alongside `pro_clock.db` while the app is open. Always **close Pro Clock** before manually copying, syncing, or replacing `pro_clock.db` outside the app (or use **Create Backup Copy** inside **Settings**).

---

## Desktop Notifications Reference

| Operating System | Mechanism Used | Notes |
|---|---|---|
| **Linux** | `notify-send -a "Pro Clock" -i pro-clock -u normal` | Requires `notify-send` (`sudo apt install libnotify-bin` on Ubuntu/Debian) |
| **Windows** | Hidden PowerShell `System.Windows.Forms.NotifyIcon` balloon tip | Uses built-in Windows PowerShell and .NET WinForms |
| **macOS** | `osascript -e 'display notification ...'` | Uses built-in macOS Notification Center |

If a notification binary is unavailable, Pro Clock fails silently without interrupting your timer.

---

## FAQ & Troubleshooting

- **I don't see notifications on Ubuntu/Debian.**  
  Install `libnotify-bin` (`sudo apt install libnotify-bin`), then click **🔔 Send Test Notification** in **⚙ Settings**.
- **Manual entry says `Invalid date/time. Use YYYY-MM-DD and HH:MM with End after Start.`**  
  Make sure Date uses 4-digit year and 2-digit month/day (e.g. `2026-11-25`), times use 24-hour `HH:MM` or `HH:MM:SS` (e.g. `09:00` and `17:30`), and End is strictly later than Start.
- **How do I restore a `.db` backup?**  
  Close Pro Clock completely, back up your current `pro_clock.db` just in case, copy your `pro-clock-backup-*.db` over `pro_clock.db`, delete any stale `pro_clock.db-wal` / `pro_clock.db-shm` files, and reopen Pro Clock.
- **Will `./uninstall.sh` or `./uninstall.ps1` delete my tracked time?**  
  No. Both uninstallers remove the binary and shortcuts while keeping your SQLite database untouched.
- **Can I build or run on PowerShell if script execution is blocked on Windows?**  
  Yes — run `powershell -ExecutionPolicy Bypass -File .\install.ps1` or `powershell -ExecutionPolicy Bypass -File .\packaging\windows\build-installer.ps1`.
