# Story 17.4: Audit Report Generation

Status: ready-for-dev

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

**As a** Security Administrator  
**I want** to generate audit reports from audit logs  
**So that** I can analyze security events, track user activities, and generate compliance reports

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 17.4 |
| Epic | Epic 17: Audit Logging |
| Sprint | Sprint 2: Audit Log Querying |
| Priority | P0 (Critical) |
| Estimated Days | 2 |
| Dependencies | Story 17.3 (Audit Log Search and Filtering) |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create `AuditReportService` in `qa-pms-audit`
   - Generate audit reports from audit logs
   - Support multiple report types: summary, detailed, compliance
   - Support filtering by date range, user, event type
   - Export to CSV, JSON, PDF (future)

2. Implement report generation
   - Summary report: event counts by category, type, user
   - Detailed report: full audit log entries with filters
   - Compliance report: security events, failed actions, admin activities

3. Support export formats
   - CSV export (immediate)
   - JSON export (immediate)
   - PDF export (future/optional)

4. Report structure
   - Report metadata: title, date range, generated_at, filters
   - Report data: summary statistics, audit log entries
   - Report format: structured for export

---

## Acceptance Criteria

- [ ] **Given** audit report service exists  
  **When** generating summary report  
  **Then** report includes event counts by category, type, user

- [ ] **Given** audit report service exists  
  **When** generating detailed report  
  **Then** report includes full audit log entries matching filters

- [ ] **Given** audit report service exists  
  **When** generating compliance report  
  **Then** report includes security events, failed actions, admin activities

- [ ] **Given** audit report service exists  
  **When** exporting to CSV  
  **Then** CSV file is generated with correct format

- [ ] **Given** audit report service exists  
  **When** exporting to JSON  
  **Then** JSON file is generated with correct structure

- [ ] **Given** audit report service exists  
  **When** generating report  
  **Then** report generation time < 5 seconds

---

## Tasks / Subtasks

- [ ] Task 1: Create AuditReportService (AC: #1, #2, #3)
  - [ ] 1.1: Create `crates/qa-pms-audit/src/service.rs`
  - [ ] 1.2: Define `AuditReportService` struct with `AuditLogRepository`
  - [ ] 1.3: Implement `new()` constructor
  - [ ] 1.4: Implement `generate_summary_report` method
  - [ ] 1.5: Implement `generate_detailed_report` method
  - [ ] 1.6: Implement `generate_compliance_report` method

- [ ] Task 2: Implement summary report (AC: #1)
  - [ ] 2.1: Query audit logs with filters
  - [ ] 2.2: Aggregate event counts by category
  - [ ] 2.3: Aggregate event counts by type
  - [ ] 2.4: Aggregate event counts by user
  - [ ] 2.5: Aggregate event counts by action
  - [ ] 2.6: Return summary report structure

- [ ] Task 3: Implement detailed report (AC: #2)
  - [ ] 3.1: Query audit logs with filters (use repository)
  - [ ] 3.2: Return all matching audit log entries
  - [ ] 3.3: Include report metadata (filters, date range, total count)

- [ ] Task 4: Implement compliance report (AC: #3)
  - [ ] 4.1: Filter audit logs for security events
  - [ ] 4.2: Filter audit logs for failed actions (success = false)
  - [ ] 4.3: Filter audit logs for admin activities (event_category = Admin)
  - [ ] 4.4: Combine results into compliance report
  - [ ] 4.5: Include summary statistics

- [ ] Task 5: Implement CSV export (AC: #4)
  - [ ] 5.1: Create `export_to_csv` method
  - [ ] 5.2: Convert audit logs to CSV format
  - [ ] 5.3: Include CSV headers
  - [ ] 5.4: Escape CSV special characters

- [ ] Task 6: Implement JSON export (AC: #5)
  - [ ] 6.1: Create `export_to_json` method
  - [ ] 6.2: Serialize report to JSON
  - [ ] 6.3: Include report metadata

- [ ] Task 7: Create report types (AC: #1, #2, #3)
  - [ ] 7.1: Create `AuditReport` struct in `src/types.rs`
  - [ ] 7.2: Create `SummaryReport` struct
  - [ ] 7.3: Create `DetailedReport` struct
  - [ ] 7.4: Create `ComplianceReport` struct
  - [ ] 7.5: Add `serde` serialization
  - [ ] 7.6: Add `utoipa::ToSchema` for OpenAPI

- [ ] Task 8: Add unit and integration tests (AC: #1, #2, #3, #4, #5, #6)
  - [ ] 8.1: Test summary report generation
  - [ ] 8.2: Test detailed report generation
  - [ ] 8.3: Test compliance report generation
  - [ ] 8.4: Test CSV export
  - [ ] 8.5: Test JSON export
  - [ ] 8.6: Test report generation performance (< 5 seconds)

---

## Files to Create

| File | Changes |
|------|---------|
| `crates/qa-pms-audit/src/service.rs` | Create audit report service |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-audit/src/types.rs` | Add AuditReport, SummaryReport, DetailedReport, ComplianceReport types |
| `crates/qa-pms-audit/src/lib.rs` | Export service |

---

## Dev Notes

### Report Service Structure

**AuditReportService:**
```rust
pub struct AuditReportService {
    repository: AuditLogRepository,
}

impl AuditReportService {
    pub fn new(repository: AuditLogRepository) -> Self {
        Self { repository }
    }

    pub async fn generate_summary_report(
        &self,
        filters: AuditLogFilters,
    ) -> Result<SummaryReport, AuditError> {
        // Query audit logs
        // Aggregate by category, type, user, action
        // Return summary report
    }

    pub async fn generate_detailed_report(
        &self,
        filters: AuditLogFilters,
        page: u32,
        page_size: u32,
    ) -> Result<DetailedReport, AuditError> {
        // Query audit logs with pagination
        // Return detailed report
    }

    pub async fn generate_compliance_report(
        &self,
        filters: AuditLogFilters,
    ) -> Result<ComplianceReport, AuditError> {
        // Filter security events
        // Filter failed actions
        // Filter admin activities
        // Return compliance report
    }
}
```

### Report Types

**SummaryReport:**
```rust
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SummaryReport {
    pub metadata: ReportMetadata,
    pub summary: SummaryStatistics,
}

pub struct SummaryStatistics {
    pub total_events: i64,
    pub events_by_category: HashMap<AuditEventCategory, i64>,
    pub events_by_type: HashMap<AuditEventType, i64>,
    pub events_by_user: HashMap<Uuid, i64>,
    pub events_by_action: HashMap<AuditAction, i64>,
    pub failed_actions: i64,
    pub date_range: (DateTime<Utc>, DateTime<Utc>),
}
```

### Project Structure Notes

**Service Pattern:**
- Follow existing service patterns (`qa-pms-integration-health/src/service.rs`)
- Use `AuditLogRepository` for data access
- Handle business logic and aggregation

**Export Formats:**
- CSV: Use `csv` crate or manual CSV generation
- JSON: Use `serde_json`
- PDF: Future (use `printpdf` or similar)

**Performance Considerations:**
- Use efficient SQL aggregations (GROUP BY) for summary reports
- Use pagination for detailed reports
- Cache report metadata if needed

### Testing Standards

**Unit Tests:**
- Test report generation logic
- Test aggregation calculations
- Test export format generation

**Integration Tests:**
- Test with real database
- Test report generation performance (< 5 seconds)
- Test export formats

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 17, Story 17.4)
- Dependency: Story 17.3 (Audit Log Search and Filtering) - must be completed first
- Service Pattern: `qa-intelligent-pms/crates/qa-pms-integration-health/src/service.rs` (reference)
- Report Patterns: `qa-intelligent-pms/crates/qa-pms-api/src/routes/reports.rs` (reference)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
