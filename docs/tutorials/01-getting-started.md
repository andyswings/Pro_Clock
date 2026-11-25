# Tutorial 1: 5-Minute Quick Start

Welcome to **Pro Clock**! By the end of this 5-minute walkthrough, you will have configured your workspace, created your first client, project, and tags, tracked a live work session, and reviewed your time in Reports.

**Prerequisites:** Pro Clock installed on Linux or Windows (see the [Installation instructions](../../README.md#installation) if you haven't installed it yet).

---

## Step 1: Configure Your Workspace in Settings

1. Launch **Pro Clock**.
2. In the left sidebar under **SYSTEM**, click **⚙ Settings**.
3. Under **Appearance & Workspace**:
   - Choose your preferred theme under **Theme** (**Ubuntu Yaru Dark**, **Ubuntu Yaru Light**, **Windows 11 Dark**, or **Windows 11 Light**), or click the **☀ Switch to Light** / **🌙 Switch to Dark** button at the bottom-left of the sidebar anytime.
   - Check or uncheck **Week starts on Monday** to match your calendar week.
   - Set **Currency Symbol** (e.g. `$`, `€`, `£`).
   - Set **Default Hourly Rate** (for example, `75.00` — this acts as your fallback rate if a project doesn't specify one).
4. Click **Save Settings**. You should see a green confirmation banner: `Settings saved.`

---

## Step 2: Add Your First Client

1. In the sidebar under **MANAGE**, click **🏢 Clients**.
2. In the **New Client** card:
   - Enter `Acme Corp` in **Client name**.
   - Optionally add notes such as `Retainer contact: billing@acme.example`.
3. Click **+ Add Client**.

`Acme Corp` now appears in the client list below.

---

## Step 3: Create a Color-Coded Project

1. In the sidebar under **MANAGE**, click **📁 Projects**.
2. In the **New Project** card:
   - Enter `Website Redesign` in **Project name**.
   - Open the client dropdown and choose **Acme Corp**.
   - Enter `100.00` in **Rate ($/h)**.
   - Leave **Billable by default** checked.
   - Click one of the 10 preset color dots (for example, **Ubuntu Orange**).
3. Click **+ Add Project**.

Because `Website Redesign` has an hourly rate of `100.00`, it will override the `75.00` default rate from Settings whenever you track billable time on it.

---

## Step 4: Create a Few Tags

Tags help you categorize *how* you spent time across projects (e.g., `Deep Work`, `Meeting`, `Bugfix`).

1. In the sidebar under **MANAGE**, click **🏷 Tags**.
2. Type `Deep Work`, pick a color dot (e.g. **Ocean Blue**), and click **+ Add Tag**.
3. Repeat to add `Meeting` with another color (e.g. **Amber**).

---

## Step 5: Track Your First Live Work Session

1. In the sidebar under **TRACK**, click **⏱ Time Tracker**.
2. In the top tracker card:
   - Type `Build responsive navigation bar` in **What are you working on?**.
   - Open **+ Project** and pick **Website Redesign (Acme Corp)**. Notice the billable `$` button turns blue automatically because `Website Redesign` has **Billable by default** enabled.
   - Click **🏷 Tags** and check **Deep Work**.
3. Click **▶ START**.
   - The clock next to **■ STOP** begins counting (`00:00:01`, `00:00:02`, ...).
   - A live **TRACKING** badge appears in the left sidebar so you can monitor your timer even while browsing other tabs.
4. After letting the timer run for a moment (or completing your work), click **■ STOP**.
   - Your completed entry appears immediately in the **Today** group below, complete with start–end times, duration, project color dot, tag chip, and `$` billable indicator.

---

## Step 6: Inspect Today's Report

1. In the sidebar under **TRACK**, click **📊 Reports & Export**.
2. Make sure the **Today** preset is selected at the top.
3. Review the four summary cards:
   - **TOTAL TIME** and **BILLABLE TIME**
   - **BILLABLE AMOUNT** calculated from your project's hourly rate
   - **ENTRIES** count
4. Scroll down to see the **Daily Hours** bar chart and **Breakdown by Project**.

---

## Next Steps

You're up and running! Continue with:
- **[Tutorial 2: Mastering Daily Time Tracking](02-time-tracker-mastery.md)** to learn live edits, one-click Continue, Manual entries, and the Edit modal.
- **[Tutorial 3: Fast Weekly Logging with the Timesheet Grid](03-weekly-timesheet.md)** if you prefer Clockify-style weekly grid entry.
