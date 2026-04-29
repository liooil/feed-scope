use rusqlite::{params, Result as SqliteResult};
use std::sync::Mutex;

use crate::models::*;

pub struct FeedRepository;

impl FeedRepository {
    pub fn get_all(db: &Mutex<rusqlite::Connection>) -> SqliteResult<Vec<Feed>> {
        let conn = db.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, url, kind, category, enabled, refresh_interval_seconds,
                    credential_id, created_at, updated_at, last_checked_at, last_success_at, last_error
             FROM feeds ORDER BY name",
        )?;
        let feeds = stmt
            .query_map([], |row| {
                Ok(Feed {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    url: row.get(2)?,
                    kind: row.get(3)?,
                    category: row.get(4)?,
                    enabled: row.get(5)?,
                    refresh_interval_seconds: row.get(6)?,
                    credential_id: row.get(7)?,
                    created_at: row.get(8)?,
                    updated_at: row.get(9)?,
                    last_checked_at: row.get(10)?,
                    last_success_at: row.get(11)?,
                    last_error: row.get(12)?,
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;
        Ok(feeds)
    }

    pub fn get_by_id(db: &Mutex<rusqlite::Connection>, id: &str) -> SqliteResult<Option<Feed>> {
        let conn = db.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, url, kind, category, enabled, refresh_interval_seconds,
                    credential_id, created_at, updated_at, last_checked_at, last_success_at, last_error
             FROM feeds WHERE id = ?1",
        )?;
        let mut feeds = stmt
            .query_map(params![id], |row| {
                Ok(Feed {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    url: row.get(2)?,
                    kind: row.get(3)?,
                    category: row.get(4)?,
                    enabled: row.get(5)?,
                    refresh_interval_seconds: row.get(6)?,
                    credential_id: row.get(7)?,
                    created_at: row.get(8)?,
                    updated_at: row.get(9)?,
                    last_checked_at: row.get(10)?,
                    last_success_at: row.get(11)?,
                    last_error: row.get(12)?,
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;
        Ok(feeds.pop())
    }

    pub fn insert(db: &Mutex<rusqlite::Connection>, feed: &Feed) -> SqliteResult<()> {
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO feeds (id, name, url, kind, category, enabled, refresh_interval_seconds,
                                credential_id, created_at, updated_at, last_checked_at, last_success_at, last_error)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                feed.id,
                feed.name,
                feed.url,
                feed.kind,
                feed.category,
                feed.enabled,
                feed.refresh_interval_seconds,
                feed.credential_id,
                feed.created_at,
                feed.updated_at,
                feed.last_checked_at,
                feed.last_success_at,
                feed.last_error,
            ],
        )?;
        Ok(())
    }

    pub fn update(db: &Mutex<rusqlite::Connection>, feed: &Feed) -> SqliteResult<()> {
        let conn = db.lock().unwrap();
        conn.execute(
            "UPDATE feeds SET name=?2, url=?3, kind=?4, category=?5, enabled=?6,
                              refresh_interval_seconds=?7, credential_id=?8, updated_at=?9,
                              last_checked_at=?10, last_success_at=?11, last_error=?12
             WHERE id=?1",
            params![
                feed.id,
                feed.name,
                feed.url,
                feed.kind,
                feed.category,
                feed.enabled,
                feed.refresh_interval_seconds,
                feed.credential_id,
                feed.updated_at,
                feed.last_checked_at,
                feed.last_success_at,
                feed.last_error,
            ],
        )?;
        Ok(())
    }

    pub fn delete(db: &Mutex<rusqlite::Connection>, id: &str) -> SqliteResult<()> {
        let conn = db.lock().unwrap();
        conn.execute("DELETE FROM feeds WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn update_sync_status(
        db: &Mutex<rusqlite::Connection>,
        id: &str,
        success: bool,
        error: Option<&str>,
    ) -> SqliteResult<()> {
        let conn = db.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        if success {
            conn.execute(
                "UPDATE feeds SET last_checked_at=?2, last_success_at=?2, last_error=NULL, updated_at=?2 WHERE id=?1",
                params![id, now],
            )?;
        } else {
            conn.execute(
                "UPDATE feeds SET last_checked_at=?2, last_error=?3, updated_at=?2 WHERE id=?1",
                params![id, now, error],
            )?;
        }
        Ok(())
    }
}
