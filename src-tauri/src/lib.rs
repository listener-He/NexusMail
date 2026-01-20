pub mod database;
pub mod engine;
pub mod security;

use crate::database::Database;
use tauri::{Manager, State};
use std::sync::Arc;
use crate::database::models::Thread;
use chrono::Utc;

// Command to fetch threads (Mock implementation for now)
#[tauri::command]
async fn get_threads(state: State<'_, Arc<Database>>) -> Result<Vec<Thread>, String> {
    // In a real scenario, we would query the database here.
    // let conn = state.conn.lock().await;
    // ... query logic ...
    
    // Returning mock data for UI development
    Ok(vec![
        Thread {
            id: "1".to_string(),
            subject: Some("Invoice #1023".to_string()),
            snippet: Some("Please find attached the invoice for last month...".to_string()),
            last_message_at: Some(Utc::now()),
            is_read: false,
            is_archived: false,
            tags: vec!["finance".to_string()],
        },
        Thread {
            id: "2".to_string(),
            subject: Some("Project Update: NexusMail".to_string()),
            snippet: Some("The new design system looks great! Let's proceed...".to_string()),
            last_message_at: Some(Utc::now()),
            is_read: true,
            is_archived: false,
            tags: vec!["work".to_string()],
        },
    ])
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

      app.manage(db_arc);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_threads])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
