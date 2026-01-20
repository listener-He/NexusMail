use rusqlite::{Connection, Result};
use std::path::Path;
use tokio::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P, key: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        // Enable encryption (SQLCipher)
        // Note: In production, this key must come from the System Keychain.
        conn.pragma_update(None, "key", &key)?;
        
        // Verify encryption works
        // conn.execute("SELECT count(*) FROM sqlite_master;", [])?;

        Ok(Database {
            conn: Mutex::new(conn),
        })
    }

    pub async fn init(&self) -> Result<()> {
        let conn = self.conn.lock().await;
        
        // Enable foreign keys
        conn.pragma_update(None, "foreign_keys", "ON")?;

        // 1. Accounts Table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS accounts (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL UNIQUE,
                provider TEXT NOT NULL, -- 'google', 'outlook', 'imap'
                credentials TEXT, -- Encrypted JSON blob
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // 2. Identity Hub (Participants)
        // Aggregates multiple email addresses under one identity if needed
        conn.execute(
            "CREATE TABLE IF NOT EXISTS participants (
                id TEXT PRIMARY KEY,
                name TEXT,
                email TEXT NOT NULL UNIQUE,
                avatar_url TEXT,
                fingerprint TEXT -- For identity merging
            )",
            [],
        )?;

        // 3. Threads (The Chat Bubble Container)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS threads (
                id TEXT PRIMARY KEY,
                subject TEXT,
                snippet TEXT,
                last_message_at DATETIME,
                is_read BOOLEAN DEFAULT 0,
                is_archived BOOLEAN DEFAULT 0,
                tags TEXT -- JSON array of tags
            )",
            [],
        )?;

        // 4. Messages (Individual Emails)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                thread_id TEXT NOT NULL,
                sender_id TEXT NOT NULL,
                subject TEXT,
                content_text TEXT,
                content_html TEXT,
                received_at DATETIME,
                FOREIGN KEY(thread_id) REFERENCES threads(id),
                FOREIGN KEY(sender_id) REFERENCES participants(id)
            )",
            [],
        )?;

        // 5. Attachments (Deduplicated via Hash)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS attachments (
                id TEXT PRIMARY KEY,
                message_id TEXT NOT NULL,
                filename TEXT NOT NULL,
                mime_type TEXT,
                size INTEGER,
                hash TEXT NOT NULL, -- SHA-3-256 for deduplication
                local_path TEXT,
                FOREIGN KEY(message_id) REFERENCES messages(id)
            )",
            [],
        )?;

        // Indexes for Performance
        conn.execute("CREATE INDEX IF NOT EXISTS idx_threads_last_msg ON threads(last_message_at DESC)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_messages_thread ON messages(thread_id)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_attachments_hash ON attachments(hash)", [])?;

        Ok(())
    }
}
