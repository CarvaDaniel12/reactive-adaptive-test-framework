# Cargo.toml

[workspace.dependencies]
tower-governor = "0.4"
```

Add to API crate:

```toml
# crates/qa-pms-api/Cargo.toml

[dependencies]
tower-governor = { workspace = true }
```

### Rate Limiting Configuration

```rust
// crates/qa-pms-api/src/rate_limit.rs

use tower_governor::{
    governor::GovernorConfigBuilder,
    key_extractor::SmartIpKeyExtractor,
    GovernorLayer,
};
use std::sync::Arc;
use std::time::Duration;

/// Rate limiting configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Number of requests allowed per minute
    pub requests_per_minute: u32,
    /// Burst size (allow temporary burst above sustained rate)
    pub burst_size: u32,
    /// Whether rate limiting is enabled
    pub enabled: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 100,
            burst_size: 20,
            enabled: true,
        }
    }
}

impl RateLimitConfig {
    /// Load from environment variables
    pub fn from_env() -> Self {
        Self {
            requests_per_minute: std::env::var("RATE_LIMIT_REQUESTS_PER_MINUTE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            burst_size: std::env::var("RATE_LIMIT_BURST_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(20),
            enabled: std::env::var("RATE_LIMIT_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
        }
    }
}

/// Create rate limiting layer
pub fn create_rate_limit_layer(
    config: &RateLimitConfig,
) -> Option<GovernorLayer<SmartIpKeyExtractor>> {
    if !config.enabled {
        return None;
    }

    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_millisecond((60_000 / config.requests_per_minute) as u64)
            .burst_size(config.burst_size)
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .expect("Failed to build rate limiter configuration"),
    );

    Some(GovernorLayer::new(governor_conf))
}

/// Start background cleanup task for rate limiter
pub async fn start_cleanup_task(config: Arc<RateLimitConfig>) {
    if !config.enabled {
        return;
    }

    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_millisecond(600) // 100 req/min default
            .burst_size(20)
            .key_extractor(SmartIpKeyExtractor)
            .finish()
            .expect("Failed to build rate limiter configuration"),
    );

    let governor_limiter = governor_conf.limiter().clone();
    
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            governor_limiter.retain_recent();
            tracing::debug!("Rate limiter cleanup completed");
        }
    });
}
```

### Update App State

```rust
// crates/qa-pms-api/src/app.rs

use crate::rate_limit::{RateLimitConfig, create_rate_limit_layer, start_cleanup_task};

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub health_store: Arc<RwLock<HealthStore>>,
    pub db_pool: PgPool,
    pub cache: Arc<AppCache>,
    pub rate_limit_config: RateLimitConfig,
}

pub async fn create_app(settings: Settings) -> Result<Router> {
    // ... existing setup (health store, cache, etc.) ...
    
    // Create rate limiting configuration
    let rate_limit_config = RateLimitConfig::from_env();
    let rate_limit_config_arc = Arc::new(rate_limit_config.clone());
    
    // Start cleanup task
    start_cleanup_task(rate_limit_config_arc).await;
    
    // Create rate limiting layer
    let rate_limit_layer = create_rate_limit_layer(&rate_limit_config);
    
    // Create the main router
    let mut router = Router::new()
        // Metrics endpoint (excluded from rate limiting)
        .route("/metrics", get(|| async move { metric_handle.render() }))
        
        // Health endpoints (excluded from rate limiting)
        .merge(routes::health::router(Arc::clone(&health_store)))
        
        // Protected routes (rate limited)
        .merge(routes::dashboard::router(Arc::clone(&health_store)))
        .merge(routes::tickets::router(Arc::clone(&health_store)))
        .merge(routes::workflows::router(Arc::clone(&health_store)))
        .merge(routes::patterns::router(Arc::clone(&health_store)))
        .merge(routes::splunk::router(Arc::clone(&health_store)))
        .merge(routes::support::router(Arc::clone(&health_store)))
        .merge(routes::ai::router(Arc::clone(&health_store)));
    
    // Apply middleware layers
    let mut service_builder = tower::ServiceBuilder::new()
        .layer(prometheus_layer)
        .layer(request_id_middleware())
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        );
    
    // Add rate limiting layer if enabled
    if let Some(layer) = rate_limit_layer {
        service_builder = service_builder.layer(layer);
    }
    
    router = router.layer(service_builder);
    
    Ok(router)
}
```

### Update Main to Use SocketAddr for IP Extraction

```rust
// crates/qa-pms-api/src/main.rs

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ... existing initialization ...
    
    let addr = settings.api.address.parse::<SocketAddr>()?;
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    info!("Starting server on http://{}", addr);
    
    // Create app
    let app = create_app(settings).await?;
    
    // IMPORTANT: Use into_make_service_with_connect_info for IP extraction
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    
    info!("Server shut down gracefully");
    Ok(())
}
```

### Add Rate Limit Metrics

```rust
// crates/qa-pms-api/src/metrics.rs

use prometheus::{IntCounter, IntGauge, LazyLock};

/// Rate limit rejections counter
pub static RATE_LIMIT_REJECTIONS_TOTAL: LazyLock<IntCounter> = LazyLock::new(|| {
    prometheus::register_int_counter!(
        "qa_pms_rate_limit_rejections_total",
        "Total number of requests rejected due to rate limiting",
        &["path"]
    ).expect("Failed to register rate_limit_rejections_total counter")
});

/// Active rate limited IP addresses gauge
pub static RATE_LIMIT_ACTIVE_IPS: LazyLock<IntGauge> = LazyLock::new(|| {
    prometheus::register_int_gauge!(
        "qa_pms_rate_limit_active_ips",
        "Number of IP addresses currently being rate limited"
    ).expect("Failed to register rate_limit_active_ips gauge")
});

/// Rate limit bucket utilization gauge
pub static RATE_LIMIT_BUCKET_UTILIZATION: LazyLock<IntGauge> = LazyLock::new(|| {
    prometheus::register_int_gauge!(
        "qa_pms_rate_limit_bucket_utilization",
        "Rate limit bucket utilization percentage (0-100)"
    ).expect("Failed to register rate_limit_bucket_utilization gauge")
});
```

### Custom Rate Limit Response with Headers

```rust
// crates/qa-pms-api/src/rate_limit.rs

use axum::{
    extract::Request,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};
use std::time::{SystemTime, UNIX_EPOCH};

/// Middleware to add rate limit headers to all responses
pub async fn rate_limit_headers_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    
    // Add rate limit headers if rate limiting is enabled
    // Note: tower-governor handles 429 responses, we add informational headers
    let headers = response.headers_mut();
    
    headers.insert(
        "X-RateLimit-Limit",
        HeaderValue::from_static("100"),
    );
    
    // These would be populated from actual rate limiter state
    // For now, set placeholder values that would be updated by governor
    headers.insert(
        "X-RateLimit-Remaining",
        HeaderValue::from_str("95").unwrap_or_default(),
    );
    
    let reset_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 60;
    headers.insert(
        "X-RateLimit-Reset",
        HeaderValue::from(reset_time).unwrap_or_default(),
    );
    
    response
}

/// Custom 429 response with retry-after
#[derive(Debug)]
pub struct RateLimitResponse {
    pub retry_after: u64,
}

impl IntoResponse for RateLimitResponse {
    fn into_response(self) -> Response {
        let body = serde_json::json!({
            "error": "Rate limit exceeded",
            "message": "Too many requests. Please try again later.",
            "retry_after": self.retry_after,
        });
        
        (
            StatusCode::TOO_MANY_REQUESTS,
            [(header::RETRY_AFTER, HeaderValue::from(self.retry_after))],
            Json(body),
        ).into_response()
    }
}
```

---

## Dependencies to Add

### Workspace Dependencies

```toml
# Cargo.toml

[workspace.dependencies]
tower-governor = "0.4"
```

### Crate Dependencies

```toml
# crates/qa-pms-api/Cargo.toml

[dependencies]
tower-governor = { workspace = true }
```

---

## Files to Create

| File | Description |
|------|-------------|
| `crates/qa-pms-api/src/rate_limit.rs` | Rate limiting configuration and middleware |

---

## Files to Modify

| File | Type | Changes |
|-------|------|---------|
| `Cargo.toml` | Modify | Add `tower-governor` to workspace dependencies |
| `crates/qa-pms-api/Cargo.toml` | Modify | Add `tower-governor` crate dependency |
| `crates/qa-pms-api/src/app.rs` | Modify | Add rate limiting layer, update AppState |
| `crates/qa-pms-api/src/main.rs` | Modify | Use `into_make_service_with_connect_info` for IP extraction |
| `crates/qa-pms-api/src/metrics.rs` | Modify | Add rate limit metrics (rejections, active IPs, utilization) |

---

## Testing Strategy

### Unit Tests for Rate Limiting

```rust
// crates/qa-pms-api/tests/rate_limit_test.rs

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    #[test]
    fn test_rate_limit_config_from_env() {
        std::env::set_var("RATE_LIMIT_REQUESTS_PER_MINUTE", "200");
        std::env::set_var("RATE_LIMIT_BURST_SIZE", "40");
        std::env::set_var("RATE_LIMIT_ENABLED", "true");
        
        let config = RateLimitConfig::from_env();
        
        assert_eq!(config.requests_per_minute, 200);
        assert_eq!(config.burst_size, 40);
        assert_eq!(config.enabled, true);
    }
    
    #[test]
    fn test_rate_limit_config_defaults() {
        // Clear environment variables
        std::env::remove_var("RATE_LIMIT_REQUESTS_PER_MINUTE");
        std::env::remove_var("RATE_LIMIT_BURST_SIZE");
        std::env::remove_var("RATE_LIMIT_ENABLED");
        
        let config = RateLimitConfig::default();
        
        assert_eq!(config.requests_per_minute, 100);
        assert_eq!(config.burst_size, 20);
        assert_eq!(config.enabled, true);
    }
    
    #[test]
    fn test_rate_limit_disabled() {
        std::env::set_var("RATE_LIMIT_ENABLED", "false");
        
        let config = RateLimitConfig::from_env();
        let layer = create_rate_limit_layer(&config);
        
        assert!(layer.is_none());
    }
}
```

### Integration Tests

```rust
// crates/qa-pms-api/tests/integration/rate_limit_test.rs

use reqwest::StatusCode;
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_rate_limit_enforcement() {
    let addr = start_test_server().await;
    let client = reqwest::Client::new();
    
    // Make rapid requests to trigger rate limit
    let mut responses = Vec::new();
    
    for _ in 0..150 {
        let response = client.get(format!("http://{}/api/v1/dashboard/kpi", addr))
            .send()
            .await
            .expect("Failed to send request");
        
        responses.push(response.status());
        
        // Small delay to allow some requests to succeed
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    
    // Count 429 responses
    let rejected_count = responses.iter().filter(|&&s| s == StatusCode::TOO_MANY_REQUESTS).count();
    
    // Should have some rejections after hitting rate limit
    assert!(rejected_count > 0, "Expected some rate limit rejections, got {}", rejected_count);
}

#[tokio::test]
async fn test_rate_limit_headers() {
    let addr = start_test_server().await;
    let client = reqwest::Client::new();
    
    let response = client.get(format!("http://{}/api/v1/dashboard/kpi", addr))
        .send()
        .await
        .expect("Failed to send request");
    
    assert_eq!(response.status(), StatusCode::OK);
    
    // Check for rate limit headers
    let headers = response.headers();
    assert!(headers.contains_key("X-RateLimit-Limit"));
    assert!(headers.contains_key("X-RateLimit-Remaining"));
    assert!(headers.contains_key("X-RateLimit-Reset"));
}

#[tokio::test]
async fn test_retry_after_header() {
    let addr = start_test_server().await;
    let client = reqwest::Client::new();
    
    // Make rapid requests until rate limited
    let mut response;
    for _ in 0..200 {
        response = client.get(format!("http://{}/api/v1/dashboard/kpi", addr))
            .send()
            .await
            .expect("Failed to send request");
        
        if response.status() == StatusCode::TOO_MANY_REQUESTS {
            break;
        }
    }
    
    // Check Retry-After header
    assert_eq!(response.unwrap().status(), StatusCode::TOO_MANY_REQUESTS);
    let headers = response.unwrap().headers();
    assert!(headers.contains_key("Retry-After"));
    
    let retry_after = headers.get("Retry-After")
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    
    // Should be a reasonable value (0-60 seconds)
    assert!(retry_after <= 60, "Retry-After should be reasonable: {}", retry_after);
}

#[tokio::test]
async fn test_excluded_endpoints_not_rate_limited() {
    let addr = start_test_server().await;
    let client = reqwest::Client::new();
    
    // Make many requests to /metrics endpoint
    let mut all_ok = true;
    for _ in 0..200 {
        let response = client.get(format!("http://{}/metrics", addr))
            .send()
            .await
            .expect("Failed to send request");
        
        if response.status() != StatusCode::OK {
            all_ok = false;
            break;
        }
    }
    
    // Metrics endpoint should not be rate limited
    assert!(all_ok, "Metrics endpoint should not be rate limited");
}
```

### Load Tests

```rust
// crates/qa-pms-api/tests/load/rate_limit_load_test.rs

use tokio::task::JoinSet;

#[tokio::test]
async fn test_concurrent_requests_respects_rate_limit() {
    let addr = start_test_server().await;
    
    // Spawn 50 concurrent clients
    let mut join_set = JoinSet::new();
    
    for client_id in 0..50 {
        let addr = addr.clone();
        join_set.spawn(async move {
            let client = reqwest::Client::new();
            let mut ok_count = 0;
            let mut rejected_count = 0;
            
            // Each client makes 10 requests
            for _ in 0..10 {
                let response = client.get(format!("http://{}/api/v1/dashboard/kpi", addr))
                    .send()
                    .await
                    .expect("Failed to send request");
                
                if response.status() == StatusCode::OK {
                    ok_count += 1;
                } else if response.status() == StatusCode::TOO_MANY_REQUESTS {
                    rejected_count += 1;
                }
            }
            
            (client_id, ok_count, rejected_count)
        });
    }
    
    // Collect results
    let mut total_ok = 0;
    let mut total_rejected = 0;
    
    while let Some(result) = join_set.join_next().await {
        let (_, ok, rejected) = result.unwrap();
        total_ok += ok;
        total_rejected += rejected;
    }
    
    // Some requests should be rejected due to rate limiting
    assert!(total_rejected > 0, "Expected some rejections, got 0");
    
    // Not all requests should be rejected (rate limit allows some through)
    assert!(total_ok > 0, "Expected some successful requests, got 0");
    
    println!("Rate limit test: {} OK, {} Rejected", total_ok, total_rejected);
}
```

---

## Success Metrics

- **Protection**: Rate limit blocks excessive requests (429 responses)
- **Headers**: All responses include rate limit headers (except excluded endpoints)
- **Retry-After**: 429 responses include valid Retry-After header
- **Exclusions**: Health and metrics endpoints not rate limited
- **Memory**: Cleanup task prevents unbounded growth
- **Configuration**: Limits configurable via environment variables
- **Logging**: Rate limit violations logged with request ID
- **Metrics**: Prometheus metrics track rejections and active IPs

---

## Context and Dependencies

This story depends on:
- **Story 14.3**: Prometheus metrics integration to track rate limit events

This story enables:
- **Story 14.6**: Distributed tracing will show rate limit decisions in request flows
- **Story 14.7**: CLI can include rate limit management commands

---

## Risks and Mitigation

| Risk | Probability | Impact | Mitigation |
|-------|-------------|--------|------------|
| False positives blocking legitimate users | Low | Medium | Start with generous limits (100 req/min), monitor and adjust |
| IP spoofing bypass | Low | Low | Rate limiting is best-effort protection, not complete security |
| Memory growth from tracking IPs | Low | Medium | Cleanup task runs every 60s to remove stale entries |
| Rate limiting affects health checks | Low | High | Explicitly exclude health/metrics endpoints |

---

## Next Steps

After this story is complete:
1. Monitor rate limit metrics via Prometheus
2. Adjust limits based on actual usage patterns
3. Consider implementing IP whitelisting for internal services
4. Proceed to Story 14.6 (OpenTelemetry Distributed Tracing)

---

## Operational Guidelines

### Monitoring

Monitor the following metrics:
- `qa_pms_rate_limit_rejections_total` - Total rejections by path
- `qa_pms_rate_limit_active_ips` - Number of tracked IPs
- `qa_pms_rate_limit_bucket_utilization` - Overall utilization

### Alerting

Set up alerts for:
- High rejection rate (> 10% of total requests)
- Sudden spike in active IPs (potential DDoS attempt)

### Configuration Adjustments

Start with default limits (100 req/min) and adjust based on:
- Average request rate per user
- Peak usage times
- Business requirements for specific endpoints

Consider higher limits for:
- Internal IPs
- Known partners/customers
- Specific API endpoints with lower impact

### Testing

Test rate limiting in staging environment before production:
- Verify 429 responses are returned correctly
- Check Retry-After header values
- Confirm excluded endpoints are not limited
- Monitor memory usage during load testing