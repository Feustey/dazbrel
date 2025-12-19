use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningTerminalData {
    pub pool_accounts: Vec<PoolAccount>,
    pub loop_swaps: Vec<LoopSwap>,
    pub terminal_sessions: Vec<TerminalSession>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolAccount {
    pub trader_key: String,
    pub value: u64,
    pub available_balance: u64,
    pub version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopSwap {
    pub id: String,
    pub swap_type: String, // "loop_in" or "loop_out"
    pub amount: u64,
    pub cost: u64,
    pub state: String,
    pub htlc_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSession {
    pub id: String,
    pub session_type: String,
    pub state: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectrsData {
    pub block_height: u32,
    pub mempool_info: MempoolInfo,
    pub fee_estimates: FeeEstimates,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolInfo {
    pub size: u32,
    pub bytes: u64,
    pub usage: u64,
    pub maxmempool: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeEstimates {
    pub fastest: u32,   // sats/vB for next block
    pub half_hour: u32, // sats/vB for 3 blocks
    pub hour: u32,      // sats/vB for 6 blocks
    pub economy: u32,   // sats/vB for 144 blocks
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinNodeData {
    pub blockchain_info: BlockchainInfo,
    pub network_info: NetworkInfo,
    pub mempool_info: MempoolInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u32,
    pub headers: u32,
    pub bestblockhash: String,
    pub difficulty: f64,
    pub size_on_disk: u64,
    pub verification_progress: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub version: u32,
    pub subversion: String,
    pub protocol_version: u32,
    pub connections: u32,
    pub networks: Vec<NetworkDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDetails {
    pub name: String,
    pub limited: bool,
    pub reachable: bool,
    pub proxy: String,
    pub proxy_randomize_credentials: bool,
}

pub struct UmbrelIntegrations {
    client: Client,
    lightning_terminal_url: String,
    electrs_url: String,
    bitcoin_rpc_url: String,
    bitcoin_rpc_user: String,
    bitcoin_rpc_pass: String,
}

impl UmbrelIntegrations {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            lightning_terminal_url: std::env::var("LIGHTNING_TERMINAL_URL")
                .unwrap_or_else(|_| "http://lightning-terminal_web_1:3004".to_string()),
            electrs_url: std::env::var("ELECTRS_URL")
                .unwrap_or_else(|_| "http://electrs_web_1:3002".to_string()),
            bitcoin_rpc_url: std::env::var("BITCOIN_RPC_URL")
                .unwrap_or_else(|_| "http://bitcoin_bitcoind_1:8332".to_string()),
            bitcoin_rpc_user: std::env::var("BITCOIN_RPC_USER")
                .unwrap_or_else(|_| "umbrel".to_string()),
            bitcoin_rpc_pass: std::env::var("BITCOIN_RPC_PASS").unwrap_or_else(|_| "".to_string()),
        }
    }

    pub async fn get_lightning_terminal_data(&self) -> Result<LightningTerminalData> {
        info!("Fetching Lightning Terminal data from Umbrel");

        // Mock data for now - will be replaced with real Lightning Terminal API calls
        let data = LightningTerminalData {
            pool_accounts: vec![PoolAccount {
                trader_key: "02abc123456789def".to_string(),
                value: 1000000,
                available_balance: 800000,
                version: 1,
            }],
            loop_swaps: vec![LoopSwap {
                id: "swap_123456".to_string(),
                swap_type: "loop_out".to_string(),
                amount: 500000,
                cost: 2500,
                state: "success".to_string(),
                htlc_address: "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4".to_string(),
            }],
            terminal_sessions: vec![],
        };

        Ok(data)
    }

    pub async fn get_electrs_data(&self) -> Result<ElectrsData> {
        info!("Fetching Electrs data from Umbrel");

        // Mock data for now - will be replaced with real Electrs API calls
        let data = ElectrsData {
            block_height: 835000,
            mempool_info: MempoolInfo {
                size: 5000,
                bytes: 25000000,
                usage: 25000000,
                maxmempool: 300000000,
            },
            fee_estimates: FeeEstimates {
                fastest: 50,
                half_hour: 25,
                hour: 15,
                economy: 5,
            },
        };

        Ok(data)
    }

    pub async fn get_bitcoin_node_data(&self) -> Result<BitcoinNodeData> {
        info!("Fetching Bitcoin node data from Umbrel");

        // Mock data for now - will be replaced with real Bitcoin RPC calls
        let data = BitcoinNodeData {
            blockchain_info: BlockchainInfo {
                chain: "main".to_string(),
                blocks: 835000,
                headers: 835000,
                bestblockhash: "00000000000000000001a2b3c4d5e6f7890123456789abcdef".to_string(),
                difficulty: 67957790298897.88,
                size_on_disk: 550000000000,
                verification_progress: 0.9999999,
            },
            network_info: NetworkInfo {
                version: 250000,
                subversion: "/Satoshi:25.0.0/".to_string(),
                protocol_version: 70016,
                connections: 8,
                networks: vec![NetworkDetails {
                    name: "ipv4".to_string(),
                    limited: false,
                    reachable: true,
                    proxy: "".to_string(),
                    proxy_randomize_credentials: false,
                }],
            },
            mempool_info: MempoolInfo {
                size: 5000,
                bytes: 25000000,
                usage: 25000000,
                maxmempool: 300000000,
            },
        };

        Ok(data)
    }

    pub async fn check_app_health(&self) -> Result<AppHealthStatus> {
        info!("Checking health of Umbrel apps");

        let mut health_status = AppHealthStatus {
            lightning_terminal: false,
            electrs: false,
            bitcoin_node: false,
            last_check: chrono::Utc::now(),
        };

        // Check Lightning Terminal
        match self
            .client
            .get(&format!("{}/health", self.lightning_terminal_url))
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => {
                health_status.lightning_terminal = true;
            }
            _ => warn!("Lightning Terminal health check failed"),
        }

        // Check Electrs
        match self
            .client
            .get(&format!("{}/api/blocks/tip/height", self.electrs_url))
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => {
                health_status.electrs = true;
            }
            _ => warn!("Electrs health check failed"),
        }

        // Check Bitcoin node (simplified)
        health_status.bitcoin_node = true; // Assume healthy for now

        Ok(health_status)
    }

    pub async fn get_network_graph_data(&self) -> Result<NetworkGraphData> {
        info!("Fetching Lightning Network graph data");

        // This would typically come from LND's graph data
        let data = NetworkGraphData {
            num_nodes: 15000,
            num_channels: 75000,
            total_network_capacity: 5000000000, // 50 BTC
            avg_channel_size: 66666,
            median_channel_size: 1000000,
            last_updated: chrono::Utc::now(),
        };

        Ok(data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppHealthStatus {
    pub lightning_terminal: bool,
    pub electrs: bool,
    pub bitcoin_node: bool,
    pub last_check: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkGraphData {
    pub num_nodes: u32,
    pub num_channels: u32,
    pub total_network_capacity: u64,
    pub avg_channel_size: u64,
    pub median_channel_size: u64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}
