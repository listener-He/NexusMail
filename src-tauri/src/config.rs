use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LLMConfig {
    pub provider: String, // "openai", "ollama", "local"
    pub model: String,
    pub base_url: Option<String>,
    // API Key should be stored in the encrypted DB or Keychain, not here in plain text
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchConfig {
    pub host: String,
    pub port: u16,
    // Password should be stored in the encrypted DB
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub language: String,
    pub theme: String, // "light", "dark", "system"
    pub llm: LLMConfig,
    pub search: SearchConfig,
    pub first_run: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            theme: "system".to_string(),
            llm: LLMConfig {
                provider: "ollama".to_string(),
                model: "llama3".to_string(),
                base_url: Some("http://localhost:11434".to_string()),
            },
            search: SearchConfig {
                host: "127.0.0.1".to_string(),
                port: 1491,
            },
            first_run: true,
        }
    }
}

pub struct ConfigManager {
    pub config: Mutex<AppConfig>,
    pub path: PathBuf,
}

impl ConfigManager {
    pub fn new(app_dir: PathBuf) -> Self {
        let config_path = app_dir.join("config.json");
        let config = if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
                Err(_) => AppConfig::default(),
            }
        } else {
            AppConfig::default()
        };

        Self {
            config: Mutex::new(config),
            path: config_path,
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.config.lock().unwrap();
        let content = serde_json::to_string_pretty(&*config)?;
        fs::write(&self.path, content)?;
        Ok(())
    }

    pub fn get_config(&self) -> AppConfig {
        self.config.lock().unwrap().clone()
    }

    pub fn update_config<F>(&self, update_fn: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut AppConfig),
    {
        {
            let mut config = self.config.lock().unwrap();
            update_fn(&mut *config);
        } // Unlock before saving
        self.save()
    }
}
