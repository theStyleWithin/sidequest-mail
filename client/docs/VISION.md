# Vision

An offline-first email client that is **yours**: your mail lives on your device,
in open formats, and no provider, company, or moderator can take it away.

## Who it is for
1. **Me, first.** Peace of mind: my mail history survives any account ban.
2. **Proton Mail users and privacy-minded people**, who care about ownership
   and tracking protection.
3. **Gamers**, who want a themeable, lightweight client that stays out of the way.

## Principles
- Local-first: a full mirror of every account, usable with no network.
- No account, no telemetry, no update server, no central community hub.
- Open formats (raw MIME + SQLite). Export is always possible.
- Portable identity: own your domain, switch providers by changing settings.
- Plain IMAP/SMTP. Proton works through Proton Mail Bridge (paid plan) as one
  source among many, not a dependency.
- Small audited core; themes and game-style features are an optional layer on top.

## Unbannable by design
The app has no online account, no login, and no server run by its author, so
there is nothing to ban you from. Nobody, including the author, can revoke
your access, lock your data, or switch the app off. It runs on your device and
keeps working offline.

Your email provider is a separate thing: it still runs your mail address and
delivery. The app is built so that never matters much. Your full mail history
lives on your device, and you can move to another provider by changing
settings, so a provider's decision cannot take your mail or the app away.

## Transparency about delivery
Until the app has its own way to send mail (such as the peer-to-peer path),
**your email provider delivers your mail**. It is the third party that sends,
receives, and can see unencrypted messages on its servers, and it can suspend
your address. The app will always say so plainly:

- Each account shows who delivers its mail (provider and server).
- Each message shows how it travelled: via your provider, or directly between
  peers.
- Nothing is routed through the author or any server the author runs.
- The app makes no promises about a provider's behavior, only about what runs
  on your device.

## What we are not building (for now)
Calendar, contacts sync, webmail, Discord/social integration, PGP/S-MIME, JMAP.
