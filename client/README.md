# Open Garden (sandbox)

An empty starting point for a new open-garden software project.
Open garden = the opposite of a walled garden: open formats, no lock-in,
user-owned data, interoperable by default.

> Status: early. Rust core with a SQLite store and swappable themes; no IMAP yet.
> Build and test: `cargo test`.

## Layout

- `docs/`  – vision, principles, decisions (see `docs/PRINCIPLES.md`)
- `src/`   – Rust core (`store`, `theme`)
- `ui/`    – desktop UI (egui): accounts, folders, message list, reader, compose, theme switcher
- `themes/` – swappable TOML themes
- `tests/` – tests

## Next steps

1. Decide what the project actually is and fill in `docs/VISION.md`.
2. Pick a stack and add build/test commands here.
3. Choose a license (none added yet on purpose).

## Run the UI (demo data)

```bash
cargo run -p garden_ui
GARDEN_THEME=fantasy-rpg cargo run -p garden_ui   # start in another theme
```

On Linux you need the usual X11/Wayland and OpenGL libraries. The UI shows demo
messages only; IMAP sync is not built yet.
