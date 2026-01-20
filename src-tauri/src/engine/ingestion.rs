use async_imap::error::Error;
use futures::stream::StreamExt;
use mailparse::*;
use std::sync::Arc;
use crate::database::Database;
use crate::database::models::Account;

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

#[derive(serde::Deserialize)]
struct ImapConfig {
    server: String,
    port: String,
    password: String,
}

impl IngestionService {
    pub fn new(db: Arc<Database>, engine: Arc<Engine>, search: Arc<SearchService>) -> Self {
        Self { db, engine, search }
    }

    /// Verify connection to an IMAP server.
    /// 验证与 IMAP 服务器的连接。
    pub async fn verify_connection(email: &str, provider: &str, credentials_json: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config: ImapConfig = serde_json::from_str(credentials_json)?;
        let domain = &config.server;
        let port = config.port.parse::<u16>()?;
        
        let tls = async_native_tls::TlsConnector::new();
        let client = async_imap::connect((domain.as_str(), port), domain, tls).await?;
        
        let mut imap_session = client.login(email, &config.password).await.map_err(|e| e.0)?;
        imap_session.logout().await?;
        
        Ok(())
    }

    /// Sync emails from a stored Account.
    /// 从存储的帐户同步电子邮件。
    pub async fn sync_account(&self, account: &Account) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(creds) = &account.credentials_json {
            let config: ImapConfig = serde_json::from_str(creds)?;
            let domain = &config.server;
            let port = config.port.parse::<u16>()?;
            let email = &account.email;
            
            let tls = async_native_tls::TlsConnector::new();
            let client = async_imap::connect((domain.as_str(), port), domain, tls).await?;
            
            let mut imap_session = client.login(email, &config.password).await.map_err(|e| e.0)?;
            
            imap_session.select("INBOX").await?;
            
            // TODO: Use UID based fetching for incremental sync
            // Fetch last 10 emails
            let messages = imap_session.fetch("1:10", "RFC822").await?;
            
            for message in messages.iter() {
                if let Some(body) = message.body() {
                    let parsed = parse_mail(body)?;
                    
                    let subject = parsed.headers.get_first_value("Subject").unwrap_or("No Subject".to_string());
                    let from = parsed.headers.get_first_value("From").unwrap_or("Unknown".to_string());
                    
                    let body_text = parsed.get_body().unwrap_or_default();
                    let body_html = body_text.clone(); // TODO: Extract real HTML
                    
                    let sender_id = self.db.create_or_get_participant(&from, None).await?;
                    let thread_id = self.db.create_thread(&subject, &body_text).await?;
                    
                    let message_id = self.db.add_message(&thread_id, &sender_id, &subject, &body_text, &body_html).await?;
                    
                    // Trigger Nexus Engine
                    if let Err(e) = self.engine.process_email(&self.db, &message_id, &subject, &from).await {
                        println!("Error processing workflow: {}", e);
                    }
                    
                    // Index for Search
                    let search_text = format!("{} {}", subject, body_text);
                    if let Err(e) = self.search.index_email(&message_id, &search_text).await {
                        println!("Error indexing email: {}", e);
                    }
                }
            }
            
            imap_session.logout().await?;
        }
        Ok(())
    }
}
