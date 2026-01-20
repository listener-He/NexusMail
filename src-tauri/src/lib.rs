pub mod config;
pub mod database;
pub mod engine;
pub mod security;

use crate::config::{AppConfig, ConfigManager};
use crate::database::Database;
use crate::engine::Engine;
use crate::engine::models::Workflow;
use crate::engine::ingestion::IngestionService;
use crate::engine::search::SearchService;
use crate::database::models::{Thread, Account};

use tauri::{Manager, State};
use std::sync::Arc;

// --- Tauri Commands / Tauri 命令 ---

/// Get the current application configuration.
/// 获取当前应用程序配置。
#[tauri::command]
async fn get_config(state: State<'_, Arc<ConfigManager>>) -> Result<AppConfig, String> {
    Ok(state.get_config())
}

/// Update the application configuration.
/// 更新应用程序配置。
#[tauri::command]
async fn save_config(state: State<'_, Arc<ConfigManager>>, config: AppConfig) -> Result<(), String> {
    state.update_config(|c| *c = config).map_err(|e| e.to_string())
}

// --- Account Management Commands / 账户管理命令 ---

#[tauri::command]
async fn get_accounts(state: State<'_, Arc<Database>>) -> Result<Vec<Account>, String> {
    state.get_accounts().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_account(
    state: State<'_, Arc<Database>>, 
    email: String, 
    provider: String, 
    credentials: String
) -> Result<String, String> {
    // Verify connection first
    IngestionService::verify_connection(&email, &provider, &credentials)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;

    // If successful, save to DB
    state.create_account(&email, &provider, &credentials).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_account(state: State<'_, Arc<Database>>, id: String) -> Result<(), String> {
    state.delete_account(&id).await.map_err(|e| e.to_string())
}

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
    // Fallback legacy sync for POC components
    ingestion.sync_account(&crate::database::models::Account {
        id: "legacy".to_string(),
        email: email.clone(),
        provider: "legacy".to_string(),
        created_at: chrono::Utc::now(),
        credentials_json: Some(serde_json::json!({
            "server": server,
            "port": "993",
            "password": password
        }).to_string())
    }).await.map_err(|e| e.to_string())
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

#[tauri::command]
async fn change_master_password(state: State<'_, Arc<Database>>, new_key: String) -> Result<(), String> {
    state.change_password(&new_key).await.map_err(|e| e.to_string())
}

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
      let app_handle = app.handle();
      let app_dir = app_handle.path().app_data_dir().unwrap();
      if !app_dir.exists() {
          std::fs::create_dir_all(&app_dir).unwrap();
      }
      
      let config_manager = Arc::new(ConfigManager::new(app_dir.clone()));
      let config = config_manager.get_config();

      // 2. Initialize Database / 初始化数据库
      let db_path = app_dir.join("nexusmail.db");
      
      // Use a hardcoded key for now, in production use Keychain
      let db = Database::new(db_path, "dev_secret_key").expect("Failed to init database");
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
      let sonic_host = config.search.host.clone();
      let sonic_port = config.search.port;
      // Password is not in config for security, using hardcoded for now
      let sonic_password = "SecretPassword".to_string(); 

      tauri::async_runtime::spawn(async move {
          match search_clone.connect(&sonic_host, sonic_port, &sonic_password).await {
              Ok(port) => log::info!("Connected to Search Engine on port {}", port),
              Err(e) => log::warn!("Failed to connect to Search Engine: {}", e),
          }
      });

      // 5. Manage State / 管理状态
      app.manage(config_manager);
      app.manage(db_arc.clone());
      app.manage(engine.clone());
      app.manage(search.clone());

      // Start Background Sync Loop
      let sync_db = db_arc.clone();
      let sync_engine = engine.clone();
      let sync_search = search.clone();
      
      tauri::async_runtime::spawn(async move {
          let ingestion = IngestionService::new(sync_db.clone(), sync_engine, sync_search);
          loop {
              tokio::time::sleep(tokio::time::Duration::from_secs(300)).await; // Sync every 5 minutes
              log::info!("Starting scheduled sync...");
              
              if let Ok(accounts) = sync_db.get_accounts().await {
                  for account in accounts {
                      if let Err(e) = ingestion.sync_account(&account).await {
                          log::error!("Failed to sync account {}: {}", account.email, e);
                      }
                  }
              }
          }
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
        get_config, save_config, 
        get_accounts, create_account, delete_account,
        change_master_password,
        get_threads, get_workflows, save_workflow, sync_account, search_emails
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
