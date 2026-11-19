# Sidequest Mail

> Your inbox is not a raid boss. Treat it like a side quest.

No account. No telemetry. No update server. No ban. MIT.

Sidequest Mail is a lightweight, offline-first email client manager written in
Rust. Your mail lives on your device, in open formats, and keeps working with no
network. There is no Sidequest account and no Sidequest server, so there is
nothing for anyone, including the author, to ban you from.

> **Status: early.** The Rust core (local store and themes) exists. IMAP sync,
> sending, and the UI are not built yet.

## How it works

```
 your device
 ┌──────────────────────────────────────────┐
 │  Local store (SQLite)   Outbox (offline) │
 │            │                  │          │
 │            └────────┬─────────┘          │
 │                     ▼                    │
 │              Email worker                │
 │               (IMAP / SMTP)              │
 └─────────────────────┼────────────────────┘
                       ▼
          your mail provider
   (Proton Mail via Bridge, Gmail, any IMAP/SMTP host)
```

- **Local store is the source of truth.** Every account is mirrored to your
  device. If a provider closes your address, your history, contacts, and search
  stay with you, and moving to another provider is a settings change.
- **Offline always works.** Compose and queue with no network; mail is sent when
  a connection returns.
- **Open formats.** Original messages are kept as raw MIME in SQLite. Export is
  always possible.

## Who delivers your mail

Until peer-to-peer sending exists, **your email provider delivers your mail**.
It sends and receives it, can see unencrypted messages on its servers, and can
suspend your address. Sidequest Mail never routes anything through the author or
any server the author runs, and it will always show which provider delivers each
account and how each message travelled. See [client/docs/VISION.md](client/docs/VISION.md).

## Roadmap

1. **Phase 1, email client manager (now).** Sign in to existing accounts and
   manage them in one place. Proton Mail through Bridge first (plain IMAP/SMTP,
   paid Proton plan required), then Gmail with an app password, then Gmail OAuth.
2. **Phase 2, peer-to-peer sending (later).** An optional direct path between
   people who both run the app, with no mail provider involved. Planned, not
   started. It stays fully independent of the email path, so a failure in one
   cannot affect the other.

Details and build order: [client/docs/PLAN.md](client/docs/PLAN.md).

## Themes

The interface is swappable, with game-inspired themes as plain TOML files:
synthwave, fantasy RPG, retro console, terminal hacker, cyberpunk, kawaii boba
tea, and a quiet minimalist default. See [client/themes/](client/themes/).

## Build and test

```bash
cd client
cargo test
```

## Principles

- No account, no telemetry, no update server, no ban.
- Local-first and offline-first.
- Small auditable core; themes and game-style features are an optional layer.
- Honest about what depends on third parties (your provider) and what does not.

The original architecture and threat-model notes are kept in
[brainstorm/architecture-and-trust.md](brainstorm/architecture-and-trust.md).

## License

MIT.
