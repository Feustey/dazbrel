use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetrics {
    pub current_roi: f64,
    pub total_channels: u32,
    pub active_channels: u32,
    pub total_capacity: u64,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub pending_htlcs: u32,
    pub fees_earned_24h: u64,
    pub fees_earned_7d: u64,
    pub fees_earned_30d: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMetrics {
    pub channel_id: String,
    pub peer_pubkey: String,
    pub capacity: u64,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub fee_rate: u32,
    pub base_fee: u32,
    pub htlc_count: u32,
    pub last_update: chrono::DateTime<chrono::Utc>,
}

impl Default for NodeMetrics {
    fn default() -> Self {
        Self {
            current_roi: 0.0,
            total_channels: 0,
            active_channels: 0,
            total_capacity: 0,
            local_balance: 0,
            remote_balance: 0,
            pending_htlcs: 0,
            fees_earned_24h: 0,
            fees_earned_7d: 0,
            fees_earned_30d: 0,
        }
    }
}