// Mock API Server for simulating api.dazno.de during development and testing

use wiremock::{Mock, MockServer, ResponseTemplate, Request};
use wiremock::matchers::{method, path, path_regex, header, body_json};
use serde_json::json;
use std::time::Duration;
use uuid::Uuid;

pub struct MockDaznoApi {
    pub server: MockServer,
    pub base_url: String,
}

impl MockDaznoApi {
    pub async fn start() -> Self {
        let server = MockServer::start().await;
        let base_url = server.uri();
        
        let mut mock_api = Self { server, base_url };
        mock_api.setup_default_endpoints().await;
        mock_api
    }
    
    async fn setup_default_endpoints(&mut self) {
        self.setup_health_endpoint().await;
        self.setup_recommendations_endpoint().await;
        self.setup_performance_analysis_endpoint().await;
        self.setup_metrics_submission_endpoint().await;
        self.setup_action_result_endpoint().await;
    }
    
    async fn setup_health_endpoint(&self) {
        Mock::given(method("GET"))
            .and(path("/api/v1/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "status": "operational",
                "version": "1.2.3",
                "uptime": 99.98,
                "timestamp": "2024-01-15T10:30:00Z",
                "services": {
                    "ml_engine": "healthy",
                    "recommendation_service": "healthy", 
                    "analytics_db": "healthy",
                    "lightning_network_monitor": "healthy"
                },
                "performance": {
                    "avg_response_time_ms": 145,
                    "requests_per_minute": 1200,
                    "success_rate": 99.95
                }
            })))
            .mount(&self.server)
            .await;
    }
    
    async fn setup_recommendations_endpoint(&self) {
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/recommendations/[0-9a-f]{66}"))
            .respond_with(|req: &Request| {
                let node_pubkey = req.url.path().split('/').last().unwrap();
                
                // Generate realistic recommendations based on node pubkey
                let recommendations = vec![
                    json!({
                        "id": format!("rec_{}_fee_opt", Uuid::new_v4().to_string()[..8].to_string()),
                        "action_type": "AdjustFees",
                        "priority": "High",
                        "expected_roi_impact": 3.2,
                        "confidence": 92.5,
                        "parameters": {
                            "channel_id": "825645821654876544",
                            "current_base_fee": 1000,
                            "suggested_base_fee": 500,
                            "current_fee_rate": 100,
                            "suggested_fee_rate": 75,
                            "reasoning": "Channel is underpriced compared to similar routes in your geographic region",
                            "market_analysis": {
                                "similar_channels_avg_fee": 80,
                                "route_competition_level": "medium",
                                "demand_forecast": "increasing"
                            }
                        },
                        "created_at": "2024-01-15T10:30:00Z",
                        "expires_at": "2024-01-15T16:30:00Z",
                        "description": "Adjust fees on high-traffic channel to capture more routing opportunities while remaining competitive"
                    }),
                    json!({
                        "id": format!("rec_{}_rebalance", Uuid::new_v4().to_string()[..8].to_string()),
                        "action_type": "RebalanceChannel", 
                        "priority": "Medium",
                        "expected_roi_impact": 1.8,
                        "confidence": 87.3,
                        "parameters": {
                            "source_channel": "867530986420135579",
                            "target_channel": "825645821654876544", 
                            "amount_sats": 500000,
                            "max_fee_sats": 1000,
                            "routing_hints": [
                                "Use submarine swap for large rebalance",
                                "Consider loop out if liquidity is consistently unbalanced"
                            ]
                        },
                        "created_at": "2024-01-15T10:25:00Z",
                        "expires_at": "2024-01-15T14:25:00Z",
                        "description": "Rebalance liquidity to improve channel efficiency and routing success rate by 12%"
                    }),
                    json!({
                        "id": format!("rec_{}_new_channel", Uuid::new_v4().to_string()[..8].to_string()),
                        "action_type": "OpenChannel",
                        "priority": "Low", 
                        "expected_roi_impact": 2.5,
                        "confidence": 78.9,
                        "parameters": {
                            "peer_pubkey": "03b4a72e4aaa69ba04b80c6891735592d7c96b2e3b82e3fd5d7b82b7d4b9d4a8c1",
                            "peer_alias": "BigRouterNode",
                            "suggested_capacity": 2000000,
                            "reasoning": "High-centrality node with excellent routing opportunities to Asian markets",
                            "peer_analysis": {
                                "centrality_score": 0.89,
                                "uptime_percentage": 99.8,
                                "avg_fee_rate": 25,
                                "routing_success_rate": 96.2
                            }
                        },
                        "created_at": "2024-01-15T10:20:00Z",
                        "expires_at": "2024-01-16T10:20:00Z", 
                        "description": "Open channel to well-connected node to expand routing opportunities and access new markets"
                    })
                ];
                
                ResponseTemplate::new(200)
                    .set_body_json(recommendations)
                    .set_delay(Duration::from_millis(150)) // Simulate ML processing time
            })
            .mount(&self.server)
            .await;
    }
    
    async fn setup_performance_analysis_endpoint(&self) {
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/analysis/[0-9a-f]{66}/performance"))
            .respond_with(|req: &Request| {
                let node_pubkey = req.url.path().split('/').nth(3).unwrap();
                let query_params: std::collections::HashMap<_, _> = req.url.query_pairs().collect();
                let days = query_params.get("days").unwrap_or(&"30".into()).parse::<u32>().unwrap_or(30);
                
                // Generate analysis based on timeframe
                let (roi_trend, roi_change) = match days {
                    7 => ("stable", 0.5),
                    30 => ("positive", 2.1),
                    90 => ("very_positive", 5.8),
                    _ => ("positive", 2.1),
                };
                
                ResponseTemplate::new(200).set_body_json(json!({
                    "node_pubkey": node_pubkey,
                    "analysis_period_days": days,
                    "generated_at": "2024-01-15T10:35:00Z",
                    "performance_metrics": {
                        "current_roi_percentage": 15.8,
                        "roi_trend": roi_trend,
                        "roi_change_period": roi_change,
                        "routing_success_rate": 94.2,
                        "total_forwards": 1247,
                        "successful_forwards": 1175,
                        "avg_response_time_ms": 145,
                        "liquidity_efficiency": 87.5,
                        "channel_utilization": 76.3,
                        "fees_earned_sats": 175000,
                        "uptime_percentage": 99.7
                    },
                    "competitive_analysis": {
                        "network_percentile": 85,
                        "vs_amboss_advantage": 15.3,
                        "dazno_ml_accuracy": 94.7,
                        "amboss_accuracy": 87.2,
                        "prediction_confidence": 92.1,
                        "market_position": "top_quartile"
                    },
                    "channel_analysis": [
                        {
                            "channel_id": "825645821654876544",
                            "efficiency_score": 92.3,
                            "revenue_contribution": 35.2,
                            "optimization_potential": "high",
                            "recommended_actions": ["fee_adjustment", "liquidity_rebalance"]
                        },
                        {
                            "channel_id": "867530986420135579", 
                            "efficiency_score": 78.9,
                            "revenue_contribution": 28.7,
                            "optimization_potential": "medium",
                            "recommended_actions": ["rebalance_only"]
                        }
                    ],
                    "insights": [
                        {
                            "category": "fee_optimization",
                            "impact": "high",
                            "confidence": 92.5,
                            "potential_revenue_increase": "15%",
                            "description": "3 channels are underpriced compared to network average",
                            "affected_channels": ["825645821654876544", "867530986420135579"],
                            "time_to_implement": "immediate",
                            "risk_level": "low"
                        },
                        {
                            "category": "liquidity_management",
                            "impact": "medium",
                            "confidence": 87.3, 
                            "potential_revenue_increase": "8%",
                            "description": "Liquidity distribution could be optimized for 12% better routing success",
                            "suggested_actions": ["loop_out", "submarine_swap", "circular_rebalance"],
                            "time_to_implement": "1-2 hours",
                            "risk_level": "medium"
                        },
                        {
                            "category": "network_expansion",
                            "impact": "low",
                            "confidence": 78.9,
                            "potential_revenue_increase": "12%",
                            "description": "5 new channel opportunities identified with high-centrality nodes",
                            "potential_peers": [
                                {
                                    "pubkey": "03b4a72e4aaa69ba04b80c6891735592d7c96b2e3b82e3fd5d7b82b7d4b9d4a8c1",
                                    "alias": "BigRouterNode",
                                    "centrality_score": 0.89
                                }
                            ],
                            "time_to_implement": "1-3 days",
                            "risk_level": "medium"
                        }
                    ],
                    "predictions": {
                        "roi_7d": 16.1,
                        "roi_30d": 18.2,
                        "roi_90d": 19.5,
                        "confidence_intervals": {
                            "roi_30d_min": 16.8,
                            "roi_30d_max": 19.6,
                            "roi_90d_min": 17.2,
                            "roi_90d_max": 22.1
                        },
                        "risk_factors": [
                            "Network congestion during high-fee periods",
                            "Competitor fee adjustments", 
                            "Channel force closures"
                        ]
                    },
                    "market_conditions": {
                        "network_avg_fee_rate": 89,
                        "routing_demand": "high",
                        "fee_competition": "moderate",
                        "liquidity_premium": 1.15
                    }
                }))
                .set_delay(Duration::from_millis(300)) // Simulate analysis processing
            })
            .mount(&self.server)
            .await;
    }
    
    async fn setup_metrics_submission_endpoint(&self) {
        Mock::given(method("POST"))
            .and(path("/api/v1/metrics"))
            .respond_with(|req: &Request| {
                // Validate the metrics payload
                let has_auth = req.headers.contains_key("authorization");
                let content_type = req.headers.get("content-type")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("");
                
                if !content_type.contains("application/json") {
                    return ResponseTemplate::new(400).set_body_json(json!({
                        "error": "Invalid content type",
                        "expected": "application/json"
                    }));
                }
                
                // Simulate processing time based on payload size
                let delay = if req.body.len() > 100_000 {
                    Duration::from_millis(500) // Large payload
                } else {
                    Duration::from_millis(100) // Normal payload
                };
                
                let response_body = json!({
                    "status": "accepted",
                    "message": "Node metrics successfully processed",
                    "metrics_id": format!("metrics_{}", Uuid::new_v4().to_string()[..8].to_string()),
                    "processed_at": "2024-01-15T10:40:00Z",
                    "processing_time_ms": delay.as_millis(),
                    "authenticated": has_auth,
                    "next_submission_recommended": "2024-01-15T11:40:00Z"
                });
                
                ResponseTemplate::new(201)
                    .set_body_json(response_body)
                    .set_delay(delay)
            })
            .mount(&self.server)
            .await;
    }
    
    async fn setup_action_result_endpoint(&self) {
        Mock::given(method("POST"))
            .and(path("/api/v1/actions/result"))
            .respond_with(|_req: &Request| {
                ResponseTemplate::new(200).set_body_json(json!({
                    "status": "recorded",
                    "message": "Action result successfully recorded",
                    "recorded_at": "2024-01-15T10:45:00Z",
                    "will_improve_recommendations": true
                }))
                .set_delay(Duration::from_millis(50))
            })
            .mount(&self.server)
            .await;
    }
    
    // Setup error scenarios for testing
    async fn setup_error_scenarios(&self) {
        // Rate limiting simulation
        Mock::given(method("GET"))
            .and(path("/api/v1/rate-limited"))
            .respond_with(ResponseTemplate::new(429).set_body_json(json!({
                "error": "Rate limit exceeded",
                "retry_after": 60,
                "limit": 100,
                "reset_time": "2024-01-15T11:00:00Z"
            })))
            .mount(&self.server)
            .await;
        
        // Service unavailable
        Mock::given(method("GET"))
            .and(path("/api/v1/unavailable"))
            .respond_with(ResponseTemplate::new(503).set_body_json(json!({
                "error": "Service temporarily unavailable",
                "message": "ML engine is updating, please try again in 5 minutes",
                "retry_after": 300
            })))
            .mount(&self.server)
            .await;
        
        // Invalid node pubkey
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/recommendations/invalid.*"))
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                "error": "Invalid node pubkey format",
                "message": "Node pubkey must be 66 character hex string",
                "provided_length": 7
            })))
            .mount(&self.server)
            .await;
        
        // Node not found
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/recommendations/02000000.*"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({
                "error": "Node not found",
                "message": "No data available for the specified node",
                "suggested_action": "Ensure node is public and has been active"
            })))
            .mount(&self.server)
            .await;
    }
    
    // Setup premium features mock (requires API key)
    async fn setup_premium_endpoints(&self) {
        Mock::given(method("GET"))
            .and(path("/api/v1/premium/advanced-analytics"))
            .and(header("authorization", "Bearer premium-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "advanced_ml_predictions": {
                    "roi_confidence": 97.3,
                    "market_trend_analysis": "bullish_lightning_adoption",
                    "competitor_analysis": {
                        "top_competitors": 5,
                        "your_ranking": 127,
                        "performance_gap": "+15.3%"
                    }
                },
                "custom_strategies": [
                    {
                        "name": "Asian Market Expansion",
                        "potential_roi": 24.5,
                        "implementation_complexity": "medium",
                        "time_horizon": "30-90 days"
                    }
                ]
            })))
            .mount(&self.server)
            .await;
        
        // Premium without auth should fail
        Mock::given(method("GET"))
            .and(path("/api/v1/premium/advanced-analytics"))
            .respond_with(ResponseTemplate::new(401).set_body_json(json!({
                "error": "Authentication required",
                "message": "Premium features require valid API key",
                "upgrade_url": "https://dazno.de/premium"
            })))
            .mount(&self.server)
            .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dazno_umbrel::api::mcp_client::{MCPClient, NodeMetrics, ChannelMetrics};
    use chrono::Utc;
    
    #[tokio::test]
    async fn test_mock_api_health() {
        let mock_api = MockDaznoApi::start().await;
        let client = MCPClient::new(mock_api.base_url, None);
        
        let health = client.health_check().await.unwrap();
        assert!(health);
    }
    
    #[tokio::test]
    async fn test_mock_api_recommendations() {
        let mock_api = MockDaznoApi::start().await;
        let client = MCPClient::new(mock_api.base_url, None);
        
        let node_pubkey = "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef";
        let recommendations = client.get_recommendations(node_pubkey).await.unwrap();
        
        assert_eq!(recommendations.len(), 3);
        assert!(recommendations.iter().any(|r| matches!(r.action_type, dazno_umbrel::api::mcp_client::ActionType::AdjustFees)));
    }
    
    #[tokio::test]
    async fn test_mock_api_performance_analysis() {
        let mock_api = MockDaznoApi::start().await;
        let client = MCPClient::new(mock_api.base_url, None);
        
        let node_pubkey = "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef";
        let analysis = client.get_performance_analysis(node_pubkey, 30).await.unwrap();
        
        assert_eq!(analysis["performance_metrics"]["current_roi_percentage"], 15.8);
        assert_eq!(analysis["competitive_analysis"]["vs_amboss_advantage"], 15.3);
        assert!(analysis["insights"].is_array());
    }
    
    #[tokio::test]
    async fn test_mock_api_metrics_submission() {
        let mock_api = MockDaznoApi::start().await;
        let client = MCPClient::new(mock_api.base_url, None);
        
        let metrics = NodeMetrics {
            pubkey: "02test".to_string(),
            alias: "Test Node".to_string(),
            channels: vec![
                ChannelMetrics {
                    channel_id: "123".to_string(),
                    peer_pubkey: "03test".to_string(),
                    capacity: 1000000,
                    local_balance: 500000,
                    remote_balance: 500000,
                    fees_earned: 1000,
                    forwards_count: 100,
                    uptime_percentage: 99.0,
                }
            ],
            wallet_balance: 2000000,
            channel_balance: 500000,
            total_capacity: 1000000,
            routing_fees_earned: 1000,
            timestamp: Utc::now(),
        };
        
        let result = client.submit_node_metrics(metrics).await;
        assert!(result.is_ok());
    }
}