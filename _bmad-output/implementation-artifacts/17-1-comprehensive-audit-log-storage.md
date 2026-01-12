# Story 17.1: Comprehensive Audit Log Storage

Status: ready-for-dev

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

**As a** Security Administrator  
**I want** a comprehensive audit log storage system for all system activities  
**So that** I can track security events, ensure compliance, and perform forensic analysis

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 17.1 |
| Epic | Epic 17: Audit Logging |
| Sprint | Sprint 1: Audit Log Storage |
| Priority | P0 (Critical) |
| Estimated Days | 2 |
| Dependencies | Epic 15 (Authentication & Authorization) - Complete ✅ |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create `audit_logs` table (comprehensive audit storage)
   - Store all system activities: user actions, API calls, configuration changes, security events
   - Columns: `id` (UUID), `user_id` (UUID, nullable), `event_type` (VARCHAR 50), `event_category` (VARCHAR 50), `action` (VARCHAR 100), `resource_type` (VARCHAR 50), `resource_id` (VARCHAR 255), `ip_address` (INET), `user_agent` (TEXT), `success` (BOOLEAN), `message` (TEXT), `metadata` (JSONB), `occurred_at` (TIMESTAMPTZ), `created_at` (TIMESTAMPTZ)
   - Indexes: `user_id`, `event_type`, `event_category`, `occurred_at DESC`, `resource_type`, `resource_id`

2. Create SQLx migration file
   - Follow existing migration patterns (Story 22.1, Story 31.9 as reference)
   - Use `CREATE TABLE IF NOT EXISTS` for idempotency
   - Include migration comment with story reference

3. Support immutable audit logs
   - No UPDATE or DELETE operations allowed (compliance requirement)
   - All audit events are append-only

4. Performance considerations
   - Indexes for fast queries by user, event type, date range
   - Support high-volume writes (audit logs for every API call)

---

## Acceptance Criteria

- [ ] **Given** database schema migration file exists  
  **When** migration runs  
  **Then** `audit_logs` table is created with correct schema

- [ ] **Given** audit_logs table exists  
  **When** querying by `user_id`  
  **Then** query performance < 100ms

- [ ] **Given** audit_logs table exists  
  **When** querying by `event_type`  
  **Then** query performance < 100ms

- [ ] **Given** audit_logs table exists  
  **When** querying by `occurred_at` date range  
  **Then** query performance < 100ms

- [ ] **Given** audit_logs table exists  
  **When** inserting audit events  
  **Then** events are stored successfully with all fields

- [ ] **Given** audit_logs table exists  
  **When** attempting UPDATE or DELETE  
  **Then** operations are prevented (compliance requirement)

---

## Tasks / Subtasks

- [ ] Task 1: Create SQLx migration file (AC: #1)
  - [ ] 1.1: Create migration file with timestamp: `migrations/YYYYMMDDHHMMSS_create_audit_logs_table.sql`
  - [ ] 1.2: Add migration comment with story reference
  - [ ] 1.3: Create `audit_logs` table with columns:
    - `id UUID PRIMARY KEY DEFAULT gen_random_uuid()`
    - `user_id UUID` (nullable, references users if user table exists)
    - `event_type VARCHAR(50) NOT NULL` (e.g., 'authentication', 'authorization', 'data_access', 'configuration_change')
    - `event_category VARCHAR(50) NOT NULL` (e.g., 'security', 'data', 'system', 'api')
    - `action VARCHAR(100) NOT NULL` (e.g., 'login', 'logout', 'create', 'update', 'delete', 'read')
    - `resource_type VARCHAR(50)` (e.g., 'ticket', 'workflow', 'report', 'user', 'config')
    - `resource_id VARCHAR(255)` (ID of the affected resource)
    - `ip_address INET` (client IP address)
    - `user_agent TEXT` (client user agent)
    - `success BOOLEAN NOT NULL DEFAULT true` (whether action succeeded)
    - `message TEXT` (optional descriptive message)
    - `metadata JSONB` (optional additional context)
    - `occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW()`
    - `created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()`
  - [ ] 1.4: Add indexes for performance:
    - `idx_audit_logs_user_id` on `user_id` (WHERE user_id IS NOT NULL)
    - `idx_audit_logs_event_type` on `event_type`
    - `idx_audit_logs_event_category` on `event_category`
    - `idx_audit_logs_occurred_at` on `occurred_at DESC`
    - `idx_audit_logs_resource_type` on `resource_type` (WHERE resource_type IS NOT NULL)
    - `idx_audit_logs_resource_id` on `resource_id` (WHERE resource_id IS NOT NULL)
    - `idx_audit_logs_user_event` on `(user_id, event_type, occurred_at DESC)` (composite index)

- [ ] Task 2: Prevent UPDATE/DELETE operations (AC: #6)
  - [ ] 2.1: Create trigger to prevent UPDATE operations on `audit_logs`
  - [ ] 2.2: Create trigger to prevent DELETE operations on `audit_logs`
  - [ ] 2.3: Test triggers prevent modifications

- [ ] Task 3: Verify migration (AC: #1, #2, #3, #4, #5)
  - [ ] 3.1: Test migration runs without errors
  - [ ] 3.2: Verify table created with correct schema
  - [ ] 3.3: Verify indexes created successfully
  - [ ] 3.4: Test query performance (< 100ms for indexed queries)
  - [ ] 3.5: Test INSERT operations work correctly

---

## Files to Create

| File | Changes |
|------|---------|
| `migrations/YYYYMMDDHHMMSS_create_audit_logs_table.sql` | Create SQLx migration file with table schema, indexes, and triggers |

---

## Files to Modify

| File | Changes |
|------|---------|
| None | New migration file only |

---

## Dev Notes

### Database Schema

**Table: `audit_logs`**

```sql
CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID, -- Nullable (system events may not have user)
    event_type VARCHAR(50) NOT NULL, -- 'authentication', 'authorization', 'data_access', 'configuration_change', 'security_event'
    event_category VARCHAR(50) NOT NULL, -- 'security', 'data', 'system', 'api', 'admin'
    action VARCHAR(100) NOT NULL, -- 'login', 'logout', 'create', 'update', 'delete', 'read', 'export'
    resource_type VARCHAR(50), -- 'ticket', 'workflow', 'report', 'user', 'config', 'integration'
    resource_id VARCHAR(255), -- ID of affected resource
    ip_address INET, -- Client IP address
    user_agent TEXT, -- Client user agent string
    success BOOLEAN NOT NULL DEFAULT true, -- Whether action succeeded
    message TEXT, -- Optional descriptive message
    metadata JSONB, -- Optional additional context (request params, response codes, etc.)
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

**Indexes:**

```sql
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id) WHERE user_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_audit_logs_event_type ON audit_logs(event_type);
CREATE INDEX IF NOT EXISTS idx_audit_logs_event_category ON audit_logs(event_category);
CREATE INDEX IF NOT EXISTS idx_audit_logs_occurred_at ON audit_logs(occurred_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_logs_resource_type ON audit_logs(resource_type) WHERE resource_type IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_audit_logs_resource_id ON audit_logs(resource_id) WHERE resource_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_event ON audit_logs(user_id, event_type, occurred_at DESC) WHERE user_id IS NOT NULL;
```

**Immutable Log Protection:**

```sql
-- Prevent UPDATE operations
CREATE OR REPLACE FUNCTION prevent_audit_log_update()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'Audit logs are immutable. UPDATE operations are not allowed.';
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER audit_logs_prevent_update
    BEFORE UPDATE ON audit_logs
    FOR EACH ROW
    EXECUTE FUNCTION prevent_audit_log_update();

-- Prevent DELETE operations
CREATE OR REPLACE FUNCTION prevent_audit_log_delete()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'Audit logs are immutable. DELETE operations are not allowed.';
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER audit_logs_prevent_delete
    BEFORE DELETE ON audit_logs
    FOR EACH ROW
    EXECUTE FUNCTION prevent_audit_log_delete();
```

### Project Structure Notes

**Migration File:**
- Follow existing migration patterns (Story 22.1: `20260111030835_create_integration_health_tables.sql`)
- Use timestamp format: `YYYYMMDDHHMMSS_create_audit_logs_table.sql`
- Include story reference in migration comment
- Use `CREATE TABLE IF NOT EXISTS` and `CREATE INDEX IF NOT EXISTS` for idempotency
- Use `CREATE OR REPLACE FUNCTION` for triggers (allows re-running migration)

**SQLx Migration Patterns:**
- Use `TIMESTAMPTZ` for timestamps (consistent with existing migrations)
- Use `UUID` with `gen_random_uuid()` for primary keys
- Use `INET` type for IP addresses (PostgreSQL native type)
- Use `JSONB` for metadata (flexible structure, queryable)
- Use partial indexes with `WHERE` clause for nullable columns (better performance)

**Performance Considerations:**
- Composite index on `(user_id, event_type, occurred_at DESC)` for common queries
- Partial indexes on nullable columns to save space
- DESC index on `occurred_at` for recent event queries
- Indexes on `event_type` and `event_category` for filtering

**Compliance Requirements:**
- Immutable logs (no UPDATE or DELETE) - enforced by triggers
- Complete audit trail (all fields captured)
- Timestamp precision (TIMESTAMPTZ with timezone)

### Testing Standards

**Migration Tests:**
- Verify migration runs without errors
- Verify table created with correct schema
- Verify indexes created successfully
- Verify triggers created and functional

**Performance Tests:**
- Test query performance by `user_id` (< 100ms)
- Test query performance by `event_type` (< 100ms)
- Test query performance by date range (< 100ms)
- Test INSERT performance (high-volume writes)

**Compliance Tests:**
- Test UPDATE operations are prevented (trigger raises exception)
- Test DELETE operations are prevented (trigger raises exception)
- Test INSERT operations work correctly

**Manual Tests:**
- Run migration: `sqlx migrate run`
- Verify table schema: `\d audit_logs` (PostgreSQL)
- Verify indexes: `\di audit_logs*` (PostgreSQL)
- Verify triggers: `\d+ audit_logs` (PostgreSQL)
- Test query performance with EXPLAIN ANALYZE

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 17, Story 17.1)
- Migration Reference: `qa-intelligent-pms/migrations/20260111030835_create_integration_health_tables.sql` (Story 22.1 pattern)
- Migration Reference: `qa-intelligent-pms/migrations/20260110033607_create_anomalies_table.sql` (Story 31.9 pattern)
- Dependency: Epic 15 (Authentication & Authorization) - must be complete (provides user_id)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
