use rusqlite::{params, Result as SqliteResult};
use std::sync::Mutex;

use crate::models::*;

pub struct ItemRepository;

impl ItemRepository {
    pub fn get_items(
        db: &Mutex<rusqlite::Connection>,
        feed_id: Option<&str>,
        unread_only: bool,
        starred_only: bool,
        limit: i32,
        offset: i32,
    ) -> SqliteResult<Vec<ItemWithState>> {
        let conn = db.lock().unwrap();
        let mut sql = String::from(
            "SELECT i.id, i.feed_id, i.guid, i.title, i.link, i.author, i.summary, i.content,
                    i.published_at, i.updated_at, i.fingerprint, i.created_at,
                    COALESCE(s.read, 0), COALESCE(s.starred, 0), COALESCE(s.hidden, 0),
                    COALESCE(s.notified, 0), COALESCE(s.archived, 0),
                    COALESCE(s.updated_at, ''), f.name, f.kind
             FROM items i
             JOIN feeds f ON i.feed_id = f.id
             LEFT JOIN item_states s ON i.id = s.item_id
             WHERE COALESCE(s.hidden, 0) = 0",
        );

        let mut conditions = Vec::new();
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(fid) = feed_id {
            conditions.push(format!("i.feed_id = ?{}", conditions.len() + 1));
            param_values.push(Box::new(fid.to_string()));
        }
        if unread_only {
            conditions.push(format!("COALESCE(s.read, 0) = 0"));
        }
        if starred_only {
            conditions.push(format!("COALESCE(s.starred, 1) = 1"));
        }

        for (i, cond) in conditions.iter().enumerate() {
            if i == 0 {
                sql.push_str(" AND ");
            }
            sql.push_str(cond);
            if i < conditions.len() - 1 {
                sql.push_str(" AND ");
            }
        }

        sql.push_str(" ORDER BY i.published_at DESC LIMIT ?");
        let limit_idx = param_values.len() + 1;
        sql.push_str(&format!("{} OFFSET ?{}", limit_idx, limit_idx + 1));
        param_values.push(Box::new(limit));
        param_values.push(Box::new(offset));

        let param_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
        let mut stmt = conn.prepare(&sql)?;
        let items = stmt
            .query_map(param_refs.as_slice(), |row| {
                Ok(ItemWithState {
                    id: row.get(0)?,
                    feed_id: row.get(1)?,
                    guid: row.get(2)?,
                    title: row.get(3)?,
                    link: row.get(4)?,
                    author: row.get(5)?,
                    summary: row.get(6)?,
                    content: row.get(7)?,
                    published_at: row.get(8)?,
                    updated_at: row.get(9)?,
                    fingerprint: row.get(10)?,
                    created_at: row.get(11)?,
                    state: ItemState {
                        item_id: row.get(0)?,
                        read: row.get::<_, i32>(12)? != 0,
                        starred: row.get::<_, i32>(13)? != 0,
                        hidden: row.get::<_, i32>(14)? != 0,
                        notified: row.get::<_, i32>(15)? != 0,
                        archived: row.get::<_, i32>(16)? != 0,
                        updated_at: row.get(17)?,
                    },
                    feed_name: row.get(18)?,
                    feed_kind: row.get(19)?,
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;
        Ok(items)
    }

    pub fn insert_item(db: &Mutex<rusqlite::Connection>, item: &Item) -> SqliteResult<()> {
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT OR IGNORE INTO items (id, feed_id, guid, title, link, author, summary, content,
                                          published_at, updated_at, fingerprint, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                item.id,
                item.feed_id,
                item.guid,
                item.title,
                item.link,
                item.author,
                item.summary,
                item.content,
                item.published_at,
                item.updated_at,
                item.fingerprint,
                item.created_at,
            ],
        )?;
        Ok(())
    }

    pub fn ensure_item_state(db: &Mutex<rusqlite::Connection>, item_id: &str) -> SqliteResult<()> {
        let conn = db.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT OR IGNORE INTO item_states (item_id, read, starred, hidden, notified, archived, updated_at)
             VALUES (?1, 0, 0, 0, 0, 0, ?2)",
            params![item_id, now],
        )?;
        Ok(())
    }

    pub fn mark_read(db: &Mutex<rusqlite::Connection>, item_id: &str, read: bool) -> SqliteResult<()> {
        let conn = db.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE item_states SET read = ?2, updated_at = ?3 WHERE item_id = ?1",
            params![item_id, read, now],
        )?;
        Ok(())
    }

    pub fn toggle_star(db: &Mutex<rusqlite::Connection>, item_id: &str) -> SqliteResult<bool> {
        let conn = db.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE item_states SET starred = CASE WHEN starred = 0 THEN 1 ELSE 0 END, updated_at = ?2 WHERE item_id = ?1",
            params![item_id, now],
        )?;
        let starred: i32 = conn.query_row(
            "SELECT starred FROM item_states WHERE item_id = ?1",
            params![item_id],
            |row| row.get(0),
        )?;
        Ok(starred != 0)
    }

    pub fn set_notified(db: &Mutex<rusqlite::Connection>, item_id: &str) -> SqliteResult<()> {
        let conn = db.lock().unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE item_states SET notified = 1, updated_at = ?2 WHERE item_id = ?1",
            params![item_id, now],
        )?;
        Ok(())
    }

    pub fn exists_by_guid(db: &Mutex<rusqlite::Connection>, feed_id: &str, guid: &str) -> SqliteResult<bool> {
        let conn = db.lock().unwrap();
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM items WHERE feed_id = ?1 AND guid = ?2",
            params![feed_id, guid],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn count_unread(db: &Mutex<rusqlite::Connection>) -> SqliteResult<i32> {
        let conn = db.lock().unwrap();
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM item_states WHERE read = 0 AND hidden = 0",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }
}
