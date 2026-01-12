# Code Review: Story 22-1 - Integration Health Database Schema

**Reviewer:** BMAD Code Review Agent  
**Date:** 2026-01-11  
**Story:** 22-1-integration-health-database-schema  
**Status:** `review` → Findings identified  
**Priority:** P0 (Foundation Story)

---

## Executive Summary

**Overall Assessment:** ⚠️ **NEEDS FIXES** - Migration file is functional but missing several critical patterns and validations that are standard in the project.

**Issues Found:** 8 issues (2 HIGH, 4 MEDIUM, 2 LOW)

**Migration Status:** ✅ Installed successfully (confirmed via `sqlx migrate info`)

---

## Review Methodology

Following BMAD adversarial code review workflow:
1. ✅ Story file loaded and parsed
2. ✅ Migration file reviewed against story requirements
3. ✅ Compared with existing migration patterns in project
4. ✅ Verified against Context7 best practices for SQLx/PostgreSQL
5. ✅ Cross-referenced with project architecture documentation

---

## Findings

### 🔴 HIGH Priority Issues

#### CR-HIGH-001: Missing `updated_at` Trigger for `integration_health` Table
**Severity:** HIGH  
**Category:** Pattern Compliance  
**Location:** `migrations/20260111030835_create_integration_health_tables.sql`

**Problem:**
The `integration_health` table has an `updated_at` column but no trigger to automatically update it on row updates. This is inconsistent with project patterns.

**Evidence:**
- `workflow_schema.sql` (20260104000001) creates triggers for all tables with `updated_at`
- `support_schema.sql` (20260105000004) creates triggers for `error_logs` and `knowledge_base_entries`
- `ai_config_schema.sql` (20260105000005) creates trigger for `ai_configs`

**Expected Pattern:**
```sql
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_integration_health_updated_at
    BEFORE UPDATE ON integration_health
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

**Impact:**
- `updated_at` will remain unchanged on updates unless manually set
- Inconsistent with project patterns
- May cause confusion in future queries expecting auto-updated timestamps

**Fix Required:** Add trigger function and trigger for `integration_health.updated_at`

---

#### CR-HIGH-002: Missing CHECK Constraints for Enum-like VARCHAR Columns
**Severity:** HIGH  
**Category:** Data Integrity  
**Location:** `migrations/20260111030835_create_integration_health_tables.sql`

**Problem:**
Multiple VARCHAR columns store enum-like values (status, severity, event_type) but have no CHECK constraints to enforce valid values. This allows invalid data to be inserted.

**Affected Columns:**
1. `integration_health.status` - Should be: 'healthy', 'warning', 'critical'
2. `integration_health.pricing_sync_status` - Should be: 'ok', 'warning', 'error'
3. `integration_health.fees_sync_status` - Should be: 'ok', 'warning', 'error'
4. `integration_events.severity` - Should be: 'low', 'medium', 'high', 'critical'
5. `integration_events.event_type` - Should be validated (story mentions: 'pricing_sync_error', 'fee_sync_error', 'booking_loss')

**Evidence:**
- Story file explicitly lists allowed values in comments
- Other migrations use ENUM types (e.g., `support_schema.sql` uses `CREATE TYPE error_status AS ENUM`)
- Story AC #5 requires "events are stored successfully with all fields" - implies validation

**Expected Pattern:**
```sql
-- Option 1: CHECK constraints (simpler, matches VARCHAR pattern)
ALTER TABLE integration_health
    ADD CONSTRAINT chk_status CHECK (status IN ('healthy', 'warning', 'critical'));

ALTER TABLE integration_health
    ADD CONSTRAINT chk_pricing_sync_status CHECK (pricing_sync_status IN ('ok', 'warning', 'error') OR pricing_sync_status IS NULL);

-- Option 2: ENUM types (more strict, used in support_schema.sql)
CREATE TYPE integration_status AS ENUM ('healthy', 'warning', 'critical');
-- Then use integration_status type in column definition
```

**Impact:**
- Invalid status values can be inserted (e.g., 'invalid', 'broken', 'unknown')
- Data integrity compromised
- Application must validate at runtime instead of database level
- Potential bugs in queries expecting specific values

**Fix Required:** Add CHECK constraints or ENUM types for all enum-like VARCHAR columns

---

### 🟡 MEDIUM Priority Issues

#### CR-MED-001: Missing CHECK Constraints for DECIMAL Range Validation
**Severity:** MEDIUM  
**Category:** Data Integrity  
**Location:** `migrations/20260111030835_create_integration_health_tables.sql`

**Problem:**
`booking_loss_rate` and `error_rate` are DECIMAL(5,4) with comments indicating range 0.0000 to 1.0000, but no CHECK constraints enforce this.

**Affected Columns:**
- `integration_health.booking_loss_rate` - Should be: 0.0000 to 1.0000
- `integration_health.error_rate` - Should be: 0.0000 to 1.0000

**Expected Pattern:**
```sql
ALTER TABLE integration_health
    ADD CONSTRAINT chk_booking_loss_rate CHECK (booking_loss_rate >= 0.0 AND booking_loss_rate <= 1.0 OR booking_loss_rate IS NULL);

ALTER TABLE integration_health
    ADD CONSTRAINT chk_error_rate CHECK (error_rate >= 0.0 AND error_rate <= 1.0 OR error_rate IS NULL);
```

**Impact:**
- Negative rates or rates > 1.0 can be inserted
- Data integrity compromised
- Business logic may break with invalid values

**Fix Required:** Add CHECK constraints for rate columns

---

#### CR-MED-002: Missing Table and Column Comments
**Severity:** MEDIUM  
**Category:** Documentation  
**Location:** `migrations/20260111030835_create_integration_health_tables.sql`

**Problem:**
No `COMMENT ON TABLE` or `COMMENT ON COLUMN` statements for documentation. Other migrations include comments.

**Evidence:**
- `splunk_templates_schema.sql` (20260105000003) has extensive comments
- `ai_config_schema.sql` (20260105000005) has comments
- `support_schema.sql` (20260105000004) has comments

**Expected Pattern:**
```sql
COMMENT ON TABLE integration_health IS 'Current integration health status (denormalized for fast reads)';
COMMENT ON TABLE integration_events IS 'Historical integration events (normalized for query flexibility)';
COMMENT ON COLUMN integration_health.booking_loss_rate IS 'Booking loss rate (0.0000 to 1.0000)';
COMMENT ON COLUMN integration_events.metadata IS 'Additional event data (flexible JSONB structure)';
```

**Impact:**
- Reduced database self-documentation
- Harder for developers to understand schema
- Missing context for future maintenance

**Fix Required:** Add COMMENT statements for tables and key columns

---

#### CR-MED-003: Missing Composite Index for Common Query Pattern
**Severity:** MEDIUM  
**Category:** Performance  
**Location:** `migrations/20260111030835_create_integration_health_tables.sql`

**Problem:**
Story mentions querying by `integration_id` and `last_checked` together, but no composite index exists. Only separate indexes on each column.

**Evidence:**
- Story AC #3: "querying by `integration_id`" - likely with time filtering
- Common pattern: Get latest health status for an integration
- Query: `SELECT * FROM integration_health WHERE integration_id = ? ORDER BY last_checked DESC LIMIT 1`

**Expected Pattern:**
```sql
CREATE INDEX IF NOT EXISTS idx_integration_health_id_checked ON integration_health(integration_id, last_checked DESC);
```

**Impact:**
- Queries filtering by both columns may not use optimal index
- Potential performance degradation for common queries
- Story AC #3 mentions < 100ms target - composite index helps achieve this

**Fix Required:** Add composite index for `(integration_id, last_checked DESC)`

---

#### CR-MED-004: Missing Index on `integration_events.severity`
**Severity:** MEDIUM  
**Category:** Performance  
**Location:** `migrations/20260111030835_create_integration_health_tables.sql`

**Problem:**
`integration_events` has indexes on `integration_id`, `occurred_at`, and `event_type`, but not on `severity`. Filtering by severity is likely common (e.g., "show me all critical events").

**Expected Pattern:**
```sql
CREATE INDEX IF NOT EXISTS idx_integration_events_severity ON integration_events(severity);
```

**Impact:**
- Queries filtering by severity will be slower
- Missing index for common filtering pattern

**Fix Required:** Add index on `severity` column

---

### 🟢 LOW Priority Issues

#### CR-LOW-001: Inconsistent Index Naming Pattern
**Severity:** LOW  
**Category:** Code Style  
**Location:** `migrations/20260111030835_create_integration_health_tables.sql`

**Problem:**
Index names are verbose (`idx_integration_health_integration_id`) while other migrations use shorter names (`idx_workflow_templates_ticket_type`).

**Evidence:**
- `workflow_schema.sql`: `idx_workflow_templates_ticket_type` (shorter)
- `pattern_detection_schema.sql`: `idx_patterns_type` (shorter)
- Current migration: `idx_integration_health_integration_id` (longer)

**Impact:**
- Minor inconsistency
- No functional impact

**Fix Required:** Optional - align naming with project patterns

---

#### CR-LOW-002: Missing Partial Index for Active Events
**Severity:** LOW  
**Category:** Performance Optimization  
**Location:** `migrations/20260111030835_create_integration_health_tables.sql`

**Problem:**
No partial index for filtering recent events (e.g., last 90 days). Story mentions 90-day retention policy.

**Expected Pattern:**
```sql
CREATE INDEX IF NOT EXISTS idx_integration_events_recent ON integration_events(occurred_at DESC)
WHERE occurred_at > NOW() - INTERVAL '90 days';
```

**Impact:**
- Minor performance optimization opportunity
- Not critical for initial implementation

**Fix Required:** Optional - add partial index for recent events

---

## Acceptance Criteria Validation

| AC | Status | Notes |
|----|--------|-------|
| AC #1: `integration_health` table created | ✅ PASS | Table exists with correct schema |
| AC #2: `integration_events` table created | ✅ PASS | Table exists with correct schema |
| AC #3: Query performance by `integration_id` < 100ms | ⚠️ PARTIAL | Index exists but composite index missing |
| AC #4: Query performance by `occurred_at` < 100ms | ✅ PASS | Index exists on `occurred_at DESC` |
| AC #5: Events stored successfully | ⚠️ PARTIAL | Schema allows storage but no validation constraints |

---

## Task Completion Validation

| Task | Status | Notes |
|------|--------|-------|
| Task 1: Create migration file | ✅ COMPLETE | File exists and follows naming pattern |
| Task 1.1-1.6: Schema definition | ✅ COMPLETE | All columns, constraints, indexes present |
| Task 1.7: Test migration runs | ✅ COMPLETE | Migration installed successfully |
| Task 2: Verify schema correctness | ⚠️ PARTIAL | Missing CHECK constraints and triggers |
| Task 3: Test query performance | ⚠️ PARTIAL | Indexes exist but composite index missing |
| Task 4: Test event storage | ⚠️ PARTIAL | Storage works but no validation |

---

## Recommendations

### Immediate Fixes (Before Merge)
1. **Add `updated_at` trigger** for `integration_health` table (HIGH)
2. **Add CHECK constraints** for enum-like VARCHAR columns (HIGH)
3. **Add CHECK constraints** for DECIMAL range validation (MEDIUM)
4. **Add composite index** for `(integration_id, last_checked DESC)` (MEDIUM)

### Nice-to-Have (Can be deferred)
5. Add table/column comments (MEDIUM)
6. Add index on `severity` column (MEDIUM)
7. Align index naming with project patterns (LOW)
8. Add partial index for recent events (LOW)

---

## Files Reviewed

**Created:**
- ✅ `migrations/20260111030835_create_integration_health_tables.sql` - Migration file

**Modified:**
- None (new migration file only)

**Git Status:**
- Migration file is untracked (new file)
- No git history found for this story

---

## Next Steps

1. **Developer Action Required:** Fix HIGH and MEDIUM priority issues
2. **Re-review:** After fixes, re-run code review to verify
3. **Update Story Status:** After fixes verified, update story status in `sprint-status.yaml`

---

**Review Complete:** 2026-01-11  
**Next Review:** After fixes applied
