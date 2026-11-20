use crate::export::{export_csv, export_json};
use crate::models::{format_duration, format_hours_decimal, format_money, parse_hex_color, TimeEntry};
use crate::theme::{card_frame, color_dot, primary_button, Palette};
use crate::ui::{DateRangePreset, ProClockApp};
use chrono::{Datelike, Duration, Local, NaiveDate};
use egui::{Align2, Color32, ComboBox, FontId, Pos2, Rect, RichText, Rounding, ScrollArea, Stroke, TextEdit, Ui, Vec2};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

fn resolve_date_range(app: &ProClockApp) -> (Option<NaiveDate>, Option<NaiveDate>) {
    let today = Local::now().date_naive();
    let week_start = |d: NaiveDate| {
        let offset = if app.settings.week_starts_monday {
            d.weekday().num_days_from_monday() as i64
        } else {
            d.weekday().num_days_from_sunday() as i64
        };
        d - Duration::days(offset)
    };

    match app.report_preset {
        DateRangePreset::Today => (Some(today), Some(today)),
        DateRangePreset::ThisWeek => {
            let start = week_start(today);
            (Some(start), Some(start + Duration::days(6)))
        }
        DateRangePreset::LastWeek => {
            let start = week_start(today) - Duration::days(7);
            (Some(start), Some(start + Duration::days(6)))
        }
        DateRangePreset::ThisMonth => {
            let start = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap_or(today);
            let next_month = if today.month() == 12 {
                NaiveDate::from_ymd_opt(today.year() + 1, 1, 1)
            } else {
                NaiveDate::from_ymd_opt(today.year(), today.month() + 1, 1)
            }
            .unwrap_or(today);
            (Some(start), Some(next_month - Duration::days(1)))
        }
        DateRangePreset::LastMonth => {
            let first_this = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap_or(today);
            let end_last = first_this - Duration::days(1);
            let start_last =
                NaiveDate::from_ymd_opt(end_last.year(), end_last.month(), 1).unwrap_or(end_last);
            (Some(start_last), Some(end_last))
        }
        DateRangePreset::AllTime => (None, None),
        DateRangePreset::Custom => (
            NaiveDate::parse_from_str(app.report_start_date.trim(), "%Y-%m-%d").ok(),
            NaiveDate::parse_from_str(app.report_end_date.trim(), "%Y-%m-%d").ok(),
        ),
    }
}

pub fn render_reports_view(app: &mut ProClockApp, ui: &mut Ui, p: &Palette) {
    ui.heading(RichText::new("Reports & Analytics").color(p.text_primary).strong());
    ui.add_space(10.0);

    // ── Filter Controls Card ──────────────────────────────────────────────────
    card_frame(p).show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            let presets = [
                (DateRangePreset::Today, "Today"),
                (DateRangePreset::ThisWeek, "This Week"),
                (DateRangePreset::LastWeek, "Last Week"),
                (DateRangePreset::ThisMonth, "This Month"),
                (DateRangePreset::LastMonth, "Last Month"),
                (DateRangePreset::AllTime, "All Time"),
                (DateRangePreset::Custom, "Custom"),
            ];
            for (preset, label) in presets {
                if ui
                    .selectable_label(app.report_preset == preset, label)
                    .clicked()
                {
                    app.report_preset = preset;
                }
            }
        });

        if app.report_preset == DateRangePreset::Custom {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("From (YYYY-MM-DD):").size(12.5).color(p.text_muted));
                ui.add(TextEdit::singleline(&mut app.report_start_date).desired_width(105.0));
                ui.label(RichText::new("To (YYYY-MM-DD):").size(12.5).color(p.text_muted));
                ui.add(TextEdit::singleline(&mut app.report_end_date).desired_width(105.0));
            });
        }

        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            let proj_label = app
                .report_project_id
                .and_then(|id| app.projects_map.get(&id))
                .map(|pr| pr.name.clone())
                .unwrap_or_else(|| "All Projects".to_string());
            ComboBox::from_id_salt("report_project_filter")
                .selected_text(proj_label)
                .width(155.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.report_project_id, None, "All Projects");
                    for proj in &app.projects {
                        ui.selectable_value(&mut app.report_project_id, Some(proj.id), &proj.name);
                    }
                });

            let client_label = app
                .report_client_id
                .and_then(|id| app.clients_map.get(&id))
                .map(|c| c.name.clone())
                .unwrap_or_else(|| "All Clients".to_string());
            ComboBox::from_id_salt("report_client_filter")
                .selected_text(client_label)
                .width(155.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.report_client_id, None, "All Clients");
                    for client in &app.clients {
                        ui.selectable_value(&mut app.report_client_id, Some(client.id), &client.name);
                    }
                });

            ui.checkbox(&mut app.report_billable_only, "Billable only");
        });
    });

    ui.add_space(12.0);

    // Filter entries according to active report criteria
    let (start_date, end_date) = resolve_date_range(app);
    let filtered: Vec<TimeEntry> = app
        .entries
        .iter()
        .filter(|e| {
            let d = e.local_start().date_naive();
            if let Some(s) = start_date {
                if d < s {
                    return false;
                }
            }
            if let Some(end) = end_date {
                if d > end {
                    return false;
                }
            }
            if app.report_billable_only && !e.billable {
                return false;
            }
            if let Some(pid) = app.report_project_id {
                if e.project_id != Some(pid) {
                    return false;
                }
            }
            if let Some(cid) = app.report_client_id {
                let entry_cid = e
                    .project_id
                    .and_then(|pid| app.projects_map.get(&pid))
                    .and_then(|pr| pr.client_id);
                if entry_cid != Some(cid) {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    let total_secs: i64 = filtered.iter().map(|e| e.duration_secs()).sum();
    let billable_secs: i64 = filtered
        .iter()
        .filter(|e| e.billable)
        .map(|e| e.duration_secs())
        .sum();
    let total_amount: f64 = filtered
        .iter()
        .map(|e| {
            let proj = e.project_id.and_then(|id| app.projects_map.get(&id));
            e.billable_amount(proj, app.settings.default_hourly_rate)
        })
        .sum();

    ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
        // ── KPI Summary Cards ─────────────────────────────────────────────────
        ui.columns(4, |cols| {
            kpi_card(&mut cols[0], p, "TOTAL TIME", &format_duration(total_secs), &format_hours_decimal(total_secs), p.text_primary);
            kpi_card(&mut cols[1], p, "BILLABLE TIME", &format_duration(billable_secs), &format_hours_decimal(billable_secs), p.accent_blue);
            kpi_card(
                &mut cols[2],
                p,
                "BILLABLE AMOUNT",
                &format_money(total_amount, &app.settings.currency_symbol),
                "Earned in range",
                p.success,
            );
            kpi_card(
                &mut cols[3],
                p,
                "ENTRIES",
                &filtered.len().to_string(),
                "Logged sessions",
                p.accent,
            );
        });

        ui.add_space(14.0);

        // ── Daily Hours Bar Chart ─────────────────────────────────────────────
        card_frame(p).show(ui, |ui| {
            ui.label(RichText::new("Daily Hours").size(15.0).strong().color(p.text_primary));
            ui.add_space(8.0);

            let mut by_day: BTreeMap<NaiveDate, i64> = BTreeMap::new();
            for e in &filtered {
                let d = e.local_start().date_naive();
                *by_day.entry(d).or_insert(0) += e.duration_secs();
            }

            if by_day.is_empty() {
                ui.label(RichText::new("No tracked hours in this date range.").color(p.text_muted));
            } else {
                draw_daily_bar_chart(ui, p, &by_day);
            }
        });

        ui.add_space(14.0);

        // ── Project Breakdown ─────────────────────────────────────────────────
        card_frame(p).show(ui, |ui| {
            ui.label(RichText::new("Breakdown by Project").size(15.0).strong().color(p.text_primary));
            ui.add_space(8.0);

            if filtered.is_empty() {
                ui.label(RichText::new("No entries to summarize.").color(p.text_muted));
            } else {
                let mut by_proj: HashMap<Option<i64>, (i64, f64)> = HashMap::new();
                for e in &filtered {
                    let proj = e.project_id.and_then(|id| app.projects_map.get(&id));
                    let amt = e.billable_amount(proj, app.settings.default_hourly_rate);
                    let entry = by_proj.entry(e.project_id).or_insert((0, 0.0));
                    entry.0 += e.duration_secs();
                    entry.1 += amt;
                }

                let mut sorted: Vec<(Option<i64>, i64, f64)> = by_proj
                    .into_iter()
                    .map(|(pid, (secs, amt))| (pid, secs, amt))
                    .collect();
                sorted.sort_by(|a, b| b.1.cmp(&a.1));

                for (pid, secs, amt) in sorted {
                    let (name, col) = match pid.and_then(|id| app.projects_map.get(&id)) {
                        Some(pr) => (pr.name.clone(), parse_hex_color(&pr.color)),
                        None => ("No Project".to_string(), p.text_muted),
                    };
                    let pct = if total_secs > 0 {
                        (secs as f32 / total_secs as f32).clamp(0.0, 1.0)
                    } else {
                        0.0
                    };

                    ui.horizontal(|ui| {
                        color_dot(ui, col, 5.0);
                        ui.label(RichText::new(&name).strong().color(p.text_primary));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!("{:.0}%", pct * 100.0))
                                    .size(12.5)
                                    .color(p.text_muted),
                            );
                            ui.add_space(8.0);
                            if amt > 0.0 {
                                ui.label(
                                    RichText::new(format_money(amt, &app.settings.currency_symbol))
                                        .size(12.5)
                                        .color(p.success),
                                );
                                ui.add_space(8.0);
                            }
                            ui.label(
                                RichText::new(format_duration(secs))
                                    .monospace()
                                    .strong()
                                    .color(p.text_primary),
                            );
                        });
                    });

                    // Progress bar
                    let bar_width = ui.available_width();
                    let (rect, _) = ui.allocate_exact_size(Vec2::new(bar_width, 8.0), egui::Sense::hover());
                    ui.painter().rect_filled(rect, Rounding::same(4.0), p.bg_elevated);
                    let filled = Rect::from_min_size(rect.min, Vec2::new(rect.width() * pct, rect.height()));
                    ui.painter().rect_filled(filled, Rounding::same(4.0), col);
                    ui.add_space(8.0);
                }
            }
        });

        ui.add_space(14.0);

        // ── Export Section ────────────────────────────────────────────────────
        card_frame(p).show(ui, |ui| {
            ui.label(RichText::new("Export Report (Offline)").size(15.0).strong().color(p.text_primary));
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Folder:").size(12.5).color(p.text_muted));
                ui.add(TextEdit::singleline(&mut app.export_dir).desired_width(300.0));

                let stamp = Local::now().format("%Y-%m-%d_%H%M%S").to_string();

                if primary_button(ui, "Export CSV", p.accent).clicked() {
                    let path = PathBuf::from(&app.export_dir)
                        .join(format!("pro-clock-report-{}.csv", stamp));
                    match export_csv(
                        &path,
                        &filtered,
                        &app.projects_map,
                        &app.clients_map,
                        &app.tags_map,
                        &app.settings,
                    ) {
                        Ok(count) => app.set_status(
                            format!("Exported {} entries to {}", count, path.display()),
                            false,
                        ),
                        Err(e) => app.set_status(format!("CSV export failed: {}", e), true),
                    }
                }

                if primary_button(ui, "Export JSON", p.accent_blue).clicked() {
                    let path = PathBuf::from(&app.export_dir)
                        .join(format!("pro-clock-report-{}.json", stamp));
                    match export_json(
                        &path,
                        &filtered,
                        &app.projects_map,
                        &app.clients_map,
                        &app.tags_map,
                        &app.settings,
                    ) {
                        Ok(count) => app.set_status(
                            format!("Exported {} entries to {}", count, path.display()),
                            false,
                        ),
                        Err(e) => app.set_status(format!("JSON export failed: {}", e), true),
                    }
                }

                if ui.button("📂 Open Folder").clicked() {
                    let _ = std::process::Command::new("xdg-open")
                        .arg(&app.export_dir)
                        .spawn();
                }
            });
        });
    });
}

fn kpi_card(ui: &mut Ui, p: &Palette, title: &str, value: &str, subtitle: &str, val_color: Color32) {
    card_frame(p).show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.label(RichText::new(title).size(11.0).strong().color(p.text_muted));
        ui.add_space(4.0);
        ui.label(RichText::new(value).size(22.0).strong().monospace().color(val_color));
        ui.add_space(2.0);
        ui.label(RichText::new(subtitle).size(11.5).color(p.text_muted));
    });
}

fn draw_daily_bar_chart(ui: &mut Ui, p: &Palette, by_day: &BTreeMap<NaiveDate, i64>) {
    let height = 165.0;
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());
    let painter = ui.painter();

    let max_secs = by_day.values().copied().max().unwrap_or(3600).max(60) as f32;
    let count = by_day.len().max(1);
    let plot_top = rect.top() + 20.0;
    let plot_bottom = rect.bottom() - 24.0;
    let plot_height = (plot_bottom - plot_top).max(10.0);

    // Baseline
    painter.line_segment(
        [Pos2::new(rect.left(), plot_bottom), Pos2::new(rect.right(), plot_bottom)],
        Stroke::new(1.0_f32, p.border),
    );

    let slot_w = rect.width() / count as f32;
    let bar_w = (slot_w * 0.56).clamp(14.0, 56.0);

    for (i, (day, secs)) in by_day.iter().enumerate() {
        let ratio = (*secs as f32 / max_secs).clamp(0.02, 1.0);
        let bar_h = plot_height * ratio;
        let center_x = rect.left() + slot_w * (i as f32 + 0.5);
        let bar_rect = Rect::from_min_max(
            Pos2::new(center_x - bar_w * 0.5, plot_bottom - bar_h),
            Pos2::new(center_x + bar_w * 0.5, plot_bottom),
        );

        painter.rect_filled(
            bar_rect,
            Rounding {
                nw: 5.0,
                ne: 5.0,
                sw: 0.0,
                se: 0.0,
            },
            p.accent_blue,
        );

        // Hours label above bar
        let hrs_label = format!("{:.1}h", *secs as f64 / 3600.0);
        painter.text(
            Pos2::new(center_x, bar_rect.top() - 4.0),
            Align2::CENTER_BOTTOM,
            hrs_label,
            FontId::proportional(11.0),
            p.text_muted,
        );

        // Date label below bar
        let date_label = day.format("%m/%d").to_string();
        painter.text(
            Pos2::new(center_x, plot_bottom + 6.0),
            Align2::CENTER_TOP,
            date_label,
            FontId::proportional(11.0),
            p.text_muted,
        );
    }
}
