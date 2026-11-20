#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod export;
mod models;
mod notify;
mod theme;
mod ui;

use db::Database;
use egui::IconData;
use std::sync::Arc;
use ui::ProClockApp;

fn generate_app_icon() -> IconData {
    let size = 64usize;
    let mut rgba = vec![0u8; size * size * 4];
    let cx = (size as f32) / 2.0;
    let cy = (size as f32) / 2.0;
    let radius = 29.0f32;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 + 0.5 - cx;
            let dy = y as f32 + 0.5 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            let idx = (y * size + x) * 4;

            if dist <= radius {
                // Smooth anti-aliased edge
                let alpha = ((radius - dist).clamp(0.0, 1.0) * 255.0) as u8;
                // Ubuntu orange background (#E95420)
                let mut r = 233u8;
                let mut g = 84u8;
                let mut b = 32u8;

                // Subtle white outer clock ring
                if dist >= 22.5 && dist <= 25.5 {
                    r = 255;
                    g = 255;
                    b = 255;
                }

                // Minute hand pointing up (12 o'clock)
                if dx.abs() <= 2.2 && dy <= 1.0 && dy >= -15.5 {
                    r = 255;
                    g = 255;
                    b = 255;
                }

                // Hour hand pointing to ~4 o'clock
                let dot_h = dx * 0.866 + dy * 0.5;
                let perp_h = (-dx * 0.5 + dy * 0.866).abs();
                if perp_h <= 2.2 && dot_h >= -1.0 && dot_h <= 12.0 {
                    r = 255;
                    g = 255;
                    b = 255;
                }

                // Center pivot
                if dist <= 3.2 {
                    r = 255;
                    g = 255;
                    b = 255;
                }

                rgba[idx] = r;
                rgba[idx + 1] = g;
                rgba[idx + 2] = b;
                rgba[idx + 3] = alpha;
            }
        }
    }

    IconData {
        rgba,
        width: size as u32,
        height: size as u32,
    }
}

fn main() -> eframe::Result<()> {
    let db = Database::open_default().expect("Failed to initialize local Pro Clock database");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Pro Clock — Offline Time Tracker")
            .with_app_id("pro-clock")
            .with_inner_size([1120.0, 740.0])
            .with_min_inner_size([840.0, 560.0])
            .with_icon(Arc::new(generate_app_icon())),
        ..Default::default()
    };

    eframe::run_native(
        "Pro Clock",
        native_options,
        Box::new(move |cc| Ok(Box::new(ProClockApp::new(cc, db)))),
    )
}
