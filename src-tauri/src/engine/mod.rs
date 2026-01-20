pub mod models;

use self::models::Workflow;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Engine {
    pub workflows: Arc<Mutex<Vec<Workflow>>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            workflows: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn load_workflow(&self, yaml_content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let workflow: Workflow = serde_yaml::from_str(yaml_content)?;
        let mut workflows = self.workflows.lock().await;
        workflows.push(workflow);
        Ok(())
    }

    pub async fn get_workflows(&self) -> Vec<Workflow> {
        let workflows = self.workflows.lock().await;
        workflows.clone()
    }
}
