use async_imap::error::Error;
use futures::stream::StreamExt;
use mailparse::*;
use std::sync::Arc;
use crate::database::Database;

use crate::engine::Engine;
use crate::engine::search::SearchService;

/// 摄取服务 (Ingestion Service)
/// Handles connecting to external email providers (IMAP) and importing data.
/// 处理连接外部电子邮件提供商 (IMAP) 并导入数据。
/// It acts as the "Source" in the data pipeline.
/// 它是数据管道中的“源”。
pub struct IngestionService {
    db: Arc<Database>,
    engine: Arc<Engine>,
    search: Arc<SearchService>,
}

impl IngestionService {
    pub fn new(db: Arc<Database>, engine: Arc<Engine>, search: Arc<SearchService>) -> Self {
        Self { db, engine, search }
    }

    /// Sync emails from an IMAP account.
    /// 从 IMAP 帐户同步电子邮件。
    ///
    /// # Arguments
    /// * `email` - User email / 用户邮箱
    /// * `password` - User password (or app password) / 用户密码（或应用密码）
    /// * `server` - IMAP server address (e.g., "imap.gmail.com") / IMAP 服务器地址
    pub async fn sync_account(&self, email: &str, password: &str, server: &str) -> Result<(), Box<dyn std::error::Error>> {
        let domain = server;
        let tls = async_native_tls::TlsConnector::new();
        
        // Connect to IMAP server over SSL/TLS
        // 通过 SSL/TLS 连接到 IMAP 服务器
        let client = async_imap::connect((domain, 993), domain, tls).await?;

        // Authenticate
        // 认证
        let mut imap_session = client.login(email, password).await.map_err(|e| e.0)?;

        // Select Inbox
        // 选择收件箱
        imap_session.select("INBOX").await?;

        // Fetch emails (Limit to top 5 for POC performance)
        // 获取电子邮件（为 POC 性能限制为前 5 封）
        // TODO: Implement pagination or UID-based fetching for production.
        let messages = imap_session.fetch("1:5", "RFC822").await?;

        for message in messages.iter() {
            if let Some(body) = message.body() {
                // Parse MIME content
                // 解析 MIME 内容
                let parsed = parse_mail(body)?;
                
                // Parse headers
                // 解析头部
                let subject = parsed.headers.get_first_value("Subject").unwrap_or("No Subject".to_string());
                let from = parsed.headers.get_first_value("From").unwrap_or("Unknown".to_string());
                
                // Simplified content extraction (Plain text body)
                // 简化的内容提取（纯文本正文）
                let body_text = parsed.get_body().unwrap_or_default();
                
                // 1. Ingest into DB (Identity Resolution)
                // 1. 摄取到数据库（身份解析）
                let sender_id = self.db.create_or_get_participant(&from, None).await?;
                
                // 2. Group into Threads
                // 2. 分组到会话
                // TODO: Implement In-Reply-To/References header checking for proper threading.
                let thread_id = self.db.create_thread(&subject, &body_text).await?;
                
                // 3. Store Message
                // 3. 存储消息
                let message_id = self.db.add_message(&thread_id, &sender_id, &subject, &body_text, "").await?;

                // 4. Trigger Nexus Engine (Automation)
                // 4. 触发 Nexus 引擎（自动化）
                if let Err(e) = self.engine.process_email(&self.db, &message_id, &subject, &from).await {
                    println!("Error processing workflow: {}", e);
                }

                // 5. Index for Search (Sonic)
                // 5. 搜索索引 (Sonic)
                let search_text = format!("{} {}", subject, body_text);
                if let Err(e) = self.search.index_email(&message_id, &search_text).await {
                     println!("Error indexing email: {}", e);
                }
            }
        }

        // Logout
        // 登出
        imap_session.logout().await?;
        Ok(())
    }
}
