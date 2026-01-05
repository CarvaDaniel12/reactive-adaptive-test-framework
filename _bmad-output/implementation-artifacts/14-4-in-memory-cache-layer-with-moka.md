# Story 14.4: In-Memory Cache Layer with Moka

**As a** user of the application  
**I want** frequently accessed data to be cached  
**So that** the application responds faster and reduces database load

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 14.4 |
| Epic | Rust Implementation Improvements |
| Sprint | 3 - Performance - Caching |
| Priority | P1 |
| Estimated Days | 2 |
| Dependencies | 14.3 (Prometheus Metrics Integration) |
| Status | ready-for-dev |

---

## Technical Requirements

### 1. Add `moka` crate for async in-memory caching

- Use `moka` crate for high-performance async caching
- Enable `future` feature for async API
- Configure caches with appropriate TTL and capacity limits

### 2. Create `AppCache` struct with typed caches

Implement the following typed caches:

#### Ticket Cache
- **Type**: `Cache<String, JiraTicket>`
- **TTL**: 5 minutes (300 seconds)
- **TTI** (Time to Idle): 1 minute (60 seconds)
- **Max Capacity**: 1,000 entries
- **Key**: Ticket key (e.g., "PROJ-123")
- **Use Case**: Cache Jira ticket details to reduce API calls

#### Search Results Cache
- **Type**: `Cache<String, Vec<SearchResult>>`
- **TTL**: 2 minutes (120 seconds)
- **Max Capacity**: 500 entries
- **Key**: Search query hash or keyword
- **Use Case**: Cache search results from Postman/Testmo

#### Dashboard Metrics Cache
- **Type**: `Cache<String, DashboardMetrics>`
- **TTL**: 30 seconds
- **Max Capacity**: 50 entries
- **Key**: Dashboard type + period (e.g., "kpi-7d")
- **Use Case**: Cache expensive dashboard calculations

#### Workflow Template Cache
- **Type**: `Cache<uuid::Uuid, WorkflowTemplate>`
- **TTL**: 10 minutes (600 seconds)
- **Max Capacity**: 100 entries
- **Key**: Template UUID
- **Use Case**: Cache workflow templates for quick lookup

### 3. Implement cache invalidation on writes

- Invalidate cache entries when data is modified
- Support selective invalidation (by key)
- Implement cache invalidation on workflow state changes
- Clear cache entries for updated Jira tickets

### 4. Add cache statistics to metrics

- Track cache hit rate (`cache_hits_total`, `cache_misses_total`)
- Track cache size (`cache_entries_total`)
- Track cache evictions (`cache_evictions_total`)
- Labels by cache type (tickets, search, metrics, templates)

### 5. Add `X-Cache-Status` header (HIT/MISS) to responses

- Add header to responses indicating cache status
- Values: "HIT" (cache hit), "MISS" (cache miss)
- Apply to tickets and search endpoints
- Useful for debugging cache behavior

---

## Acceptance Criteria

- [ ] Repeated ticket fetches return cached data
- [ ] Cache respects TTL and evicts expired entries
- [ ] Write operations invalidate relevant cache entries
- [ ] Cache statistics exposed via `/metrics`
- [ ] Response headers indicate cache status (X-Cache-Status)
- [ ] Memory usage bounded by max capacity
- [ ] No stale data returned after writes
- [ ] Cache improves response time for cached endpoints (>30% reduction)

---

## Implementation Notes

### Installation

Add `moka` to workspace dependencies:

```toml
# Cargo.toml

[workspace.dependencies]
moka = { version = "0.12", features = ["future"] }
```

Add to core crate:

```toml
# crates/qa-pms-core/Cargo.toml

[dependencies]
moka = { workspace = true }
```

### Cache Implementation

```rust
// crates/qa-pms-core/src/cache.rs

use moka::future::Cache;
use std::time::Duration;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Application cache wrapper for all in-memory caches
pub struct AppCache {
    /// Cache for Jira tickets
    pub tickets: Cache<String, JiraTicket>,
    
    /// Cache for search results
    pub search: Cache<String, Vec<SearchResult>>,
    
    /// Cache for dashboard metrics
    pub metrics: Cache<String, DashboardMetrics>,
    
    /// Cache for workflow templates
    pub templates: Cache<Uuid, WorkflowTemplate>,
}

/// Jira ticket for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraTicket {
    pub key: String,
    pub summary: String,
    pub status: String,
    pub description: Option<String>,
    pub assignee: Option<String>,
    pub priority: Option<String>,
}

/// Search result for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub source: String, // "postman" or "testmo"
    pub name: String,
    pub url: String,
    pub relevance: f64,
}

/// Dashboard metrics for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardMetrics {
    pub tickets_completed: u64,
    pub average_time: f64,
    pub patterns_detected: u64,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

/// Workflow template for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTemplate {
    pub id: Uuid,
    pub name: String,
    pub steps: Vec<String>,
    pub workflow_type: String,
}

impl AppCache {
    /// Create a new AppCache instance with configured caches
    pub fn new() -> Self {
        Self {
            tickets: Cache::builder()
                .max_capacity(1_000)
                .time_to_live(Duration::from_secs(300)) // 5 minutes
                .time_to_idle(Duration::from_secs(60))   // 1 minute idle
                .build(),
            
            search: Cache::builder()
                .max_capacity(500)
                .time_to_live(Duration::from_secs(120)) // 2 minutes
                .build(),
            
            metrics: Cache::builder()
                .max_capacity(50)
                .time_to_live(Duration::from_secs(30))  // 30 seconds
                .build(),
            
            templates: Cache::builder()
                .max_capacity(100)
                .time_to_live(Duration::from_secs(600)) // 10 minutes
                .build(),
        }
    }
    
    /// Invalidate ticket cache for a specific key
    pub fn invalidate_ticket(&self, key: &str) {
        self.tickets.invalidate(key);
    }
    
    /// Invalidate search results cache
    pub fn invalidate_search(&self, key: &str) {
        self.search.invalidate(key);
    }
    
    /// Invalidate dashboard metrics cache
    pub fn invalidate_metrics(&self, key: &str) {
        self.metrics.invalidate(key);
    }
    
    /// Invalidate workflow template cache
    pub fn invalidate_template(&self, id: &Uuid) {
        self.templates.invalidate(id);
    }
    
    /// Clear all caches (useful for testing or manual invalidation)
    pub fn clear_all(&self) {
        self.tickets.invalidate_all();
        self.search.invalidate_all();
        self.metrics.invalidate_all();
        self.templates.invalidate_all();
    }
    
    /// Get cache statistics for monitoring
    pub fn get_stats(&self) -> CacheStats {
        CacheStats {
            tickets_size: self.tickets.entry_count(),
            search_size: self.search.entry_count(),
            metrics_size: self.metrics.entry_count(),
            templates_size: self.templates.entry_count(),
            tickets_hit_count: self.tickets.hit_count(),
            tickets_miss_count: self.tickets.miss_count(),
            search_hit_count: self.search.hit_count(),
            search_miss_count: self.search.miss_count(),
            metrics_hit_count: self.metrics.hit_count(),
            metrics_miss_count: self.metrics.miss_count(),
            templates_hit_count: self.templates.hit_count(),
            templates_miss_count: self.templates.miss_count(),
        }
    }
}

/// Cache statistics for metrics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub tickets_size: u64,
    pub search_size: u64,
    pub metrics_size: u64,
    pub templates_size: u64,
    pub tickets_hit_count: u64,
    pub tickets_miss_count: u64,
    pub search_hit_count: u64,
    pub search_miss_count: u64,
    pub metrics_hit_count: u64,
    pub metrics_miss_count: u64,
    pub templates_hit_count: u64,
    pub templates_miss_count: u64,
}
```

### Export from Core Crate

```rust
// crates/qa-pms-core/src/lib.rs

pub mod cache;

pub use cache::{AppCache, JiraTicket, SearchResult, DashboardMetrics, WorkflowTemplate, CacheStats};
```

### Add Cache Metrics

```rust
// crates/qa-pms-api/src/metrics.rs

use prometheus::{IntGauge, IntCounter, LazyLock};

/// Cache hit counter
pub static CACHE_HITS_TOTAL: LazyLock<IntCounter> = LazyLock::new(|| {
    prometheus::register_int_counter!(
        "qa_pms_cache_hits_total",
        "Total number of cache hits",
        &["cache_type"]
    ).expect("Failed to register cache_hits_total counter")
});

/// Cache miss counter
pub static CACHE_MISSES_TOTAL: LazyLock<IntCounter> = LazyLock::new(|| {
    prometheus::register_int_counter!(
        "qa_pms_cache_misses_total",
        "Total number of cache misses",
        &["cache_type"]
    ).expect("Failed to register cache_misses_total counter")
});

/// Cache entries gauge
pub static CACHE_ENTRIES_TOTAL: LazyLock<IntGauge> = LazyLock::new(|| {
    prometheus::register_int_gauge!(
        "qa_pms_cache_entries_total",
        "Total number of entries in cache",
        &["cache_type"]
    ).expect("Failed to register cache_entries_total gauge")
});

/// Cache evictions counter
pub static CACHE_EVICTIONS_TOTAL: LazyLock<IntCounter> = LazyLock::new(|| {
    prometheus::register_int_counter!(
        "qa_pms_cache_evictions_total",
        "Total number of cache evictions",
        &["cache_type"]
    ).expect("Failed to register cache_evictions_total counter")
});
```

### Update App State

```rust
// crates/qa-pms-api/src/app.rs

use qa_pms_core::AppCache;

#[derive(Clone)]
pub struct AppState {
    pub settings: Settings,
    pub health_store: Arc<RwLock<HealthStore>>,
    pub db_pool: PgPool,
    pub cache: Arc<AppCache>,
}

pub async fn create_app(settings: Settings) -> Result<Router> {
    // ... existing setup ...
    
    // Create cache instance
    let cache = Arc::new(AppCache::new());
    
    // Create the main router
    let app = Router::new()
        // ... existing routes ...
        .with_state(AppState {
            settings,
            health_store,
            db_pool,
            cache,
        });
    
    Ok(app)
}
```

### Use Cache in Tickets Route

```rust
// crates/qa-pms-api/src/routes/tickets.rs

use axum::{extract::State, response::{IntoResponse, Response, Json}, http::header};
use crate::cache::AppCache;
use crate::metrics::{CACHE_HITS_TOTAL, CACHE_MISSES_TOTAL};
use crate::AppError;

pub async fn get_ticket(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Response, AppError> {
    // Try to get from cache
    if let Some(ticket) = state.cache.tickets.get(&key).await {
        CACHE_HITS_TOTAL
            .with_label_values(&["tickets"])
            .inc();
        
        return Ok((
            [(header::CACHE_STATUS, "HIT")],
            Json(ticket)
        ).into_response());
    }
    
    CACHE_MISSES_TOTAL
        .with_label_values(&["tickets"])
        .inc();
    
    // Fetch from database
    let ticket = sqlx::query_as::<_, JiraTicket>(
        "SELECT key, summary, status, description, assignee, priority FROM jira_tickets WHERE key = $1"
    )
    .bind(&key)
    .fetch_one(&state.db_pool)
    .await?;
    
    // Store in cache
    state.cache.tickets.insert(key.clone(), ticket.clone()).await;
    
    Ok((
        [(header::CACHE_STATUS, "MISS")],
        Json(ticket)
    ).into_response())
}

pub async fn update_ticket(
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(request): Json<UpdateTicketRequest>,
) -> Result<Json<JiraTicket>, AppError> {
    // Update in database
    sqlx::query!(
        "UPDATE jira_tickets SET status = $1, assignee = $2 WHERE key = $3",
        request.status,
        request.assignee,
        key
    )
    .execute(&state.db_pool)
    .await?;
    
    // Invalidate cache
    state.cache.invalidate_ticket(&key);
    
    // Fetch updated ticket
    let ticket = sqlx::query_as::<_, JiraTicket>(
        "SELECT key, summary, status, description, assignee, priority FROM jira_tickets WHERE key = $1"
    )
    .bind(&key)
    .fetch_one(&state.db_pool)
    .await?;
    
    Ok(Json(ticket))
}
```

### Use Cache in Search Route

```rust
// crates/qa-pms-api/src/routes/search.rs

use axum::{extract::State, response::{IntoResponse, Response, Json}, http::header};
use crate::metrics::{CACHE_HITS_TOTAL, CACHE_MISSES_TOTAL};
use crate::AppError;

pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Response, AppError> {
    let cache_key = format!("{}:{}", params.query, params.limit.unwrap_or(10));
    
    // Try to get from cache
    if let Some(results) = state.cache.search.get(&cache_key).await {
        CACHE_HITS_TOTAL
            .with_label_values(&["search"])
            .inc();
        
        return Ok((
            [(header::CACHE_STATUS, "HIT")],
            Json(results)
        ).into_response());
    }
    
    CACHE_MISSES_TOTAL
        .with_label_values(&["search"])
        .inc();
    
    // Perform search
    let mut results = Vec::new();
    
    // Search Postman
    if let Some(postman_results) = search_postman(&state, &params.query).await? {
        results.extend(postman_results);
    }
    
    // Search Testmo
    if let Some(testmo_results) = search_testmo(&state, &params.query).await? {
        results.extend(testmo_results);
    }
    
    // Sort by relevance and limit
    results.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());
    if let Some(limit) = params.limit {
        results.truncate(limit);
    }
    
    // Store in cache
    state.cache.search.insert(cache_key.clone(), results.clone()).await;
    
    Ok((
        [(header::CACHE_STATUS, "MISS")],
        Json(results)
    ).into_response())
}
```

### Update Cache Metrics Periodically

```rust
// crates/qa-pms-api/src/app.rs

use crate::metrics::{CACHE_ENTRIES_TOTAL, CACHE_EVICTIONS_TOTAL};
use crate::cache::AppCache;

pub async fn update_cache_metrics_task(cache: Arc<AppCache>) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
    
    loop {
        interval.tick().await;
        
        let stats = cache.get_stats();
        
        // Update cache entry counts
        CACHE_ENTRIES_TOTAL
            .with_label_values(&["tickets"])
            .set(stats.tickets_size as i64);
        CACHE_ENTRIES_TOTAL
            .with_label_values(&["search"])
            .set(stats.search_size as i64);
        CACHE_ENTRIES_TOTAL
            .with_label_values(&["metrics"])
            .set(stats.metrics_size as i64);
        CACHE_ENTRIES_TOTAL
            .with_label_values(&["templates"])
            .set(stats.templates_size as i64);
        
        // Evictions are already tracked by moka's internal metrics
        // We can expose them if needed using moka's internal APIs
    }
}
```

---

## Dependencies to Add

### Workspace Dependencies

```toml
# Cargo.toml

[workspace.dependencies]
moka = { version = "0.12", features = ["future"] }
```

### Crate Dependencies

```toml
# crates/qa-pms-core/Cargo.toml

[dependencies]
moka = { workspace = true }
```

---

## Files to Create

| File | Description |
|------|-------------|
| `crates/qa-pms-core/src/cache.rs` | Cache implementation with AppCache and cache structs |

---

## Files to Modify

| File | Type | Changes |
|-------|------|---------|
| `Cargo.toml` | Modify | Add `moka` to workspace dependencies |
| `crates/qa-pms-core/Cargo.toml` | Modify | Add `moka` crate dependency |
| `crates/qa-pms-core/src/lib.rs` | Modify | Export cache module |
| `crates/qa-pms-api/src/app.rs` | Modify | Add cache to AppState, start metrics update task |
| `crates/qa-pms-api/src/routes/tickets.rs` | Modify | Use cache for ticket fetches, invalidate on updates |
| `crates/qa-pms-api/src/routes/search.rs` | Modify | Use cache for search results |
| `crates/qa-pms-api/src/metrics.rs` | Modify | Add cache metrics (hits, misses, entries, evictions) |

---

## Testing Strategy

### Unit Tests for Cache

```rust
// crates/qa-pms-core/tests/cache_test.rs

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    #[tokio::test]
    async fn test_ticket_cache_hit() {
        let cache = AppCache::new();
        
        // Insert ticket
        let ticket = JiraTicket {
            key: "TEST-123".to_string(),
            summary: "Test ticket".to_string(),
            status: "Open".to_string(),
            description: None,
            assignee: None,
            priority: None,
        };
        cache.tickets.insert("TEST-123".to_string(), ticket.clone()).await;
        
        // Retrieve from cache
        let cached = cache.tickets.get("TEST-123").await;
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().key, "TEST-123");
    }
    
    #[tokio::test]
    async fn test_ticket_cache_miss() {
        let cache = AppCache::new();
        
        // Try to get non-existent ticket
        let cached = cache.tickets.get("NONEXISTENT").await;
        assert!(cached.is_none());
    }
    
    #[tokio::test]
    async fn test_cache_invalidation() {
        let cache = AppCache::new();
        
        // Insert ticket
        let ticket = JiraTicket {
            key: "TEST-456".to_string(),
            summary: "Test ticket".to_string(),
            status: "Open".to_string(),
            description: None,
            assignee: None,
            priority: None,
        };
        cache.tickets.insert("TEST-456".to_string(), ticket).await;
        
        // Invalidate
        cache.invalidate_ticket("TEST-456");
        
        // Verify invalidated
        let cached = cache.tickets.get("TEST-456").await;
        assert!(cached.is_none());
    }
    
    #[tokio::test]
    async fn test_cache_ttl() {
        let cache = Cache::builder()
            .max_capacity(100)
            .time_to_live(Duration::from_millis(100))
            .build();
        
        // Insert
        cache.insert("key".to_string(), "value".to_string()).await;
        
        // Wait for TTL to expire
        tokio::time::sleep(Duration::from_millis(150)).await;
        
        // Should be expired
        let cached = cache.get("key").await;
        assert!(cached.is_none());
    }
}
```

### Integration Tests

```rust
// crates/qa-pms-api/tests/integration/cache_test.rs

use reqwest::StatusCode;
use std::time::Instant;

#[tokio::test]
async fn test_ticket_caching() {
    let addr = start_test_server().await;
    let client = reqwest::Client::new();
    
    // First request - should be cache miss
    let start = Instant::now();
    let response1 = client.get(format!("http://{}/api/v1/tickets/TEST-123", addr))
        .send()
        .await
        .expect("Failed to get ticket");
    let duration1 = start.elapsed();
    
    assert_eq!(response1.status(), StatusCode::OK);
    assert_eq!(response1.headers().get("x-cache-status").unwrap(), "MISS");
    
    // Second request - should be cache hit
    let start = Instant::now();
    let response2 = client.get(format!("http://{}/api/v1/tickets/TEST-123", addr))
        .send()
        .await
        .expect("Failed to get ticket");
    let duration2 = start.elapsed();
    
    assert_eq!(response2.status(), StatusCode::OK);
    assert_eq!(response2.headers().get("x-cache-status").unwrap(), "HIT");
    
    // Cached response should be significantly faster
    assert!(duration2 < duration1);
}

#[tokio::test]
async fn test_cache_invalidation_on_update() {
    let addr = start_test_server().await;
    let client = reqwest::Client::new();
    
    // Get ticket (cache miss)
    let response1 = client.get(format!("http://{}/api/v1/tickets/TEST-456", addr))
        .send()
        .await
        .expect("Failed to get ticket");
    assert_eq!(response1.headers().get("x-cache-status").unwrap(), "MISS");
    
    // Update ticket
    client.patch(format!("http://{}/api/v1/tickets/TEST-456", addr))
        .json(&serde_json::json!({"status": "Closed"}))
        .send()
        .await
        .expect("Failed to update ticket");
    
    // Get ticket again - should be cache miss (invalidated)
    let response2 = client.get(format!("http://{}/api/v1/tickets/TEST-456", addr))
        .send()
        .await
        .expect("Failed to get ticket");
    assert_eq!(response2.headers().get("x-cache-status").unwrap(), "MISS");
}
```

### Performance Tests

```rust
// crates/qa-pms-api/tests/performance/cache_performance_test.rs

#[tokio::test]
async fn test_cache_performance_improvement() {
    let addr = start_test_server().await;
    let client = reqwest::Client::new();
    
    // Warm up cache
    for _ in 0..10 {
        client.get(format!("http://{}/api/v1/tickets/TEST-789", addr))
            .send()
            .await
            .expect("Failed to get ticket");
    }
    
    // Measure cached response times
    let mut durations = Vec::new();
    for _ in 0..100 {
        let start = Instant::now();
        client.get(format!("http://{}/api/v1/tickets/TEST-789", addr))
            .send()
            .await
            .expect("Failed to get ticket");
        durations.push(start.elapsed());
    }
    
    let avg_duration: Duration = durations.iter().sum::<Duration>() / durations.len() as u32;
    
    // Cached responses should be very fast (< 10ms)
    assert!(avg_duration.as_millis() < 10, "Average cached duration: {:?}", avg_duration);
}
```

---

## Success Metrics

- **Performance**: 30-50% reduction in response time for cached endpoints
- **Cache Hit Rate**: >70% for frequently accessed data after warmup
- **Memory Usage**: Bounded by max capacity (no unbounded growth)
- **Latency**: < 1ms overhead for cache operations
- **Metrics**: Cache statistics exposed via `/metrics` endpoint
- **Correctness**: No stale data returned after updates

---

## Context and Dependencies

This story depends on:
- **Story 14.3**: Prometheus metrics integration to track cache performance

This story enables:
- **Story 14.5**: Rate limiting metrics will complement cache metrics
- **Story 14.6**: Distributed tracing will show cache hit/miss in request flows
- **Story 14.7**: CLI can include cache management commands

---

## Risks and Mitigation

| Risk | Probability | Impact | Mitigation |
|-------|-------------|--------|------------|
| Stale data after updates | Medium | High | Implement proper cache invalidation on writes |
| Cache eviction bugs | Low | High | Comprehensive integration tests, conservative TTLs |
| Memory leaks | Low | High | Use bounded caches, monitor memory usage with metrics |
| Cache stampede | Low | Medium | Consider implementing cache warming for hot keys |

---

## Next Steps

After this story is complete:
1. Monitor cache hit/miss rates via Prometheus
2. Adjust TTL and capacity based on production data
3. Implement cache warming for frequently accessed tickets
4. Proceed to Story 14.5 (Rate Limiting)