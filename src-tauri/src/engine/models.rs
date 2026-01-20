use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TriggerType {
    OnNewEmail,
    OnAttachmentReceived,
    Manual,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trigger {
    pub id: String,
    pub type_: TriggerType,
    pub payload: Option<String>, // e.g., JSON config for the trigger
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ActionType {
    ArchiveEmail,
    MarkAsRead,
    UploadToS3,
    CreateNotionPage,
    SendWebhook,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    pub id: String,
    pub type_: ActionType,
    pub config: String, // JSON string of config (bucket name, url, etc.)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Filter {
    pub id: String,
    pub field: String, // "subject", "sender", "has_attachment"
    pub operator: String, // "contains", "equals", "starts_with"
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Workflow {
    pub id: String,
    pub name: String,
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

    // A simple evaluation function (mock logic for now)
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
