use std::sync::Arc;

use crate::db::repositories::credentials::CredentialRepository;
use crate::db::repositories::feed::FeedRepository;
use crate::db::repositories::item::ItemRepository;
use crate::db::repositories::rules::RuleRepository;
use crate::db::repositories::settings::SettingsRepository;
use crate::db::repositories::sync_logs::SyncLogRepository;
use crate::db::Database;
use crate::models::*;
use crate::services::{FeedParser, NotificationEngine};

pub struct SyncEngine {
    db: Arc<Database>,
    notifier: NotificationEngine,
}

impl SyncEngine {
    pub fn new(db: Arc<Database>) -> Self {
        SyncEngine {
            db,
            notifier: NotificationEngine,
        }
    }

    pub async fn sync_feed(&self, feed_id: &str) -> anyhow::Result<SyncLog> {
        let now = chrono::Utc::now().to_rfc3339();
        let mut log = SyncLog {
            id: uuid::Uuid::new_v4().to_string(),
            feed_id: feed_id.to_string(),
            started_at: now,
            finished_at: None,
            status: "running".to_string(),
            items_found: 0,
            items_new: 0,
            error_message: None,
        };

        SyncLogRepository::insert(&self.db.conn, &log)?;

        let feed = match FeedRepository::get_by_id(&self.db.conn, feed_id)? {
            Some(f) => f,
            None => anyhow::bail!("Feed not found"),
        };

        let result = self.fetch_and_process(&feed, &mut log).await;

        let now = chrono::Utc::now().to_rfc3339();
        log.finished_at = Some(now);

        match result {
            Ok(()) => {
                log.status = "success".to_string();
                SyncLogRepository::update(&self.db.conn, &log)?;
                FeedRepository::update_sync_status(&self.db.conn, feed_id, true, None)?;
            }
            Err(e) => {
                let err_msg = format!("{:#}", e);
                log.status = "error".to_string();
                log.error_message = Some(err_msg.clone());
                SyncLogRepository::update(&self.db.conn, &log)?;
                FeedRepository::update_sync_status(&self.db.conn, feed_id, false, Some(&err_msg))?;
            }
        }

        Ok(log)
    }

    async fn fetch_and_process(&self, feed: &Feed, log: &mut SyncLog) -> anyhow::Result<()> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        let mut request = client.get(&feed.url);

        if let Some(ref cred_id) = feed.credential_id {
            let creds = CredentialRepository::get_all(&self.db.conn)?;
            if let Some(cred) = creds.into_iter().find(|c| &c.id == cred_id) {
                if let Some(secret) = CredentialRepository::get_secret(&self.db.conn, cred_id)? {
                    request = match cred.auth_type.as_str() {
                        "basic" => {
                            request.header("Authorization", format!("Basic {}", secret))
                        }
                        "bearer" => {
                            request.header("Authorization", format!("Bearer {}", secret))
                        }
                        "header" => {
                            if let Some((name, value)) = secret.split_once(':') {
                                request.header(name.trim(), value.trim())
                            } else {
                                request
                            }
                        }
                        _ => request,
                    };
                }
            }
        }

        let response = request.send().await?;
        let status = response.status();
        if !status.is_success() {
            anyhow::bail!("HTTP {}: {}", status.as_u16(), status.canonical_reason().unwrap_or("Unknown"));
        }

        let content = response.text().await?;
        let parsed = FeedParser::parse(&content, &feed.id, &feed.kind)?;

        log.items_found = parsed.items.len() as i32;

        let rules = RuleRepository::get_enabled(&self.db.conn)?;
        let settings = SettingsRepository::get_all(&self.db.conn)?;

        for item in &parsed.items {
            if ItemRepository::exists_by_guid(&self.db.conn, &feed.id, &item.guid)? {
                continue;
            }

            ItemRepository::insert_item(&self.db.conn, item)?;
            ItemRepository::ensure_item_state(&self.db.conn, &item.id)?;
            log.items_new += 1;

            let actions = evaluate_rules(&rules, item, &feed.id);
            for action in &actions {
                match action {
                    RuleAction::Notify => {
                        if settings.notifications_enabled {
                            self.notifier.notify(&feed.name, &item.title, &item.summary);
                        }
                        ItemRepository::set_notified(&self.db.conn, &item.id)?;
                    }
                }
            }
        }

        Self::clean_old_items(&self.db.conn, &feed.id, settings.max_items_per_feed as usize)?;

        Ok(())
    }

    fn clean_old_items(
        db: &std::sync::Mutex<rusqlite::Connection>,
        feed_id: &str,
        max_items: usize,
    ) -> anyhow::Result<()> {
        let conn = db.lock().unwrap();
        conn.execute(
            "DELETE FROM items WHERE feed_id = ?1 AND id NOT IN (
                SELECT id FROM items WHERE feed_id = ?1
                ORDER BY COALESCE(published_at, created_at) DESC LIMIT ?2
            )",
            rusqlite::params![feed_id, max_items],
        )?;
        Ok(())
    }
}

#[derive(Debug)]
enum RuleAction {
    Notify,
}

fn evaluate_rules(rules: &[Rule], item: &Item, feed_id: &str) -> Vec<RuleAction> {
    let mut actions = Vec::new();

    for rule in rules {
        if !rule.enabled {
            continue;
        }
        if let Some(ref fid) = rule.feed_id {
            if fid != feed_id {
                continue;
            }
        }
        if let Ok(conditions) = serde_json::from_str::<Vec<serde_json::Value>>(&rule.condition_json) {
            let matches = conditions.iter().all(|cond| {
                let field = cond.get("field").and_then(|v| v.as_str()).unwrap_or("");
                let op = cond.get("op").and_then(|v| v.as_str()).unwrap_or("");
                let value = cond.get("value").and_then(|v| v.as_str()).unwrap_or("");

                match (field, op) {
                    ("title", "contains") => item.title.contains(value),
                    ("content", "contains") => item.content.as_deref().unwrap_or("").contains(value),
                    ("summary", "contains") => item.summary.as_deref().unwrap_or("").contains(value),
                    ("feed_id", "equals") => feed_id == value,
                    _ => false,
                }
            });

            if matches {
                if let Ok(rule_actions) =
                    serde_json::from_str::<Vec<serde_json::Value>>(&rule.action_json)
                {
                    for action in rule_actions {
                        let action_type =
                            action.get("type").and_then(|v| v.as_str()).unwrap_or("");
                        if action_type == "notify" {
                            actions.push(RuleAction::Notify);
                        }
                    }
                }
            }
        }
    }

    actions
}
