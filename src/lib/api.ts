import { invoke } from "@tauri-apps/api/core";
import type { Feed, ItemWithState, Rule, SyncLog, Credential, AppSettings } from "./types";

// Feed commands
export async function getFeeds(): Promise<Feed[]> {
  return invoke("get_feeds");
}

export async function addFeed(feed: Omit<Feed, "id" | "created_at" | "updated_at" | "last_checked_at" | "last_success_at" | "last_error">): Promise<Feed> {
  return invoke("add_feed", { feed });
}

export async function updateFeed(feed: Feed): Promise<Feed> {
  return invoke("update_feed", { feed });
}

export async function deleteFeed(id: string): Promise<void> {
  return invoke("delete_feed", { id });
}

export async function syncFeed(id: string): Promise<SyncLog> {
  return invoke("sync_feed", { id });
}

// Item commands
export async function getItems(feedId?: string, opts?: { unreadOnly?: boolean; starredOnly?: boolean; limit?: number; offset?: number }): Promise<ItemWithState[]> {
  return invoke("get_items", { feedId, opts });
}

export async function markRead(itemId: string): Promise<void> {
  return invoke("mark_read", { itemId });
}

export async function markUnread(itemId: string): Promise<void> {
  return invoke("mark_unread", { itemId });
}

export async function toggleStar(itemId: string): Promise<void> {
  return invoke("toggle_star", { itemId });
}

// Rule commands
export async function getRules(): Promise<Rule[]> {
  return invoke("get_rules");
}

export async function addRule(rule: Omit<Rule, "id" | "created_at" | "updated_at">): Promise<Rule> {
  return invoke("add_rule", { rule });
}

export async function deleteRule(id: string): Promise<void> {
  return invoke("delete_rule", { id });
}

// Credential commands
export async function getCredentials(): Promise<Credential[]> {
  return invoke("get_credentials");
}

export async function saveCredential(credential: Omit<Credential, "id" | "created_at" | "updated_at">, secret: string): Promise<Credential> {
  return invoke("save_credential", { credential, secret });
}

export async function deleteCredential(id: string): Promise<void> {
  return invoke("delete_credential", { id });
}

// Settings commands
export async function getSettings(): Promise<AppSettings> {
  return invoke("get_settings");
}

export async function saveSettings(settings: AppSettings): Promise<void> {
  return invoke("save_settings", { settings });
}
