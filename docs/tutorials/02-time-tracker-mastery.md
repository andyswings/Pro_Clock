# Tutorial 2: Mastering Daily Time Tracking

Once you know the basics of starting and stopping a timer, Pro Clock's **⏱ Time Tracker** has several workflow accelerators for realistic day-to-day work—switching tasks mid-stream, recovering from accidental app closes, logging forgotten sessions manually, and fixing entries.

---

## 1. Updating a Running Timer on the Fly

Forgot to tag your task or realized you selected the wrong project *after* clicking **▶ START**? You don't have to stop the timer.

While a timer is running:
- **Change Project**: Open the project dropdown and pick another project. Pro Clock updates the running SQLite record immediately and applies the new project's **Billable by default** setting.
- **Add or Remove Tags**: Open **🏷 Tags** and check/uncheck tags—changes save immediately.
- **Toggle Billable**: Click the `$` (currency) button to mark the running session billable or non-billable.
- **Edit Description**: Type your updated description and click outside the text box (or press `Tab`) so it loses focus—Pro Clock persists the new description right away.

---

## 2. Seamless Task Switching (Single Active Timer Rule)

Suppose you are in the middle of coding and a teammate pulls you into an impromptu call:

1. Type `Quick sync on API schema` in the description box (or switch project/tags if desired) while the previous timer is still running—wait, if you edit the description bar while a timer is running, remember that edits the *currently running* timer!
2. Instead, first click **■ STOP**—or even faster, click **▶** (Continue) on a past meeting entry in your list below!
3. Whenever you start a timer (via **▶ START** or **▶ Continue**), Pro Clock automatically stops any running timer at the exact same timestamp (`Utc::now()`), so there are zero overlapping timers and zero gaps.

> **Pro Tip:** Keep one or two recurring entries (like *Daily Standup* or *Email & Triage*) in your history so you can switch to them in **one click** using **▶**.

---

## 3. Crash-Safe Timer Recovery

Pro Clock writes a running timer's `start_time` to `pro_clock.db` with `end_time = NULL` the moment you click **▶ START**.

- If you close Pro Clock, restart your laptop, or lose power while a timer is running, **your elapsed time is safe**.
- The next time you open Pro Clock, it detects the open row, reloads its description, project, tags, and billable flag into the tracker bar, and continues counting from the original `start_time`.

---

## 4. Logging Forgotten Sessions with `✏ Manual` Mode

Forgot to start the timer before a 45-minute meeting? Log it manually:

1. At the top-right of **⏱ Time Tracker**, click **✏ Manual**.
2. Fill in the top row as usual: description, project, tags, and billable toggle.
3. In the second row, enter:
   - **Date (`YYYY-MM-DD`)**: e.g. `2026-11-25` (defaults to today)
   - **Start (`HH:MM`)**: e.g. `14:00` (24-hour clock; `HH:MM:SS` is also accepted)
   - **End (`HH:MM`)**: e.g. `14:45`
4. Check the live duration preview next to **End** (e.g., `00:45:00` in blue). If End is earlier than or equal to Start, Pro Clock warns `End must be after start`.
5. Click **+ ADD ENTRY**.

Your manual entry is inserted chronologically into the daily grouped list.

---

## 5. Fixing Mistakes & Setting Custom Rates with `✏ Edit`

Need to trim 10 minutes off a session where you forgot to hit stop, change a date, or apply a special hourly rate to one specific entry?

1. Hover over any entry row and click **✏** (Edit entry).
2. In the **Edit Time Entry** modal, you can adjust:
   - **Description**
   - **Project** (note: archived projects are also selectable here for historical edits)
   - **Custom Rate (`$/h`)**: Leave blank to use the project's rate (or Settings default), or enter a number like `150.00` to override the rate **for this entry only**.
   - **Billable** checkbox
   - **Date (`YYYY-MM-DD`)**, **Start (`HH:MM:SS`)**, and **End (`HH:MM:SS`)** *(End is hidden if the entry is currently running)*
   - **Tags** checkboxes
3. Click **Save Changes** (or **Cancel** to discard).

---

## 6. Searching & Filtering Your History

Right below the tracker card, use the filter bar to locate past work quickly:
- **Search (`🔍 Search entries...`)**: Filters entries where either the **description** or the **project name** contains your query.
- **Project dropdown**: Narrow down to a single project (including archived projects).
- **Billable only**: Show only billable entries.
- Click **Clear filters** to restore the full grouped list.

---

[← Tutorial 1: Quick Start](01-getting-started.md) · [Tutorial 3: Weekly Timesheet Grid →](03-weekly-timesheet.md)
