mod commands;
mod db;
mod models;
mod services;

use commands::AppState;
use db::Database;
use services::SyncEngine;
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to resolve app data directory");

            let db = Database::new(app_dir).expect("Failed to initialize database");
            let db = Arc::new(db);
            let sync_engine = SyncEngine::new(db.clone());

            app.manage(AppState { db, sync_engine });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_feeds,
            commands::add_feed,
            commands::update_feed,
            commands::delete_feed,
            commands::get_items,
            commands::mark_read,
            commands::mark_unread,
            commands::toggle_star,
            commands::get_rules,
            commands::add_rule,
            commands::delete_rule,
            commands::get_credentials,
            commands::save_credential,
            commands::delete_credential,
            commands::get_settings,
            commands::save_settings,
            commands::sync_feed,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
