# Story 16.3: Report Scheduling and Automation

Status: ready-for-dev

## Story

**As a** QA Lead  
**I want** to schedule automated report generation  
**So that** I can receive regular reports without manual intervention

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 16.3 |
| Epic | Epic 16: Reports Enhancement |
| Sprint | Sprint 1: Report Automation |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Epic 7 (Reporting & Documentation) - Complete ✅ |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create report schedules database schema
   - Create `report_schedules` table
   - Columns: `id` (UUID), `name` (VARCHAR 255), `schedule_type` (VARCHAR 50), `schedule_config` (JSONB), `template_id` (UUID), `filters` (JSONB), `enabled` (BOOLEAN), `last_run_at`, `next_run_at`, `created_at`, `updated_at`

2. Create schedule types
   - Support: daily, weekly, monthly
   - Support: cron expression (future)
   - Store schedule configuration in JSONB

3. Create schedule service
   - Background task to check schedules and generate reports
   - Use Tokio task for periodic execution
   - Generate reports based on schedule configuration

4. Create schedule API endpoints
   - `GET /api/v1/reports/schedules` - list schedules
   - `POST /api/v1/reports/schedules` - create schedule
   - `PUT /api/v1/reports/schedules/:id` - update schedule
   - `DELETE /api/v1/reports/schedules/:id` - delete schedule
   - `POST /api/v1/reports/schedules/:id/run` - manual trigger

---

## Acceptance Criteria

- [ ] **Given** report schedule exists  
  **When** schedule time arrives  
  **Then** report is generated automatically

- [ ] **Given** schedule API exists  
  **When** creating schedule  
  **Then** schedule is saved and activated

- [ ] **Given** schedule service exists  
  **When** checking schedules  
  **Then** due schedules trigger report generation

---

## Tasks / Subtasks

- [ ] Task 1: Create report_schedules table
- [ ] Task 2: Create schedule types
- [ ] Task 3: Create schedule service
- [ ] Task 4: Create schedule API endpoints
- [ ] Task 5: Integrate schedule service with API startup

---

## Files to Create

| File | Changes |
|------|---------|
| `migrations/YYYYMMDDHHMMSS_create_report_schedules_table.sql` | Create report schedules table |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-api/src/routes/reports.rs` | Add schedule endpoints |
| `crates/qa-pms-api/src/app.rs` | Add schedule service to startup |

---

## Dev Notes

### Database Schema

```sql
CREATE TABLE IF NOT EXISTS report_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    schedule_type VARCHAR(50) NOT NULL, -- 'daily', 'weekly', 'monthly', 'cron'
    schedule_config JSONB NOT NULL, -- ScheduleConfig structure
    template_id UUID REFERENCES report_templates(id),
    filters JSONB, -- ReportFilters structure
    enabled BOOLEAN NOT NULL DEFAULT true,
    last_run_at TIMESTAMPTZ,
    next_run_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### Schedule Service

```rust
pub struct ReportScheduleService {
    pool: PgPool,
}

impl ReportScheduleService {
    pub async fn check_and_run_schedules(&self) -> Result<Vec<Uuid>, ScheduleError> {
        // Query due schedules
        // Generate reports
        // Update last_run_at and next_run_at
    }
}
```

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 16, Story 16.3)
- Dependency: Epic 7 (Reporting & Documentation) - must be complete
- Background Task Pattern: `qa-intelligent-pms/crates/qa-pms-api/src/health_scheduler.rs` (reference)

---

## Dev Agent Record

### File List
