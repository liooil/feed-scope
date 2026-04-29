use rusqlite::{params, Result as SqliteResult};
use std::sync::Mutex;

use crate::models::*;

pub struct SettingsRepository;

impl SettingsRepository {
    pub fn get_all(db: &Mutex<rusqlite::Connection>) -> SqliteResult<AppSettings> {
        let conn = db.lock().unwrap();
        let get = |key: &str, default: &str| -> String {
            conn.query_row(
                "SELECT value FROM app_settings WHERE key = ?1",
                params![key],
                |row| row.get(0),
            )
            .unwrap_or_else(|_| default.to_string())
        };

        Ok(AppSettings {
            sync_interval_default: get("sync_interval_default", "300").parse().unwrap_or(300),
            notifications_enabled: get("notifications_enabled", "true") == "true",
            quiet_hours_start: {
                let v = get("quiet_hours_start", "");
                if v.is_empty() { None } else { Some(v) }
            },
            quiet_hours_end: {
                let v = get("quiet_hours_end", "");
                if v.is_empty() { None } else { Some(v) }
            },
            max_items_per_feed: get("max_items_per_feed", "500").parse().unwrap_or(500),
        })
    }

    pub fn save(db: &Mutex<rusqlite::Connection>, settings: &AppSettings) -> SqliteResult<()> {
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO app_settings (key, value) VALUES ('sync_interval_default', ?1)",
            params![settings.sync_interval_default.to_string()],
        )?;
        conn.execute(
            "INSERT OR REPLACE INTO app_settings (key, value) VALUES ('notifications_enabled', ?1)",
            params![settings.notifications_enabled.to_string()],
        )?;
        conn.execute(
            "INSERT OR REPLACE INTO app_settings (key, value) VALUES ('quiet_hours_start', ?1)",
            params![settings.quiet_hours_start.clone().unwrap_or_default()],
        )?;
        conn.execute(
            "INSERT OR REPLACE INTO app_settings (key, value) VALUES ('quiet_hours_end', ?1)",
            params![settings.quiet_hours_end.clone().unwrap_or_default()],
        )?;
        conn.execute(
            "INSERT OR REPLACE INTO app_settings (key, value) VALUES ('max_items_per_feed', ?1)",
            params![settings.max_items_per_feed.to_string()],
        )?;
        Ok(())
    }
}
