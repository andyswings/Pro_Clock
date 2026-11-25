pub mod help;
pub mod manage;
pub mod pomodoro;
pub mod reports;
pub mod timesheet;
pub mod tracker;

use crate::db::Database;
use crate::models::{format_duration, AppSettings, Client, Project, Tag, TimeEntry};
use crate::theme::{apply_theme, color_dot, Palette};
use crate::ui::pomodoro::{PomodoroPhase, PomodoroState};
use chrono::{Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use egui::{Color32, Context, Frame, Margin, RichText, Rounding, Stroke, Vec2};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavTab {
    Tracker,
    Timesheet,
    Pomodoro,
    Reports,
    Projects,
    Clients,
    Tags,
    Settings,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackerMode {
    Timer,
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateRangePreset {
    Today,
    ThisWeek,
    LastWeek,
    ThisMonth,
    LastMonth,
    AllTime,
    Custom,
}

#[derive(Debug, Clone)]
pub struct EditEntryModal {
    pub id: i64,
    pub description: String,
    pub project_id: Option<i64>,
    pub date_str: String,
    pub start_str: String,
    pub end_str: String,
    pub is_running: bool,
    pub billable: bool,
    pub hourly_rate_str: String,
    pub tag_ids: Vec<i64>,
}

pub struct ProClockApp {
    pub db: Database,
    pub settings: AppSettings,
    pub active_tab: NavTab,

    // Cached DB records
    pub clients: Vec<Client>,
    pub projects: Vec<Project>,
    pub tags: Vec<Tag>,
    pub entries: Vec<TimeEntry>,
    pub clients_map: HashMap<i64, Client>,
    pub projects_map: HashMap<i64, Project>,
    pub tags_map: HashMap<i64, Tag>,

    // Status toast: (message, is_error, timestamp)
    pub status_msg: Option<(String, bool, Instant)>,

    // Tracker bar state
    pub tracker_mode: TrackerMode,
    pub input_description: String,
    pub input_project_id: Option<i64>,
    pub input_billable: bool,
    pub input_tag_ids: Vec<i64>,
    pub manual_date: String,
    pub manual_start: String,
    pub manual_end: String,

    // Tracker list filters
    pub search_query: String,
    pub filter_project_id: Option<i64>,
    pub filter_billable_only: bool,

    // Edit entry modal
    pub edit_modal: Option<EditEntryModal>,

    // Reports state
    pub report_preset: DateRangePreset,
    pub report_start_date: String,
    pub report_end_date: String,
    pub report_project_id: Option<i64>,
    pub report_client_id: Option<i64>,
    pub report_billable_only: bool,
    pub export_dir: String,

    // Manage: Projects form
    pub proj_edit_id: Option<i64>,
    pub proj_name: String,
    pub proj_client_id: Option<i64>,
    pub proj_color: String,
    pub proj_billable_default: bool,
    pub proj_hourly_rate: String,
    pub proj_show_archived: bool,

    // Manage: Clients form
    pub client_edit_id: Option<i64>,
    pub client_name: String,
    pub client_notes: String,
    pub client_show_archived: bool,

    // Manage: Tags form
    pub tag_edit_id: Option<i64>,
    pub tag_name: String,
    pub tag_color: String,

    // Settings form buffer
    pub settings_rate_str: String,

    // Weekly Timesheet state
    pub timesheet_week_offset: i64,
    pub timesheet_extra_rows: Vec<Option<i64>>,
    pub timesheet_add_project_id: Option<i64>,
    pub timesheet_selected_cell: Option<(Option<i64>, NaiveDate)>,
    pub timesheet_quick_duration: String,
    pub timesheet_quick_desc: String,

    // Pomodoro state
    pub pomodoro: PomodoroState,

    // Help & Guide state
    pub help_topic: help::HelpTopic,
    pub help_search: String,
}

impl ProClockApp {
    pub fn new(cc: &eframe::CreationContext<'_>, db: Database) -> Self {
        let settings = db.load_settings().unwrap_or_default();
        apply_theme(&cc.egui_ctx, settings.dark_mode);

        let now = Local::now();
        let one_hour_ago = now - Duration::hours(1);
        let default_export_dir = directories::UserDirs::new()
            .and_then(|u| u.document_dir().map(|p| p.to_path_buf()))
            .or_else(|| directories::UserDirs::new().map(|u| u.home_dir().to_path_buf()))
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .to_string_lossy()
            .to_string();

        let pomodoro = PomodoroState::new(&settings);
        let mut app = Self {
            db,
            settings_rate_str: format!("{:.2}", settings.default_hourly_rate),
            settings,
            active_tab: NavTab::Tracker,
            clients: Vec::new(),
            projects: Vec::new(),
            tags: Vec::new(),
            entries: Vec::new(),
            clients_map: HashMap::new(),
            projects_map: HashMap::new(),
            tags_map: HashMap::new(),
            status_msg: None,
            tracker_mode: TrackerMode::Timer,
            input_description: String::new(),
            input_project_id: None,
            input_billable: false,
            input_tag_ids: Vec::new(),
            manual_date: now.format("%Y-%m-%d").to_string(),
            manual_start: one_hour_ago.format("%H:%M").to_string(),
            manual_end: now.format("%H:%M").to_string(),
            search_query: String::new(),
            filter_project_id: None,
            filter_billable_only: false,
            edit_modal: None,
            report_preset: DateRangePreset::ThisWeek,
            report_start_date: now.format("%Y-%m-%d").to_string(),
            report_end_date: now.format("%Y-%m-%d").to_string(),
            report_project_id: None,
            report_client_id: None,
            report_billable_only: false,
            export_dir: default_export_dir,
            proj_edit_id: None,
            proj_name: String::new(),
            proj_client_id: None,
            proj_color: "#E95420".to_string(),
            proj_billable_default: true,
            proj_hourly_rate: "0.00".to_string(),
            proj_show_archived: false,
            client_edit_id: None,
            client_name: String::new(),
            client_notes: String::new(),
            client_show_archived: false,
            tag_edit_id: None,
            tag_name: String::new(),
            tag_color: "#03A9F4".to_string(),
            timesheet_week_offset: 0,
            timesheet_extra_rows: Vec::new(),
            timesheet_add_project_id: None,
            timesheet_selected_cell: None,
            timesheet_quick_duration: "1:00".to_string(),
            timesheet_quick_desc: String::new(),
            pomodoro,
            help_topic: help::HelpTopic::QuickStart,
            help_search: String::new(),
        };
        app.reload_data();
        app.sync_tracker_from_running();
        app
    }

    pub fn reload_data(&mut self) {
        if let Ok(clients) = self.db.list_clients() {
            self.clients_map = clients.iter().map(|c| (c.id, c.clone())).collect();
            self.clients = clients;
        }
        if let Ok(projects) = self.db.list_projects() {
            self.projects_map = projects.iter().map(|p| (p.id, p.clone())).collect();
            self.projects = projects;
        }
        if let Ok(tags) = self.db.list_tags() {
            self.tags_map = tags.iter().map(|t| (t.id, t.clone())).collect();
            self.tags = tags;
        }
        if let Ok(entries) = self.db.list_time_entries() {
            self.entries = entries;
        }
    }

    pub fn running_entry(&self) -> Option<&TimeEntry> {
        self.entries.iter().find(|e| e.is_running())
    }

    pub fn sync_tracker_from_running(&mut self) {
        if let Some(running) = self.running_entry().cloned() {
            self.input_description = running.description;
            self.input_project_id = running.project_id;
            self.input_billable = running.billable;
            self.input_tag_ids = running.tag_ids;
        }
    }

    pub fn set_status(&mut self, msg: impl Into<String>, is_error: bool) {
        self.status_msg = Some((msg.into(), is_error, Instant::now()));
    }

    pub fn parse_local_datetime(date_str: &str, time_str: &str) -> Option<chrono::DateTime<Utc>> {
        let date = NaiveDate::parse_from_str(date_str.trim(), "%Y-%m-%d").ok()?;
        let time = NaiveTime::parse_from_str(time_str.trim(), "%H:%M:%S")
            .or_else(|_| NaiveTime::parse_from_str(time_str.trim(), "%H:%M"))
            .ok()?;
        let naive = NaiveDateTime::new(date, time);
        Local
            .from_local_datetime(&naive)
            .single()
            .or_else(|| Local.from_local_datetime(&naive).earliest())
            .map(|dt| dt.with_timezone(&Utc))
    }

    fn render_sidebar(&mut self, ctx: &Context, p: &Palette) {
        egui::SidePanel::left("pro_clock_sidebar")
            .resizable(false)
            .exact_width(215.0)
            .frame(
                Frame::none()
                    .fill(p.bg_sidebar)
                    .stroke(Stroke::new(1.0_f32, p.border))
                    .inner_margin(Margin::symmetric(14.0, 16.0)),
            )
            .show(ctx, |ui| {
                // Brand header
                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(Vec2::splat(28.0), egui::Sense::hover());
                    let painter = ui.painter();
                    painter.circle_filled(rect.center(), 13.0, p.accent);
                    painter.line_segment(
                        [rect.center(), rect.center() + Vec2::new(0.0, -7.0)],
                        Stroke::new(2.2_f32, Color32::WHITE),
                    );
                    painter.line_segment(
                        [rect.center(), rect.center() + Vec2::new(5.5, 2.5)],
                        Stroke::new(2.2_f32, Color32::WHITE),
                    );
                    ui.add_space(4.0);
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Pro Clock").size(17.5).strong().color(p.text_primary));
                        ui.label(RichText::new("Offline Time Tracker").size(11.0).color(p.text_muted));
                    });
                });

                ui.add_space(14.0);

                egui::ScrollArea::vertical()
                    .id_salt("sidebar_nav_scroll")
                    .max_height((ui.available_height() - 62.0).max(120.0))
                    .show(ui, |ui| {
                        // Live running timer badge in sidebar
                        if let Some(running) = self.running_entry().cloned() {
                            Frame::none()
                                .fill(Color32::from_rgba_unmultiplied(233, 84, 32, 32))
                                .stroke(Stroke::new(1.0_f32, p.accent))
                                .rounding(Rounding::same(8.0))
                                .inner_margin(Margin::same(10.0))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        color_dot(ui, p.accent, 4.5);
                                        ui.label(RichText::new("TRACKING").size(10.5).strong().color(p.accent));
                                    });
                                    ui.label(
                                        RichText::new(format_duration(running.duration_secs()))
                                            .monospace()
                                            .size(18.0)
                                            .strong()
                                            .color(p.text_primary),
                                    );
                                    let desc = if running.description.is_empty() {
                                        "(No description)"
                                    } else {
                                        &running.description
                                    };
                                    ui.label(
                                        RichText::new(desc)
                                            .size(12.0)
                                            .color(p.text_muted),
                                    );
                                });
                            ui.add_space(10.0);
                        }

                        // Live Pomodoro indicator in sidebar
                        if app_pomo_active(&self.pomodoro) {
                            let pomo_color = match self.pomodoro.phase {
                                PomodoroPhase::Focus => p.accent,
                                PomodoroPhase::ShortBreak => p.accent_blue,
                                PomodoroPhase::LongBreak => p.success,
                            };
                            Frame::none()
                                .fill(Color32::from_rgba_unmultiplied(
                                    pomo_color.r(),
                                    pomo_color.g(),
                                    pomo_color.b(),
                                    30,
                                ))
                                .stroke(Stroke::new(1.0_f32, pomo_color))
                                .rounding(Rounding::same(8.0))
                                .inner_margin(Margin::same(10.0))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        color_dot(ui, pomo_color, 4.5);
                                        ui.label(
                                            RichText::new(self.pomodoro.phase.label().to_uppercase())
                                                .size(10.5)
                                                .strong()
                                                .color(pomo_color),
                                        );
                                    });
                                    ui.label(
                                        RichText::new(self.pomodoro.formatted_remaining())
                                            .monospace()
                                            .size(18.0)
                                            .strong()
                                            .color(p.text_primary),
                                    );
                                });
                            ui.add_space(10.0);
                        }

                        ui.label(RichText::new("TRACK").size(10.5).strong().color(p.text_muted));
                        ui.add_space(2.0);
                        self.nav_button(ui, p, NavTab::Tracker, "⏱  Time Tracker");
                        self.nav_button(ui, p, NavTab::Timesheet, "📅  Timesheet");
                        self.nav_button(ui, p, NavTab::Pomodoro, "🍅  Pomodoro");
                        self.nav_button(ui, p, NavTab::Reports, "📊  Reports & Export");

                        ui.add_space(12.0);
                        ui.label(RichText::new("MANAGE").size(10.5).strong().color(p.text_muted));
                        ui.add_space(2.0);
                        self.nav_button(ui, p, NavTab::Projects, "📁  Projects");
                        self.nav_button(ui, p, NavTab::Clients, "🏢  Clients");
                        self.nav_button(ui, p, NavTab::Tags, "🏷  Tags");

                        ui.add_space(12.0);
                        ui.label(RichText::new("SYSTEM").size(10.5).strong().color(p.text_muted));
                        ui.add_space(2.0);
                        self.nav_button(ui, p, NavTab::Settings, "⚙  Settings");
                        self.nav_button(ui, p, NavTab::Help, "❓  Help & Guide");
                    });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.label(
                        RichText::new("v0.2.0 • MIT • 100% Offline")
                            .size(11.0)
                            .color(p.text_muted),
                    );
                    ui.add_space(4.0);
                    let theme_label = if self.settings.dark_mode {
                        "☀  Switch to Light"
                    } else {
                        "🌙  Switch to Dark"
                    };
                    if ui
                        .add(
                            egui::Button::new(RichText::new(theme_label).size(12.5))
                                .min_size(Vec2::new(ui.available_width(), 30.0)),
                        )
                        .clicked()
                    {
                        self.settings.dark_mode = !self.settings.dark_mode;
                        let _ = self.db.save_settings(&self.settings);
                        apply_theme(ctx, self.settings.dark_mode);
                    }
                });
            });
    }

    fn nav_button(&mut self, ui: &mut egui::Ui, p: &Palette, tab: NavTab, label: &str) {
        let selected = self.active_tab == tab;
        let fill = if selected {
            Color32::from_rgba_unmultiplied(233, 84, 32, 42)
        } else {
            Color32::TRANSPARENT
        };
        let text_color = if selected { p.accent } else { p.text_primary };
        let stroke = if selected {
            Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(233, 84, 32, 110))
        } else {
            Stroke::NONE
        };

        let btn = egui::Button::new(RichText::new(label).size(13.5).color(text_color).strong())
            .fill(fill)
            .stroke(stroke)
            .rounding(Rounding::same(7.0))
            .min_size(Vec2::new(ui.available_width(), 34.0));

        if ui.add(btn).clicked() {
            self.active_tab = tab;
        }
    }
}

impl eframe::App for ProClockApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Advance Pomodoro timer regardless of active tab
        pomodoro::tick_pomodoro(self);

        // Repaint smoothly every half-second when either timer is running
        if self.running_entry().is_some() || self.pomodoro.running {
            ctx.request_repaint_after(std::time::Duration::from_millis(500));
        }

        // Expire status banner after 4.5s
        if let Some((_, _, ts)) = self.status_msg {
            if ts.elapsed().as_secs_f32() > 4.5 {
                self.status_msg = None;
            } else {
                ctx.request_repaint_after(std::time::Duration::from_millis(500));
            }
        }

        let p = Palette::for_mode(self.settings.dark_mode);
        self.render_sidebar(ctx, &p);

        egui::CentralPanel::default()
            .frame(
                Frame::none()
                    .fill(p.bg_canvas)
                    .inner_margin(Margin::same(20.0)),
            )
            .show(ctx, |ui| {
                // Status toast bar
                if let Some((msg, is_err, _)) = self.status_msg.clone() {
                    let banner_color = if is_err { p.danger } else { p.success };
                    Frame::none()
                        .fill(Color32::from_rgba_unmultiplied(
                            banner_color.r(),
                            banner_color.g(),
                            banner_color.b(),
                            35,
                        ))
                        .stroke(Stroke::new(1.0_f32, banner_color))
                        .rounding(Rounding::same(8.0))
                        .inner_margin(Margin::symmetric(12.0, 8.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(&msg).color(banner_color).strong());
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.small_button("✕").clicked() {
                                        self.status_msg = None;
                                    }
                                });
                            });
                        });
                    ui.add_space(10.0);
                }

                match self.active_tab {
                    NavTab::Tracker => tracker::render_tracker_view(self, ui, &p),
                    NavTab::Timesheet => timesheet::render_timesheet_view(self, ui, &p),
                    NavTab::Pomodoro => pomodoro::render_pomodoro_view(self, ui, &p),
                    NavTab::Reports => reports::render_reports_view(self, ui, &p),
                    NavTab::Projects => manage::render_projects_view(self, ui, &p),
                    NavTab::Clients => manage::render_clients_view(self, ui, &p),
                    NavTab::Tags => manage::render_tags_view(self, ui, &p),
                    NavTab::Settings => manage::render_settings_view(self, ctx, ui, &p),
                    NavTab::Help => help::render_help_view(self, ui, &p),
                }
            });

        // Modal dialog for editing a time entry
        tracker::render_edit_modal(self, ctx, &p);
    }
}

fn app_pomo_active(pomo: &PomodoroState) -> bool {
    pomo.running
}
