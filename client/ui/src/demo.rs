//! Sample data so the UI can be seen before IMAP sync exists. Clearly marked as demo in the UI.

#[derive(Clone, Copy, PartialEq)]
pub enum Via {
    Provider,
    #[allow(dead_code)] // peer-to-peer path is Phase 2
    Peer,
}

pub struct Message {
    pub from: &'static str,
    pub subject: &'static str,
    pub date: &'static str,
    pub body: &'static str,
    pub unread: bool,
    pub via: Via,
}

pub struct Folder {
    pub name: &'static str,
    pub messages: Vec<Message>,
}

pub struct Account {
    pub label: &'static str,
    pub address: &'static str,
    /// Who delivers this account's mail, shown in the UI.
    pub delivered_by: &'static str,
    pub folders: Vec<Folder>,
}

pub fn accounts() -> Vec<Account> {
    vec![
        Account {
            label: "Proton",
            address: "you@proton.me",
            delivered_by: "Proton Mail, through Proton Mail Bridge on this device",
            folders: vec![
                Folder {
                    name: "INBOX",
                    messages: vec![
                        Message { from: "Ada <ada@example.org>", subject: "Welcome to your side quest", date: "2026-11-18", body: "This is demo data. Real mail appears here once IMAP sync is built.", unread: true, via: Via::Provider },
                        Message { from: "Bridge <noreply@example.org>", subject: "Your inbox, on your device", date: "2026-11-17", body: "A full copy of your mail lives locally, so a provider's decision cannot erase it.", unread: false, via: Via::Provider },
                    ],
                },
                Folder { name: "Sent", messages: vec![] },
                Folder { name: "Archive", messages: vec![] },
            ],
        },
        Account {
            label: "Gmail",
            address: "you@gmail.com",
            delivered_by: "Google (Gmail IMAP/SMTP)",
            folders: vec![
                Folder {
                    name: "INBOX",
                    messages: vec![Message { from: "Newsletter <news@example.com>", subject: "Weekly digest", date: "2026-11-16", body: "Remote images are blocked by default. Demo message only.", unread: true, via: Via::Provider }],
                },
                Folder { name: "Sent", messages: vec![] },
                Folder { name: "Archive", messages: vec![] },
            ],
        },
    ]
}
