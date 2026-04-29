use std::sync::Arc;
use tauri::State;

use crate::db::repositories::credentials::CredentialRepository;
use crate::db::repositories::feed::FeedRepository;
use crate::db::repositories::item::ItemRepository;
use crate::db::repositories::rules::RuleRepository;
use crate::db::repositories::settings::SettingsRepository;
use crate::db::Database;
use crate::models::*;
use crate::services::SyncEngine;

pub struct AppState {
    pub db: Arc<Database>,
    pub sync_engine: SyncEngine,
}

// ── Feeds ──

#[tauri::command]
pub fn get_feeds(state: State<AppState>) -> Result<Vec<Feed>, String> {
    FeedRepository::get_all(&state.db.conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_feed(state: State<AppState>, feed: Feed) -> Result<Feed, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let mut feed = feed;
    feed.id = uuid::Uuid::new_v4().to_string();
    feed.created_at = now.clone();
    feed.updated_at = now;
    FeedRepository::insert(&state.db.conn, &feed).map_err(|e| e.to_string())?;
    Ok(feed)
}

#[tauri::command]
pub fn update_feed(state: State<AppState>, feed: Feed) -> Result<Feed, String> {
    let mut feed = feed;
    feed.updated_at = chrono::Utc::now().to_rfc3339();
    FeedRepository::update(&state.db.conn, &feed).map_err(|e| e.to_string())?;
    Ok(feed)
}

#[tauri::command]
pub fn delete_feed(state: State<AppState>, id: String) -> Result<(), String> {
    FeedRepository::delete(&state.db.conn, &id).map_err(|e| e.to_string())
}

// ── Items ──

#[tauri::command]
pub fn get_items(
    state: State<AppState>,
    feed_id: Option<String>,
    opts: Option<GetItemsOpts>,
) -> Result<Vec<ItemWithState>, String> {
    let opts = opts.unwrap_or_default();
    ItemRepository::get_items(
        &state.db.conn,
        feed_id.as_deref(),
        opts.unread_only,
        opts.starred_only,
        opts.limit,
        opts.offset,
    )
    .map_err(|e| e.to_string())
}

#[derive(serde::Deserialize, Default)]
pub struct GetItemsOpts {
    pub unread_only: bool,
    pub starred_only: bool,
    pub limit: i32,
    pub offset: i32,
}

#[tauri::command]
pub fn mark_read(state: State<AppState>, item_id: String) -> Result<(), String> {
    ItemRepository::mark_read(&state.db.conn, &item_id, true).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn mark_unread(state: State<AppState>, item_id: String) -> Result<(), String> {
    ItemRepository::mark_read(&state.db.conn, &item_id, false).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_star(state: State<AppState>, item_id: String) -> Result<bool, String> {
    ItemRepository::toggle_star(&state.db.conn, &item_id).map_err(|e| e.to_string())
}

// ── Rules ──

#[tauri::command]
pub fn get_rules(state: State<AppState>) -> Result<Vec<Rule>, String> {
    RuleRepository::get_all(&state.db.conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_rule(state: State<AppState>, rule: Rule) -> Result<Rule, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let mut rule = rule;
    rule.id = uuid::Uuid::new_v4().to_string();
    rule.created_at = now.clone();
    rule.updated_at = now;
    RuleRepository::insert(&state.db.conn, &rule).map_err(|e| e.to_string())?;
    Ok(rule)
}

#[tauri::command]
pub fn delete_rule(state: State<AppState>, id: String) -> Result<(), String> {
    RuleRepository::delete(&state.db.conn, &id).map_err(|e| e.to_string())
}

// ── Credentials ──

#[tauri::command]
pub fn get_credentials(state: State<AppState>) -> Result<Vec<Credential>, String> {
    CredentialRepository::get_all(&state.db.conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_credential(
    state: State<AppState>,
    credential: Credential,
    secret: String,
) -> Result<Credential, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let mut cred = credential;
    cred.id = uuid::Uuid::new_v4().to_string();
    cred.created_at = now.clone();
    cred.updated_at = now;
    CredentialRepository::insert(&state.db.conn, &cred, &secret).map_err(|e| e.to_string())?;
    Ok(cred)
}

#[tauri::command]
pub fn delete_credential(state: State<AppState>, id: String) -> Result<(), String> {
    CredentialRepository::delete(&state.db.conn, &id).map_err(|e| e.to_string())
}

// ── Settings ──

#[tauri::command]
pub fn get_settings(state: State<AppState>) -> Result<AppSettings, String> {
    SettingsRepository::get_all(&state.db.conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_settings(state: State<AppState>, settings: AppSettings) -> Result<(), String> {
    SettingsRepository::save(&state.db.conn, &settings).map_err(|e| e.to_string())
}

// ── Sync ──

#[tauri::command]
pub async fn sync_feed(state: State<'_, AppState>, id: String) -> Result<SyncLog, String> {
    state.sync_engine.sync_feed(&id).await.map_err(|e| e.to_string())
}
