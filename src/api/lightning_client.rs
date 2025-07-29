use serde::{Deserialize, Serialize};
use tracing::info;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub pubkey: String,
    pub alias: String,
    pub num_channels: u32,
    pub num_active_channels: u32,
    pub block_height: u32,
    pub synced_to_chain: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelInfo {
    pub channel_id: String,
    pub peer_pubkey: String,
    pub capacity: u64,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelParams {
    pub peer_pubkey: String,
    pub amount: u64,
    pub fee_rate: Option<u32>,
}

pub struct LightningClient {
    _connected: bool,
}

impl LightningClient {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("Initializing Lightning client (mock mode)");
        Ok(Self { _connected: true })
    }

    pub async fn get_node_info(&mut self) -> Result<NodeInfo, Box<dyn std::error::Error>> {
        info!("Fetching node info from LND (mock)");
        
        let node_info = NodeInfo {
            pubkey: "02d7f8c2abc123456789abcdef123456789abcdef123456789abcdef123456789ab".to_string(),
            alias: "Dazno Lightning Node".to_string(),
            num_channels: 5,
            num_active_channels: 4,
            block_height: 830000,
            synced_to_chain: true,
        };

        Ok(node_info)
    }

    pub async fn list_channels(&mut self) -> Result<Vec<ChannelInfo>> {
        info!("Listing Lightning channels (mock)");
        
        let channels = vec![
            ChannelInfo {
                channel_id: "123456789012345678".to_string(),
                peer_pubkey: "03abc123456789abcdef123456789abcdef123456789abcdef123456789abcdef12".to_string(),
                capacity: 1000000,
                local_balance: 400000,
                remote_balance: 600000,
                active: true,
            },
            ChannelInfo {
                channel_id: "987654321098765432".to_string(),
                peer_pubkey: "03def987654321abcdef987654321abcdef987654321abcdef987654321abcdef".to_string(),
                capacity: 2000000,
                local_balance: 1200000,
                remote_balance: 800000,
                active: true,
            },
        ];

        Ok(channels)
    }

    pub async fn open_channel(&mut self, params: ChannelParams) -> Result<(), Box<dyn std::error::Error>> {
        info!("Opening channel with peer: {}, amount: {} (mock)", params.peer_pubkey, params.amount);
        Ok(())
    }

    pub async fn close_channel(&mut self, channel_id: String) -> Result<(), Box<dyn std::error::Error>> {
        info!("Closing channel: {} (mock)", channel_id);
        Ok(())
    }

    pub async fn update_channel_fees(&mut self, channel_id: &str, base_fee: u32, fee_rate: u32) -> Result<()> {
        info!("Updating fees for channel {} (mock): base={}, rate={}", channel_id, base_fee, fee_rate);
        Ok(())
    }

    pub async fn get_balance(&self) -> Result<(u64, u64)> {
        info!("Getting wallet balance (mock)");
        Ok((1500000, 800000)) // (confirmed, unconfirmed)
    }
}