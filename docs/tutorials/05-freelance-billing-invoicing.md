# Tutorial 5: Freelance Billing, Reports & Client Invoicing

This tutorial walks through setting up clean client billing rules, understanding Pro Clock's 3-tier hourly rate precedence, and generating month-end CSV/JSON reports ready for client invoicing.

---

## 1. Design a Clean Client → Project → Tag Structure

A well-organized setup keeps invoicing effortless:
- **Clients** (`🏢 Clients`): One record per paying client (plus an `Internal` client if you track non-billable admin).
- **Projects** (`📁 Projects`): Distinct deliverables or retainers linked to each client, each with its own color, hourly rate, and **Billable by default** flag.
- **Tags** (`🏷 Tags`): Cross-cutting work types like `Development`, `Consulting`, `Design`, `Meeting`, or `Urgent`.

---

## 2. Understand the 3-Tier Hourly Rate Hierarchy

For every **billable** entry, Pro Clock determines the hourly rate in this order:

1. **Entry Custom Rate** (set via **✏ Edit** on an individual entry) — *highest priority*
2. **Project Hourly Rate** (if `> 0`)
3. **Default Hourly Rate** (from **⚙ Settings**) — *fallback*

Non-billable entries always have a billable amount of `0.00`.

### Worked Example

Suppose your **Settings** Default Hourly Rate is **`$75.00`**, and you have a project **Cloud Migration** (`Billable by default = true`, `Rate = $110.00`):

| Entry | Billable? | Custom Rate | Effective Rate | Duration | Billable Amount |
|---|---|---|---|---|---|
| Architecture review | Yes | *(empty)* | `$110.00` (Project) | `2:00:00` | **`$220.00`** |
| Emergency Sunday hotfix | Yes | `165.00` | `$165.00` (Entry override) | `1:30:00` | **`$247.50`** |
| Internal tool setup | No | *(empty)* | `$110.00` | `1:00:00` | **`$0.00`** |
| Misc consulting (project rate `0.00`) | Yes | *(empty)* | `$75.00` (Settings default) | `1:00:00` | **`$75.00`** |

---

## 3. Audit Month-End Hours in `📊 Reports & Export`

1. Open **📊 Reports & Export**.
2. Select a date range:
   - Click **Last Month** for standard monthly billing, or
   - Click **Custom** and enter `From (YYYY-MM-DD)` and `To (YYYY-MM-DD)` for custom billing periods (e.g. bi-weekly or 15th-to-15th).
3. Filter to a specific **Client** (or **Project**) using the dropdowns, and check **Billable only** if your invoice excludes non-billable entries.
4. Verify the four KPI cards (**TOTAL TIME**, **BILLABLE TIME**, **BILLABLE AMOUNT**, **ENTRIES**), the **Daily Hours** bar chart, and **Breakdown by Project**.

---

## 4. Export to CSV or JSON for Invoicing

1. At the bottom of **📊 Reports & Export**, confirm the **Folder:** path (defaults to your `Documents` folder).
2. Click **Export CSV** (writes `pro-clock-report-YYYY-MM-DD_HHMMSS.csv`) or **Export JSON** (writes `pro-clock-report-YYYY-MM-DD_HHMMSS.json`).
3. Click **📂 Open Folder** to jump straight to the exported file.

### Turning the CSV Into a Client Invoice

Open the CSV in **LibreOffice Calc**, **Microsoft Excel**, or **Google Sheets**. Pro Clock exports 13 Clockify-compatible columns:

- Use **`Start Date`**, **`Project`**, **`Description`**, and **`Tags`** as your line-item details.
- Use **`Duration (h)`** (`HH:MM:SS`) or **`Duration (decimal)`** (`1.50`) for hours worked.
- Use **`Billable Rate`** and **`Billable Amount`** for line totals—then simply `SUM` the `Billable Amount` column at the bottom of your invoice!

---

## 5. Archiving Finished Projects & Clients

When a client engagement wraps up:
1. Go to **📁 Projects** (or **🏢 Clients**) and click **Archive**.
2. Archived items disappear from the active **Time Tracker**, **Timesheet**, and **Pomodoro** dropdowns so your workspace stays clutter-free—while remaining intact in **Reports & Export** whenever you look up historical data.
3. Need to reactivate an engagement later? Check **Show archived** at the top-right and click **Restore**.

---

[← Tutorial 4: Pomodoro Deep Work](04-pomodoro-deep-work.md) · [Tutorial 6: Backups, SQL & Clockify Migration →](06-backups-sql-and-clockify-migration.md)
