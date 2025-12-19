use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::Response;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeUpdate {
    pub r#type: String,
    pub payload: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ROIUpdate {
    pub current: f64,
    pub predicted: f64,
    pub trend: String,
    pub network_average: f64,
    pub change_24h: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationResult {
    pub recommendation_id: String,
    pub success: bool,
    pub roi_impact: f64,
    pub execution_time_ms: u64,
    pub message: String,
}

#[derive(Clone)]
pub struct WebSocketState {
    pub tx: broadcast::Sender<RealTimeUpdate>,
}

impl WebSocketState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1000);
        Self { tx }
    }

    pub fn broadcast_update(&self, update: RealTimeUpdate) {
        if let Err(e) = self.tx.send(update) {
            warn!("Failed to broadcast WebSocket update: {}", e);
        }
    }

    pub fn broadcast_roi_update(&self, roi_data: ROIUpdate) {
        let update = RealTimeUpdate {
            r#type: "roi_update".to_string(),
            payload: serde_json::to_value(roi_data).unwrap_or_default(),
            timestamp: chrono::Utc::now(),
        };
        self.broadcast_update(update);
    }

    pub fn broadcast_new_recommendation(&self, recommendation: serde_json::Value) {
        let update = RealTimeUpdate {
            r#type: "new_recommendation".to_string(),
            payload: recommendation,
            timestamp: chrono::Utc::now(),
        };
        self.broadcast_update(update);
    }

    pub fn broadcast_automation_result(&self, result: AutomationResult) {
        let update = RealTimeUpdate {
            r#type: "automation_result".to_string(),
            payload: serde_json::to_value(result).unwrap_or_default(),
            timestamp: chrono::Utc::now(),
        };
        self.broadcast_update(update);
    }

    pub fn broadcast_competitive_update(&self, competitive_data: serde_json::Value) {
        let update = RealTimeUpdate {
            r#type: "competitive_update".to_string(),
            payload: competitive_data,
            timestamp: chrono::Utc::now(),
        };
        self.broadcast_update(update);
    }
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(app_state): State<Arc<crate::AppState>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, app_state.ws_state.clone()))
}

async fn handle_socket(socket: WebSocket, state: Arc<WebSocketState>) {
    let mut rx = state.tx.subscribe();
    let (mut sender, mut receiver) = socket.split();

    info!("New WebSocket connection established");

    // Send initial connection confirmation
    let welcome_msg = RealTimeUpdate {
        r#type: "connection_established".to_string(),
        payload: serde_json::json!({
            "message": "Connected to Dazno Pro real-time updates",
            "features": ["roi_updates", "recommendations", "automation", "competitive_analysis"]
        }),
        timestamp: chrono::Utc::now(),
    };

    if let Ok(msg) = serde_json::to_string(&welcome_msg) {
        let _ = sender.send(Message::Text(msg)).await;
    }

    // Handle incoming messages from client
    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    info!("Received WebSocket message: {}", text);
                    // Handle client commands here if needed
                }
                Ok(Message::Close(_)) => {
                    info!("WebSocket connection closed by client");
                    break;
                }
                Err(e) => {
                    warn!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });

    // Handle outgoing broadcasts to client
    let mut send_task = tokio::spawn(async move {
        while let Ok(update) = rx.recv().await {
            if let Ok(msg) = serde_json::to_string(&update) {
                if sender.send(Message::Text(msg)).await.is_err() {
                    info!("WebSocket client disconnected");
                    break;
                }
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        }
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }

    info!("WebSocket connection closed");
}

// Background task to generate periodic updates
pub async fn start_real_time_updates(state: Arc<WebSocketState>) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
    let mut roi_base = 15.5;

    loop {
        interval.tick().await;

        // Simulate ROI fluctuations
        let change = (rand::random::<f64>() - 0.5) * 2.0; // -1% to +1%
        roi_base += change * 0.1; // Small incremental changes

        let roi_update = ROIUpdate {
            current: roi_base,
            predicted: roi_base + 2.5,
            trend: if change > 0.0 {
                "positive".to_string()
            } else {
                "negative".to_string()
            },
            network_average: 12.3,
            change_24h: change,
        };

        state.broadcast_roi_update(roi_update);

        // Occasionally broadcast competitive updates
        if rand::random::<f64>() > 0.8 {
            let competitive_data = serde_json::json!([
                {
                    "metric": "accuracy",
                    "dazno_value": "95%",
                    "competitor_value": "85%"
                },
                {
                    "metric": "speed",
                    "dazno_value": "150ms",
                    "competitor_value": "380ms"
                },
                {
                    "metric": "roi_improvement",
                    "dazno_value": "+18.5%",
                    "competitor_value": "+12.1%"
                }
            ]);

            state.broadcast_competitive_update(competitive_data);
        }
    }
}

// Re-exports are handled by the imports at the top
