use rusqlite::{params, Result as SqliteResult};
use std::sync::Mutex;

use crate::models::*;

pub struct RuleRepository;

impl RuleRepository {
    pub fn get_all(db: &Mutex<rusqlite::Connection>) -> SqliteResult<Vec<Rule>> {
        let conn = db.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, enabled, feed_id, condition_json, action_json, created_at, updated_at
             FROM rules ORDER BY name",
        )?;
        let rules = stmt
            .query_map([], |row| {
                Ok(Rule {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    enabled: row.get(2)?,
                    feed_id: row.get(3)?,
                    condition_json: row.get(4)?,
                    action_json: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;
        Ok(rules)
    }

    pub fn get_enabled(db: &Mutex<rusqlite::Connection>) -> SqliteResult<Vec<Rule>> {
        let conn = db.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, enabled, feed_id, condition_json, action_json, created_at, updated_at
             FROM rules WHERE enabled = 1 ORDER BY name",
        )?;
        let rules = stmt
            .query_map([], |row| {
                Ok(Rule {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    enabled: row.get(2)?,
                    feed_id: row.get(3)?,
                    condition_json: row.get(4)?,
                    action_json: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;
        Ok(rules)
    }

    pub fn insert(db: &Mutex<rusqlite::Connection>, rule: &Rule) -> SqliteResult<()> {
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO rules (id, name, enabled, feed_id, condition_json, action_json, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                rule.id,
                rule.name,
                rule.enabled,
                rule.feed_id,
                rule.condition_json,
                rule.action_json,
                rule.created_at,
                rule.updated_at,
            ],
        )?;
        Ok(())
    }

    pub fn delete(db: &Mutex<rusqlite::Connection>, id: &str) -> SqliteResult<()> {
        let conn = db.lock().unwrap();
        conn.execute("DELETE FROM rules WHERE id = ?1", params![id])?;
        Ok(())
    }
}
