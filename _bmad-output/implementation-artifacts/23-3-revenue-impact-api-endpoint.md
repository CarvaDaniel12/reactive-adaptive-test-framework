# Story 23.3: Revenue Impact API Endpoint

**Status:** `ready-for-dev`

## Story

**As a** QA Engineer working on PMS integration quality  
**I want** to access revenue impact data via REST API  
**So that** I can display revenue metrics in the dashboard and make data-driven decisions

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 23.3 |
| Epic | Epic 23: Revenue Impact Calculator and Dashboard |
| Sprint | Sprint 2: Revenue API and Dashboard |
| Priority | P0 |
| Estimated Days | 1 |
| Dependencies | Story 23.2 (Revenue Impact Calculation Engine) |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create `routes/revenue.rs` in `qa-pms-api` crate
   - Follow existing route patterns (e.g., `routes/dashboard.rs`, `routes/pm_dashboard.rs`)
   - Use Axum routing with `State<AppState>` pattern

2. Endpoint: `GET /api/v1/revenue/impact?period=30d`
   - Query parameter: `period` (7d, 30d, 90d, 1y) - defaults to "30d"
   - Returns `RevenueImpact` response with revenue at risk, revenue protected, breakdown
   - Response time target: < 500ms

3. Endpoint: `GET /api/v1/revenue/config` (admin only, future)
   - Note: Mark as future/optional for this story
   - Can be implemented in Phase 2 if needed

4. Endpoint: `PUT /api/v1/revenue/config` (admin only, future)
   - Note: Mark as future/optional for this story
   - Can be implemented in Phase 2 if needed

5. Response structure:
   - Use `RevenueImpact` type from Story 23.2
   - Include `revenue_at_risk` (KPIMetric), `revenue_protected` (KPIMetric), `breakdown` (Vec<RevenueBreakdown>)
   - Use `utoipa` for OpenAPI documentation

6. Integration with calculation engine:
   - Use `RevenueCalculator` from Story 23.2
   - Call `calculate_revenue_impact(period)` function
   - Handle errors gracefully (return appropriate HTTP status codes)

7. Add routes to API router:
   - Register routes in `app.rs`
   - Include in OpenAPI documentation
   - Follow existing route registration patterns

---

## Acceptance Criteria

- [ ] **Given** API endpoint implemented  
  **When** GET /api/v1/revenue/impact  
  **Then** returns revenue at risk and revenue protected metrics

- [ ] **Given** API endpoint implemented  
  **When** GET /api/v1/revenue/impact?period=7d  
  **Then** returns data for 7 days period

- [ ] **Given** API endpoint implemented  
  **When** response returned  
  **Then** response includes breakdown by integration (booking-com, airbnb, vrbo, hmbn)

- [ ] **Given** API endpoint implemented  
  **When** response returned  
  **Then** response time < 500ms

- [ ] **Given** API endpoint implemented  
  **When** error occurs (missing config, missing health data)  
  **Then** error response is clear with appropriate HTTP status code (500 for server errors, 400 for bad requests)

---

## Tasks

- [ ] Task 1: Create revenue routes module (AC: #1)
  - [ ] 1.1: Create `crates/qa-pms-api/src/routes/revenue.rs`
  - [ ] 1.2: Import required dependencies (axum, qa-pms-revenue, qa-pms-dashboard)
  - [ ] 1.3: Create `router()` function returning `Router<AppState>`
  - [ ] 1.4: Define route: `GET /api/v1/revenue/impact`

- [ ] Task 2: Implement GET /api/v1/revenue/impact handler (AC: #1, #2, #3)
  - [ ] 2.1: Create `RevenueImpactQuery` struct for query parameters (period)
  - [ ] 2.2: Create `get_revenue_impact()` handler function
  - [ ] 2.3: Parse period parameter (default to "30d" if missing)
  - [ ] 2.4: Call calculation engine: `calculate_revenue_impact(period)`
  - [ ] 2.5: Return `RevenueImpact` as JSON response
  - [ ] 2.6: Add OpenAPI documentation using `utoipa::path` macro

- [ ] Task 3: Add error handling (AC: #5)
  - [ ] 3.1: Map calculation errors to HTTP status codes
  - [ ] 3.2: Return 500 for server errors (missing config, calculation errors)
  - [ ] 3.3: Return 400 for bad requests (invalid period parameter)
  - [ ] 3.4: Return clear error messages in error response

- [ ] Task 4: Register routes in app (AC: #1)
  - [ ] 4.1: Import revenue router in `crates/qa-pms-api/src/app.rs`
  - [ ] 4.2: Add revenue router to main API router
  - [ ] 4.3: Verify routes registered correctly
  - [ ] 4.4: Update OpenAPI documentation tags

- [ ] Task 5: Add dependencies (AC: #1)
  - [ ] 5.1: Add `qa-pms-revenue` dependency to `crates/qa-pms-api/Cargo.toml` (if not already added)
  - [ ] 5.2: Verify workspace compiles with new dependency

- [ ] Task 6: Add response types to OpenAPI schema (AC: #1)
  - [ ] 6.1: Ensure `RevenueImpact` and `RevenueBreakdown` types have `ToSchema` derive
  - [ ] 6.2: Include types in OpenAPI schema generation
  - [ ] 6.3: Verify OpenAPI documentation includes revenue endpoints

---

## Files to Create

| File | Changes |
|------|---------|
| `crates/qa-pms-api/src/routes/revenue.rs` | Create revenue API routes with GET /api/v1/revenue/impact endpoint |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-api/src/app.rs` | Add revenue router to main API router |
| `crates/qa-pms-api/src/routes/mod.rs` | Export revenue module (if using mod.rs pattern) |
| `crates/qa-pms-api/Cargo.toml` | Verify `qa-pms-revenue` dependency exists (should be added in Story 23.1 or 23.2) |

---

## Implementation Notes

### Route Pattern

Follow existing patterns from `routes/dashboard.rs`:

```rust
// crates/qa-pms-api/src/routes/revenue.rs
use axum::{extract::Query, extract::State, routing::get, Json, Router};
use qa_pms_revenue::RevenueImpact;
use qa_pms_dashboard::parse_period;
use crate::app::AppState;
use qa_pms_core::error::ApiError;

type ApiResult<T> = Result<T, ApiError>;

pub fn router() -> Router<AppState> {
    Router::new().route("/api/v1/revenue/impact", get(get_revenue_impact))
}

#[derive(Debug, Deserialize)]
pub struct RevenueImpactQuery {
    #[serde(default = "default_period")]
    pub period: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/revenue/impact",
    params(
        ("period" = String, Query, description = "Period: 7d, 30d, 90d, 1y")
    ),
    responses(
        (status = 200, description = "Revenue impact data", body = RevenueImpact),
        (status = 500, description = "Internal server error")
    ),
    tag = "Revenue"
)]
pub async fn get_revenue_impact(
    State(state): State<AppState>,
    Query(query): Query<RevenueImpactQuery>,
) -> ApiResult<Json<RevenueImpact>> {
    let period_days = parse_period(&query.period);
    // Call calculation engine
    let impact = calculate_revenue_impact(&state.db, period_days).await?;
    Ok(Json(impact))
}
```

### Integration with Calculation Engine

- Use `RevenueCalculator` from Story 23.2
- Call `calculate_revenue_impact(period)` function
- Pass database pool if calculator needs to query integration health data
- Handle errors: map calculation errors to `ApiError`

### Period Parsing

- Use `qa_pms_dashboard::parse_period()` function (consistent with other endpoints)
- Supported periods: "7d", "30d", "90d", "1y"
- Default to "30d" if period parameter missing or invalid

### Error Handling

Map calculation errors to HTTP status codes:
- **500 Internal Server Error**: Missing config, calculation errors, database errors
- **400 Bad Request**: Invalid period parameter
- Use `ApiError` from `qa-pms-core` for consistent error responses

### OpenAPI Documentation

- Use `utoipa::path` macro for endpoint documentation
- Ensure `RevenueImpact` and `RevenueBreakdown` types have `ToSchema` derive
- Include in OpenAPI schema generation (automatic if types have `ToSchema`)

### Route Registration

In `app.rs`, add revenue router:

```rust
// crates/qa-pms-api/src/app.rs
use crate::routes::revenue;

// In router() function:
.merge(revenue::router())
```

### Performance

- Response time target: < 500ms
- Calculation engine should handle performance (from Story 23.2)
- Consider caching if calculations are expensive (future optimization)

---

## Testing Strategy

### Unit Tests

- **Route Handlers**: Test `get_revenue_impact()` handler with mock data
- **Query Parameters**: Test period parsing (7d, 30d, 90d, 1y, invalid)
- **Error Handling**: Test error cases (missing config, calculation errors)

### Integration Tests

- **API Endpoints**: Test GET /api/v1/revenue/impact endpoint
- **Response Format**: Verify response structure matches `RevenueImpact` type
- **Performance**: Verify response time < 500ms
- **OpenAPI**: Verify OpenAPI documentation includes revenue endpoints

### Manual Tests

- Test endpoint with different period parameters
- Verify response structure and values are correct
- Test error cases (missing config, missing health data)
- Verify OpenAPI documentation in Swagger UI

---

## Success Metrics

- API endpoint functional and returns correct data
- Response time < 500ms
- OpenAPI documentation complete
- Error handling works correctly
- Ready for Story 23.4 (Revenue Impact KPI Cards)

---

## Context & Dependencies

**Dependencies:**
- Story 23.2: Revenue Impact Calculation Engine (provides calculation logic)

**Enables:**
- Story 23.4: Revenue Impact KPI Cards (needs API endpoint)

**Related Documentation:**
- Architecture Document: `_bmad-output/planning-artifacts/architecture-observability-pms-integrations-2026-01-10.md`
- Epic Document: `_bmad-output/planning-artifacts/epics-observability-pms-integrations-2026-01-10.md`
- PRD: `_bmad-output/planning-artifacts/prd-observability-pms-integrations-2026-01-10.md` (FR-2.4)

**Project Context:**
- See `_bmad-output/planning-artifacts/project-context.md` for Rust patterns, API patterns
- Follow existing patterns from `routes/dashboard.rs`, `routes/pm_dashboard.rs`
- Use `ApiError` from `qa-pms-core` for error handling
- Use `utoipa` for OpenAPI documentation

---

## Dev Notes

### Key Implementation Details

1. **Route Pattern**: Follow existing route patterns (dashboard, pm_dashboard)
2. **Period Parsing**: Use `qa_pms_dashboard::parse_period()` for consistency
3. **Error Handling**: Map calculation errors to appropriate HTTP status codes
4. **OpenAPI**: Use `utoipa::path` macro for endpoint documentation
5. **Performance**: Response time target < 500ms (calculation engine handles this)

### Future Endpoints

- `GET /api/v1/revenue/config` (admin only) - marked as future/optional
- `PUT /api/v1/revenue/config` (admin only) - marked as future/optional
- Can be implemented in Phase 2 if needed

---

**Story Status:** `ready-for-dev`  
**Last Updated:** 2026-01-11  
**Next Review:** When moving to `in-progress`
