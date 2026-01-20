use serde::{Deserialize, Serialize};

/// 触发器类型 (Trigger Type)
/// Defines when a workflow should start.
/// 定义工作流何时应该启动。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TriggerType {
    /// Triggers when a new email arrives / 当新邮件到达时触发
    OnNewEmail,
    /// Triggers when an attachment is detected / 当检测到附件时触发
    OnAttachmentReceived,
    /// Manually triggered by user / 用户手动触发
    Manual,
}

/// 触发器 (Trigger)
/// The event that initiates the workflow.
/// 启动工作流的事件。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trigger {
    pub id: String,
    pub type_: TriggerType,
    /// Optional configuration (JSON) for the trigger / 触发器的可选配置 (JSON)
    pub payload: Option<String>, 
}

/// 动作类型 (Action Type)
/// Defines what operation to perform.
/// 定义要执行的操作。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ActionType {
    /// Move email to archive / 将邮件移至归档
    ArchiveEmail,
    /// Mark email as read / 标记邮件为已读
    MarkAsRead,
    /// Upload attachment to AWS S3 / 上传附件到 AWS S3
    UploadToS3,
    /// Create a page in Notion / 在 Notion 中创建页面
    CreateNotionPage,
    /// Send a JSON payload to a webhook URL / 发送 JSON 负载到 Webhook URL
    SendWebhook,
}

/// 动作 (Action)
/// An operation to be executed when the workflow runs.
/// 工作流运行时要执行的操作。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    pub id: String,
    pub type_: ActionType,
    /// Configuration for the action (e.g., S3 bucket name) / 动作的配置（例如 S3 存储桶名称）
    pub config: String, 
}

/// 过滤器 (Filter)
/// Conditions that must be met for the workflow to proceed.
/// 工作流继续执行必须满足的条件。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Filter {
    pub id: String,
    /// Field to check (e.g., "subject", "sender") / 要检查的字段
    pub field: String, 
    /// Comparison operator (e.g., "contains", "equals") / 比较运算符
    pub operator: String, 
    /// Value to compare against / 要比较的值
    pub value: String,
}

/// 工作流 (Workflow)
/// A collection of triggers, filters, and actions.
/// 触发器、过滤器和动作的集合。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Workflow {
    pub id: String,
    /// Display name of the workflow / 工作流的显示名称
    pub name: String,
    /// Whether the workflow is active / 工作流是否处于活动状态
    pub enabled: bool,
    pub triggers: Vec<Trigger>,
    pub filters: Vec<Filter>,
    pub actions: Vec<Action>,
}

impl Workflow {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            enabled: true,
            triggers: vec![],
            filters: vec![],
            actions: vec![],
        }
    }

    /// Evaluate filters against email metadata.
    /// 根据邮件元数据评估过滤器。
    /// Returns true if all filters pass.
    /// 如果所有过滤器都通过，则返回 true。
    pub fn evaluate(&self, email_subject: &str, sender: &str) -> bool {
        for filter in &self.filters {
            let match_found = match filter.field.as_str() {
                "subject" => match filter.operator.as_str() {
                    "contains" => email_subject.contains(&filter.value),
                    "equals" => email_subject == filter.value,
                    _ => false,
                },
                "sender" => match filter.operator.as_str() {
                    "contains" => sender.contains(&filter.value),
                    "equals" => sender == filter.value,
                    _ => false,
                },
                _ => false,
            };
            
            if !match_found {
                return false;
            }
        }
        true
    }
}
