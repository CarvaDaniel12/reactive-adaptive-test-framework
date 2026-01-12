# Story 17.6: Real-time Audit Monitoring Dashboard

Status: ready-for-dev

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

**As a** Security Administrator  
**I want** a real-time audit monitoring dashboard  
**So that** I can monitor security events, track user activities, and identify suspicious patterns in real-time

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 17.6 |
| Epic | Epic 17: Audit Logging |
| Sprint | Sprint 3: Audit Log Management |
| Priority | P0 (Critical) |
| Estimated Days | 2 |
| Dependencies | Story 17.3 (Audit Log Search and Filtering) |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create audit API endpoints
   - Create `routes/audit.rs` in `qa-pms-api`
   - Endpoint: `GET /api/v1/audit/logs` (search with filters)
   - Endpoint: `GET /api/v1/audit/summary` (summary statistics)
   - Endpoint: `GET /api/v1/audit/realtime` (recent events, WebSocket future)

2. Create audit dashboard page
   - Create `AuditDashboardPage.tsx` component
   - Timeline visualization showing audit events over time
   - Event type breakdown (pie chart or table)
   - Recent events table (last 100 events)
   - Filters: date range, event type, user, category

3. Create audit dashboard hook
   - Create `useAuditLogs.ts` hook
   - Use React Query for data fetching
   - Support real-time refresh (polling every 10 seconds)
   - Support filtering

4. Integration with routing
   - Add route `/dashboard/audit` to `App.tsx`
   - Support query parameters for filters

---

## Acceptance Criteria

- [ ] **Given** audit API endpoint exists  
  **When** GET /api/v1/audit/logs  
  **Then** returns audit logs with filters and pagination

- [ ] **Given** audit API endpoint exists  
  **When** GET /api/v1/audit/summary  
  **Then** returns summary statistics (event counts by category, type, user)

- [ ] **Given** audit dashboard exists  
  **When** loading page  
  **Then** timeline visualization is displayed showing audit events over time

- [ ] **Given** audit dashboard exists  
  **When** viewing summary  
  **Then** event type breakdown is displayed

- [ ] **Given** audit dashboard exists  
  **When** viewing recent events  
  **Then** recent events table is displayed (last 100 events)

- [ ] **Given** audit dashboard exists  
  **When** filtering by date range  
  **Then** data updates for selected date range

- [ ] **Given** audit dashboard exists  
  **When** real-time refresh enabled  
  **Then** data refreshes automatically every 10 seconds

---

## Tasks / Subtasks

- [ ] Task 1: Create audit API routes (AC: #1)
  - [ ] 1.1: Create `crates/qa-pms-api/src/routes/audit.rs`
  - [ ] 1.2: Add module declaration in `crates/qa-pms-api/src/routes/mod.rs`
  - [ ] 1.3: Implement `router()` function returning `Router<AppState>`
  - [ ] 1.4: Add routes:
    - `GET /api/v1/audit/logs` - search audit logs
    - `GET /api/v1/audit/summary` - summary statistics
    - `GET /api/v1/audit/realtime` - recent events (last 100)

- [ ] Task 2: Implement search audit logs endpoint (AC: #1)
  - [ ] 2.1: Create `search_audit_logs` handler function
  - [ ] 2.2: Extract query parameters (filters, page, page_size)
  - [ ] 2.3: Use `AuditLogRepository` to search logs
  - [ ] 2.4: Return paginated results
  - [ ] 2.5: Add `utoipa::path` macro for OpenAPI documentation

- [ ] Task 3: Implement summary endpoint (AC: #2)
  - [ ] 3.1: Create `get_audit_summary` handler function
  - [ ] 3.2: Use `AuditReportService` to generate summary
  - [ ] 3.3: Return summary statistics
  - [ ] 3.4: Add `utoipa::path` macro for OpenAPI documentation

- [ ] Task 4: Implement realtime endpoint (AC: #1)
  - [ ] 4.1: Create `get_recent_audit_logs` handler function
  - [ ] 4.2: Query recent audit logs (last 100, ordered by occurred_at DESC)
  - [ ] 4.3: Return recent events
  - [ ] 4.4: Add `utoipa::path` macro for OpenAPI documentation

- [ ] Task 5: Add routes to app.rs (AC: #1)
  - [ ] 5.1: Import `routes::audit` module in `app.rs`
  - [ ] 5.2: Add `.merge(routes::audit::router())` to main router

- [ ] Task 6: Add dependencies (AC: #1)
  - [ ] 6.1: Add `qa-pms-audit = { workspace = true }` to `crates/qa-pms-api/Cargo.toml`

- [ ] Task 7: Create audit dashboard hook (AC: #3)
  - [ ] 7.1: Create `frontend/src/hooks/useAuditLogs.ts`
  - [ ] 7.2: Implement `useAuditLogs` hook with React Query
  - [ ] 7.3: Support filtering by date range, event type, user, category
  - [ ] 7.4: Support real-time refresh (polling every 10 seconds)
  - [ ] 7.5: Create `useAuditSummary` hook for summary statistics

- [ ] Task 8: Create AuditDashboardPage component (AC: #2, #4, #5, #6, #7)
  - [ ] 8.1: Create `frontend/src/pages/Dashboard/AuditDashboardPage.tsx`
  - [ ] 8.2: Use `useSearchParams` for URL query parameters
  - [ ] 8.3: Use `useAuditLogs` and `useAuditSummary` hooks
  - [ ] 8.4: Implement filter handlers that update URL query parameters
  - [ ] 8.5: Render timeline visualization (use existing chart components or create new)
  - [ ] 8.6: Render event type breakdown (pie chart or table)
  - [ ] 8.7: Render recent events table
  - [ ] 8.8: Add loading and error states
  - [ ] 8.9: Use `useLayoutStore` for page title

- [ ] Task 9: Add route to App.tsx (AC: #4)
  - [ ] 9.1: Import `AuditDashboardPage` in `frontend/src/App.tsx`
  - [ ] 9.2: Add route `/dashboard/audit` to router

- [ ] Task 10: Add unit and integration tests (AC: #1, #2, #3, #4, #5, #6, #7)
  - [ ] 10.1: Test API endpoints with various filters
  - [ ] 10.2: Test dashboard component rendering
  - [ ] 10.3: Test hooks data fetching
  - [ ] 10.4: Test filter handlers
  - [ ] 10.5: Test real-time refresh

---

## Files to Create

| File | Changes |
|------|---------|
| `crates/qa-pms-api/src/routes/audit.rs` | Create audit API routes |
| `frontend/src/pages/Dashboard/AuditDashboardPage.tsx` | Create audit dashboard page |
| `frontend/src/hooks/useAuditLogs.ts` | Create audit logs hook |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-api/src/routes/mod.rs` | Add `pub mod audit;` |
| `crates/qa-pms-api/src/app.rs` | Add `.merge(routes::audit::router())` to router |
| `crates/qa-pms-api/Cargo.toml` | Add `qa-pms-audit = { workspace = true }` dependency |
| `frontend/src/App.tsx` | Add route `/dashboard/audit` |

---

## Dev Notes

### API Endpoint Patterns

**Router Structure:**
```rust
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/audit/logs", get(search_audit_logs))
        .route("/api/v1/audit/summary", get(get_audit_summary))
        .route("/api/v1/audit/realtime", get(get_recent_audit_logs))
}
```

**Search Endpoint:**
```rust
#[utoipa::path(
    get,
    path = "/api/v1/audit/logs",
    params(
        ("userId" = Option<Uuid>, Query, description = "Filter by user ID"),
        ("eventType" = Option<String>, Query, description = "Filter by event type"),
        ("eventCategory" = Option<String>, Query, description = "Filter by event category"),
        ("startDate" = Option<String>, Query, description = "Start date (ISO 8601)"),
        ("endDate" = Option<String>, Query, description = "End date (ISO 8601)"),
        ("page" = Option<u32>, Query, description = "Page number (default: 1)"),
        ("pageSize" = Option<u32>, Query, description = "Page size (default: 20, max: 100)")
    ),
    responses(
        (status = 200, description = "Audit logs", body = AuditLogSearchResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Audit"
)]
pub async fn search_audit_logs(
    State(state): State<AppState>,
    Query(query): Query<AuditLogQuery>,
) -> ApiResult<Json<AuditLogSearchResponse>> {
    // Implementation
}
```

### Dashboard Component

**AuditDashboardPage:**
```typescript
export function AuditDashboardPage() {
  const [searchParams, setSearchParams] = useSearchParams();
  const { setPageTitle } = useLayoutStore();
  
  const startDate = searchParams.get("startDate");
  const endDate = searchParams.get("endDate");
  const eventType = searchParams.get("eventType");
  
  const { data: logs, isLoading } = useAuditLogs({ startDate, endDate, eventType });
  const { data: summary } = useAuditSummary({ startDate, endDate });
  
  useEffect(() => {
    setPageTitle("Audit Logs", "Security and compliance monitoring");
    return () => setPageTitle("");
  }, [setPageTitle]);
  
  // Render timeline, summary, recent events
}
```

**useAuditLogs Hook:**
```typescript
export function useAuditLogs(filters: AuditLogFilters) {
  return useQuery({
    queryKey: ["audit-logs", filters],
    queryFn: () => fetchAuditLogs(filters),
    staleTime: 10_000, // 10 seconds
    refetchInterval: 10_000, // Auto-refresh every 10 seconds
  });
}
```

### Project Structure Notes

**API Patterns:**
- Follow existing route patterns (`routes/dashboard.rs`, `routes/support.rs`)
- Use `State<AppState>` for database pool access
- Use `ApiResult<T>` type alias for error handling
- Use `SqlxResultExt` for database error mapping

**Frontend Patterns:**
- Follow existing page patterns (`DashboardPage.tsx`, `PMDashboardPage.tsx`)
- Use React Query for data fetching
- Use React Router for navigation and URL state
- Use Zustand stores for global state

**Real-time Refresh:**
- Use React Query `refetchInterval` for polling
- Poll every 10 seconds for recent events
- Disable polling when page is not visible (future enhancement)

### Testing Standards

**Unit Tests:**
- Test API route handlers
- Test component rendering
- Test hooks data fetching

**Integration Tests:**
- Test endpoints with real database
- Test dashboard integration
- Test real-time refresh

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 17, Story 17.6)
- Dependency: Story 17.3 (Audit Log Search and Filtering) - must be completed first
- Route Patterns: `qa-intelligent-pms/crates/qa-pms-api/src/routes/dashboard.rs` (reference)
- Page Patterns: `qa-intelligent-pms/frontend/src/pages/Dashboard/DashboardPage.tsx` (reference)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
