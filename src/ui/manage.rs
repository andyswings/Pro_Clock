use crate::models::{format_money, parse_hex_color, PRESET_COLORS};
use crate::notify::{open_folder, send_notification};
use crate::theme::{apply_theme, card_frame, color_dot, pill_badge, primary_button, Palette};
use crate::ui::ProClockApp;
use chrono::Local;
use egui::{ComboBox, Context, RichText, Rounding, ScrollArea, Stroke, TextEdit, Ui, Vec2};
use std::path::PathBuf;

// ─── Projects View ────────────────────────────────────────────────────────────

pub fn render_projects_view(app: &mut ProClockApp, ui: &mut Ui, p: &Palette) {
    ui.horizontal(|ui| {
        ui.heading(RichText::new("Projects").color(p.text_primary).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.checkbox(&mut app.proj_show_archived, "Show archived");
        });
    });
    ui.add_space(10.0);

    // Create / Edit Project Card
    card_frame(p).show(ui, |ui| {
        let title = if app.proj_edit_id.is_some() {
            "Edit Project"
        } else {
            "New Project"
        };
        ui.label(RichText::new(title).size(14.5).strong().color(p.text_primary));
        ui.add_space(6.0);

        ui.horizontal_wrapped(|ui| {
            ui.add(
                TextEdit::singleline(&mut app.proj_name)
                    .hint_text("Project name")
                    .desired_width(210.0),
            );

            let client_label = app
                .proj_client_id
                .and_then(|id| app.clients_map.get(&id))
                .map(|c| c.name.clone())
                .unwrap_or_else(|| "No Client".to_string());
            ComboBox::from_id_salt("proj_form_client")
                .selected_text(client_label)
                .width(150.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.proj_client_id, None, "No Client");
                    for client in app.clients.iter().filter(|c| !c.archived) {
                        ui.selectable_value(&mut app.proj_client_id, Some(client.id), &client.name);
                    }
                });

            ui.label(
                RichText::new(format!("Rate ({}/h):", app.settings.currency_symbol))
                    .size(12.5)
                    .color(p.text_muted),
            );
            ui.add(TextEdit::singleline(&mut app.proj_hourly_rate).desired_width(75.0));

            ui.checkbox(&mut app.proj_billable_default, "Billable by default");
        });

        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("Color:").size(12.5).color(p.text_muted));
            color_picker_row(ui, &mut app.proj_color, p);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let btn_label = if app.proj_edit_id.is_some() {
                    "Save Project"
                } else {
                    "+ Add Project"
                };
                if primary_button(ui, btn_label, p.accent).clicked() {
                    if app.proj_name.trim().is_empty() {
                        app.set_status("Project name cannot be empty.", true);
                    } else {
                        let rate = app.proj_hourly_rate.trim().parse::<f64>().unwrap_or(0.0);
                        let res = if let Some(edit_id) = app.proj_edit_id {
                            let archived = app
                                .projects_map
                                .get(&edit_id)
                                .map(|pr| pr.archived)
                                .unwrap_or(false);
                            app.db.update_project(
                                edit_id,
                                &app.proj_name,
                                app.proj_client_id,
                                &app.proj_color,
                                app.proj_billable_default,
                                rate,
                                archived,
                            )
                        } else {
                            app.db
                                .create_project(
                                    &app.proj_name,
                                    app.proj_client_id,
                                    &app.proj_color,
                                    app.proj_billable_default,
                                    rate,
                                )
                                .map(|_| ())
                        };
                        match res {
                            Ok(()) => {
                                let was_edit = app.proj_edit_id.is_some();
                                app.proj_edit_id = None;
                                app.proj_name.clear();
                                app.proj_client_id = None;
                                app.proj_hourly_rate = "0.00".to_string();
                                app.reload_data();
                                app.set_status(
                                    if was_edit { "Project updated." } else { "Project created." },
                                    false,
                                );
                            }
                            Err(e) => app.set_status(format!("Error saving project: {}", e), true),
                        }
                    }
                }
                if app.proj_edit_id.is_some() && ui.button("Cancel").clicked() {
                    app.proj_edit_id = None;
                    app.proj_name.clear();
                    app.proj_client_id = None;
                    app.proj_hourly_rate = "0.00".to_string();
                }
            });
        });
    });

    ui.add_space(12.0);

    // Projects List
    let projects: Vec<_> = app
        .projects
        .iter()
        .filter(|pr| app.proj_show_archived || !pr.archived)
        .cloned()
        .collect();

    if projects.is_empty() {
        ui.label(RichText::new("No projects yet. Create one above!").color(p.text_muted));
        return;
    }

    ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
        card_frame(p).show(ui, |ui| {
            for (i, proj) in projects.iter().enumerate() {
                if i > 0 {
                    ui.separator();
                }
                ui.horizontal(|ui| {
                    ui.set_min_height(34.0);
                    color_dot(ui, parse_hex_color(&proj.color), 5.5);
                    let name_text = if proj.archived {
                        RichText::new(format!("{} (Archived)", proj.name))
                            .strikethrough()
                            .color(p.text_muted)
                    } else {
                        RichText::new(&proj.name).strong().color(p.text_primary)
                    };
                    ui.label(name_text);

                    if let Some(cid) = proj.client_id {
                        if let Some(c) = app.clients_map.get(&cid) {
                            ui.label(RichText::new(format!("• {}", c.name)).size(12.5).color(p.text_muted));
                        }
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .small_button(RichText::new("🗑").color(p.danger))
                            .on_hover_text("Delete project")
                            .clicked()
                        {
                            if let Err(e) = app.db.delete_project(proj.id) {
                                app.set_status(format!("Failed to delete project: {}", e), true);
                            } else {
                                app.reload_data();
                                app.set_status("Project deleted.", false);
                            }
                        }

                        let arch_label = if proj.archived { "Restore" } else { "Archive" };
                        if ui.small_button(arch_label).clicked() {
                            if let Err(e) = app.db.update_project(
                                proj.id,
                                &proj.name,
                                proj.client_id,
                                &proj.color,
                                proj.billable_default,
                                proj.hourly_rate,
                                !proj.archived,
                            ) {
                                app.set_status(format!("Error: {}", e), true);
                            } else {
                                app.reload_data();
                            }
                        }

                        if ui.small_button("✏ Edit").clicked() {
                            app.proj_edit_id = Some(proj.id);
                            app.proj_name = proj.name.clone();
                            app.proj_client_id = proj.client_id;
                            app.proj_color = proj.color.clone();
                            app.proj_billable_default = proj.billable_default;
                            app.proj_hourly_rate = format!("{:.2}", proj.hourly_rate);
                        }

                        ui.add_space(10.0);
                        if proj.hourly_rate > 0.0 {
                            ui.label(
                                RichText::new(format!(
                                    "{}/h",
                                    format_money(proj.hourly_rate, &app.settings.currency_symbol)
                                ))
                                .size(12.5)
                                .color(p.success),
                            );
                        }
                        if proj.billable_default {
                            pill_badge(ui, "Billable", p.accent_blue);
                        }
                    });
                });
            }
        });
    });
}

// ─── Clients View ─────────────────────────────────────────────────────────────

pub fn render_clients_view(app: &mut ProClockApp, ui: &mut Ui, p: &Palette) {
    ui.horizontal(|ui| {
        ui.heading(RichText::new("Clients").color(p.text_primary).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.checkbox(&mut app.client_show_archived, "Show archived");
        });
    });
    ui.add_space(10.0);

    card_frame(p).show(ui, |ui| {
        let title = if app.client_edit_id.is_some() {
            "Edit Client"
        } else {
            "New Client"
        };
        ui.label(RichText::new(title).size(14.5).strong().color(p.text_primary));
        ui.add_space(6.0);

        ui.horizontal_wrapped(|ui| {
            ui.add(
                TextEdit::singleline(&mut app.client_name)
                    .hint_text("Client name")
                    .desired_width(210.0),
            );
            ui.add(
                TextEdit::singleline(&mut app.client_notes)
                    .hint_text("Notes (optional)")
                    .desired_width(260.0),
            );

            let btn_label = if app.client_edit_id.is_some() {
                "Save Client"
            } else {
                "+ Add Client"
            };
            if primary_button(ui, btn_label, p.accent).clicked() {
                if app.client_name.trim().is_empty() {
                    app.set_status("Client name cannot be empty.", true);
                } else {
                    let res = if let Some(edit_id) = app.client_edit_id {
                        let archived = app
                            .clients_map
                            .get(&edit_id)
                            .map(|c| c.archived)
                            .unwrap_or(false);
                        app.db
                            .update_client(edit_id, &app.client_name, &app.client_notes, archived)
                    } else {
                        app.db
                            .create_client(&app.client_name, &app.client_notes)
                            .map(|_| ())
                    };
                    match res {
                        Ok(()) => {
                            let was_edit = app.client_edit_id.is_some();
                            app.client_edit_id = None;
                            app.client_name.clear();
                            app.client_notes.clear();
                            app.reload_data();
                            app.set_status(
                                if was_edit { "Client updated." } else { "Client created." },
                                false,
                            );
                        }
                        Err(e) => app.set_status(format!("Error saving client: {}", e), true),
                    }
                }
            }
            if app.client_edit_id.is_some() && ui.button("Cancel").clicked() {
                app.client_edit_id = None;
                app.client_name.clear();
                app.client_notes.clear();
            }
        });
    });

    ui.add_space(12.0);

    let clients: Vec<_> = app
        .clients
        .iter()
        .filter(|c| app.client_show_archived || !c.archived)
        .cloned()
        .collect();

    if clients.is_empty() {
        ui.label(RichText::new("No clients yet. Add your first client above!").color(p.text_muted));
        return;
    }

    ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
        card_frame(p).show(ui, |ui| {
            for (i, client) in clients.iter().enumerate() {
                if i > 0 {
                    ui.separator();
                }
                ui.horizontal(|ui| {
                    ui.set_min_height(34.0);
                    let label = if client.archived {
                        RichText::new(format!("{} (Archived)", client.name))
                            .strikethrough()
                            .color(p.text_muted)
                    } else {
                        RichText::new(&client.name).strong().color(p.text_primary)
                    };
                    ui.label(label);
                    if !client.notes.is_empty() {
                        ui.label(RichText::new(format!("— {}", client.notes)).size(12.5).color(p.text_muted));
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .small_button(RichText::new("🗑").color(p.danger))
                            .on_hover_text("Delete client")
                            .clicked()
                        {
                            if let Err(e) = app.db.delete_client(client.id) {
                                app.set_status(format!("Failed to delete client: {}", e), true);
                            } else {
                                app.reload_data();
                                app.set_status("Client deleted.", false);
                            }
                        }
                        let arch_label = if client.archived { "Restore" } else { "Archive" };
                        if ui.small_button(arch_label).clicked() {
                            let _ = app.db.update_client(
                                client.id,
                                &client.name,
                                &client.notes,
                                !client.archived,
                            );
                            app.reload_data();
                        }
                        if ui.small_button("✏ Edit").clicked() {
                            app.client_edit_id = Some(client.id);
                            app.client_name = client.name.clone();
                            app.client_notes = client.notes.clone();
                        }
                    });
                });
            }
        });
    });
}

// ─── Tags View ────────────────────────────────────────────────────────────────

pub fn render_tags_view(app: &mut ProClockApp, ui: &mut Ui, p: &Palette) {
    ui.heading(RichText::new("Tags").color(p.text_primary).strong());
    ui.add_space(10.0);

    card_frame(p).show(ui, |ui| {
        let title = if app.tag_edit_id.is_some() { "Edit Tag" } else { "New Tag" };
        ui.label(RichText::new(title).size(14.5).strong().color(p.text_primary));
        ui.add_space(6.0);

        ui.horizontal_wrapped(|ui| {
            ui.add(
                TextEdit::singleline(&mut app.tag_name)
                    .hint_text("Tag name (e.g. Urgent, Meeting, Deep Work)")
                    .desired_width(260.0),
            );
            color_picker_row(ui, &mut app.tag_color, p);

            let btn_label = if app.tag_edit_id.is_some() { "Save Tag" } else { "+ Add Tag" };
            if primary_button(ui, btn_label, p.accent).clicked() {
                if app.tag_name.trim().is_empty() {
                    app.set_status("Tag name cannot be empty.", true);
                } else {
                    let res = if let Some(edit_id) = app.tag_edit_id {
                        app.db.update_tag(edit_id, &app.tag_name, &app.tag_color)
                    } else {
                        app.db.create_tag(&app.tag_name, &app.tag_color).map(|_| ())
                    };
                    match res {
                        Ok(()) => {
                            let was_edit = app.tag_edit_id.is_some();
                            app.tag_edit_id = None;
                            app.tag_name.clear();
                            app.reload_data();
                            app.set_status(
                                if was_edit { "Tag updated." } else { "Tag created." },
                                false,
                            );
                        }
                        Err(e) => app.set_status(format!("Error saving tag: {}", e), true),
                    }
                }
            }
            if app.tag_edit_id.is_some() && ui.button("Cancel").clicked() {
                app.tag_edit_id = None;
                app.tag_name.clear();
            }
        });
    });

    ui.add_space(12.0);

    let tags = app.tags.clone();
    if tags.is_empty() {
        ui.label(RichText::new("No tags yet. Create tags to organize your time entries!").color(p.text_muted));
        return;
    }

    ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
        card_frame(p).show(ui, |ui| {
            for (i, tag) in tags.iter().enumerate() {
                if i > 0 {
                    ui.separator();
                }
                ui.horizontal(|ui| {
                    ui.set_min_height(32.0);
                    pill_badge(ui, &tag.name, parse_hex_color(&tag.color));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .small_button(RichText::new("🗑").color(p.danger))
                            .on_hover_text("Delete tag")
                            .clicked()
                        {
                            if let Err(e) = app.db.delete_tag(tag.id) {
                                app.set_status(format!("Failed to delete tag: {}", e), true);
                            } else {
                                app.reload_data();
                                app.set_status("Tag deleted.", false);
                            }
                        }
                        if ui.small_button("✏ Edit").clicked() {
                            app.tag_edit_id = Some(tag.id);
                            app.tag_name = tag.name.clone();
                            app.tag_color = tag.color.clone();
                        }
                    });
                });
            }
        });
    });
}

// ─── Settings View ────────────────────────────────────────────────────────────

pub fn render_settings_view(app: &mut ProClockApp, ctx: &Context, ui: &mut Ui, p: &Palette) {
    ui.heading(RichText::new("Settings").color(p.text_primary).strong());
    ui.add_space(10.0);

    ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
        card_frame(p).show(ui, |ui| {
            ui.label(RichText::new("Appearance & Workspace").size(15.0).strong().color(p.text_primary));
            ui.add_space(8.0);

            let mut dark = app.settings.dark_mode;
            if ui.checkbox(&mut dark, "Dark Mode (Ubuntu Yaru Dark)").changed() {
                app.settings.dark_mode = dark;
                let _ = app.db.save_settings(&app.settings);
                apply_theme(ctx, dark);
            }

            ui.add_space(6.0);
            let mut week_mon = app.settings.week_starts_monday;
            if ui.checkbox(&mut week_mon, "Week starts on Monday").changed() {
                app.settings.week_starts_monday = week_mon;
                let _ = app.db.save_settings(&app.settings);
            }

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Currency Symbol:").color(p.text_muted));
                ui.add(TextEdit::singleline(&mut app.settings.currency_symbol).desired_width(60.0));

                ui.add_space(16.0);
                ui.label(RichText::new("Default Hourly Rate:").color(p.text_muted));
                ui.add(TextEdit::singleline(&mut app.settings_rate_str).desired_width(90.0));
            });

            ui.add_space(12.0);
            if primary_button(ui, "Save Settings", p.accent).clicked() {
                if let Ok(rate) = app.settings_rate_str.trim().parse::<f64>() {
                    app.settings.default_hourly_rate = rate.max(0.0);
                    if app.settings.currency_symbol.trim().is_empty() {
                        app.settings.currency_symbol = "$".to_string();
                    }
                    match app.db.save_settings(&app.settings) {
                        Ok(()) => app.set_status("Settings saved.", false),
                        Err(e) => app.set_status(format!("Failed to save settings: {}", e), true),
                    }
                } else {
                    app.set_status("Invalid default hourly rate.", true);
                }
            }
        });

        ui.add_space(14.0);

        // ── Pomodoro & Desktop Notifications ──────────────────────────────────
        card_frame(p).show(ui, |ui| {
            ui.label(
                RichText::new("Pomodoro & Desktop Notifications")
                    .size(15.0)
                    .strong()
                    .color(p.text_primary),
            );
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                if ui
                    .checkbox(
                        &mut app.settings.notifications_enabled,
                        "Enable desktop notifications",
                    )
                    .changed()
                {
                    let _ = app.db.save_settings(&app.settings);
                }

                ui.add_space(12.0);
                if ui.button("🔔 Send Test Notification").clicked() {
                    send_notification(
                        true,
                        "Pro Clock",
                        "Desktop notifications are working!",
                    );
                    app.set_status("Sent test desktop notification.", false);
                }
            });

            ui.add_space(8.0);
            if ui
                .checkbox(
                    &mut app.settings.pomodoro_auto_log,
                    "Automatically log completed Pomodoro focus sessions to Time Tracker",
                )
                .changed()
            {
                let _ = app.db.save_settings(&app.settings);
            }

            ui.add_space(10.0);
            let mut pomo_changed = false;
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Focus (min):").color(p.text_muted));
                pomo_changed |= ui
                    .add(egui::DragValue::new(&mut app.settings.pomodoro_focus_mins).range(1..=180))
                    .changed();

                ui.add_space(12.0);
                ui.label(RichText::new("Short Break (min):").color(p.text_muted));
                pomo_changed |= ui
                    .add(
                        egui::DragValue::new(&mut app.settings.pomodoro_short_break_mins)
                            .range(1..=60),
                    )
                    .changed();

                ui.add_space(12.0);
                ui.label(RichText::new("Long Break (min):").color(p.text_muted));
                pomo_changed |= ui
                    .add(
                        egui::DragValue::new(&mut app.settings.pomodoro_long_break_mins)
                            .range(1..=90),
                    )
                    .changed();

                ui.add_space(12.0);
                ui.label(RichText::new("Cycles before Long Break:").color(p.text_muted));
                pomo_changed |= ui
                    .add(
                        egui::DragValue::new(&mut app.settings.pomodoro_cycles_before_long)
                            .range(1..=12),
                    )
                    .changed();
            });

            if pomo_changed {
                let _ = app.db.save_settings(&app.settings);
                if !app.pomodoro.running {
                    let phase = app.pomodoro.phase;
                    app.pomodoro.switch_phase(phase, &app.settings);
                }
            }
        });

        ui.add_space(14.0);

        card_frame(p).show(ui, |ui| {
            ui.label(
                RichText::new("Offline Database & Backup")
                    .size(15.0)
                    .strong()
                    .color(p.text_primary),
            );
            ui.add_space(6.0);
            let db_path = app.db.path().display().to_string();
            ui.label(
                RichText::new(
                    "All your data is stored 100% locally in a single SQLite file — no cloud, no account, no telemetry.",
                )
                .size(12.5)
                .color(p.text_muted),
            );
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Database file:").strong().color(p.text_primary));
                ui.label(RichText::new(&db_path).monospace().size(12.5).color(p.accent_blue));
            });
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if primary_button(ui, "Create Backup Copy", p.accent_blue).clicked() {
                    let stamp = Local::now().format("%Y-%m-%d_%H%M%S").to_string();
                    let backup_path = PathBuf::from(&app.export_dir)
                        .join(format!("pro-clock-backup-{}.db", stamp));
                    match std::fs::copy(app.db.path(), &backup_path) {
                        Ok(_) => app.set_status(
                            format!("Database backed up to {}", backup_path.display()),
                            false,
                        ),
                        Err(e) => app.set_status(format!("Backup failed: {}", e), true),
                    }
                }
                if ui.button("📂 Open Data Folder").clicked() {
                    if let Some(parent) = app.db.path().parent() {
                        open_folder(parent);
                    }
                }
            });
        });
    });
}

fn color_picker_row(ui: &mut Ui, selected_hex: &mut String, p: &Palette) {
    for (name, hex) in PRESET_COLORS {
        let col = parse_hex_color(hex);
        let is_selected = selected_hex.eq_ignore_ascii_case(hex);
        let (rect, resp) = ui.allocate_exact_size(Vec2::splat(22.0), egui::Sense::click());
        ui.painter().circle_filled(rect.center(), 8.5, col);
        if is_selected {
            ui.painter().circle_stroke(rect.center(), 10.5, Stroke::new(2.0_f32, p.text_primary));
        } else if resp.hovered() {
            ui.painter().circle_stroke(rect.center(), 10.0, Stroke::new(1.5_f32, p.text_muted));
        }
        if resp.on_hover_text(*name).clicked() {
            *selected_hex = (*hex).to_string();
        }
    }
    let _ = Rounding::ZERO;
}
