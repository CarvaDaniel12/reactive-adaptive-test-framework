# Story 17.5: Audit Log Retention and Archival

Status: ready-for-dev

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

**As a** System Administrator  
**I want** automated audit log retention and archival  
**So that** I can manage storage costs while maintaining compliance with retention requirements

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 17.5 |
| Epic | Epic 17: Audit Logging |
| Sprint | Sprint 3: Audit Log Management |
| Priority | P0 (Critical) |
| Estimated Days | 1 |
| Dependencies | Story 17.1 (Comprehensive Audit Log Storage) |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create audit log retention policy
   - Configurable retention period (default: 90 days)
   - Archive logs older than retention period
   - Delete archived logs after archive period (future/optional)

2. Create archival mechanism
   - Archive logs to separate table or file storage (future)
   - Mark logs as archived in database (immediate)
   - Support manual archival trigger

3. Create retention cleanup job
   - Background task to archive old logs
   - Run periodically (daily, configurable)
   - Use Tokio task for background execution

4. Support manual archival
   - API endpoint to trigger archival
   - Support archival by date range
   - Return archival statistics

---

## Acceptance Criteria

- [ ] **Given** retention policy exists  
  **When** audit logs are older than retention period  
  **Then** logs are archived automatically

- [ ] **Given** archival mechanism exists  
  **When** archiving logs  
  **Then** logs are marked as archived in database

- [ ] **Given** retention cleanup job exists  
  **When** job runs periodically  
  **Then** old logs are archived automatically

- [ ] **Given** manual archival endpoint exists  
  **When** triggering archival by date range  
  **Then** logs in date range are archived

- [ ] **Given** archival process exists  
  **When** archiving logs  
  **Then** archival statistics are returned (count, date range)

---

## Tasks / Subtasks

- [ ] Task 1: Add archival fields to audit_logs table (AC: #2)
  - [ ] 1.1: Create migration to add `archived_at TIMESTAMPTZ` column
  - [ ] 1.2: Add index on `archived_at` (WHERE archived_at IS NULL) for active logs
  - [ ] 1.3: Update repository types to include `archived_at` field

- [ ] Task 2: Create retention configuration (AC: #1)
  - [ ] 2.1: Add retention period to configuration (default: 90 days)
  - [ ] 2.2: Create `AuditRetentionConfig` struct in `src/types.rs`
  - [ ] 2.3: Support configuration via environment variable or config file

- [ ] Task 3: Implement archival methods (AC: #2)
  - [ ] 3.1: Create `archive_logs` method in `AuditLogRepository`
  - [ ] 3.2: Update `archived_at` timestamp for logs matching criteria
  - [ ] 3.3: Support archival by date range
  - [ ] 3.4: Return count of archived logs

- [ ] Task 4: Create retention cleanup job (AC: #3)
  - [ ] 4.1: Create `AuditRetentionService` in `src/service.rs`
  - [ ] 4.2: Implement `cleanup_old_logs` method
  - [ ] 4.3: Use Tokio task for background execution
  - [ ] 4.4: Run periodically (daily, configurable)
  - [ ] 4.5: Log archival statistics

- [ ] Task 5: Integrate cleanup job with API (AC: #3)
  - [ ] 5.1: Add cleanup job to API startup (future story: routes)
  - [ ] 5.2: Schedule periodic execution
  - [ ] 5.3: Handle errors gracefully (log warnings, don't crash)

- [ ] Task 6: Add unit and integration tests (AC: #1, #2, #3, #4)
  - [ ] 6.1: Test archival by date range
  - [ ] 6.2: Test automatic cleanup job
  - [ ] 6.3: Test archival statistics
  - [ ] 6.4: Test configuration loading

---

## Files to Create

| File | Changes |
|------|---------|
| `migrations/YYYYMMDDHHMMSS_add_audit_log_archival.sql` | Add archival fields to audit_logs table |
| `crates/qa-pms-audit/src/service.rs` | Add AuditRetentionService (or extend existing service) |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-audit/src/repository.rs` | Add archive_logs method |
| `crates/qa-pms-audit/src/types.rs` | Add AuditRetentionConfig, update AuditLog type |

---

## Dev Notes

### Retention Policy

**Default Retention:**
- Retention period: 90 days (configurable)
- Archive logs older than retention period
- Keep archived logs for compliance (future: move to cold storage)

**Configuration:**
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct AuditRetentionConfig {
    pub retention_days: u32, // Default: 90
    pub archive_enabled: bool, // Default: true
}
```

### Archival Mechanism

**Archival Strategy:**
- Mark logs as archived (`archived_at` timestamp) - immediate
- Archive to separate table (future enhancement)
- Archive to file storage (future enhancement)

**Database Schema Update:**
```sql
ALTER TABLE audit_logs
ADD COLUMN IF NOT EXISTS archived_at TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS idx_audit_logs_archived_at 
ON audit_logs(archived_at) WHERE archived_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_audit_logs_active 
ON audit_logs(occurred_at DESC) WHERE archived_at IS NULL;
```

**Archival Method:**
```rust
pub async fn archive_logs(
    &self,
    before_date: DateTime<Utc>,
) -> Result<i64, AuditError> {
    sqlx::query!(
        r#"
        UPDATE audit_logs
        SET archived_at = NOW()
        WHERE occurred_at < $1
          AND archived_at IS NULL
        "#,
        before_date
    )
    .execute(&self.pool)
    .await
    .map_db_err()?;
    
    // Return count (requires separate COUNT query)
}
```

### Cleanup Job

**Background Task:**
```rust
pub struct AuditRetentionService {
    repository: AuditLogRepository,
    config: AuditRetentionConfig,
}

impl AuditRetentionService {
    pub async fn cleanup_old_logs(&self) -> Result<i64, AuditError> {
        let cutoff_date = Utc::now() - chrono::Duration::days(self.config.retention_days as i64);
        self.repository.archive_logs(cutoff_date).await
    }

    pub fn spawn_cleanup_task(self, interval: Duration) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval);
            loop {
                interval.tick().await;
                match self.cleanup_old_logs().await {
                    Ok(count) => info!(archived_logs = count, "Audit log cleanup completed"),
                    Err(e) => warn!(error = %e, "Audit log cleanup failed"),
                }
            }
        })
    }
}
```

### Project Structure Notes

**Service Pattern:**
- Follow existing service patterns
- Use `AuditLogRepository` for data access
- Use Tokio tasks for background execution

**Configuration:**
- Support environment variable: `AUDIT_RETENTION_DAYS` (default: 90)
- Support config file (future)
- Store in `qa-pms-config` or `AuditRetentionConfig`

**Performance Considerations:**
- Use efficient UPDATE query with WHERE clause
- Index on `archived_at IS NULL` for active logs
- Run cleanup job during off-peak hours (configurable)

### Testing Standards

**Unit Tests:**
- Test archival logic
- Test retention configuration
- Test date calculations

**Integration Tests:**
- Test archival with real database
- Test cleanup job execution
- Test archival statistics

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 17, Story 17.5)
- Dependency: Story 17.1 (Comprehensive Audit Log Storage) - must be completed first
- Service Pattern: `qa-intelligent-pms/crates/qa-pms-integration-health/src/service.rs` (reference)
- Background Task Pattern: `qa-intelligent-pms/crates/qa-pms-api/src/health_scheduler.rs` (reference)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
