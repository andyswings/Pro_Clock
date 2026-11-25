# Tutorial 3: Fast Weekly Logging with the Timesheet Grid

If you prefer logging hours at the end of the day—or filling out a whole week at once like in Clockify's Timesheet view—the **📅 Timesheet** tab gives you a fast **Project × 7-Day** matrix.

---

## 1. Set Your Week Start Day

1. Open **⚙ Settings** in the left sidebar.
2. Check or uncheck **Week starts on Monday**:
   - **Checked**: Weeks run **Mon – Sun** (default).
   - **Unchecked**: Weeks run **Sun – Sat**.
3. Return to **📅 Timesheet**. Your grid columns update immediately, and **Today**'s column header is highlighted in orange.

---

## 2. Navigate Weeks & Add Project Rows

- Use **◀ Prev**, **This Week**, and **Next ▶** at the top-right to move between weeks.
- Any project that already has entries during the displayed week appears automatically as a row.
- To log time to a project that doesn't have entries yet this week:
  1. Open **Select project to add row...** at the bottom of the grid card.
  2. Pick a project (or **No Project**).
  3. Click **+ Add Row**.

---

## 3. Quick-Log Time Into Any Cell

1. Click any `00:00` (or existing `HH:MM`) cell in the grid. The selected cell highlights in orange and opens the **Quick Log** card below the table.
2. Type a **Duration** and an optional **Note** (for example, `Sprint planning and PR reviews`).
3. Press **Enter** (or click **+ Add Time**).

Pro Clock supports several fast duration formats in the Quick Log box:

| Format Type | Examples | Result |
|---|---|---|
| Hours & minutes (`HH:MM`) | `1:30`, `0:45`, `2:05` | `01:30`, `00:45`, `02:05` |
| Hours, minutes & seconds (`HH:MM:SS`) | `1:30:00`, `0:15:30` | `01:30`, `00:15` |
| Decimal hours | `1.5`, `0.25`, `2` | `01:30`, `00:15`, `02:00` |
| Hours suffix (`h`) | `1.5h`, `2h` | `01:30`, `02:00` |
| Minutes suffix (`m`) | `45m`, `90m` | `00:45`, `01:30` |

> **Tip:** Logging multiple tasks on the same project and day simply **adds** to that cell's total—it won't overwrite earlier entries!

---

## 4. How Quick-Logged Entries Behave Under the Hood

- Each quick-logged entry is stored as a standard entry in SQLite starting at **`09:00:00` local time** on that cell's date, with an end time of `09:00:00 + duration`.
- Its **Billable** flag automatically matches the project's **Billable by default** setting (`false` for `No Project`).
- Want to attach tags, set exact afternoon timestamps, or apply a custom rate? Switch to **⏱ Time Tracker**, find the entry under that day, and click **✏**.

---

## 5. Review Totals at a Glance

- **Rightmost column (`Total`)**: Total hours per project for the selected week.
- **Bottom row (`Daily Total`)**: Total hours per day across all projects, plus the weekly **Grand Total** in the bottom-right corner.
- **Week Billable**: Displayed at the bottom-right of the grid card whenever billable earnings in the week are greater than zero.

---

[← Tutorial 2: Mastering Daily Time Tracking](02-time-tracker-mastery.md) · [Tutorial 4: Deep Work with Pomodoro →](04-pomodoro-deep-work.md)
