pub mod models;
pub mod ingestion;
pub mod search;

use self::models::Workflow;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Nexus 引擎 (Nexus Engine)
/// The core automation system that manages and executes workflows.
/// 管理和执行工作流的核心自动化系统。
pub struct Engine {
    /// List of active workflows stored in memory / 存储在内存中的活动工作流列表
    pub workflows: Arc<Mutex<Vec<Workflow>>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            workflows: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Load a workflow from a YAML string.
    /// 从 YAML 字符串加载工作流。
    pub async fn load_workflow(&self, yaml_content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let workflow: Workflow = serde_yaml::from_str(yaml_content)?;
        let mut workflows = self.workflows.lock().await;
        workflows.push(workflow);
        Ok(())
    }

    /// Retrieve all loaded workflows.
    /// 检索所有已加载的工作流。
    pub async fn get_workflows(&self) -> Vec<Workflow> {
        let workflows = self.workflows.lock().await;
        workflows.clone()
    }

    /// Process a newly received email against all active workflows.
    /// 针对所有活动工作流处理新接收的电子邮件。
    ///
    /// # Arguments
    /// * `db` - Database reference / 数据库引用
    /// * `message_id` - ID of the email message / 电子邮件消息ID
    /// * `subject` - Email subject / 邮件主题
    /// * `sender` - Email sender / 邮件发送者
    pub async fn process_email(&self, db: &Arc<crate::database::Database>, message_id: &str, subject: &str, sender: &str) -> Result<(), Box<dyn std::error::Error>> {
        let workflows = self.workflows.lock().await;
        
        for workflow in workflows.iter() {
            if !workflow.enabled { continue; }
            
            // Check if workflow has an "OnNewEmail" trigger
            // 检查工作流是否有 "OnNewEmail" 触发器
            let has_email_trigger = workflow.triggers.iter().any(|t| matches!(t.type_, models::TriggerType::OnNewEmail));
            
            if has_email_trigger {
                // Evaluate filters
                // 评估过滤器
                if workflow.evaluate(subject, sender) {
                    println!("Workflow '{}' matched for email: {}", workflow.name, subject);
                    self.execute_actions(db, &workflow.actions, message_id).await?;
                }
            }
        }
        Ok(())
    }

    /// Execute a list of actions.
    /// 执行动作列表。
    async fn execute_actions(&self, db: &Arc<crate::database::Database>, actions: &[models::Action], message_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        for action in actions {
            match action.type_ {
                models::ActionType::ArchiveEmail => {
                    println!("Executing Action: Archive Email {}", message_id);
                    // Update DB state
                    // 更新数据库状态
                    if let Err(e) = db.archive_message(message_id).await {
                        println!("Failed to archive message: {}", e);
                    }
                }
                models::ActionType::MarkAsRead => {
                    println!("Executing Action: Mark as Read {}", message_id);
                    // TODO: Implement DB update for Read status
                }
                models::ActionType::UploadToS3 => {
                    println!("Executing Action: Upload to S3 (Simulated)");
                    // TODO: Implement S3 upload logic
                }
                _ => println!("Action type not implemented yet"),
            }
        }
        Ok(())
    }
}
