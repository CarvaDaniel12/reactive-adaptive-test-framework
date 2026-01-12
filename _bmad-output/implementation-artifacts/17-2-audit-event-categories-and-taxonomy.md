# Story 17.2: Audit Event Categories and Taxonomy

Status: ready-for-dev

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

**As a** Security Administrator  
**I want** a standardized taxonomy for audit event categories and types  
**So that** I can consistently categorize and analyze audit events across the system

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 17.2 |
| Epic | Epic 17: Audit Logging |
| Sprint | Sprint 1: Audit Log Storage |
| Priority | P0 (Critical) |
| Estimated Days | 1 |
| Dependencies | Story 17.1 (Comprehensive Audit Log Storage) |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Define audit event categories enum
   - Categories: `Security`, `Data`, `System`, `Api`, `Admin`
   - Use Rust enum with `sqlx::Type` for database mapping
   - Use `serde` for JSON serialization

2. Define audit event types enum
   - Types: `Authentication`, `Authorization`, `DataAccess`, `ConfigurationChange`, `SecurityEvent`, `ApiCall`, `SystemEvent`
   - Use Rust enum with `sqlx::Type` for database mapping
   - Use `serde` for JSON serialization

3. Define audit action types enum
   - Actions: `Login`, `Logout`, `Create`, `Update`, `Delete`, `Read`, `Export`, `Import`, `Grant`, `Revoke`
   - Use Rust enum with `sqlx::Type` for database mapping
   - Use `serde` for JSON serialization

4. Create Rust types module
   - Create `qa-pms-audit` crate (or add to existing crate)
   - Define enums in `types.rs`
   - Implement `Display`, `FromStr` for all enums
   - Implement validation helpers

---

## Acceptance Criteria

- [ ] **Given** audit event categories enum exists  
  **When** using event categories  
  **Then** categories are consistent across the system

- [ ] **Given** audit event types enum exists  
  **When** using event types  
  **Then** event types are consistent across the system

- [ ] **Given** audit action types enum exists  
  **When** using action types  
  **Then** action types are consistent across the system

- [ ] **Given** Rust types exist  
  **When** serializing to JSON  
  **Then** enums serialize to strings (camelCase)

- [ ] **Given** Rust types exist  
  **When** mapping from database  
  **Then** database strings map to enum variants correctly

---

## Tasks / Subtasks

- [ ] Task 1: Create qa-pms-audit crate (AC: #1, #2, #3)
  - [ ] 1.1: Create `crates/qa-pms-audit/Cargo.toml`
  - [ ] 1.2: Add dependencies: `qa-pms-core`, `serde`, `sqlx`, `utoipa`
  - [ ] 1.3: Create `src/lib.rs` with module exports
  - [ ] 1.4: Add crate to workspace `Cargo.toml`

- [ ] Task 2: Create audit event categories enum (AC: #1)
  - [ ] 2.1: Create `src/types.rs` with `AuditEventCategory` enum
  - [ ] 2.2: Define variants: `Security`, `Data`, `System`, `Api`, `Admin`
  - [ ] 2.3: Add `sqlx::Type` derive for database mapping
  - [ ] 2.4: Add `serde` serialization with camelCase
  - [ ] 2.5: Implement `Display` trait
  - [ ] 2.6: Implement `FromStr` trait
  - [ ] 2.7: Add `utoipa::ToSchema` for OpenAPI

- [ ] Task 3: Create audit event types enum (AC: #2)
  - [ ] 3.1: Add `AuditEventType` enum to `src/types.rs`
  - [ ] 3.2: Define variants: `Authentication`, `Authorization`, `DataAccess`, `ConfigurationChange`, `SecurityEvent`, `ApiCall`, `SystemEvent`
  - [ ] 3.3: Add `sqlx::Type` derive for database mapping
  - [ ] 3.4: Add `serde` serialization with camelCase
  - [ ] 3.5: Implement `Display` trait
  - [ ] 3.6: Implement `FromStr` trait
  - [ ] 3.7: Add `utoipa::ToSchema` for OpenAPI

- [ ] Task 4: Create audit action types enum (AC: #3)
  - [ ] 4.1: Add `AuditAction` enum to `src/types.rs`
  - [ ] 4.2: Define variants: `Login`, `Logout`, `Create`, `Update`, `Delete`, `Read`, `Export`, `Import`, `Grant`, `Revoke`
  - [ ] 4.3: Add `sqlx::Type` derive for database mapping
  - [ ] 4.4: Add `serde` serialization with camelCase
  - [ ] 4.5: Implement `Display` trait
  - [ ] 4.6: Implement `FromStr` trait
  - [ ] 4.7: Add `utoipa::ToSchema` for OpenAPI

- [ ] Task 5: Add unit tests (AC: #1, #2, #3, #4, #5)
  - [ ] 5.1: Test enum serialization to JSON
  - [ ] 5.2: Test enum deserialization from JSON
  - [ ] 5.3: Test enum Display implementation
  - [ ] 5.4: Test enum FromStr implementation
  - [ ] 5.5: Test database mapping (string to enum)

---

## Files to Create

| File | Changes |
|------|---------|
| `crates/qa-pms-audit/Cargo.toml` | Create new crate with dependencies |
| `crates/qa-pms-audit/src/lib.rs` | Create crate root with module exports |
| `crates/qa-pms-audit/src/types.rs` | Create audit types (categories, event types, actions) |

---

## Files to Modify

| File | Changes |
|------|---------|
| `Cargo.toml` (workspace root) | Add `qa-pms-audit` to `members` and `workspace.dependencies` |

---

## Dev Notes

### Audit Event Categories

**Categories:**
- `Security`: Security-related events (login, logout, authorization failures)
- `Data`: Data access events (read, create, update, delete)
- `System`: System events (startup, shutdown, configuration changes)
- `Api`: API call events (endpoint access, rate limiting)
- `Admin`: Administrative events (user management, role changes)

**Rust Enum:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum AuditEventCategory {
    Security,
    Data,
    System,
    Api,
    Admin,
}
```

### Audit Event Types

**Event Types:**
- `Authentication`: User authentication events (login, logout, token refresh)
- `Authorization`: Authorization events (permission checks, role changes)
- `DataAccess`: Data access events (read, create, update, delete operations)
- `ConfigurationChange`: Configuration changes (settings, integrations)
- `SecurityEvent`: Security events (failed login attempts, suspicious activity)
- `ApiCall`: API endpoint calls (endpoint access, method, status)
- `SystemEvent`: System events (startup, shutdown, health checks)

**Rust Enum:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum AuditEventType {
    Authentication,
    Authorization,
    DataAccess,
    ConfigurationChange,
    SecurityEvent,
    ApiCall,
    SystemEvent,
}
```

### Audit Actions

**Actions:**
- `Login`: User login
- `Logout`: User logout
- `Create`: Create resource
- `Update`: Update resource
- `Delete`: Delete resource
- `Read`: Read resource
- `Export`: Export data
- `Import`: Import data
- `Grant`: Grant permission
- `Revoke`: Revoke permission

**Rust Enum:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum AuditAction {
    Login,
    Logout,
    Create,
    Update,
    Delete,
    Read,
    Export,
    Import,
    Grant,
    Revoke,
}
```

### Project Structure Notes

**Crate Structure:**
- Follow existing crate patterns (`qa-pms-integration-health`, `qa-pms-patterns`)
- Separate crate: `qa-pms-audit`
- Module structure: `lib.rs`, `types.rs` (future: `repository.rs`, `service.rs`)

**Dependencies:**
- `qa-pms-core`: Shared types, error handling
- `serde`: Serialization for JSON
- `sqlx`: Database type mapping
- `utoipa`: OpenAPI documentation

**Database Mapping:**
- Use `sqlx::Type` derive for enum-to-VARCHAR mapping
- Use `rename_all = "snake_case"` for database representation
- Use `rename_all = "camelCase"` for JSON serialization

### Testing Standards

**Unit Tests:**
- Test enum serialization to JSON (camelCase)
- Test enum deserialization from JSON
- Test enum Display implementation (snake_case)
- Test enum FromStr implementation
- Test database mapping (VARCHAR to enum)

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 17, Story 17.2)
- Dependency: Story 17.1 (Comprehensive Audit Log Storage) - must be completed first
- Type Patterns: `qa-intelligent-pms/crates/qa-pms-integration-health/src/types.rs` (reference)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
