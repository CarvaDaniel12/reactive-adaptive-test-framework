# Story 17.7: Compliance Export for Audits

Status: ready-for-dev

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

**As a** Compliance Officer  
**I want** to export audit logs in compliance formats  
**So that** I can provide audit evidence to auditors and meet regulatory requirements

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 17.7 |
| Epic | Epic 17: Audit Logging |
| Sprint | Sprint 3: Audit Log Management |
| Priority | P0 (Critical) |
| Estimated Days | 2 |
| Dependencies | Story 17.4 (Audit Report Generation) |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create compliance export endpoint
   - Endpoint: `GET /api/v1/audit/export/compliance`
   - Support CSV and JSON export formats
   - Include all audit fields for compliance
   - Support filtering by date range, event type, user

2. Implement compliance report format
   - Include report metadata: title, date range, generated_at, filters
   - Include audit trail: all matching audit log entries
   - Include summary: event counts, user activity, security events
   - Format: structured for audit review

3. Support export options
   - Export format: CSV, JSON
   - Date range selection
   - Event type filtering
   - User filtering
   - Include/exclude fields

4. Compliance features
   - Tamper-evident export (future: digital signatures)
   - Complete audit trail (no omissions)
   - Timestamp precision (ISO 8601 with timezone)
   - Export metadata (who exported, when)

---

## Acceptance Criteria

- [ ] **Given** compliance export endpoint exists  
  **When** GET /api/v1/audit/export/compliance?format=csv  
  **Then** CSV file is generated with all audit fields

- [ ] **Given** compliance export endpoint exists  
  **When** GET /api/v1/audit/export/compliance?format=json  
  **Then** JSON file is generated with complete audit trail

- [ ] **Given** compliance export exists  
  **When** exporting with date range  
  **Then** export includes all audit logs in date range

- [ ] **Given** compliance export exists  
  **When** exporting with filters  
  **Then** export includes all matching audit logs

- [ ] **Given** compliance export exists  
  **When** export is generated  
  **Then** export includes metadata (title, date range, generated_at, filters)

- [ ] **Given** compliance export exists  
  **When** export is generated  
  **Then** export generation time < 10 seconds for typical datasets

---

## Tasks / Subtasks

- [ ] Task 1: Create compliance export endpoint (AC: #1)
  - [ ] 1.1: Add `export_compliance` handler function to `routes/audit.rs`
  - [ ] 1.2: Extract query parameters (format, filters, date range)
  - [ ] 1.3: Use `AuditReportService` to generate compliance report
  - [ ] 1.4: Generate export file based on format (CSV or JSON)
  - [ ] 1.5: Return file with appropriate content-type headers
  - [ ] 1.6: Add `utoipa::path` macro for OpenAPI documentation

- [ ] Task 2: Implement CSV export (AC: #2)
  - [ ] 2.1: Create `export_compliance_csv` method
  - [ ] 2.2: Include report metadata as CSV header comments
  - [ ] 2.3: Include CSV headers row
  - [ ] 2.4: Include all audit log entries as CSV rows
  - [ ] 2.5: Escape CSV special characters correctly
  - [ ] 2.6: Format timestamps as ISO 8601 strings

- [ ] Task 3: Implement JSON export (AC: #2)
  - [ ] 3.1: Create `export_compliance_json` method
  - [ ] 3.2: Structure JSON with metadata and audit trail
  - [ ] 3.3: Include report metadata (title, date range, generated_at, filters)
  - [ ] 3.4: Include audit trail array with all matching logs
  - [ ] 3.5: Include summary statistics
  - [ ] 3.6: Use `serde_json::to_string_pretty` for readability

- [ ] Task 4: Create compliance report structure (AC: #2)
  - [ ] 4.1: Create `ComplianceExport` struct in `src/types.rs`
  - [ ] 4.2: Include metadata: title, date_range, generated_at, filters, exported_by
  - [ ] 4.3: Include audit_trail: Vec<AuditLog>
  - [ ] 4.4: Include summary: event counts, user activity, security events
  - [ ] 4.5: Add `serde` serialization

- [ ] Task 5: Add export metadata tracking (AC: #4)
  - [ ] 5.1: Log export events to audit_logs table (self-auditing)
  - [ ] 5.2: Include export metadata in export file
  - [ ] 5.3: Track who exported, when, and filters used

- [ ] Task 6: Add unit and integration tests (AC: #1, #2, #3, #4, #5, #6)
  - [ ] 6.1: Test CSV export format
  - [ ] 6.2: Test JSON export format
  - [ ] 6.3: Test export with filters
  - [ ] 6.4: Test export with date range
  - [ ] 6.5: Test export metadata
  - [ ] 6.6: Test export performance (< 10 seconds)

---

## Files to Create

| File | Changes |
|------|---------|
| None | Extend existing files |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-api/src/routes/audit.rs` | Add export_compliance endpoint |
| `crates/qa-pms-audit/src/service.rs` | Add export_compliance_csv and export_compliance_json methods |
| `crates/qa-pms-audit/src/types.rs` | Add ComplianceExport type |

---

## Dev Notes

### Export Endpoint

**Compliance Export:**
```rust
#[utoipa::path(
    get,
    path = "/api/v1/audit/export/compliance",
    params(
        ("format" = String, Query, description = "Export format: csv or json"),
        ("startDate" = Option<String>, Query, description = "Start date (ISO 8601)"),
        ("endDate" = Option<String>, Query, description = "End date (ISO 8601)"),
        ("eventType" = Option<String>, Query, description = "Filter by event type"),
        ("userId" = Option<Uuid>, Query, description = "Filter by user ID")
    ),
    responses(
        (status = 200, description = "Compliance export file"),
        (status = 400, description = "Invalid export format"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Audit"
)]
pub async fn export_compliance(
    State(state): State<AppState>,
    Query(query): Query<ComplianceExportQuery>,
) -> ApiResult<impl IntoResponse> {
    // Generate compliance export
    // Return file with appropriate content-type
}
```

### CSV Export Format

**CSV Structure:**
```csv
# Compliance Audit Export
# Generated: 2026-01-11T12:00:00Z
# Date Range: 2026-01-01 to 2026-01-11
# Filters: eventType=Authentication
# Exported By: admin@example.com

id,user_id,event_type,event_category,action,resource_type,resource_id,ip_address,user_agent,success,message,occurred_at
...
```

### JSON Export Format

**JSON Structure:**
```json
{
  "metadata": {
    "title": "Compliance Audit Export",
    "dateRange": {
      "start": "2026-01-01T00:00:00Z",
      "end": "2026-01-11T23:59:59Z"
    },
    "generatedAt": "2026-01-11T12:00:00Z",
    "filters": {
      "eventType": "Authentication"
    },
    "exportedBy": "admin@example.com"
  },
  "summary": {
    "totalEvents": 1234,
    "eventsByCategory": {...},
    "eventsByType": {...},
    "eventsByUser": {...}
  },
  "auditTrail": [
    {
      "id": "...",
      "userId": "...",
      "eventType": "Authentication",
      ...
    }
  ]
}
```

### Project Structure Notes

**Export Patterns:**
- Follow existing export patterns (`routes/pm_dashboard.rs::export_pm_dashboard`)
- Use CSV generation library or manual CSV generation
- Use `serde_json` for JSON export

**Compliance Features:**
- Complete audit trail (no omissions)
- Timestamp precision (ISO 8601 with timezone)
- Export metadata (self-auditing)
- Tamper-evident format (future: digital signatures)

### Testing Standards

**Unit Tests:**
- Test CSV export format
- Test JSON export format
- Test export metadata

**Integration Tests:**
- Test export with real database
- Test export with filters
- Test export performance (< 10 seconds)

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 17, Story 17.7)
- Dependency: Story 17.4 (Audit Report Generation) - must be completed first
- Export Patterns: `qa-intelligent-pms/crates/qa-pms-api/src/routes/pm_dashboard.rs` (reference)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
