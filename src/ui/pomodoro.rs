use crate::models::{parse_hex_color, AppSettings};
use crate::notify::send_notification;
use crate::theme::{card_frame, color_dot, primary_button, Palette};
use crate::ui::ProClockApp;
use chrono::{DateTime, Utc};
use egui::{Align2, Color32, ComboBox, FontId, Pos2, RichText, Stroke, TextEdit, Ui, Vec2};
use std::f32::consts::{FRAC_PI_2, TAU};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PomodoroPhase {
    Focus,
    ShortBreak,
    LongBreak,
}

impl PomodoroPhase {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Focus => "Focus Session",
            Self::ShortBreak => "Short Break",
            Self::LongBreak => "Long Break",
        }
    }

    pub fn duration_secs(&self, s: &AppSettings) -> i64 {
        match self {
            Self::Focus => (s.pomodoro_focus_mins.max(1) as i64) * 60,
            Self::ShortBreak => (s.pomodoro_short_break_mins.max(1) as i64) * 60,
            Self::LongBreak => (s.pomodoro_long_break_mins.max(1) as i64) * 60,
        }
    }
}

pub struct PomodoroState {
    pub phase: PomodoroPhase,
    pub running: bool,
    pub remaining_secs: i64,
    pub last_tick: Option<Instant>,
    pub completed_focus_sessions: u32,
    pub focus_started_at: Option<DateTime<Utc>>,
    pub description: String,
    pub project_id: Option<i64>,
    pub billable: bool,
}

impl PomodoroState {
    pub fn new(settings: &AppSettings) -> Self {
        Self {
            phase: PomodoroPhase::Focus,
            running: false,
            remaining_secs: PomodoroPhase::Focus.duration_secs(settings),
            last_tick: None,
            completed_focus_sessions: 0,
            focus_started_at: None,
            description: String::new(),
            project_id: None,
            billable: false,
        }
    }

    pub fn formatted_remaining(&self) -> String {
        let secs = self.remaining_secs.max(0);
        format!("{:02}:{:02}", secs / 60, secs % 60)
    }

    pub fn switch_phase(&mut self, phase: PomodoroPhase, settings: &AppSettings) {
        self.phase = phase;
        self.running = false;
        self.last_tick = None;
        self.focus_started_at = None;
        self.remaining_secs = phase.duration_secs(settings);
    }
}

/// Called every frame from `ProClockApp::update` so Pomodoro keeps ticking regardless of active tab.
pub fn tick_pomodoro(app: &mut ProClockApp) {
    if !app.pomodoro.running {
        return;
    }

    let now = Instant::now();
    let elapsed_secs = match app.pomodoro.last_tick {
        Some(prev) => {
            let secs = now.duration_since(prev).as_secs() as i64;
            if secs == 0 {
                return;
            }
            secs
        }
        None => {
            app.pomodoro.last_tick = Some(now);
            return;
        }
    };

    app.pomodoro.last_tick = Some(now);
    app.pomodoro.remaining_secs = (app.pomodoro.remaining_secs - elapsed_secs).max(0);

    if app.pomodoro.remaining_secs == 0 {
        complete_current_phase(app, true);
    }
}

fn complete_current_phase(app: &mut ProClockApp, notify: bool) {
    let finished_phase = app.pomodoro.phase;
    app.pomodoro.running = false;
    app.pomodoro.last_tick = None;

    match finished_phase {
        PomodoroPhase::Focus => {
            app.pomodoro.completed_focus_sessions += 1;

            // Auto-log completed focus session into SQLite time_entries
            if app.settings.pomodoro_auto_log {
                let end = Utc::now();
                let start = app.pomodoro.focus_started_at.unwrap_or_else(|| {
                    end - chrono::Duration::seconds(
                        PomodoroPhase::Focus.duration_secs(&app.settings),
                    )
                });
                if end > start {
                    let desc = if app.pomodoro.description.trim().is_empty() {
                        "Pomodoro Focus Session".to_string()
                    } else {
                        app.pomodoro.description.clone()
                    };
                    if let Err(e) = app.db.create_manual_entry(
                        &desc,
                        app.pomodoro.project_id,
                        start,
                        end,
                        app.pomodoro.billable,
                        None,
                        &[],
                    ) {
                        app.set_status(format!("Failed to auto-log Pomodoro: {}", e), true);
                    } else {
                        app.reload_data();
                    }
                }
            }
            app.pomodoro.focus_started_at = None;

            let cycles = app.settings.pomodoro_cycles_before_long.max(1);
            let next_phase = if app.pomodoro.completed_focus_sessions % cycles == 0 {
                PomodoroPhase::LongBreak
            } else {
                PomodoroPhase::ShortBreak
            };
            app.pomodoro.switch_phase(next_phase, &app.settings);

            if notify {
                let msg = format!(
                    "Focus session #{} complete! Time for a {} ({} min).",
                    app.pomodoro.completed_focus_sessions,
                    next_phase.label().to_lowercase(),
                    next_phase.duration_secs(&app.settings) / 60
                );
                send_notification(
                    app.settings.notifications_enabled,
                    "Pro Clock — Focus Complete",
                    &msg,
                );
                app.set_status(msg, false);
            }
        }
        PomodoroPhase::ShortBreak | PomodoroPhase::LongBreak => {
            app.pomodoro.switch_phase(PomodoroPhase::Focus, &app.settings);
            if notify {
                let msg = "Break finished! Ready for your next focus session.";
                send_notification(
                    app.settings.notifications_enabled,
                    "Pro Clock — Break Over",
                    msg,
                );
                app.set_status(msg, false);
            }
        }
    }
}

pub fn render_pomodoro_view(app: &mut ProClockApp, ui: &mut Ui, p: &Palette) {
    ui.horizontal(|ui| {
        ui.heading(RichText::new("Pomodoro Timer").color(p.text_primary).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                RichText::new(format!(
                    "Completed today: {} focus session(s)",
                    app.pomodoro.completed_focus_sessions
                ))
                .size(12.5)
                .color(p.text_muted),
            );
        });
    });

    ui.add_space(10.0);

    // ── Task & Project Binding Card ───────────────────────────────────────────
    card_frame(p).show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.add(
                TextEdit::singleline(&mut app.pomodoro.description)
                    .hint_text("What are you focusing on?")
                    .desired_width((ui.available_width() - 340.0).max(200.0)),
            );

            let proj_label = app
                .pomodoro
                .project_id
                .and_then(|id| app.projects_map.get(&id))
                .map(|pr| pr.name.clone())
                .unwrap_or_else(|| "+ Project".to_string());
            let prev_proj = app.pomodoro.project_id;
            ComboBox::from_id_salt("pomo_project_combo")
                .selected_text(proj_label)
                .width(155.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.pomodoro.project_id, None, "No Project");
                    for proj in app.projects.iter().filter(|pr| !pr.archived) {
                        ui.horizontal(|ui| {
                            color_dot(ui, parse_hex_color(&proj.color), 4.5);
                            ui.selectable_value(
                                &mut app.pomodoro.project_id,
                                Some(proj.id),
                                &proj.name,
                            );
                        });
                    }
                });
            if app.pomodoro.project_id != prev_proj {
                if let Some(pid) = app.pomodoro.project_id {
                    if let Some(pr) = app.projects_map.get(&pid) {
                        app.pomodoro.billable = pr.billable_default;
                    }
                }
            }

            ui.checkbox(&mut app.pomodoro.billable, "Billable");

            if ui
                .checkbox(&mut app.settings.pomodoro_auto_log, "Auto-log to Tracker")
                .changed()
            {
                let _ = app.db.save_settings(&app.settings);
            }
        });
    });

    ui.add_space(16.0);

    // ── Main Pomodoro Ring Card ───────────────────────────────────────────────
    let phase = app.pomodoro.phase;
    let phase_color = match phase {
        PomodoroPhase::Focus => p.accent,
        PomodoroPhase::ShortBreak => p.accent_blue,
        PomodoroPhase::LongBreak => p.success,
    };

    card_frame(p).show(ui, |ui| {
        ui.vertical_centered(|ui| {
            // Phase selector tabs
            ui.horizontal(|ui| {
                let total_w = 340.0;
                let pad = ((ui.available_width() - total_w) * 0.5).max(0.0);
                ui.add_space(pad);
                for candidate in [
                    PomodoroPhase::Focus,
                    PomodoroPhase::ShortBreak,
                    PomodoroPhase::LongBreak,
                ] {
                    if ui
                        .selectable_label(phase == candidate, candidate.label())
                        .clicked()
                    {
                        app.pomodoro.switch_phase(candidate, &app.settings);
                    }
                }
            });

            ui.add_space(18.0);

            // Circular progress ring
            let total_secs = phase.duration_secs(&app.settings).max(1) as f32;
            let progress = (1.0 - (app.pomodoro.remaining_secs as f32 / total_secs)).clamp(0.0, 1.0);
            draw_pomodoro_ring(
                ui,
                p,
                phase_color,
                progress,
                &app.pomodoro.formatted_remaining(),
                phase.label(),
            );

            ui.add_space(16.0);

            // Cycle indicator dots
            let cycles = app.settings.pomodoro_cycles_before_long.max(1);
            let done_in_cycle = app.pomodoro.completed_focus_sessions % cycles;
            ui.horizontal(|ui| {
                let dots_w = (cycles as f32) * 18.0 + 90.0;
                let pad = ((ui.available_width() - dots_w) * 0.5).max(0.0);
                ui.add_space(pad);
                ui.label(RichText::new("Cycle:").size(12.5).color(p.text_muted));
                for i in 0..cycles {
                    let filled = i < done_in_cycle;
                    color_dot(ui, if filled { p.accent } else { p.border }, 5.0);
                }
            });

            ui.add_space(18.0);

            // Control buttons
            ui.horizontal(|ui| {
                let controls_w = 330.0;
                let pad = ((ui.available_width() - controls_w) * 0.5).max(0.0);
                ui.add_space(pad);

                if app.pomodoro.running {
                    if primary_button(ui, "⏸  PAUSE", p.danger).clicked() {
                        app.pomodoro.running = false;
                        app.pomodoro.last_tick = None;
                    }
                } else if primary_button(ui, "▶  START", phase_color).clicked() {
                    app.pomodoro.running = true;
                    app.pomodoro.last_tick = Some(Instant::now());
                    if app.pomodoro.phase == PomodoroPhase::Focus
                        && app.pomodoro.focus_started_at.is_none()
                    {
                        app.pomodoro.focus_started_at = Some(Utc::now());
                    }
                }

                ui.add_space(6.0);
                if ui
                    .add(egui::Button::new("↺ Reset").min_size(Vec2::new(76.0, 34.0)))
                    .clicked()
                {
                    app.pomodoro.switch_phase(phase, &app.settings);
                }

                ui.add_space(6.0);
                if ui
                    .add(egui::Button::new("⏭ Skip").min_size(Vec2::new(76.0, 34.0)))
                    .clicked()
                {
                    complete_current_phase(app, false);
                }
            });

            ui.add_space(10.0);
            ui.label(
                RichText::new(format!(
                    "Focus: {}m  •  Short Break: {}m  •  Long Break: {}m  (adjust in Settings)",
                    app.settings.pomodoro_focus_mins,
                    app.settings.pomodoro_short_break_mins,
                    app.settings.pomodoro_long_break_mins,
                ))
                .size(12.0)
                .color(p.text_muted),
            );
        });
    });
}

fn draw_pomodoro_ring(
    ui: &mut Ui,
    p: &Palette,
    color: Color32,
    progress: f32,
    time_text: &str,
    subtitle: &str,
) {
    let diameter = 220.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(diameter), egui::Sense::hover());
    let painter = ui.painter();
    let center = rect.center();
    let radius = diameter * 0.5 - 12.0;

    // Background track circle
    painter.circle_stroke(center, radius, Stroke::new(10.0_f32, p.bg_elevated));

    // Progress arc
    if progress > 0.001 {
        let segments = 96;
        let sweep = TAU * progress;
        let step_count = ((segments as f32) * progress).ceil().max(2.0) as usize;
        let points: Vec<Pos2> = (0..=step_count)
            .map(|i| {
                let t = i as f32 / step_count as f32;
                let angle = -FRAC_PI_2 + sweep * t;
                Pos2::new(center.x + radius * angle.cos(), center.y + radius * angle.sin())
            })
            .collect();
        painter.add(egui::Shape::line(points, Stroke::new(10.0_f32, color)));
    }

    // Center countdown text
    painter.text(
        center + Vec2::new(0.0, -8.0),
        Align2::CENTER_CENTER,
        time_text,
        FontId::monospace(42.0),
        p.text_primary,
    );
    painter.text(
        center + Vec2::new(0.0, 26.0),
        Align2::CENTER_CENTER,
        subtitle,
        FontId::proportional(13.5),
        color,
    );
}
