// Test configuration and utilities

use std::env;
use std::sync::Once;
use tracing_subscriber;

static INIT: Once = Once::new();

/// Initialize logging for tests
pub fn init_test_logging() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .init();
    });
}

/// Test configuration struct
pub struct TestConfig {
    pub mock_api_url: String,
    pub test_node_pubkey: String,
    pub test_api_key: Option<String>,
    pub enable_real_api_tests: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            mock_api_url: "http://localhost:8080".to_string(),
            test_node_pubkey:
                "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef".to_string(),
            test_api_key: env::var("DAZNO_TEST_API_KEY").ok(),
            enable_real_api_tests: env::var("ENABLE_REAL_API_TESTS").is_ok(),
        }
    }
}

impl TestConfig {
    pub fn new() -> Self {
        init_test_logging();
        Self::default()
    }

    pub fn with_real_api() -> Self {
        let mut config = Self::new();
        config.enable_real_api_tests = true;
        config
    }
}

/// Test data generators
pub mod test_data {
    use chrono::Utc;
    use dazno_umbrel::api::mcp_client::{
        ActionResult, ActionType, ChannelMetrics, MCPRecommendation, NodeMetrics, Priority,
    };
    use serde_json::json;
    use uuid::Uuid;

    pub fn create_test_node_metrics(channel_count: usize) -> NodeMetrics {
        let mut channels = Vec::new();

        for i in 0..channel_count {
            channels.push(ChannelMetrics {
                channel_id: format!("channel_{:06}", i),
                peer_pubkey: format!("03{:062x}", i),
                capacity: 1000000 + (i as u64) * 1000,
                local_balance: 500000,
                remote_balance: 500000,
                fees_earned: i as u64 * 10,
                forwards_count: i as u32,
                uptime_percentage: 99.0 - (i as f64) * 0.001,
            });
        }

        NodeMetrics {
            pubkey: "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef"
                .to_string(),
            alias: format!("Test Node with {} channels", channel_count),
            channels,
            wallet_balance: 2000000,
            channel_balance: 5000000,
            total_capacity: (channel_count as u64) * 1000000,
            routing_fees_earned: (channel_count as u64) * 10,
            timestamp: Utc::now(),
        }
    }

    pub fn create_test_recommendation(
        action_type: ActionType,
        priority: Priority,
    ) -> MCPRecommendation {
        MCPRecommendation {
            id: Uuid::new_v4().to_string(),
            action_type,
            priority,
            expected_roi_impact: 2.5,
            parameters: json!({
                "channel_id": "123456",
                "fee_rate": 500,
                "reasoning": "Test recommendation"
            }),
            created_at: Utc::now(),
            description: "Test recommendation for unit tests".to_string(),
        }
    }

    pub fn create_test_action_result(success: bool) -> ActionResult {
        ActionResult {
            action_id: Uuid::new_v4().to_string(),
            success,
            message: if success {
                "Test action completed successfully"
            } else {
                "Test action failed"
            }
            .to_string(),
            timestamp: Utc::now(),
        }
    }

    pub fn create_performance_analysis_json() -> serde_json::Value {
        json!({
            "node_pubkey": "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef",
            "analysis_period_days": 30,
            "generated_at": "2024-01-15T10:35:00Z",
            "performance_metrics": {
                "current_roi_percentage": 15.8,
                "roi_trend": "positive",
                "roi_change_30d": 2.1,
                "routing_success_rate": 94.2,
                "avg_response_time_ms": 145,
                "liquidity_efficiency": 87.5,
                "channel_utilization": 76.3
            },
            "competitive_analysis": {
                "network_percentile": 85,
                "vs_amboss_advantage": 15.3,
                "dazno_ml_accuracy": 94.7,
                "amboss_accuracy": 87.2
            },
            "insights": [
                {
                    "category": "fee_optimization",
                    "impact": "high",
                    "confidence": 92.5,
                    "description": "Test insight for fee optimization"
                }
            ],
            "predictions": {
                "roi_7d": 16.1,
                "roi_30d": 18.2,
                "roi_90d": 19.5
            }
        })
    }
}

/// Test utilities
pub mod test_utils {
    use std::time::{Duration, Instant};
    use tokio::time;

    /// Wait for a condition to be true with timeout
    pub async fn wait_for_condition<F, Fut>(mut condition: F, timeout: Duration) -> bool
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = bool>,
    {
        let start = Instant::now();

        while start.elapsed() < timeout {
            if condition().await {
                return true;
            }
            time::sleep(Duration::from_millis(100)).await;
        }

        false
    }

    /// Measure execution time of an async function
    pub async fn measure_time<F, Fut, T>(f: F) -> (T, Duration)
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let start = Instant::now();
        let result = f().await;
        let duration = start.elapsed();
        (result, duration)
    }

    /// Generate test node pubkey
    pub fn generate_test_pubkey(suffix: u8) -> String {
        format!("02{:062x}{:02x}", 0xabcdef123456789u64, suffix)
    }

    /// Validate Lightning Network pubkey format
    pub fn is_valid_pubkey(pubkey: &str) -> bool {
        pubkey.len() == 66 && pubkey.starts_with("02")
            || pubkey.starts_with("03") && pubkey.chars().skip(2).all(|c| c.is_ascii_hexdigit())
    }
}

/// Test assertions and validators
pub mod test_assertions {
    use dazno_umbrel::api::mcp_client::{ActionType, MCPRecommendation, NodeMetrics, Priority};
    use serde_json::Value;

    pub fn assert_valid_node_metrics(metrics: &NodeMetrics) {
        assert!(
            !metrics.pubkey.is_empty(),
            "Node pubkey should not be empty"
        );
        assert!(!metrics.alias.is_empty(), "Node alias should not be empty");
        assert!(
            metrics.wallet_balance > 0,
            "Wallet balance should be positive"
        );
        assert!(
            metrics.total_capacity >= metrics.channel_balance,
            "Total capacity should be >= channel balance"
        );

        for channel in &metrics.channels {
            assert!(
                !channel.channel_id.is_empty(),
                "Channel ID should not be empty"
            );
            assert!(
                !channel.peer_pubkey.is_empty(),
                "Peer pubkey should not be empty"
            );
            assert!(channel.capacity > 0, "Channel capacity should be positive");
            assert!(
                channel.local_balance + channel.remote_balance <= channel.capacity,
                "Channel balances should not exceed capacity"
            );
            assert!(
                channel.uptime_percentage >= 0.0 && channel.uptime_percentage <= 100.0,
                "Uptime percentage should be between 0 and 100"
            );
        }
    }

    pub fn assert_valid_recommendation(rec: &MCPRecommendation) {
        assert!(!rec.id.is_empty(), "Recommendation ID should not be empty");
        assert!(
            !rec.description.is_empty(),
            "Recommendation description should not be empty"
        );
        assert!(
            rec.expected_roi_impact.is_finite(),
            "ROI impact should be finite"
        );

        match rec.action_type {
            ActionType::AdjustFees => {
                assert!(
                    rec.parameters.get("channel_id").is_some(),
                    "Fee adjustment should have channel_id"
                );
            }
            ActionType::OpenChannel => {
                assert!(
                    rec.parameters.get("peer_pubkey").is_some(),
                    "Open channel should have peer_pubkey"
                );
                assert!(
                    rec.parameters.get("suggested_capacity").is_some(),
                    "Open channel should have capacity"
                );
            }
            ActionType::CloseChannel => {
                assert!(
                    rec.parameters.get("channel_id").is_some(),
                    "Close channel should have channel_id"
                );
            }
            ActionType::RebalanceChannel => {
                assert!(
                    rec.parameters.get("amount_sats").is_some(),
                    "Rebalance should have amount"
                );
            }
        }
    }

    pub fn assert_valid_performance_analysis(analysis: &Value) {
        assert!(
            analysis.get("performance_metrics").is_some(),
            "Should have performance metrics"
        );
        assert!(
            analysis.get("competitive_analysis").is_some(),
            "Should have competitive analysis"
        );
        assert!(analysis.get("insights").is_some(), "Should have insights");
        assert!(
            analysis.get("predictions").is_some(),
            "Should have predictions"
        );

        let metrics = analysis.get("performance_metrics").unwrap();
        assert!(
            metrics.get("current_roi_percentage").is_some(),
            "Should have ROI percentage"
        );
        assert!(
            metrics.get("routing_success_rate").is_some(),
            "Should have success rate"
        );

        let competitive = analysis.get("competitive_analysis").unwrap();
        assert!(
            competitive.get("vs_amboss_advantage").is_some(),
            "Should have Amboss comparison"
        );
        assert!(
            competitive.get("dazno_ml_accuracy").is_some(),
            "Should have ML accuracy"
        );

        let insights = analysis.get("insights").unwrap();
        assert!(insights.is_array(), "Insights should be an array");

        let predictions = analysis.get("predictions").unwrap();
        assert!(
            predictions.get("roi_30d").is_some(),
            "Should have 30d ROI prediction"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dazno_umbrel::api::mcp_client::{ActionType, Priority};
    use test_assertions::*;
    use test_data::*;
    use test_utils::*;

    #[test]
    fn test_config_creation() {
        let config = TestConfig::new();
        assert!(!config.test_node_pubkey.is_empty());
    }

    #[test]
    fn test_data_generators() {
        let metrics = create_test_node_metrics(5);
        assert_valid_node_metrics(&metrics);
        assert_eq!(metrics.channels.len(), 5);

        let recommendation = create_test_recommendation(ActionType::AdjustFees, Priority::High);
        assert_valid_recommendation(&recommendation);

        let action_result = create_test_action_result(true);
        assert!(action_result.success);
        assert!(!action_result.action_id.is_empty());

        let analysis = create_performance_analysis_json();
        assert_valid_performance_analysis(&analysis);
    }

    #[test]
    fn test_utilities() {
        let pubkey = generate_test_pubkey(42);
        assert!(is_valid_pubkey(&pubkey));
        assert_eq!(pubkey.len(), 66);
        assert!(pubkey.starts_with("02"));

        assert!(is_valid_pubkey(
            "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef"
        ));
        assert!(is_valid_pubkey(
            "03a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef"
        ));
        assert!(!is_valid_pubkey("invalid_pubkey"));
        assert!(!is_valid_pubkey("02abc")); // Too short
    }

    #[tokio::test]
    async fn test_async_utilities() {
        let (result, duration) = measure_time(|| async {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            42
        })
        .await;

        assert_eq!(result, 42);
        assert!(duration >= std::time::Duration::from_millis(90));
        assert!(duration < std::time::Duration::from_millis(200));

        let condition_met =
            wait_for_condition(|| async { true }, std::time::Duration::from_millis(500)).await;
        assert!(condition_met);

        let condition_timeout =
            wait_for_condition(|| async { false }, std::time::Duration::from_millis(100)).await;
        assert!(!condition_timeout);
    }
}
