//! Local SQLite store: accounts, folders, messages (raw MIME kept verbatim), outbox.
use rusqlite::{Connection, Result};
use std::path::Path;

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS accounts (
  id INTEGER PRIMARY KEY,
  label TEXT NOT NULL,
  address TEXT NOT NULL UNIQUE,
  provider TEXT NOT NULL,       -- shown to the user: who delivers this mail
  imap_host TEXT NOT NULL,
  smtp_host TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS folders (
  id INTEGER PRIMARY KEY,
  account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  uidvalidity INTEGER,
  UNIQUE(account_id, name)
);
CREATE TABLE IF NOT EXISTS messages (
  id INTEGER PRIMARY KEY,
  folder_id INTEGER NOT NULL REFERENCES folders(id) ON DELETE CASCADE,
  uid INTEGER NOT NULL,
  subject TEXT,
  from_addr TEXT,
  date TEXT,
  raw_mime BLOB NOT NULL,       -- original message, never modified
  via TEXT NOT NULL DEFAULT 'provider',  -- 'provider' or 'peer'
  UNIQUE(folder_id, uid)
);
CREATE TABLE IF NOT EXISTS outbox (
  id INTEGER PRIMARY KEY,
  account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  raw_mime BLOB NOT NULL,
  queued_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  sent_at TEXT
);
";

pub fn open(path: impl AsRef<Path>) -> Result<Connection> {
    let conn = Connection::open(path)?;
    migrate(&conn)?;
    Ok(conn)
}

pub fn open_in_memory() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    migrate(&conn)?;
    Ok(conn)
}

fn migrate(conn: &Connection) -> Result<()> {
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.execute_batch(SCHEMA)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_message_and_cascades_on_account_delete() {
        let c = open_in_memory().unwrap();
        c.execute("INSERT INTO accounts(label,address,provider,imap_host,smtp_host) VALUES('p','a@b.c','Proton Bridge','127.0.0.1','127.0.0.1')", []).unwrap();
        c.execute("INSERT INTO folders(account_id,name) VALUES(1,'INBOX')", []).unwrap();
        c.execute("INSERT INTO messages(folder_id,uid,subject,raw_mime) VALUES(1,1,'hi',x'00')", []).unwrap();
        c.execute("DELETE FROM accounts WHERE id=1", []).unwrap();
        let n: i64 = c.query_row("SELECT COUNT(*) FROM messages", [], |r| r.get(0)).unwrap();
        assert_eq!(n, 0);
    }
}
