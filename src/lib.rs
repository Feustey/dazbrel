// Dazno Umbrel Library
// Superior Lightning ROI Optimizer for Umbrel

pub mod api;
pub mod middleware;
pub mod models;
pub mod handlers;
pub mod utils;

// Re-export commonly used types for easier access
pub use api::mcp_client::{
    MCPClient, MCPRecommendation, ActionType, Priority, ActionResult, 
    NodeMetrics, ChannelMetrics
};
pub use api::local_lightning_client::{
    LocalLightningClient, LocalNodeInfo, LocalChannelInfo, 
    LocalChannelParams, LocalWalletBalance, LocalChannelBalance
};
pub use models::recommendation::Recommendation;
pub use models::analytics::NodeAnalytics;
pub use utils::config::AppConfig;

// AppState structure for handlers
use std::sync::Arc;
use handlebars::Handlebars;

pub struct AppState {
    pub mcp_client: MCPClient,
    pub lightning_client: Arc<tokio::sync::Mutex<LocalLightningClient>>,
    pub handlebars: Handlebars<'static>,
    pub ws_state: Arc<handlers::websocket::WebSocketState>,
}