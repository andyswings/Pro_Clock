# Tutorial 4: Deep Work & Focus Sessions with the Pomodoro Timer

Pro Clock's **🍅 Pomodoro** view pairs a distraction-free countdown ring with automatic time-entry logging and native desktop notifications—so your focus blocks count toward your tracked project hours automatically.

---

## 1. Configure Your Intervals & Notifications

1. Open **⚙ Settings** and scroll to **Pomodoro & Desktop Notifications**.
2. Adjust your preferred durations (changes save automatically):
   - **Focus (min)**: `1` to `180` (default `25`)
   - **Short Break (min)**: `1` to `60` (default `5`)
   - **Long Break (min)**: `1` to `90` (default `15`)
   - **Cycles before Long Break**: `1` to `12` (default `4`)

   **Popular setups:**
   - *Classic Pomodoro*: `25m` Focus / `5m` Short Break / `15m` Long Break / `4` cycles
   - *Deep Work Block*: `50m` Focus / `10m` Short Break / `20m` Long Break / `3` cycles

3. Ensure **Enable desktop notifications** and **Automatically log completed Pomodoro focus sessions to Time Tracker** are checked.
4. Click **🔔 Send Test Notification** to verify OS notifications work:
   - **Linux**: Requires `notify-send` (`sudo apt install libnotify-bin` on Ubuntu/Debian if missing).
   - **Windows**: Displays a native Windows tray balloon notification via PowerShell.
   - **macOS**: Displays a macOS Notification Center alert via `osascript`.

---

## 2. Bind a Task & Project

1. Open **🍅 Pomodoro** in the sidebar.
2. In the top card:
   - Enter a task description (e.g. `Refactor SQLite query layer`). *(If left blank, auto-logged sessions default to `"Pomodoro Focus Session"`.)*
   - Select a **Project** (this automatically sets **Billable** to the project's default, which you can override).
   - Confirm **Auto-log to Tracker** is checked.

---

## 3. Run a Full Focus & Break Cycle

1. Verify the **Focus Session** tab is selected above the circular ring, then click **▶ START**.
2. **Work freely across tabs**: Switch to **Time Tracker**, **Timesheet**, or **Reports**—the Pomodoro timer keeps ticking in the background and shows a live countdown badge in the left sidebar.
3. When the countdown reaches `00:00`:
   - Pro Clock sends a desktop notification (`Focus session #1 complete! Time for a short break...`).
   - Your completed focus session is **automatically logged** to **⏱ Time Tracker** with your description, project, billable flag, and actual start/end timestamps.
   - The cycle dot increments and the timer switches to **Short Break** (or **Long Break** after completing your configured number of cycles).
4. Click **▶ START** on the break when you're ready to rest, and get notified again when it's time for the next focus block.

---

## 4. Pause vs. Reset vs. Skip

- **⏸ PAUSE**: Freezes the countdown where it is so you can resume later.
- **↺ Reset**: Stops and restores the current phase's full duration without logging anything.
- **⏭ Skip**: Advances immediately to the next phase **without** sending a notification or auto-logging the skipped phase.

---

[← Tutorial 3: Weekly Timesheet Grid](03-weekly-timesheet.md) · [Tutorial 5: Freelance Billing & Invoicing →](05-freelance-billing-invoicing.md)
