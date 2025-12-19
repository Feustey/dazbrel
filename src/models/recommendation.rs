use crate::api::mcp_client::{ActionType, MCPRecommendation, Priority};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: String,
    pub action_type: ActionType,
    pub priority: Priority,
    pub expected_roi_impact: f64,
    pub description: String,
    pub parameters: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub status: RecommendationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
    Failed,
}

impl From<MCPRecommendation> for Recommendation {
    fn from(mcp_rec: MCPRecommendation) -> Self {
        Self {
            id: mcp_rec.id,
            action_type: mcp_rec.action_type,
            priority: mcp_rec.priority,
            expected_roi_impact: mcp_rec.expected_roi_impact,
            description: mcp_rec.description,
            parameters: mcp_rec.parameters,
            created_at: mcp_rec.created_at,
            status: RecommendationStatus::Pending,
        }
    }
}

impl Recommendation {
    pub fn priority_class(&self) -> &'static str {
        match self.priority {
            Priority::High => "high",
            Priority::Medium => "medium",
            Priority::Low => "low",
        }
    }

    pub fn action_type_display(&self) -> &'static str {
        match self.action_type {
            ActionType::OpenChannel => "Open Channel",
            ActionType::CloseChannel => "Close Channel",
            ActionType::AdjustFees => "Adjust Fees",
            ActionType::RebalanceChannel => "Rebalance Channel",
        }
    }
}
