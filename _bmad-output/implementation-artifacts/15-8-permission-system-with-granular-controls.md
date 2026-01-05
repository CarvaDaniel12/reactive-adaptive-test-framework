# Story 15.8: Permission System with Granular Controls

Status: ready-for-dev

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a **System Administrator**,
I want to **define fine-grained permissions**,
So that **users have access only to what they need**.

## Acceptance Criteria

1. **Given** the permission system is configured
   **When** I define a permission like `tickets.update.own`
   **Then** users can only update their own tickets
   **And** users with `tickets.update.all` can update any ticket

2. **Given** a permission check is performed
   **When** checking if user can perform action
   **Then** system checks user's role permissions
   **And** supports resource ownership checks
   **And** supports custom permission predicates

3. **Given** hierarchical permissions exist
   **When** checking for permission like `tickets.update.all`
   **Then** system automatically includes `tickets.update.own` (inheritance)
   **And** permission inheritance follows parent-child relationships

4. **Given** a resource ownership check is required
   **When** checking `tickets.update.own` permission
   **Then** system validates that user owns the resource (ticket.created_by == user.id)
   **And** returns true if user has `tickets.update.all` (admin bypass)
   **And** supports custom ownership predicates for complex checks

5. **Given** permission middleware is applied to route
   **When** request is made to protected endpoint
   **Then** middleware checks permission before handler execution
   **And** returns 403 Forbidden if permission denied
   **And** logs unauthorized access attempts
   **And** continues to handler if permission granted

6. **Given** permission caching is enabled
   **When** checking permissions
   **Then** results are cached for 5 minutes (TTL)
   **And** cache is invalidated on role/permission changes
   **And** cache key includes user_id + permission string

7. **Given** custom permission predicates exist
   **When** checking complex permissions
   **Then** system supports predicate functions (e.g., user is assigned to ticket)
   **And** predicates can combine multiple checks
   **And** predicates receive resource context

8. **Given** permission matrix documentation exists
   **When** viewing permissions
   **Then** system provides human-readable permission descriptions
   **And** shows permission hierarchy visually
   **And** documents which permissions include others

## Tasks / Subtasks

- [ ] Task 1: Define permission enum and hierarchy structure (AC: #1, #3, #8)
  - [ ] 1.1: Create `Permission` enum in `crates/qa-pms-core/src/permissions.rs`
  - [ ] 1.2: Define permission variants: TicketsView, TicketsUpdate, TicketsUpdateOwn, TicketsUpdateAll, TicketsDelete, TicketsDeleteOwn, TicketsDeleteAll
  - [ ] 1.3: Define workflow permissions: WorkflowsExecute, WorkflowsManage, WorkflowsManageOwn, WorkflowsManageAll, WorkflowsDelete
  - [ ] 1.4: Define admin permissions: AdminUsers, AdminRoles, AdminPermissions, AdminSystem
  - [ ] 1.5: Implement `Permission::as_str()` for string representation
  - [ ] 1.6: Implement `Permission::from_str()` for parsing
  - [ ] 1.7: Create `PermissionHierarchy` trait with `includes()` method
  - [ ] 1.8: Implement permission inheritance logic (all includes own, etc.)
  - [ ] 1.9: Create `permission_matrix()` function for documentation
  - [ ] 1.10: Add unit tests for permission parsing and inheritance

- [ ] Task 2: Create permission database schema (AC: #1, #6)
  - [ ] 2.1: Create migration `migrations/YYYYMMDDHHMMSS_permissions_schema.sql`
  - [ ] 2.2: Add `permissions` table: id, name, description, resource_type, action, parent_id
  - [ ] 2.3: Add `role_permissions` junction table: role_id, permission_id
  - [ ] 2.4: Add `user_permissions` junction table: user_id, permission_id (for direct grants)
  - [ ] 2.5: Add indexes on role_id, user_id, permission_id
  - [ ] 2.6: Seed default permissions (tickets.*, workflows.*, admin.*)
  - [ ] 2.7: Link permissions to existing roles (from Story 15.3)
  - [ ] 2.8: Add unit tests for schema

- [ ] Task 3: Implement PermissionService for permission checks (AC: #2, #3, #4, #6)
  - [ ] 3.1: Create `PermissionService` struct in `crates/qa-pms-core/src/permissions.rs`
  - [ ] 3.2: Implement `has_permission(user_id, permission)` method
  - [ ] 3.3: Check direct user permissions first
  - [ ] 3.4: Check role permissions (union of all roles)
  - [ ] 3.5: Implement permission inheritance (check parent permissions)
  - [ ] 3.6: Add caching layer (Moka cache, 5min TTL, key: user_id + permission_str)
  - [ ] 3.7: Implement cache invalidation on role/permission changes
  - [ ] 3.8: Add `check_resource_ownership(user_id, resource_type, resource_id)` method
  - [ ] 3.9: Support custom ownership predicates (Fn closures)
  - [ ] 3.10: Add unit tests for permission checking logic

- [ ] Task 4: Create permission middleware for Axum (AC: #5)
  - [ ] 4.1: Create `crates/qa-pms-api/src/middleware/permissions.rs`
  - [ ] 4.2: Implement `require_permission()` middleware factory function
  - [ ] 4.3: Extract `UserContext` from request extensions (from auth middleware, Story 15.1)
  - [ ] 4.4: Get `PermissionService` from app state
  - [ ] 4.5: Check permission using `PermissionService::has_permission()`
  - [ ] 4.6: Return 403 Forbidden with JSON error if permission denied
  - [ ] 4.7: Log unauthorized access attempts (user_id, permission, endpoint, IP)
  - [ ] 4.8: Continue to next middleware/handler if permission granted
  - [ ] 4.9: Use Tower Layer pattern (from Axum docs: `middleware::from_fn()`)
  - [ ] 4.10: Add unit tests for middleware

- [ ] Task 5: Implement resource ownership checks (AC: #4)
  - [ ] 5.1: Create `OwnershipCheck` struct in `crates/qa-pms-core/src/permissions.rs`
  - [ ] 5.2: Implement `check_ticket_ownership(user_id, ticket_id)` method
  - [ ] 5.3: Query database: `SELECT created_by FROM tickets WHERE id = $1`
  - [ ] 5.4: Return true if user_id == created_by
  - [ ] 5.5: Implement `check_workflow_ownership(user_id, workflow_id)` method
  - [ ] 5.6: Query database: `SELECT created_by FROM workflow_instances WHERE id = $1`
  - [ ] 5.7: Support custom ownership predicates via trait
  - [ ] 5.8: Create `OwnershipPredicate` trait: `fn check(&self, user_id: Uuid, resource_id: Uuid) -> bool`
  - [ ] 5.9: Add admin bypass: if user has admin permission, return true
  - [ ] 5.10: Add unit tests for ownership checks

- [ ] Task 6: Create permission predicate system (AC: #7)
  - [ ] 6.1: Create `PermissionPredicate` trait in `crates/qa-pms-core/src/permissions.rs`
  - [ ] 6.2: Trait: `fn evaluate(&self, user: &UserContext, resource: &ResourceContext) -> bool`
  - [ ] 6.3: Create `AndPredicate` combinator (all must pass)
  - [ ] 6.4: Create `OrPredicate` combinator (any can pass)
  - [ ] 6.5: Create `NotPredicate` combinator (negation)
  - [ ] 6.6: Implement example predicate: `UserAssignedToTicket`
  - [ ] 6.7: Support predicate chaining in middleware
  - [ ] 6.8: Add unit tests for predicates

- [ ] Task 7: Add permission middleware to routes (AC: #5)
  - [ ] 7.1: Update `crates/qa-pms-api/src/routes/tickets.rs`
  - [ ] 7.2: Apply `require_permission(Permission::TicketsUpdateOwn)` to `PATCH /api/v1/tickets/:id`
  - [ ] 7.3: Apply `require_permission(Permission::TicketsUpdateAll)` to admin endpoints
  - [ ] 7.4: Update `crates/qa-pms-api/src/routes/workflows.rs`
  - [ ] 7.5: Apply `require_permission(Permission::WorkflowsManageOwn)` to workflow endpoints
  - [ ] 7.6: Update `crates/qa-pms-api/src/app.rs` to include PermissionService in AppState
  - [ ] 7.7: Initialize PermissionService with database pool
  - [ ] 7.8: Add permission middleware to admin routes (`/api/v1/admin/*`)
  - [ ] 7.9: Test all protected routes return 403 without permission
  - [ ] 7.10: Add integration tests for permission enforcement

- [ ] Task 8: Implement permission cache with Moka (AC: #6)
  - [ ] 8.1: Add `moka` dependency to `crates/qa-pms-core/Cargo.toml`
  - [ ] 8.2: Create `PermissionCache` struct wrapping `Cache<(Uuid, String), bool>`
  - [ ] 8.3: Configure cache: max capacity 10,000 entries, TTL 5 minutes
  - [ ] 8.4: Integrate cache into `PermissionService::has_permission()`
  - [ ] 8.5: Cache key: `(user_id, permission.as_str())`
  - [ ] 8.6: Implement cache invalidation on role/permission changes
  - [ ] 8.7: Add `invalidate_user_permissions(user_id)` method
  - [ ] 8.8: Add `invalidate_all()` method for full cache clear
  - [ ] 8.9: Add cache metrics (hit rate, size, evictions)
  - [ ] 8.10: Add unit tests for cache behavior

- [ ] Task 9: Create permission management API endpoints (AC: #1, #8)
  - [ ] 9.1: Create `crates/qa-pms-api/src/routes/permissions.rs`
  - [ ] 9.2: Add `GET /api/v1/admin/permissions` - list all permissions
  - [ ] 9.3: Add `GET /api/v1/admin/permissions/matrix` - permission hierarchy matrix
  - [ ] 9.4: Add `GET /api/v1/admin/permissions/:id` - get permission details
  - [ ] 9.5: Add `POST /api/v1/admin/users/:user_id/permissions` - grant permission to user
  - [ ] 9.6: Add `DELETE /api/v1/admin/users/:user_id/permissions/:permission_id` - revoke permission
  - [ ] 9.7: Protect all endpoints with `Permission::AdminPermissions`
  - [ ] 9.8: Invalidate permission cache on grant/revoke
  - [ ] 9.9: Add OpenAPI documentation with `utoipa`
  - [ ] 9.10: Add integration tests for permission management

- [ ] Task 10: Create permission documentation and helpers (AC: #8)
  - [ ] 10.1: Implement `Permission::description()` method returning human-readable description
  - [ ] 10.2: Implement `Permission::resource_type()` method returning resource category
  - [ ] 10.3: Create `permission_matrix()` function returning permission hierarchy as JSON
  - [ ] 10.4: Create `docs/permissions.md` documenting all permissions
  - [ ] 10.5: Document permission inheritance rules
  - [ ] 10.6: Document resource ownership checks
  - [ ] 10.7: Add examples of custom predicates
  - [ ] 10.8: Generate permission matrix diagram (text-based)

## Dev Notes

### Architecture Compliance

**Tech Stack:**
- Rust 1.80+ with Tokio async runtime
- Axum 0.7+ web framework (from `architecture.md`)
- Tower ecosystem for middleware layers
- Moka for in-memory caching (Story 14.4 pattern)
- PostgreSQL with SQLx 0.7 for persistence
- `tracing` for structured logging (never `println!`)

**Code Structure:**
- **Core permissions logic**: `crates/qa-pms-core/src/permissions.rs` (new file)
- **Middleware**: `crates/qa-pms-api/src/middleware/permissions.rs` (new file)
- **API routes**: `crates/qa-pms-api/src/routes/permissions.rs` (new file)
- **Database migrations**: `migrations/YYYYMMDDHHMMSS_permissions_schema.sql` (new file)

**Middleware Pattern (Axum):**
Following Axum middleware pattern from Story 15.1 and 15.7:
```rust
use axum::{
    middleware::{self, Next},
    extract::Request,
    response::Response,
    http::StatusCode,
};

async fn require_permission(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract UserContext from request extensions (set by auth middleware)
    // Check permission using PermissionService
    // Return 403 if denied, continue if granted
}
```

Apply to routes using `route_layer()`:
```rust
router.route("/api/v1/tickets/:id", patch(update_ticket))
    .route_layer(middleware::from_fn_with_state(
        state,
        require_permission(Permission::TicketsUpdateOwn)
    ));
```

**Permission Inheritance:**
- `tickets.update.all` automatically includes `tickets.update.own`
- Check parent permissions first, then specific
- Store hierarchy in database with `parent_id` foreign key

**Resource Ownership:**
- Extend existing `OwnershipCheck` from Story 15.3 (`15-3-role-based-access-control-rbac.md`)
- Query database to check `created_by` field
- Admin bypass: if user has `admin.*` permission, return true immediately

**Caching Strategy:**
- Use Moka cache (Story 14.4 pattern) for permission checks
- Cache key: `(user_id: Uuid, permission: String)`
- TTL: 5 minutes (configurable)
- Invalidate on role/permission changes
- Cache size: 10,000 entries max

### Previous Story Intelligence

**From Story 15.3 (RBAC):**
- Already have `RbacService` with `has_permission()` method
- Already have `OwnershipCheck` struct and `check_resource_ownership()` method
- Permission checking is done via roles, not direct grants
- This story extends RBAC with **direct user permissions** and **granular controls**

**Key Differences:**
- Story 15.3: Role-based (users → roles → permissions)
- Story 15.8: Direct permissions + ownership checks + predicates
- Story 15.8 extends 15.3, doesn't replace it

**Integration Points:**
- Reuse `UserContext` from Story 15.1 (JWT authentication)
- Extend `RbacService` with direct permission checks
- Build on `OwnershipCheck` from Story 15.3
- Use same middleware pattern as Story 15.7 (API key auth)

**From Story 15.7 (API Key Auth):**
- Middleware pattern using `middleware::from_fn()` 
- Request extension pattern for `UserContext`
- Error handling: return `StatusCode` from middleware
- Logging unauthorized access attempts

**Code Patterns to Reuse:**
```rust
// From 15-7-api-key-authentication.md
let user_context = req.extensions().get::<UserContext>().unwrap();
// Apply same pattern for permission checks
```

### Project Structure Notes

**Alignment with unified structure:**
- ✅ Permissions logic in `qa-pms-core` (core domain)
- ✅ Middleware in `qa-pms-api` (application layer)
- ✅ Routes in `qa-pms-api/src/routes/` (API layer)
- ✅ Migrations in `migrations/` (database layer)

**Files to Create:**
- `crates/qa-pms-core/src/permissions.rs` (new module)
- `crates/qa-pms-api/src/middleware/permissions.rs` (new module)
- `crates/qa-pms-api/src/routes/permissions.rs` (new route module)
- `migrations/YYYYMMDDHHMMSS_permissions_schema.sql` (new migration)

**Files to Modify:**
- `crates/qa-pms-core/src/lib.rs` - export `permissions` module
- `crates/qa-pms-api/src/app.rs` - add `PermissionService` to `AppState`
- `crates/qa-pms-api/src/routes/mod.rs` - add `permissions` route
- `crates/qa-pms-api/src/routes/tickets.rs` - apply permission middleware
- `crates/qa-pms-api/src/routes/workflows.rs` - apply permission middleware
- `crates/qa-pms-core/Cargo.toml` - add `moka` dependency

**Naming Conventions:**
- Permissions: `ResourceAction` (e.g., `TicketsUpdateOwn`)
- Services: `PermissionService`, `OwnershipChecker`
- Middleware: `require_permission()`
- Routes: `/api/v1/admin/permissions/*`

### Testing Standards

**Unit Tests:**
- Test permission parsing (string ↔ enum)
- Test permission inheritance (`all` includes `own`)
- Test ownership checks (tickets, workflows)
- Test predicate combinators (And, Or, Not)
- Test cache behavior (TTL, invalidation)

**Integration Tests:**
- Test permission middleware on protected routes
- Test 403 Forbidden response when permission denied
- Test permission grant/revoke via API
- Test cache invalidation on permission changes
- Test resource ownership checks with database

**Test Files:**
- `crates/qa-pms-core/src/permissions.rs` - `#[cfg(test)]` module
- `crates/qa-pms-api/src/middleware/permissions.rs` - `#[cfg(test)]` module
- `crates/qa-pms-api/tests/permissions_integration.rs` - integration tests

**Test Coverage Target:**
- Minimum 80% coverage for permission logic
- 100% coverage for middleware error paths
- Integration tests for all API endpoints

### References

- **Source: `_bmad-output/planning-artifacts/epics-detailed/epic-15-authentication-authorization.md#story-15.8`** - Story requirements and acceptance criteria
- **Source: `_bmad-output/planning-artifacts/architecture.md#middleware-pattern`** - Axum middleware patterns
- **Source: `_bmad-output/implementation-artifacts/15-3-role-based-access-control-rbac.md`** - RBAC implementation (extends this)
- **Source: `_bmad-output/implementation-artifacts/15-7-api-key-authentication.md`** - Middleware pattern reference
- **Source: `qa-intelligent-pms/crates/qa-pms-api/src/app.rs`** - AppState structure and middleware setup
- **Source: `qa-intelligent-pms/crates/qa-pms-core/src/health.rs`** - Service pattern example
- **Source: Axum docs (`/tokio-rs/axum`) - middleware examples** - Tower Layer patterns
- **Source: `_bmad-output/planning-artifacts/project-context.md`** - Rust patterns, error handling, logging

## Dev Agent Record

### Agent Model Used

Claude Sonnet 4.5 (via Cursor)

### Debug Log References

(None yet - story not implemented)

### Completion Notes List

(None yet - story not implemented)

### File List

**Created:**
- `crates/qa-pms-core/src/permissions.rs` - Permission enum, service, predicates
- `crates/qa-pms-api/src/middleware/permissions.rs` - Permission middleware
- `crates/qa-pms-api/src/routes/permissions.rs` - Permission management API
- `migrations/YYYYMMDDHHMMSS_permissions_schema.sql` - Database schema

**Modified:**
- `crates/qa-pms-core/src/lib.rs` - Export permissions module
- `crates/qa-pms-core/Cargo.toml` - Add moka dependency
- `crates/qa-pms-api/src/app.rs` - Add PermissionService to AppState
- `crates/qa-pms-api/src/routes/mod.rs` - Add permissions router
- `crates/qa-pms-api/src/routes/tickets.rs` - Apply permission middleware
- `crates/qa-pms-api/src/routes/workflows.rs` - Apply permission middleware
