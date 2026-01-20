use rusqlite::{Connection, Result, params};
use std::path::Path;
use tokio::sync::Mutex;
use uuid::Uuid;
use chrono::Utc;
use crate::database::models::{Thread, Message, Participant};

pub mod models;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P, key: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "key", &key)?;
        Ok(Database {
            conn: Mutex::new(conn),
        })
    }

    pub async fn init(&self) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.pragma_update(None, "foreign_keys", "ON")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS accounts (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL UNIQUE,
                provider TEXT NOT NULL,
                credentials TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS participants (
                id TEXT PRIMARY KEY,
                name TEXT,
                email TEXT NOT NULL UNIQUE,
                avatar_url TEXT,
                fingerprint TEXT
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS threads (
                id TEXT PRIMARY KEY,
                subject TEXT,
                snippet TEXT,
                last_message_at DATETIME,
                is_read BOOLEAN DEFAULT 0,
                is_archived BOOLEAN DEFAULT 0,
                tags TEXT
            )",
            [],
        )?;

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

        conn.execute(
            "CREATE TABLE IF NOT EXISTS attachments (
                id TEXT PRIMARY KEY,
                message_id TEXT NOT NULL,
                filename TEXT NOT NULL,
                mime_type TEXT,
                size INTEGER,
                hash TEXT NOT NULL,
                local_path TEXT,
                FOREIGN KEY(message_id) REFERENCES messages(id)
            )",
            [],
        )?;

        conn.execute("CREATE INDEX IF NOT EXISTS idx_threads_last_msg ON threads(last_message_at DESC)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_messages_thread ON messages(thread_id)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_attachments_hash ON attachments(hash)", [])?;

        Ok(())
    }

    // --- Insert / Update Methods ---

    pub async fn create_or_get_participant(&self, email: &str, name: Option<&str>) -> Result<String> {
        let conn = self.conn.lock().await;
        
        // Try to find existing
        let mut stmt = conn.prepare("SELECT id FROM participants WHERE email = ?1")?;
        let existing_id: Option<String> = stmt.query_row(params![email], |row| row.get(0)).ok();

        if let Some(id) = existing_id {
            return Ok(id);
        }

        // Create new
        let id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO participants (id, email, name) VALUES (?1, ?2, ?3)",
            params![id, email, name],
        )?;
        Ok(id)
    }

    pub async fn create_thread(&self, subject: &str, snippet: &str) -> Result<String> {
        let conn = self.conn.lock().await;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        conn.execute(
            "INSERT INTO threads (id, subject, snippet, last_message_at, is_read, tags) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, subject, snippet, now, false, "[]"],
        )?;
        Ok(id)
    }

    pub async fn add_message(&self, thread_id: &str, sender_id: &str, subject: &str, text: &str, html: &str) -> Result<String> {
        let conn = self.conn.lock().await;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        conn.execute(
            "INSERT INTO messages (id, thread_id, sender_id, subject, content_text, content_html, received_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, thread_id, sender_id, subject, text, html, now],
        )?;

        // Update thread snippet and time
        conn.execute(
            "UPDATE threads SET snippet = ?1, last_message_at = ?2 WHERE id = ?3",
            params![text.chars().take(100).collect::<String>(), now, thread_id],
        )?;

        Ok(id)
    }

    pub async fn get_all_threads(&self) -> Result<Vec<Thread>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT id, subject, snippet, last_message_at, is_read, is_archived, tags FROM threads ORDER BY last_message_at DESC")?;
        
        let thread_iter = stmt.query_map([], |row| {
            let tags_json: String = row.get(6).unwrap_or("[]".to_string());
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
            
            Ok(Thread {
                id: row.get(0)?,
                subject: row.get(1)?,
                snippet: row.get(2)?,
                last_message_at: row.get(3)?,
                is_read: row.get(4)?,
                is_archived: row.get(5)?,
                tags,
            })
        })?;

        let mut threads = Vec::new();
        for thread in thread_iter {
            threads.push(thread?);
        }
        Ok(threads)
    }
}
