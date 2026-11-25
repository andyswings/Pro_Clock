use crate::models::{AppSettings, AppTheme, Client, Project, Tag, TimeEntry};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use directories::ProjectDirs;
use rusqlite::{params, Connection};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct Database {
    conn: Connection,
    path: PathBuf,
}

impl Database {
    pub fn open_default() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("org", "proclock", "pro-clock")
            .context("Could not determine XDG data directory")?;
        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(data_dir)
            .with_context(|| format!("Failed to create data directory {:?}", data_dir))?;
        let db_path = data_dir.join("pro_clock.db");
        Self::open(&db_path)
    }

    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)
            .with_context(|| format!("Failed to open SQLite database at {:?}", path))?;
        let db = Self {
            conn,
            path: path.to_path_buf(),
        };
        db.init_schema()?;
        Ok(db)
    }

    #[cfg(test)]
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self {
            conn,
            path: PathBuf::from(":memory:"),
        };
        db.init_schema()?;
        Ok(db)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;

            CREATE TABLE IF NOT EXISTS clients (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                notes TEXT NOT NULL DEFAULT '',
                archived INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                client_id INTEGER REFERENCES clients(id) ON DELETE SET NULL,
                color TEXT NOT NULL DEFAULT '#03A9F4',
                billable_default INTEGER NOT NULL DEFAULT 1,
                hourly_rate REAL NOT NULL DEFAULT 0.0,
                archived INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                color TEXT NOT NULL DEFAULT '#607D8B',
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS time_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL DEFAULT '',
                project_id INTEGER REFERENCES projects(id) ON DELETE SET NULL,
                start_time TEXT NOT NULL,
                end_time TEXT,
                billable INTEGER NOT NULL DEFAULT 0,
                hourly_rate REAL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS time_entry_tags (
                time_entry_id INTEGER NOT NULL REFERENCES time_entries(id) ON DELETE CASCADE,
                tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
                PRIMARY KEY (time_entry_id, tag_id)
            );

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_time_entries_start ON time_entries(start_time DESC);
            CREATE INDEX IF NOT EXISTS idx_time_entries_project ON time_entries(project_id);
            ",
        )?;
        Ok(())
    }

    // ─── Settings ─────────────────────────────────────────────────────────────

    pub fn load_settings(&self) -> Result<AppSettings> {
        let mut settings = AppSettings::default();
        let mut saved_theme: Option<AppTheme> = None;
        let mut legacy_dark_mode: Option<bool> = None;

        let mut stmt = self.conn.prepare("SELECT key, value FROM settings")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        for row in rows {
            let (k, v) = row?;
            match k.as_str() {
                "theme" => {
                    if let Some(t) = AppTheme::from_key(&v) {
                        saved_theme = Some(t);
                    }
                }
                "dark_mode" => legacy_dark_mode = Some(v == "true"),
                "currency_symbol" => settings.currency_symbol = v,
                "default_hourly_rate" => {
                    if let Ok(r) = v.parse::<f64>() {
                        settings.default_hourly_rate = r;
                    }
                }
                "week_starts_monday" => settings.week_starts_monday = v == "true",
                "notifications_enabled" => settings.notifications_enabled = v == "true",
                "pomodoro_focus_mins" => {
                    if let Ok(m) = v.parse::<u32>() {
                        settings.pomodoro_focus_mins = m.clamp(1, 180);
                    }
                }
                "pomodoro_short_break_mins" => {
                    if let Ok(m) = v.parse::<u32>() {
                        settings.pomodoro_short_break_mins = m.clamp(1, 60);
                    }
                }
                "pomodoro_long_break_mins" => {
                    if let Ok(m) = v.parse::<u32>() {
                        settings.pomodoro_long_break_mins = m.clamp(1, 90);
                    }
                }
                "pomodoro_cycles_before_long" => {
                    if let Ok(c) = v.parse::<u32>() {
                        settings.pomodoro_cycles_before_long = c.clamp(1, 12);
                    }
                }
                "pomodoro_auto_log" => settings.pomodoro_auto_log = v == "true",
                _ => {}
            }
        }

        if let Some(theme) = saved_theme {
            settings.theme = theme;
        } else if let Some(dark) = legacy_dark_mode {
            settings.theme = AppTheme::default_for_os().with_dark(dark);
        }

        Ok(settings)
    }

    pub fn save_settings(&self, settings: &AppSettings) -> Result<()> {
        let pairs = [
            ("theme", settings.theme.as_key().to_string()),
            (
                "dark_mode",
                if settings.theme.is_dark() { "true" } else { "false" }.to_string(),
            ),
            ("currency_symbol", settings.currency_symbol.clone()),
            ("default_hourly_rate", format!("{:.2}", settings.default_hourly_rate)),
            (
                "week_starts_monday",
                if settings.week_starts_monday { "true" } else { "false" }.to_string(),
            ),
            (
                "notifications_enabled",
                if settings.notifications_enabled { "true" } else { "false" }.to_string(),
            ),
            ("pomodoro_focus_mins", settings.pomodoro_focus_mins.to_string()),
            (
                "pomodoro_short_break_mins",
                settings.pomodoro_short_break_mins.to_string(),
            ),
            (
                "pomodoro_long_break_mins",
                settings.pomodoro_long_break_mins.to_string(),
            ),
            (
                "pomodoro_cycles_before_long",
                settings.pomodoro_cycles_before_long.to_string(),
            ),
            (
                "pomodoro_auto_log",
                if settings.pomodoro_auto_log { "true" } else { "false" }.to_string(),
            ),
        ];
        for (k, v) in pairs {
            self.conn.execute(
                "INSERT INTO settings (key, value) VALUES (?1, ?2)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                params![k, v],
            )?;
        }
        Ok(())
    }

    // ─── Clients ──────────────────────────────────────────────────────────────

    pub fn list_clients(&self) -> Result<Vec<Client>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, notes, archived, created_at FROM clients ORDER BY archived ASC, name COLLATE NOCASE ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            let created_str: String = row.get(4)?;
            Ok(Client {
                id: row.get(0)?,
                name: row.get(1)?,
                notes: row.get(2)?,
                archived: row.get::<_, i64>(3)? != 0,
                created_at: parse_dt(&created_str),
            })
        })?;
        let mut result = Vec::new();
        for r in rows {
            result.push(r?);
        }
        Ok(result)
    }

    pub fn create_client(&self, name: &str, notes: &str) -> Result<i64> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO clients (name, notes, archived, created_at) VALUES (?1, ?2, 0, ?3)",
            params![name.trim(), notes.trim(), now],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn update_client(&self, id: i64, name: &str, notes: &str, archived: bool) -> Result<()> {
        self.conn.execute(
            "UPDATE clients SET name = ?1, notes = ?2, archived = ?3 WHERE id = ?4",
            params![name.trim(), notes.trim(), archived as i64, id],
        )?;
        Ok(())
    }

    pub fn delete_client(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM clients WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ─── Projects ─────────────────────────────────────────────────────────────

    pub fn list_projects(&self) -> Result<Vec<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, client_id, color, billable_default, hourly_rate, archived, created_at
             FROM projects ORDER BY archived ASC, name COLLATE NOCASE ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            let created_str: String = row.get(7)?;
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                client_id: row.get(2)?,
                color: row.get(3)?,
                billable_default: row.get::<_, i64>(4)? != 0,
                hourly_rate: row.get(5)?,
                archived: row.get::<_, i64>(6)? != 0,
                created_at: parse_dt(&created_str),
            })
        })?;
        let mut result = Vec::new();
        for r in rows {
            result.push(r?);
        }
        Ok(result)
    }

    pub fn create_project(
        &self,
        name: &str,
        client_id: Option<i64>,
        color: &str,
        billable_default: bool,
        hourly_rate: f64,
    ) -> Result<i64> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO projects (name, client_id, color, billable_default, hourly_rate, archived, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, 0, ?6)",
            params![
                name.trim(),
                client_id,
                color,
                billable_default as i64,
                hourly_rate.max(0.0),
                now
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn update_project(
        &self,
        id: i64,
        name: &str,
        client_id: Option<i64>,
        color: &str,
        billable_default: bool,
        hourly_rate: f64,
        archived: bool,
    ) -> Result<()> {
        self.conn.execute(
            "UPDATE projects
             SET name = ?1, client_id = ?2, color = ?3, billable_default = ?4, hourly_rate = ?5, archived = ?6
             WHERE id = ?7",
            params![
                name.trim(),
                client_id,
                color,
                billable_default as i64,
                hourly_rate.max(0.0),
                archived as i64,
                id
            ],
        )?;
        Ok(())
    }

    pub fn delete_project(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM projects WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ─── Tags ─────────────────────────────────────────────────────────────────

    pub fn list_tags(&self) -> Result<Vec<Tag>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, color, created_at FROM tags ORDER BY name COLLATE NOCASE ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            let created_str: String = row.get(3)?;
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                created_at: parse_dt(&created_str),
            })
        })?;
        let mut result = Vec::new();
        for r in rows {
            result.push(r?);
        }
        Ok(result)
    }

    pub fn create_tag(&self, name: &str, color: &str) -> Result<i64> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO tags (name, color, created_at) VALUES (?1, ?2, ?3)",
            params![name.trim(), color, now],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn update_tag(&self, id: i64, name: &str, color: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE tags SET name = ?1, color = ?2 WHERE id = ?3",
            params![name.trim(), color, id],
        )?;
        Ok(())
    }

    pub fn delete_tag(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM tags WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ─── Time Entries ─────────────────────────────────────────────────────────

    pub fn list_time_entries(&self) -> Result<Vec<TimeEntry>> {
        let mut tag_map: HashMap<i64, Vec<i64>> = HashMap::new();
        {
            let mut tag_stmt = self
                .conn
                .prepare("SELECT time_entry_id, tag_id FROM time_entry_tags")?;
            let tag_rows = tag_stmt.query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)))?;
            for r in tag_rows {
                let (entry_id, tag_id) = r?;
                tag_map.entry(entry_id).or_default().push(tag_id);
            }
        }

        let mut stmt = self.conn.prepare(
            "SELECT id, description, project_id, start_time, end_time, billable, hourly_rate, created_at
             FROM time_entries
             ORDER BY start_time DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let start_str: String = row.get(3)?;
            let end_str: Option<String> = row.get(4)?;
            let created_str: String = row.get(7)?;
            Ok(TimeEntry {
                id,
                description: row.get(1)?,
                project_id: row.get(2)?,
                start_time: parse_dt(&start_str),
                end_time: end_str.map(|s| parse_dt(&s)),
                billable: row.get::<_, i64>(5)? != 0,
                hourly_rate: row.get(6)?,
                tag_ids: tag_map.remove(&id).unwrap_or_default(),
                created_at: parse_dt(&created_str),
            })
        })?;

        let mut entries = Vec::new();
        for r in rows {
            entries.push(r?);
        }
        Ok(entries)
    }

    pub fn start_timer(
        &self,
        description: &str,
        project_id: Option<i64>,
        billable: bool,
        tag_ids: &[i64],
    ) -> Result<i64> {
        let now = Utc::now();
        // Stop any currently running timer first so only one timer runs at a time
        self.stop_running_timers_at(now)?;

        let now_str = now.to_rfc3339();
        self.conn.execute(
            "INSERT INTO time_entries (description, project_id, start_time, end_time, billable, hourly_rate, created_at)
             VALUES (?1, ?2, ?3, NULL, ?4, NULL, ?5)",
            params![description.trim(), project_id, now_str, billable as i64, now_str],
        )?;
        let id = self.conn.last_insert_rowid();
        self.set_entry_tags(id, tag_ids)?;
        Ok(id)
    }

    pub fn stop_running_timers_at(&self, stop_time: DateTime<Utc>) -> Result<usize> {
        let count = self.conn.execute(
            "UPDATE time_entries SET end_time = ?1 WHERE end_time IS NULL",
            params![stop_time.to_rfc3339()],
        )?;
        Ok(count)
    }

    pub fn create_manual_entry(
        &self,
        description: &str,
        project_id: Option<i64>,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        billable: bool,
        hourly_rate: Option<f64>,
        tag_ids: &[i64],
    ) -> Result<i64> {
        let now_str = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO time_entries (description, project_id, start_time, end_time, billable, hourly_rate, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                description.trim(),
                project_id,
                start_time.to_rfc3339(),
                end_time.to_rfc3339(),
                billable as i64,
                hourly_rate,
                now_str
            ],
        )?;
        let id = self.conn.last_insert_rowid();
        self.set_entry_tags(id, tag_ids)?;
        Ok(id)
    }

    pub fn update_time_entry(
        &self,
        id: i64,
        description: &str,
        project_id: Option<i64>,
        start_time: DateTime<Utc>,
        end_time: Option<DateTime<Utc>>,
        billable: bool,
        hourly_rate: Option<f64>,
        tag_ids: &[i64],
    ) -> Result<()> {
        self.conn.execute(
            "UPDATE time_entries
             SET description = ?1, project_id = ?2, start_time = ?3, end_time = ?4, billable = ?5, hourly_rate = ?6
             WHERE id = ?7",
            params![
                description.trim(),
                project_id,
                start_time.to_rfc3339(),
                end_time.map(|t| t.to_rfc3339()),
                billable as i64,
                hourly_rate,
                id
            ],
        )?;
        self.set_entry_tags(id, tag_ids)?;
        Ok(())
    }

    pub fn delete_time_entry(&self, id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM time_entries WHERE id = ?1", params![id])?;
        Ok(())
    }

    fn set_entry_tags(&self, entry_id: i64, tag_ids: &[i64]) -> Result<()> {
        self.conn.execute(
            "DELETE FROM time_entry_tags WHERE time_entry_id = ?1",
            params![entry_id],
        )?;
        for &tag_id in tag_ids {
            self.conn.execute(
                "INSERT OR IGNORE INTO time_entry_tags (time_entry_id, tag_id) VALUES (?1, ?2)",
                params![entry_id, tag_id],
            )?;
        }
        Ok(())
    }
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_crud_flow() -> Result<()> {
        let db = Database::open_in_memory()?;

        let client_id = db.create_client("Acme Corp", "Key client")?;
        let project_id = db.create_project("Website Redesign", Some(client_id), "#E95420", true, 120.0)?;
        let tag_id = db.create_tag("Frontend", "#03A9F4")?;

        // Start timer
        let entry1 = db.start_timer("Building header", Some(project_id), true, &[tag_id])?;
        let entries = db.list_time_entries()?;
        assert_eq!(entries.len(), 1);
        assert!(entries[0].is_running());
        assert_eq!(entries[0].tag_ids, vec![tag_id]);

        // Starting a second timer automatically stops the first
        let entry2 = db.start_timer("Fixing nav", Some(project_id), false, &[])?;
        assert_ne!(entry1, entry2);
        let entries = db.list_time_entries()?;
        let running_count = entries.iter().filter(|e| e.is_running()).count();
        assert_eq!(running_count, 1);

        // Manual entry
        let end = Utc::now();
        let start = end - Duration::hours(2);
        let manual_id = db.create_manual_entry(
            "Client call",
            Some(project_id),
            start,
            end,
            true,
            None,
            &[tag_id],
        )?;
        let entries = db.list_time_entries()?;
        let manual = entries.iter().find(|e| e.id == manual_id).unwrap();
        assert_eq!(manual.duration_secs(), 7200);

        let projects = db.list_projects()?;
        assert_eq!(manual.billable_amount(projects.first(), 0.0), 240.0);

        // Settings roundtrip
        let mut settings = db.load_settings()?;
        settings.currency_symbol = "€".to_string();
        settings.default_hourly_rate = 85.0;
        db.save_settings(&settings)?;
        let loaded = db.load_settings()?;
        assert_eq!(loaded.currency_symbol, "€");
        assert_eq!(loaded.default_hourly_rate, 85.0);

        Ok(())
    }

    #[test]
    fn test_theme_settings_roundtrip() -> Result<()> {
        let db = Database::open_in_memory()?;
        for &theme in AppTheme::ALL {
            let mut settings = db.load_settings()?;
            settings.theme = theme;
            db.save_settings(&settings)?;
            let loaded = db.load_settings()?;
            assert_eq!(loaded.theme, theme);
        }
        Ok(())
    }

    #[test]
    fn test_legacy_dark_mode_fallback_and_precedence() -> Result<()> {
        let db = Database::open_in_memory()?;
        db.conn.execute(
            "INSERT INTO settings (key, value) VALUES ('dark_mode', 'false')",
            [],
        )?;
        let loaded = db.load_settings()?;
        assert!(!loaded.theme.is_dark());
        assert_eq!(loaded.theme, AppTheme::default_for_os().with_dark(false));

        db.conn.execute(
            "INSERT INTO settings (key, value) VALUES ('theme', 'windows_light')
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            [],
        )?;
        db.conn.execute(
            "INSERT INTO settings (key, value) VALUES ('dark_mode', 'true')
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            [],
        )?;
        let loaded = db.load_settings()?;
        assert_eq!(loaded.theme, AppTheme::WindowsLight);
        Ok(())
    }
}
