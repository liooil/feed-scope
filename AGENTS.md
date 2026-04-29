# AGENTS.md

## Project Overview

This project is a local-first desktop RSS / developer monitoring client built with:

* Tauri v2
* React + Vite
* TypeScript
* Tailwind CSS
* shadcn/ui
* SQLite
* Rust-side background workers

The app is intended to monitor authenticated/internal feeds such as:

* Jira issues from self-hosted Jira instances
* GitLab merge requests
* GitLab pipeline status
* Other RSS / Atom / JSON feeds

The product goal is closer to a **developer event monitor** than a traditional casual RSS reader.

Core priorities:

1. Local-first data ownership
2. Reliable desktop notifications
3. Secure credential handling
4. Good UI/UX for engineering workflows
5. Extensible feed parsing and filtering

---

## Architecture Principles

### 1. Keep React as the UI layer only

React should be responsible for:

* Rendering views
* Managing local UI state
* Forms and settings screens
* Calling Tauri commands
* Displaying synchronized data

React should **not** be responsible for:

* Periodic background feed fetching
* Storing secrets
* Directly writing complex persistence logic
* Long-running sync jobs
* Notification decisions

Those belong on the Rust/Tauri side.

---

### 2. Rust owns system integration and background work

Rust/Tauri should handle:

* Feed fetching
* Authentication headers
* Credential access
* SQLite access
* Background sync scheduling
* Feed parsing
* Deduplication
* Notification triggering
* OS-level integrations

Expose clean Tauri commands to the frontend.

Preferred pattern:

```text
React UI
  ↓ invoke()
Tauri command
  ↓
Rust service layer
  ↓
SQLite / network / notification / credential store
```

---

### 3. SQLite is the source of truth

Use SQLite for persistent application data.

Avoid using IndexedDB or localStorage as the primary database.

Allowed frontend storage:

* temporary UI preferences
* ephemeral view state
* non-sensitive cached UI state

Not allowed in frontend storage:

* passwords
* tokens
* full feed database
* notification state
* sync metadata

---

## Suggested Project Structure

```text
src/
  app/
    App.tsx
    routes.tsx
  components/
    ui/
    layout/
    feed/
    item/
    rules/
  features/
    feeds/
    items/
    rules/
    settings/
    notifications/
  lib/
    api.ts
    types.ts
    utils.ts

src-tauri/
  src/
    main.rs
    commands/
      feeds.rs
      items.rs
      rules.rs
      settings.rs
      sync.rs
    services/
      feed_fetcher.rs
      feed_parser.rs
      sync_engine.rs
      notification_engine.rs
      credential_store.rs
    db/
      mod.rs
      migrations.rs
      repositories/
    models/
      feed.rs
      item.rs
      rule.rs
      credential.rs
```

This structure is a guideline, not a strict rule. Prefer clarity over over-abstraction.

---

## Database Design Guidelines

Use normalized SQLite tables.

Recommended core tables:

```text
feeds
items
item_states
credentials
rules
sync_logs
notification_logs
```

### feeds

Stores subscription metadata.

Suggested fields:

```text
id
name
url
kind
category
enabled
refresh_interval_seconds
created_at
updated_at
last_checked_at
last_success_at
last_error
```

### items

Stores objective feed content.

Suggested fields:

```text
id
feed_id
guid
title
link
author
summary
content
published_at
updated_at
fingerprint
created_at
```

### item_states

Stores user/app state separately from content.

Suggested fields:

```text
item_id
read
starred
hidden
notified
archived
updated_at
```

### rules

Rules decide filtering and notification behavior.

Suggested fields:

```text
id
name
enabled
feed_id nullable
condition_json
action_json
created_at
updated_at
```

---

## Feed Fetching Rules

Feed fetching must happen in Rust, not React.

The sync engine should:

1. Load enabled feeds from SQLite
2. Fetch each feed with proper authentication
3. Parse RSS / Atom / JSON Feed
4. Normalize items
5. Deduplicate by GUID, link, or fingerprint
6. Insert new items
7. Evaluate rules
8. Trigger notifications when needed
9. Record sync logs

Do not notify repeatedly for the same item. Use `item_states.notified` or `notification_logs`.

---

## Authentication and Secrets

Credentials must not be stored in frontend code, localStorage, IndexedDB, or plaintext config files.

Preferred storage:

* OS keychain if available
* Tauri-compatible secure storage plugin
* encrypted local storage as a fallback

Credential records in SQLite should only reference credential IDs, not store raw secrets.

Example:

```text
feeds.credential_id → credential_store lookup
```

Supported auth modes should include:

* no auth
* Basic Auth
* Bearer token
* custom headers

Future auth modes may include:

* cookie-based auth
* GitLab personal access token
* Jira personal access token
* proxy auth

---

## Notification Guidelines

Notifications are a core feature.

Notification decisions should be made by the Rust notification engine, not React.

The notification engine should consider:

* Whether the item is new
* Whether it already notified
* Matching rules
* Feed-level notification settings
* Quiet hours, if implemented
* Priority level

Notification payloads should be concise:

```text
Title: [Jira] BUG-123: Login fails
Body: High priority · Assigned to you · Updated 3 min ago
Action: Open item detail or original link
```

Do not notify for every item by default if a feed is noisy. Prefer rule-based notifications.

---

## Rule Engine Guidelines

The rule engine should support simple conditions first.

Start with:

* title contains
* content contains
* URL contains
* feed equals
* category equals
* regex match
* published/updated time window

Actions:

* notify
* mark as important
* hide
* assign label
* set priority

Store rules as JSON for flexibility, but expose them through typed Rust and TypeScript models.

Example conceptual rule:

```json
{
  "conditions": [
    { "field": "feed_id", "op": "equals", "value": "jira-bugs" },
    { "field": "title", "op": "contains", "value": "High" }
  ],
  "actions": [
    { "type": "notify" },
    { "type": "set_priority", "value": "high" }
  ]
}
```

---

## UI Guidelines

Use a modern developer-tool style.

Recommended layout:

```text
Sidebar: feeds, categories, smart views
Main panel: event/item list
Detail panel: selected item content and actions
Top bar: search, filters, refresh, unread count
```

Prioritize:

* high information density
* keyboard navigation
* fast filtering
* clear unread/important states
* readable typography
* dark mode support

Use shadcn/ui components when possible.

Prefer:

* Button
* Card
* Dialog
* Sheet
* DropdownMenu
* Command
* Badge
* Tabs
* Tooltip
* ScrollArea
* Separator
* Input
* Select
* Switch

Avoid overly decorative UI. This is a monitoring tool.

---

## TypeScript Guidelines

Use strict TypeScript.

Prefer explicit domain types:

```ts
export type Feed = {
  id: string;
  name: string;
  url: string;
  kind: "rss" | "atom" | "json" | "jira" | "gitlab";
  enabled: boolean;
};
```

Avoid `any` unless there is a strong reason.

Use Zod or equivalent validation for data crossing the Tauri boundary if needed.

---

## Rust Guidelines

Keep Rust modules small and domain-focused.

Prefer:

* typed errors
* clear service boundaries
* repository pattern for database access
* migration-based schema changes
* async networking

Avoid putting all logic inside Tauri command handlers.

Bad:

```text
command handler → fetch → parse → db → notify all inline
```

Good:

```text
command handler → sync_engine.run_feed_sync(feed_id)
```

---

## Error Handling

Errors should be visible and actionable.

For feed sync errors, store:

* feed ID
* timestamp
* error type
* human-readable message
* optional technical details

In the UI, show concise messages such as:

```text
Jira Bugs failed to sync: 401 Unauthorized. Check credentials.
```

Do not hide repeated failures silently.

---

## Performance Guidelines

RSS clients can accumulate a lot of data.

Plan for:

* thousands of feeds/items
* fast unread filtering
* incremental sync
* pagination or virtualized lists
* database indexes

Recommended indexes:

```sql
CREATE INDEX idx_items_feed_id ON items(feed_id);
CREATE INDEX idx_items_published_at ON items(published_at);
CREATE INDEX idx_items_guid ON items(guid);
CREATE INDEX idx_item_states_read ON item_states(read);
```

Use virtualized rendering for large item lists.

---

## Security Guidelines

Never log raw credentials.

Do not display passwords or tokens after saving.

When exporting app data, exclude secrets by default.

When opening external links, use the OS default browser safely.

Validate feed URLs before fetching.

Consider SSRF-like risks if users can add arbitrary URLs, especially in enterprise networks.

---

## Testing Guidelines

Prioritize tests for:

* feed parsing
* item deduplication
* rule matching
* notification suppression
* credential lookup
* database migrations

Use fixture files for RSS/Atom/Jira/GitLab feed examples.

Recommended fixture categories:

```text
fixtures/rss/basic.xml
fixtures/rss/missing-guid.xml
fixtures/atom/basic.xml
fixtures/jira/issues.xml
fixtures/gitlab/merge-requests.atom
fixtures/gitlab/pipelines.xml
```

---

## Development Commands

Use these as placeholders and update when the project scripts are finalized.

```bash
npm install
npm run dev
npm run tauri dev
npm run build
npm run tauri build
```

For Rust checks:

```bash
cd src-tauri
cargo fmt
cargo clippy
cargo test
```

---

## Implementation Priorities

Build in this order:

1. Basic Tauri + React shell
2. SQLite schema and migrations
3. Add/edit/delete feed
4. Manual feed sync
5. RSS/Atom parsing
6. Item list and detail view
7. Read/unread state
8. Basic Auth credential support
9. Background sync
10. Desktop notifications
11. Rules engine
12. Jira/GitLab-specific presets
13. Search and filters
14. Import/export OPML
15. Polish and packaging

---

## Product Direction

This app should not become a generic clone of Feedly.

The product niche is:

```text
A modern local-first desktop event monitor for developers.
```

Optimize for:

* internal tools
* authenticated feeds
* noisy engineering systems
* reliable notifications
* fast triage

Primary user workflows:

* “Show me new bugs assigned to me.”
* “Notify me when a GitLab pipeline fails.”
* “Show merge requests that need my review.”
* “Keep a searchable local history of these events.”

---

## Agent Instructions

When modifying this project:

1. Preserve the local-first architecture.
2. Do not move feed fetching into React.
3. Do not store secrets in frontend storage.
4. Prefer SQLite for durable app data.
5. Keep notification logic deterministic and deduplicated.
6. Add migrations for schema changes.
7. Keep UI components accessible and keyboard-friendly.
8. Use shadcn/ui and Tailwind instead of ad-hoc styling.
9. Avoid introducing heavy dependencies without clear value.
10. Keep enterprise/internal-network use cases in mind.

When unsure, favor reliability and debuggability over clever abstractions.
