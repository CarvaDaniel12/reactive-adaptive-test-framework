# Story 14.6: OpenTelemetry Distributed Tracing

**As a** developer troubleshooting distributed systems  
**I want** traces exported to OpenTelemetry-compatible backends  
**So that** I can visualize request flows across services

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 14.6 |
| Epic | Rust Implementation Improvements |
| Sprint | 5 - Observability - Tracing |
| Priority | P1 |
| Estimated Days | 2 |
| Dependencies | 14.2 (Request ID Middleware), 14.3 (Prometheus Metrics Integration) |
| Status | ready-for-dev |

---

## Technical Requirements

### 1. Add `tracing-opentelemetry` and `opentelemetry-otlp` crates

- Integrate OpenTelemetry with the existing tracing infrastructure
- Use OTLP (OpenTelemetry Protocol) for trace export
- Support both gRPC and HTTP transport options

### 2. Configure OTLP exporter for traces

- Export traces to configurable OTLP endpoint (default: localhost:4317)
- Support TLS/SSL connections for secure transport
- Implement batch export for efficiency
- Handle export failures gracefully with retries

### 3. Set up proper resource attributes

- Configure service name: `qa-pms-api`
- Add service version from Cargo.toml
- Include environment: `development` / `staging` / `production`
- Add deployment information when available

### 4. Enable context propagation for distributed tracing

- Propagate trace context via HTTP headers (`traceparent`, `tracestate`)
- Extract incoming trace context from requests
- Inject trace context into outgoing HTTP requests
- Maintain span context through async boundaries

### 5. Configure sampling (default: 100% in dev, 10% in prod)

- Implement parent-based sampling strategy
- Allow configuration via environment variable
- Support trace ID ratio-based sampling
- Always sample error spans regardless of sampling rate

### 6. Graceful shutdown of tracer provider

- Flush pending traces on shutdown
- Timeout shutdown (default: 5 seconds)
- Integrate with graceful shutdown from Story 14.1
- Ensure no traces are lost during shutdown

### 7. Support configurable OTLP endpoint via environment variable

- `OTEL_EXPORTER_OTLP_ENDPOINT` - OTLP endpoint address
- `OTEL_EXPORTER_OTLP_PROTOCOL` - Protocol (grpc/http)
- `OTEL_SERVICE_NAME` - Override service name
- `OTEL_SERVICE_VERSION` - Override service version
- `OTEL_SAMPLING_RATIO` - Sampling ratio (0.0 to 1.0)
- `OTEL_ENVIRONMENT` - Environment identifier

---

## Acceptance Criteria

- [ ] Traces exported to configured OTLP endpoint
- [ ] Service name and version visible in traces
- [ ] Parent-child span relationships correct
- [ ] Context propagated via HTTP headers
- [ ] Sampling rate configurable and working
- [ ] Tracer shuts down cleanly on app exit
- [ ] Works with Jaeger, Zipkin, and cloud providers (GCP Cloud Trace, AWS X-Ray, Azure Monitor)
- [ ] Request ID from Story 14.2 included in span attributes
- [ ] HTTP endpoints automatically instrumented with spans
- [ ] Database queries automatically traced
- [ ] External API calls traced with proper propagation

---

## Implementation Notes

### Installation

Add OpenTelemetry dependencies to workspace:

```toml
# Cargo.toml

[workspace.dependencies]
tracing-opentelemetry = "0.22"
opentelemetry = { version = "0.21", features = ["rt-tokio"] }
opentelemetry-otlp = { version = "0.14", features = ["tonic"] }
opentelemetry_sdk = { version = "0.21", features = ["rt-tokio"] }
opentelemetry-http = "0.9"
```

Add to API crate:

```toml
# crates/qa-pms-api/Cargo.toml

[dependencies]
tracing-opentelemetry = { workspace = true }
opentelemetry = { workspace = true }
opentelemetry-otlp = { workspace = true }
opentelemetry_sdk = { workspace = true }
opentelemetry-http = {0.9}
```

### Telemetry Module

```rust
// crates/qa-pms-api/src/telemetry.rs

use opentelemetry::{
    global,
    trace::{TraceError, TracerProvider as _},
    KeyValue,
};
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_sdk::{
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
    Resource,
    runtime::Tokio,
};
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing::Subscriber;

/// Telemetry configuration
#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    /// OTLP endpoint address
    pub otlp_endpoint: String,
    /// Export protocol (grpc or http)
    pub protocol: String,
    /// Service name
    pub service_name: String,
    /// Service version
    pub service_version: String,
    /// Environment
    pub environment: String,
    /// Sampling ratio (0.0 to 1.0)
    pub sampling_ratio: f64,
    /// Whether OpenTelemetry is enabled
    pub enabled: bool,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            otlp_endpoint: env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:4317".to_string()),
            protocol: env::var("OTEL_EXPORTER_OTLP_PROTOCOL")
                .unwrap_or_else(|_| "grpc".to_string()),
            service_name: env::var("OTEL_SERVICE_NAME")
                .unwrap_or_else(|_| "qa-pms-api".to_string()),
            service_version: env::var("CARGO_PKG_VERSION")
                .unwrap_or_else(|_| env!("CARGO_PKG_VERSION").to_string()),
            environment: env::var("OTEL_ENVIRONMENT")
                .unwrap_or_else(|_| "development".to_string()),
            sampling_ratio: env::var("OTEL_SAMPLING_RATIO")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or_else(|| match env::var("OTEL_ENVIRONMENT").as_deref() {
                    Ok("production") => 0.1,
                    _ => 1.0,
                }),
            enabled: env::var("OTEL_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
        }
    }
}

/// Initialize tracer provider
pub fn init_tracer_provider(config: &TelemetryConfig) -> Result<SdkTracerProvider, TraceError> {
    // Build resource attributes
    let resource = Resource::builder()
        .with_service_name(&config.service_name)
        .with(KeyValue::new("service.version", config.service_version.clone()))
        .with(KeyValue::new("deployment.environment", config.environment.clone()))
        .with(KeyValue::new("telemetry.sdk.name", "opentelemetry"))
        .with(KeyValue::new("telemetry.sdk.language", "rust"))
        .with(KeyValue::new("telemetry.sdk.version", env!("CARGO_PKG_VERSION")))
        .build();

    // Create exporter based on protocol
    let exporter = if config.protocol == "http" {
        // HTTP exporter
        let endpoint = if config.otlp_endpoint.contains("/v1/traces") {
            config.otlp_endpoint.clone()
        } else {
            format!("{}/v1/traces", config.otlp_endpoint.trim_end_matches('/'))
        };
        
        opentelemetry_otlp::new_exporter()
            .http()
            .with_endpoint(endpoint)
            .build()?
    } else {
        // gRPC exporter (default)
        let endpoint = if config.otlp_endpoint.contains(":4317") {
            config.otlp_endpoint.clone()
        } else {
            config.otlp_endpoint.trim_end_matches('/').to_string()
        };
        
        opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(endpoint)
            .build()?
    };

    // Configure sampling
    let sampler = Sampler::ParentBased(Box::new(
        Sampler::TraceIdRatioBased(config.sampling_ratio)
    ));

    // Build tracer provider
    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter, Tokio)
        .with_sampler(sampler)
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(resource)
        .build();

    // Set as global tracer provider
    global::set_tracer_provider(provider.clone());

    Ok(provider)
}

/// Initialize tracing with OpenTelemetry
pub fn init_tracing(config: &TelemetryConfig) -> Result<Option<SdkTracerProvider>, TraceError> {
    if !config.enabled {
        // Initialize basic tracing without OpenTelemetry
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));

        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .init();

        return Ok(None);
    }

    // Initialize OpenTelemetry
    let tracer_provider = init_tracer_provider(config)?;
    let tracer = tracer_provider.tracer(&config.service_name);

    // Create OpenTelemetry layer
    let telemetry_layer = tracing_opentelemetry::layer()
        .with_tracer(tracer);

    // Create filter from environment
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn,tower_http=warn"));

    // Initialize subscriber with multiple layers
    tracing_subscriber::registry()
        .with(filter)
        .with(telemetry_layer)
        .with(tracing_subscriber::fmt::layer())
        .init();

    Ok(Some(tracer_provider))
}

/// Shutdown tracing gracefully
pub async fn shutdown_tracing(provider: Option<SdkTracerProvider>) {
    if let Some(provider) = provider {
        tracing::info!("Shutting down OpenTelemetry tracer provider...");
        
        tokio::time::timeout(
            tokio::time::Duration::from_secs(5),
            provider.shutdown(),
        )
        .await
        .ok();
        
        global::shutdown_tracer_provider();
        tracing::info!("OpenTelemetry tracer provider shut down");
    }
}

/// Extract trace context from HTTP headers
pub fn extract_trace_context(headers: &http::HeaderMap) -> Option<opentelemetry::Context> {
    use opentelemetry::propagation::Extractor;
    use opentelemetry_http::HeaderExtractor;

    let propagator = opentelemetry::propagation::TraceContextPropagator::new();
    let context = propagator.extract(HeaderExtractor(headers));
    Some(context)
}

/// Inject trace context into HTTP headers
pub fn inject_trace_context(context: &opentelemetry::Context, headers: &mut http::HeaderMap) {
    use opentelemetry::propagation::Injector;
    use opentelemetry_http::HeaderInjector;

    let propagator = opentelemetry::propagation::TraceContextPropagator::new();
    let mut header_injector = HeaderInjector(headers);
    propagator.inject_context(context, &mut header_injector);
}
```

### HTTP Request Tracing Middleware

```rust
// crates/qa-pms-api/src/middleware/tracing.rs

use axum::{
    extract::Request,
    http::{HeaderMap, HeaderName, HeaderValue},
    middleware::Next,
    response::Response,
};
use tracing::{info_span, Level, Span};
use std::time::Instant;

/// HTTP request span name
const HTTP_SPAN_NAME: &str = "http.request";

/// Middleware to trace HTTP requests
pub async fn tracing_middleware(request: Request, next: Next) -> Response {
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let version = format!("{:?}", request.version());
    
    // Extract request ID from headers (set by Story 14.2)
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    // Create span with relevant attributes
    let span = info_span!(
        HTTP_SPAN_NAME,
        http.method = %method,
        http.target = %path,
        http.version = %version,
        http.request_id = %request_id,
        otel.kind = "server",
        otel.status_code = tracing::field::Empty,
    );

    // Start timer for duration
    let start = Instant::now();

    // Execute request with span context
    let response = async {
        let span = Span::current();
        
        // Record user agent
        if let Some(user_agent) = request.headers().get("user-agent")
            .and_then(|v| v.to_str().ok())
        {
            span.record("http.user_agent", user_agent);
        }

        // Record client IP (if available)
        if let Some(forwarded_for) = request.headers().get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
        {
            span.record("http.client_ip", forwarded_for);
        }

        next.run(request).await
    }
    .instrument(span)
    .await;

    // Calculate duration
    let duration = start.elapsed();
    let status = response.status();
    let status_code = status.as_u16();

    // Record status and duration
    Span::current().record("http.status_code", status_code);
    Span::current().record("http.flavor", &version);
    Span::current().record("otel.status_code", status_code);

    // Add duration as attribute
    if duration.as_secs_f64() > 0.0 {
        Span::current().record(
            "http.request_duration_ms",
            duration.as_secs_f64() * 1000.0
        );
    }

    // Set span status based on HTTP status
    if status.is_client_error() {
        Span::current().record("otel.status", "error");
    } else if status.is_server_error() {
        Span::current().record("otel.status", "error");
    }

    response
}
```

### Database Query Tracing

```rust
// crates/qa-pms-api/src/db/tracing.rs

use sqlx::postgres::PgPoolOptions;
use tracing::info_span;

/// Create database pool with tracing enabled
pub async fn create_pool(database_url: &str) -> Result<sqlx::PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    // SQLx automatically integrates with tracing
    // Queries will be traced with the following spans:
    // - sqlx::query (for each query)
    // - sqlx::prepare (for prepared statements)
    // - sqlx::execute (for execution)
    
    Ok(pool)
}

/// Execute query with custom span
pub async fn execute_traced<F, R>(
    pool: &sqlx::PgPool,
    query: &str,
    operation: F,
) -> Result<R, sqlx::Error>
where
    F: FnOnce(&sqlx::PgPool) -> futures::future::BoxFuture<'_, Result<R, sqlx::Error>>,
{
    let _span = info_span!(
        "database.query",
        db.system = "postgresql",
        db.name = "qa_pms",
        db.statement = %query,
        db.operation = "execute",
    );

    operation(pool).await
}
```

### Update Main to Initialize Tracing

```rust
// crates/qa-pms-api/src/main.rs

use crate::telemetry::{TelemetryConfig, init_tracing, shutdown_tracing};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize telemetry
    let telemetry_config = TelemetryConfig::default();
    let tracer_provider = init_tracing(&telemetry_config)
        .expect("Failed to initialize tracing");

    tracing::info!(
        service = %telemetry_config.service_name,
        version = %telemetry_config.service_version,
        environment = %telemetry_config.environment,
        otlp_endpoint = %telemetry_config.otlp_endpoint,
        sampling_ratio = telemetry_config.sampling_ratio,
        "QA Intelligent PMS API starting"
    );

    // ... rest of initialization (settings, database, etc.) ...

    // ... create app, listener ...

    info!("Starting server on http://{}", addr);

    // Run server with graceful shutdown
    let shutdown_handle = async {
        shutdown_signal().await?;
        Ok::<_, anyhow::Error>(())
    };

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_handle)
        .await?;

    // Shutdown tracing
    shutdown_tracing(tracer_provider).await;

    info!("Server shut down gracefully");
    Ok(())
}
```

### Add Tracing Middleware to App

```rust
// crates/qa-pms-api/src/app.rs

use crate::middleware::tracing::tracing_middleware;

pub async fn create_app(settings: Settings) -> Result<Router> {
    // ... existing setup ...

    let app = Router::new()
        // ... existing routes ...
        .layer(
            tower::ServiceBuilder::new()
                .layer(prometheus_layer)
                // HTTP request tracing
                .layer(tracing_middleware)
                // Request ID middleware (must come before tracing to include request_id)
                .layer(request_id_middleware())
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any)
                ),
        );

    Ok(app)
}
```

---

## Dependencies to Add

### Workspace Dependencies

```toml
# Cargo.toml

[workspace.dependencies]
tracing-opentelemetry = "0.22"
opentelemetry = { version = "0.21", features = ["rt-tokio"] }
opentelemetry-otlp = { version = "0.14", features = ["tonic"] }
opentelemetry_sdk = { version = "0.21", features = ["rt-tokio"] }
opentelemetry-http = "0.9"
```

### Crate Dependencies

```toml
# crates/qa-pms-api/Cargo.toml

[dependencies]
tracing-opentelemetry = { workspace = true }
opentelemetry = { workspace = true }
opentelemetry-otlp = { workspace = true }
opentelemetry_sdk = { workspace = true }
opentelemetry-http = { workspace = true }
```

---

## Files to Create

| File | Description |
|------|-------------|
| `crates/qa-pms-api/src/telemetry.rs` | Telemetry initialization and configuration |
| `crates/qa-pms-api/src/middleware/tracing.rs` | HTTP request tracing middleware |
| `crates/qa-pms-api/src/db/tracing.rs` | Database query tracing utilities |

---

## Files to Modify

| File | Type | Changes |
|-------|------|---------|
| `Cargo.toml` | Modify | Add OpenTelemetry workspace dependencies |
| `crates/qa-pms-api/Cargo.toml` | Modify | Add OpenTelemetry crate dependencies |
| `crates/qa-pms-api/src/main.rs` | Modify | Initialize telemetry on startup, shutdown on exit |
| `crates/qa-pms-api/src/app.rs` | Modify | Add tracing middleware to router |
| `crates/qa-pms-api/src/middleware/mod.rs` | Modify | Export tracing middleware |

---

## Testing Strategy

### Unit Tests for Telemetry Configuration

```rust
// crates/qa-pms-api/tests/telemetry_config_test.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_config_defaults() {
        let config = TelemetryConfig::default();
        
        assert_eq!(config.service_name, "qa-pms-api");
        assert_eq!(config.environment, "development");
        assert_eq!(config.sampling_ratio, 1.0);
        assert!(config.enabled);
    }

    #[test]
    fn test_telemetry_config_from_env() {
        std::env::set_var("OTEL_SERVICE_NAME", "test-service");
        std::env::set_var("OTEL_ENVIRONMENT", "production");
        std::env::set_var("OTEL_SAMPLING_RATIO", "0.5");
        std::env::set_var("OTEL_ENABLED", "false");
        
        let config = TelemetryConfig::default();
        
        assert_eq!(config.service_name, "test-service");
        assert_eq!(config.environment, "production");
        assert_eq!(config.sampling_ratio, 0.5);
        assert!(!config.enabled);
        
        // Cleanup
        std::env::remove_var("OTEL_SERVICE_NAME");
        std::env::remove_var("OTEL_ENVIRONMENT");
        std::env::remove_var("OTEL_SAMPLING_RATIO");
        std::env::remove_var("OTEL_ENABLED");
    }

    #[test]
    fn test_production_sampling() {
        std::env::set_var("OTEL_ENVIRONMENT", "production");
        
        let config = TelemetryConfig::default();
        
        // Production should have 10% sampling by default
        assert_eq!(config.sampling_ratio, 0.1);
        
        std::env::remove_var("OTEL_ENVIRONMENT");
    }
}
```

### Integration Tests with Jaeger

```rust
// crates/qa-pms-api/tests/integration/tracing_test.rs

use reqwest::StatusCode;

#[tokio::test]
async fn test_http_request_creates_span() {
    let addr = start_test_server().await;
    let client = reqwest::Client::new();
    
    // Make a request
    let response = client
        .get(format!("http://{}/api/v1/health", addr))
        .header("x-request-id", "test-request-123")
        .send()
        .await
        .expect("Failed to send request");
    
    assert_eq!(response.status(), StatusCode::OK);
    
    // Note: In a real test, we would verify that the trace
    // was exported to the OTLP endpoint (e.g., Jaeger)
    // by querying the Jaeger API
}

#[tokio::test]
async fn test_trace_context_propagation() {
    let addr = start_test_server().await;
    let client = reqwest::Client::new();
    
    // Send request with trace context headers
    let response = client
        .get(format!("http://{}/api/v1/health", addr))
        .header("traceparent", "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01")
        .send()
        .await
        .expect("Failed to send request");
    
    assert_eq!(response.status(), StatusCode::OK);
    
    // Verify trace context is returned (if the endpoint supports it)
    // This tests that we're properly extracting and propagating context
}
```

### Manual Testing with Jaeger

1. Start Jaeger in Docker:
```bash
docker run -d --name jaeger \
  -e COLLECTOR_OTLP_ENABLED=true \
  -p 4317:4317 \
  -p 4318:4318 \
  -p 16686:16686 \
  jaegertracing/all-in-one:latest
```

2. Configure the application:
```bash
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
export OTEL_SERVICE_NAME=qa-pms-api-test
export OTEL_ENVIRONMENT=development
export OTEL_SAMPLING_RATIO=1.0
```

3. Run the application and make HTTP requests

4. Open Jaeger UI at `http://localhost:16686`

5. Search for traces by service name and verify:
   - Service name is correct
   - Spans are present with correct hierarchy
   - HTTP request spans have proper attributes (method, path, status)
   - Database query spans are visible
   - Request ID is included as span attribute

---

## Success Metrics

- **Trace Export**: Traces successfully exported to OTLP endpoint
- **Service Identification**: Service name and version visible in trace metadata
- **Span Hierarchy**: Parent-child relationships correctly established
- **Context Propagation**: Trace context propagated across service boundaries
- **Sampling**: Configurable sampling rate working correctly
- **Shutdown**: Tracer provider flushes pending traces before exit
- **Performance**: < 5ms overhead from tracing instrumentation
- **Coverage**: All HTTP endpoints, database queries, and external API calls traced

---

## Context and Dependencies

This story depends on:
- **Story 14.1**: Graceful shutdown to flush traces before shutdown
- **Story 14.2**: Request ID middleware to correlate traces with logs
- **Story 14.3**: Prometheus metrics to complement traces for full observability

This story enables:
- **Story 14.7**: CLI can include tracing management commands
- **Story 14.8**: Integration tests can verify trace behavior

---

## Risks and Mitigation

| Risk | Probability | Impact | Mitigation |
|-------|-------------|--------|------------|
| High trace volume cost | Medium | High | Implement appropriate sampling (10% in production) |
| OTLP endpoint failures | Low | Medium | Implement retries and batch export with timeout |
| Performance overhead | Low | Medium | Measure overhead, optimize if needed |
| Context propagation errors | Low | Medium | Test with Jaeger/Tempo before production |

---

## Next Steps

After this story is complete:
1. Set up Jaeger or Tempo for trace visualization
2. Create trace-based dashboards
3. Implement trace-based alerting (e.g., high error rate in traces)
4. Consider adding custom spans for business operations
5. Proceed to Story 14.7 (CLI Admin Tool)

---

## Operational Guidelines

### Supported Backends

The OpenTelemetry OTLP exporter works with:
- **Jaeger**: Self-hosted or Jaeger Cloud
- **Grafana Tempo**: Self-hosted or Grafana Cloud
- **GCP Cloud Trace**: Via OpenTelemetry Collector
- **AWS X-Ray**: Via OpenTelemetry Collector
- **Azure Monitor**: Via OpenTelemetry Collector
- **Datadog**: Via OpenTelemetry Collector
- **Honeycomb**: Direct OTLP support

### Configuration Examples

#### Jaeger (Local Development)
```bash
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
export OTEL_ENVIRONMENT=development
export OTEL_SAMPLING_RATIO=1.0
```

#### Grafana Cloud
```bash
export OTEL_EXPORTER_OTLP_ENDPOINT=https://tempo-us-central1.grafana.net:4317
export OTEL_ENVIRONMENT=production
export OTEL_SAMPLING_RATIO=0.1
export OTEL_EXPORTER_OTLP_HEADERS="Authorization=Basic <base64-encoded-credentials>"
```

#### GCP Cloud Trace (via Collector)
```bash
export OTEL_EXPORTER_OTLP_ENDPOINT=http://otel-collector:4317
export OTEL_SERVICE_NAME=qa-pms-api
export OTEL_RESOURCE_ATTRIBUTES="cloud.provider=gcp,cloud.region=us-central1"
```

### Trace Sampling Strategy

- **Development**: 100% sampling (all traces exported)
- **Staging**: 50% sampling (half of traces exported)
- **Production**: 10% sampling (10% of traces exported)
- **High-traffic endpoints**: Lower sampling (1-5%)
- **Error scenarios**: Always sample (regardless of sampling rate)

### Span Best Practices

1. **Span Naming**: Use descriptive, hierarchical names
   - Good: `http.request`, `database.query`, `external_api.call`
   - Bad: `request`, `query`, `call`

2. **Attributes**: Add relevant context to spans
   - HTTP: method, path, status, user_agent
   - Database: statement, operation, rows
   - External: url, service_name, status

3. **Span Events**: Record important events during span lifetime
   - `cache.hit`, `cache.miss`
   - `retry.attempt`
   - `validation.error`

4. **Span Links**: Connect related spans
   - Async operations
   - Background jobs
   - Multi-service workflows

### Monitoring

Monitor these metrics in Prometheus (from Story 14.3):
- Traces exported per second
- Export failures
- Dropped traces (due to sampling)
- Span processing latency

Alert on:
- High export failure rate (> 1%)
- Sudden drop in trace volume
- High span processing latency (> 100ms)

---

## Troubleshooting

### Traces Not Appearing in Jaeger

1. Verify OTLP endpoint is correct
2. Check Jaeger is receiving traces: `docker logs jaeger`
3. Verify sampling ratio is not too low
4. Check for export errors in application logs
5. Verify network connectivity to OTLP endpoint

### High CPU Usage

1. Reduce sampling ratio
2. Exclude noisy spans (e.g., health checks)
3. Use batch export with larger batch size
4. Profile to identify expensive span operations

### Memory Leaks

1. Verify tracer provider is shutting down properly
2. Check for unbounded span collections
3. Monitor memory usage during load testing
4. Ensure async tasks complete before shutdown

---

## References

- [OpenTelemetry Rust Documentation](https://docs.rs/opentelemetry)
- [OpenTelemetry Specification](https://opentelemetry.io/docs/reference/specification/)
- [Jaeger Documentation](https://www.jaegertracing.io/docs/)
- [Grafana Tempo Documentation](https://grafana.com/docs/tempo/latest/)
- [Trace Context Format](https://www.w3.org/TR/trace-context/)
- [OTLP Specification](https://opentelemetry.io/docs/reference/specification/protocol/otlp/)