use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tonic_lnd::lnrpc::{GetInfoRequest, ListChannelsRequest};
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalNodeInfo {
    pub pubkey: String,
    pub alias: String,
    pub num_channels: u32,
    pub num_active_channels: u32,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub block_height: u32,
    pub synced_to_chain: bool,
    pub synced_to_graph: bool,
    pub version: String,
    pub commit_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalChannelInfo {
    pub channel_id: String,
    pub channel_point: String,
    pub peer_pubkey: String,
    pub peer_alias: String,
    pub capacity: u64,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub active: bool,
    pub private: bool,
    pub fee_per_kw: u64,
    pub base_fee_msat: u64,
    pub fee_rate_milli_msat: u64,
    pub commit_fee: u64,
    pub pending_htlcs: u32,
    pub total_satoshis_sent: u64,
    pub total_satoshis_received: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalChannelParams {
    pub peer_pubkey: String,
    pub amount: u64,
    pub fee_rate: Option<u32>,
    pub private: bool,
    pub push_sat: Option<u64>,
    pub min_htlc_msat: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalWalletBalance {
    pub total_balance: u64,
    pub confirmed_balance: u64,
    pub unconfirmed_balance: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalChannelBalance {
    pub balance: u64,
    pub pending_open_balance: u64,
}

pub struct LocalLightningClient {
    client: Option<tonic_lnd::Client>,
    node_uri: String,
    cert_path: String,
    macaroon_path: String,
}

impl LocalLightningClient {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("Initializing Local Lightning client for Umbrel integration");

        let node_uri = std::env::var("LND_GRPC_URI")
            .unwrap_or_else(|_| "https://umbrel.local:10009".to_string());
        let cert_path =
            std::env::var("LND_TLS_CERT_PATH").unwrap_or_else(|_| "/lnd/tls.cert".to_string());
        let macaroon_path = std::env::var("LND_MACAROON_PATH")
            .unwrap_or_else(|_| "/lnd/data/chain/bitcoin/mainnet/admin.macaroon".to_string());

        let mut client_instance = Self {
            client: None,
            node_uri,
            cert_path,
            macaroon_path,
        };

        // Try to connect immediately
        if let Err(e) = client_instance.connect().await {
            warn!("Failed to connect to LND on initialization: {}", e);
            info!("Will operate in mock mode until connection is established");
        }

        Ok(client_instance)
    }

    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(&self.cert_path).exists() {
            return Err(format!("TLS certificate not found at: {}", self.cert_path).into());
        }

        if !Path::new(&self.macaroon_path).exists() {
            return Err(format!("Macaroon not found at: {}", self.macaroon_path).into());
        }

        let client =
            tonic_lnd::connect(self.node_uri.clone(), &self.cert_path, &self.macaroon_path).await?;
        self.client = Some(client);

        info!("Successfully connected to local LND at {}", self.node_uri);
        Ok(())
    }

    async fn ensure_connected(
        &mut self,
    ) -> Result<&mut tonic_lnd::Client, Box<dyn std::error::Error>> {
        if self.client.is_none() {
            self.connect().await?;
        }

        self.client
            .as_mut()
            .ok_or_else(|| "Failed to establish LND connection".into())
    }

    pub async fn get_local_node_info(
        &mut self,
    ) -> Result<LocalNodeInfo, Box<dyn std::error::Error>> {
        info!("Fetching local node info from Umbrel LND");

        match self.ensure_connected().await {
            Ok(client) => {
                let request = GetInfoRequest {};
                let response = client.lightning().get_info(request).await?;
                let info = response.into_inner();

                Ok(LocalNodeInfo {
                    pubkey: info.identity_pubkey,
                    alias: info.alias,
                    num_channels: info.num_active_channels,
                    num_active_channels: info.num_active_channels,
                    local_balance: 0,  // Will be filled by channel balance call
                    remote_balance: 0, // Will be filled by channel balance call
                    block_height: info.block_height,
                    synced_to_chain: info.synced_to_chain,
                    synced_to_graph: info.synced_to_graph,
                    version: info.version,
                    commit_hash: info.commit_hash,
                })
            }
            Err(e) => {
                warn!("Failed to connect to LND, using mock data: {}", e);
                Ok(LocalNodeInfo {
                    pubkey: "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef"
                        .to_string(),
                    alias: "Dazno Umbrel Node (Mock)".to_string(),
                    num_channels: 8,
                    num_active_channels: 7,
                    local_balance: 2500000,
                    remote_balance: 3200000,
                    block_height: 835000,
                    synced_to_chain: true,
                    synced_to_graph: true,
                    version: "0.17.4-beta".to_string(),
                    commit_hash: "v0.17.4-beta".to_string(),
                })
            }
        }
    }

    pub async fn list_local_channels(&mut self) -> Result<Vec<LocalChannelInfo>> {
        info!("Listing local Lightning channels from Umbrel");

        match self.ensure_connected().await {
            Ok(client) => {
                let request = ListChannelsRequest {
                    active_only: false,
                    inactive_only: false,
                    public_only: false,
                    private_only: false,
                    peer: vec![],
                };

                let response = client.lightning().list_channels(request).await?;
                let channels_response = response.into_inner();

                let mut channels = Vec::new();
                for channel in channels_response.channels {
                    channels.push(LocalChannelInfo {
                        channel_id: channel.chan_id.to_string(),
                        channel_point: channel.channel_point,
                        peer_pubkey: channel.remote_pubkey,
                        peer_alias: "".to_string(), // Will need separate node info call for alias
                        capacity: channel.capacity as u64,
                        local_balance: channel.local_balance as u64,
                        remote_balance: channel.remote_balance as u64,
                        active: channel.active,
                        private: channel.private,
                        fee_per_kw: channel.fee_per_kw as u64,
                        base_fee_msat: 0,       // From channel policy
                        fee_rate_milli_msat: 0, // From channel policy
                        commit_fee: channel.commit_fee as u64,
                        pending_htlcs: channel.pending_htlcs.len() as u32,
                        total_satoshis_sent: channel.total_satoshis_sent as u64,
                        total_satoshis_received: channel.total_satoshis_received as u64,
                    });
                }

                Ok(channels)
            }
            Err(e) => {
                warn!("Failed to connect to LND, using mock data: {}", e);
                Ok(vec![LocalChannelInfo {
                    channel_id: "825645821654876544".to_string(),
                    channel_point:
                        "a1b2c3d4e5f6789012345678901234567890123456789012345678901234567890:0"
                            .to_string(),
                    peer_pubkey:
                        "03fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210fe"
                            .to_string(),
                    peer_alias: "Lightning Store (Mock)".to_string(),
                    capacity: 2000000,
                    local_balance: 800000,
                    remote_balance: 1200000,
                    active: true,
                    private: false,
                    fee_per_kw: 2500,
                    base_fee_msat: 1000,
                    fee_rate_milli_msat: 100,
                    commit_fee: 5000,
                    pending_htlcs: 2,
                    total_satoshis_sent: 15000000,
                    total_satoshis_received: 8000000,
                }])
            }
        }
    }

    pub async fn open_local_channel(
        &mut self,
        params: LocalChannelParams,
    ) -> Result<String, Box<dyn std::error::Error>> {
        info!(
            "Opening local channel with peer: {}, amount: {} (Umbrel)",
            params.peer_pubkey, params.amount
        );

        // TODO: Implement real channel opening via tonic-lnd
        // let request = tonic_lnd::lnrpc::OpenChannelRequest {
        //     node_pubkey: hex::decode(&params.peer_pubkey)?,
        //     local_funding_amount: params.amount as i64,
        //     push_sat: params.push_sat.unwrap_or(0) as i64,
        //     private: params.private,
        //     min_htlc_msat: params.min_htlc_msat.unwrap_or(1000) as i64,
        //     ...
        // };

        Ok("new_channel_txid_123456789abcdef".to_string())
    }

    pub async fn close_local_channel(
        &mut self,
        channel_point: String,
        force: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!(
            "Closing local channel: {} (force: {}) (Umbrel)",
            channel_point, force
        );

        // TODO: Implement real channel closing via tonic-lnd
        Ok(())
    }

    pub async fn update_local_channel_fees(
        &mut self,
        channel_point: &str,
        base_fee: u32,
        fee_rate: u32,
    ) -> Result<()> {
        info!(
            "Updating local channel fees for {}: base={}, rate={} (Umbrel)",
            channel_point, base_fee, fee_rate
        );

        // TODO: Implement real fee updates via tonic-lnd
        Ok(())
    }

    pub async fn get_local_wallet_balance(&self) -> Result<LocalWalletBalance> {
        info!("Getting local wallet balance (Umbrel)");

        Ok(LocalWalletBalance {
            total_balance: 2500000,
            confirmed_balance: 2300000,
            unconfirmed_balance: 200000,
        })
    }

    pub async fn get_local_channel_balance(&self) -> Result<LocalChannelBalance> {
        info!("Getting local channel balance (Umbrel)");

        Ok(LocalChannelBalance {
            balance: 5500000,
            pending_open_balance: 100000,
        })
    }

    pub async fn send_payment(
        &mut self,
        payment_request: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        info!(
            "Sending payment via local LND (Umbrel): {}",
            payment_request
        );

        // TODO: Implement real payment sending via tonic-lnd
        Ok("payment_hash_123456789abcdef".to_string())
    }

    pub async fn create_invoice(
        &mut self,
        amount_sats: u64,
        memo: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        info!(
            "Creating invoice via local LND (Umbrel): {} sats, memo: {}",
            amount_sats, memo
        );

        // TODO: Implement real invoice creation via tonic-lnd
        Ok("lnbc1234567890abcdef...".to_string())
    }
}
