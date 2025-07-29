use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRequest {
    pub recommendation_id: String,
    pub action: ActionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Approve,
    Reject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResponse {
    pub success: bool,
    pub message: String,
}

pub async fn process_recommendation_action(request: ActionRequest) -> ActionResponse {
    match request.action {
        ActionType::Approve => {
            ActionResponse {
                success: true,
                message: format!("Recommendation {} approved", request.recommendation_id),
            }
        }
        ActionType::Reject => {
            ActionResponse {
                success: true,
                message: format!("Recommendation {} rejected", request.recommendation_id),
            }
        }
    }
}