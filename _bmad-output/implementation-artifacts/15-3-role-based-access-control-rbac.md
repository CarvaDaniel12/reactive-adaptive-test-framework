# Story 15.3: Role-Based Access Control (RBAC)

Status: ready-for-dev

## Story

As a System Administrator,
I want to define roles and permissions,
So that users have appropriate access based on their responsibilities.

## Acceptance Criteria

1. **Given** RBAC system is configured with roles and permissions
   **When** a user is created or updated
   **Then** the user is assigned a role
   **And** inherits all permissions from that role
   **And** permissions are stored as JSONB array

2. **Given** a user with role "QA Engineer" attempts to access admin endpoint
   **When** the request is made
   **Then** the system checks if role has required permission
   **And** returns 403 Forbidden if not authorized
   **And** logs unauthorized access attempt

3. **Given** an admin user
   **When** they want to create custom role
   **Then** they can select from available permissions
   **And** save role with descriptive name
   **And** assign users to the role

4. **Given** a role is deleted or modified
   **When** changes are applied
   **Then** all users with that role reflect changes immediately
   **And** cached permissions are invalidated
   **And** users must re-authenticate if needed

5. **Given** a permission check is performed
   **When** evaluating access
   **Then** system checks both direct user permissions AND role permissions
   **And** grants access if either has permission
   **And** returns false if neither has permission

6. **Given** multiple roles exist
   **When** a user has multiple roles
   **Then** user has union of all permissions from all roles
   **And** conflicts are resolved (allow wins over deny)
   **And** permission set is calculated once per session

7. **Given** predefined roles exist
   **When** system is initialized
   **Then** default roles are created (admin, qa_lead, qa_engineer, pm_po, viewer, service_account)
   **And** each role has appropriate permissions
   **And** roles are marked as system roles (cannot be deleted)

8. **Given** hierarchical permissions exist
   **When** checking for permission like "tickets.update.own"
   **Then** system interprets as update own tickets
   **And** checks resource ownership
   **And** returns true if user is resource owner

9. **Given** permission cache is implemented
   **When** checking permissions
   **Then** results are cached for performance
   **And** cache is invalidated on role/permission changes
   **And** cache TTL is 5 minutes

10. **Given** admin wants to audit permissions
   **When** viewing permission audit log
   **Then** all permission checks are logged
   **And** includes user, resource, action, result
   **And** shows timestamp and IP address

## Tasks / Subtasks

- [ ] Task 1: Setup RBAC database schema (AC: #1, #3, #7)
  - [ ] 1.1: Create `roles` table migration
  - [ ] 1.2: Add name, description, permissions, is_system columns
  - [ ] 1.3: Create `user_roles` table migration
  - [ ] 1.4: Add user_id, role_id, assigned_at, assigned_by columns
  - [ ] 1.5: Add foreign keys and indexes
  - [ ] 1.6: Insert predefined roles in migration
  - [ ] 1.7: Add unit tests for schema

- [ ] Task 2: Define permission enum and structure (AC: #5, #8)
  - [ ] 2.1: Create `Permission` enum in `qa-pms-auth`
  - [ ] 2.2: Define all permission constants
  - [ ] 2.3: Implement hierarchical permissions (tickets.update.own)
  - [ ] 2.4: Create permission hierarchy utilities
  - [ ] 2.5: Add unit tests for permission structure

- [ ] Task 3: Implement RBAC service (AC: #1, #4, #6)
  - [ ] 3.1: Create `RbacService` in `qa-pms-auth`
  - [ ] 3.2: Implement `assign_role_to_user()`
  - [ ] 3.3: Implement `remove_role_from_user()`
  - [ ] 3.4: Implement `get_user_permissions()` with role inheritance
  - [ ] 3.5: Implement `has_permission()` check
  - [ ] 3.6: Handle multiple roles (union of permissions)
  - [ ] 3.7: Add unit tests for RBAC logic

- [ ] Task 4: Create permission middleware (AC: #2, #9)
  - [ ] 4.1: Create `require_permission()` middleware factory
  - [ ] 4.2: Extract user context from request
  - [ ] 4.3: Check permission against user's permission set
  - [ ] 4.4: Return 403 on unauthorized
  - [ ] 4.5: Implement caching layer for permission checks
  - [ ] 4.6: Add cache invalidation on role changes
  - [ ] 4.7: Add unit tests for middleware

- [ ] Task 5: Implement resource ownership checks (AC: #8)
  - [ ] 5.1: Create `check_resource_ownership()` function
  - [ ] 5.2: Implement ownership checks for tickets
  - [ ] 5.3: Implement ownership checks for workflows
  - [ ] 5.4: Support resource type parameter
  - [ ] 5.5: Return true if user has admin permission
  - [ ] 5.6: Add unit tests for ownership logic

- [ ] Task 6: Create role management API endpoints (AC: #3, #4)
  - [ ] 6.1: Add `GET /api/v1/admin/roles` endpoint
  - [ ] 6.2: Add `POST /api/v1/admin/roles` endpoint
  - [ ] 6.3: Add `PUT /api/v1/admin/roles/{id}` endpoint
  - [ ] 6.4: Add `DELETE /api/v1/admin/roles/{id}` endpoint
  - [ ] 6.5: Protect all endpoints with admin permission
  - [ ] 6.6: Add OpenAPI documentation
  - [ ] 6.7: Add integration tests for CRUD

- [ ] Task 7: Create user role assignment endpoints (AC: #1, #3)
  - [ ] 7.1: Add `POST /api/v1/admin/users/{user_id}/roles` endpoint
  - [ ] 7.2: Add `DELETE /api/v1/admin/users/{user_id}/roles/{role_id}` endpoint
  - [ ] 7.3: Validate user exists
  - [ ] 7.4: Validate role exists
  - [ ] 7.5: Log all role changes
  - [ ] 7.6: Add OpenAPI documentation

- [ ] Task 8: Implement permission audit logging (AC: #10)
  - [ ] 8.1: Create `permission_audit_log` table
  - [ ] 8.2: Log every permission check
  - [ ] 8.3: Include user_id, permission, resource, result
  - [ ] 8.4: Include timestamp, IP address
  - [ ] 8.5: Create audit query endpoint
  - [ ] 8.6: Add retention policy (90 days)
  - [ ] 8.7: Add unit tests for audit logging

- [ ] Task 9: Create RBAC management UI (AC: #3, #4, #10)
  - [ ] 9.1: Create `RolesManagement` page
  - [ ] 9.2: Create `RoleList` component
  - [ ] 9.3: Create `RoleForm` for creating/editing roles
  - [ ] 9.4: Create `PermissionCheckboxGroup` component
  - [ ] 9.5: Create `UserRoles` component
  - [ ] 9.6: Show permission audit log
  - [ ] 9.7: Add role deletion confirmation
  - [ ] 9.8: Show system role indicator

- [ ] Task 10: Implement permission caching (AC: #9)
  - [ ] 10.1: Create `PermissionCache` struct
  - [ ] 10.2: Use Redis or in-memory cache
  - [ ] 10.3: Implement cache key generation
  - [ ] 10.4: Set 5-minute TTL
  - [ ] 10.5: Implement cache invalidation on changes
  - [ ] 10.6: Add cache hit/miss metrics
  - [ ] 10.7: Add unit tests for caching

- [ ] Task 11: Add hierarchical permission support (AC: #8)
  - [ ] 11.1: Create permission tree structure
  - [ ] 11.2: Implement `tickets.update.own` logic
  - [ ] 11.3: Implement `tickets.view.all` vs `tickets.view.own`
  - [ ] 11.4: Add wildcard permissions support (`tickets.*`)
  - [ ] 11.5: Implement permission expansion
  - [ ] 11.6: Add unit tests for hierarchy

- [ ] Task 12: Integrate with authentication middleware (AC: #2, #6)
  - [ ] 12.1: Load user permissions into UserContext
  - [ ] 12.2: Extend auth middleware with RBAC
  - [ ] 12.3: Add permission checks to auth
  - [ ] 12.4: Update login flow to include roles
  - [ ] 12.5: Add role to JWT claims
  - [ ] 12.6: Add integration tests

## Dev Notes

### Architecture Alignment

This story implements **Role-Based Access Control (RBAC)** per Epic 15 requirements:

- **Backend Location**: `crates/qa-pms-auth/src/rbac/`
- **Middleware**: `crates/qa-pms-auth/src/middleware/require_permission.rs`
- **Security**: Fine-grained permissions, role inheritance, audit logging
- **Performance**: Permission caching with Redis/in-memory

### Technical Implementation Details

#### Dependencies to Add

```toml
# crates/qa-pms-auth/Cargo.toml
[dependencies]
# Existing
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.35", features = ["full"] }
axum = "0.7"

# New for RBAC
cached = "0.49"
async-trait = "0.1"

# Optional: Redis for distributed caching
redis = { version = "0.24", optional = true }
deadpool-redis = { version = "0.14", optional = true }
```

#### Database Schema

```sql
-- Roles table
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    permissions TEXT[] NOT NULL, -- JSONB array of permissions
    is_system BOOLEAN DEFAULT FALSE, -- Cannot be deleted
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- User roles junction table
CREATE TABLE user_roles (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    assigned_at TIMESTAMP DEFAULT NOW(),
    assigned_by UUID REFERENCES users(id),
    PRIMARY KEY (user_id, role_id)
);

-- Permission audit log
CREATE TABLE permission_audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    permission TEXT NOT NULL,
    resource_type TEXT,
    resource_id TEXT,
    action TEXT, -- GRANTED | DENIED
    result TEXT,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_user_roles_user_id ON user_roles(user_id);
CREATE INDEX idx_user_roles_role_id ON user_roles(role_id);
CREATE INDEX idx_permission_audit_user_id ON permission_audit_log(user_id);
CREATE INDEX idx_permission_audit_permission ON permission_audit_log(permission);
CREATE INDEX idx_permission_audit_created_at ON permission_audit_log(created_at);

-- Predefined roles
INSERT INTO roles (name, description, permissions, is_system) VALUES
('admin', 'Full system access', ARRAY['*'], TRUE),
('qa_lead', 'Team management, reports', ARRAY['qa.*', 'reports.*', 'users.view'], TRUE),
('qa_engineer', 'Execute workflows, view tickets', ARRAY['workflows.*', 'tickets.*', 'time.*'], TRUE),
('pm_po', 'View-only dashboards, metrics', ARRAY['dashboards.view', 'reports.view', 'patterns.view'], TRUE),
('viewer', 'Read-only access', ARRAY['dashboards.view', 'tickets.view'], TRUE),
('service_account', 'API access only', ARRAY['api.read', 'api.write'], TRUE);
```

#### Permission Enum and Structure

```rust
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumIter, IntoStaticStr, Serialize, Deserialize)]
pub enum Permission {
    // Ticket permissions
    #[strum(serialize = "tickets.view")]
    TicketsView,
    #[strum(serialize = "tickets.view.own")]
    TicketsViewOwn,
    #[strum(serialize = "tickets.view.all")]
    TicketsViewAll,
    #[strum(serialize = "tickets.update")]
    TicketsUpdate,
    #[strum(serialize = "tickets.update.own")]
    TicketsUpdateOwn,
    #[strum(serialize = "tickets.update.all")]
    TicketsUpdateAll,
    #[strum(serialize = "tickets.delete")]
    TicketsDelete,

    // Workflow permissions
    #[strum(serialize = "workflows.view")]
    WorkflowsView,
    #[strum(serialize = "workflows.execute")]
    WorkflowsExecute,
    #[strum(serialize = "workflows.manage")]
    WorkflowsManage,

    // Report permissions
    #[strum(serialize = "reports.view")]
    ReportsView,
    #[strum(serialize = "reports.generate")]
    ReportsGenerate,
    #[strum(serialize = "reports.export")]
    ReportsExport,

    // User management
    #[strum(serialize = "users.view")]
    UsersView,
    #[strum(serialize = "users.manage")]
    UsersManage,

    // System permissions
    #[strum(serialize = "system.config")]
    SystemConfig,
    #[strum(serialize = "system.audit")]
    SystemAudit,

    // API permissions
    #[strum(serialize = "api.read")]
    ApiRead,
    #[strum(serialize = "api.write")]
    ApiWrite,
}

impl Permission {
    /// Check if this permission allows updating any resource
    pub fn is_update_any(&self) -> bool {
        matches!(self, Permission::TicketsUpdateAll | Permission::WorkflowsManage)
    }

    /// Get hierarchical parent permission
    pub fn parent(&self) -> Option<Self> {
        match self {
            Permission::TicketsUpdateOwn | Permission::TicketsUpdateAll => Some(Permission::TicketsUpdate),
            Permission::TicketsViewOwn | Permission::TicketsViewAll => Some(Permission::TicketsView),
            _ => None,
        }
    }
}
```

#### RBAC Service

```rust
use sqlx::PgPool;
use std::collections::HashSet;
use std::sync::Arc;

pub struct RbacService {
    db: Arc<PgPool>,
    cache: Arc<PermissionCache>,
}

impl RbacService {
    pub async fn assign_role_to_user(
        &self,
        user_id: &Uuid,
        role_id: &Uuid,
        assigned_by: &Uuid,
    ) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role_id, assigned_by)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, role_id) DO NOTHING
            "#
        )
        .bind(user_id, role_id, assigned_by)
        .execute(&*self.db)
        .await?;

        // Invalidate cache for this user
        self.cache.invalidate_user(user_id).await;

        Ok(())
    }

    pub async fn get_user_permissions(
        &self,
        user_id: &Uuid,
    ) -> Result<HashSet<Permission>, DbError> {
        // Check cache first
        if let Some(cached) = self.cache.get_user_permissions(user_id).await {
            return Ok(cached);
        }

        // Fetch user's roles
        let roles = sqlx::query!(
            r#"
            SELECT r.permissions
            FROM user_roles ur
            JOIN roles r ON ur.role_id = r.id
            WHERE ur.user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_all(&*self.db)
        .await?;

        // Flatten and deduplicate permissions
        let mut permissions = HashSet::new();
        for role in roles {
            for perm in role.permissions {
                if let Ok(permission) = perm.parse::<Permission>() {
                    // Add permission
                    permissions.insert(permission);
                    
                    // Add parent permission (if any)
                    if let Some(parent) = permission.parent() {
                        permissions.insert(parent);
                    }
                }
            }
        }

        // Cache the result
        self.cache.set_user_permissions(user_id, permissions.clone()).await;

        Ok(permissions)
    }

    pub async fn has_permission(
        &self,
        user_id: &Uuid,
        required_permission: Permission,
        resource_ownership_check: Option<OwnershipCheck>,
    ) -> Result<bool, DbError> {
        let permissions = self.get_user_permissions(user_id).await?;

        // Check wildcard permission
        if permissions.contains(&Permission::SystemConfig) {
            return Ok(true);
        }

        // Check exact permission
        if permissions.contains(&required_permission) {
            return Ok(true);
        }

        // Check hierarchical permissions
        if let Some(parent) = required_permission.parent() {
            if permissions.contains(&parent) {
                // If parent exists, check resource ownership
                if let Some(ownership) = resource_ownership_check {
                    return self.check_resource_ownership(user_id, ownership).await;
                }
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub async fn check_resource_ownership(
        &self,
        user_id: &Uuid,
        ownership: OwnershipCheck,
    ) -> Result<bool, DbError> {
        match ownership.resource_type {
            ResourceType::Ticket => {
                let owner = sqlx::query!(
                    r#"SELECT created_by FROM tickets WHERE id = $1"#
                )
                .bind(&ownership.resource_id)
                .fetch_optional(&*self.db)
                .await?;

                Ok(owner.map(|owner_id| owner_id == *user_id).unwrap_or(false))
            }
            ResourceType::Workflow => {
                let owner = sqlx::query!(
                    r#"SELECT created_by FROM workflows WHERE id = $1"#
                )
                .bind(&ownership.resource_id)
                .fetch_optional(&*self.db)
                .await?;

                Ok(owner.map(|owner_id| owner_id == *user_id).unwrap_or(false))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct OwnershipCheck {
    pub resource_type: ResourceType,
    pub resource_id: Uuid,
}

#[derive(Debug, Clone)]
pub enum ResourceType {
    Ticket,
    Workflow,
}
```

#### Permission Middleware

```rust
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
    Extension,
};
use std::sync::Arc;

pub fn require_permission(
    permission: Permission,
    require_ownership: bool,
) -> impl Fn(Request, Next) -> Result<Response, StatusCode> {
    move |req: Request, next: Next| {
        let rbac_service = req.extensions().get::<Arc<RbacService>>().unwrap();
        let user_context = req.extensions().get::<UserContext>().unwrap();
        let ownership_check = req.extensions().get::<OwnershipCheck>().cloned();

        async move {
            let has_perm = rbac_service
                .has_permission(&user_context.user_id, permission, ownership_check)
                .await
                .unwrap_or(false);

            if !has_perm {
                // Log denied access
                log_permission_denied(
                    &user_context.user_id,
                    &permission,
                    req.uri().path(),
                ).await;

                return Ok(Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body("Insufficient permissions".into())
                    .unwrap());
            }

            Ok(next.run(req).await)
        }
    }
}

// Usage in routes
router.route(
    "/api/v1/tickets/:id",
    patch(update_ticket_handler)
        .layer(require_permission(Permission::TicketsUpdate, true)),
)
```

#### Permission Cache

```rust
use cached::AsyncCache;
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct PermissionCache {
    cache: Arc<RwLock<AsyncCache<Uuid, HashSet<Permission>>>>,
}

impl PermissionCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(
                AsyncCache::new(Duration::from_secs(300)) // 5 minutes
            ))
        ),
        }
    }

    pub async fn get_user_permissions(
        &self,
        user_id: &Uuid,
    ) -> Option<HashSet<Permission>> {
        self.cache.read().await.get(user_id).copied()
    }

    pub async fn set_user_permissions(
        &self,
        user_id: &Uuid,
        permissions: HashSet<Permission>,
    ) {
        self.cache.write().await.insert(*user_id, permissions).await;
    }

    pub async fn invalidate_user(&self, user_id: &Uuid) {
        self.cache.write().await.remove(user_id).await;
    }

    pub async fn invalidate_all(&self) {
        self.cache.write().await.flush().await;
    }
}
```

#### Permission Audit Logging

```rust
use tracing::{info, error};

pub async fn log_permission_denied(
    user_id: &Uuid,
    permission: &Permission,
    resource: &str,
) {
    info!(
        user_id = %user_id,
        permission = %permission,
        resource = %resource,
        action = "DENIED",
        "Permission check failed"
    );

    // Insert into audit log
    sqlx::query!(
        r#"
        INSERT INTO permission_audit_log (user_id, permission, resource_type, action, result)
        VALUES ($1, $2, $3, $4, $5)
        "#
    )
    .bind(user_id, &permission.to_string(), resource, "DENIED")
    .execute(&db_pool)
    .await
    .ok_or_else(|e| error!("Failed to log permission denial: {}", e));
}
```

### Testing Strategy

#### Unit Tests

- Permission enum parsing and hierarchy
- Permission cache operations
- RBAC service logic (has_permission, get_user_permissions)
- Resource ownership checks
- Permission middleware logic
- Audit logging functions

#### Integration Tests

- Role creation, update, deletion
- User role assignment/removal
- Permission checks with various roles
- Permission cache invalidation
- Audit log recording and querying

#### End-to-End Tests

- Admin creates custom role
- Admin assigns role to user
- User with role accesses permitted resource
- User without role gets 403
- User with partial permission (view.own) checks ownership
- Admin views permission audit log

#### Security Tests

- User without permission attempts access
- User with old permissions (after role change)
- Permission escalation attempt
- Audit log tampering resistance
- Cache poisoning resistance

### File List

**Files to be created:**
- `crates/qa-pms-auth/src/rbac/mod.rs`
- `crates/qa-pms-auth/src/rbac/service.rs`
- `crates/qa-pms-auth/src/rbac/middleware.rs`
- `crates/qa-pms-auth/src/rbac/permission.rs`
- `crates/qa-pms-auth/src/rbac/cache.rs`
- `migrations/create_roles_table.sql`
- `migrations/create_user_roles_table.sql`
- `migrations/create_permission_audit_log_table.sql`
- `migrations/insert_predefined_roles.sql`
- `frontend/src/pages/admin/RolesManagement.tsx`
- `frontend/src/components/admin/RoleForm.tsx`
- `frontend/src/components/admin/UserRoles.tsx`
- `frontend/src/components/admin/PermissionCheckboxGroup.tsx`

**Files to be modified:**
- `crates/qa-pms-auth/Cargo.toml` (add cached dependency)
- `crates/qa-pms-api/src/main.rs` (add RBAC routes and middleware)
- `frontend/src/api/admin.ts` (add RBAC API calls)