use crate::notify::open_folder;
use crate::theme::{card_frame, pill_badge, primary_button, with_alpha, Palette};
use crate::ui::{NavTab, ProClockApp};
use egui::{RichText, ScrollArea, TextEdit, Ui, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HelpTopic {
    QuickStart,
    Tracker,
    Timesheet,
    Pomodoro,
    Reports,
    ProjectsBilling,
    DataBackup,
    Faq,
}

impl HelpTopic {
    pub const ALL: &'static [HelpTopic] = &[
        HelpTopic::QuickStart,
        HelpTopic::Tracker,
        HelpTopic::Timesheet,
        HelpTopic::Pomodoro,
        HelpTopic::Reports,
        HelpTopic::ProjectsBilling,
        HelpTopic::DataBackup,
        HelpTopic::Faq,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            HelpTopic::QuickStart => "🚀 Quick Start",
            HelpTopic::Tracker => "⏱ Time Tracker",
            HelpTopic::Timesheet => "📅 Timesheet",
            HelpTopic::Pomodoro => "🍅 Pomodoro",
            HelpTopic::Reports => "📊 Reports & Export",
            HelpTopic::ProjectsBilling => "💰 Projects & Billing",
            HelpTopic::DataBackup => "💾 Data & Backup",
            HelpTopic::Faq => "❓ FAQ",
        }
    }

    pub fn badge_name(&self) -> &'static str {
        match self {
            HelpTopic::QuickStart => "Quick Start",
            HelpTopic::Tracker => "Time Tracker",
            HelpTopic::Timesheet => "Timesheet",
            HelpTopic::Pomodoro => "Pomodoro",
            HelpTopic::Reports => "Reports & Export",
            HelpTopic::ProjectsBilling => "Projects & Billing",
            HelpTopic::DataBackup => "Data & Backup",
            HelpTopic::Faq => "FAQ",
        }
    }
}

pub struct HelpCard {
    pub topic: HelpTopic,
    pub title: &'static str,
    pub summary: &'static str,
    pub bullets: &'static [&'static str],
    pub tip: Option<&'static str>,
    pub target_tab: Option<NavTab>,
    pub target_label: Option<&'static str>,
}

pub const HELP_CARDS: &[HelpCard] = &[
    // ── Quick Start ───────────────────────────────────────────────────────────
    HelpCard {
        topic: HelpTopic::QuickStart,
        title: "5-Step Getting Started Guide",
        summary: "Go from a fresh install to tracking billable hours and exporting reports in under 5 minutes — completely offline.",
        bullets: &[
            "1. Configure Settings: Set your Currency Symbol, Default Hourly Rate, Week Start (Monday/Sunday), and Ubuntu Yaru or Windows 11 Dark/Light theme.",
            "2. Add Clients & Projects: Create clients in the Clients tab, then create color-coded projects linked to clients with default billable status and rates.",
            "3. Track Your Time: Use Time Tracker (live stopwatch or Manual mode), Weekly Timesheet (quick grid logging), or Pomodoro (focus intervals).",
            "4. Review Reports & Export: Filter by date range, project, client, or billable status in Reports & Export, then export to CSV or JSON.",
            "5. Back Up Your Data: Use Create Backup Copy in Settings to save a timestamped copy of your local SQLite database.",
        ],
        tip: Some("Tip: Use the sidebar theme button at the bottom-left anytime to toggle Dark/Light mode, and switch between Ubuntu Yaru and Windows 11 styles in Settings."),
        target_tab: Some(NavTab::Settings),
        target_label: Some("Open Settings →"),
    },
    HelpCard {
        topic: HelpTopic::QuickStart,
        title: "Choose the Right Tracking Mode for Your Workflow",
        summary: "Pro Clock gives you three complementary ways to record time, all stored in the same local SQLite database.",
        bullets: &[
            "⏱ Time Tracker: Best for real-time stopwatch tracking during the day, switching tasks seamlessly, or adding/editing exact Start–End timestamps.",
            "📅 Weekly Timesheet: Best for rapid end-of-day or end-of-week batch entry across a Clockify-style Project × Day grid.",
            "🍅 Pomodoro Timer: Best for structured deep work with Focus / Break rings, desktop notifications, and automatic time logging.",
        ],
        tip: Some("All three views write to the same entries list — anything logged in Timesheet or Pomodoro appears immediately in Time Tracker and Reports."),
        target_tab: Some(NavTab::Tracker),
        target_label: Some("Open Time Tracker →"),
    },

    // ── Time Tracker ──────────────────────────────────────────────────────────
    HelpCard {
        topic: HelpTopic::Tracker,
        title: "Live Timer Mode (Crash-Safe Stopwatch)",
        summary: "Start and stop a live timer with zero fear of losing work if the app closes or your computer restarts.",
        bullets: &[
            "Enter a task description, choose a Project, attach optional Tags, and toggle the billable currency button before or after clicking ▶ START.",
            "Live Edits Persist Immediately: Changing description (on focus loss), project, tags, or billable status while a timer is running updates the active SQLite row right away.",
            "Single Active Timer Rule: Clicking ▶ START (or ▶ Continue on any past entry) automatically stops any currently running timer at the exact handoff moment.",
            "Crash Safety: A running timer stores its start_time in SQLite with end_time = NULL, so closing and reopening Pro Clock resumes counting seamlessly.",
        ],
        tip: Some("A live TRACKING badge stays visible in the left sidebar while you browse Timesheet, Reports, or Settings."),
        target_tab: Some(NavTab::Tracker),
        target_label: Some("Go to Time Tracker →"),
    },
    HelpCard {
        topic: HelpTopic::Tracker,
        title: "Manual Entry, Continue, Edit Modal & Filtering",
        summary: "Log forgotten sessions, resume recurring tasks in one click, and audit past entries with full precision.",
        bullets: &[
            "✏ Manual Mode: Switch to Manual at the top-right of Time Tracker, enter Date (YYYY-MM-DD), Start (HH:MM or HH:MM:SS), and End after Start, verify the live duration preview, and click + ADD ENTRY.",
            "▶ Continue: Click ▶ on any past entry row to immediately start a new live timer copying its description, project, billable flag, and tags.",
            "✏ Edit Entry Modal: Adjust description, project, Custom Rate override, billable flag, Date, Start/End (HH:MM:SS), and tags.",
            "Search & Filters: Filter entries by description or project name in the search box, restrict to a single Project, or check Billable only.",
        ],
        tip: Some("Deleting an entry (🗑) permanently removes that time entry from SQLite, so double-check before clicking."),
        target_tab: Some(NavTab::Tracker),
        target_label: Some("Open Time Tracker →"),
    },

    // ── Weekly Timesheet ──────────────────────────────────────────────────────
    HelpCard {
        topic: HelpTopic::Timesheet,
        title: "Weekly Timesheet Grid & Quick-Log Formats",
        summary: "Log hours across an entire week in seconds using the Clockify-style Project × Day matrix.",
        bullets: &[
            "Navigate weeks using ◀ Prev, This Week, and Next ▶. Today's column header is highlighted in the theme accent color.",
            "Set whether weeks start on Monday or Sunday in Settings (Week starts on Monday).",
            "Add an empty project row using the dropdown (Select project to add row...) and clicking + Add Row.",
            "Click any cell in the grid to open the Quick Log card, type a duration and optional note, and press Enter or click + Add Time.",
            "Supported Duration Formats: HH:MM (e.g. 1:30, 0:45), HH:MM:SS (1:30:00), decimal hours (1.5, 2), hours suffix (1.5h, 2h), or minutes suffix (45m).",
        ],
        tip: Some("Quick-logged entries default to a 09:00 local start time and inherit the selected project's 'Billable by default' setting. Use ✏ in Time Tracker if you want to adjust exact timestamps or tags later."),
        target_tab: Some(NavTab::Timesheet),
        target_label: Some("Open Timesheet →"),
    },

    // ── Pomodoro ──────────────────────────────────────────────────────────────
    HelpCard {
        topic: HelpTopic::Pomodoro,
        title: "Pomodoro Focus & Break Timer",
        summary: "Stay focused with customizable work intervals, visual progress ring, cycle tracking, and automatic logging.",
        bullets: &[
            "Bind your session to a task description, Project, and Billable toggle before clicking ▶ START.",
            "Background Ticking: The Pomodoro timer keeps ticking across all tabs, displaying a live countdown card in the sidebar.",
            "Controls: ⏸ PAUSE freezes the countdown without resetting; ↺ Reset restarts the current phase; ⏭ Skip advances to the next phase without triggering a completion notification or auto-logging.",
            "Auto-log to Tracker: When enabled, each naturally completed Focus Session is saved to SQLite (using 'Pomodoro Focus Session' if the description is blank) and automatically advances to Short Break or Long Break.",
            "Customize Focus (1–180m), Short Break (1–60m), Long Break (1–90m), Cycles before Long Break (1–12), and desktop notifications in Settings.",
        ],
        tip: Some("Desktop notifications use notify-send on Linux, PowerShell balloon tips on Windows, and osascript on macOS. Test yours in Settings → Send Test Notification."),
        target_tab: Some(NavTab::Pomodoro),
        target_label: Some("Open Pomodoro →"),
    },

    // ── Reports & Export ──────────────────────────────────────────────────────
    HelpCard {
        topic: HelpTopic::Reports,
        title: "Reports, Charts & Offline CSV / JSON Export",
        summary: "Analyze your hours and earnings with flexible filters and export Clockify-compatible reports with zero internet connection.",
        bullets: &[
            "Date Presets: Filter instantly by Today, This Week, Last Week, This Month, Last Month, All Time, or Custom (YYYY-MM-DD range).",
            "Combine Project, Client, and Billable only filters to isolate exactly the sessions needed for a client invoice or status update.",
            "Inspect 4 KPI summary cards (Total Time, Billable Time, Billable Amount, Entries), the Daily Hours bar chart, and Breakdown by Project bars.",
            "CSV Export Columns (13 Clockify-compatible fields): Project, Client, Description, Tags, Billable, Start Date, Start Time, End Date, End Time, Duration (h), Duration (decimal), Billable Rate, Billable Amount.",
            "JSON Export writes pretty-printed structured records to pro-clock-report-YYYY-MM-DD_HHMMSS.json. Click 📂 Open Folder to open your export directory.",
        ],
        tip: Some("CSV and JSON exports respect your currently active filters — filter to 'Last Month' + one Client + 'Billable only' before exporting for fast invoicing."),
        target_tab: Some(NavTab::Reports),
        target_label: Some("Open Reports & Export →"),
    },

    // ── Projects & Billing ────────────────────────────────────────────────────
    HelpCard {
        topic: HelpTopic::ProjectsBilling,
        title: "Clients, Projects, Tags & Archiving",
        summary: "Organize work cleanly across clients and categories while keeping historical reports intact.",
        bullets: &[
            "Clients: Store client names and optional notes; link one or more Projects to each Client.",
            "Projects: Choose from 10 preset colors (Ubuntu Orange, Ocean Blue, Emerald, etc.), set a project Hourly Rate, and toggle Billable by default.",
            "Tags: Create color-coded tags (e.g., Deep Work, Meeting, Bugfix) and attach multiple tags to any time entry.",
            "Archive vs Delete: Archiving a project or client hides it from active dropdown pickers while preserving all history. Even if you delete a project or client, SQLite ON DELETE SET NULL preserves your time entries.",
        ],
        tip: Some("Toggle 'Show archived' at the top-right of Projects or Clients whenever you want to restore an archived item."),
        target_tab: Some(NavTab::Projects),
        target_label: Some("Open Projects →"),
    },
    HelpCard {
        topic: HelpTopic::ProjectsBilling,
        title: "3-Tier Hourly Rate Precedence",
        summary: "Pro Clock resolves the effective hourly rate for every billable entry using a clear, predictable 3-tier fallback.",
        bullets: &[
            "Tier 1 — Entry Custom Rate: If you set a Custom Rate in the Edit Time Entry modal, that rate wins.",
            "Tier 2 — Project Hourly Rate: Otherwise, if the entry's Project has an hourly rate > 0, that project rate is used.",
            "Tier 3 — Default Hourly Rate: Otherwise, Pro Clock falls back to the Default Hourly Rate configured in Settings.",
            "Formula: Billable Amount = (Duration in Seconds / 3600) × Effective Rate. Non-billable entries always calculate 0.00.",
        ],
        tip: Some("To override a project's rate on a single entry (e.g., urgent weekend support), edit the entry (✏) and fill in Custom Rate."),
        target_tab: Some(NavTab::Settings),
        target_label: Some("Check Default Rate in Settings →"),
    },

    // ── Data & Backup ─────────────────────────────────────────────────────────
    HelpCard {
        topic: HelpTopic::DataBackup,
        title: "100% Offline SQLite Storage & Backups",
        summary: "You own 100% of your data in a single local SQLite file — no accounts, no cloud sync fees, and no telemetry.",
        bullets: &[
            "Linux path: ~/.local/share/pro-clock/pro_clock.db",
            "Windows path: %APPDATA%\\proclock\\pro-clock\\data\\pro_clock.db",
            "macOS path: ~/Library/Application Support/org.proclock.pro-clock/pro_clock.db",
            "One-Click Backup: In Settings → Offline Database & Backup, click Create Backup Copy to save a timestamped pro-clock-backup-YYYY-MM-DD_HHMMSS.db file in your export folder.",
            "WAL Mode Note: Pro Clock uses SQLite WAL mode. Before copying or replacing pro_clock.db manually outside the app, close Pro Clock first so pending WAL frames are checkpointed.",
        ],
        tip: Some("See docs/HELP.md and docs/tutorials/06-backups-sql-and-clockify-migration.md in the repository for SQL query examples and restore steps."),
        target_tab: Some(NavTab::Settings),
        target_label: Some("Open Backup Settings →"),
    },

    // ── FAQ ───────────────────────────────────────────────────────────────────
    HelpCard {
        topic: HelpTopic::Faq,
        title: "Frequently Asked Questions",
        summary: "Quick answers to common questions about offline reliability, Clockify migration, notifications, and uninstalling.",
        bullets: &[
            "What happens if Pro Clock crashes while a timer is running? Running timers are stored in SQLite with end_time = NULL. Reopen Pro Clock and the timer picks up seamlessly from its original start_time.",
            "Can I move from Clockify or share reports with clients using Clockify? Yes. Reports → Export CSV writes a 13-column spreadsheet matching Clockify's detailed report structure.",
            "Why aren't desktop notifications showing on Linux? Pro Clock calls notify-send. Install it on Ubuntu/Debian with: sudo apt install libnotify-bin, then test in Settings.",
            "Does uninstalling delete my tracked hours? No — uninstall.sh, uninstall.ps1, and apt remove keep your pro_clock.db database intact.",
            "Where can I find full step-by-step tutorials? Check docs/HELP.md and docs/tutorials/ in the Pro Clock project folder or GitHub repository.",
        ],
        tip: None,
        target_tab: None,
        target_label: None,
    },
];

pub fn matching_cards(topic: HelpTopic, search: &str) -> Vec<&'static HelpCard> {
    let q = search.trim().to_lowercase();
    if q.is_empty() {
        HELP_CARDS.iter().filter(|c| c.topic == topic).collect()
    } else {
        HELP_CARDS
            .iter()
            .filter(|c| {
                c.title.to_lowercase().contains(&q)
                    || c.summary.to_lowercase().contains(&q)
                    || c.topic.badge_name().to_lowercase().contains(&q)
                    || c.bullets.iter().any(|b| b.to_lowercase().contains(&q))
                    || c.tip.map(|t| t.to_lowercase().contains(&q)).unwrap_or(false)
            })
            .collect()
    }
}

pub fn render_help_view(app: &mut ProClockApp, ui: &mut Ui, p: &Palette) {
    // ── Header + Search Bar ───────────────────────────────────────────────────
    ui.horizontal(|ui| {
        ui.heading(RichText::new("Help & User Guide").color(p.text_primary).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if !app.help_search.trim().is_empty() && ui.button("Clear").clicked() {
                app.help_search.clear();
            }
            ui.add(
                TextEdit::singleline(&mut app.help_search)
                    .hint_text("🔍 Search help topics...")
                    .desired_width(240.0),
            );
        });
    });

    ui.add_space(10.0);

    let search_snapshot = app.help_search.clone();
    let is_searching = !search_snapshot.trim().is_empty();

    // ── Topic Selector Pills (shown when not searching) ───────────────────────
    if !is_searching {
        card_frame(p).show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                for &topic in HelpTopic::ALL {
                    if ui
                        .selectable_label(app.help_topic == topic, topic.label())
                        .clicked()
                    {
                        app.help_topic = topic;
                    }
                }
            });
        });
        ui.add_space(12.0);
    } else {
        let count = matching_cards(app.help_topic, &search_snapshot).len();
        ui.label(
            RichText::new(format!(
                "Showing {} matching guide section(s) for \"{}\"",
                count,
                search_snapshot.trim()
            ))
            .size(12.5)
            .color(p.text_muted),
        );
        ui.add_space(8.0);
    }

    // ── Cards Area ────────────────────────────────────────────────────────────
    let cards = matching_cards(app.help_topic, &search_snapshot);
    let mut goto_tab: Option<NavTab> = None;
    let mut open_db_folder = false;
    let db_path_str = app.db.path().display().to_string();

    if cards.is_empty() {
        card_frame(p).show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(16.0);
                ui.label(
                    RichText::new("No help topics matched your search.")
                        .size(16.0)
                        .strong()
                        .color(p.text_muted),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new("Try searching for \"timer\", \"CSV\", \"timesheet\", \"Pomodoro\", \"rate\", or \"SQLite\".")
                        .size(13.0)
                        .color(p.text_muted),
                );
                ui.add_space(16.0);
            });
        });
        return;
    }

    ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
        for card in cards {
            card_frame(p).show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new(card.title)
                            .size(16.0)
                            .strong()
                            .color(p.text_primary),
                    );
                    if is_searching {
                        ui.add_space(6.0);
                        pill_badge(ui, card.topic.badge_name(), p.accent_blue);
                    }
                });

                ui.add_space(4.0);
                ui.label(RichText::new(card.summary).size(13.5).color(p.text_muted));
                ui.add_space(10.0);

                for bullet in card.bullets {
                    ui.horizontal_top(|ui| {
                        ui.label(RichText::new("•").strong().color(p.accent));
                        ui.label(RichText::new(*bullet).size(13.5).color(p.text_primary));
                    });
                    ui.add_space(3.0);
                }

                if card.topic == HelpTopic::DataBackup {
                    ui.add_space(8.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("Active database:").strong().size(12.5).color(p.text_primary));
                        ui.label(
                            RichText::new(&db_path_str)
                                .monospace()
                                .size(12.5)
                                .color(p.accent_blue),
                        );
                    });
                }

                if let Some(tip) = card.tip {
                    ui.add_space(8.0);
                    egui::Frame::none()
                        .fill(with_alpha(p.accent_blue, 22))
                        .stroke(egui::Stroke::new(1.0_f32, with_alpha(p.accent_blue, 90)))
                        .rounding(egui::Rounding::same(6.0))
                        .inner_margin(egui::Margin::symmetric(10.0, 6.0))
                        .show(ui, |ui| {
                            ui.label(RichText::new(tip).size(12.5).color(p.text_primary));
                        });
                }

                if card.target_tab.is_some() || card.topic == HelpTopic::DataBackup {
                    ui.add_space(10.0);
                    ui.horizontal_wrapped(|ui| {
                        if let (Some(tab), Some(label)) = (card.target_tab, card.target_label) {
                            if primary_button(ui, label, p.accent).clicked() {
                                goto_tab = Some(tab);
                            }
                        }
                        if card.topic == HelpTopic::DataBackup {
                            if ui
                                .add(
                                    egui::Button::new("📂 Open Data Folder")
                                        .min_size(Vec2::new(140.0, 34.0)),
                                )
                                .clicked()
                            {
                                open_db_folder = true;
                            }
                        }
                    });
                }
            });
            ui.add_space(12.0);
        }
    });

    if let Some(tab) = goto_tab {
        app.active_tab = tab;
    }
    if open_db_folder {
        if let Some(parent) = app.db.path().parent() {
            open_folder(parent);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_help_topic_has_cards() {
        for &topic in HelpTopic::ALL {
            let cards = matching_cards(topic, "");
            assert!(
                !cards.is_empty(),
                "Expected at least one help card for topic {:?}",
                topic
            );
        }
    }

    #[test]
    fn help_search_finds_expected_topics() {
        let csv_matches = matching_cards(HelpTopic::QuickStart, "CSV");
        assert!(csv_matches.iter().any(|c| c.topic == HelpTopic::Reports));

        let pomo_matches = matching_cards(HelpTopic::QuickStart, "Pomodoro");
        assert!(pomo_matches.iter().any(|c| c.topic == HelpTopic::Pomodoro));

        let sqlite_matches = matching_cards(HelpTopic::QuickStart, "SQLite");
        assert!(sqlite_matches.iter().any(|c| c.topic == HelpTopic::DataBackup));

        let none_matches = matching_cards(HelpTopic::QuickStart, "xyznonexistentterm123");
        assert!(none_matches.is_empty());
    }
}
