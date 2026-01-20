use sonic_channel::*;
use std::sync::Arc;
use tokio::sync::Mutex;

/// 搜索服务 (Search Service)
/// Manages connection to the Sonic search engine backend.
/// 管理与 Sonic 搜索引擎后端的连接。
pub struct SearchService {
    /// Channel for pushing data (Indexing) / 推送数据（索引）的通道
    ingest_channel: Arc<Mutex<Option<IngestChannel>>>,
    /// Channel for querying data (Searching) / 查询数据（搜索）的通道
    search_channel: Arc<Mutex<Option<SearchChannel>>>,
}

impl SearchService {
    pub fn new() -> Self {
        Self {
            ingest_channel: Arc::new(Mutex::new(None)),
            search_channel: Arc::new(Mutex::new(None)),
        }
    }

    /// Connect to the Sonic server with port fallback mechanism.
    /// 使用端口回退机制连接到 Sonic 服务器。
    ///
    /// # Arguments
    /// * `host` - Server hostname (e.g., "127.0.0.1") / 服务器主机名
    /// * `start_port` - Starting port number to attempt (e.g., 1491) / 尝试的起始端口号
    /// * `password` - Authentication password / 认证密码
    ///
    /// # Returns
    /// * `Ok(u16)` - The port successfully connected to / 成功连接的端口
    pub async fn connect(&self, host: &str, start_port: u16, password: &str) -> Result<u16, Box<dyn std::error::Error>> {
        // Attempt to connect to ports in sequence (e.g., 1491, 1492, 1493)
        // 依次尝试连接端口
        let ports = vec![start_port, start_port + 1, start_port + 2];
        
        for port in ports {
            let addr = format!("{}:{}", host, port);
            println!("Attempting to connect to Sonic at {}", addr);
            
            // Try to establish Ingest connection
            // 尝试建立索引连接
            match IngestChannel::start(&addr, password).await {
                Ok(ingest) => {
                    // If ingest works, try search channel
                    // 如果索引连接成功，尝试建立搜索连接
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

    /// Index an email for full-text search.
    /// 为电子邮件建立全文搜索索引。
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the object / 对象的唯一标识符
    /// * `text` - The text content to index / 要索引的文本内容
    pub async fn index_email(&self, id: &str, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut guard = self.ingest_channel.lock().await;
        if let Some(channel) = guard.as_mut() {
            // Push to "emails" collection, "default" bucket
            // 推送到 "emails" 集合，"default" 存储桶
            channel.push("emails", "default", id, text).await?;
        }
        Ok(())
    }

    /// Perform a full-text search query.
    /// 执行全文搜索查询。
    ///
    /// # Returns
    /// * `Vec<String>` - List of matching object IDs / 匹配的对象 ID 列表
    pub async fn search(&self, query: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut guard = self.search_channel.lock().await;
        if let Some(channel) = guard.as_mut() {
            // Query "emails" collection, "default" bucket
            // 查询 "emails" 集合，"default" 存储桶
            let results = channel.query("emails", "default", query).await?;
            return Ok(results);
        }
        Ok(vec![])
    }
}
