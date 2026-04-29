use anyhow::{Context, Result};
use feed_rs::parser;
use sha2::{Digest, Sha256};

use crate::models::*;

pub struct FeedParser;

#[derive(Debug)]
pub struct ParsedItems {
    pub items: Vec<Item>,
}

impl FeedParser {
    pub fn parse(content: &str, feed_id: &str, kind: &str) -> Result<ParsedItems> {
        match kind {
            "json" => Self::parse_json_feed(content, feed_id),
            _ => Self::parse_xml_feed(content, feed_id),
        }
    }

    fn parse_xml_feed(content: &str, feed_id: &str) -> Result<ParsedItems> {
        let feed = parser::parse(content.as_bytes())
            .context("Failed to parse XML feed")?;

        let now = chrono::Utc::now().to_rfc3339();
        let items: Vec<Item> = feed
            .entries
            .into_iter()
            .map(|entry| {
                let guid = entry.id.clone();
                let title = entry
                    .title
                    .map(|t| t.content)
                    .unwrap_or_else(|| "(untitled)".to_string());
                let link = entry
                    .links
                    .first()
                    .map(|l| l.href.clone());
                let author = entry.authors.first().map(|a| a.name.clone());
                let summary = entry.summary.map(|s| s.content);
                let content = entry.content.and_then(|c| match c.body {
                    Some(b) => Some(b),
                    None => None,
                });
                let published_at = entry.published.or(entry.updated).map(|d| d.to_rfc3339());
                let updated_at = entry.updated.map(|d| d.to_rfc3339());

                let fingerprint = Self::compute_fingerprint(&guid, &title, &link);

                Item {
                    id: uuid::Uuid::new_v4().to_string(),
                    feed_id: feed_id.to_string(),
                    guid,
                    title,
                    link,
                    author,
                    summary,
                    content,
                    published_at,
                    updated_at,
                    fingerprint: Some(fingerprint),
                    created_at: now.clone(),
                }
            })
            .collect();

        Ok(ParsedItems { items })
    }

    fn parse_json_feed(content: &str, feed_id: &str) -> Result<ParsedItems> {
        let json: serde_json::Value =
            serde_json::from_str(content).context("Failed to parse JSON feed")?;

        let now = chrono::Utc::now().to_rfc3339();
        let items = json
            .get("items")
            .and_then(|i| i.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|item| {
                        let guid = item
                            .get("id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        let guid = if guid.is_empty() {
                            uuid::Uuid::new_v4().to_string()
                        } else {
                            guid
                        };
                        let title = item
                            .get("title")
                            .and_then(|v| v.as_str())
                            .unwrap_or("(untitled)")
                            .to_string();
                        let link = item
                            .get("url")
                            .or_else(|| item.get("external_url"))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        let author = item
                            .get("author")
                            .and_then(|a| a.get("name"))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        let summary = item
                            .get("summary")
                            .or_else(|| item.get("content_text"))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        let content = item
                            .get("content_html")
                            .or_else(|| item.get("content_text"))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        let published_at = item
                            .get("date_published")
                            .or_else(|| item.get("published_at"))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        let fingerprint = Self::compute_fingerprint(&guid, &title, &link);

                        Item {
                            id: uuid::Uuid::new_v4().to_string(),
                            feed_id: feed_id.to_string(),
                            guid,
                            title,
                            link,
                            author,
                            summary,
                            content,
                            published_at,
                            updated_at: None,
                            fingerprint: Some(fingerprint),
                            created_at: now.clone(),
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(ParsedItems { items })
    }

    fn compute_fingerprint(guid: &str, title: &str, link: &Option<String>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(guid.as_bytes());
        hasher.update(title.as_bytes());
        if let Some(link) = link {
            hasher.update(link.as_bytes());
        }
        format!("{:x}", hasher.finalize())
    }
}
