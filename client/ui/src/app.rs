use crate::demo::{self, Via};
use crate::style::{self, Palette};
use eframe::egui::{self, Align, Align2, Color32, FontId, Frame, Layout, Margin, Pos2, Rect, Sense, TextStyle, Ui, text::LayoutJob, vec2};
use garden_core::theme::{BUILTIN, Labels, Theme};
use std::sync::Arc;

// Everything is laid out on an 8px grid with fixed sizes, so nothing jitters or wraps.
const TOP_H: f32 = 48.0;
const STATUS_H: f32 = 32.0;
const SIDE_W: f32 = 240.0;
const LIST_W: f32 = 376.0;
const ACCOUNT_H: f32 = 56.0;
const FOLDER_H: f32 = 32.0;
const LIST_HEAD_H: f32 = 48.0;
const ROW_H: f32 = 72.0;
const PAD: f32 = 16.0;

struct Compose {
    from: usize,
    to: String,
    subject: String,
    body: String,
}

#[allow(dead_code)] // shown/persisted once the outbox is wired to the store
struct Queued {
    from: String,
    to: String,
    subject: String,
}

enum Action {
    Folder(usize, usize),
    Select(usize),
    Archive,
    Reply,
}

pub struct GardenApp {
    themes: Vec<(&'static str, Theme)>,
    current: usize,
    applied: Option<usize>,
    accounts: Vec<demo::Account>,
    acct: usize,
    folder: usize,
    msg: Option<usize>,
    query: String,
    compose: Option<Compose>,
    outbox: Vec<Queued>,
}

fn shown_folder<'a>(name: &'a str, l: &'a Labels) -> &'a str {
    match name {
        "INBOX" => l.inbox_or("Inbox"),
        "Sent" => l.sent_or("Sent"),
        "Archive" => l.archive_or("Archive"),
        other => other,
    }
}

/// One line of text, cut with an ellipsis instead of wrapping.
fn line(ui: &Ui, text: &str, font: FontId, color: Color32, width: f32) -> Arc<egui::Galley> {
    let mut job = LayoutJob::simple(text.to_string(), font, color, width.max(1.0));
    job.wrap.max_rows = 1;
    job.wrap.break_anywhere = true;
    ui.painter().layout_job(job)
}

fn text_at(ui: &Ui, pos: Pos2, text: &str, font: FontId, color: Color32, width: f32) {
    let g = line(ui, text, font, color, width);
    ui.painter().galley(pos, g, color);
}

fn text_right(ui: &Ui, right_top: Pos2, text: &str, font: FontId, color: Color32) {
    let g = ui.painter().layout_no_wrap(text.to_string(), font, color);
    ui.painter().galley(Pos2::new(right_top.x - g.size().x, right_top.y), g, color);
}

fn avatar(ui: &Ui, center: Pos2, r: f32, initial: char, p: &Palette, font: FontId) {
    ui.painter().circle_filled(center, r, p.accent);
    ui.painter().text(center, Align2::CENTER_CENTER, initial.to_ascii_uppercase(), font, p.bg);
}

impl GardenApp {
    pub fn new() -> Self {
        let themes = BUILTIN.iter().map(|(id, src)| (*id, Theme::parse(src).expect("built-in theme is valid"))).collect();
        Self {
            themes,
            current: 0,
            applied: None,
            accounts: demo::accounts(),
            acct: 0,
            folder: 0,
            msg: Some(0),
            query: String::new(),
            compose: None,
            outbox: Vec::new(),
        }
    }

    pub fn set_theme(&mut self, id: &str) {
        if let Some(i) = self.themes.iter().position(|(n, _)| *n == id) {
            self.current = i;
        }
    }

    #[cfg(test)]
    pub fn theme_ids(&self) -> Vec<&'static str> {
        self.themes.iter().map(|(id, _)| *id).collect()
    }

    fn visible(&self) -> Vec<usize> {
        let q = self.query.trim().to_lowercase();
        self.accounts[self.acct].folders[self.folder]
            .messages
            .iter()
            .enumerate()
            .filter(|(_, m)| q.is_empty() || [m.from_name, m.from_addr, m.subject, m.body].iter().any(|s| s.to_lowercase().contains(&q)))
            .map(|(i, _)| i)
            .collect()
    }

    fn apply(&mut self, a: Action) {
        match a {
            Action::Folder(ai, fi) => {
                self.acct = ai;
                self.folder = fi;
                self.msg = None;
            }
            Action::Select(i) => {
                self.msg = Some(i);
                self.accounts[self.acct].folders[self.folder].messages[i].unread = false;
            }
            Action::Archive => {
                if let Some(i) = self.msg {
                    let acct = &mut self.accounts[self.acct];
                    if let Some(arch) = acct.folders.iter().position(|f| f.name == "Archive") {
                        if arch != self.folder {
                            let m = acct.folders[self.folder].messages.remove(i);
                            acct.folders[arch].messages.push(m);
                            self.msg = None;
                        }
                    }
                }
            }
            Action::Reply => {
                if let Some(i) = self.msg {
                    let m = &self.accounts[self.acct].folders[self.folder].messages[i];
                    let quoted: String = m.body.lines().map(|l| format!("> {l}\n")).collect();
                    self.compose = Some(Compose {
                        from: self.acct,
                        to: m.from_addr.to_string(),
                        subject: if m.subject.starts_with("Re: ") { m.subject.to_string() } else { format!("Re: {}", m.subject) },
                        body: format!("\n\nOn {}, {} wrote:\n{quoted}", m.long_date, m.from_name),
                    });
                }
            }
        }
    }

    /// Draw one frame. Kept separate from `eframe::App` so tests can run it headless.
    pub fn ui(&mut self, ctx: &egui::Context) {
        if self.applied != Some(self.current) {
            style::apply(ctx, &self.themes[self.current].1);
            self.applied = Some(self.current);
        }
        let theme_labels = Labels {
            inbox: self.themes[self.current].1.labels.inbox.clone(),
            sent: self.themes[self.current].1.labels.sent.clone(),
            compose: self.themes[self.current].1.labels.compose.clone(),
            archive: self.themes[self.current].1.labels.archive.clone(),
        };
        let p = style::palette(&self.themes[self.current].1);
        let fam = match self.themes[self.current].1.font.as_str() {
            "monospace" => egui::FontFamily::Monospace,
            _ => egui::FontFamily::Proportional,
        };
        let f = |size: f32| FontId::new(size, fam.clone());
        let mut action: Option<Action> = None;

        let bar = |fill: Color32| Frame::none().fill(fill).inner_margin(Margin::symmetric(PAD, 0.0));

        // ---- top bar ----
        egui::TopBottomPanel::top("top").exact_height(TOP_H).frame(bar(p.surface)).show_separator_line(false).show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.label(egui::RichText::new("Sidequest Mail").size(16.0).color(p.text));
                ui.add_space(PAD);
                if ui.add_sized([128.0, 32.0], egui::Button::new(theme_labels.compose_or("Compose"))).clicked() {
                    self.compose = Some(Compose { from: self.acct, to: String::new(), subject: String::new(), body: String::new() });
                }
                ui.add_space(PAD);
                ui.add_sized([320.0, 32.0], egui::TextEdit::singleline(&mut self.query).hint_text("Search this folder").vertical_align(Align::Center).margin(Margin::symmetric(12.0, 0.0)));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let mut chosen = self.current;
                    egui::ComboBox::from_id_salt("theme").width(160.0).selected_text(self.themes[self.current].1.name.clone()).show_ui(ui, |ui| {
                        for (i, (_, t)) in self.themes.iter().enumerate() {
                            ui.selectable_value(&mut chosen, i, &t.name);
                        }
                    });
                    self.current = chosen;
                    ui.label(egui::RichText::new("Theme").color(p.weak));
                });
            });
        });

        // ---- status bar ----
        egui::TopBottomPanel::bottom("status").exact_height(STATUS_H).frame(bar(p.surface)).show_separator_line(false).show(ctx, |ui| {
            let delivered = format!("Delivered by: {}", self.accounts[self.acct].delivered_by);
            let right = format!("Outbox: {} queued   ·   Demo data", self.outbox.len());
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.add(egui::Label::new(egui::RichText::new(delivered).size(12.0).color(p.weak)).truncate());
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(egui::RichText::new(right).size(12.0).color(p.weak));
                });
            });
        });

        // ---- accounts and folders ----
        egui::SidePanel::left("side").exact_width(SIDE_W).resizable(false).frame(Frame::none().fill(p.bg)).show_separator_line(false).show(ctx, |ui| {
            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            ui.add_space(PAD / 2.0);
            for ai in 0..self.accounts.len() {
                let a = &self.accounts[ai];
                let (rect, _) = ui.allocate_exact_size(vec2(SIDE_W, ACCOUNT_H), Sense::hover());
                avatar(ui, Pos2::new(rect.left() + PAD + 16.0, rect.center().y), 16.0, a.label.chars().next().unwrap_or('?'), &p, f(15.0));
                let tx = rect.left() + PAD + 32.0 + 12.0;
                let tw = rect.right() - tx - PAD;
                text_at(ui, Pos2::new(tx, rect.top() + 10.0), a.label, f(15.0), p.text, tw);
                text_at(ui, Pos2::new(tx, rect.top() + 31.0), a.address, f(12.0), p.weak, tw);
                for fi in 0..a.folders.len() {
                    let fo = &a.folders[fi];
                    let (rect, resp) = ui.allocate_exact_size(vec2(SIDE_W, FOLDER_H), Sense::click());
                    let sel = self.acct == ai && self.folder == fi;
                    let inner = rect.shrink2(vec2(8.0, 0.0));
                    if sel {
                        ui.painter().rect_filled(inner, 6.0, p.selected);
                    } else if resp.hovered() {
                        ui.painter().rect_filled(inner, 6.0, p.hover);
                    }
                    let name = shown_folder(fo.name, &theme_labels);
                    text_at(ui, Pos2::new(rect.left() + PAD + 44.0, rect.center().y - 8.0), name, f(14.0), p.text, SIDE_W - 120.0);
                    let unread = fo.messages.iter().filter(|m| m.unread).count();
                    if unread > 0 {
                        text_right(ui, Pos2::new(rect.right() - PAD - 8.0, rect.center().y - 8.0), &unread.to_string(), f(13.0), style::readable(p.unread, if sel { p.selected } else { p.bg }, p.text));
                    }
                    if resp.clicked() {
                        action = Some(Action::Folder(ai, fi));
                    }
                }
                ui.add_space(PAD);
            }
            let r = ui.max_rect();
            ui.painter().vline(r.right() - 0.5, r.y_range(), egui::Stroke::new(1.0_f32, p.line));
        });

        // ---- message list ----
        let vis = self.visible();
        egui::SidePanel::left("list").exact_width(LIST_W).resizable(false).frame(Frame::none().fill(p.bg)).show_separator_line(false).show(ctx, |ui| {
            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            let folder = &self.accounts[self.acct].folders[self.folder];
            let unread = folder.messages.iter().filter(|m| m.unread).count();
            let (head, _) = ui.allocate_exact_size(vec2(LIST_W, LIST_HEAD_H), Sense::hover());
            text_at(ui, Pos2::new(head.left() + PAD + 16.0, head.center().y - 11.0), shown_folder(folder.name, &theme_labels), f(18.0), p.text, 220.0);
            let count = if unread > 0 { format!("{unread} unread") } else { format!("{} messages", folder.messages.len()) };
            text_right(ui, Pos2::new(head.right() - PAD, head.center().y - 8.0), &count, f(13.0), p.weak);
            ui.painter().hline(head.x_range(), head.bottom(), egui::Stroke::new(1.0_f32, p.line));
            if vis.is_empty() {
                ui.add_space(PAD);
                let (r, _) = ui.allocate_exact_size(vec2(LIST_W, 24.0), Sense::hover());
                let msg = if self.query.trim().is_empty() { "Nothing here." } else { "No messages match your search." };
                text_at(ui, Pos2::new(r.left() + PAD + 16.0, r.top()), msg, f(14.0), p.weak, LIST_W - 2.0 * PAD);
            }
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                for &i in &vis {
                    let m = &folder.messages[i];
                    let (rect, resp) = ui.allocate_exact_size(vec2(LIST_W, ROW_H), Sense::click());
                    let sel = self.msg == Some(i);
                    if sel {
                        ui.painter().rect_filled(rect, 0.0, p.selected);
                        ui.painter().rect_filled(Rect::from_min_size(rect.min, vec2(3.0, ROW_H)), 0.0, p.accent);
                    } else if resp.hovered() {
                        ui.painter().rect_filled(rect, 0.0, p.hover);
                    }
                    let x = rect.left() + PAD + 16.0;
                    let w = rect.right() - PAD - x;
                    if m.unread {
                        ui.painter().circle_filled(Pos2::new(rect.left() + PAD + 3.0, rect.top() + 20.0), 4.0, p.unread);
                    }
                    let name_font = if m.unread { f(14.0) } else { f(14.0) };
                    let row_bg = if sel { p.selected } else { p.bg };
                    let name_color = if m.unread { style::readable(p.unread, row_bg, p.text) } else { p.text };
                    text_right(ui, Pos2::new(rect.right() - PAD, rect.top() + 12.0), m.short_date, f(12.0), p.weak);
                    text_at(ui, Pos2::new(x, rect.top() + 12.0), m.from_name, name_font, name_color, w - 64.0);
                    text_at(ui, Pos2::new(x, rect.top() + 30.0), m.subject, f(14.0), p.text, w);
                    text_at(ui, Pos2::new(x, rect.top() + 49.0), &m.preview(), f(12.0), p.weak, w);
                    ui.painter().hline(rect.x_range(), rect.bottom() - 0.5, egui::Stroke::new(1.0_f32, p.line));
                    if resp.clicked() {
                        action = Some(Action::Select(i));
                    }
                }
            });
            let r = ui.max_rect();
            ui.painter().vline(r.right() - 0.5, r.y_range(), egui::Stroke::new(1.0_f32, p.line));
        });

        // ---- reading pane ----
        let acct = &self.accounts[self.acct];
        let can_archive = acct.folders[self.folder].name != "Archive";
        egui::CentralPanel::default().frame(Frame::none().fill(p.bg).inner_margin(Margin::same(PAD * 2.0))).show(ctx, |ui| {
            match self.msg.and_then(|i| acct.folders[self.folder].messages.get(i)) {
                Some(m) => {
                    ui.spacing_mut().item_spacing = vec2(8.0, 8.0);
                    ui.horizontal(|ui| {
                        if ui.button("Reply").clicked() {
                            action = Some(Action::Reply);
                        }
                        if ui.add_enabled(can_archive, egui::Button::new(theme_labels.archive_or("Archive"))).clicked() {
                            action = Some(Action::Archive);
                        }
                    });
                    ui.add_space(PAD);
                    ui.label(egui::RichText::new(m.subject).size(22.0).color(p.text));
                    ui.add_space(PAD / 2.0);
                    let (hdr, _) = ui.allocate_exact_size(vec2(ui.available_width(), 48.0), Sense::hover());
                    avatar(ui, Pos2::new(hdr.left() + 20.0, hdr.center().y), 20.0, m.from_name.chars().next().unwrap_or('?'), &p, f(18.0));
                    let tx = hdr.left() + 52.0;
                    text_at(ui, Pos2::new(tx, hdr.top() + 6.0), m.from_name, f(15.0), p.text, hdr.right() - tx - 220.0);
                    text_at(ui, Pos2::new(tx, hdr.top() + 27.0), &format!("<{}>  to me", m.from_addr), f(12.0), p.weak, hdr.right() - tx - 220.0);
                    text_right(ui, Pos2::new(hdr.right(), hdr.top() + 6.0), m.long_date, f(12.0), p.weak);
                    let how = if m.via == Via::Provider { format!("Travelled via your provider ({})", acct.label) } else { "Travelled directly from a peer, no provider".to_string() };
                    ui.add(egui::Label::new(egui::RichText::new(how).size(12.0).color(p.accent)).truncate());
                    ui.add_space(PAD / 2.0);
                    let (line_r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
                    ui.painter().rect_filled(line_r, 0.0, p.line);
                    ui.add_space(PAD);
                    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                        ui.set_max_width(ui.available_width().min(680.0));
                        ui.label(egui::RichText::new(m.body).size(15.0));
                    });
                }
                None => {
                    let r = ui.available_rect_before_wrap();
                    ui.painter().text(r.center(), Align2::CENTER_CENTER, "Select a message to read it", f(15.0), p.weak);
                }
            }
        });

        // ---- compose window ----
        let mut close = false;
        let mut send: Option<Queued> = None;
        if let Some(c) = self.compose.as_mut() {
            let mut open = true;
            let names: Vec<String> = self.accounts.iter().map(|a| a.address.to_string()).collect();
            egui::Window::new(theme_labels.compose_or("Compose")).open(&mut open).collapsible(false).resizable(false).default_width(560.0).show(ctx, |ui| {
                egui::Grid::new("compose").num_columns(2).spacing([PAD, 8.0]).show(ui, |ui| {
                    ui.label("From");
                    egui::ComboBox::from_id_salt("from").width(440.0).selected_text(names[c.from].clone()).show_ui(ui, |ui| {
                        for (i, n) in names.iter().enumerate() {
                            ui.selectable_value(&mut c.from, i, n);
                        }
                    });
                    ui.end_row();
                    ui.label("To");
                    ui.add_sized([440.0, 32.0], egui::TextEdit::singleline(&mut c.to).vertical_align(Align::Center).margin(Margin::symmetric(12.0, 0.0)));
                    ui.end_row();
                    ui.label("Subject");
                    ui.add_sized([440.0, 32.0], egui::TextEdit::singleline(&mut c.subject).vertical_align(Align::Center).margin(Margin::symmetric(12.0, 0.0)));
                    ui.end_row();
                });
                ui.add_space(8.0);
                ui.add_sized([ui.available_width(), 200.0], egui::TextEdit::multiline(&mut c.body));
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("Queue for sending").clicked() && !c.to.trim().is_empty() {
                        send = Some(Queued { from: names[c.from].clone(), to: c.to.clone(), subject: c.subject.clone() });
                        close = true;
                    }
                    ui.label(egui::RichText::new("Works offline. Sent when a connection is available.").color(p.weak).size(12.0));
                });
            });
            if !open {
                close = true;
            }
        }
        if let Some(q) = send {
            self.outbox.push(q);
        }
        if close {
            self.compose = None;
        }
        if let Some(a) = action {
            self.apply(a);
        }
        let _ = (TextStyle::Body, Rect::NOTHING);
    }
}

impl eframe::App for GardenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui(ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_theme_renders_a_frame_headless() {
        let mut app = GardenApp::new();
        for id in app.theme_ids() {
            app.set_theme(id);
            let ctx = egui::Context::default();
            let out = ctx.run(egui::RawInput::default(), |ctx| app.ui(ctx));
            assert!(!out.shapes.is_empty(), "{id} drew nothing");
        }
    }

    #[test]
    fn archive_moves_message_and_reply_prefills() {
        let mut app = GardenApp::new();
        let before = app.accounts[0].folders[0].messages.len();
        app.apply(Action::Reply);
        let c = app.compose.as_ref().unwrap();
        assert_eq!(c.to, "ada@example.org");
        assert!(c.subject.starts_with("Re: "));
        app.apply(Action::Archive);
        assert_eq!(app.accounts[0].folders[0].messages.len(), before - 1);
        assert_eq!(app.accounts[0].folders[2].messages.len(), 1);
        assert!(app.msg.is_none());
    }

    #[test]
    fn search_filters_and_selecting_marks_read() {
        let mut app = GardenApp::new();
        app.query = "build".into();
        assert_eq!(app.visible().len(), 1);
        app.query = "zzz".into();
        assert_eq!(app.visible().len(), 0);
        app.query.clear();
        let unread = app.accounts[0].folders[0].messages.iter().filter(|m| m.unread).count();
        app.apply(Action::Select(1));
        let now = app.accounts[0].folders[0].messages.iter().filter(|m| m.unread).count();
        assert_eq!(now, unread - 1);
    }
}
