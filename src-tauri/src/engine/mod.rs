pub mod models;
pub mod ingestion;
pub mod search;

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

    pub async fn process_email(&self, db: &Arc<crate::database::Database>, message_id: &str, subject: &str, sender: &str) -> Result<(), Box<dyn std::error::Error>> {
        let workflows = self.workflows.lock().await;
        
        for workflow in workflows.iter() {
            if !workflow.enabled { continue; }
            
            let has_email_trigger = workflow.triggers.iter().any(|t| matches!(t.type_, models::TriggerType::OnNewEmail));
            
            if has_email_trigger {
                if workflow.evaluate(subject, sender) {
                    println!("Workflow '{}' matched for email: {}", workflow.name, subject);
                    self.execute_actions(db, &workflow.actions, message_id).await?;
                }
            }
        }
        Ok(())
    }

    async fn execute_actions(&self, db: &Arc<crate::database::Database>, actions: &[models::Action], message_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        for action in actions {
            match action.type_ {
                models::ActionType::ArchiveEmail => {
                    println!("Executing Action: Archive Email {}", message_id);
                    // In a real implementation, we would update the DB here
                    // db.archive_message(message_id).await?;
                }
                models::ActionType::MarkAsRead => {
                    println!("Executing Action: Mark as Read {}", message_id);
                }
                models::ActionType::UploadToS3 => {
                    println!("Executing Action: Upload to S3 (Simulated)");
                }
                _ => println!("Action type not implemented yet"),
            }
        }
        Ok(())
    }
}
