# System-Level Test Design

**Project:** QA Intelligent PMS - Companion Framework
**Date:** 2026-01-04
**Author:** TEA Agent (Test Architect)
**Mode:** System-Level Testability Review (Phase 3 - Solutioning)
**Status:** Complete

---

## Executive Summary

This document provides a comprehensive testability assessment of the QA Intelligent PMS architecture before implementation begins. It evaluates the system's controllability, observability, and reliability from a testing perspective, identifies Architecturally Significant Requirements (ASRs) that require special testing attention, and defines the test strategy for the project.

**Architecture Overview:**
- **Backend:** Rust 1.80+ with Tokio async runtime, Axum 0.7+ web framework
- **Database:** Neon PostgreSQL (cloud) with SQLx 0.7
- **Frontend:** React 18+ with Vite 5+, Tailwind CSS v4, Zustand state management
- **Integrations:** 5 external systems (Jira, Postman, Testmo, Splunk, Grafana)

---

## Testability Assessment

### Controllability: ✅ PASS

**Assessment:** The architecture provides excellent control over system state for testing.

| Aspect | Status | Evidence |
|--------|--------|----------|
| API Seeding | ✅ | Axum REST API allows direct database seeding via `/api/v1/*` endpoints |
| Database Reset | ✅ | SQLx migrations + Neon branching enables isolated test databases |
| Dependency Injection | ✅ | Rust trait-based integration interfaces enable mocking (`qa-pms-core` traits) |
| External Service Mocking | ✅ | Each integration crate (`qa-pms-jira`, `qa-pms-postman`, etc.) can be mocked via traits |
| Error Condition Triggering | ✅ | Structured error types (`thiserror`) allow controlled error injection |
| Configuration Control | ✅ | YAML configuration with environment variable overrides via `dotenv` |

**Recommendations:**
1. Implement test factories in `qa-pms-core` for common entities (User, Workflow, Ticket)
2. Use Neon database branching for isolated integration test environments
3. Create mock implementations for all integration traits for unit testing

### Observability: ✅ PASS

**Assessment:** The architecture provides comprehensive observability for test validation.

| Aspect | Status | Evidence |
|--------|--------|----------|
| Structured Logging | ✅ | `tracing` + `tracing-subscriber` with structured spans and fields |
| Request Tracing | ✅ | `tower-http::TraceLayer` for HTTP request correlation |
| Metrics Collection | ✅ | Foundation ready for Prometheus integration |
| Health Endpoints | ✅ | `/api/v1/health` with integration status (NFR-REL-02) |
| Error Reporting | ✅ | `anyhow` context chains + `thiserror` structured errors |
| Test Result Determinism | ✅ | Async tests with `#[tokio::test]`, no global mutable state |

**Recommendations:**
1. Add test-specific tracing subscriber for capturing logs during tests
2. Implement correlation IDs for end-to-end request tracing
3. Add performance metrics endpoints for NFR validation

### Reliability: ✅ PASS

**Assessment:** The architecture supports reliable, isolated, and reproducible tests.

| Aspect | Status | Evidence |
|--------|--------|----------|
| Test Isolation | ✅ | Cargo workspace enables per-crate testing; database branching for integration tests |
| Parallel Safety | ✅ | Rust ownership model prevents data races; async tests are isolated |
| Stateless Design | ✅ | REST API is stateless; state in PostgreSQL with transaction isolation |
| Cleanup Discipline | ✅ | SQLx transactions can be rolled back; test fixtures with cleanup |
| Failure Reproducibility | ✅ | Deterministic waits via `tokio::time::sleep`, structured error context |
| Loose Coupling | ✅ | Modular crate architecture with clear boundaries |

**Recommendations:**
1. Implement database transaction rollback for integration tests
2. Use deterministic test data (seeded random generators)
3. Capture HAR files for network-dependent tests

---

## Architecturally Significant Requirements (ASRs)

These quality requirements drive architecture decisions and pose testability challenges.

### High-Risk ASRs (Score ≥6)

| ID | Requirement | Category | Probability | Impact | Score | Testing Approach |
|----|-------------|----------|-------------|--------|-------|------------------|
| ASR-001 | API calls < 2s for 95% (NFR-PERF-01) | PERF | 2 | 3 | 6 | k6 load testing, p95 latency monitoring |
| ASR-002 | Encrypted token storage (NFR-SEC-01) | SEC | 2 | 3 | 6 | Unit tests for encryption/decryption, security audit |
| ASR-003 | OAuth 2.0 + PKCE for Jira (NFR-SEC-04) | SEC | 2 | 3 | 6 | Integration tests with mock OAuth server |
| ASR-004 | 99.5% uptime (NFR-REL-01) | REL | 2 | 3 | 6 | Health check monitoring, chaos testing |
| ASR-005 | 100 concurrent QAs (NFR-SCAL-01) | PERF | 2 | 3 | 6 | Load testing with k6, connection pool validation |
| ASR-006 | Retry with exponential backoff (NFR-REL-03) | REL | 3 | 2 | 6 | Unit tests for retry logic, integration tests with flaky mock |

### Medium-Risk ASRs (Score 3-4)

| ID | Requirement | Category | Probability | Impact | Score | Testing Approach |
|----|-------------|----------|-------------|--------|-------|------------------|
| ASR-007 | Dashboard < 5s for historical data (NFR-PERF-02) | PERF | 2 | 2 | 4 | Performance tests with 90-day data sets |
| ASR-008 | Search < 3s for 90% (NFR-PERF-03) | PERF | 2 | 2 | 4 | Load testing search endpoints |
| ASR-009 | Health checks every 60s (NFR-REL-02) | REL | 2 | 2 | 4 | Integration tests for health check timing |
| ASR-010 | HTTPS/TLS 1.2+ (NFR-SEC-03) | SEC | 1 | 3 | 3 | Security audit, TLS version validation |
| ASR-011 | Secure logging (NFR-SEC-02) | SEC | 2 | 2 | 4 | Log audit tests, no sensitive data in traces |
| ASR-012 | YAML config up to 10K lines (NFR-SCAL-03) | PERF | 1 | 3 | 3 | Performance tests with large config files |

### Low-Risk ASRs (Score 1-2)

| ID | Requirement | Category | Probability | Impact | Score | Testing Approach |
|----|-------------|----------|-------------|--------|-------|------------------|
| ASR-013 | 30-day log retention (NFR-REL-04) | OPS | 1 | 2 | 2 | Configuration validation |
| ASR-014 | 7-day API change notice (NFR-INT-01) | OPS | 1 | 2 | 2 | Documentation review |
| ASR-015 | Plugin architecture (NFR-SCAL-04) | TECH | 1 | 2 | 2 | Integration tests for new plugins |

---

## Test Levels Strategy

Based on the architecture (Rust backend + React frontend + 5 external integrations):

### Recommended Test Distribution

| Level | Percentage | Rationale |
|-------|------------|-----------|
| **Unit** | 60% | Rust's strong type system enables comprehensive unit testing; business logic in crates |
| **Integration** | 25% | API contracts between crates; database operations; integration client behavior |
| **E2E** | 15% | Critical user journeys (Ana's workflow, Carlos's dashboard); cross-system validation |

### Test Level Selection by Component

| Component | Primary Level | Secondary Level | Rationale |
|-----------|---------------|-----------------|-----------|
| `qa-pms-core` | Unit | - | Pure types, traits, validation logic |
| `qa-pms-config` | Unit | Integration | Encryption logic (unit), YAML parsing (integration) |
| `qa-pms-jira` | Unit | Integration | Client logic (unit), OAuth flow (integration) |
| `qa-pms-postman` | Unit | Integration | API client (unit), search (integration) |
| `qa-pms-testmo` | Unit | Integration | API client (unit), test run creation (integration) |
| `qa-pms-splunk` | Unit | Integration | Query templates (unit), log retrieval (integration) |
| `qa-pms-workflow` | Unit | Integration | Engine logic (unit), state persistence (integration) |
| `qa-pms-tracking` | Unit | Integration | Timer logic (unit), session management (integration) |
| `qa-pms-dashboard` | Unit | Integration | Metrics calculation (unit), aggregation (integration) |
| `qa-pms-api` | Integration | E2E | Route handlers (integration), full flows (E2E) |
| `frontend/` | Component | E2E | UI components (component), user journeys (E2E) |

### Test Framework Selection

| Level | Backend (Rust) | Frontend (React) |
|-------|----------------|------------------|
| Unit | `cargo test` + `#[tokio::test]` | Vitest |
| Integration | `cargo test` + SQLx test utilities | Vitest + MSW (Mock Service Worker) |
| E2E | - | Playwright |
| Performance | k6 | Lighthouse |

---

## NFR Testing Approach

### Security (SEC)

| NFR | Testing Approach | Tools |
|-----|------------------|-------|
| NFR-SEC-01 (Encrypted tokens) | Unit tests for `aes-gcm` encryption/decryption; verify `secrecy` prevents logging | `cargo test` |
| NFR-SEC-02 (Secure logging) | Log audit tests; grep for sensitive patterns in trace output | Custom audit script |
| NFR-SEC-03 (HTTPS/TLS 1.2+) | TLS version validation; certificate pinning tests | `rustls` configuration tests |
| NFR-SEC-04 (OAuth 2.0 + PKCE) | Mock OAuth server; PKCE code challenge validation | Integration tests with mock |

**Security Test Checklist:**
- [ ] Token encryption at rest verified
- [ ] No secrets in log output
- [ ] TLS 1.2+ enforced for all external calls
- [ ] PKCE flow correctly implemented
- [ ] Input validation on all API endpoints

### Performance (PERF)

| NFR | Testing Approach | Tools | Thresholds |
|-----|------------------|-------|------------|
| NFR-PERF-01 (API < 2s) | Load testing with realistic data | k6 | p95 < 2000ms |
| NFR-PERF-02 (Dashboard < 5s) | Performance tests with 90-day data | k6 + Lighthouse | p95 < 5000ms |
| NFR-PERF-03 (Search < 3s) | Load testing search endpoints | k6 | p90 < 3000ms |

**Performance Test Scenarios:**
1. **Baseline:** Single user, warm cache
2. **Load:** 50 concurrent users, steady state
3. **Stress:** 100 concurrent users, peak load
4. **Spike:** Sudden burst to 150 users

### Reliability (REL)

| NFR | Testing Approach | Tools |
|-----|------------------|-------|
| NFR-REL-01 (99.5% uptime) | Health check monitoring; chaos testing | Health endpoint + chaos monkey |
| NFR-REL-02 (60s health checks) | Timer validation; integration status tests | Integration tests |
| NFR-REL-03 (Retry 1s, 2s, 4s) | Unit tests for retry logic; integration with flaky mock | `cargo test` |
| NFR-REL-04 (30-day logs) | Configuration validation; log rotation tests | Config tests |

**Reliability Test Scenarios:**
1. **Integration Failure:** Mock Jira/Postman/Testmo returning 500 errors
2. **Database Failure:** Neon connection timeout simulation
3. **Network Partition:** Delayed responses, dropped connections
4. **Recovery:** Verify system recovers after transient failures

### Maintainability

| Aspect | Testing Approach | Targets |
|--------|------------------|---------|
| Code Coverage | `cargo tarpaulin` / `llvm-cov` | ≥80% for core functionality |
| Clippy Warnings | `cargo clippy -- -D warnings` | Zero warnings |
| Documentation | `cargo doc` | All public items documented |
| Type Safety | Compile-time checks | No `unwrap()` in production code |

---

## Test Environment Requirements

### Local Development

```bash
# Backend
cargo test                    # All unit + integration tests
cargo test -p qa-pms-core     # Single crate tests
cargo test --test integration # Integration tests only

# Frontend
npm run test                  # Vitest unit tests
npm run test:e2e              # Playwright E2E tests
```

### CI/CD Pipeline (GitHub Actions)

| Stage | Tests | Timeout | Trigger |
|-------|-------|---------|---------|
| Lint | `cargo fmt --check`, `cargo clippy` | 5 min | Every commit |
| Unit | `cargo test --lib` | 10 min | Every commit |
| Integration | `cargo test --test '*'` | 15 min | PR to main |
| E2E | Playwright critical paths | 20 min | PR to main |
| Performance | k6 baseline | 10 min | Nightly |

### Test Database

- **Development:** Neon branch per developer
- **CI:** Ephemeral Neon branch per pipeline run
- **Staging:** Dedicated Neon branch with production-like data

---

## Testability Concerns

### No Blockers Identified ✅

The architecture is well-designed for testability with no blocking concerns.

### Minor Concerns (Recommendations)

| Concern | Impact | Mitigation |
|---------|--------|------------|
| External API rate limits | Tests may be throttled | Use mock servers for unit/integration; real APIs only for E2E |
| Neon cold start latency | First test may be slow | Warm up database connection in test setup |
| OAuth token refresh | Flaky tests if tokens expire | Use long-lived test tokens; mock OAuth for unit tests |
| Time-dependent tests | Timer tests may be flaky | Use `tokio::time::pause()` for deterministic time control |

---

## Recommendations for Sprint 0

### Test Infrastructure Setup

1. **Configure Test Utilities Crate**
   ```
   crates/qa-pms-test-utils/
   ├── src/
   │   ├── lib.rs
   │   ├── factories.rs      # Test data factories
   │   ├── fixtures.rs       # Database fixtures
   │   ├── mocks.rs          # Mock implementations
   │   └── assertions.rs     # Custom test assertions
   ```

2. **Database Test Harness**
   - Implement transaction rollback for integration tests
   - Create Neon branch per test suite
   - Seed common test data via migrations

3. **Mock Server Setup**
   - Implement mock Jira OAuth server
   - Create Postman API mock with sample responses
   - Build Testmo mock for search/run creation

4. **CI Pipeline Configuration**
   - Add test stages to GitHub Actions
   - Configure Neon branch creation/cleanup
   - Set up test result reporting

### Test Data Strategy

1. **Factories:** Use `fake` crate for generating realistic test data
2. **Fixtures:** Pre-defined database states for common scenarios
3. **Cleanup:** Automatic cleanup after each test via transaction rollback

### Performance Baseline

1. **Establish Baselines:** Run k6 tests against empty system
2. **Define SLOs:** Document p95 latency targets per endpoint
3. **Monitor Regression:** Alert on >10% degradation from baseline

---

## Quality Gate Criteria (Solutioning Phase)

For the project to proceed to Implementation (Phase 4):

### Required ✅

- [x] Testability assessment complete (this document)
- [x] No blocking testability concerns
- [x] Test levels strategy defined
- [x] NFR testing approach documented
- [x] Test environment requirements specified

### Recommended

- [ ] Test utilities crate scaffolded
- [ ] CI pipeline with test stages configured
- [ ] Mock servers for external integrations ready
- [ ] Performance baseline established

---

## Appendix

### Knowledge Base References

- `nfr-criteria.md` - NFR validation approach
- `test-levels-framework.md` - Test level selection guidance
- `risk-governance.md` - Testability risk identification
- `test-quality.md` - Quality standards and Definition of Done

### Related Documents

- **Architecture:** `_bmad-output/planning-artifacts/architecture.md`
- **PRD:** `_bmad-output/planning-artifacts/prd.md`
- **Epics:** `_bmad-output/planning-artifacts/epics.md`
- **Project Context:** `_bmad-output/planning-artifacts/project-context.md`

---

**Generated by:** BMad TEA Agent - Test Architect Module
**Workflow:** `_bmad/bmm/workflows/testarch/test-design` (System-Level Mode)
**Version:** 4.0 (BMad v6)
