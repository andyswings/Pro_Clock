use crate::models::{
    format_duration_hm, format_money, parse_duration_input, parse_hex_color,
};
use crate::theme::{card_frame, color_dot, primary_button, Palette};
use crate::ui::ProClockApp;
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use egui::{Color32, ComboBox, Grid, RichText, Rounding, ScrollArea, Stroke, TextEdit, Ui, Vec2};
use std::collections::{BTreeSet, HashMap};

fn week_start_for(date: NaiveDate, monday_start: bool) -> NaiveDate {
    let offset = if monday_start {
        date.weekday().num_days_from_monday() as i64
    } else {
        date.weekday().num_days_from_sunday() as i64
    };
    date - Duration::days(offset)
}

pub fn render_timesheet_view(app: &mut ProClockApp, ui: &mut Ui, p: &Palette) {
    let today = Local::now().date_naive();
    let base_start = week_start_for(today, app.settings.week_starts_monday)
        + Duration::weeks(app.timesheet_week_offset);
    let days: Vec<NaiveDate> = (0..7).map(|i| base_start + Duration::days(i)).collect();
    let week_end = days[6];

    // ── Header & Week Navigation ──────────────────────────────────────────────
    ui.horizontal(|ui| {
        ui.heading(RichText::new("Weekly Timesheet").color(p.text_primary).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Next ▶").clicked() {
                app.timesheet_week_offset += 1;
                app.timesheet_selected_cell = None;
            }
            if app.timesheet_week_offset != 0 && ui.button("This Week").clicked() {
                app.timesheet_week_offset = 0;
                app.timesheet_selected_cell = None;
            }
            if ui.button("◀ Prev").clicked() {
                app.timesheet_week_offset -= 1;
                app.timesheet_selected_cell = None;
            }
            ui.add_space(10.0);
            let range_str = format!(
                "{} – {}",
                base_start.format("%b %d, %Y"),
                week_end.format("%b %d, %Y")
            );
            ui.label(RichText::new(range_str).size(14.5).strong().color(p.text_primary));
        });
    });

    ui.add_space(10.0);

    // Aggregate time entries for this week by (project_id, day)
    let mut cell_secs: HashMap<(Option<i64>, NaiveDate), i64> = HashMap::new();
    let mut day_totals = [0i64; 7];
    let mut week_billable_amount = 0.0f64;
    let mut row_keys: BTreeSet<Option<i64>> = BTreeSet::new();

    for entry in &app.entries {
        let d = entry.local_start().date_naive();
        if d >= base_start && d <= week_end {
            let secs = entry.duration_secs();
            *cell_secs.entry((entry.project_id, d)).or_insert(0) += secs;
            let day_idx = (d - base_start).num_days() as usize;
            if day_idx < 7 {
                day_totals[day_idx] += secs;
            }
            let proj = entry.project_id.and_then(|id| app.projects_map.get(&id));
            week_billable_amount += entry.billable_amount(proj, app.settings.default_hourly_rate);
            row_keys.insert(entry.project_id);
        }
    }

    for &extra in &app.timesheet_extra_rows {
        row_keys.insert(extra);
    }

    // Ensure at least one row or active projects are easy to add
    let rows: Vec<Option<i64>> = row_keys.into_iter().collect();
    let grand_total_secs: i64 = day_totals.iter().sum();

    // ── Timesheet Grid Card ───────────────────────────────────────────────────
    card_frame(p).show(ui, |ui| {
        ScrollArea::horizontal().show(ui, |ui| {
            Grid::new("weekly_timesheet_grid")
                .num_columns(9)
                .spacing(Vec2::new(8.0, 8.0))
                .striped(true)
                .show(ui, |ui| {
                    // Header row
                    ui.label(RichText::new("Project").strong().size(13.0).color(p.text_muted));
                    for day in &days {
                        let is_today = *day == today;
                        let col = if is_today { p.accent } else { p.text_muted };
                        let hdr = format!("{}\n{}", day.format("%a"), day.format("%b %d"));
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new(hdr).size(12.0).strong().color(col));
                        });
                    }
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("Total").strong().size(13.0).color(p.text_muted));
                    });
                    ui.end_row();

                    // Project rows
                    if rows.is_empty() {
                        ui.label(
                            RichText::new("No projects logged this week — add a project row below.")
                                .italics()
                                .color(p.text_muted),
                        );
                        for _ in 0..8 {
                            ui.label("");
                        }
                        ui.end_row();
                    } else {
                        for proj_id in &rows {
                            // Project column
                            ui.horizontal(|ui| {
                                ui.set_min_width(180.0);
                                match proj_id.and_then(|id| app.projects_map.get(&id)) {
                                    Some(proj) => {
                                        color_dot(ui, parse_hex_color(&proj.color), 5.0);
                                        ui.label(
                                            RichText::new(&proj.name)
                                                .strong()
                                                .color(p.text_primary),
                                        );
                                    }
                                    None => {
                                        color_dot(ui, p.text_muted, 5.0);
                                        ui.label(
                                            RichText::new("No Project")
                                                .italics()
                                                .color(p.text_muted),
                                        );
                                    }
                                }
                            });

                            let mut row_total = 0i64;
                            for day in &days {
                                let secs = cell_secs.get(&(*proj_id, *day)).copied().unwrap_or(0);
                                row_total += secs;
                                let is_selected =
                                    app.timesheet_selected_cell == Some((*proj_id, *day));

                                let label = if secs > 0 {
                                    format_duration_hm(secs)
                                } else {
                                    "00:00".to_string()
                                };
                                let text_col = if is_selected {
                                    Color32::WHITE
                                } else if secs > 0 {
                                    p.text_primary
                                } else {
                                    p.text_muted
                                };
                                let fill = if is_selected {
                                    p.accent
                                } else if secs > 0 {
                                    Color32::from_rgba_unmultiplied(3, 169, 244, 28)
                                } else {
                                    p.bg_elevated
                                };
                                let stroke = if is_selected {
                                    Stroke::new(1.5_f32, p.accent)
                                } else {
                                    Stroke::new(1.0_f32, p.border)
                                };

                                let btn = egui::Button::new(
                                    RichText::new(label).monospace().size(13.0).color(text_col),
                                )
                                .fill(fill)
                                .stroke(stroke)
                                .rounding(Rounding::same(6.0))
                                .min_size(Vec2::new(72.0, 32.0));

                                if ui
                                    .add(btn)
                                    .on_hover_text("Click to log time to this project & day")
                                    .clicked()
                                {
                                    app.timesheet_selected_cell = Some((*proj_id, *day));
                                    if app.timesheet_quick_duration.is_empty() {
                                        app.timesheet_quick_duration = "1:00".to_string();
                                    }
                                }
                            }

                            // Row total
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    RichText::new(format_duration_hm(row_total))
                                        .monospace()
                                        .strong()
                                        .size(13.5)
                                        .color(p.text_primary),
                                );
                            });
                            ui.end_row();
                        }
                    }

                    // Daily totals footer row
                    ui.label(RichText::new("Daily Total").strong().size(13.0).color(p.text_primary));
                    for &tot in &day_totals {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new(format_duration_hm(tot))
                                    .monospace()
                                    .strong()
                                    .size(13.0)
                                    .color(if tot > 0 { p.accent_blue } else { p.text_muted }),
                            );
                        });
                    }
                    ui.vertical_centered(|ui| {
                        ui.label(
                            RichText::new(format_duration_hm(grand_total_secs))
                                .monospace()
                                .strong()
                                .size(14.5)
                                .color(p.accent),
                        );
                    });
                    ui.end_row();
                });
        });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(8.0);

        // Add project row bar + weekly billable summary
        ui.horizontal_wrapped(|ui| {
            let add_label = app
                .timesheet_add_project_id
                .and_then(|id| app.projects_map.get(&id))
                .map(|pr| pr.name.clone())
                .unwrap_or_else(|| "Select project to add row...".to_string());

            ComboBox::from_id_salt("timesheet_add_row_combo")
                .selected_text(add_label)
                .width(210.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.timesheet_add_project_id, None, "No Project");
                    for proj in app.projects.iter().filter(|pr| !pr.archived) {
                        ui.selectable_value(
                            &mut app.timesheet_add_project_id,
                            Some(proj.id),
                            &proj.name,
                        );
                    }
                });

            if ui.button("+ Add Row").clicked() {
                if !app.timesheet_extra_rows.contains(&app.timesheet_add_project_id) {
                    app.timesheet_extra_rows.push(app.timesheet_add_project_id);
                }
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if week_billable_amount > 0.0 {
                    ui.label(
                        RichText::new(format!(
                            "Week Billable: {}",
                            format_money(week_billable_amount, &app.settings.currency_symbol)
                        ))
                        .strong()
                        .color(p.success),
                    );
                }
            });
        });
    });

    ui.add_space(12.0);

    // ── Quick-Log Selected Cell Card ──────────────────────────────────────────
    if let Some((sel_proj_id, sel_day)) = app.timesheet_selected_cell {
        let proj_name = sel_proj_id
            .and_then(|id| app.projects_map.get(&id))
            .map(|pr| pr.name.clone())
            .unwrap_or_else(|| "No Project".to_string());

        card_frame(p).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!(
                        "Quick Log — {} on {}",
                        proj_name,
                        sel_day.format("%a, %b %d, %Y")
                    ))
                    .size(14.5)
                    .strong()
                    .color(p.accent),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.small_button("✕").clicked() {
                        app.timesheet_selected_cell = None;
                    }
                });
            });
            ui.add_space(6.0);

            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Duration:").size(12.5).color(p.text_muted));
                let dur_resp = ui.add(
                    TextEdit::singleline(&mut app.timesheet_quick_duration)
                        .hint_text("e.g. 1:30, 1.5, 45m")
                        .desired_width(120.0),
                );

                ui.label(RichText::new("Note:").size(12.5).color(p.text_muted));
                let note_resp = ui.add(
                    TextEdit::singleline(&mut app.timesheet_quick_desc)
                        .hint_text("Optional description")
                        .desired_width(240.0),
                );

                let enter_pressed = (dur_resp.lost_focus() || note_resp.lost_focus())
                    && ui.input(|i| i.key_pressed(egui::Key::Enter));

                if primary_button(ui, "+ Add Time", p.accent).clicked() || enter_pressed {
                    if let Some(secs) = parse_duration_input(&app.timesheet_quick_duration) {
                        let start_naive = NaiveDateTime::new(
                            sel_day,
                            NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                        );
                        let start_utc = Local
                            .from_local_datetime(&start_naive)
                            .single()
                            .or_else(|| Local.from_local_datetime(&start_naive).earliest())
                            .map(|dt| dt.with_timezone(&Utc))
                            .unwrap_or_else(Utc::now);
                        let end_utc = start_utc + Duration::seconds(secs);
                        let billable = sel_proj_id
                            .and_then(|id| app.projects_map.get(&id))
                            .map(|pr| pr.billable_default)
                            .unwrap_or(false);

                        match app.db.create_manual_entry(
                            &app.timesheet_quick_desc,
                            sel_proj_id,
                            start_utc,
                            end_utc,
                            billable,
                            None,
                            &[],
                        ) {
                            Ok(_) => {
                                app.timesheet_quick_desc.clear();
                                app.reload_data();
                                app.set_status(
                                    format!(
                                        "Added {} to {} ({}).",
                                        format_duration_hm(secs),
                                        proj_name,
                                        sel_day.format("%b %d")
                                    ),
                                    false,
                                );
                            }
                            Err(e) => app.set_status(format!("Failed to log time: {}", e), true),
                        }
                    } else {
                        app.set_status(
                            "Invalid duration. Try formats like 1:30, 1.5, 2h, or 45m.",
                            true,
                        );
                    }
                }
            });
        });
    } else {
        ui.label(
            RichText::new("Tip: Click any cell in the grid above to quickly log hours for that project and day.")
                .size(12.5)
                .color(p.text_muted),
        );
    }
}
