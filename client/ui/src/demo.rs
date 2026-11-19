//! Sample data so the UI can be seen before IMAP sync exists. Clearly marked as demo in the UI.

#[derive(Clone, Copy, PartialEq)]
pub enum Via {
    Provider,
    #[allow(dead_code)] // peer-to-peer path is Phase 2
    Peer,
}

pub struct Message {
    pub from_name: &'static str,
    pub from_addr: &'static str,
    pub subject: &'static str,
    pub short_date: &'static str,
    pub long_date: &'static str,
    pub body: &'static str,
    pub unread: bool,
    pub via: Via,
}

impl Message {
    /// One-line preview for the list.
    pub fn preview(&self) -> String {
        self.body.split_whitespace().collect::<Vec<_>>().join(" ")
    }
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

fn m(from_name: &'static str, from_addr: &'static str, subject: &'static str, short_date: &'static str, long_date: &'static str, body: &'static str, unread: bool) -> Message {
    Message { from_name, from_addr, subject, short_date, long_date, body, unread, via: Via::Provider }
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
                        m("Ada Lovelace", "ada@example.org", "Welcome to your side quest", "Nov 18", "Wed, 18 Nov 2026, 09:14",
                          "This is demo data. Real mail appears here once IMAP sync is built.\n\nUntil then, try the themes, the search box, and Compose.", true),
                        m("Grace Hopper", "grace@example.org", "Notes from Tuesday", "Nov 18", "Wed, 18 Nov 2026, 08:02",
                          "Thanks for joining. Here are the notes we agreed on:\n\n1. Keep the core small.\n2. Keep the data local.\n3. Keep the themes optional.", true),
                        m("Bridge Status", "status@example.org", "Your inbox, on your device", "Nov 17", "Tue, 17 Nov 2026, 17:40",
                          "A full copy of your mail lives locally, so a provider's decision cannot erase it.", false),
                        m("Linus T.", "linus@example.org", "Re: build times", "Nov 16", "Mon, 16 Nov 2026, 11:25",
                          "Release builds with link-time optimization take longer, but the binary is smaller and faster. Worth it.", false),
                        m("Margaret H.", "margaret@example.org", "Weekend plans", "Nov 14", "Sat, 14 Nov 2026, 19:03",
                          "Are we still on for Saturday? Let me know and I will bring the maps.", false),
                    ],
                },
                Folder {
                    name: "Sent",
                    messages: vec![m("You", "you@proton.me", "Re: build times", "Nov 16", "Mon, 16 Nov 2026, 11:31", "Agreed. Shipping it that way.", false)],
                },
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
                    messages: vec![
                        m("Weekly Digest", "news@example.com", "Five things worth reading", "Nov 18", "Wed, 18 Nov 2026, 06:00",
                          "Remote images are blocked by default, so tracking pixels in newsletters like this one never load.", true),
                        m("Ticket Desk", "tickets@example.com", "Your ticket was updated", "Nov 15", "Sun, 15 Nov 2026, 14:12",
                          "Your request has been marked as resolved. Reply to this message to reopen it.", false),
                    ],
                },
                Folder { name: "Sent", messages: vec![] },
                Folder { name: "Archive", messages: vec![] },
            ],
        },
    ]
}
