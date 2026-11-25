use crate::models::{format_duration, format_money, parse_hex_color, TimeEntry};
use crate::notify::send_notification;
use crate::theme::{card_frame, color_dot, pill_badge, primary_button, with_alpha, Palette};
use crate::ui::{EditEntryModal, ProClockApp, TrackerMode};
use chrono::{Duration, Local, NaiveDate, Utc};
use egui::{Color32, ComboBox, Context, Frame, Margin, RichText, Rounding, ScrollArea, Stroke, TextEdit, Ui, Vec2};
use std::collections::BTreeMap;

pub fn render_tracker_view(app: &mut ProClockApp, ui: &mut Ui, p: &Palette) {
    // ── Header + Mode Switcher ────────────────────────────────────────────────
    ui.horizontal(|ui| {
        ui.heading(RichText::new("Time Tracker").color(p.text_primary).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let manual_selected = app.tracker_mode == TrackerMode::Manual;
            if ui
                .selectable_label(manual_selected, RichText::new("✏ Manual").size(13.0))
                .clicked()
            {
                app.tracker_mode = TrackerMode::Manual;
                let now = Local::now();
                app.manual_date = now.format("%Y-%m-%d").to_string();
                app.manual_start = (now - Duration::hours(1)).format("%H:%M").to_string();
                app.manual_end = now.format("%H:%M").to_string();
            }
            if ui
                .selectable_label(!manual_selected, RichText::new("⏱ Timer").size(13.0))
                .clicked()
            {
                app.tracker_mode = TrackerMode::Timer;
            }
        });
    });

    ui.add_space(10.0);

    // ── Top Tracker Bar Card ──────────────────────────────────────────────────
    let running = app.running_entry().cloned();
    card_frame(p).show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            // Description input
            let desc_resp = ui.add(
                TextEdit::singleline(&mut app.input_description)
                    .hint_text("What are you working on?")
                    .desired_width((ui.available_width() - 480.0).max(200.0))
                    .margin(Vec2::new(10.0, 8.0)),
            );

            // If timer is running and user edits description, persist it
            if let Some(ref run) = running {
                if desc_resp.lost_focus() && app.input_description != run.description {
                    if let Err(e) = app.db.update_time_entry(
                        run.id,
                        &app.input_description,
                        app.input_project_id,
                        run.start_time,
                        None,
                        app.input_billable,
                        run.hourly_rate,
                        &app.input_tag_ids,
                    ) {
                        app.set_status(format!("Failed to update timer: {}", e), true);
                    } else {
                        app.reload_data();
                    }
                }
            }

            // Project dropdown
            let selected_proj_label = app
                .input_project_id
                .and_then(|id| app.projects_map.get(&id))
                .map(|pr| pr.name.clone())
                .unwrap_or_else(|| "+ Project".to_string());

            let prev_proj = app.input_project_id;
            ComboBox::from_id_salt("tracker_project_combo")
                .selected_text(selected_proj_label)
                .width(145.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.input_project_id, None, "No Project");
                    for proj in app.projects.iter().filter(|pr| !pr.archived) {
                        let client_suffix = proj
                            .client_id
                            .and_then(|cid| app.clients_map.get(&cid))
                            .map(|c| format!(" ({})", c.name))
                            .unwrap_or_default();
                        ui.horizontal(|ui| {
                            color_dot(ui, parse_hex_color(&proj.color), 4.5);
                            ui.selectable_value(
                                &mut app.input_project_id,
                                Some(proj.id),
                                format!("{}{}", proj.name, client_suffix),
                            );
                        });
                    }
                });

            if app.input_project_id != prev_proj {
                if let Some(pid) = app.input_project_id {
                    if let Some(pr) = app.projects_map.get(&pid) {
                        app.input_billable = pr.billable_default;
                    }
                }
                if let Some(ref run) = running {
                    let _ = app.db.update_time_entry(
                        run.id,
                        &app.input_description,
                        app.input_project_id,
                        run.start_time,
                        None,
                        app.input_billable,
                        run.hourly_rate,
                        &app.input_tag_ids,
                    );
                    app.reload_data();
                }
            }

            // Tags menu button
            let tags_label = if app.input_tag_ids.is_empty() {
                "🏷 Tags".to_string()
            } else {
                format!("🏷 Tags ({})", app.input_tag_ids.len())
            };
            let prev_tags = app.input_tag_ids.clone();
            ui.menu_button(tags_label, |ui| {
                if app.tags.is_empty() {
                    ui.label(RichText::new("No tags yet — add them in Tags tab").color(p.text_muted).small());
                } else {
                    for tag in &app.tags {
                        let mut checked = app.input_tag_ids.contains(&tag.id);
                        if ui.checkbox(&mut checked, &tag.name).changed() {
                            if checked {
                                if !app.input_tag_ids.contains(&tag.id) {
                                    app.input_tag_ids.push(tag.id);
                                }
                            } else {
                                app.input_tag_ids.retain(|&id| id != tag.id);
                            }
                        }
                    }
                }
            });
            if app.input_tag_ids != prev_tags {
                if let Some(ref run) = running {
                    let _ = app.db.update_time_entry(
                        run.id,
                        &app.input_description,
                        app.input_project_id,
                        run.start_time,
                        None,
                        app.input_billable,
                        run.hourly_rate,
                        &app.input_tag_ids,
                    );
                    app.reload_data();
                }
            }

            // Billable toggle button
            let bill_color = if app.input_billable { p.accent_blue } else { p.text_muted };
            let bill_bg = if app.input_billable {
                with_alpha(p.accent_blue, 35)
            } else {
                Color32::TRANSPARENT
            };
            if ui
                .add(
                    egui::Button::new(
                        RichText::new(&app.settings.currency_symbol)
                            .size(15.0)
                            .strong()
                            .color(bill_color),
                    )
                    .fill(bill_bg)
                    .min_size(Vec2::new(34.0, 32.0)),
                )
                .on_hover_text("Toggle billable")
                .clicked()
            {
                app.input_billable = !app.input_billable;
                if let Some(ref run) = running {
                    let _ = app.db.update_time_entry(
                        run.id,
                        &app.input_description,
                        app.input_project_id,
                        run.start_time,
                        None,
                        app.input_billable,
                        run.hourly_rate,
                        &app.input_tag_ids,
                    );
                    app.reload_data();
                }
            }

            // Timer mode: live duration + START/STOP
            if app.tracker_mode == TrackerMode::Timer {
                let elapsed_str = running
                    .as_ref()
                    .map(|r| format_duration(r.duration_secs()))
                    .unwrap_or_else(|| "00:00:00".to_string());

                ui.add_space(4.0);
                ui.label(
                    RichText::new(elapsed_str)
                        .monospace()
                        .size(18.0)
                        .strong()
                        .color(if running.is_some() { p.accent } else { p.text_primary }),
                );
                ui.add_space(4.0);

                if let Some(ref run) = running {
                    if primary_button(ui, "■  STOP", p.danger).clicked() {
                        let dur_label = format_duration(run.duration_secs());
                        let task_label = if run.description.trim().is_empty() {
                            "Time entry".to_string()
                        } else {
                            run.description.clone()
                        };
                        if let Err(e) = app.db.stop_running_timers_at(Utc::now()) {
                            app.set_status(format!("Error stopping timer: {}", e), true);
                        } else {
                            app.input_description.clear();
                            app.input_tag_ids.clear();
                            app.reload_data();
                            send_notification(
                                app.settings.notifications_enabled,
                                "Pro Clock — Timer Stopped",
                                &format!("Logged {} for {}", dur_label, task_label),
                            );
                            app.set_status(format!("Timer stopped ({}) and saved.", dur_label), false);
                        }
                    }
                } else if primary_button(ui, "▶  START", p.accent).clicked() {
                    if let Err(e) = app.db.start_timer(
                        &app.input_description,
                        app.input_project_id,
                        app.input_billable,
                        &app.input_tag_ids,
                    ) {
                        app.set_status(format!("Error starting timer: {}", e), true);
                    } else {
                        app.reload_data();
                        app.set_status("Timer started.", false);
                    }
                }
            }
        });

        // Manual mode extra row for Date, Start, End, and ADD button
        if app.tracker_mode == TrackerMode::Manual {
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Date (YYYY-MM-DD):").size(12.5).color(p.text_muted));
                ui.add(TextEdit::singleline(&mut app.manual_date).desired_width(98.0));

                ui.label(RichText::new("Start (HH:MM):").size(12.5).color(p.text_muted));
                ui.add(TextEdit::singleline(&mut app.manual_start).desired_width(66.0));

                ui.label(RichText::new("End (HH:MM):").size(12.5).color(p.text_muted));
                ui.add(TextEdit::singleline(&mut app.manual_end).desired_width(66.0));

                // Live preview of manual duration
                if let (Some(s), Some(e)) = (
                    ProClockApp::parse_local_datetime(&app.manual_date, &app.manual_start),
                    ProClockApp::parse_local_datetime(&app.manual_date, &app.manual_end),
                ) {
                    let diff = e.signed_duration_since(s).num_seconds();
                    if diff > 0 {
                        ui.label(
                            RichText::new(format_duration(diff))
                                .monospace()
                                .strong()
                                .color(p.accent_blue),
                        );
                    } else {
                        ui.label(RichText::new("End must be after start").small().color(p.danger));
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if primary_button(ui, "+  ADD ENTRY", p.accent_blue).clicked() {
                        match (
                            ProClockApp::parse_local_datetime(&app.manual_date, &app.manual_start),
                            ProClockApp::parse_local_datetime(&app.manual_date, &app.manual_end),
                        ) {
                            (Some(start), Some(end)) if end > start => {
                                if let Err(e) = app.db.create_manual_entry(
                                    &app.input_description,
                                    app.input_project_id,
                                    start,
                                    end,
                                    app.input_billable,
                                    None,
                                    &app.input_tag_ids,
                                ) {
                                    app.set_status(format!("Failed to add entry: {}", e), true);
                                } else {
                                    app.input_description.clear();
                                    app.reload_data();
                                    app.set_status("Manual time entry added.", false);
                                }
                            }
                            _ => {
                                app.set_status(
                                    "Invalid date/time. Use YYYY-MM-DD and HH:MM with End after Start.",
                                    true,
                                );
                            }
                        }
                    }
                });
            });
        }
    });

    ui.add_space(12.0);

    // ── Search & Filter Bar ───────────────────────────────────────────────────
    ui.horizontal_wrapped(|ui| {
        ui.add(
            TextEdit::singleline(&mut app.search_query)
                .hint_text("🔍 Search entries...")
                .desired_width(220.0),
        );

        let filter_proj_label = app
            .filter_project_id
            .and_then(|id| app.projects_map.get(&id))
            .map(|pr| pr.name.clone())
            .unwrap_or_else(|| "All Projects".to_string());

        ComboBox::from_id_salt("tracker_filter_project")
            .selected_text(filter_proj_label)
            .width(150.0)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.filter_project_id, None, "All Projects");
                for proj in &app.projects {
                    ui.selectable_value(&mut app.filter_project_id, Some(proj.id), &proj.name);
                }
            });

        ui.checkbox(&mut app.filter_billable_only, "Billable only");

        if !app.search_query.is_empty() || app.filter_project_id.is_some() || app.filter_billable_only {
            if ui.button("Clear filters").clicked() {
                app.search_query.clear();
                app.filter_project_id = None;
                app.filter_billable_only = false;
            }
        }
    });

    ui.add_space(10.0);

    // ── Grouped Daily Entries List ────────────────────────────────────────────
    let query_lower = app.search_query.to_lowercase();
    let filtered: Vec<TimeEntry> = app
        .entries
        .iter()
        .filter(|e| {
            if app.filter_billable_only && !e.billable {
                return false;
            }
            if let Some(fpid) = app.filter_project_id {
                if e.project_id != Some(fpid) {
                    return false;
                }
            }
            if !query_lower.is_empty() {
                let proj_match = e
                    .project_id
                    .and_then(|pid| app.projects_map.get(&pid))
                    .map(|pr| pr.name.to_lowercase().contains(&query_lower))
                    .unwrap_or(false);
                if !e.description.to_lowercase().contains(&query_lower) && !proj_match {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    if filtered.is_empty() {
        ui.add_space(36.0);
        ui.vertical_centered(|ui| {
            ui.label(RichText::new("No time entries yet").size(17.0).strong().color(p.text_muted));
            ui.add_space(4.0);
            ui.label(
                RichText::new("Type what you're working on above and click START, or switch to Manual mode to log past work.")
                    .size(13.0)
                    .color(p.text_muted),
            );
        });
        return;
    }

    // Group by local NaiveDate (newest day first using Reverse)
    let mut grouped: BTreeMap<std::cmp::Reverse<NaiveDate>, Vec<TimeEntry>> = BTreeMap::new();
    for entry in filtered {
        let day = entry.local_start().date_naive();
        grouped.entry(std::cmp::Reverse(day)).or_default().push(entry);
    }

    let today = Local::now().date_naive();
    let yesterday = today - Duration::days(1);

    let mut action_continue: Option<TimeEntry> = None;
    let mut action_delete: Option<i64> = None;
    let mut action_edit: Option<TimeEntry> = None;

    ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
        for (std::cmp::Reverse(day), day_entries) in &grouped {
            let day_label = if *day == today {
                format!("Today — {}", day.format("%a, %b %d, %Y"))
            } else if *day == yesterday {
                format!("Yesterday — {}", day.format("%a, %b %d, %Y"))
            } else {
                day.format("%a, %b %d, %Y").to_string()
            };

            let total_secs: i64 = day_entries.iter().map(|e| e.duration_secs()).sum();
            let total_billable: f64 = day_entries
                .iter()
                .map(|e| {
                    let proj = e.project_id.and_then(|id| app.projects_map.get(&id));
                    e.billable_amount(proj, app.settings.default_hourly_rate)
                })
                .sum();

            // Day Group Header
            Frame::none()
                .fill(p.bg_elevated)
                .stroke(Stroke::new(1.0_f32, p.border))
                .rounding(Rounding {
                    nw: 8.0,
                    ne: 8.0,
                    sw: 0.0,
                    se: 0.0,
                })
                .inner_margin(Margin::symmetric(14.0, 8.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&day_label).strong().size(13.5).color(p.text_primary));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format_duration(total_secs))
                                    .monospace()
                                    .strong()
                                    .size(14.0)
                                    .color(p.text_primary),
                            );
                            if total_billable > 0.0 {
                                ui.add_space(10.0);
                                ui.label(
                                    RichText::new(format_money(
                                        total_billable,
                                        &app.settings.currency_symbol,
                                    ))
                                    .size(12.5)
                                    .color(p.success),
                                );
                            }
                            ui.add_space(6.0);
                            ui.label(RichText::new("Total:").size(12.0).color(p.text_muted));
                        });
                    });
                });

            // Day Entries Rows
            Frame::none()
                .fill(p.bg_card)
                .stroke(Stroke::new(1.0_f32, p.border))
                .rounding(Rounding {
                    nw: 0.0,
                    ne: 0.0,
                    sw: 8.0,
                    se: 8.0,
                })
                .inner_margin(Margin::symmetric(14.0, 6.0))
                .show(ui, |ui| {
                    for (idx, entry) in day_entries.iter().enumerate() {
                        if idx > 0 {
                            ui.separator();
                        }
                        ui.horizontal(|ui| {
                            ui.set_min_height(36.0);

                            // Description
                            let desc = if entry.description.is_empty() {
                                RichText::new("(No description)").italics().color(p.text_muted)
                            } else {
                                RichText::new(&entry.description).color(p.text_primary)
                            };
                            ui.label(desc);

                            // Project badge
                            if let Some(pid) = entry.project_id {
                                if let Some(proj) = app.projects_map.get(&pid) {
                                    ui.add_space(6.0);
                                    let col = parse_hex_color(&proj.color);
                                    color_dot(ui, col, 4.5);
                                    let client_str = proj
                                        .client_id
                                        .and_then(|cid| app.clients_map.get(&cid))
                                        .map(|c| format!(" • {}", c.name))
                                        .unwrap_or_default();
                                    ui.label(
                                        RichText::new(format!("{}{}", proj.name, client_str))
                                            .size(12.5)
                                            .color(col)
                                            .strong(),
                                    );
                                }
                            }

                            // Tag chips
                            for tid in &entry.tag_ids {
                                if let Some(tag) = app.tags_map.get(tid) {
                                    pill_badge(ui, &tag.name, parse_hex_color(&tag.color));
                                }
                            }

                            // Right-aligned metadata & actions
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui
                                    .small_button(RichText::new("🗑").color(p.danger))
                                    .on_hover_text("Delete entry")
                                    .clicked()
                                {
                                    action_delete = Some(entry.id);
                                }
                                if ui
                                    .small_button("✏")
                                    .on_hover_text("Edit entry")
                                    .clicked()
                                {
                                    action_edit = Some(entry.clone());
                                }
                                if !entry.is_running() {
                                    if ui
                                        .small_button(RichText::new("▶").color(p.accent))
                                        .on_hover_text("Continue timer with this entry")
                                        .clicked()
                                    {
                                        action_continue = Some(entry.clone());
                                    }
                                }

                                ui.add_space(6.0);

                                // Duration
                                let dur_color = if entry.is_running() {
                                    p.accent
                                } else {
                                    p.text_primary
                                };
                                ui.label(
                                    RichText::new(format_duration(entry.duration_secs()))
                                        .monospace()
                                        .strong()
                                        .color(dur_color),
                                );

                                ui.add_space(8.0);

                                // Start - End time range
                                let start_str = entry.local_start().format("%H:%M").to_string();
                                let end_str = entry
                                    .local_end()
                                    .map(|t| t.format("%H:%M").to_string())
                                    .unwrap_or_else(|| "now".to_string());
                                ui.label(
                                    RichText::new(format!("{} - {}", start_str, end_str))
                                        .size(12.5)
                                        .color(p.text_muted),
                                );

                                ui.add_space(6.0);

                                // Billable icon
                                if entry.billable {
                                    ui.label(
                                        RichText::new(&app.settings.currency_symbol)
                                            .strong()
                                            .color(p.accent_blue),
                                    )
                                    .on_hover_text("Billable");
                                }
                            });
                        });
                    }
                });

            ui.add_space(12.0);
        }
    });

    // Apply deferred actions
    if let Some(entry) = action_continue {
        if let Err(e) = app.db.start_timer(
            &entry.description,
            entry.project_id,
            entry.billable,
            &entry.tag_ids,
        ) {
            app.set_status(format!("Failed to continue timer: {}", e), true);
        } else {
            app.reload_data();
            app.sync_tracker_from_running();
            app.set_status("Continued timer.", false);
        }
    }

    if let Some(id) = action_delete {
        if let Err(e) = app.db.delete_time_entry(id) {
            app.set_status(format!("Failed to delete entry: {}", e), true);
        } else {
            app.reload_data();
            app.set_status("Time entry deleted.", false);
        }
    }

    if let Some(entry) = action_edit {
        let local_start = entry.local_start();
        let local_end = entry.local_end();
        app.edit_modal = Some(EditEntryModal {
            id: entry.id,
            description: entry.description,
            project_id: entry.project_id,
            date_str: local_start.format("%Y-%m-%d").to_string(),
            start_str: local_start.format("%H:%M:%S").to_string(),
            end_str: local_end
                .map(|t| t.format("%H:%M:%S").to_string())
                .unwrap_or_default(),
            is_running: entry.end_time.is_none(),
            billable: entry.billable,
            hourly_rate_str: entry
                .hourly_rate
                .map(|r| format!("{:.2}", r))
                .unwrap_or_default(),
            tag_ids: entry.tag_ids,
        });
    }
}

pub fn render_edit_modal(app: &mut ProClockApp, ctx: &Context, p: &Palette) {
    let Some(mut modal) = app.edit_modal.clone() else {
        return;
    };

    let mut close_modal = false;
    let mut save_modal = false;

    egui::Window::new("Edit Time Entry")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .fixed_size(Vec2::new(430.0, 340.0))
        .show(ctx, |ui| {
            ui.label(RichText::new("Description").size(12.5).color(p.text_muted));
            ui.add(
                TextEdit::singleline(&mut modal.description)
                    .desired_width(ui.available_width())
                    .hint_text("What did you work on?"),
            );
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Project").size(12.5).color(p.text_muted));
                    let proj_label = modal
                        .project_id
                        .and_then(|id| app.projects_map.get(&id))
                        .map(|pr| pr.name.clone())
                        .unwrap_or_else(|| "No Project".to_string());
                    ComboBox::from_id_salt("edit_modal_project")
                        .selected_text(proj_label)
                        .width(190.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut modal.project_id, None, "No Project");
                            for proj in &app.projects {
                                ui.selectable_value(&mut modal.project_id, Some(proj.id), &proj.name);
                            }
                        });
                });

                ui.add_space(12.0);
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(format!("Custom Rate ({}/h)", app.settings.currency_symbol))
                            .size(12.5)
                            .color(p.text_muted),
                    );
                    ui.add(
                        TextEdit::singleline(&mut modal.hourly_rate_str)
                            .hint_text("Use project rate")
                            .desired_width(140.0),
                    );
                });
            });

            ui.add_space(8.0);
            ui.checkbox(&mut modal.billable, "Billable");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Date (YYYY-MM-DD)").size(12.5).color(p.text_muted));
                    ui.add(TextEdit::singleline(&mut modal.date_str).desired_width(120.0));
                });
                ui.vertical(|ui| {
                    ui.label(RichText::new("Start (HH:MM:SS)").size(12.5).color(p.text_muted));
                    ui.add(TextEdit::singleline(&mut modal.start_str).desired_width(110.0));
                });
                if !modal.is_running {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("End (HH:MM:SS)").size(12.5).color(p.text_muted));
                        ui.add(TextEdit::singleline(&mut modal.end_str).desired_width(110.0));
                    });
                }
            });

            ui.add_space(8.0);
            ui.label(RichText::new("Tags").size(12.5).color(p.text_muted));
            ui.horizontal_wrapped(|ui| {
                if app.tags.is_empty() {
                    ui.label(RichText::new("No tags created yet").small().color(p.text_muted));
                } else {
                    for tag in &app.tags {
                        let mut selected = modal.tag_ids.contains(&tag.id);
                        if ui.checkbox(&mut selected, &tag.name).changed() {
                            if selected {
                                if !modal.tag_ids.contains(&tag.id) {
                                    modal.tag_ids.push(tag.id);
                                }
                            } else {
                                modal.tag_ids.retain(|&id| id != tag.id);
                            }
                        }
                    }
                }
            });

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if primary_button(ui, "Save Changes", p.accent).clicked() {
                        save_modal = true;
                    }
                    if ui.button("Cancel").clicked() {
                        close_modal = true;
                    }
                });
            });
        });

    if save_modal {
        let parsed_start = ProClockApp::parse_local_datetime(&modal.date_str, &modal.start_str);
        let parsed_end = if modal.is_running {
            Some(None)
        } else {
            ProClockApp::parse_local_datetime(&modal.date_str, &modal.end_str).map(Some)
        };
        let parsed_rate = if modal.hourly_rate_str.trim().is_empty() {
            Ok(None)
        } else {
            modal.hourly_rate_str.trim().parse::<f64>().map(Some)
        };

        match (parsed_start, parsed_end, parsed_rate) {
            (Some(start), Some(end_opt), Ok(rate_opt)) => {
                if let Some(end) = end_opt {
                    if end < start {
                        app.set_status("End time must be after start time.", true);
                        app.edit_modal = Some(modal);
                        return;
                    }
                }
                if let Err(e) = app.db.update_time_entry(
                    modal.id,
                    &modal.description,
                    modal.project_id,
                    start,
                    end_opt,
                    modal.billable,
                    rate_opt,
                    &modal.tag_ids,
                ) {
                    app.set_status(format!("Failed to update entry: {}", e), true);
                } else {
                    app.reload_data();
                    if modal.is_running {
                        app.sync_tracker_from_running();
                    }
                    app.set_status("Time entry updated.", false);
                    app.edit_modal = None;
                }
            }
            _ => {
                app.set_status("Invalid date, time, or hourly rate format.", true);
                app.edit_modal = Some(modal);
            }
        }
    } else if close_modal {
        app.edit_modal = None;
    } else {
        app.edit_modal = Some(modal);
    }
}
