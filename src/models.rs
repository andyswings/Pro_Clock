use chrono::{DateTime, Duration, Local, Utc};
use egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub id: i64,
    pub name: String,
    pub notes: String,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub client_id: Option<i64>,
    pub color: String,
    pub billable_default: bool,
    pub hourly_rate: f64,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: i64,
    pub description: String,
    pub project_id: Option<i64>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub billable: bool,
    pub hourly_rate: Option<f64>,
    pub tag_ids: Vec<i64>,
    pub created_at: DateTime<Utc>,
}

impl TimeEntry {
    pub fn is_running(&self) -> bool {
        self.end_time.is_none()
    }

    pub fn duration(&self) -> Duration {
        let end = self.end_time.unwrap_or_else(Utc::now);
        let diff = end.signed_duration_since(self.start_time);
        if diff < Duration::zero() {
            Duration::zero()
        } else {
            diff
        }
    }

    pub fn duration_secs(&self) -> i64 {
        self.duration().num_seconds()
    }

    pub fn effective_rate(&self, project: Option<&Project>, default_rate: f64) -> f64 {
        if let Some(rate) = self.hourly_rate {
            return rate;
        }
        if let Some(proj) = project {
            if proj.hourly_rate > 0.0 {
                return proj.hourly_rate;
            }
        }
        default_rate
    }

    pub fn billable_amount(&self, project: Option<&Project>, default_rate: f64) -> f64 {
        if !self.billable {
            return 0.0;
        }
        let hours = self.duration_secs() as f64 / 3600.0;
        hours * self.effective_rate(project, default_rate)
    }

    pub fn local_start(&self) -> DateTime<Local> {
        self.start_time.with_timezone(&Local)
    }

    pub fn local_end(&self) -> Option<DateTime<Local>> {
        self.end_time.map(|t| t.with_timezone(&Local))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub dark_mode: bool,
    pub currency_symbol: String,
    pub default_hourly_rate: f64,
    pub week_starts_monday: bool,
    pub notifications_enabled: bool,
    pub pomodoro_focus_mins: u32,
    pub pomodoro_short_break_mins: u32,
    pub pomodoro_long_break_mins: u32,
    pub pomodoro_cycles_before_long: u32,
    pub pomodoro_auto_log: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            dark_mode: true,
            currency_symbol: "$".to_string(),
            default_hourly_rate: 0.0,
            week_starts_monday: true,
            notifications_enabled: true,
            pomodoro_focus_mins: 25,
            pomodoro_short_break_mins: 5,
            pomodoro_long_break_mins: 15,
            pomodoro_cycles_before_long: 4,
            pomodoro_auto_log: true,
        }
    }
}

pub const PRESET_COLORS: &[(&str, &str)] = &[
    ("Ubuntu Orange", "#E95420"),
    ("Ocean Blue", "#03A9F4"),
    ("Emerald", "#2ECC71"),
    ("Amethyst", "#9B59B6"),
    ("Amber", "#F39C12"),
    ("Crimson", "#E74C3C"),
    ("Teal", "#1ABC9C"),
    ("Indigo", "#5C6BC0"),
    ("Rose", "#EC407A"),
    ("Slate", "#607D8B"),
];

pub fn parse_hex_color(hex: &str) -> Color32 {
    let clean = hex.trim().trim_start_matches('#');
    if clean.len() == 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&clean[0..2], 16),
            u8::from_str_radix(&clean[2..4], 16),
            u8::from_str_radix(&clean[4..6], 16),
        ) {
            return Color32::from_rgb(r, g, b);
        }
    }
    Color32::from_rgb(3, 169, 244)
}

pub fn format_duration(total_secs: i64) -> String {
    let secs = total_secs.max(0);
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

pub fn format_hours_decimal(total_secs: i64) -> String {
    let hours = (total_secs.max(0) as f64) / 3600.0;
    format!("{:.2}h", hours)
}

pub fn format_money(amount: f64, currency: &str) -> String {
    format!("{}{:.2}", currency, amount)
}

pub fn format_duration_hm(total_secs: i64) -> String {
    let secs = total_secs.max(0);
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    format!("{:02}:{:02}", h, m)
}

/// Parses flexible duration inputs like "1:30", "1.5", "1.5h", "45m", or "2h 15m" into seconds.
pub fn parse_duration_input(input: &str) -> Option<i64> {
    let s = input.trim().to_lowercase();
    if s.is_empty() {
        return None;
    }

    // "HH:MM" or "HH:MM:SS"
    if s.contains(':') {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() == 2 {
            let h: i64 = parts[0].trim().parse().ok()?;
            let m: i64 = parts[1].trim().parse().ok()?;
            if h >= 0 && m >= 0 {
                return Some(h * 3600 + m * 60);
            }
        } else if parts.len() == 3 {
            let h: i64 = parts[0].trim().parse().ok()?;
            let m: i64 = parts[1].trim().parse().ok()?;
            let sec: i64 = parts[2].trim().parse().ok()?;
            if h >= 0 && m >= 0 && sec >= 0 {
                return Some(h * 3600 + m * 60 + sec);
            }
        }
        return None;
    }

    // "45m"
    if let Some(mins_str) = s.strip_suffix('m') {
        if let Ok(mins) = mins_str.trim().parse::<f64>() {
            if mins > 0.0 {
                return Some((mins * 60.0).round() as i64);
            }
        }
    }

    // "1.5h"
    if let Some(hrs_str) = s.strip_suffix('h') {
        if let Ok(hrs) = hrs_str.trim().parse::<f64>() {
            if hrs > 0.0 {
                return Some((hrs * 3600.0).round() as i64);
            }
        }
    }

    // Plain decimal hours like "1.5" or "2"
    if let Ok(hrs) = s.parse::<f64>() {
        if hrs > 0.0 {
            return Some((hrs * 3600.0).round() as i64);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration_input() {
        assert_eq!(parse_duration_input("1:30"), Some(5400));
        assert_eq!(parse_duration_input("0:45"), Some(2700));
        assert_eq!(parse_duration_input("1.5"), Some(5400));
        assert_eq!(parse_duration_input("2h"), Some(7200));
        assert_eq!(parse_duration_input("45m"), Some(2700));
        assert_eq!(parse_duration_input("0"), None);
        assert_eq!(parse_duration_input("abc"), None);
    }
}
