use rusqlite::{Connection, Result as SqliteResult};
use std::path::PathBuf;
use std::sync::Mutex;

pub mod repositories;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(app_dir: PathBuf) -> SqliteResult<Self> {
        std::fs::create_dir_all(&app_dir).ok();
        let db_path = app_dir.join("feed-scope.db");
        let conn = Connection::open(db_path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let db = Database {
            conn: Mutex::new(conn),
        };
        db.run_migrations()?;
        Ok(db)
    }

    fn run_migrations(&self) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS feeds (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                url TEXT NOT NULL,
                kind TEXT NOT NULL DEFAULT 'rss',
                category TEXT NOT NULL DEFAULT 'uncategorized',
                enabled INTEGER NOT NULL DEFAULT 1,
                refresh_interval_seconds INTEGER NOT NULL DEFAULT 300,
                credential_id TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                last_checked_at TEXT,
                last_success_at TEXT,
                last_error TEXT
            );

            CREATE TABLE IF NOT EXISTS items (
                id TEXT PRIMARY KEY,
                feed_id TEXT NOT NULL,
                guid TEXT NOT NULL,
                title TEXT NOT NULL,
                link TEXT,
                author TEXT,
                summary TEXT,
                content TEXT,
                published_at TEXT,
                updated_at TEXT,
                fingerprint TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (feed_id) REFERENCES feeds(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS item_states (
                item_id TEXT PRIMARY KEY,
                read INTEGER NOT NULL DEFAULT 0,
                starred INTEGER NOT NULL DEFAULT 0,
                hidden INTEGER NOT NULL DEFAULT 0,
                notified INTEGER NOT NULL DEFAULT 0,
                archived INTEGER NOT NULL DEFAULT 0,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS rules (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1,
                feed_id TEXT,
                condition_json TEXT NOT NULL DEFAULT '[]',
                action_json TEXT NOT NULL DEFAULT '[]',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (feed_id) REFERENCES feeds(id) ON DELETE SET NULL
            );

            CREATE TABLE IF NOT EXISTS sync_logs (
                id TEXT PRIMARY KEY,
                feed_id TEXT NOT NULL,
                started_at TEXT NOT NULL,
                finished_at TEXT,
                status TEXT NOT NULL DEFAULT 'running',
                items_found INTEGER NOT NULL DEFAULT 0,
                items_new INTEGER NOT NULL DEFAULT 0,
                error_message TEXT,
                FOREIGN KEY (feed_id) REFERENCES feeds(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS credentials (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                auth_type TEXT NOT NULL DEFAULT 'none',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS credential_secrets (
                credential_id TEXT PRIMARY KEY,
                secret TEXT NOT NULL,
                FOREIGN KEY (credential_id) REFERENCES credentials(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS app_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_items_feed_id ON items(feed_id);
            CREATE INDEX IF NOT EXISTS idx_items_published_at ON items(published_at);
            CREATE INDEX IF NOT EXISTS idx_items_guid ON items(guid);
            CREATE INDEX IF NOT EXISTS idx_item_states_read ON item_states(read);
            CREATE INDEX IF NOT EXISTS idx_item_states_starred ON item_states(starred);
            ",
        )?;

        // Insert default settings if not exists
        conn.execute(
            "INSERT OR IGNORE INTO app_settings (key, value) VALUES ('sync_interval_default', '300')",
            [],
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO app_settings (key, value) VALUES ('notifications_enabled', 'true')",
            [],
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO app_settings (key, value) VALUES ('max_items_per_feed', '500')",
            [],
        )?;

        Ok(())
    }
}
