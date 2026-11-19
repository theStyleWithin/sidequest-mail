# Plan

## Phases
**Phase 1: email client manager (now).** A local-first client that signs into
existing accounts (Gmail and any provider that allows third-party clients) over
IMAP/SMTP, mirrors them to your device, and manages them in one place.
Providers deliver the mail; the app is transparent about that.

**Phase 2: peer-to-peer sending (later).** An optional direct path between
people who both run the app, so mail needs no provider at all. Planned, not
started; see the sidequest-mail roadmap.

## Build order (Phase 1)
1. **Local store.** SQLite schema: accounts, folders, messages (raw MIME + parsed
   fields), threads, outbox. Encrypted at rest.
2. **IMAP mirror (read-only).** One account, one-way sync into the store.
   Handle UIDVALIDITY changes, moves and deletions. Test against a local
   IMAP server (Dovecot or GreenMail) and a messy-MIME corpus.
3. **Offline outbox + SMTP send.** Compose and queue offline; send on reconnect.
4. **Account sign-in.** App passwords / Proton Bridge first; OAuth for Gmail
   and Microsoft once the open question below is settled.
5. **Minimal UI.** Inbox list, reading view, composer. Sandboxed HTML rendering,
   remote images blocked by default. Shows who delivers each account and how
   each message travelled (see `VISION.md`).
6. **Privacy features.** Tracker report, alias awareness, Proton Bridge testing.
7. **Theme system, then optional game-style layer** (themes as plain files).
8. **Release readiness.** License, signed/reproducible builds, docs.

## Risk: providers blocking third-party clients
Gmail and Microsoft require OAuth for third-party clients, and Google treats
full mailbox access as a restricted scope. For personal use, an unverified app
in testing mode works but with limits (such as short-lived tokens and a small
user cap); a public release would need verification and possibly a security
assessment. The app has no server, so it would use a desktop OAuth flow with
PKCE and an embedded client ID. Plan the first version around providers that
allow plain IMAP/SMTP with app passwords, and treat Gmail OAuth as a separate
milestone. Verify current provider rules before building.

## Decisions still open
- **Stack:** DECIDED: Rust core (lightweight, fast). UI toolkit still open.
- **First provider:** DECIDED: Proton Mail via Bridge first (plain IMAP/SMTP on
  localhost, no OAuth), then Gmail with an app password, then Gmail OAuth.
  Proton needs a paid plan and Bridge running.
- **Current provider, OS, mailbox size** (drives sync edge cases).
- **Key storage:** OS keychain vs passphrase-derived key.
- **Gmail/Microsoft OAuth:** which milestone, and whether a public release will go through provider verification.
- **License and name** (sidequest-mail is MIT).

## Done so far
- `src/store.rs`: SQLite schema with tests.
- `src/theme.rs` + `themes/*.toml`: 7 swappable TOML themes (synthwave, fantasy
  RPG, retro console, terminal hacker, cyberpunk, kawaii boba, minimalist),
  validated by tests.

## Next step
IMAP mirror prototype against Proton Bridge / a local test server.

## Original first step
Define the SQLite schema and write a tiny IMAP-mirror prototype against a
local test server. No UI yet.
