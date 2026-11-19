use crate::{demo, style};
use eframe::egui;
use garden_core::theme::{BUILTIN, Theme};

struct Compose {
    from: usize,
    to: String,
    subject: String,
    body: String,
}

struct Queued {
    from: String,
    to: String,
    subject: String,
}

pub struct GardenApp {
    themes: Vec<(&'static str, Theme)>,
    current: usize,
    applied: Option<usize>,
    accounts: Vec<demo::Account>,
    acct: usize,
    folder: usize,
    msg: Option<usize>,
    compose: Option<Compose>,
    outbox: Vec<Queued>,
}

impl GardenApp {
    pub fn new() -> Self {
        let themes = BUILTIN
            .iter()
            .map(|(id, src)| (*id, Theme::parse(src).expect("built-in theme is valid")))
            .collect();
        Self {
            themes,
            current: 0,
            applied: None,
            accounts: demo::accounts(),
            acct: 0,
            folder: 0,
            msg: None,
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

    /// Draw one frame. Kept separate from `eframe::App` so tests can run it headless.
    pub fn ui(&mut self, ctx: &egui::Context) {
        if self.applied != Some(self.current) {
            style::apply(ctx, &self.themes[self.current].1);
            self.applied = Some(self.current);
        }
        let theme = &self.themes[self.current].1;
        let accent = style::accent(theme);
        let unread = style::unread(theme);
        let labels = &theme.labels;

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Sidequest Mail");
                ui.separator();
                if ui.button(format!("✉ {}", labels.compose_or("Compose"))).clicked() {
                    self.compose = Some(Compose { from: self.acct, to: String::new(), subject: String::new(), body: String::new() });
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let mut chosen = self.current;
                    egui::ComboBox::from_id_salt("theme")
                        .selected_text(self.themes[self.current].1.name.clone())
                        .show_ui(ui, |ui| {
                            for (i, (_, t)) in self.themes.iter().enumerate() {
                                ui.selectable_value(&mut chosen, i, &t.name);
                            }
                        });
                    self.current = chosen;
                    ui.label("Theme");
                });
            });
            ui.colored_label(accent, "Demo data only: IMAP sync is not built yet.");
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            let a = &self.accounts[self.acct];
            ui.label(format!(
                "Delivered by: {}   |   Offline outbox: {} queued   |   No account, no telemetry, nothing leaves this device in this demo",
                a.delivered_by,
                self.outbox.len()
            ));
        });

        egui::SidePanel::left("accounts").min_width(170.0).show(ctx, |ui| {
            ui.heading("Accounts");
            for ai in 0..self.accounts.len() {
                let (label, address) = (self.accounts[ai].label, self.accounts[ai].address);
                ui.label(egui::RichText::new(format!("{label}  ({address})")).strong());
                for fi in 0..self.accounts[ai].folders.len() {
                    let name = self.accounts[ai].folders[fi].name;
                    let shown = match name {
                        "INBOX" => labels.inbox_or("Inbox"),
                        "Sent" => labels.sent_or("Sent"),
                        "Archive" => labels.archive_or("Archive"),
                        other => other,
                    };
                    let sel = self.acct == ai && self.folder == fi;
                    if ui.selectable_label(sel, format!("   {shown}")).clicked() {
                        self.acct = ai;
                        self.folder = fi;
                        self.msg = None;
                    }
                }
                ui.add_space(8.0);
            }
        });

        egui::SidePanel::left("messages").min_width(280.0).show(ctx, |ui| {
            let folder = &self.accounts[self.acct].folders[self.folder];
            if folder.messages.is_empty() {
                ui.label("Nothing here. A clear board.");
            }
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (mi, m) in folder.messages.iter().enumerate() {
                    let title = egui::RichText::new(m.subject);
                    let title = if m.unread { title.strong().color(unread) } else { title };
                    let r = ui.selectable_label(self.msg == Some(mi), title);
                    ui.label(egui::RichText::new(format!("{}  {}", m.from, m.date)).small());
                    ui.separator();
                    if r.clicked() {
                        self.msg = Some(mi);
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let a = &self.accounts[self.acct];
            match self.msg.and_then(|i| a.folders[self.folder].messages.get(i)) {
                Some(m) => {
                    ui.heading(m.subject);
                    ui.label(format!("From: {}   {}", m.from, m.date));
                    let how = if m.via == demo::Via::Provider {
                        format!("via your provider: {}", a.delivered_by)
                    } else {
                        "directly from a peer, no provider".to_string()
                    };
                    ui.colored_label(accent, format!("Travelled {how}"));
                    ui.separator();
                    ui.label(m.body);
                }
                None => {
                    ui.label("Select a message to read it.");
                }
            }
        });

        let mut close = false;
        let mut send: Option<Queued> = None;
        if let Some(c) = self.compose.as_mut() {
            let mut open = true;
            let names: Vec<String> = self.accounts.iter().map(|a| a.address.to_string()).collect();
            egui::Window::new(labels.compose_or("Compose")).open(&mut open).show(ctx, |ui| {
                egui::ComboBox::from_label("From").selected_text(names[c.from].clone()).show_ui(ui, |ui| {
                    for (i, n) in names.iter().enumerate() {
                        ui.selectable_value(&mut c.from, i, n);
                    }
                });
                ui.add(egui::TextEdit::singleline(&mut c.to).hint_text("To"));
                ui.add(egui::TextEdit::singleline(&mut c.subject).hint_text("Subject"));
                ui.add(egui::TextEdit::multiline(&mut c.body).desired_rows(8));
                if ui.button("Queue for sending (works offline)").clicked() && !c.to.trim().is_empty() {
                    send = Some(Queued { from: names[c.from].clone(), to: c.to.clone(), subject: c.subject.clone() });
                    close = true;
                }
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

        if let Some(last) = self.outbox.last() {
            egui::Window::new("Outbox").anchor(egui::Align2::RIGHT_BOTTOM, [-10.0, -40.0]).collapsible(true).show(ctx, |ui| {
                for q in &self.outbox {
                    ui.label(format!("{} -> {}: {}", q.from, q.to, q.subject));
                }
                let _ = last;
            });
        }
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
}
