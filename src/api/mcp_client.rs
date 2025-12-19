use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPRecommendation {
    pub id: String,
    pub action_type: ActionType,
    pub priority: Priority,
    pub expected_roi_impact: f64,
    pub parameters: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    OpenChannel,
    CloseChannel,
    AdjustFees,
    RebalanceChannel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub action_id: String,
    pub success: bool,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetrics {
    pub pubkey: String,
    pub alias: String,
    pub channels: Vec<ChannelMetrics>,
    pub wallet_balance: u64,
    pub channel_balance: u64,
    pub total_capacity: u64,
    pub routing_fees_earned: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMetrics {
    pub channel_id: String,
    pub peer_pubkey: String,
    pub capacity: u64,
    pub local_balance: u64,
    pub remote_balance: u64,
    pub fees_earned: u64,
    pub forwards_count: u32,
    pub uptime_percentage: f64,
}

#[derive(Clone)]
pub struct MCPClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl MCPClient {
    pub fn new(base_url: String, api_key: Option<String>) -> Self {
        let client = Client::builder()
            .no_proxy()
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            client,
            base_url,
            api_key,
        }
    }

    pub async fn get_recommendations(&self, node_pubkey: &str) -> Result<Vec<MCPRecommendation>> {
        let url = format!("{}/api/v1/recommendations/{}", self.base_url, node_pubkey);

        let mut request = self.client.get(&url);

        if let Some(key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        info!("Fetching recommendations from MCP: {}", url);

        let response = request.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!("MCP API returned status: {} - body: {}", status, body);
            return Ok(vec![]);
        }

        let recommendations = response.json::<Vec<MCPRecommendation>>().await?;
        info!(
            "Retrieved {} recommendations from MCP",
            recommendations.len()
        );

        Ok(recommendations)
    }

    pub async fn submit_action_result(&self, result: ActionResult) -> Result<()> {
        let url = format!("{}/api/v1/actions/result", self.base_url);

        let mut request = self.client.post(&url).json(&result);

        if let Some(key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        info!("Submitting action result to MCP: {}", result.action_id);

        let response = request.send().await?;

        if !response.status().is_success() {
            error!("Failed to submit action result: {}", response.status());
            return Err(anyhow::anyhow!("Failed to submit action result"));
        }

        info!("Successfully submitted action result");
        Ok(())
    }

    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/api/v1/health", self.base_url);

        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    pub async fn submit_node_metrics(&self, metrics: NodeMetrics) -> Result<()> {
        let url = format!("{}/api/v1/metrics", self.base_url);

        let mut request = self.client.post(&url).json(&metrics);

        if let Some(key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        info!(
            "Submitting node metrics to MCP for pubkey: {}",
            metrics.pubkey
        );

        let response = request.send().await?;

        if !response.status().is_success() {
            error!("Failed to submit node metrics: {}", response.status());
            return Err(anyhow::anyhow!("Failed to submit node metrics"));
        }

        info!("Successfully submitted node metrics");
        Ok(())
    }

    pub async fn get_performance_analysis(
        &self,
        node_pubkey: &str,
        timeframe_days: u32,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "{}/api/v1/analysis/{}/performance?days={}",
            self.base_url, node_pubkey, timeframe_days
        );

        let mut request = self.client.get(&url);

        if let Some(key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        info!(
            "Fetching performance analysis from MCP for {} days",
            timeframe_days
        );

        let response = request.send().await?;

        if !response.status().is_success() {
            warn!(
                "MCP API returned status for performance analysis: {}",
                response.status()
            );
            return Ok(serde_json::json!({}));
        }

        let analysis = response.json::<serde_json::Value>().await?;
        info!("Retrieved performance analysis from MCP");

        Ok(analysis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use uuid::Uuid;
    use wiremock::matchers::{body_json, header, method, path, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // Test helper to create mock recommendations
    fn create_mock_recommendation(
        action_type: ActionType,
        priority: Priority,
    ) -> MCPRecommendation {
        MCPRecommendation {
            id: Uuid::new_v4().to_string(),
            action_type,
            priority,
            expected_roi_impact: 2.5,
            parameters: json!({"channel_id": "123456", "fee_rate": 500}),
            created_at: chrono::Utc::now(),
            description: "Test recommendation".to_string(),
        }
    }

    // Test helper to create mock node metrics
    fn create_mock_node_metrics() -> NodeMetrics {
        NodeMetrics {
            pubkey: "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef"
                .to_string(),
            alias: "Test Node".to_string(),
            channels: vec![ChannelMetrics {
                channel_id: "12345".to_string(),
                peer_pubkey: "03fedcba".to_string(),
                capacity: 1000000,
                local_balance: 500000,
                remote_balance: 500000,
                fees_earned: 1000,
                forwards_count: 100,
                uptime_percentage: 99.5,
            }],
            wallet_balance: 2000000,
            channel_balance: 5000000,
            total_capacity: 10000000,
            routing_fees_earned: 50000,
            timestamp: chrono::Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_get_recommendations_success() {
        // Arrange
        let mock_server = MockServer::start().await;
        let node_pubkey = "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef";

        let mock_recommendations = vec![
            create_mock_recommendation(ActionType::AdjustFees, Priority::High),
            create_mock_recommendation(ActionType::OpenChannel, Priority::Medium),
        ];

        Mock::given(method("GET"))
            .and(path(format!("/api/v1/recommendations/{}", node_pubkey)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_recommendations))
            .mount(&mock_server)
            .await;

        let client = MCPClient::new(mock_server.uri(), None);

        // Act
        let result = client.get_recommendations(node_pubkey).await;

        // Assert
        assert!(result.is_ok());
        let recommendations = result.unwrap();
        assert_eq!(recommendations.len(), 2);
        assert!(matches!(
            recommendations[0].action_type,
            ActionType::AdjustFees
        ));
        assert!(matches!(recommendations[0].priority, Priority::High));
        assert_eq!(recommendations[0].expected_roi_impact, 2.5);
    }

    #[tokio::test]
    async fn test_get_recommendations_with_api_key() {
        // Arrange
        let mock_server = MockServer::start().await;
        let node_pubkey = "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef";
        let api_key = "test-api-key-123";

        let mock_recommendations = vec![create_mock_recommendation(
            ActionType::RebalanceChannel,
            Priority::Low,
        )];

        Mock::given(method("GET"))
            .and(path(format!("/api/v1/recommendations/{}", node_pubkey)))
            .and(header("Authorization", format!("Bearer {}", api_key)))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_recommendations))
            .mount(&mock_server)
            .await;

        let client = MCPClient::new(mock_server.uri(), Some(api_key.to_string()));

        // Act
        let result = client.get_recommendations(node_pubkey).await;

        // Assert
        assert!(result.is_ok());
        let recommendations = result.unwrap();
        assert_eq!(recommendations.len(), 1);
        assert!(matches!(
            recommendations[0].action_type,
            ActionType::RebalanceChannel
        ));
    }

    #[tokio::test]
    async fn test_get_recommendations_api_error() {
        // Arrange
        let mock_server = MockServer::start().await;
        let node_pubkey = "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef";

        Mock::given(method("GET"))
            .and(path(format!("/api/v1/recommendations/{}", node_pubkey)))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let client = MCPClient::new(mock_server.uri(), None);

        // Act
        let result = client.get_recommendations(node_pubkey).await;

        // Assert
        assert!(result.is_ok());
        let recommendations = result.unwrap();
        assert_eq!(recommendations.len(), 0); // Should return empty vec on API error
    }

    #[tokio::test]
    async fn test_submit_action_result_success() {
        // Arrange
        let mock_server = MockServer::start().await;
        let action_result = ActionResult {
            action_id: "action-123".to_string(),
            success: true,
            message: "Action completed successfully".to_string(),
            timestamp: chrono::Utc::now(),
        };

        Mock::given(method("POST"))
            .and(path("/api/v1/actions/result"))
            .and(body_json(&action_result))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let client = MCPClient::new(mock_server.uri(), None);

        // Act
        let result = client.submit_action_result(action_result).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_submit_action_result_failure() {
        // Arrange
        let mock_server = MockServer::start().await;
        let action_result = ActionResult {
            action_id: "action-456".to_string(),
            success: false,
            message: "Action failed".to_string(),
            timestamp: chrono::Utc::now(),
        };

        Mock::given(method("POST"))
            .and(path("/api/v1/actions/result"))
            .respond_with(ResponseTemplate::new(400))
            .mount(&mock_server)
            .await;

        let client = MCPClient::new(mock_server.uri(), None);

        // Act
        let result = client.submit_action_result(action_result).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_health_check_success() {
        // Arrange
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "ok"})))
            .mount(&mock_server)
            .await;

        let client = MCPClient::new(mock_server.uri(), None);

        // Act
        let result = client.health_check().await;

        // Assert
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_health_check_failure() {
        // Arrange
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/health"))
            .respond_with(ResponseTemplate::new(503))
            .mount(&mock_server)
            .await;

        let client = MCPClient::new(mock_server.uri(), None);

        // Act
        let result = client.health_check().await;

        // Assert
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should return false for non-200 status
    }

    #[tokio::test]
    async fn test_submit_node_metrics_success() {
        // Arrange
        let mock_server = MockServer::start().await;
        let node_metrics = create_mock_node_metrics();

        Mock::given(method("POST"))
            .and(path("/api/v1/metrics"))
            .and(body_json(&node_metrics))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        let client = MCPClient::new(mock_server.uri(), None);

        // Act
        let result = client.submit_node_metrics(node_metrics).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_submit_node_metrics_with_auth() {
        // Arrange
        let mock_server = MockServer::start().await;
        let node_metrics = create_mock_node_metrics();
        let api_key = "premium-key-789";

        Mock::given(method("POST"))
            .and(path("/api/v1/metrics"))
            .and(header("Authorization", format!("Bearer {}", api_key)))
            .and(body_json(&node_metrics))
            .respond_with(ResponseTemplate::new(201))
            .mount(&mock_server)
            .await;

        let client = MCPClient::new(mock_server.uri(), Some(api_key.to_string()));

        // Act
        let result = client.submit_node_metrics(node_metrics).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_performance_analysis_success() {
        // Arrange
        let mock_server = MockServer::start().await;
        let node_pubkey = "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef";
        let timeframe_days = 30;

        let mock_analysis = json!({
            "roi_trend": "positive",
            "efficiency_score": 87.5,
            "recommendations_count": 12,
            "avg_response_time": 145,
            "performance_vs_network": 15.3,
            "prediction_accuracy": 94.7,
            "insights": [
                {
                    "type": "fee_optimization",
                    "impact": "high",
                    "description": "Optimize fees on 3 high-traffic channels"
                },
                {
                    "type": "liquidity_management",
                    "impact": "medium",
                    "description": "Rebalance channels for better flow"
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path(format!(
                "/api/v1/analysis/{}/performance",
                node_pubkey
            )))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_analysis))
            .mount(&mock_server)
            .await;

        let client = MCPClient::new(mock_server.uri(), None);

        // Act
        let result = client
            .get_performance_analysis(node_pubkey, timeframe_days)
            .await;

        // Assert
        assert!(result.is_ok());
        let analysis = result.unwrap();
        assert_eq!(analysis["roi_trend"], "positive");
        assert_eq!(analysis["efficiency_score"], 87.5);
        assert_eq!(analysis["performance_vs_network"], 15.3);
        assert!(analysis["insights"].is_array());
        assert_eq!(analysis["insights"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_get_performance_analysis_not_found() {
        // Arrange
        let mock_server = MockServer::start().await;
        let node_pubkey = "02invalid";
        let timeframe_days = 7;

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/analysis/.*/performance.*"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let client = MCPClient::new(mock_server.uri(), None);

        // Act
        let result = client
            .get_performance_analysis(node_pubkey, timeframe_days)
            .await;

        // Assert
        assert!(result.is_ok());
        let analysis = result.unwrap();
        assert_eq!(analysis, json!({})); // Should return empty object on error
    }

    #[tokio::test]
    async fn test_multiple_concurrent_requests() {
        // Arrange
        let mock_server = MockServer::start().await;
        let node_pubkey = "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef";

        // Setup mocks for different endpoints
        Mock::given(method("GET"))
            .and(path("/api/v1/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "ok"})))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path(format!("/api/v1/recommendations/{}", node_pubkey)))
            .respond_with(ResponseTemplate::new(200).set_body_json(vec![
                create_mock_recommendation(ActionType::AdjustFees, Priority::High),
            ]))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/analysis/.*/performance.*"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "analyzed"})))
            .mount(&mock_server)
            .await;

        let client = MCPClient::new(mock_server.uri(), None);

        // Act - Make concurrent requests
        let (health_result, recommendations_result, analysis_result) = tokio::join!(
            client.health_check(),
            client.get_recommendations(node_pubkey),
            client.get_performance_analysis(node_pubkey, 30)
        );

        // Assert
        assert!(health_result.is_ok());
        assert!(health_result.unwrap());

        assert!(recommendations_result.is_ok());
        assert_eq!(recommendations_result.unwrap().len(), 1);

        assert!(analysis_result.is_ok());
        assert_eq!(analysis_result.unwrap()["status"], "analyzed");
    }

    #[tokio::test]
    async fn test_error_handling_network_failure() {
        // Arrange - Use invalid URL to simulate network failure
        let client = MCPClient::new(
            "http://invalid-domain-that-does-not-exist.com".to_string(),
            None,
        );
        let node_pubkey = "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef";

        // Act
        let health_result = client.health_check().await;
        let recommendations_result = client.get_recommendations(node_pubkey).await;

        // Assert
        assert!(health_result.is_ok());
        assert!(!health_result.unwrap()); // Should return false on network error

        assert!(recommendations_result.is_err()); // Should return error on network failure
    }

    #[tokio::test]
    async fn test_json_serialization_deserialization() {
        // Test that our data structures properly serialize/deserialize
        let recommendation = create_mock_recommendation(ActionType::CloseChannel, Priority::Medium);
        let json_str = serde_json::to_string(&recommendation).unwrap();
        let deserialized: MCPRecommendation = serde_json::from_str(&json_str).unwrap();

        assert_eq!(recommendation.id, deserialized.id);
        assert!(matches!(deserialized.action_type, ActionType::CloseChannel));
        assert!(matches!(deserialized.priority, Priority::Medium));

        let metrics = create_mock_node_metrics();
        let json_str = serde_json::to_string(&metrics).unwrap();
        let deserialized: NodeMetrics = serde_json::from_str(&json_str).unwrap();

        assert_eq!(metrics.pubkey, deserialized.pubkey);
        assert_eq!(metrics.channels.len(), deserialized.channels.len());
    }
}
