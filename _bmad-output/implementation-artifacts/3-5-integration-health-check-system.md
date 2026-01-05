# Story 3.5: Integration Health Check System

Status: done

## Story

As a system administrator,
I want automatic health checks for integrations,
So that I'm alerted when something is wrong.

## Acceptance Criteria

1. **Given** the application is running
   **When** 60 seconds have elapsed since last check
   **Then** health checks run for all configured integrations

2. **Given** health check runs
   **When** Jira is configured
   **Then** Jira API endpoint is pinged

3. **Given** health check runs
   **When** Postman is configured
   **Then** Postman API key is validated

4. **Given** health check runs
   **When** Testmo is configured
   **Then** Testmo API key is validated

5. **Given** health check completes
   **When** response is received
   **Then** response time and status are recorded

6. **Given** health status changes
   **When** update occurs
   **Then** health status is stored in memory

7. **Given** integration is down
   **When** downtime exceeds 2 minutes
   **Then** alert is logged (NFR-REL-02)

8. **Given** health check is running
   **When** UI is active
   **Then** health check runs in background (doesn't block UI)

## Tasks / Subtasks

- [x] Task 1: Create HealthCheck trait and types (AC: #5, #6)
  - [x] 1.1: Define `HealthCheck` trait in `qa-pms-core`
  - [x] 1.2: Create `HealthStatus` enum (Online, Degraded, Offline)
  - [x] 1.3: Create `HealthCheckResult` struct
  - [x] 1.4: Create `IntegrationHealth` aggregate type

- [x] Task 2: Implement Jira health check (AC: #2)
  - [x] 2.1: Create `JiraHealthCheck` in `qa-pms-jira`
  - [x] 2.2: Ping `/rest/api/3/myself` endpoint
  - [x] 2.3: Measure response time
  - [x] 2.4: Return appropriate status

- [x] Task 3: Implement Postman health check (AC: #3)
  - [x] 3.1: Create `PostmanHealthCheck` in `qa-pms-postman`
  - [x] 3.2: Validate API key with `/me` endpoint
  - [x] 3.3: Measure response time
  - [x] 3.4: Return appropriate status

- [x] Task 4: Implement Testmo health check (AC: #4)
  - [x] 4.1: Create `TestmoHealthCheck` in `qa-pms-testmo`
  - [x] 4.2: Validate API key with ping endpoint
  - [x] 4.3: Measure response time
  - [x] 4.4: Return appropriate status

- [x] Task 5: Create health check scheduler (AC: #1, #8)
  - [x] 5.1: Create background task with 60s interval
  - [x] 5.2: Run checks in parallel (futures::join_all)
  - [x] 5.3: Spawn as non-blocking task

- [x] Task 6: Create health status store (AC: #6)
  - [x] 6.1: Create `HealthStore` with in-memory state
  - [x] 6.2: Track last successful check timestamp
  - [x] 6.3: Track consecutive failures count
  - [x] 6.4: Provide thread-safe access (tokio::sync::RwLock)

- [x] Task 7: Implement downtime alerting (AC: #7)
  - [x] 7.1: Track downtime duration per integration
  - [x] 7.2: Log warning when downtime > 2 minutes
  - [x] 7.3: Clear alert when integration recovers

- [x] Task 8: Create health status API endpoint (AC: #5)
  - [x] 8.1: Add `GET /api/v1/health/integrations` endpoint
  - [x] 8.2: Return all integration statuses
  - [x] 8.3: Include response times and errors

## Dev Notes

### Architecture Alignment

This story implements **Integration Health Check System** per Epic 3 requirements:

- **Core**: `crates/qa-pms-core/src/health.rs` (trait and types)
- **Integrations**: Health checks in each integration crate
- **API**: `GET /api/v1/health/integrations`

### Technical Implementation Details

#### Health Check Types

```rust
// crates/qa-pms-core/src/health.rs
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Online,
    Degraded,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResult {
    pub integration: String,
    pub status: HealthStatus,
    pub response_time_ms: Option<u64>,
    pub error_message: Option<String>,
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationHealth {
    pub integration: String,
    pub status: HealthStatus,
    pub last_successful_check: Option<DateTime<Utc>>,
    pub last_check: DateTime<Utc>,
    pub response_time_ms: Option<u64>,
    pub error_message: Option<String>,
    pub consecutive_failures: u32,
    pub downtime_start: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait HealthCheck: Send + Sync {
    fn integration_name(&self) -> &str;
    async fn check(&self) -> HealthCheckResult;
}

impl HealthCheckResult {
    pub fn online(integration: &str, response_time: Duration) -> Self {
        Self {
            integration: integration.to_string(),
            status: HealthStatus::Online,
            response_time_ms: Some(response_time.as_millis() as u64),
            error_message: None,
            checked_at: Utc::now(),
        }
    }

    pub fn degraded(integration: &str, response_time: Duration, message: &str) -> Self {
        Self {
            integration: integration.to_string(),
            status: HealthStatus::Degraded,
            response_time_ms: Some(response_time.as_millis() as u64),
            error_message: Some(message.to_string()),
            checked_at: Utc::now(),
        }
    }

    pub fn offline(integration: &str, error: &str) -> Self {
        Self {
            integration: integration.to_string(),
            status: HealthStatus::Offline,
            response_time_ms: None,
            error_message: Some(error.to_string()),
            checked_at: Utc::now(),
        }
    }
}
```

#### Jira Health Check

```rust
// crates/qa-pms-jira/src/health.rs
use async_trait::async_trait;
use qa_pms_core::health::{HealthCheck, HealthCheckResult};
use std::time::Instant;

pub struct JiraHealthCheck {
    client: JiraClient,
}

impl JiraHealthCheck {
    pub fn new(client: JiraClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl HealthCheck for JiraHealthCheck {
    fn integration_name(&self) -> &str {
        "jira"
    }

    async fn check(&self) -> HealthCheckResult {
        let start = Instant::now();
        
        let url = format!("{}/rest/api/3/myself", self.client.base_url);
        
        match self.client.http_client
            .get(&url)
            .bearer_auth(&self.client.access_token)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
        {
            Ok(response) => {
                let duration = start.elapsed();
                
                if response.status().is_success() {
                    if duration.as_secs() > 2 {
                        HealthCheckResult::degraded(
                            "jira",
                            duration,
                            "Response time exceeds 2s threshold"
                        )
                    } else {
                        HealthCheckResult::online("jira", duration)
                    }
                } else if response.status() == 401 {
                    HealthCheckResult::offline("jira", "Authentication expired - re-authentication required")
                } else {
                    HealthCheckResult::offline(
                        "jira",
                        &format!("HTTP {}", response.status())
                    )
                }
            }
            Err(e) => {
                if e.is_timeout() {
                    HealthCheckResult::offline("jira", "Request timeout (>10s)")
                } else if e.is_connect() {
                    HealthCheckResult::offline("jira", "Connection failed - check network")
                } else {
                    HealthCheckResult::offline("jira", &e.to_string())
                }
            }
        }
    }
}
```

#### Postman Health Check

```rust
// crates/qa-pms-postman/src/health.rs
use async_trait::async_trait;
use qa_pms_core::health::{HealthCheck, HealthCheckResult};
use std::time::Instant;

pub struct PostmanHealthCheck {
    api_key: String,
    http_client: reqwest::Client,
}

impl PostmanHealthCheck {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl HealthCheck for PostmanHealthCheck {
    fn integration_name(&self) -> &str {
        "postman"
    }

    async fn check(&self) -> HealthCheckResult {
        let start = Instant::now();
        
        match self.http_client
            .get("https://api.getpostman.com/me")
            .header("X-Api-Key", &self.api_key)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
        {
            Ok(response) => {
                let duration = start.elapsed();
                
                if response.status().is_success() {
                    HealthCheckResult::online("postman", duration)
                } else if response.status() == 401 {
                    HealthCheckResult::offline("postman", "Invalid API key")
                } else {
                    HealthCheckResult::offline(
                        "postman",
                        &format!("HTTP {}", response.status())
                    )
                }
            }
            Err(e) => {
                HealthCheckResult::offline("postman", &e.to_string())
            }
        }
    }
}
```

#### Testmo Health Check

```rust
// crates/qa-pms-testmo/src/health.rs
use async_trait::async_trait;
use qa_pms_core::health::{HealthCheck, HealthCheckResult};
use std::time::Instant;

pub struct TestmoHealthCheck {
    base_url: String,
    api_key: String,
    http_client: reqwest::Client,
}

impl TestmoHealthCheck {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
            http_client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl HealthCheck for TestmoHealthCheck {
    fn integration_name(&self) -> &str {
        "testmo"
    }

    async fn check(&self) -> HealthCheckResult {
        let start = Instant::now();
        
        let url = format!("{}/api/v1/user", self.base_url);
        
        match self.http_client
            .get(&url)
            .bearer_auth(&self.api_key)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
        {
            Ok(response) => {
                let duration = start.elapsed();
                
                if response.status().is_success() {
                    HealthCheckResult::online("testmo", duration)
                } else if response.status() == 401 {
                    HealthCheckResult::offline("testmo", "Invalid API key")
                } else {
                    HealthCheckResult::offline(
                        "testmo",
                        &format!("HTTP {}", response.status())
                    )
                }
            }
            Err(e) => {
                HealthCheckResult::offline("testmo", &e.to_string())
            }
        }
    }
}
```

#### Health Store

```rust
// crates/qa-pms-core/src/health_store.rs
use crate::health::{HealthCheckResult, HealthStatus, IntegrationHealth};
use chrono::{Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::warn;

pub struct HealthStore {
    state: Arc<RwLock<HashMap<String, IntegrationHealth>>>,
}

impl HealthStore {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn update(&self, result: HealthCheckResult) {
        let mut state = self.state.write().await;
        
        let entry = state.entry(result.integration.clone()).or_insert(IntegrationHealth {
            integration: result.integration.clone(),
            status: HealthStatus::Online,
            last_successful_check: None,
            last_check: Utc::now(),
            response_time_ms: None,
            error_message: None,
            consecutive_failures: 0,
            downtime_start: None,
        });

        entry.last_check = result.checked_at;
        entry.status = result.status;
        entry.response_time_ms = result.response_time_ms;
        entry.error_message = result.error_message.clone();

        match result.status {
            HealthStatus::Online => {
                entry.last_successful_check = Some(result.checked_at);
                entry.consecutive_failures = 0;
                
                // Log recovery if was down
                if entry.downtime_start.is_some() {
                    let downtime = entry.downtime_start
                        .map(|start| Utc::now() - start)
                        .unwrap_or_default();
                    tracing::info!(
                        "{} integration recovered after {:?}",
                        entry.integration,
                        downtime
                    );
                }
                entry.downtime_start = None;
            }
            HealthStatus::Degraded => {
                entry.last_successful_check = Some(result.checked_at);
                entry.consecutive_failures = 0;
                entry.downtime_start = None;
            }
            HealthStatus::Offline => {
                entry.consecutive_failures += 1;
                
                // Start downtime tracking
                if entry.downtime_start.is_none() {
                    entry.downtime_start = Some(Utc::now());
                }

                // Alert if down > 2 minutes (NFR-REL-02)
                if let Some(start) = entry.downtime_start {
                    let downtime = Utc::now() - start;
                    if downtime > Duration::minutes(2) {
                        warn!(
                            integration = %entry.integration,
                            downtime_minutes = downtime.num_minutes(),
                            consecutive_failures = entry.consecutive_failures,
                            error = ?entry.error_message,
                            "Integration has been offline for more than 2 minutes"
                        );
                    }
                }
            }
        }
    }

    pub async fn get_all(&self) -> Vec<IntegrationHealth> {
        self.state.read().await.values().cloned().collect()
    }

    pub async fn get(&self, integration: &str) -> Option<IntegrationHealth> {
        self.state.read().await.get(integration).cloned()
    }
}
```

#### Health Check Scheduler

```rust
// crates/qa-pms-api/src/health_scheduler.rs
use qa_pms_core::health::{HealthCheck, HealthStore};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tracing::info;

pub struct HealthScheduler {
    checks: Vec<Arc<dyn HealthCheck>>,
    store: Arc<HealthStore>,
    interval_secs: u64,
}

impl HealthScheduler {
    pub fn new(store: Arc<HealthStore>, interval_secs: u64) -> Self {
        Self {
            checks: Vec::new(),
            store,
            interval_secs,
        }
    }

    pub fn add_check(mut self, check: Arc<dyn HealthCheck>) -> Self {
        self.checks.push(check);
        self
    }

    pub async fn start(self) {
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(self.interval_secs));
            
            loop {
                interval.tick().await;
                
                info!("Running health checks for {} integrations", self.checks.len());
                
                // Run all checks in parallel
                let futures: Vec<_> = self.checks.iter().map(|c| c.check()).collect();
                let results = futures::future::join_all(futures).await;
                
                // Update store with results
                for result in results {
                    self.store.update(result).await;
                }
            }
        });
    }
}
```

#### API Endpoint

```rust
// crates/qa-pms-api/src/routes/health.rs
use axum::{extract::State, Json, routing::get, Router};
use qa_pms_core::health::IntegrationHealth;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/integrations", get(get_integration_health))
}

#[utoipa::path(
    get,
    path = "/api/v1/health/integrations",
    responses(
        (status = 200, description = "Integration health status", body = Vec<IntegrationHealth>),
    ),
    tag = "Health"
)]
async fn get_integration_health(
    State(state): State<AppState>,
) -> Json<Vec<IntegrationHealth>> {
    let health = state.health_store.get_all().await;
    Json(health)
}
```

### Project Structure Notes

Files to create:
```
crates/qa-pms-core/src/
├── health.rs           # HealthCheck trait and types
└── health_store.rs     # In-memory health store

crates/qa-pms-jira/src/
└── health.rs           # Jira health check

crates/qa-pms-postman/src/
└── health.rs           # Postman health check

crates/qa-pms-testmo/src/
└── health.rs           # Testmo health check

crates/qa-pms-api/src/
├── health_scheduler.rs # Background scheduler
└── routes/health.rs    # Health API routes
```

### Testing Notes

- Unit test each health check with mocked HTTP responses
- Unit test store updates for all status transitions
- Unit test 2-minute alerting threshold
- Integration test: Full scheduler cycle
- Test thread safety of HealthStore

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 3.5]
- [Source: _bmad-output/planning-artifacts/prd.md#NFR-REL-02]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### Debug Log References

N/A

### Completion Notes List

1. Created `HealthCheck` trait with async support using `async-trait` crate
2. Implemented `HealthStatus` enum with Online, Degraded, Offline states
3. Created `HealthCheckResult` with factory methods for each status
4. Created `IntegrationHealth` for aggregated state tracking
5. Implemented `HealthStore` with `tokio::sync::RwLock` for thread-safety
6. Added downtime alerting when offline > 2 minutes (NFR-REL-02)
7. Implemented health checks for Jira, Postman, and Testmo
8. Created `HealthScheduler` for background periodic checks (60s interval)
9. Added `GET /api/v1/health/integrations` endpoint with OpenAPI docs
10. All tests passing (50+ backend tests)

### File List

**Created:**
- `crates/qa-pms-core/src/health.rs` - HealthCheck trait and types
- `crates/qa-pms-core/src/health_store.rs` - Thread-safe health store
- `crates/qa-pms-jira/src/health.rs` - Jira health check
- `crates/qa-pms-postman/src/health.rs` - Postman health check
- `crates/qa-pms-testmo/src/health.rs` - Testmo health check
- `crates/qa-pms-api/src/health_scheduler.rs` - Background scheduler

**Modified:**
- `crates/qa-pms-core/src/lib.rs` - Export health modules
- `crates/qa-pms-core/Cargo.toml` - Add tokio dependency
- `crates/qa-pms-jira/src/lib.rs` - Export JiraHealthCheck
- `crates/qa-pms-postman/src/lib.rs` - Export PostmanHealthCheck
- `crates/qa-pms-testmo/src/lib.rs` - Export TestmoHealthCheck
- `crates/qa-pms-api/src/main.rs` - Add health_scheduler module
- `crates/qa-pms-api/src/app.rs` - Add HealthStore to AppState
- `crates/qa-pms-api/src/routes/health.rs` - Add integrations endpoint
- `crates/qa-pms-api/src/routes/mod.rs` - Update OpenAPI docs
- `crates/qa-pms-api/Cargo.toml` - Add dependencies
- `Cargo.toml` - Add futures to workspace
