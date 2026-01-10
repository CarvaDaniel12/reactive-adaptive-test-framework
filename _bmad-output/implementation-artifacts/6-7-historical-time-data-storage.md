# Story 6.7: Historical Time Data Storage

Status: done

## Story

As a developer,
I want historical time data aggregated,
So that dashboards can show trends.

## Acceptance Criteria

1. **Given** time sessions are recorded
   **When** a workflow completes
   **Then** aggregated data is stored

2. **Given** aggregation runs
   **When** data is computed
   **Then** total time by ticket type is calculated

3. **Given** aggregation runs
   **When** data is computed
   **Then** average time per step is calculated

4. **Given** aggregation runs
   **When** data is computed
   **Then** user's historical averages are calculated

5. **Given** aggregation runs
   **When** data is computed
   **Then** trends over time are computed

6. **Given** aggregated data exists
   **When** queried
   **Then** data is queryable by date range

7. **Given** aggregated data exists
   **When** dashboards render
   **Then** data is used for dashboard metrics (Epic 8)

8. **Given** data retention policy
   **When** old data is checked
   **Then** old sessions are retained per NFR-REL-04 (30 days minimum)

## Tasks

- [x] Task 1: Create aggregates tables (daily/step/user + gap alerts)
- [x] Task 2: Create aggregation service (SQL functions + Rust wrapper)
- [x] Task 3: Trigger aggregation on workflow completion
- [x] Task 4: Create date range query methods
- [ ] Task 5: Implement data retention cleanup scheduling job (function exists; scheduling is follow-up)
- [x] Task 6: Create historical/aggregations API endpoints (history/trend/averages/alerts)

## Review Follow-ups (AI)

- [x] [AI-Review][MED] Update this story file (tasks/status/dev record) to match the implemented aggregates module and migrations [_bmad-output/implementation-artifacts/6-7-historical-time-data-storage.md]

## Dev Notes

### Database Schema

Implemented in:
- `migrations/20260104000002_time_tracking_schema.sql` (raw sessions + estimates + pause events)
- `migrations/20260105000002_time_aggregates_schema.sql` (aggregates + gap alerts + SQL functions)

### Aggregation Service

Implemented in `crates/qa-pms-time/src/aggregates.rs`:
- `record_workflow_completion()` calls SQL functions (`update_*`) to keep aggregates in sync
- query helpers: `get_historical_summary`, `get_trend_data`, `get_user_averages`, `get_daily_aggregates`, `get_undismissed_alerts`

### Data Retention Job

Implemented in DB + Rust wrapper:
- `migrations/20260105000002_time_aggregates_schema.sql` defines `cleanup_old_aggregates()`
- `crates/qa-pms-time/src/aggregates.rs` provides `cleanup_old_data()`

Scheduling the cleanup periodically is tracked as a follow-up (Task 5).

### API Endpoint

Implemented endpoints:
- `GET /api/v1/time/history/{user_id}` (summary)
- `GET /api/v1/time/history/{user_id}/trend`
- `GET /api/v1/time/history/{user_id}/averages`
- `GET /api/v1/time/history/{user_id}/alerts`

### References

- [Source: epics.md#Story 6.7]
- [NFR: NFR-REL-04 - 30 days data retention]
- [Dependency: Epic 8 - QA Individual Dashboard]

## Completion Notes List

- Aggregates schema implemented via `time_daily_aggregates`, `time_step_averages`, `time_user_averages`, `time_gap_alerts`
- Aggregation logic implemented via PostgreSQL functions (`update_daily_aggregate`, `update_step_average`, `update_user_average`)
- Aggregation is triggered on workflow completion (`POST /api/v1/workflows/{id}/complete`)
- Dashboards can query trends/summary via `/api/v1/dashboard` and `/api/v1/time/history/*`
