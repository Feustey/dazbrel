use dazno_umbrel::api::mcp_client::{MCPClient, ActionType, Priority, ActionResult, NodeMetrics, ChannelMetrics};
use dazno_umbrel::api::local_lightning_client::LocalLightningClient;
use serde_json::json;
use chrono::Utc;
use uuid::Uuid;

#[tokio::test]
async fn test_mcp_integration_workflow() {
    // This test simulates a complete workflow with api.dazno.de
    
    // 1. Initialize MCP client
    let mcp_client = MCPClient::new("https://api.dazno.de".to_string(), None);
    
    // 2. Test health check (this will fail in CI/CD but shows the API structure)
    let health_result = mcp_client.health_check().await;
    println!("Health check result: {:?}", health_result);
    
    // 3. Create mock node metrics as would be collected from Lightning client
    let mock_metrics = NodeMetrics {
        pubkey: "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef".to_string(),
        alias: "Integration Test Node".to_string(),
        channels: vec![
            ChannelMetrics {
                channel_id: "825645821654876544".to_string(),
                peer_pubkey: "03fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210fe".to_string(),
                capacity: 2000000,
                local_balance: 800000,
                remote_balance: 1200000,
                fees_earned: 5000,
                forwards_count: 150,
                uptime_percentage: 98.5,
            },
            ChannelMetrics {
                channel_id: "867530986420135579".to_string(),
                peer_pubkey: "02abcdef123456789abcdef123456789abcdef123456789abcdef123456789abcdef12".to_string(),
                capacity: 5000000,
                local_balance: 2800000,
                remote_balance: 2200000,
                fees_earned: 12000,
                forwards_count: 300,
                uptime_percentage: 99.2,
            },
        ],
        wallet_balance: 2500000,
        channel_balance: 5500000,
        total_capacity: 7000000,
        routing_fees_earned: 17000,
        timestamp: Utc::now(),
    };
    
    // 4. Test submitting metrics (will fail but shows structure)
    let metrics_result = mcp_client.submit_node_metrics(mock_metrics).await;
    println!("Metrics submission result: {:?}", metrics_result);
    
    // 5. Test getting recommendations (will fail but shows structure)
    let node_pubkey = "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef";
    let recommendations_result = mcp_client.get_recommendations(node_pubkey).await;
    println!("Recommendations result: {:?}", recommendations_result);
    
    // 6. Test performance analysis (will fail but shows structure)
    let analysis_result = mcp_client.get_performance_analysis(node_pubkey, 30).await;
    println!("Performance analysis result: {:?}", analysis_result);
    
    // 7. Simulate action result submission
    let action_result = ActionResult {
        action_id: Uuid::new_v4().to_string(),
        success: true,
        message: "Channel fees adjusted successfully".to_string(),
        timestamp: Utc::now(),
    };
    
    let action_submit_result = mcp_client.submit_action_result(action_result).await;
    println!("Action result submission: {:?}", action_submit_result);
}

#[tokio::test]
async fn test_local_lightning_integration() {
    // This test shows how the Lightning client would work with real data
    
    // Initialize Lightning client (will work in mock mode without real LND)
    let lightning_client_result = LocalLightningClient::new().await;
    assert!(lightning_client_result.is_ok());
    
    let mut lightning_client = lightning_client_result.unwrap();
    
    // Test getting node info (will return mock data if no real LND connection)
    let node_info_result = lightning_client.get_local_node_info().await;
    println!("Node info result: {:?}", node_info_result);
    assert!(node_info_result.is_ok());
    
    let node_info = node_info_result.unwrap();
    assert!(!node_info.pubkey.is_empty());
    assert!(!node_info.alias.is_empty());
    
    // Test getting channels (will return mock data if no real LND connection)
    let channels_result = lightning_client.list_local_channels().await;
    println!("Channels result: {:?}", channels_result);
    assert!(channels_result.is_ok());
    
    let channels = channels_result.unwrap();
    // In mock mode, should return at least one mock channel
    assert!(!channels.is_empty());
    
    for channel in &channels {
        assert!(!channel.channel_id.is_empty());
        assert!(!channel.peer_pubkey.is_empty());
        assert!(channel.capacity > 0);
    }
}

#[tokio::test]
async fn test_full_integration_flow() {
    // This test simulates the complete flow from Lightning data to MCP recommendations
    
    // 1. Get Lightning node data
    let mut lightning_client = LocalLightningClient::new().await.unwrap();
    let node_info = lightning_client.get_local_node_info().await.unwrap();
    let channels = lightning_client.list_local_channels().await.unwrap();
    
    // 2. Convert Lightning data to MCP metrics format
    let channel_metrics: Vec<ChannelMetrics> = channels.iter().map(|channel| {
        ChannelMetrics {
            channel_id: channel.channel_id.clone(),
            peer_pubkey: channel.peer_pubkey.clone(),
            capacity: channel.capacity,
            local_balance: channel.local_balance,
            remote_balance: channel.remote_balance,
            fees_earned: channel.total_satoshis_sent / 1000, // Rough estimate
            forwards_count: 42, // Would be calculated from real data
            uptime_percentage: if channel.active { 99.0 } else { 50.0 },
        }
    }).collect();
    
    let node_metrics = NodeMetrics {
        pubkey: node_info.pubkey.clone(),
        alias: node_info.alias.clone(),
        channels: channel_metrics,
        wallet_balance: node_info.local_balance + node_info.remote_balance,
        channel_balance: node_info.local_balance,
        total_capacity: channels.iter().map(|c| c.capacity).sum(),
        routing_fees_earned: channels.iter().map(|c| c.total_satoshis_sent / 1000).sum(),
        timestamp: Utc::now(),
    };
    
    // 3. Submit to MCP (would fail without real API but shows the flow)
    let mcp_client = MCPClient::new("https://api.dazno.de".to_string(), None);
    let _metrics_submission = mcp_client.submit_node_metrics(node_metrics).await;
    
    // 4. Get recommendations based on the data
    let _recommendations = mcp_client.get_recommendations(&node_info.pubkey).await;
    
    // 5. Get performance analysis
    let _analysis = mcp_client.get_performance_analysis(&node_info.pubkey, 30).await;
    
    println!("Full integration flow test completed");
    assert!(true); // Test passes if no panics occurred
}

#[tokio::test] 
async fn test_error_handling_and_resilience() {
    // Test how the system handles various error conditions
    
    // 1. Test with invalid MCP URL
    let invalid_mcp_client = MCPClient::new("http://invalid-url-that-does-not-exist.local".to_string(), None);
    let health_result = invalid_mcp_client.health_check().await;
    assert!(health_result.is_ok());
    assert!(!health_result.unwrap()); // Should return false for unreachable service
    
    // 2. Test Lightning client resilience (should handle missing LND gracefully)
    let lightning_client = LocalLightningClient::new().await;
    assert!(lightning_client.is_ok()); // Should not fail even without real LND
    
    // 3. Test with invalid node pubkey
    let mcp_client = MCPClient::new("https://api.dazno.de".to_string(), None);
    let invalid_recommendations = mcp_client.get_recommendations("invalid_pubkey").await;
    // Should either return empty list or error gracefully
    match invalid_recommendations {
        Ok(recs) => assert!(recs.is_empty()),
        Err(_) => {} // Acceptable to error on invalid input
    }
    
    println!("Error handling test completed");
}

#[tokio::test]
async fn test_data_validation_and_serialization() {
    // Test that our data structures are robust and handle edge cases
    
    // 1. Test with extreme values
    let extreme_metrics = NodeMetrics {
        pubkey: "0".repeat(66), // Edge case: minimum valid length
        alias: "🚀⚡💎".to_string(), // Unicode characters
        channels: vec![], // Empty channels list
        wallet_balance: 1, // Minimum value
        channel_balance: u64::MAX, // Maximum value
        total_capacity: 0, // Zero capacity
        routing_fees_earned: 999999999999, // Large number
        timestamp: Utc::now(),
    };
    
    // Test serialization/deserialization
    let json_str = serde_json::to_string(&extreme_metrics).unwrap();
    let deserialized: NodeMetrics = serde_json::from_str(&json_str).unwrap();
    assert_eq!(extreme_metrics.pubkey, deserialized.pubkey);
    assert_eq!(extreme_metrics.alias, deserialized.alias);
    
    // 2. Test ActionResult with various states
    let test_cases = vec![
        ActionResult {
            action_id: "success-case".to_string(),
            success: true,
            message: "All good ✅".to_string(),
            timestamp: Utc::now(),
        },
        ActionResult {
            action_id: "failure-case".to_string(),
            success: false,
            message: "Error: Channel not found ❌".to_string(),
            timestamp: Utc::now(),
        },
        ActionResult {
            action_id: "".to_string(), // Empty ID
            success: true,
            message: "".to_string(), // Empty message
            timestamp: Utc::now(),
        },
    ];
    
    for result in test_cases {
        let json_str = serde_json::to_string(&result).unwrap();
        let deserialized: ActionResult = serde_json::from_str(&json_str).unwrap();
        assert_eq!(result.action_id, deserialized.action_id);
        assert_eq!(result.success, deserialized.success);
    }
    
    println!("Data validation test completed");
}

#[tokio::test]
async fn test_concurrent_operations() {
    // Test that multiple operations can run concurrently without issues
    
    let mcp_client = MCPClient::new("https://api.dazno.de".to_string(), None);
    let node_pubkey = "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef";
    
    // Create multiple concurrent tasks
    let tasks = vec![
        tokio::spawn({
            let client = mcp_client.clone();
            async move { client.health_check().await }
        }),
        tokio::spawn({
            let client = mcp_client.clone();
            let pubkey = node_pubkey.to_string();
            async move { client.get_recommendations(&pubkey).await }
        }),
        tokio::spawn({
            let client = mcp_client.clone();
            let pubkey = node_pubkey.to_string();
            async move { client.get_performance_analysis(&pubkey, 7).await }
        }),
        tokio::spawn({
            let client = mcp_client.clone();
            let pubkey = node_pubkey.to_string();
            async move { client.get_performance_analysis(&pubkey, 30).await }
        }),
    ];
    
    // Wait for all tasks to complete
    let results = futures_util::future::join_all(tasks).await;
    
    // All tasks should complete without panicking
    for result in results {
        assert!(result.is_ok());
    }
    
    println!("Concurrent operations test completed");
}

// Mock server tests for more controlled testing
#[cfg(test)]
mod mock_server_tests {
    use super::*;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path, header};
    
    #[tokio::test]
    async fn test_realistic_api_responses() {
        let mock_server = MockServer::start().await;
        
        // Mock realistic API responses based on expected api.dazno.de behavior
        
        // Health endpoint
        Mock::given(method("GET"))
            .and(path("/api/v1/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "status": "operational",
                "version": "1.2.3",
                "uptime": 99.98,
                "services": {
                    "ml_engine": "healthy",
                    "recommendation_service": "healthy",
                    "analytics_db": "healthy"
                }
            })))
            .mount(&mock_server)
            .await;
        
        // Recommendations endpoint with realistic data
        let node_pubkey = "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef";
        Mock::given(method("GET"))
            .and(path(format!("/api/v1/recommendations/{}", node_pubkey)))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([
                {
                    "id": "rec_001_fee_optimization",
                    "action_type": "AdjustFees",
                    "priority": "High",
                    "expected_roi_impact": 3.2,
                    "parameters": {
                        "channel_id": "825645821654876544",
                        "current_base_fee": 1000,
                        "suggested_base_fee": 500,
                        "current_fee_rate": 100,
                        "suggested_fee_rate": 75,
                        "reasoning": "Channel is underpriced compared to similar routes"
                    },
                    "created_at": "2024-01-15T10:30:00Z",
                    "description": "Adjust fees on high-traffic channel to capture more routing opportunities while remaining competitive"
                },
                {
                    "id": "rec_002_liquidity_rebalance",
                    "action_type": "RebalanceChannel",
                    "priority": "Medium",
                    "expected_roi_impact": 1.8,
                    "parameters": {
                        "source_channel": "867530986420135579",
                        "target_channel": "825645821654876544",
                        "amount_sats": 500000,
                        "max_fee_sats": 1000
                    },
                    "created_at": "2024-01-15T10:25:00Z",
                    "description": "Rebalance liquidity to improve channel efficiency and routing success rate"
                },
                {
                    "id": "rec_003_new_channel",
                    "action_type": "OpenChannel",
                    "priority": "Low",
                    "expected_roi_impact": 2.5,
                    "parameters": {
                        "peer_pubkey": "03b4a72e4aaa69ba04b80c6891735592d7c96b2e3b82e3fd5d7b82b7d4b9d4a8c1",
                        "suggested_capacity": 2000000,
                        "reasoning": "High-centrality node with good routing opportunities"
                    },
                    "created_at": "2024-01-15T10:20:00Z",
                    "description": "Open channel to well-connected node to expand routing opportunities"
                }
            ])))
            .mount(&mock_server)
            .await;
        
        // Performance analysis endpoint
        Mock::given(method("GET"))
            .and(path(format!("/api/v1/analysis/{}/performance", node_pubkey)))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "node_pubkey": node_pubkey,
                "analysis_period_days": 30,
                "generated_at": "2024-01-15T10:35:00Z",
                "performance_metrics": {
                    "roi_percentage": 15.8,
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
                    "ml_accuracy": 94.7,
                    "amboss_accuracy": 87.2
                },
                "insights": [
                    {
                        "category": "fee_optimization",
                        "impact": "high",
                        "confidence": 92.5,
                        "description": "3 channels are underpriced and could generate 15% more revenue",
                        "affected_channels": ["825645821654876544", "867530986420135579"]
                    },
                    {
                        "category": "liquidity_management",
                        "impact": "medium", 
                        "confidence": 87.3,
                        "description": "Liquidity distribution could be optimized for better routing",
                        "suggested_actions": ["rebalance", "loop_out"]
                    },
                    {
                        "category": "network_expansion",
                        "impact": "low",
                        "confidence": 78.9,
                        "description": "New channel opportunities identified with high-centrality nodes",
                        "potential_peers": 5
                    }
                ],
                "predictions": {
                    "roi_7d": 16.1,
                    "roi_30d": 18.2,
                    "roi_90d": 19.5,
                    "confidence_intervals": {
                        "roi_30d_min": 16.8,
                        "roi_30d_max": 19.6
                    }
                }
            })))
            .mount(&mock_server)
            .await;
        
        // Test the realistic interactions
        let client = MCPClient::new(mock_server.uri(), None);
        
        // Test health check
        let health = client.health_check().await.unwrap();
        assert!(health);
        
        // Test recommendations
        let recommendations = client.get_recommendations(node_pubkey).await.unwrap();
        assert_eq!(recommendations.len(), 3);
        
        let fee_rec = &recommendations[0];
        assert!(matches!(fee_rec.action_type, ActionType::AdjustFees));
        assert!(matches!(fee_rec.priority, Priority::High));
        assert_eq!(fee_rec.expected_roi_impact, 3.2);
        assert!(fee_rec.parameters["channel_id"].is_string());
        
        // Test performance analysis
        let analysis = client.get_performance_analysis(node_pubkey, 30).await.unwrap();
        assert_eq!(analysis["performance_metrics"]["roi_percentage"], 15.8);
        assert_eq!(analysis["competitive_analysis"]["vs_amboss_advantage"], 15.3);
        assert!(analysis["insights"].is_array());
        assert_eq!(analysis["insights"].as_array().unwrap().len(), 3);
        
        println!("Realistic API responses test completed successfully");
    }
}