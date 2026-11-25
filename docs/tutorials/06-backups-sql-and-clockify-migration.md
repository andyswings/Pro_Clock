# Tutorial 6: Offline Data Ownership, Backups & Switching from Clockify

Unlike cloud time trackers where your work history sits on a third-party server behind paid tiers, **Pro Clock stores everything locally in a single SQLite database** (`pro_clock.db`) that you completely control.

---

## 1. Where Your Database Lives

| Operating System | Default `pro_clock.db` Location |
|---|---|
| **Linux** | `~/.local/share/pro-clock/pro_clock.db` |
| **Windows** | `%APPDATA%\proclock\pro-clock\data\pro_clock.db` |
| **macOS** | `~/Library/Application Support/org.proclock.pro-clock/pro_clock.db` |

You can also view the exact path—and open its folder in one click—under **⚙ Settings → Offline Database & Backup**.

---

## 2. Create One-Click Backups & Safely Restore

### Creating a Backup
1. Open **⚙ Settings** and scroll to **Offline Database & Backup**.
2. Click **Create Backup Copy**.
3. Pro Clock writes a timestamped snapshot (`pro-clock-backup-YYYY-MM-DD_HHMMSS.db`) into your configured export folder (shown under **Reports & Export → Folder**).

### Restoring from a Backup
1. **Close Pro Clock completely** (important because SQLite runs in WAL mode).
2. Open your data folder (where `pro_clock.db` lives).
3. Move or rename the current `pro_clock.db`, plus any `pro_clock.db-wal` and `pro_clock.db-shm` files, to a safe temp location.
4. Copy your `pro-clock-backup-*.db` file into the data folder and rename it to `pro_clock.db`.
5. Relaunch Pro Clock.

---

## 3. Moving or Syncing Between Linux and Windows

Because `pro_clock.db` uses the exact same SQLite schema on Linux, Windows, and macOS:
1. Close Pro Clock on machine A so all WAL changes are flushed into `pro_clock.db`.
2. Copy `pro_clock.db` (e.g. via USB, `scp`, or a synced folder) to the corresponding path on machine B (with Pro Clock closed on machine B).
3. Launch Pro Clock on machine B—all clients, projects, tags, settings, and time entries appear immediately.

> **Warning:** Avoid opening Pro Clock simultaneously on two machines pointed at the same cloud-synced `pro_clock.db`, as concurrent SQLite writes across cloud sync can create conflicted copies.

---

## 4. Power-User Bonus: Query `pro_clock.db` with `sqlite3`

Because your data is standard SQLite, you can inspect it read-only anytime (for custom automation, tax prep, or dashboards):

### Total Hours & Earnings by Client (Read-Only)
```bash
sqlite3 -header -column "file:$HOME/.local/share/pro-clock/pro_clock.db?mode=ro" "
SELECT
    COALESCE(c.name, 'No Client') AS client,
    ROUND(SUM((strftime('%s', COALESCE(t.end_time, 'now')) - strftime('%s', t.start_time)) / 3600.0), 2) AS hours
FROM time_entries t
LEFT JOIN projects p ON t.project_id = p.id
LEFT JOIN clients c ON p.client_id = c.id
GROUP BY client
ORDER BY hours DESC;
"
```

### List Recent Time Entries with Tags
```bash
sqlite3 -header -column "file:$HOME/.local/share/pro-clock/pro_clock.db?mode=ro" "
SELECT
    t.id,
    substr(t.start_time, 1, 10) AS date,
    COALESCE(p.name, '(No Project)') AS project,
    t.description,
    COALESCE(GROUP_CONCAT(tg.name, ', '), '') AS tags
FROM time_entries t
LEFT JOIN projects p ON t.project_id = p.id
LEFT JOIN time_entry_tags tet ON tet.time_entry_id = t.id
LEFT JOIN tags tg ON tg.id = tet.tag_id
GROUP BY t.id
ORDER BY t.start_time DESC
LIMIT 20;
"
```

---

## 5. Switching from Clockify to Pro Clock

If you're coming from Clockify, every daily workflow maps directly—without the account login, browser tabs, or feature paywalls:

| Clockify Concept | Pro Clock Equivalent | Notes |
|---|---|---|
| **Time Tracker** (Timer & Manual) | **⏱ Time Tracker** (`⏱ Timer` / `✏ Manual`) | Crash-safe in SQLite; live edits while running |
| **Timesheet** (Weekly Grid) | **📅 Timesheet** | Project × Day matrix with `1:30`, `1.5`, `45m` quick-log |
| **Pomodoro Timer** (Extension/App) | **🍅 Pomodoro** | Built right in with ring UI, cycles, notifications & auto-log |
| **Projects / Clients / Tags** | **📁 Projects** / **🏢 Clients** / **🏷 Tags** | Color-coded, archivable, zero project limits |
| **Billable Rates & Custom Overrides** | Project Rates + Entry **Custom Rate** | All included free (no paid tier required) |
| **Summary / Detailed Reports** | **📊 Reports & Export** | Presets, client/project/billable filters, charts |
| **Export CSV** | **Export CSV** (`13` Clockify-style columns) | Includes `Project`, `Client`, `Description`, `Tags`, `Billable`, `Start/End Date & Time`, `Duration (h)`, `Duration (decimal)`, `Billable Rate`, `Billable Amount` |

---

[← Tutorial 5: Freelance Billing & Invoicing](05-freelance-billing-invoicing.md) · [Back to Tutorials Hub →](README.md)
