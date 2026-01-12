# Story 19.8: Advanced RBAC with Dynamic Permissions

Status: ready-for-dev

## Story

**As a** Security Administrator  
**I want** advanced RBAC with dynamic permissions  
**So that** I can manage permissions dynamically and granularly

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 19.8 |
| Epic | Epic 19: Advanced Features |
| Sprint | Sprint 3: Security |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Epic 15 (Authentication) - Complete ✅ |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create dynamic permissions system
   - Add `permissions` table (id, resource, action, condition)
   - Add `role_permissions` table (role_id, permission_id)
   - Add `user_permissions` table (user_id, permission_id) - override
   - Support permission conditions (dynamic evaluation)

2. Implement permission evaluation
   - Check user permissions
   - Evaluate permission conditions
   - Support resource-level permissions
   - Support action-level permissions

3. Create permission management UI
   - Manage role permissions
   - Manage user permissions
   - Permission testing/debugging
   - Permission audit log

4. Integrate with existing RBAC
   - Enhance existing RBAC system (Epic 15)
   - Support dynamic permissions alongside static roles
   - Permission inheritance from roles
   - User permission overrides

---

## Acceptance Criteria

- [ ] **Given** dynamic permissions exist  
  **When** checking permission  
  **Then** permission is evaluated dynamically

- [ ] **Given** dynamic permissions exist  
  **When** managing permissions  
  **Then** permissions are saved and applied

- [ ] **Given** dynamic permissions exist  
  **When** user has permission  
  **Then** access is granted

- [ ] **Given** dynamic permissions exist  
  **When** user lacks permission  
  **Then** access is denied

---

## Tasks / Subtasks

- [ ] Task 1: Create permissions database schema
  - [ ] 1.1: Create migration: `YYYYMMDDHHMMSS_create_permissions_tables.sql`
  - [ ] 1.2: Define `permissions` table
  - [ ] 1.3: Define `role_permissions` table
  - [ ] 1.4: Define `user_permissions` table

- [ ] Task 2: Create permission service
  - [ ] 2.1: Create `crates/qa-pms-permissions/Cargo.toml`
  - [ ] 2.2: Create `crates/qa-pms-permissions/src/permission_service.rs`
  - [ ] 2.3: Implement permission evaluation
  - [ ] 2.4: Support permission conditions

- [ ] Task 3: Integrate with existing RBAC
  - [ ] 3.1: Enhance Epic 15 RBAC system
  - [ ] 3.2: Add dynamic permission checks
  - [ ] 3.3: Support permission inheritance

- [ ] Task 4: Create permission management API
  - [ ] 4.1: Create `crates/qa-pms-api/src/routes/permissions.rs`
  - [ ] 4.2: GET /api/v1/permissions - list permissions
  - [ ] 4.3: POST /api/v1/permissions - create permission
  - [ ] 4.4: POST /api/v1/roles/:id/permissions - assign permission to role
  - [ ] 4.5: POST /api/v1/users/:id/permissions - assign permission to user

- [ ] Task 5: Create permission management UI
  - [ ] 5.1: Create `frontend/src/pages/Settings/PermissionsPage.tsx`
  - [ ] 5.2: Display permissions list
  - [ ] 5.3: Manage role permissions
  - [ ] 5.4: Manage user permissions

---

## Files to Create

| File | Changes |
|------|---------|
| `migrations/YYYYMMDDHHMMSS_create_permissions_tables.sql` | Create permissions tables |
| `crates/qa-pms-permissions/Cargo.toml` | Create permissions crate |
| `crates/qa-pms-permissions/src/lib.rs` | Create permissions crate root |
| `crates/qa-pms-permissions/src/permission_service.rs` | Create permission service |
| `crates/qa-pms-api/src/routes/permissions.rs` | Create permission API routes |
| `frontend/src/pages/Settings/PermissionsPage.tsx` | Create permissions management page |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-api/src/middleware/auth.rs` | Enhance with dynamic permission checks |

---

## Dev Notes

### Permission System Architecture

**Permission Model:**
```rust
pub struct Permission {
    pub id: Uuid,
    pub resource: String,  // e.g., "tickets", "workflows", "reports"
    pub action: String,    // e.g., "read", "write", "delete", "share"
    pub condition: Option<String>, // Optional condition (JSONB or expression)
}
```

**Permission Evaluation:**
- Check user permissions (explicit)
- Check role permissions (inherited)
- Evaluate conditions (dynamic)
- Return allow/deny

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 19, Story 19.8)
- Dependency: Epic 15 (Authentication) - must be complete
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### File List
