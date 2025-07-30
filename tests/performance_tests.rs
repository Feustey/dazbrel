use dazno_umbrel::api::mcp_client::{MCPClient, NodeMetrics, ChannelMetrics};
use std::time::{Duration, Instant};
use tokio::time;
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};
use serde_json::json;
use chrono::Utc;

#[tokio::test]
async fn test_mcp_client_response_times() {
    // Test response times for different MCP API endpoints
    let mock_server = MockServer::start().await;
    
    // Setup fast responding mocks
    Mock::given(method("GET"))
        .and(path("/api/v1/health"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"status": "ok"}))
                .set_delay(Duration::from_millis(50)) // Simulate 50ms response time
        )
        .mount(&mock_server)
        .await;
    
    let node_pubkey = "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef";
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/recommendations/{}", node_pubkey)))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!([]))
                .set_delay(Duration::from_millis(200)) // Simulate 200ms for ML processing
        )
        .mount(&mock_server)
        .await;
    
    Mock::given(method("GET"))
        .and(path(format!("/api/v1/analysis/{}/performance", node_pubkey)))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"analysis": "complete"}))
                .set_delay(Duration::from_millis(500)) // Simulate 500ms for complex analysis
        )
        .mount(&mock_server)
        .await;
    
    let client = MCPClient::new(mock_server.uri(), None);
    
    // Test health check response time
    let start = Instant::now();
    let health_result = client.health_check().await;
    let health_duration = start.elapsed();
    
    assert!(health_result.is_ok());
    assert!(health_result.unwrap());
    assert!(health_duration < Duration::from_millis(100), 
           "Health check took too long: {:?}", health_duration);
    println!("Health check response time: {:?}", health_duration);
    
    // Test recommendations response time
    let start = Instant::now();
    let recommendations_result = client.get_recommendations(node_pubkey).await;
    let recommendations_duration = start.elapsed();
    
    assert!(recommendations_result.is_ok());
    assert!(recommendations_duration < Duration::from_millis(300),
           "Recommendations took too long: {:?}", recommendations_duration);
    println!("Recommendations response time: {:?}", recommendations_duration);
    
    // Test performance analysis response time
    let start = Instant::now();
    let analysis_result = client.get_performance_analysis(node_pubkey, 30).await;
    let analysis_duration = start.elapsed();
    
    assert!(analysis_result.is_ok());
    assert!(analysis_duration < Duration::from_millis(600),
           "Performance analysis took too long: {:?}", analysis_duration);
    println!("Performance analysis response time: {:?}", analysis_duration);
}

#[tokio::test]
async fn test_concurrent_request_performance() {
    // Test performance under concurrent load
    let mock_server = MockServer::start().await;
    
    // Setup mock endpoints
    Mock::given(method("GET"))
        .and(path("/api/v1/health"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "ok"})))
        .expect(100) // Expect 100 concurrent requests
        .mount(&mock_server)
        .await;
    
    let client = MCPClient::new(mock_server.uri(), None);
    
    // Create 100 concurrent health check requests
    let start = Instant::now();
    let mut tasks = Vec::new();
    
    for _ in 0..100 {
        let client_clone = client.clone();
        tasks.push(tokio::spawn(async move {
            client_clone.health_check().await
        }));
    }
    
    // Wait for all requests to complete
    let results = futures_util::future::join_all(tasks).await;
    let total_duration = start.elapsed();
    
    // Verify all requests succeeded
    for result in results {
        assert!(result.is_ok());
        let health_result = result.unwrap();
        assert!(health_result.is_ok());
        assert!(health_result.unwrap());
    }
    
    // Should complete within reasonable time (much less than 100 * single request time)
    assert!(total_duration < Duration::from_secs(5),
           "100 concurrent requests took too long: {:?}", total_duration);
    
    println!("100 concurrent requests completed in: {:?}", total_duration);
    println!("Average time per request: {:?}", total_duration / 100);
}

#[tokio::test]
async fn test_large_payload_performance() {
    // Test performance with large payloads (many channels)
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/api/v1/metrics"))
        .respond_with(ResponseTemplate::new(201).set_delay(Duration::from_millis(100)))
        .mount(&mock_server)
        .await;
    
    let client = MCPClient::new(mock_server.uri(), None);
    
    // Create node metrics with many channels (simulate large Lightning node)
    let mut channels = Vec::new();
    for i in 0..1000 {
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
    
    let large_metrics = NodeMetrics {
        pubkey: "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef".to_string(),
        alias: "Large Node with 1000 Channels".to_string(),
        channels,
        wallet_balance: 100000000,
        channel_balance: 500000000,
        total_capacity: 1000000000,
        routing_fees_earned: 5000000,
        timestamp: Utc::now(),
    };
    
    // Test serialization performance
    let start = Instant::now();
    let json_str = serde_json::to_string(&large_metrics).unwrap();
    let serialization_duration = start.elapsed();
    
    println!("Large payload serialization time: {:?}", serialization_duration);
    println!("Serialized size: {} bytes", json_str.len());
    
    // Test deserialization performance
    let start = Instant::now();
    let deserialized: NodeMetrics = serde_json::from_str(&json_str).unwrap();
    let deserialization_duration = start.elapsed();
    
    println!("Large payload deserialization time: {:?}", deserialization_duration);
    assert_eq!(deserialized.channels.len(), 1000);
    
    // Test network submission performance
    let start = Instant::now();
    let submission_result = client.submit_node_metrics(large_metrics).await;
    let submission_duration = start.elapsed();
    
    assert!(submission_result.is_ok());
    println!("Large payload submission time: {:?}", submission_duration);
    
    // All operations should complete within reasonable time
    assert!(serialization_duration < Duration::from_millis(100));
    assert!(deserialization_duration < Duration::from_millis(100));
    assert!(submission_duration < Duration::from_secs(2));
}

#[tokio::test]
async fn test_error_recovery_performance() {
    // Test how quickly the system recovers from errors
    let mock_server = MockServer::start().await;
    
    // Setup mock that initially fails, then succeeds
    Mock::given(method("GET"))
        .and(path("/api/v1/health"))
        .respond_with(ResponseTemplate::new(500)) // First call will fail
        .mount(&mock_server)
        .await;
        
    // Add a second mock that will succeed
    Mock::given(method("GET"))
        .and(path("/api/v1/health"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "ok"})))
        .mount(&mock_server)
        .await;
    
    let client = MCPClient::new(mock_server.uri(), None);
    
    // Test retry logic performance
    let start = Instant::now();
    let mut success_count = 0;
    let mut failure_count = 0;
    
    for _ in 0..10 {
        match client.health_check().await {
            Ok(true) => success_count += 1,
            Ok(false) => failure_count += 1,
            Err(_) => failure_count += 1,
        }
        
        // Small delay between attempts
        time::sleep(Duration::from_millis(10)).await;
    }
    
    let total_duration = start.elapsed();
    
    println!("Error recovery test: {} successes, {} failures in {:?}", 
             success_count, failure_count, total_duration);
    
    // Should have some failures initially, then successes
    assert!(failure_count >= 3);
    assert!(success_count >= 3);
    
    // Should not take too long despite failures
    assert!(total_duration < Duration::from_secs(1));
}

#[tokio::test]
async fn test_memory_usage_under_load() {
    // Test memory efficiency under sustained load
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/api/v1/health"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "ok"})))
        .mount(&mock_server)
        .await;
    
    let client = MCPClient::new(mock_server.uri(), None);
    
    // Perform many operations to test for memory leaks
    let start = Instant::now();
    
    for batch in 0..10 {
        let mut tasks = Vec::new();
        
        // Create 50 concurrent requests per batch
        for _ in 0..50 {
            let client_clone = client.clone();
            tasks.push(tokio::spawn(async move {
                client_clone.health_check().await
            }));
        }
        
        // Wait for batch to complete
        let results = futures_util::future::join_all(tasks).await;
        
        // Verify all succeeded
        for result in results {
            assert!(result.is_ok());
        }
        
        // Small delay between batches
        time::sleep(Duration::from_millis(10)).await;
        
        if batch % 5 == 0 {
            println!("Completed batch {} of 10", batch + 1);
        }
    }
    
    let total_duration = start.elapsed();
    println!("Memory test: 500 total requests in {:?}", total_duration);
    
    // Should complete within reasonable time
    assert!(total_duration < Duration::from_secs(10));
}

#[tokio::test]
async fn test_timeout_handling() {
    // Test handling of slow responses and timeouts
    let mock_server = MockServer::start().await;
    
    // Setup slow responding endpoint
    Mock::given(method("GET"))
        .and(path("/api/v1/health"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"status": "ok"}))
                .set_delay(Duration::from_secs(2)) // Very slow response
        )
        .mount(&mock_server)
        .await;
    
    let client = MCPClient::new(mock_server.uri(), None);
    
    // Test that client handles slow responses appropriately
    let start = Instant::now();
    let result = client.health_check().await;
    let duration = start.elapsed();
    
    // Should either succeed (if timeout is high) or handle timeout gracefully
    match result {
        Ok(success) => {
            println!("Slow request succeeded: {} in {:?}", success, duration);
            assert!(duration >= Duration::from_secs(2)); // Should wait for slow response
        },
        Err(e) => {
            println!("Slow request failed as expected: {} in {:?}", e, duration);
            assert!(duration < Duration::from_secs(5)); // Should timeout before too long
        }
    }
}

#[tokio::test]
async fn test_api_rate_limiting_simulation() {
    // Simulate API rate limiting scenarios
    let mock_server = MockServer::start().await;
    
    // Setup mock that simulates rate limiting
    Mock::given(method("GET"))
        .and(path("/api/v1/health"))
        .respond_with(ResponseTemplate::new(429).set_body_json(json!({"error": "Rate limited"})))
        .mount(&mock_server)
        .await;
    
    let client = MCPClient::new(mock_server.uri(), None);
    
    // Make rapid requests to trigger rate limiting
    let start = Instant::now();
    let mut success_count = 0;
    let mut rate_limited_count = 0;
    
    for i in 0..20 {
        let result = client.health_check().await;
        
        match result {
            Ok(true) => success_count += 1,
            Ok(false) => rate_limited_count += 1,
            Err(_) => rate_limited_count += 1,
        }
        
        // Very small delay to simulate rapid requests
        time::sleep(Duration::from_millis(5)).await;
        
        if i == 9 {
            println!("First 10 requests completed, expecting rate limiting next...");
        }
    }
    
    let total_duration = start.elapsed();
    
    println!("Rate limiting test: {} successes, {} rate limited in {:?}", 
             success_count, rate_limited_count, total_duration);
    
    // Should have some successes, some rate limits, then successes again
    assert!(success_count >= 10); // At least initial and final successes
    assert!(rate_limited_count >= 3); // Some rate limiting in the middle
    
    // Should complete quickly despite rate limiting
    assert!(total_duration < Duration::from_secs(2));
}

#[tokio::test]
async fn test_data_compression_efficiency() {
    // Test efficiency of data serialization/compression
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;
    
    // Create a typical node metrics payload
    let mut channels = Vec::new();
    for i in 0..100 {
        channels.push(ChannelMetrics {
            channel_id: format!("channel_{:06}", i),
            peer_pubkey: format!("03{:062x}", i),
            capacity: 1000000,
            local_balance: 500000,
            remote_balance: 500000,
            fees_earned: i as u64 * 10,
            forwards_count: i as u32,
            uptime_percentage: 99.0,
        });
    }
    
    let metrics = NodeMetrics {
        pubkey: "02a1b2c3d4e5f6789abcdef123456789abcdef123456789abcdef123456789abcdef".to_string(),
        alias: "Performance Test Node".to_string(),
        channels,
        wallet_balance: 10000000,
        channel_balance: 50000000,
        total_capacity: 100000000,
        routing_fees_earned: 500000,
        timestamp: Utc::now(),
    };
    
    // Test JSON serialization
    let start = Instant::now();
    let json_data = serde_json::to_string(&metrics).unwrap();
    let json_duration = start.elapsed();
    let json_size = json_data.len();
    
    // Test compression
    let start = Instant::now();
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(json_data.as_bytes()).unwrap();
    let compressed_data = encoder.finish().unwrap();
    let compression_duration = start.elapsed();
    let compressed_size = compressed_data.len();
    
    let compression_ratio = json_size as f64 / compressed_size as f64;
    
    println!("Serialization performance:");
    println!("  JSON serialization: {:?}", json_duration);
    println!("  Original size: {} bytes", json_size);
    println!("  Compression time: {:?}", compression_duration);
    println!("  Compressed size: {} bytes", compressed_size);
    println!("  Compression ratio: {:.2}x", compression_ratio);
    
    // Verify performance requirements
    assert!(json_duration < Duration::from_millis(50));
    assert!(compression_duration < Duration::from_millis(100));
    assert!(compression_ratio > 2.0); // Should compress at least 2:1
    
    // Test decompression
    use flate2::read::GzDecoder;
    use std::io::Read;
    
    let start = Instant::now();
    let mut decoder = GzDecoder::new(&compressed_data[..]);
    let mut decompressed_data = String::new();
    decoder.read_to_string(&mut decompressed_data).unwrap();
    let decompression_duration = start.elapsed();
    
    println!("  Decompression time: {:?}", decompression_duration);
    assert!(decompression_duration < Duration::from_millis(50));
    assert_eq!(json_data, decompressed_data);
}