use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 账户模型 (Account Model)
/// Represents a user's email account connected to the application.
/// 代表用户连接到应用程序的电子邮件帐户。
#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    /// Unique identifier (UUID) / 唯一标识符
    pub id: String,
    /// Email address / 电子邮件地址
    pub email: String,
    /// Service provider (e.g., "google", "outlook", "imap") / 服务提供商
    pub provider: String,
    /// Creation timestamp / 创建时间戳
    pub created_at: DateTime<Utc>,
}

/// 参与者模型 (Participant Model)
/// Represents an entity (person or service) that participates in a conversation.
/// 代表参与对话的实体（人或服务）。
/// This acts as an Identity Hub to merge multiple email addresses for the same person.
/// 这充当身份中心，用于合并同一人的多个电子邮件地址。
#[derive(Debug, Serialize, Deserialize)]
pub struct Participant {
    /// Unique identifier (UUID) / 唯一标识符
    pub id: String,
    /// Display name / 显示名称
    pub name: Option<String>,
    /// Primary email address / 主要电子邮件地址
    pub email: String,
    /// URL to the avatar image / 头像图片链接
    pub avatar_url: Option<String>,
}

/// 邮件会话模型 (Thread Model)
/// Represents a conversation thread containing multiple messages.
/// 代表包含多条消息的邮件会话。
/// A thread is the primary unit of organization in the Inbox.
/// 会话是收件箱中的主要组织单位。
#[derive(Debug, Serialize, Deserialize)]
pub struct Thread {
    /// Unique identifier (UUID) / 唯一标识符
    pub id: String,
    /// Subject line of the thread / 会话主题
    pub subject: Option<String>,
    /// Short preview of the latest content / 最新内容的简短预览
    pub snippet: Option<String>,
    /// Timestamp of the last message / 最后一条消息的时间戳
    pub last_message_at: Option<DateTime<Utc>>,
    /// Read status / 阅读状态
    pub is_read: bool,
    /// Archived status / 归档状态
    pub is_archived: bool,
    /// List of tags (e.g., "work", "invoice") / 标签列表
    pub tags: Vec<String>, 
}

/// 消息模型 (Message Model)
/// Represents an individual email message within a thread.
/// 代表会话中的单条电子邮件消息。
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    /// Unique identifier (UUID) / 唯一标识符
    pub id: String,
    /// ID of the parent thread / 父会话ID
    pub thread_id: String,
    /// ID of the sender (Participant) / 发送者ID
    pub sender_id: String,
    /// Subject of this specific message / 此特定消息的主题
    pub subject: Option<String>,
    /// Plain text content / 纯文本内容
    pub content_text: Option<String>,
    /// HTML content / HTML内容
    pub content_html: Option<String>,
    /// Time received / 接收时间
    pub received_at: Option<DateTime<Utc>>,
}

/// 附件模型 (Attachment Model)
/// Represents a file attached to a message.
/// 代表附加到消息的文件。
/// Attachments are deduplicated using a content hash.
/// 附件使用内容哈希进行重复数据删除。
#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    /// Unique identifier (UUID) / 唯一标识符
    pub id: String,
    /// ID of the parent message / 父消息ID
    pub message_id: String,
    /// Original filename / 原始文件名
    pub filename: String,
    /// MIME type (e.g., "application/pdf") / MIME类型
    pub mime_type: Option<String>,
    /// File size in bytes / 文件大小（字节）
    pub size: Option<i64>,
    /// SHA-3-256 hash for deduplication / 用于去重的 SHA-3-256 哈希
    pub hash: String,
    /// Local file system path to the stored attachment / 存储附件的本地文件系统路径
    pub local_path: Option<String>,
}
