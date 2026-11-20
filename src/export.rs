use crate::models::{format_duration, AppSettings, Client, Project, Tag, TimeEntry};
use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Serialize)]
struct ExportRow {
    project: String,
    client: String,
    description: String,
    tags: String,
    billable: String,
    start_date: String,
    start_time: String,
    end_date: String,
    end_time: String,
    duration_hms: String,
    duration_decimal: f64,
    hourly_rate: f64,
    billable_amount: f64,
}

fn build_rows(
    entries: &[TimeEntry],
    projects: &HashMap<i64, Project>,
    clients: &HashMap<i64, Client>,
    tags: &HashMap<i64, Tag>,
    settings: &AppSettings,
) -> Vec<ExportRow> {
    entries
        .iter()
        .map(|entry| {
            let project = entry.project_id.and_then(|id| projects.get(&id));
            let client = project
                .and_then(|p| p.client_id)
                .and_then(|cid| clients.get(&cid));
            let tag_names: Vec<String> = entry
                .tag_ids
                .iter()
                .filter_map(|tid| tags.get(tid).map(|t| t.name.clone()))
                .collect();

            let local_start = entry.local_start();
            let local_end = entry.local_end();
            let secs = entry.duration_secs();
            let decimal_hours = (secs as f64 / 3600.0 * 100.0).round() / 100.0;
            let rate = entry.effective_rate(project, settings.default_hourly_rate);
            let amount = (entry.billable_amount(project, settings.default_hourly_rate) * 100.0).round() / 100.0;

            ExportRow {
                project: project.map(|p| p.name.clone()).unwrap_or_default(),
                client: client.map(|c| c.name.clone()).unwrap_or_default(),
                description: entry.description.clone(),
                tags: tag_names.join(", "),
                billable: if entry.billable { "Yes".into() } else { "No".into() },
                start_date: local_start.format("%Y-%m-%d").to_string(),
                start_time: local_start.format("%H:%M:%S").to_string(),
                end_date: local_end
                    .map(|t| t.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "Running".to_string()),
                end_time: local_end
                    .map(|t| t.format("%H:%M:%S").to_string())
                    .unwrap_or_else(|| "Running".to_string()),
                duration_hms: format_duration(secs),
                duration_decimal: decimal_hours,
                hourly_rate: rate,
                billable_amount: amount,
            }
        })
        .collect()
}

pub fn export_csv(
    path: &Path,
    entries: &[TimeEntry],
    projects: &HashMap<i64, Project>,
    clients: &HashMap<i64, Client>,
    tags: &HashMap<i64, Tag>,
    settings: &AppSettings,
) -> Result<usize> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let rows = build_rows(entries, projects, clients, tags, settings);
    let mut wtr = csv::Writer::from_path(path)?;
    wtr.write_record([
        "Project",
        "Client",
        "Description",
        "Tags",
        "Billable",
        "Start Date",
        "Start Time",
        "End Date",
        "End Time",
        "Duration (h)",
        "Duration (decimal)",
        "Billable Rate",
        "Billable Amount",
    ])?;
    for row in &rows {
        wtr.write_record([
            &row.project,
            &row.client,
            &row.description,
            &row.tags,
            &row.billable,
            &row.start_date,
            &row.start_time,
            &row.end_date,
            &row.end_time,
            &row.duration_hms,
            &format!("{:.2}", row.duration_decimal),
            &format!("{:.2}", row.hourly_rate),
            &format!("{:.2}", row.billable_amount),
        ])?;
    }
    wtr.flush()?;
    Ok(rows.len())
}

pub fn export_json(
    path: &Path,
    entries: &[TimeEntry],
    projects: &HashMap<i64, Project>,
    clients: &HashMap<i64, Client>,
    tags: &HashMap<i64, Tag>,
    settings: &AppSettings,
) -> Result<usize> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let rows = build_rows(entries, projects, clients, tags, settings);
    let json = serde_json::to_string_pretty(&rows)?;
    std::fs::write(path, json)?;
    Ok(rows.len())
}
