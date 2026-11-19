//! Turns a `garden_core` theme into egui styling.
use eframe::egui::{self, Color32, FontFamily, FontId};
use garden_core::theme::{Theme, hex_to_rgb};

fn c(hex: &str) -> Color32 {
    let [r, g, b] = hex_to_rgb(hex).unwrap_or([128, 128, 128]);
    Color32::from_rgb(r, g, b)
}

/// Perceived brightness, to decide light vs dark base visuals.
fn is_dark(hex: &str) -> bool {
    let [r, g, b] = hex_to_rgb(hex).unwrap_or([0, 0, 0]);
    (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) < 128.0
}

pub fn apply(ctx: &egui::Context, t: &Theme) {
    let col = &t.colors;
    let mut v = if is_dark(&col.background) { egui::Visuals::dark() } else { egui::Visuals::light() };
    v.panel_fill = c(&col.background);
    v.window_fill = c(&col.surface);
    v.extreme_bg_color = c(&col.surface);
    v.faint_bg_color = c(&col.surface);
    v.override_text_color = Some(c(&col.text));
    v.hyperlink_color = c(&col.accent);
    v.selection.bg_fill = c(&col.accent);
    v.selection.stroke.color = c(&col.background);
    v.widgets.noninteractive.bg_fill = c(&col.surface);
    v.widgets.inactive.bg_fill = c(&col.surface);
    v.widgets.inactive.weak_bg_fill = c(&col.surface);
    v.widgets.hovered.bg_fill = c(&col.accent);
    v.widgets.hovered.weak_bg_fill = c(&col.accent);
    v.widgets.active.bg_fill = c(&col.accent);
    ctx.set_visuals(v);

    let family = match t.font.as_str() {
        "monospace" => FontFamily::Monospace,
        _ => FontFamily::Proportional, // serif/sans-serif: egui ships one proportional face
    };
    ctx.style_mut(|s| s.override_font_id = Some(FontId::new(15.0, family)));
}

pub fn accent(t: &Theme) -> Color32 {
    c(&t.colors.accent)
}
pub fn unread(t: &Theme) -> Color32 {
    c(&t.colors.unread)
}
