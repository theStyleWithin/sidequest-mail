# Sidequest Mail — Architecture & Trust

> Your inbox is not a raid boss. Treat it like a side quest.

No account. No telemetry. No update server. No ban. MIT.

---

## Architecture

Two independent send paths. One local store. No Sidequest-operated server.
   
┌─────────────────────────────────────────────────┐
│ Local Account (per-device, no cloud identity) │
│ │
│ ┌───────────┐ ┌──────────────────────┐ │
│ │ SQLite DB │ │ Outbox (local queue) │ │
│ └───────────┘ └────────┬─────────────┘ │
│ │ │
│ ┌─────────────┴─────────────┐ │
│ │ │ │
│ ▼ ▼ │
│ ┌────────────────┐ ┌────────────────┐│
│ │ Email Worker │ │ Sidequest Worker││
│ │ (SMTP/IMAP) │ │ (Tailcat P2P) ││
│ └───────┬────────┘ └───────┬────────┘│
└─────────────┼───────────────────────────┼────────┘
│ │
▼ ▼
User's mail provider Peer's device
(Gmail, Fastmail, (via WireGuard tunnel,
any IMAP/SMTP host) DERP fallback)

# Sidequest Mail — Messaging Infrastructure Roadmap

> See [Architecture & Trust](./architecture%20and%20trust) for threat model, design rules, and non-goals.

## Goal

Local-first messaging with two independent send paths. No Sidequest-operated server.

- **Email path:** local store → IMAP in / SMTP out (user's mail provider)
- **Sidequest path (experimental):** local store → tailcat-rs peer-to-peer (no mail provider)

The two paths share a local SQLite store but never share a network connection or a failure mode.

---

## Principles

1. Local store is the source of truth. Network is a sync mechanism.
2. The two paths are independent. `channel=email` or `channel=sidequest`. Never both.
3. Offline compose always works. Delivery happens when the path is open.
4. If tailcat-rs panics, the email path is unaffected. (Trait isolation — see Phase 4.)
5. No OAuth in v1. App passwords + IMAP/SMTP credentials only.

---

## Phase 0 — Definitions

| Term | Meaning |
|------|---------|
| **Local account** | Per-device profile. Display name, optional mail credentials, optional sidequest keypair. Not a cloud identity. |
| **Mail identity** | IMAP/SMTP host, port, username. Password in OS keyring. |
| **Sidequest identity** | tailcat-rs keypair + `tc…` address. Stored in account data dir, `0600`. |
| **Outbox** | Local queue of pending sends. Rows reference a `messages` row + `channel`. |
| **Conversation** | Local thread. May mix channels in the UI. Backends stay separate. |

---

## Phase 1 — Local account + store

**Outcome:** App creates a local account, persists messages, composes offline. No network.

### 1.1 Account model
- Create / switch / eject. Per-account data directory.
- Fields: `id`, `display_name`, `created_at`, optional `avatar_path`.
- No password for local-only use.

### 1.2 SQLite schema

```sql
-- One DB per local account: {data_dir}/{account_id}/mail.db

CREATE TABLE messages (
    id          INTEGER PRIMARY KEY,
    direction   TEXT NOT NULL,       -- 'in' | 'out'
    channel     TEXT NOT NULL,       -- 'email' | 'sidequest'
    peer        TEXT,                -- email address or tc… address
    subject     TEXT,
    body_html   TEXT,
    body_text   TEXT,
    state       TEXT NOT NULL,       -- 'draft'|'queued'|'sending'|'sent'|'failed'|'received'
    created_at  TEXT NOT NULL,
    sent_at     TEXT,
    error       TEXT,                -- last error text if state='failed'
    meta_json   TEXT                 -- MIME headers, flags, etc.
);

CREATE TABLE outbox (
    id              INTEGER PRIMARY KEY,
    message_id      INTEGER NOT NULL REFERENCES messages(id),
    channel         TEXT NOT NULL,
    payload_json    TEXT,            -- channel-specific send data
    attempts        INTEGER DEFAULT 0,
    next_attempt_at TEXT,
    state           TEXT NOT NULL    -- 'pending'|'in_flight'|'done'|'dead'
);

CREATE TABLE peers (
    id           INTEGER PRIMARY KEY,
    kind         TEXT NOT NULL,      -- 'email' | 'sidequest'
    address      TEXT NOT NULL,
    display_name TEXT,
    meta_json    TEXT
);

CREATE TABLE sync_state (
    mailbox       TEXT PRIMARY KEY,  -- e.g. 'INBOX', 'Sent'
    last_uid      INTEGER DEFAULT 0,
    last_sync_at  TEXT
);

CREATE INDEX idx_messages_channel_state ON messages(channel, state);
CREATE INDEX idx_messages_peer ON messages(peer);
CREATE INDEX idx_outbox_state ON outbox(state, next_attempt_at);   
