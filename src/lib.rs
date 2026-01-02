#![allow(dead_code)]

// Dazno Umbrel Library
// Superior Lightning ROI Optimizer for Umbrel

pub mod api;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod utils;

// Re-export commonly used types for easier access
pub use api::local_lightning_client::{
    LocalChannelBalance, LocalChannelInfo, LocalChannelParams, LocalLightningClient, LocalNodeInfo,
    LocalWalletBalance,
};
pub use api::mcp_client::{
    ActionResult, ActionType, ChannelMetrics, MCPClient, MCPRecommendation, NodeMetrics, Priority,
};
pub use models::analytics::NodeAnalytics;
pub use models::recommendation::Recommendation;
pub use utils::config::AppConfig;
pub use utils::ml_engine::MLEngine;

// AppState structure for handlers
use handlebars::Handlebars;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub mcp_client: MCPClient,
    pub lightning_client: Arc<tokio::sync::Mutex<LocalLightningClient>>,
    pub handlebars: Arc<Handlebars<'static>>,
    pub ws_state: Arc<handlers::websocket::WebSocketState>,
    pub ml_engine: MLEngine,
    pub config: AppConfig,
}
