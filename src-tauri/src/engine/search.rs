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

    pub async fn connect(&self, host: &str, start_port: u16, password: &str) -> Result<u16, Box<dyn std::error::Error>> {
        let ports = vec![start_port, start_port + 1, start_port + 2];
        
        for port in ports {
            let addr = format!("{}:{}", host, port);
            println!("Attempting to connect to Sonic at {}", addr);
            
            match IngestChannel::start(&addr, password).await {
                Ok(ingest) => {
                    // If ingest works, try search channel
                    match SearchChannel::start(&addr, password).await {
                        Ok(search) => {
                            println!("Successfully connected to Sonic at {}", addr);
                            *self.ingest_channel.lock().await = Some(ingest);
                            *self.search_channel.lock().await = Some(search);
                            return Ok(port);
                        },
                        Err(_) => continue,
                    }
                },
                Err(_) => continue,
            }
        }
        
        Err("Failed to connect to Sonic on any attempted port".into())
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
