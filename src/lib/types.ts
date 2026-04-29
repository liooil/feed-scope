export type FeedKind = "rss" | "atom" | "json" | "jira" | "gitlab";

export type Feed = {
  id: string;
  name: string;
  url: string;
  kind: FeedKind;
  category: string;
  enabled: boolean;
  refresh_interval_seconds: number;
  credential_id: string | null;
  created_at: string;
  updated_at: string;
  last_checked_at: string | null;
  last_success_at: string | null;
  last_error: string | null;
};

export type Item = {
  id: string;
  feed_id: string;
  guid: string;
  title: string;
  link: string | null;
  author: string | null;
  summary: string | null;
  content: string | null;
  published_at: string | null;
  updated_at: string | null;
  fingerprint: string | null;
  created_at: string;
};

export type ItemState = {
  item_id: string;
  read: boolean;
  starred: boolean;
  hidden: boolean;
  notified: boolean;
  archived: boolean;
  updated_at: string;
};

export type ItemWithState = Item & {
  state: ItemState;
  feed_name: string;
  feed_kind: FeedKind;
};

export type Rule = {
  id: string;
  name: string;
  enabled: boolean;
  feed_id: string | null;
  condition_json: string;
  action_json: string;
  created_at: string;
  updated_at: string;
};

export type SyncLog = {
  id: string;
  feed_id: string;
  started_at: string;
  finished_at: string | null;
  status: "running" | "success" | "error";
  items_found: number;
  items_new: number;
  error_message: string | null;
};

export type Credential = {
  id: string;
  name: string;
  auth_type: "none" | "basic" | "bearer" | "header";
  created_at: string;
  updated_at: string;
};

export type AppSettings = {
  sync_interval_default: number;
  notifications_enabled: boolean;
  quiet_hours_start: string | null;
  quiet_hours_end: string | null;
  max_items_per_feed: number;
};
