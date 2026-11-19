//! Themes are plain TOML files: no code, easy to share, easy to audit.
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Theme {
    pub name: String,
    pub description: String,
    pub font: String,
    pub colors: Colors,
    /// Optional game-style flavour text; the UI falls back to plain words.
    #[serde(default)]
    pub labels: Labels,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Colors {
    pub background: String,
    pub surface: String,
    pub text: String,
    pub accent: String,
    pub unread: String,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Labels {
    pub inbox: Option<String>,
    pub sent: Option<String>,
    pub compose: Option<String>,
    pub archive: Option<String>,
}

fn is_hex(c: &str) -> bool {
    c.len() == 7 && c.starts_with('#') && c[1..].chars().all(|x| x.is_ascii_hexdigit())
}

impl Theme {
    pub fn parse(src: &str) -> Result<Theme, String> {
        let t: Theme = toml::from_str(src).map_err(|e| e.to_string())?;
        for (k, v) in [
            ("background", &t.colors.background),
            ("surface", &t.colors.surface),
            ("text", &t.colors.text),
            ("accent", &t.colors.accent),
            ("unread", &t.colors.unread),
        ] {
            if !is_hex(v) {
                return Err(format!("colors.{k} must be #RRGGBB, got {v:?}"));
            }
        }
        Ok(t)
    }
}

/// Built-in themes, embedded so the app works with no files and no network.
pub const BUILTIN: &[(&str, &str)] = &[
    ("synthwave", include_str!("../themes/synthwave.toml")),
    ("fantasy-rpg", include_str!("../themes/fantasy-rpg.toml")),
    ("retro-console", include_str!("../themes/retro-console.toml")),
    ("terminal-hacker", include_str!("../themes/terminal-hacker.toml")),
    ("cyberpunk", include_str!("../themes/cyberpunk.toml")),
    ("kawaii-boba", include_str!("../themes/kawaii-boba.toml")),
    ("minimalist", include_str!("../themes/minimalist.toml")),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_builtin_themes_are_valid() {
        for (id, src) in BUILTIN {
            Theme::parse(src).unwrap_or_else(|e| panic!("{id}: {e}"));
        }
    }

    #[test]
    fn rejects_bad_color() {
        let bad = BUILTIN[0].1.replace("#", "").replacen("background = \"", "background = \"red", 1);
        assert!(Theme::parse(&bad).is_err());
    }
}
