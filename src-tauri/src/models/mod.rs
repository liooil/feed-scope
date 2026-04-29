use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feed {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub category: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_refresh_interval")]
    pub refresh_interval_seconds: i32,
    #[serde(default)]
    pub credential_id: Option<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub last_checked_at: Option<String>,
    #[serde(default)]
    pub last_success_at: Option<String>,
    #[serde(default)]
    pub last_error: Option<String>,
}

fn default_true() -> bool {
    true
}

fn default_refresh_interval() -> i32 {
    300
}

fn empty_array() -> String {
    "[]".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub feed_id: String,
    pub guid: String,
    pub title: String,
    pub link: Option<String>,
    pub author: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub published_at: Option<String>,
    pub updated_at: Option<String>,
    pub fingerprint: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemState {
    pub item_id: String,
    pub read: bool,
    pub starred: bool,
    pub hidden: bool,
    pub notified: bool,
    pub archived: bool,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemWithState {
    pub id: String,
    pub feed_id: String,
    pub guid: String,
    pub title: String,
    pub link: Option<String>,
    pub author: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub published_at: Option<String>,
    pub updated_at: Option<String>,
    pub fingerprint: Option<String>,
    pub created_at: String,
    pub state: ItemState,
    pub feed_name: String,
    pub feed_kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    #[serde(default)]
    pub id: String,
    pub name: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub feed_id: Option<String>,
    #[serde(default = "empty_array")]
    pub condition_json: String,
    #[serde(default = "empty_array")]
    pub action_json: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncLog {
    pub id: String,
    pub feed_id: String,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub status: String,
    pub items_found: i32,
    pub items_new: i32,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    #[serde(default)]
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub auth_type: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub sync_interval_default: i32,
    pub notifications_enabled: bool,
    pub quiet_hours_start: Option<String>,
    pub quiet_hours_end: Option<String>,
    pub max_items_per_feed: i32,
}
