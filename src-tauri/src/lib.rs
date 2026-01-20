pub mod database;
pub mod engine;
pub mod security;

use crate::database::Database;
use crate::engine::Engine;
use crate::engine::models::Workflow;
use tauri::{Manager, State};
use std::sync::Arc;
use crate::database::models::Thread;
use chrono::Utc;

#[tauri::command]
async fn get_workflows(state: State<'_, Arc<Engine>>) -> Result<Vec<Workflow>, String> {
    Ok(state.get_workflows().await)
}

#[tauri::command]
async fn save_workflow(state: State<'_, Arc<Engine>>, yaml: String) -> Result<(), String> {
    state.load_workflow(&yaml).await.map_err(|e| e.to_string())
}

// Command to fetch threads (Mock implementation for now)
use crate::engine::ingestion::IngestionService;

use crate::engine::search::SearchService;

#[tauri::command]
async fn sync_account(state: State<'_, Arc<Database>>, engine: State<'_, Arc<Engine>>, search: State<'_, Arc<SearchService>>, email: String, password: String, server: String) -> Result<(), String> {
    let ingestion = IngestionService::new(state.inner().clone(), engine.inner().clone(), search.inner().clone());
    ingestion.sync_account(&email, &password, &server).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn search_emails(search: State<'_, Arc<SearchService>>, query: String) -> Result<Vec<String>, String> {
    search.search(&query).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_threads(state: State<'_, Arc<Database>>) -> Result<Vec<Thread>, String> {
    // Return real data from DB
    state.get_all_threads().await.map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // Initialize Database
      // Note: In production, the path should be app_data_dir/nexusmail.db and key from Keychain
      let app_handle = app.handle();
      let app_dir = app_handle.path().app_data_dir().unwrap();
      if !app_dir.exists() {
          std::fs::create_dir_all(&app_dir).unwrap();
      }
      let db_path = app_dir.join("nexusmail.db");
      
      // Using a hardcoded key for development ONLY. 
      // TODO: Integrate with Keychain for key management.
      let db = Database::new(db_path, "dev_secret_key").expect("Failed to init database");
      
      // Run migrations async
      let db_arc = Arc::new(db);
      let db_clone = db_arc.clone();
      tauri::async_runtime::spawn(async move {
          db_clone.init().await.expect("Failed to run migrations");
      });

      // Initialize Engine
      let engine = Arc::new(Engine::new());
      
      // Initialize Search Service
      let search = Arc::new(SearchService::new());
      let search_clone = search.clone();
      
      // Connect to Sonic (Localhost for now)
      tauri::async_runtime::spawn(async move {
          // Note: In production, ensure Sonic is running or start it as a sidecar
          match search_clone.connect("127.0.0.1", 1491, "SecretPassword").await {
              Ok(port) => log::info!("Connected to Search Engine on port {}", port),
              Err(e) => log::warn!("Failed to connect to Search Engine: {}", e),
          }
      });

      app.manage(db_arc);
      app.manage(engine);
      app.manage(search);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_threads, get_workflows, save_workflow, sync_account, search_emails])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
