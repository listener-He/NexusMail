pub mod database;
pub mod engine;
pub mod security;

use crate::database::Database;
use crate::engine::Engine;
use crate::engine::models::Workflow;
use crate::engine::ingestion::IngestionService;
use crate::engine::search::SearchService;
use crate::database::models::Thread;

use tauri::{Manager, State};
use std::sync::Arc;

// --- Configuration Struct / 配置结构 ---
// In a real app, this would be loaded from a config file or env vars.
// 在实际应用中，这将从配置文件或环境变量加载。
struct AppConfig {
    db_key: String,
    sonic_host: String,
    sonic_password: String,
    sonic_port: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_key: "dev_secret_key".to_string(), // TODO: Use Keychain / 使用钥匙串
            sonic_host: "127.0.0.1".to_string(),
            sonic_password: "SecretPassword".to_string(),
            sonic_port: 1491,
        }
    }
}

// --- Tauri Commands / Tauri 命令 ---

/// Retrieve all active workflows.
/// 获取所有活动工作流。
#[tauri::command]
async fn get_workflows(state: State<'_, Arc<Engine>>) -> Result<Vec<Workflow>, String> {
    Ok(state.get_workflows().await)
}

/// Save a workflow configuration (YAML).
/// 保存工作流配置 (YAML)。
#[tauri::command]
async fn save_workflow(state: State<'_, Arc<Engine>>, yaml: String) -> Result<(), String> {
    state.load_workflow(&yaml).await.map_err(|e| e.to_string())
}

/// Sync an external email account.
/// 同步外部电子邮件帐户。
/// Triggers ingestion, storage, indexing, and automation.
/// 触发摄取、存储、索引和自动化。
#[tauri::command]
async fn sync_account(state: State<'_, Arc<Database>>, engine: State<'_, Arc<Engine>>, search: State<'_, Arc<SearchService>>, email: String, password: String, server: String) -> Result<(), String> {
    let ingestion = IngestionService::new(state.inner().clone(), engine.inner().clone(), search.inner().clone());
    ingestion.sync_account(&email, &password, &server).await.map_err(|e| e.to_string())
}

/// Search for emails using the full-text search engine.
/// 使用全文搜索引擎搜索电子邮件。
#[tauri::command]
async fn search_emails(search: State<'_, Arc<SearchService>>, query: String) -> Result<Vec<String>, String> {
    search.search(&query).await.map_err(|e| e.to_string())
}

/// Retrieve all threads from the local database.
/// 从本地数据库获取所有会话。
#[tauri::command]
async fn get_threads(state: State<'_, Arc<Database>>) -> Result<Vec<Thread>, String> {
    state.get_all_threads().await.map_err(|e| e.to_string())
}

// --- Application Entry Point / 应用程序入口点 ---

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      // 1. Setup Logging / 设置日志
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // Load Config
      // 加载配置
      let config = AppConfig::default();

      // 2. Initialize Database / 初始化数据库
      let app_handle = app.handle();
      let app_dir = app_handle.path().app_data_dir().unwrap();
      if !app_dir.exists() {
          std::fs::create_dir_all(&app_dir).unwrap();
      }
      let db_path = app_dir.join("nexusmail.db");
      
      let db = Database::new(db_path, &config.db_key).expect("Failed to init database");
      let db_arc = Arc::new(db);
      
      // Run DB Migrations (Async)
      // 运行数据库迁移（异步）
      let db_clone = db_arc.clone();
      tauri::async_runtime::spawn(async move {
          db_clone.init().await.expect("Failed to run migrations");
      });

      // 3. Initialize Nexus Engine (Workflow) / 初始化 Nexus 引擎（工作流）
      let engine = Arc::new(Engine::new());
      
      // 4. Initialize Search Service (Sonic) / 初始化搜索服务 (Sonic)
      let search = Arc::new(SearchService::new());
      let search_clone = search.clone();
      
      // Connect to Sonic (Async Background Task)
      // 连接到 Sonic（异步后台任务）
      tauri::async_runtime::spawn(async move {
          match search_clone.connect(&config.sonic_host, config.sonic_port, &config.sonic_password).await {
              Ok(port) => log::info!("Connected to Search Engine on port {}", port),
              Err(e) => log::warn!("Failed to connect to Search Engine: {}", e),
          }
      });

      // 5. Manage State / 管理状态
      app.manage(db_arc);
      app.manage(engine);
      app.manage(search);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_threads, get_workflows, save_workflow, sync_account, search_emails])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
