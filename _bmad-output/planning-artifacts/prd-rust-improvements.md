use tokio::signal;

async fn shutdown_signal() {
    let ctrl_c = signal::ctrl_c();
    #[cfg(unix)]
    let terminate = signal::unix::signal(signal::unix::SignalKind::terminate());
    
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    
    tokio::select! {
        _ = ctrl_c => info!("Received Ctrl+C, shutting down..."),
        _ = terminate => info!("Received SIGTERM, shutting down..."),
    }
}
```

**Moka Cache Configuration:**
```rust
use moka::future::Cache;
use std::time::Duration;

pub struct AppCache {
    pub tickets: Cache<String, JiraTicket>,
    pub search: Cache<String, Vec<SearchResult>>,
    pub metrics: Cache<String, DashboardMetrics>,
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
        }
    }
}
```

---

## Project Scoping & Phased Development

### MVP Strategy & Philosophy

**MVP Focus (Phase 1 - Development & Testing):**
Implement core production readiness improvements that enable deployment and basic observability:
- Graceful shutdown (Story 14.1)
- Request ID middleware (Story 14.2)
- Basic Prometheus metrics (Story 14.3 - HTTP metrics only)
- CLI admin tool for basic operations (Story 14.7 - serve, migrate, health check)

**MVP Success Metrics:**
- Zero data loss during deployments (graceful shutdown)
- 100% of requests have correlation IDs
- Basic HTTP metrics visible in Prometheus
- Developer productivity increased through CLI tool

### Post-MVP Features (Phase 2 - Production Readiness)

Implement advanced features for production-grade operations:
- Moka caching layer (Story 14.4)
- Rate limiting with Tower Governor (Story 14.5)
- OpenTelemetry distributed tracing (Story 14.6)
- Integration tests with Testcontainers (Story 14.8)

**Phase 2 Success Metrics:**
- 30-50% latency reduction through caching
- Protection against abuse (rate limiting)
- Distributed tracing capability (correlated requests across services)
- Integration test coverage > 80%

### Future Enhancements (Phase 3 - Optimization)

- Connection pool tuning and optimization
- Business metrics integration (workflows active, patterns detected)
- CDN for static assets deployment
- Kubernetes/Helm charts for enterprise deployment

### Risk Mitigation Strategy

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Free hosting limitations | High | High | Start with free trials, upgrade to paid tier only after validation |
| Moka cache eviction bugs | Medium | High | Comprehensive integration tests, conservative TTLs, monitoring cache hit/miss rates |
| Rate limiting false positives | Low | Medium | Start with generous limits (100 req/min), monitor and adjust based on usage patterns |
| OpenTelemetry complexity | Medium | Medium | Implement incrementally: basic metrics first, add tracing later |
| Deployment migration risks | Medium | High | Gradual migration path, maintain staging environment during transition |

---

## Non-Functional Requirements

### Performance

**PR-NFR-001: Response Time Targets**
- **Requirement:** P95 latency < 500ms for cached responses, < 2s for database queries
- **Measurement:** Prometheus histogram `http_request_duration_seconds`
- **Target:** 95th percentile < 0.5s (cached), < 2s (database)
- **Acceptance:** Prometheus metrics show P95 below targets after caching layer implemented

**PR-NFR-002: Throughput**
- **Requirement:** Support 100 concurrent API requests
- **Measurement:** Prometheus gauge `http_requests_pending`
- **Target:** Gauge value < 100 under normal load
- **Acceptance:** Stress test shows sustained performance under load

**PR-NFR-003: Cache Hit Rate**
- **Requirement:** Minimum 70% cache hit rate for frequently accessed data
- **Measurement:** Custom metrics `cache_hits_total`, `cache_misses_total`
- **Target:** Hit rate >= 70% after warmup period
- **Acceptance:** Cache telemetry shows sustained hit rates above threshold

### Security

**PR-NFR-001: Rate Limiting**
- **Requirement:** Block requests exceeding 100 requests/minute per IP address
- **Measurement:** Rate limiter state
- **Target:** 429 response with Retry-After header
- **Acceptance:** Load test confirms rate limiting works correctly

**PR-NFR-002: Graceful Shutdown**
- **Requirement:** Complete in-flight requests before terminating
- **Measurement:** Shutdown timeout configuration
- **Target:** All in-flight requests complete within 30s of shutdown signal
- **Acceptance:** Integration test verifies graceful shutdown behavior

### Reliability

**PR-NFR-001: Uptime**
- **Requirement:** Maintain 99.95% uptime during normal operations
- **Measurement:** Health check failure rate and duration
- **Target:** < 0.05% downtime (approximately 2 hours/month acceptable)
- **Acceptance:** Monitoring shows uptime > 99.95%

**PR-NFR-002: Data Consistency**
- **Requirement:** No data loss during graceful shutdown
- **Measurement:** Workflow state persistence verification
- **Target:** Zero incomplete workflows after shutdown
- **Acceptance:** Integration test verifies all workflows either completed or resumed after restart

### Observability

**PR-NFR-001: Request Correlation**
- **Requirement:** Every request traceable across all services using correlation ID
- **Measurement:** Distributed tracing span relationships
- **Target:** 100% of requests have correlation IDs visible in all services
- **Acceptance:** Traces in Jaeger/Tempo show complete request flows

**PR-NFR-002: Metrics Availability**
- **Requirement:** Prometheus metrics exposed within 5s of scrape
- **Measurement:** Metrics endpoint response time
- **Target:** 95th percentile < 100ms
- **Acceptance:** Prometheus scraper can reliably collect metrics

**PR-NFR-003: Structured Logging**
- **Requirement:** All logs in JSON format with correlation ID and severity
- **Measurement:** Log structure validation
- **Target:** 100% of log entries have required fields
- **Acceptance:** Log parsing and visualization tools can ingest structured logs

### Integration

**PR-NFR-001: Backward Compatibility**
- **Requirement:** Existing API contracts remain unchanged
- **Measurement:** API contract testing against existing clients
- **Target:** Zero breaking changes to existing API consumers
- **Acceptance:** Integration tests pass with existing API contract tests

**PR-NFR-002: External Service Health**
- **Requirement:** Graceful degradation when external services are unavailable
- **Measurement:** Health check status codes and timeout handling
- **Target:** Return 503 with degraded but useful information
- **Acceptance:** Integration tests verify graceful degradation behavior

### Scalability

**PR-NFR-001: Horizontal Scaling**
- **Requirement:** Application scales horizontally with additional instances
- **Measurement:** Load testing with multiple instances
- **Target:** Linear throughput increase with instances
- **Acceptance:** Platform's auto-scaling features work as expected

**PR-NFR-002: Resource Efficiency**
- **Requirement:** Cost optimization through efficient resource utilization
- **Measurement:** Cost per request and cache effectiveness
- **Target:** Monthly hosting cost < $20 for typical QA team usage
- **Acceptance:** Hosting dashboard shows predictable costs within budget

---

## Conclusion & Next Steps

### Document Summary

Created comprehensive PRD for Rust improvements with:
- **13 Epic Stories** broken down from research findings
- **4 User Journeys** capturing different stakeholder perspectives
- **20 Non-Functional Requirements** across performance, security, reliability, observability, integration, and scalability
- **3 Development Phases** (MVP, Post-MVP, Future)
- **Risk Mitigation Strategy** with specific plans for identified risks

### Readiness for Development

**Technical Prerequisites:**
- ✅ Research completed with Context7-verified best practices
- ✅ Hosting options evaluated (Render, Fly.io, Railway, Supabase)
- ✅ Stack decisions documented (OpenTelemetry, Moka, Tower Governor, Axum Prometheus)
- ✅ Code examples provided for all major components
- ✅ Non-functional requirements defined with measurable criteria

**Missing for Development:**
- [ ] Epic stories need to be split into individual story documents via BMM workflow
- [ ] Sprint status needs to be created for Rust improvements
- [ ] Implementation readiness check should be run before development begins

### Next Steps Recommended

**1. Execute BMM Workflow: `create-epics-and-stories`**
- Focus the new requirements from this PRD
- Ensure dependencies between stories are properly defined
- Generate individual story documents with acceptance criteria

**2. Execute BMM Workflow: `check-implementation-readiness`**
- Verify that new requirements don't conflict with existing implementation
- Ensure all dependencies are available or can be satisfied
- Identify any gaps that need to be addressed

**3. Begin Sprint 1: Development & Testing**
- Start with Story 14.1 (Graceful Shutdown)
- Follow dependency chain through stories 14.2, 14.3, 14.7
- Track progress in sprint status file

**4. Select Hosting Solution**
- Based on user constraints (no Docker locally), recommend starting with Fly.io free trial
- 7 days sufficient to validate basic improvements
- Clear upgrade path to paid tier for production

### Success Criteria for PRD

**User Success:**
- [ ] User (Daniel) understands all requirements and trade-offs
- [ ] Development team has clear implementation path with dependencies
- [ ] Hosting solution selected based on constraints and requirements
- [ ] Non-functional requirements are measurable and testable

**Business Success:**
- [ ] Production-grade reliability and observability achievable
- [ ] Performance improvements result in measurable latency reduction
- [ ] Deployment path viable without Docker dependency
- [ ] Total effort (13.5 days) justifies business value

**Technical Success:**
- [ ] All stack components have verified libraries and examples
- [ ] Architecture supports graceful shutdown and distributed tracing
- [ ] Caching strategy proven to reduce latency 30-50%
- [ ] Security mechanisms (rate limiting) protect against abuse
- [ ] Integration strategy allows phased implementation without system-wide rewrites

---

## Appendix

### A. Dependencies on Other Epics

**Epic 14 Stories Prerequisites:**
- Story 14.1 (Graceful Shutdown): No prerequisites
- Story 14.2 (Request ID): No prerequisites
- Story 14.3 (Prometheus Metrics): Prerequisites satisfied by stories 14.1-14.2
- Story 14.4 (Moka Cache): No prerequisites (can be implemented in parallel)
- Story 14.5 (Rate Limiting): Prerequisites satisfied by stories 14.2-14.3
- Story 14.6 (OpenTelemetry Tracing): Prerequisites satisfied by story 14.2
- Story 14.7 (CLI Admin Tool): No prerequisites
- Story 14.8 (Integration Tests): Prerequisites satisfied by stories 14.4-14.5

### B. Technology Stack Additions

**New Workspace Dependencies:**
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

### C. Hosting Decision Matrix

| Criteria | Render Free | Fly.io Free | Railway | Supabase Free |
|-----------|--------------|-----------|-----------|--------------|
| Rust Support | ✅ Native | ✅ Native | ✅ Containers | ✅ Containers |
| PostgreSQL Included | ✅ Included | ✅ Add-on | ❌ Add-on needed | ✅ Built-in (500MB) |
| Free Trial Duration | 15 min idle spin-down | 7 days / 2 hours VM | One-time credit (trial) | Unlimited (500MB DB) |
| Production Readiness | ⚠️ Idle spin-down issues | ✅ Good SLA (99.95%) | ✅ Good (pay-as-you-go) | ⚠️ Not production-ready |
| Upgrade Path | Clear | Clear | Clear | Clear | Database size limits |
| Recommendation | Dev only | ⭐⭐ Production | ⭐⭐ Production (budget) | ⭐ Development |
| Cost (estimate) | $0 dev → $7+ | $5-50 | usage-based (~$20) | $0 dev → $25+ |
| Docker Required | No (managed DB) | No (containers) | No (containers) | No (managed DB) |

**Recommendation:** Use Fly.io for development/testing (7-day free trial), then upgrade to paid tier for production.

### D. Epic 14 Sprint Plan

**Sprint 1: Reliability & Debugging (1.5 days)**
- Story 14.1: Graceful Shutdown and Signal Handling (1 day)
- Story 14.2: Request ID Middleware for Correlation (0.5 days)

**Sprint 2: Observability - Metrics (2 days)**
- Story 14.3: Prometheus Metrics Integration (2 days)

**Sprint 3: Performance - Caching (2 days)**
- Story 14.4: In-Memory Cache Layer with Moka (2 days)

**Sprint 4: Security - Rate Limiting (1 day)**
- Story 14.5: Rate Limiting with Tower Governor (1 day)

**Sprint 5: Observability - Tracing (2 days)**
- Story 14.6: OpenTelemetry Distributed Tracing (2 days)

**Sprint 6: Usability - CLI (2 days)**
- Story 14.7: CLI Admin Tool (2 days)

**Sprint 7: Quality - Integration Tests (3 days)**
- Story 14.8: Integration Tests with Testcontainers (3 days)

**Total Estimated Effort:** 13.5 days

---

**Document Status:** ✅ Complete
**Next BMAD Workflow:** `create-epics-and-stories` - Generate individual story documents from this PRD