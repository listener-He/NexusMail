use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

/// 同步状态 (Sync Status)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncStatus {
    Idle,
    Syncing,
    Error(String),
    Success,
}

/// 同步渠道特征 (Sync Channel Trait)
/// Defines the interface for any remote sync destination (S3, WebDAV, Postgres).
/// 定义任何远程同步目标（S3、WebDAV、Postgres）的接口。
#[async_trait]
pub trait SyncChannel: Send + Sync {
    /// Initialize connection with encrypted config
    async fn connect(&mut self, config_json: &str) -> Result<(), Box<dyn Error + Send + Sync>>;
    
    /// Perform a full backup (upload snapshot)
    async fn backup(&self, data: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>>;
    
    /// Perform an incremental sync (mock for now)
    async fn sync(&self) -> Result<(), Box<dyn Error + Send + Sync>>;
    
    /// Get channel type identifier
    fn name(&self) -> &str;
}

/// 模拟渠道 (Mock Channel)
/// A dummy implementation for testing the UI and architecture.
pub struct MockChannel {
    pub name: String,
    pub status: SyncStatus,
}

impl MockChannel {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            status: SyncStatus::Idle,
        }
    }
}

#[async_trait]
impl SyncChannel for MockChannel {
    async fn connect(&mut self, _config_json: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Simulate connection delay
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        println!("MockChannel connected: {}", self.name);
        Ok(())
    }

    async fn backup(&self, data: &[u8]) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("MockChannel {} backing up {} bytes...", self.name, data.len());
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        println!("MockChannel backup complete.");
        Ok(())
    }

    async fn sync(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("MockChannel {} syncing...", self.name);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// 同步引擎 (Sync Engine)
/// Manages multiple active sync channels.
pub struct SyncEngine {
    pub channels: Arc<Mutex<Vec<Box<dyn SyncChannel>>>>,
}

impl SyncEngine {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn add_channel(&self, channel: Box<dyn SyncChannel>) {
        let mut channels = self.channels.lock().await;
        channels.push(channel);
    }

    pub async fn run_backup_all(&self) {
        let channels = self.channels.lock().await;
        for channel in channels.iter() {
            // In a real app, we'd stream the DB file here
            let dummy_data = vec![0; 1024]; 
            if let Err(e) = channel.backup(&dummy_data).await {
                println!("Backup failed for {}: {}", channel.name(), e);
            }
        }
    }
}
