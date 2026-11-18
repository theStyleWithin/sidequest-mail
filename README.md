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
