# Story 24.2: Correlation Database Schema

Status: ready-for-dev

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

**As a** QA Engineer working on PMS integration quality  
**I want** a database schema to cache correlation results between test results and integration health  
**So that** correlation calculations can be stored and retrieved efficiently for performance

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 24.2 |
| Epic | Epic 24: Test-Integration Correlation Engine |
| Sprint | Sprint 1: Correlation Engine |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Story 24.1 (Correlation Calculation Engine) |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create `test_integration_correlations` table (cache)
   - Store correlation scores, types, patterns, confidence
   - Support unique constraint on (test_case_id, integration_id)
   - Use TIMESTAMPTZ for timestamps (consistent with existing migrations)
   - Use DECIMAL for scores (0.00 to 1.00)

2. Add indexes for performance
   - Index on `test_case_id` for test case queries
   - Index on `integration_id` for integration queries
   - Index on `correlation_score DESC` for high correlation queries

3. Create SQLx migration file
   - Follow existing migration patterns (Story 22.1 as reference)
   - Use `CREATE TABLE IF NOT EXISTS` for idempotency
   - Use `CREATE INDEX IF NOT EXISTS` for indexes
   - Include migration comment with story reference

4. Cache correlation results
   - Store correlation results from Story 24.1 engine
   - Support upsert operations (ON CONFLICT UPDATE)
   - Track when correlation was last calculated

---

## Acceptance Criteria

- [ ] **Given** database schema  
  **When** migration runs  
  **Then** `test_integration_correlations` table is created

- [ ] **Given** table exists  
  **When** querying by `test_case_id`  
  **Then** query performance < 100ms

- [ ] **Given** table exists  
  **When** querying by `integration_id`  
  **Then** query performance < 100ms

- [ ] **Given** table exists  
  **When** querying by `correlation_score` (DESC)  
  **Then** query performance < 100ms

---

## Tasks / Subtasks

- [ ] Task 1: Create SQLx migration file (AC: #1)
  - [ ] 1.1: Create migration file with timestamp: `migrations/YYYYMMDDHHMMSS_create_correlation_table.sql`
  - [ ] 1.2: Add migration comment with story reference
  - [ ] 1.3: Create `test_integration_correlations` table with columns:
    - `id UUID PRIMARY KEY DEFAULT gen_random_uuid()`
    - `test_case_id VARCHAR(100) NOT NULL`
    - `integration_id VARCHAR(50) NOT NULL`
    - `correlation_score DECIMAL(3,2) NOT NULL` (0.00 to 1.00)
    - `correlation_type VARCHAR(20) NOT NULL` ('high', 'medium', 'low')
    - `pattern VARCHAR(50) NOT NULL` (e.g., 'test_failure_precedes_integration_failure')
    - `confidence DECIMAL(3,2) NOT NULL` (0.00 to 1.00)
    - `last_correlated TIMESTAMPTZ NOT NULL`
    - `created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()`
    - `updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()`
  - [ ] 1.4: Add UNIQUE constraint on (test_case_id, integration_id)

- [ ] Task 2: Create indexes for performance (AC: #2, #3, #4)
  - [ ] 2.1: Create index on `test_case_id`: `idx_correlation_test_case`
  - [ ] 2.2: Create index on `integration_id`: `idx_correlation_integration`
  - [ ] 2.3: Create index on `correlation_score DESC`: `idx_correlation_score`
  - [ ] 2.4: Use `CREATE INDEX IF NOT EXISTS` for idempotency

- [ ] Task 3: Verify migration (AC: #1, #2, #3, #4)
  - [ ] 3.1: Test migration runs without errors
  - [ ] 3.2: Verify table created with correct schema
  - [ ] 3.3: Verify indexes created successfully
  - [ ] 3.4: Test query performance (< 100ms for all indexed queries)

---

## Files to Create

| File | Changes |
|------|---------|
| `migrations/YYYYMMDDHHMMSS_create_correlation_table.sql` | Create SQLx migration file with table schema and indexes |

---

## Files to Modify

| File | Changes |
|------|---------|
| None | New migration file only |

---

## Dev Notes

### Database Schema

**Table: `test_integration_correlations`**

```sql
CREATE TABLE IF NOT EXISTS test_integration_correlations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    test_case_id VARCHAR(100) NOT NULL, -- Testmo test case ID
    integration_id VARCHAR(50) NOT NULL, -- 'booking-com', 'airbnb', 'vrbo', 'hmbn'
    correlation_score DECIMAL(3,2) NOT NULL, -- 0.00 to 1.00
    correlation_type VARCHAR(20) NOT NULL, -- 'high', 'medium', 'low'
    pattern VARCHAR(50) NOT NULL, -- 'test_failure_precedes_integration_failure'
    confidence DECIMAL(3,2) NOT NULL, -- 0.00 to 1.00
    last_correlated TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(test_case_id, integration_id)
);
```

**Indexes:**

```sql
CREATE INDEX IF NOT EXISTS idx_correlation_test_case ON test_integration_correlations(test_case_id);
CREATE INDEX IF NOT EXISTS idx_correlation_integration ON test_integration_correlations(integration_id);
CREATE INDEX IF NOT EXISTS idx_correlation_score ON test_integration_correlations(correlation_score DESC);
```

### Project Structure Notes

**Migration File:**
- Follow existing migration patterns (Story 22.1: `20260111030835_create_integration_health_tables.sql`)
- Use timestamp format: `YYYYMMDDHHMMSS_create_correlation_table.sql`
- Include story reference in migration comment
- Use `CREATE TABLE IF NOT EXISTS` and `CREATE INDEX IF NOT EXISTS` for idempotency

**SQLx Migration Patterns:**
- Use `TIMESTAMPTZ` for timestamps (consistent with existing migrations)
- Use `DECIMAL(3,2)` for scores (0.00 to 1.00) for precision
- Use `VARCHAR` for string fields with appropriate lengths
- Use `UUID` with `gen_random_uuid()` for primary keys
- Use `UNIQUE` constraint for (test_case_id, integration_id) to prevent duplicates

**Performance Considerations:**
- Indexes on `test_case_id`, `integration_id`, and `correlation_score` for fast queries
- DESC index on `correlation_score` for queries filtering high correlations
- UNIQUE constraint supports efficient upsert operations

### Testing Standards

**Migration Tests:**
- Verify migration runs without errors
- Verify table created with correct schema
- Verify indexes created successfully
- Test query performance (< 100ms for indexed queries)

**Manual Tests:**
- Run migration: `sqlx migrate run`
- Verify table schema: `\d test_integration_correlations` (PostgreSQL)
- Verify indexes: `\di test_integration_correlations*` (PostgreSQL)
- Test query performance with EXPLAIN ANALYZE

### References

- Epic Document: `_bmad-output/planning-artifacts/epics-observability-pms-integrations-2026-01-10.md` (Epic 24, Story 24.2)
- Architecture Document: `_bmad-output/planning-artifacts/architecture-observability-pms-integrations-2026-01-10.md` (Section 4.3: Correlation Data Model)
- Migration Reference: `qa-intelligent-pms/migrations/20260111030835_create_integration_health_tables.sql` (Story 22.1 pattern)
- Dependency: Story 24.1 (Correlation Calculation Engine) - must be completed first
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
