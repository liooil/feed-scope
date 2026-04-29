use rusqlite::{params, Result as SqliteResult};
use std::sync::Mutex;

use crate::models::*;

pub struct CredentialRepository;

impl CredentialRepository {
    pub fn get_all(db: &Mutex<rusqlite::Connection>) -> SqliteResult<Vec<Credential>> {
        let conn = db.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, auth_type, created_at, updated_at FROM credentials ORDER BY name",
        )?;
        let creds = stmt
            .query_map([], |row| {
                Ok(Credential {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    auth_type: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;
        Ok(creds)
    }

    pub fn insert(db: &Mutex<rusqlite::Connection>, cred: &Credential, secret: &str) -> SqliteResult<()> {
        let conn = db.lock().unwrap();
        conn.execute(
            "INSERT INTO credentials (id, name, auth_type, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![cred.id, cred.name, cred.auth_type, cred.created_at, cred.updated_at],
        )?;
        conn.execute(
            "INSERT INTO credential_secrets (credential_id, secret) VALUES (?1, ?2)",
            params![cred.id, secret],
        )?;
        Ok(())
    }

    pub fn delete(db: &Mutex<rusqlite::Connection>, id: &str) -> SqliteResult<()> {
        let conn = db.lock().unwrap();
        conn.execute("DELETE FROM credentials WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_secret(db: &Mutex<rusqlite::Connection>, id: &str) -> SqliteResult<Option<String>> {
        let conn = db.lock().unwrap();
        let result = conn.query_row(
            "SELECT secret FROM credential_secrets WHERE credential_id = ?1",
            params![id],
            |row| row.get(0),
        );
        match result {
            Ok(secret) => Ok(Some(secret)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
