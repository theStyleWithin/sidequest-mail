//! Turns a `garden_core` theme into egui styling. One font family, one spacing grid (8px).
use eframe::egui::{self, Color32, FontFamily, FontId, Rounding, TextStyle, vec2};
use garden_core::theme::{Theme, hex_to_rgb};

pub fn c(hex: &str) -> Color32 {
    let [r, g, b] = hex_to_rgb(hex).unwrap_or([128, 128, 128]);
    Color32::from_rgb(r, g, b)
}

fn is_dark(hex: &str) -> bool {
    let [r, g, b] = hex_to_rgb(hex).unwrap_or([0, 0, 0]);
    (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) < 128.0
}

pub fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let l = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgb(l(a.r(), b.r()), l(a.g(), b.g()), l(a.b(), b.b()))
}

fn luminance(c: Color32) -> f32 {
    let ch = |v: u8| {
        let v = v as f32 / 255.0;
        if v <= 0.03928 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * ch(c.r()) + 0.7152 * ch(c.g()) + 0.0722 * ch(c.b())
}

fn contrast(a: Color32, b: Color32) -> f32 {
    let (x, y) = (luminance(a), luminance(b));
    (x.max(y) + 0.05) / (x.min(y) + 0.05)
}

/// `fg` if it is readable on `bg` (WCAG 4.5:1), otherwise the theme's plain text colour.
pub fn readable(fg: Color32, bg: Color32, fallback: Color32) -> Color32 {
    if contrast(fg, bg) >= 4.5 { fg } else { fallback }
}

/// Colors the painter code needs, derived once per theme.
#[derive(Clone, Copy)]
pub struct Palette {
    pub bg: Color32,
    pub surface: Color32,
    pub text: Color32,
    pub weak: Color32,
    pub accent: Color32,
    pub unread: Color32,
    pub line: Color32,
    pub selected: Color32,
    pub hover: Color32,
}

pub fn palette(t: &Theme) -> Palette {
    let col = &t.colors;
    let (bg, surface, text, accent) = (c(&col.background), c(&col.surface), c(&col.text), c(&col.accent));
    Palette {
        bg,
        surface,
        text,
        weak: mix(text, bg, 0.45),
        accent,
        unread: c(&col.unread),
        line: mix(bg, text, 0.14),
        selected: mix(surface, accent, 0.22),
        hover: mix(bg, surface, 0.7),
    }
}

pub fn apply(ctx: &egui::Context, t: &Theme) {
    let p = palette(t);
    let mut v = if is_dark(&t.colors.background) { egui::Visuals::dark() } else { egui::Visuals::light() };
    v.panel_fill = p.bg;
    v.window_fill = p.surface;
    v.extreme_bg_color = p.surface;
    v.faint_bg_color = p.surface;
    v.override_text_color = Some(p.text);
    v.hyperlink_color = p.accent;
    v.selection.bg_fill = p.selected;
    v.selection.stroke.color = p.text;
    v.window_rounding = Rounding::same(8.0);
    for w in [&mut v.widgets.noninteractive, &mut v.widgets.inactive, &mut v.widgets.hovered, &mut v.widgets.active, &mut v.widgets.open] {
        w.rounding = Rounding::same(6.0);
    }
    v.widgets.noninteractive.bg_fill = p.surface;
    v.widgets.noninteractive.bg_stroke.color = p.line;
    let control = mix(p.surface, p.text, 0.10);
    v.widgets.inactive.bg_fill = control;
    v.widgets.inactive.weak_bg_fill = control;
    v.widgets.inactive.bg_stroke = egui::Stroke::new(1.0_f32, mix(p.bg, p.text, 0.28));
    v.widgets.hovered.bg_fill = p.selected;
    v.widgets.hovered.weak_bg_fill = p.selected;
    v.widgets.hovered.bg_stroke = egui::Stroke::new(1.0_f32, p.accent);
    v.widgets.active.bg_fill = p.accent;
    v.widgets.active.weak_bg_fill = p.accent;
    v.widgets.active.fg_stroke.color = p.bg;
    ctx.set_visuals(v);

    let family = match t.font.as_str() {
        "monospace" => FontFamily::Monospace,
        _ => FontFamily::Proportional, // serif/sans-serif: egui ships one proportional face
    };
    ctx.style_mut(|s| {
        let f = |size: f32| FontId::new(size, family.clone());
        s.text_styles = [
            (TextStyle::Heading, f(20.0)),
            (TextStyle::Body, f(14.0)),
            (TextStyle::Button, f(14.0)),
            (TextStyle::Small, f(12.0)),
            (TextStyle::Monospace, FontId::new(13.0, FontFamily::Monospace)),
        ]
        .into();
        s.override_font_id = None;
        s.spacing.item_spacing = vec2(8.0, 8.0);
        s.spacing.button_padding = vec2(16.0, 8.0);
        s.spacing.interact_size.y = 32.0;
        s.spacing.window_margin = egui::Margin::same(16.0);
    });
}
