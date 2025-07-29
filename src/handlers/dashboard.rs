use serde::{Deserialize, Serialize};
use crate::models::{metrics::NodeMetrics, recommendation::Recommendation};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    pub connection_status: String,
    pub current_roi: f64,
    pub metrics: NodeMetrics,
    pub recommendations: Vec<Recommendation>,
    pub recent_actions: Vec<ActionHistory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionHistory {
    pub id: String,
    pub action_type: String,
    pub executed_at: chrono::DateTime<chrono::Utc>,
    pub success: bool,
    pub impact: Option<f64>,
}

impl Default for DashboardData {
    fn default() -> Self {
        Self {
            connection_status: "disconnected".to_string(),
            current_roi: 0.0,
            metrics: NodeMetrics::default(),
            recommendations: vec![],
            recent_actions: vec![],
        }
    }
}