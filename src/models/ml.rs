use serde::{Deserialize, Serialize};

use crate::api::{
    local_lightning_client::LocalChannelInfo,
    mcp_client::{ActionType, Priority},
};

/// Résumé chiffré produit par le moteur ML local.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLScorecard {
    pub predicted_roi_30d: f64,
    pub predicted_roi_90d: f64,
    pub predicted_roi_365d: f64,
    pub risk_index: f64,
    pub capacity_saturation: f64,
    pub advantage_vs_amboss: f64,
    pub confidence: f64,
}

/// Insight ponctuel sur la performance ou les risques.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLInsight {
    pub title: String,
    pub detail: String,
    pub impact: f64,
    pub confidence: f64,
}

/// Recommandation enrichie avec le contexte ML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartRecommendation {
    pub id: String,
    pub action_type: ActionType,
    pub priority: Priority,
    pub expected_roi_impact: f64,
    pub confidence: f64,
    pub risk_score: f64,
    pub rationale: Vec<String>,
    pub target_channels: Vec<String>,
}

/// Recommandation d’exécution automatique après analyse ML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationReadiness {
    pub ready: bool,
    pub mode: String,
    pub gating_factors: Vec<String>,
    pub confidence: f64,
    pub recommended_daily_actions: u32,
    pub ml_confidence_threshold: f64,
}

/// Projection détaillée de la simulation d’action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationOutcome {
    pub recommendation_id: String,
    pub expected_roi: f64,
    pub success_probability: f64,
    pub risk_level: String,
    pub estimated_cost: u64,
    pub timeline: Vec<SimulationStep>,
}

/// Étape chronologique dans une simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationStep {
    pub label: String,
    pub expected_outcome: String,
    pub probability: f64,
    pub delta_roi: f64,
}

/// Fenêtre optimale proposée par le moteur ML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimalWindow {
    pub window: String,
    pub confidence: f64,
    pub reasons: Vec<String>,
}

/// Photographie rapide des canaux utilisée par le moteur ML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSnapshot {
    pub channel_id: String,
    pub capacity: u64,
    pub local_ratio: f64,
    pub forwards: u32,
    pub uptime: f64,
}

impl From<&LocalChannelInfo> for ChannelSnapshot {
    fn from(channel: &LocalChannelInfo) -> Self {
        let capacity = channel.capacity.max(1);
        let local_ratio = channel.local_balance as f64 / capacity as f64;
        Self {
            channel_id: channel.channel_id.clone(),
            capacity,
            local_ratio,
            forwards: channel.total_satoshis_sent as u32 + channel.total_satoshis_received as u32,
            uptime: 0.94, // Valeur par défaut faute de données temps réel
        }
    }
}
