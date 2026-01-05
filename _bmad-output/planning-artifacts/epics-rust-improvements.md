---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - _bmad-output/planning-artifacts/architecture.md
  - _bmad-output/planning-artifacts/prd.md
  - qa-intelligent-pms/Cargo.toml
  - qa-intelligent-pms/crates/qa-pms-api/src/app.rs
  - qa-intelligent-pms/crates/qa-pms-api/src/main.rs
workflowType: 'epics-and-stories'
project_name: 'estrategia preventiva-reativa'
date: '2026-01-07'
author: 'Daniel'
status: 'ready-for-implementation'
---

# Rust Improvements Epic - QA Intelligent PMS

## Overview

This document defines the epic and stories for improving the Rust implementation of QA Intelligent PMS, focusing on observability, performance, usability, and code quality.

## ⚠️ MANDATORY: Context7 Usage Requirement

**CRITICAL REQUIREMENT:** All implementations in Epic 14 **MUST** use Context7 MCP to:
1. **Look up library/API documentation** before writing any code
2. **Check code examples and best practices** from official documentation
3. **Verify correct syntax, method signatures, and patterns** from Context7
4. **Ensure up-to-date patterns** by querying Context7 for the specific library being used

**Process for each story:**
1. **First:** Use Context7 to resolve library ID (e.g., `/tokio-rs/axum`, `/websites/rs_moka`)
2. **Second:** Query Context7 for specific implementation patterns and examples
3. **Third:** Review best practices and common pitfalls from Context7 documentation
4. **Finally:** Implement the code using verified patterns from Context7

**Why this is mandatory:**
- Ensures we use the most current and recommended patterns
- Prevents anti-patterns and outdated code
- Guarantees we follow library-specific best practices
- Reduces bugs from incorrect API usage

## Epic 14: Rust Implementation Improvements

**Epic Goal:** Maximize Rust potential in the project by implementing production-grade observability, performance optimizations, and developer experience improvements.

**Business Value:** 
- Improved system reliability through better monitoring
- Faster response times through caching and optimization
- Easier debugging through distributed tracing
- Protection against abuse through rate limiting

**Dependencies:** 
- Epic 1 (Project Foundation) - Complete ✅
- All existing 13 epics - Complete ✅

**Priority:** P0 (Critical for Production Readiness)

---

## Stories

### Story 14.1: Graceful Shutdown and Signal Handling

**As a** DevOps engineer  
**I want** the server to handle shutdown signals gracefully  
**So that** in-flight requests complete before the server stops, preventing data corruption

**Priority:** P0  
**Estimated Effort:** 1 day  
**Sprint:** 1

#### Technical Requirements

1. Implement signal handling for SIGTERM and SIGINT (Ctrl+C)
2. Use `tokio::signal` for cross-platform signal handling
3. Integrate with Axum's `with_graceful_shutdown`
4. Allow configurable shutdown timeout (default: 30 seconds)
5. Log shutdown events with tracing

#### Acceptance Criteria

- [ ] Server responds to SIGTERM by initiating graceful shutdown
- [ ] Server responds to Ctrl+C (SIGINT) by initiating graceful shutdown
- [ ] In-flight requests complete before server stops
- [ ] New requests are rejected during shutdown (503 Service Unavailable)
- [ ] Shutdown completes within configured timeout
- [ ] Health scheduler stops cleanly
- [ ] Database connections are properly closed
- [ ] Logs clearly indicate shutdown progress

#### Context7 Requirements

**MANDATORY:** Before implementing, use Context7 to:
1. Resolve library ID: `/tokio-rs/tokio`
2. Query Context7 for: "Graceful shutdown with signal handling in Axum server"
3. Verify patterns for: `tokio::signal`, `axum::serve::with_graceful_shutdown`
4. Check best practices for: Health scheduler cleanup, database pool shutdown

#### Implementation Notes

```rust
// Reference from Tokio documentation via Context7
// Context7 library: /tokio-rs/tokio
use tokio::signal;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
```

#### Files to Modify

- `crates/qa-pms-api/src/main.rs` - Add shutdown signal handling
- `crates/qa-pms-api/src/app.rs` - Return shutdown handle for health scheduler

---

### Story 14.2: Request ID Middleware for Correlation

**As a** developer debugging production issues  
**I want** every request to have a unique correlation ID  
**So that** I can trace requests across logs and services

**Priority:** P0  
**Estimated Effort:** 0.5 days  
**Sprint:** 1

#### Technical Requirements

1. Create middleware that extracts or generates `x-request-id` header
2. Add request ID to tracing span context
3. Include request ID in all log entries for that request
4. Return request ID in response headers
5. Support accepting request ID from upstream proxies

#### Acceptance Criteria

- [ ] Every request has a unique UUID request ID
- [ ] Request ID is visible in all log entries for that request
- [ ] Request ID is returned in `x-request-id` response header
- [ ] Incoming `x-request-id` header is preserved if present
- [ ] Request ID is recorded in tracing spans
- [ ] No performance degradation (< 1ms overhead)

#### Context7 Requirements

**MANDATORY:** Before implementing, use Context7 to:
1. Resolve library ID: `/tokio-rs/axum`
2. Query Context7 for: "Request ID middleware with tracing span context in Axum"
3. Verify patterns for: Axum middleware creation, tracing span recording
4. Check best practices for: Header extraction, UUID generation, response modification

#### Implementation Notes

```rust
// Reference from Axum documentation via Context7
// Context7 library: /tokio-rs/axum
use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use tracing::Span;
use uuid::Uuid;

const REQUEST_ID_HEADER: &str = "x-request-id";

pub async fn request_id_middleware(mut request: Request, next: Next) -> Response {
    let request_id = request
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(ToString::to_string)
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    Span::current().record("request_id", &request_id);

    let mut response = next.run(request).await;
    response.headers_mut().insert(
        REQUEST_ID_HEADER,
        HeaderValue::from_str(&request_id).unwrap_or_default(),
    );

    response
}
```

#### Files to Create

- `crates/qa-pms-api/src/middleware/request_id.rs`

#### Files to Modify

- `crates/qa-pms-api/src/middleware/mod.rs`
- `crates/qa-pms-api/src/app.rs` - Add middleware layer

---

### Story 14.3: Prometheus Metrics Integration

**As a** SRE/DevOps engineer  
**I want** HTTP metrics exposed in Prometheus format  
**So that** I can monitor application health and performance in Grafana

**Priority:** P0  
**Estimated Effort:** 2 days  
**Sprint:** 2

#### Technical Requirements

1. Add `axum-prometheus` crate for metrics middleware
2. Expose `/metrics` endpoint for Prometheus scraping
3. Track standard HTTP metrics:
   - `http_requests_total` (counter)
   - `http_requests_duration_seconds` (histogram)
   - `http_requests_pending` (gauge)
4. Add custom business metrics:
   - `workflows_active` (gauge)
   - `workflows_completed_total` (counter)
   - `integration_health_status` (gauge per integration)
5. Configure histogram buckets for latency tracking
6. Exclude `/metrics` and `/health` from metrics collection

#### Acceptance Criteria

- [ ] `/metrics` endpoint returns Prometheus-formatted metrics
- [ ] HTTP request count tracked by method, path, status
- [ ] HTTP latency histogram with appropriate buckets
- [ ] Business metrics for workflows visible
- [ ] Integration health status exposed as metrics
- [ ] Health endpoints excluded from metrics
- [ ] Metrics endpoint protected (optional auth header)

#### Context7 Requirements

**MANDATORY:** Before implementing, use Context7 to:
1. Resolve library ID: `/ptrskay3/axum-prometheus`
2. Query Context7 for: "Prometheus metrics integration with axum-prometheus"
3. Verify patterns for: `PrometheusMetricLayerBuilder`, metrics endpoint setup
4. Check best practices for: Metric labels, histogram buckets, endpoint exclusion

#### Implementation Notes

```rust
// Reference from axum-prometheus documentation via Context7
// Context7 library: /ptrskay3/axum-prometheus
use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayerBuilder;

let (prometheus_layer, metric_handle) = PrometheusMetricLayerBuilder::new()
    .with_prefix("qa_pms")
    .with_ignore_patterns(&["/metrics", "/health", "/api/v1/health"])
    .build_pair();

let app = Router::new()
    .route("/metrics", get(|| async move { metric_handle.render() }))
    .layer(prometheus_layer);
```

#### Dependencies to Add (Cargo.toml)

```toml
axum-prometheus = "0.7"
```

#### Files to Modify

- `Cargo.toml` - Add workspace dependency
- `crates/qa-pms-api/Cargo.toml` - Add crate dependency
- `crates/qa-pms-api/src/app.rs` - Add metrics layer and endpoint
- `crates/qa-pms-api/src/routes/mod.rs` - Document metrics endpoint

---

### Story 14.4: In-Memory Cache Layer with Moka

**As a** user of the application  
**I want** frequently accessed data to be cached  
**So that** the application responds faster and reduces database load

**Priority:** P1  
**Estimated Effort:** 2 days  
**Sprint:** 3

#### Technical Requirements

1. Add `moka` crate for async in-memory caching
2. Create `AppCache` struct with typed caches:
   - Ticket cache (TTL: 5 minutes, max: 1000 entries)
   - Search results cache (TTL: 2 minutes, max: 500 entries)
   - Dashboard metrics cache (TTL: 30 seconds, max: 50 entries)
   - Workflow template cache (TTL: 10 minutes, max: 100 entries)
3. Implement cache invalidation on writes
4. Add cache statistics to metrics
5. Add `X-Cache-Status` header (HIT/MISS) to responses

#### Acceptance Criteria

- [ ] Repeated ticket fetches return cached data
- [ ] Cache respects TTL and evicts expired entries
- [ ] Write operations invalidate relevant cache entries
- [ ] Cache statistics exposed via `/metrics`
- [ ] Response headers indicate cache status
- [ ] Memory usage bounded by max capacity
- [ ] No stale data returned after writes

#### Context7 Requirements

**MANDATORY:** Before implementing, use Context7 to:
1. Resolve library ID: `/websites/rs_moka`
2. Query Context7 for: "Async cache with TTL and capacity limits using Moka"
3. Verify patterns for: `Cache::builder()`, `time_to_live`, `time_to_idle`, `max_capacity`
4. Check best practices for: Cache invalidation, statistics collection, memory management

#### Implementation Notes

```rust
// Reference from Moka documentation via Context7
// Context7 library: /websites/rs_moka
use moka::future::Cache;
use std::time::Duration;
use std::sync::Arc;

pub struct AppCache {
    pub tickets: Cache<String, JiraTicket>,
    pub search: Cache<String, Vec<SearchResult>>,
    pub metrics: Cache<String, DashboardMetrics>,
    pub templates: Cache<uuid::Uuid, WorkflowTemplate>,
}

impl AppCache {
    pub fn new() -> Self {
        Self {
            tickets: Cache::builder()
                .max_capacity(1_000)
                .time_to_live(Duration::from_secs(300))
                .time_to_idle(Duration::from_secs(60))
                .build(),
            search: Cache::builder()
                .max_capacity(500)
                .time_to_live(Duration::from_secs(120))
                .build(),
            metrics: Cache::builder()
                .max_capacity(50)
                .time_to_live(Duration::from_secs(30))
                .build(),
            templates: Cache::builder()
                .max_capacity(100)
                .time_to_live(Duration::from_secs(600))
                .build(),
        }
    }
}
```

#### Dependencies to Add (Cargo.toml)

```toml
moka = { version = "0.12", features = ["future"] }
```

#### Files to Create

- `crates/qa-pms-core/src/cache.rs`

#### Files to Modify

- `Cargo.toml` - Add workspace dependency
- `crates/qa-pms-core/Cargo.toml` - Add crate dependency
- `crates/qa-pms-core/src/lib.rs` - Export cache module
- `crates/qa-pms-api/src/app.rs` - Add cache to AppState
- `crates/qa-pms-api/src/routes/tickets.rs` - Use cache for ticket fetches
- `crates/qa-pms-api/src/routes/search.rs` - Use cache for search results

---

### Story 14.5: Rate Limiting with Tower Governor

**As a** system administrator  
**I want** API rate limiting per client IP  
**So that** the system is protected from abuse and DoS attacks

**Priority:** P1  
**Estimated Effort:** 1 day  
**Sprint:** 4

#### Technical Requirements

1. Add `tower-governor` crate for rate limiting
2. Configure rate limiting:
   - Default: 100 requests/minute per IP
   - Burst: Allow 20 request burst
   - Use `SmartIpKeyExtractor` for proxy support
3. Return `429 Too Many Requests` when limit exceeded
4. Include `Retry-After` header in 429 responses
5. Add rate limit headers to all responses:
   - `X-RateLimit-Limit`
   - `X-RateLimit-Remaining`
   - `X-RateLimit-Reset`
6. Exclude health/metrics endpoints from rate limiting
7. Background task to clean up rate limiter storage

#### Acceptance Criteria

- [ ] Requests exceeding rate limit receive 429 status
- [ ] Rate limit headers present in all responses
- [ ] Retry-After header indicates when to retry
- [ ] Health/metrics endpoints not rate limited
- [ ] Rate limiter works behind reverse proxy
- [ ] Memory cleanup prevents unbounded growth
- [ ] Configurable limits via environment variables

#### Context7 Requirements

**MANDATORY:** Before implementing, use Context7 to:
1. Resolve library ID: `/benwis/tower-governor`
2. Query Context7 for: "Rate limiting with tower-governor and SmartIpKeyExtractor"
3. Verify patterns for: `GovernorConfigBuilder`, `per_second`, `burst_size`, proxy support
4. Check best practices for: Rate limit headers, Retry-After, cleanup tasks

#### Implementation Notes

```rust
// Reference from tower-governor documentation via Context7
// Context7 library: /benwis/tower-governor
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer, key_extractor::SmartIpKeyExtractor};
use std::sync::Arc;
use std::time::Duration;

let governor_conf = Arc::new(
    GovernorConfigBuilder::default()
        .per_second(2)  // Replenish rate
        .burst_size(20)
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .unwrap(),
);

// Background cleanup
let governor_limiter = governor_conf.limiter().clone();
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(60));
    loop {
        interval.tick().await;
        governor_limiter.retain_recent();
    }
});

let app = Router::new()
    .layer(GovernorLayer::new(governor_conf));

// Important: Use into_make_service_with_connect_info for IP extraction
axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
```

#### Dependencies to Add (Cargo.toml)

```toml
tower-governor = "0.4"
```

#### Files to Modify

- `Cargo.toml` - Add workspace dependency
- `crates/qa-pms-api/Cargo.toml` - Add crate dependency
- `crates/qa-pms-api/src/app.rs` - Add rate limiting layer
- `crates/qa-pms-api/src/main.rs` - Use `into_make_service_with_connect_info`

---

### Story 14.6: OpenTelemetry Distributed Tracing

**As a** developer troubleshooting distributed systems  
**I want** traces exported to OpenTelemetry-compatible backends  
**So that** I can visualize request flows across services

**Priority:** P1  
**Estimated Effort:** 2 days  
**Sprint:** 5

#### Technical Requirements

1. Add `tracing-opentelemetry` and `opentelemetry-otlp` crates
2. Configure OTLP exporter for traces
3. Set up proper resource attributes (service name, version, environment)
4. Enable context propagation for distributed tracing
5. Configure sampling (default: 100% in dev, 10% in prod)
6. Graceful shutdown of tracer provider
7. Support configurable OTLP endpoint via environment variable

#### Acceptance Criteria

- [ ] Traces exported to configured OTLP endpoint
- [ ] Service name and version visible in traces
- [ ] Parent-child span relationships correct
- [ ] Context propagated via HTTP headers
- [ ] Sampling rate configurable
- [ ] Tracer shuts down cleanly on app exit
- [ ] Works with Jaeger, Zipkin, and cloud providers

#### Context7 Requirements

**MANDATORY:** Before implementing, use Context7 to:
1. Resolve library ID: `/tokio-rs/tracing-opentelemetry`
2. Query Context7 for: "OpenTelemetry OTLP exporter setup with tracing"
3. Verify patterns for: `SdkTracerProvider`, OTLP exporter, context propagation
4. Check best practices for: Sampling strategies, resource attributes, graceful shutdown

#### Implementation Notes

```rust
// Reference from tracing-opentelemetry documentation via Context7
// Context7 library: /tokio-rs/tracing-opentelemetry
use opentelemetry::{global, trace::TracerProvider as _};
use opentelemetry_sdk::{
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
    Resource,
};
use opentelemetry_otlp::SpanExporter;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, Registry};

fn init_tracer_provider() -> SdkTracerProvider {
    let exporter = SpanExporter::builder()
        .with_tonic()
        .build()
        .unwrap();

    SdkTracerProvider::builder()
        .with_sampler(Sampler::ParentBased(Box::new(
            Sampler::TraceIdRatioBased(1.0)
        )))
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(Resource::builder()
            .with_service_name(env!("CARGO_PKG_NAME"))
            .build())
        .with_batch_exporter(exporter)
        .build()
}

fn init_tracing() -> impl Drop {
    let tracer_provider = init_tracer_provider();
    let tracer = tracer_provider.tracer("qa-pms-api");

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    tracer_provider // Return for cleanup on drop
}
```

#### Dependencies to Add (Cargo.toml)

```toml
tracing-opentelemetry = "0.22"
opentelemetry = { version = "0.21", features = ["rt-tokio"] }
opentelemetry-otlp = { version = "0.14", features = ["tonic"] }
opentelemetry_sdk = { version = "0.21", features = ["rt-tokio"] }
```

#### Files to Create

- `crates/qa-pms-api/src/telemetry.rs`

#### Files to Modify

- `Cargo.toml` - Add workspace dependencies
- `crates/qa-pms-api/Cargo.toml` - Add crate dependencies
- `crates/qa-pms-api/src/main.rs` - Initialize telemetry

---

### Story 14.7: CLI Admin Tool

**As a** system administrator  
**I want** a CLI tool for administrative tasks  
**So that** I can manage the application without using the API directly

**Priority:** P2  
**Estimated Effort:** 2 days  
**Sprint:** 6

#### Technical Requirements

1. Add `clap` crate for CLI argument parsing
2. Implement subcommands:
   - `serve` - Start the API server (default)
   - `migrate run` - Run pending database migrations
   - `migrate status` - Show migration status
   - `health` - Check integration health
   - `config validate` - Validate configuration
   - `config show` - Show masked configuration
   - `config generate-key` - Generate encryption key
3. Support `--config` flag for custom config file
4. Colorized output with `--no-color` option
5. JSON output option for scripting

#### Acceptance Criteria

- [ ] `qa-pms serve` starts the server (current behavior)
- [ ] `qa-pms migrate run` runs migrations successfully
- [ ] `qa-pms migrate status` shows migration state
- [ ] `qa-pms health` checks all integrations
- [ ] `qa-pms config validate` validates without starting server
- [ ] `qa-pms config generate-key` outputs valid 256-bit hex key
- [ ] `--help` shows comprehensive help for all commands
- [ ] Exit codes indicate success/failure appropriately

#### Context7 Requirements

**MANDATORY:** Before implementing, use Context7 to:
1. Resolve library ID: `/clap-rs/clap`
2. Query Context7 for: "CLI argument parsing with clap derive API"
3. Verify patterns for: `#[derive(Parser)]`, `#[command(subcommand)]`, nested commands
4. Check best practices for: Global flags, environment variable parsing, output formatting

#### Implementation Notes

```rust
// Reference from clap documentation via Context7
// Context7 library: /clap-rs/clap
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "qa-pms")]
#[command(about = "QA Intelligent PMS CLI Tool")]
#[command(version)]
pub struct Cli {
    /// Path to configuration file
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Output format (text, json)
    #[arg(short, long, global = true, default_value = "text")]
    pub output: OutputFormat,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the API server
    Serve {
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
    /// Database migration commands
    Migrate {
        #[command(subcommand)]
        action: MigrateAction,
    },
    /// Check integration health
    Health {
        /// Specific integration to check
        #[arg(short, long)]
        integration: Option<String>,
    },
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}
```

#### Dependencies to Add (Cargo.toml)

```toml
clap = { version = "4.4", features = ["derive", "env"] }
```

#### Files to Create

- `crates/qa-pms-api/src/cli.rs`
- `crates/qa-pms-api/src/commands/mod.rs`
- `crates/qa-pms-api/src/commands/migrate.rs`
- `crates/qa-pms-api/src/commands/health.rs`
- `crates/qa-pms-api/src/commands/config.rs`

#### Files to Modify

- `crates/qa-pms-api/Cargo.toml` - Add clap dependency
- `crates/qa-pms-api/src/main.rs` - Parse CLI args and route to commands

---

### Story 14.8: Integration Tests with Testcontainers

**As a** developer  
**I want** automated integration tests using real database containers  
**So that** I can validate the full system behavior before deployment

**Priority:** P2  
**Estimated Effort:** 3 days  
**Sprint:** 7

#### Technical Requirements

1. Add `testcontainers` crate for container management
2. Create test fixtures for common scenarios
3. Implement integration tests for:
   - Full workflow lifecycle (create → steps → complete)
   - Time tracking accuracy
   - Pattern detection triggers
   - Report generation
   - Dashboard metrics calculation
4. Use PostgreSQL container for tests
5. Automatic cleanup after tests
6. Parallel test execution where safe

#### Acceptance Criteria

- [ ] Integration tests use real PostgreSQL container
- [ ] Workflow lifecycle test passes end-to-end
- [ ] Time tracking tests verify accuracy
- [ ] Pattern detection tests verify alerts
- [ ] Tests run in CI pipeline
- [ ] Tests clean up containers after execution
- [ ] Test execution time < 5 minutes

#### Context7 Requirements

**MANDATORY:** Before implementing, use Context7 to:
1. Resolve library ID: `/testcontainers/testcontainers-rs`
2. Query Context7 for: "PostgreSQL testcontainers setup with async/await"
3. Verify patterns for: Container lifecycle, port mapping, cleanup patterns
4. Check best practices for: Test fixtures, parallel execution, CI/CD integration

#### Implementation Notes

```rust
// Reference from testcontainers documentation via Context7
// Context7 library: /testcontainers/testcontainers-rs
use testcontainers::{clients::Cli, images::postgres::Postgres};
use sqlx::PgPool;

async fn setup_test_db() -> (PgPool, impl Drop) {
    let docker = Cli::default();
    let postgres = docker.run(Postgres::default());
    
    let db_url = format!(
        "postgres://postgres:postgres@localhost:{}/postgres",
        postgres.get_host_port_ipv4(5432)
    );
    
    let pool = PgPool::connect(&db_url).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    
    (pool, postgres)
}

#[tokio::test]
async fn test_workflow_lifecycle() {
    let (pool, _container) = setup_test_db().await;
    
    // Create workflow
    let workflow_id = create_workflow(&pool, "TEST-123", "standard").await.unwrap();
    
    // Complete steps
    for step in 1..=5 {
        complete_step(&pool, workflow_id, step, "Test notes").await.unwrap();
    }
    
    // Verify completion
    let workflow = get_workflow(&pool, workflow_id).await.unwrap();
    assert_eq!(workflow.status, "completed");
}
```

#### Dependencies to Add (Cargo.toml - dev)

```toml
[dev-dependencies]
testcontainers = "0.15"
```

#### Files to Create

- `crates/qa-pms-api/tests/common/mod.rs`
- `crates/qa-pms-api/tests/common/fixtures.rs`
- `crates/qa-pms-api/tests/integration/workflow_test.rs`
- `crates/qa-pms-api/tests/integration/time_tracking_test.rs`
- `crates/qa-pms-api/tests/integration/pattern_detection_test.rs`

---

## Sprint Summary

| Sprint | Stories | Focus Area | Days |
|--------|---------|------------|------|
| 1 | 14.1, 14.2 | Reliability & Debugging | 1.5 |
| 2 | 14.3 | Observability - Metrics | 2 |
| 3 | 14.4 | Performance - Caching | 2 |
| 4 | 14.5 | Security - Rate Limiting | 1 |
| 5 | 14.6 | Observability - Tracing | 2 |
| 6 | 14.7 | Usability - CLI | 2 |
| 7 | 14.8 | Quality - Integration Tests | 3 |

**Total Estimated Effort:** 13.5 days

---

## Dependencies Summary

### New Crate Dependencies

```toml
[workspace.dependencies]
# Observability
axum-prometheus = "0.7"
tracing-opentelemetry = "0.22"
opentelemetry = { version = "0.21", features = ["rt-tokio"] }
opentelemetry-otlp = { version = "0.14", features = ["tonic"] }
opentelemetry_sdk = { version = "0.21", features = ["rt-tokio"] }

# Performance
moka = { version = "0.12", features = ["future"] }

# Security
tower-governor = "0.4"

# CLI
clap = { version = "4.4", features = ["derive", "env"] }

# Testing (dev)
testcontainers = "0.15"
```

---

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| OpenTelemetry complexity | Medium | Medium | Start with basic setup, add features incrementally |
| Cache invalidation bugs | Medium | High | Comprehensive test coverage, conservative TTLs |
| Rate limiting false positives | Low | Medium | Start with generous limits, monitor and adjust |
| Testcontainers CI issues | Medium | Low | Fallback to mock-based tests if containers fail |

---

## Success Metrics

1. **Observability:** 100% of requests have correlation IDs in logs
2. **Performance:** P95 latency reduced by 30% through caching
3. **Reliability:** Zero dropped requests during graceful shutdown
4. **Security:** Rate limiting blocks 100% of requests exceeding threshold
5. **Quality:** Integration test coverage > 80% for critical paths