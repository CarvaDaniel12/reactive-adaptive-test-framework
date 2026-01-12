# Story 24.3: Correlation API Endpoint

Status: ready-for-dev

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

**As a** QA Engineer working on PMS integration quality  
**I want** REST API endpoints to retrieve correlation data between test results and integration health  
**So that** I can access correlation information from the dashboard and other frontend components

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 24.3 |
| Epic | Epic 24: Test-Integration Correlation Engine |
| Sprint | Sprint 2: Correlation API and Dashboard |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Story 24.2 (Correlation Database Schema) |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create `routes/correlation.rs` in `qa-pms-api`
   - Follow existing route patterns (`routes/dashboard.rs`, `routes/support.rs` as reference)
   - Use Axum router with `State<AppState>`
   - Use `utoipa` for OpenAPI documentation

2. Implement API endpoints:
   - `GET /api/v1/correlation/test-integration?period=30d&integration=airbnb`
     - Returns all correlations filtered by period and integration
   - `GET /api/v1/correlation/test-integration/:test_case_id`
     - Returns correlations for a specific test case

3. Support filtering:
   - Filter by period (7d, 30d, 90d, 1y) - use `qa_pms_dashboard::parse_period`
   - Filter by integration_id (booking-com, airbnb, vrbo, hmbn)
   - Filter by test_case_id (path parameter)

4. Response format:
   - Returns correlation data with: score, type, pattern, confidence
   - Use correlation types from `qa-pms-correlation` crate
   - Response time < 500ms

5. Error handling:
   - Use `ApiError` from `qa_pms_core`
   - Return appropriate HTTP status codes
   - Use `SqlxResultExt` for database error mapping

---

## Acceptance Criteria

- [ ] **Given** API endpoint exists  
  **When** GET /api/v1/correlation/test-integration  
  **Then** returns all correlations

- [ ] **Given** API endpoint exists  
  **When** GET /api/v1/correlation/test-integration?integration=airbnb  
  **Then** returns correlations filtered for Airbnb integration

- [ ] **Given** API endpoint exists  
  **When** GET /api/v1/correlation/test-integration?period=30d  
  **Then** returns correlations within the 30-day period

- [ ] **Given** API endpoint exists  
  **When** GET /api/v1/correlation/test-integration/:test_case_id  
  **Then** returns correlations for the specific test case

- [ ] **Given** API endpoint exists  
  **When** response is returned  
  **Then** response includes correlation score, type, pattern, confidence

- [ ] **Given** API endpoint exists  
  **When** response is returned  
  **Then** response time < 500ms

---

## Tasks / Subtasks

- [ ] Task 1: Create correlation routes module (AC: #1)
  - [ ] 1.1: Create `crates/qa-pms-api/src/routes/correlation.rs`
  - [ ] 1.2: Add module declaration in `crates/qa-pms-api/src/routes/mod.rs`
  - [ ] 1.3: Implement `router()` function returning `Router<AppState>`
  - [ ] 1.4: Add routes to router:
    - `GET /api/v1/correlation/test-integration`
    - `GET /api/v1/correlation/test-integration/:test_case_id`

- [ ] Task 2: Implement list correlations endpoint (AC: #1, #2, #3, #5, #6)
  - [ ] 2.1: Create `CorrelationQuery` struct with query parameters (period, integration)
  - [ ] 2.2: Implement `list_correlations` handler function
  - [ ] 2.3: Parse period parameter using `qa_pms_dashboard::parse_period`
  - [ ] 2.4: Query correlations from database (use repository from Story 24.1 if exists, or direct SQLx query)
  - [ ] 2.5: Filter by integration_id if provided
  - [ ] 2.6: Filter by period (date range) if provided
  - [ ] 2.7: Return correlations in response format
  - [ ] 2.8: Add `utoipa::path` macro for OpenAPI documentation

- [ ] Task 3: Implement get correlation by test case endpoint (AC: #4, #5, #6)
  - [ ] 3.1: Create `get_correlation_by_test_case` handler function
  - [ ] 3.2: Extract test_case_id from path parameter
  - [ ] 3.3: Query correlations for specific test_case_id
  - [ ] 3.4: Return correlations in response format
  - [ ] 3.5: Add `utoipa::path` macro for OpenAPI documentation

- [ ] Task 4: Create request/response types (AC: #5)
  - [ ] 4.1: Create `CorrelationQuery` struct with `Deserialize`, `IntoParams`
  - [ ] 4.2: Create `CorrelationResponse` struct with `Serialize`, `ToSchema`
  - [ ] 4.3: Use types from `qa-pms-correlation` crate (if available) or define inline
  - [ ] 4.4: Use camelCase serialization (`serde(rename_all = "camelCase")`)

- [ ] Task 5: Add routes to app.rs (AC: #1)
  - [ ] 5.1: Import `routes::correlation` module in `app.rs`
  - [ ] 5.2: Add `.merge(routes::correlation::router())` to main router

- [ ] Task 6: Add dependencies (AC: #1)
  - [ ] 6.1: Add `qa-pms-correlation = { workspace = true }` to `crates/qa-pms-api/Cargo.toml`
  - [ ] 6.2: Verify `qa-pms-dashboard` dependency exists (for `parse_period`)

- [ ] Task 7: Add unit and integration tests (AC: #1, #2, #3, #4, #5, #6)
  - [ ] 7.1: Test list correlations endpoint with various query parameters
  - [ ] 7.2: Test get correlation by test case endpoint
  - [ ] 7.3: Test error handling (invalid test_case_id, etc.)
  - [ ] 7.4: Test response time < 500ms

---

## Files to Create

| File | Changes |
|------|---------|
| `crates/qa-pms-api/src/routes/correlation.rs` | Create correlation routes module with endpoints |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-api/src/routes/mod.rs` | Add `pub mod correlation;` |
| `crates/qa-pms-api/src/app.rs` | Add `.merge(routes::correlation::router())` to router |
| `crates/qa-pms-api/Cargo.toml` | Add `qa-pms-correlation = { workspace = true }` dependency |

---

## Dev Notes

### API Endpoint Patterns

**Router Structure:**
```rust
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/correlation/test-integration", get(list_correlations))
        .route("/api/v1/correlation/test-integration/:test_case_id", get(get_correlation_by_test_case))
}
```

**Handler Pattern:**
```rust
#[utoipa::path(
    get,
    path = "/api/v1/correlation/test-integration",
    params(
        ("period" = String, Query, description = "Period: 7d, 30d, 90d, 1y"),
        ("integration" = String, Query, description = "Integration ID: booking-com, airbnb, vrbo, hmbn")
    ),
    responses(
        (status = 200, description = "Correlations", body = Vec<CorrelationResponse>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Correlation"
)]
pub async fn list_correlations(
    State(state): State<AppState>,
    Query(query): Query<CorrelationQuery>,
) -> ApiResult<Json<Vec<CorrelationResponse>>> {
    // Implementation
}
```

**Query Parameters:**
```rust
#[derive(Debug, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct CorrelationQuery {
    /// Period: 7d, 30d, 90d, 1y
    pub period: Option<String>,
    /// Integration ID: booking-com, airbnb, vrbo, hmbn
    pub integration: Option<String>,
}
```

**Response Type:**
```rust
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CorrelationResponse {
    pub test_case_id: String,
    pub test_case_name: String,
    pub integration_id: String,
    pub correlation_score: f64,
    pub correlation_type: String, // "high", "medium", "low"
    pub pattern: String,
    pub confidence: f64,
    pub last_correlated: String, // ISO 8601 timestamp
}
```

### Database Query Pattern

**Query with Filters:**
```rust
// Parse period to date range
let (start, end, _) = if let Some(period_str) = &query.period {
    let days = parse_period(period_str);
    period_boundaries(days)
} else {
    // Default: last 30 days
    period_boundaries(30)
};

// Build query with optional filters
let mut query_builder = sqlx::QueryBuilder::new(
    "SELECT * FROM test_integration_correlations WHERE 1=1"
);

if let Some(integration_id) = &query.integration {
    query_builder.push(" AND integration_id = ");
    query_builder.push_bind(integration_id);
}

query_builder.push(" AND last_correlated >= ");
query_builder.push_bind(start);
query_builder.push(" AND last_correlated <= ");
query_builder.push_bind(end);

query_builder.push(" ORDER BY correlation_score DESC");
```

### Project Structure Notes

**Route Module:**
- Follow existing patterns from `routes/dashboard.rs`, `routes/support.rs`
- Use `State<AppState>` for database pool access
- Use `ApiResult<T>` type alias for error handling
- Use `SqlxResultExt` for database error mapping

**Dependencies:**
- `qa-pms-correlation`: Correlation types and repository (if exists)
- `qa-pms-dashboard`: Period parsing utilities (`parse_period`, `period_boundaries`)
- `qa-pms-core`: `ApiError` for error handling

**OpenAPI Documentation:**
- Use `utoipa::path` macro for each endpoint
- Use `IntoParams` for query parameters
- Use `ToSchema` for response types
- Tag endpoints with `tag = "Correlation"`

### Testing Standards

**Unit Tests:**
- Test route handlers with mock data
- Test query parameter parsing
- Test error handling

**Integration Tests:**
- Test endpoints with real database
- Test filtering by period
- Test filtering by integration
- Test filtering by test_case_id
- Verify response format

**Performance Tests:**
- Verify response time < 500ms
- Test with various query combinations

### References

- Epic Document: `_bmad-output/planning-artifacts/epics-observability-pms-integrations-2026-01-10.md` (Epic 24, Story 24.3)
- Architecture Document: `_bmad-output/planning-artifacts/architecture-observability-pms-integrations-2026-01-10.md` (Section 5.3: Correlation API)
- Route Patterns: `qa-intelligent-pms/crates/qa-pms-api/src/routes/dashboard.rs` (reference)
- Route Patterns: `qa-intelligent-pms/crates/qa-pms-api/src/routes/support.rs` (reference)
- Dependency: Story 24.2 (Correlation Database Schema) - must be completed first
- Dependency: Story 24.1 (Correlation Calculation Engine) - for correlation types
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
