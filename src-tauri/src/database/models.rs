use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub email: String,
    pub provider: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Participant {
    pub id: String,
    pub name: Option<String>,
    pub email: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thread {
    pub id: String,
    pub subject: Option<String>,
    pub snippet: Option<String>,
    pub last_message_at: Option<DateTime<Utc>>,
    pub is_read: bool,
    pub is_archived: bool,
    pub tags: Vec<String>, // Parsed from JSON
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub thread_id: String,
    pub sender_id: String,
    pub subject: Option<String>,
    pub content_text: Option<String>,
    pub content_html: Option<String>,
    pub received_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub id: String,
    pub message_id: String,
    pub filename: String,
    pub mime_type: Option<String>,
    pub size: Option<i64>,
    pub hash: String,
    pub local_path: Option<String>,
}
