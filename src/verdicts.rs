use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemAction {
    pub action_type: String,
    pub payload: String,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Verdict {
    Approved,
    Rejected(String),
    RejectedWithSuggestion(String, String),
}
