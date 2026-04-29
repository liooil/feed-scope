pub struct NotificationEngine;

impl NotificationEngine {
    pub fn notify(&self, feed_name: &str, title: &str, summary: &Option<String>) {
        let body = summary.as_deref().unwrap_or("New item from feed");

        // Use Tauri notification plugin via command - in the sync engine,
        // we send a notification event to the frontend which triggers the OS notification
        // This is a simplified version; full implementation would use tauri_plugin_notification
        log::info!("NOTIFICATION: [{}] {} - {}", feed_name, title, body);
    }
}
