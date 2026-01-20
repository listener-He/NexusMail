use async_imap::error::Error;
use futures::stream::StreamExt;
use mailparse::*;
use std::sync::Arc;
use crate::database::Database;

use crate::engine::Engine;
use crate::engine::search::SearchService;

pub struct IngestionService {
    db: Arc<Database>,
    engine: Arc<Engine>,
    search: Arc<SearchService>,
}

impl IngestionService {
    pub fn new(db: Arc<Database>, engine: Arc<Engine>, search: Arc<SearchService>) -> Self {
        Self { db, engine, search }
    }

    pub async fn sync_account(&self, email: &str, password: &str, server: &str) -> Result<(), Box<dyn std::error::Error>> {
        let domain = server;
        let tls = async_native_tls::TlsConnector::new();
        let client = async_imap::connect((domain, 993), domain, tls).await?;

        let mut imap_session = client.login(email, password).await.map_err(|e| e.0)?;

        imap_session.select("INBOX").await?;

        // Fetch last 5 emails for testing
        let messages = imap_session.fetch("1:5", "RFC822").await?;

        for message in messages.iter() {
            if let Some(body) = message.body() {
                let parsed = parse_mail(body)?;
                
                // Parse headers
                let subject = parsed.headers.get_first_value("Subject").unwrap_or("No Subject".to_string());
                let from = parsed.headers.get_first_value("From").unwrap_or("Unknown".to_string());
                
                // Simplified content extraction
                let body_text = parsed.get_body().unwrap_or_default();
                
                // Ingest into DB
                let sender_id = self.db.create_or_get_participant(&from, None).await?;
                
                // Check if thread exists (simplified: new thread for every sync in this POC)
                // In reality, we'd check In-Reply-To or Subject grouping
                let thread_id = self.db.create_thread(&subject, &body_text).await?;
                
                let message_id = self.db.add_message(&thread_id, &sender_id, &subject, &body_text, "").await?;

                // Trigger Nexus Engine
                if let Err(e) = self.engine.process_email(&self.db, &message_id, &subject, &from).await {
                    println!("Error processing workflow: {}", e);
                }

                // Index for Search
                // In production, you might want to strip HTML tags or use the body_text more intelligently
                let search_text = format!("{} {}", subject, body_text);
                if let Err(e) = self.search.index_email(&message_id, &search_text).await {
                     println!("Error indexing email: {}", e);
                }
            }
        }

        imap_session.logout().await?;
        Ok(())
    }
}
