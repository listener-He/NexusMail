use sonic_channel::*;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SearchService {
    ingest_channel: Arc<Mutex<Option<IngestChannel>>>,
    search_channel: Arc<Mutex<Option<SearchChannel>>>,
}

impl SearchService {
    pub fn new() -> Self {
        Self {
            ingest_channel: Arc::new(Mutex::new(None)),
            search_channel: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn connect(&self, addr: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Ingest Connection
        let ingest = IngestChannel::start(addr, password).await?;
        *self.ingest_channel.lock().await = Some(ingest);

        // Search Connection
        let search = SearchChannel::start(addr, password).await?;
        *self.search_channel.lock().await = Some(search);

        Ok(())
    }

    pub async fn index_email(&self, id: &str, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut guard = self.ingest_channel.lock().await;
        if let Some(channel) = guard.as_mut() {
            channel.push("emails", "default", id, text).await?;
        }
        Ok(())
    }

    pub async fn search(&self, query: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut guard = self.search_channel.lock().await;
        if let Some(channel) = guard.as_mut() {
            let results = channel.query("emails", "default", query).await?;
            return Ok(results);
        }
        Ok(vec![])
    }
}
