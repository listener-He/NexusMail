use rusqlite::{Connection, Result, params};
use std::path::Path;
use tokio::sync::Mutex;
use uuid::Uuid;
use chrono::Utc;
use crate::database::models::{Thread, Message, Participant, Account};

pub mod models;

/// 数据库管理器 (Database Manager)
/// Wraps the SQLite connection in a thread-safe Mutex.
/// 将 SQLite 连接包装在线程安全的互斥锁中。
pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    /// Create a new encrypted database connection.
    /// 创建一个新的加密数据库连接。
    ///
    /// # Arguments
    /// * `path` - File path to the database / 数据库文件路径
    /// * `key` - Encryption key (SQLCipher) / 加密密钥
    pub fn new<P: AsRef<Path>>(path: P, key: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        // Set the encryption key via PRAGMA
        // 通过 PRAGMA 设置加密密钥
        conn.pragma_update(None, "key", &key)?;
        
        Ok(Database {
            conn: Mutex::new(conn),
        })
    }

    /// Initialize database schema (Tables & Indexes).
    /// 初始化数据库模式（表和索引）。
    /// This should be called on application startup.
    /// 应在应用程序启动时调用此方法。
    pub async fn init(&self) -> Result<()> {
        let conn = self.conn.lock().await;
        
        // Enforce foreign key constraints
        // 强制执行外键约束
        conn.pragma_update(None, "foreign_keys", "ON")?;

        // 1. Accounts Table / 账户表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS accounts (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL UNIQUE,
                provider TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                credentials_json TEXT
            )",
            [],
        )?;

        // 1.1 Sync Channels Table / 同步渠道表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS sync_channels (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                type_ TEXT NOT NULL,
                config_json TEXT NOT NULL,
                last_sync_at DATETIME,
                is_active BOOLEAN DEFAULT 1
            )",
            [],
        )?;

        // 2. Identity Hub (Participants) / 身份中心（参与者）
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

        // 3. Threads / 会话
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

        // 4. Messages / 消息
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

        // 5. Attachments / 附件
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

        // Indexes for Performance / 性能索引
        conn.execute("CREATE INDEX IF NOT EXISTS idx_threads_last_msg ON threads(last_message_at DESC)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_messages_thread ON messages(thread_id)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_attachments_hash ON attachments(hash)", [])?;

        Ok(())
    }

    // --- Account Methods / 账户方法 ---

    /// Create a new account with encrypted credentials.
    /// 创建带有加密凭据的新帐户。
    pub async fn create_account(&self, email: &str, provider: &str, credentials_json: &str) -> Result<String> {
        let conn = self.conn.lock().await;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        conn.execute(
            "INSERT INTO accounts (id, email, provider, created_at, credentials_json) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, email, provider, now, credentials_json],
        )?;
        Ok(id)
    }

    /// Get all accounts.
    /// 获取所有帐户。
    pub async fn get_accounts(&self) -> Result<Vec<Account>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT id, email, provider, created_at, credentials_json FROM accounts")?;
        
        let account_iter = stmt.query_map([], |row| {
            Ok(Account {
                id: row.get(0)?,
                email: row.get(1)?,
                provider: row.get(2)?,
                created_at: row.get(3)?,
                credentials_json: row.get(4)?,
            })
        })?;

        let mut accounts = Vec::new();
        for account in account_iter {
            accounts.push(account?);
        }
        Ok(accounts)
    }

    /// Delete an account.
    /// 删除帐户。
    pub async fn delete_account(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM accounts WHERE id = ?1", params![id])?;
        Ok(())
    }

    // --- Insert / Update Methods ---

    /// Create or retrieve a participant by email.
    /// 通过电子邮件创建或检索参与者。
    /// Ensures idempotency for senders.
    /// 确保发送者的幂等性。
    pub async fn create_or_get_participant(&self, email: &str, name: Option<&str>) -> Result<String> {
        let conn = self.conn.lock().await;
        
        // Try to find existing / 尝试查找现有的
        let mut stmt = conn.prepare("SELECT id FROM participants WHERE email = ?1")?;
        let existing_id: Option<String> = stmt.query_row(params![email], |row| row.get(0)).ok();

        if let Some(id) = existing_id {
            return Ok(id);
        }

        // Create new / 创建新的
        let id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO participants (id, email, name) VALUES (?1, ?2, ?3)",
            params![id, email, name],
        )?;
        Ok(id)
    }

    /// Create a new thread.
    /// 创建一个新的会话。
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

    /// Add a message to a thread.
    /// 向会话添加一条消息。
    /// Also updates the thread's `last_message_at` and `snippet`.
    /// 同时更新会话的 `last_message_at` 和 `snippet`。
    pub async fn add_message(&self, thread_id: &str, sender_id: &str, subject: &str, text: &str, html: &str) -> Result<String> {
        let conn = self.conn.lock().await;
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        conn.execute(
            "INSERT INTO messages (id, thread_id, sender_id, subject, content_text, content_html, received_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, thread_id, sender_id, subject, text, html, now],
        )?;

        // Update thread snippet and time / 更新会话摘要和时间
        conn.execute(
            "UPDATE threads SET snippet = ?1, last_message_at = ?2 WHERE id = ?3",
            params![text.chars().take(100).collect::<String>(), now, thread_id],
        )?;

        Ok(id)
    }

    /// Archive a message (and its thread).
    /// 归档消息（及其会话）。
    pub async fn archive_message(&self, message_id: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        
        // Find thread for message / 查找消息所属的会话
        let thread_id: String = conn.query_row(
            "SELECT thread_id FROM messages WHERE id = ?1",
            params![message_id],
            |row| row.get(0),
        )?;

        // Mark thread as archived / 标记会话为已归档
        conn.execute(
            "UPDATE threads SET is_archived = 1 WHERE id = ?1",
            params![thread_id],
        )?;

        Ok(())
    }

    /// Change the database encryption key (Rekey).
    /// 更改数据库加密密钥 (Rekey)。
    /// This is an expensive operation as it rewrites the entire database.
    /// 这是一个昂贵的操作，因为它会重写整个数据库。
    pub async fn change_password(&self, new_key: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        // SQLCipher PRAGMA rekey
        conn.pragma_update(None, "rekey", &new_key)?;
        Ok(())
    }

    /// Fetch all threads sorted by latest activity.
    /// 获取按最新活动排序的所有会话。
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
