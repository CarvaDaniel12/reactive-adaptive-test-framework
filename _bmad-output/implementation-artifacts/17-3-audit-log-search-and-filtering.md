# Story 17.3: Audit Log Search and Filtering

Status: ready-for-dev

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

**As a** Security Administrator  
**I want** to search and filter audit logs by various criteria  
**So that** I can quickly find specific audit events for security investigations and compliance reviews

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 17.3 |
| Epic | Epic 17: Audit Logging |
| Sprint | Sprint 2: Audit Log Querying |
| Priority | P0 (Critical) |
| Estimated Days | 2 |
| Dependencies | Story 17.2 (Audit Event Categories and Taxonomy) |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create `AuditLogRepository` in `qa-pms-audit`
   - Implement search and filtering methods
   - Support filtering by: user_id, event_type, event_category, action, resource_type, resource_id, date range, success
   - Support pagination (page, page_size)
   - Support sorting (by occurred_at DESC)

2. Create `search_audit_logs` method
   - Query `audit_logs` table with filters
   - Use SQLx query builder for dynamic filtering
   - Return paginated results
   - Response time < 500ms

3. Implement filters
   - Filter by user_id (UUID)
   - Filter by event_type (enum)
   - Filter by event_category (enum)
   - Filter by action (enum)
   - Filter by resource_type (string)
   - Filter by resource_id (string)
   - Filter by date range (start_date, end_date)
   - Filter by success (boolean)

4. Support pagination
   - Page number (1-indexed)
   - Page size (max 100, default 20)
   - Total count of matching records

---

## Acceptance Criteria

- [ ] **Given** audit log repository exists  
  **When** searching audit logs by user_id  
  **Then** returns audit logs for the specified user

- [ ] **Given** audit log repository exists  
  **When** searching audit logs by event_type  
  **Then** returns audit logs for the specified event type

- [ ] **Given** audit log repository exists  
  **When** searching audit logs by date range  
  **Then** returns audit logs within the specified date range

- [ ] **Given** audit log repository exists  
  **When** searching with multiple filters  
  **Then** returns audit logs matching all filters

- [ ] **Given** audit log repository exists  
  **When** searching with pagination  
  **Then** returns paginated results with total count

- [ ] **Given** audit log repository exists  
  **When** searching audit logs  
  **Then** query response time < 500ms

---

## Tasks / Subtasks

- [ ] Task 1: Create AuditLogRepository (AC: #1, #2, #3, #4, #5, #6)
  - [ ] 1.1: Create `crates/qa-pms-audit/src/repository.rs`
  - [ ] 1.2: Define `AuditLogRepository` struct with `PgPool`
  - [ ] 1.3: Implement `new()` constructor
  - [ ] 1.4: Implement `search_audit_logs` method with filters:
    - `user_id: Option<Uuid>`
    - `event_type: Option<AuditEventType>`
    - `event_category: Option<AuditEventCategory>`
    - `action: Option<AuditAction>`
    - `resource_type: Option<String>`
    - `resource_id: Option<String>`
    - `start_date: Option<DateTime<Utc>>`
    - `end_date: Option<DateTime<Utc>>`
    - `success: Option<bool>`
    - `page: u32` (default: 1)
    - `page_size: u32` (default: 20, max: 100)
  - [ ] 1.5: Use SQLx query builder for dynamic filtering
  - [ ] 1.6: Implement pagination with LIMIT and OFFSET
  - [ ] 1.7: Return total count of matching records
  - [ ] 1.8: Sort by `occurred_at DESC` (most recent first)

- [ ] Task 2: Create AuditLog type (AC: #1, #2, #3)
  - [ ] 2.1: Create `AuditLog` struct in `src/types.rs`
  - [ ] 2.2: Define fields matching `audit_logs` table schema
  - [ ] 2.3: Implement `sqlx::FromRow` for database mapping
  - [ ] 2.4: Add `serde` serialization with camelCase
  - [ ] 2.5: Add `utoipa::ToSchema` for OpenAPI

- [ ] Task 3: Create search filters type (AC: #3)
  - [ ] 3.1: Create `AuditLogFilters` struct in `src/types.rs`
  - [ ] 3.2: Define filter fields (all optional)
  - [ ] 3.3: Add `serde` serialization
  - [ ] 3.4: Add `utoipa::IntoParams` for query parameters

- [ ] Task 4: Create paginated results type (AC: #4)
  - [ ] 4.1: Create `AuditLogSearchResponse` struct
  - [ ] 4.2: Include `logs: Vec<AuditLog>`, `total: i64`, `page: u32`, `page_size: u32`
  - [ ] 4.3: Add `serde` serialization
  - [ ] 4.4: Add `utoipa::ToSchema` for OpenAPI

- [ ] Task 5: Add unit and integration tests (AC: #1, #2, #3, #4, #5, #6)
  - [ ] 5.1: Test search by user_id
  - [ ] 5.2: Test search by event_type
  - [ ] 5.3: Test search by date range
  - [ ] 5.4: Test search with multiple filters
  - [ ] 5.5: Test pagination
  - [ ] 5.6: Test query performance (< 500ms)

---

## Files to Create

| File | Changes |
|------|---------|
| `crates/qa-pms-audit/src/repository.rs` | Create audit log repository with search and filtering |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-audit/src/types.rs` | Add AuditLog, AuditLogFilters, AuditLogSearchResponse types |
| `crates/qa-pms-audit/src/lib.rs` | Export repository and types |

---

## Dev Notes

### Repository Structure

**AuditLogRepository:**
```rust
pub struct AuditLogRepository {
    pool: PgPool,
}

impl AuditLogRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn search_audit_logs(
        &self,
        filters: AuditLogFilters,
        page: u32,
        page_size: u32,
    ) -> Result<AuditLogSearchResponse, AuditError> {
        // Build dynamic query with filters
        // Use SQLx query builder
        // Implement pagination
    }
}
```

**Search Filters:**
```rust
#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogFilters {
    pub user_id: Option<Uuid>,
    pub event_type: Option<AuditEventType>,
    pub event_category: Option<AuditEventCategory>,
    pub action: Option<AuditAction>,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub success: Option<bool>,
}
```

**Dynamic Query Building:**
```rust
let mut query = sqlx::QueryBuilder::new(
    "SELECT * FROM audit_logs WHERE 1=1"
);

if let Some(user_id) = filters.user_id {
    query.push(" AND user_id = ");
    query.push_bind(user_id);
}

if let Some(event_type) = filters.event_type {
    query.push(" AND event_type = ");
    query.push_bind(event_type.to_string());
}

// ... more filters

query.push(" ORDER BY occurred_at DESC");
query.push(" LIMIT ");
query.push_bind(page_size);
query.push(" OFFSET ");
query.push_bind((page - 1) * page_size);
```

### Project Structure Notes

**Repository Pattern:**
- Follow existing repository patterns (`qa-pms-integration-health/src/repository.rs`)
- Use `PgPool` for database access
- Use `SqlxResultExt` for error mapping
- Use SQLx query builder for dynamic filtering

**Dependencies:**
- `qa-pms-core`: Shared types, error handling
- `qa-pms-dashboard`: SqlxResultExt for error mapping (if available)
- `sqlx`: Database queries
- `chrono`: DateTime handling
- `uuid`: UUID handling

**Performance Considerations:**
- Use indexes from Story 17.1 for fast queries
- Composite index on `(user_id, event_type, occurred_at DESC)` for common queries
- Limit page size to 100 to prevent large result sets
- Use DESC index on `occurred_at` for recent event queries

### Testing Standards

**Unit Tests:**
- Test filter combinations
- Test pagination logic
- Test query building

**Integration Tests:**
- Test with real database
- Test query performance (< 500ms)
- Test pagination with large datasets

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 17, Story 17.3)
- Dependency: Story 17.2 (Audit Event Categories and Taxonomy) - must be completed first
- Dependency: Story 17.1 (Comprehensive Audit Log Storage) - database schema must exist
- Repository Pattern: `qa-intelligent-pms/crates/qa-pms-integration-health/src/repository.rs` (reference)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
