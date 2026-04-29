use rusqlite::{params, Result as SqliteResult};
use std::sync::Mutex;

use crate::models::*;

pub struct SyncLogRepository;

impl SyncLogRepository {
    pub fn insert(db: &Mutex<rusqlite::Connection>, log: &SyncLog) -> SqliteResult<()> {
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO sync_logs (id, feed_id, started_at, finished_at, status, items_found, items_new, error_message)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                log.id,
                log.feed_id,
                log.started_at,
                log.finished_at,
                log.status,
                log.items_found,
                log.items_new,
                log.error_message,
            ],
        )?;
        Ok(())
    }

    pub fn update(db: &Mutex<rusqlite::Connection>, log: &SyncLog) -> SqliteResult<()> {
        let conn = db.lock().unwrap();
        conn.execute(
            "UPDATE sync_logs SET finished_at=?2, status=?3, items_found=?4, items_new=?5, error_message=?6
             WHERE id=?1",
            params![
                log.id,
                log.finished_at,
                log.status,
                log.items_found,
                log.items_new,
                log.error_message,
            ],
        )?;
        Ok(())
    }
}
