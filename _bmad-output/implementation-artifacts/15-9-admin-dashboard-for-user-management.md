# Story 15.9: Admin Dashboard for User Management

Status: ready-for-dev

Epic: 15 - Authentication & Authorization
Priority: P0 (Critical for Security Management)
Estimated Effort: 3 days
Sprint: 1

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a **System Administrator**,
I want to **manage users, roles, and permissions via UI**,
So that **I can maintain system security without writing SQL or API calls**.

## Acceptance Criteria

1. **Given** I have admin access
   **When** I access User Management Dashboard
   **Then** I see list of all users with filters
   **And** I can view user details, roles, permissions
   **And** I can create new users
   **And** I can assign/modify roles
   **And** I can reset passwords
   **And** I can revoke access

2. **Given** I'm viewing user list
   **When** I filter by role, status, or search by email
   **Then** list updates in real-time
   **And** I can export user list as CSV
   **And** filters persist in URL parameters

3. **Given** I need to audit user changes
   **When** I view user history
   **Then** I see all changes (role assignments, password resets, etc.)
   **And** I see who made the change and when
   **And** I can see detailed change context

4. **Given** I want to create a new user
   **When** I fill in user form (name, email, roles)
   **Then** system validates input (email format, required fields)
   **And** user is created with default password
   **And** password reset email is sent
   **And** user appears in list immediately

5. **Given** I want to modify user roles
   **When** I select user and assign roles
   **Then** roles are updated immediately
   **And** permission cache is invalidated
   **And** user's session is refreshed with new permissions
   **And** change is logged in audit trail

6. **Given** I want to reset user password
   **When** I click "Reset Password" for a user
   **Then** system generates temporary password
   **And** password reset email is sent
   **And** user is required to change password on next login
   **And** action is logged in audit trail

7. **Given** I want to revoke user access
   **When** I deactivate or delete a user
   **Then** user cannot log in
   **And** all active sessions are revoked
   **And** action requires confirmation
   **And** action is logged in audit trail

8. **Given** I need to view user details
   **When** I click on a user
   **Then** I see user profile information
   **And** I see assigned roles with permissions breakdown
   **And** I see active sessions (with revoke option)
   **And** I see login history (last 10 logins)
   **And** I see audit trail (all changes)

9. **Given** I want to manage roles
   **When** I access Role Management section
   **Then** I see list of all roles with descriptions
   **And** I see permissions per role
   **And** I can create custom roles
   **And** I can edit role permissions
   **And** I can assign users to roles

10. **Given** I want to perform bulk actions
    **When** I select multiple users
    **Then** I can bulk assign roles
    **And** I can bulk deactivate users
    **And** I can bulk export user data
    **And** bulk actions require confirmation

## Tasks / Subtasks

- [ ] Task 1: Create admin user management API endpoints (AC: #1, #4, #5, #6, #7)
  - [ ] 1.1: Create `crates/qa-pms-api/src/routes/admin/users.rs` module
  - [ ] 1.2: Add `GET /api/v1/admin/users` endpoint with pagination and filters
  - [ ] 1.3: Add query parameters: `?role=qa_engineer&status=active&search=john&page=1&limit=20`
  - [ ] 1.4: Add `GET /api/v1/admin/users/:user_id` endpoint for user details
  - [ ] 1.5: Add `POST /api/v1/admin/users` endpoint for creating users
  - [ ] 1.6: Add `PUT /api/v1/admin/users/:user_id` endpoint for updating users
  - [ ] 1.7: Add `POST /api/v1/admin/users/:user_id/reset-password` endpoint
  - [ ] 1.8: Add `DELETE /api/v1/admin/users/:user_id` endpoint (soft delete or deactivate)
  - [ ] 1.9: Add `POST /api/v1/admin/users/:user_id/assign-role` endpoint
  - [ ] 1.10: Add `DELETE /api/v1/admin/users/:user_id/roles/:role_id` endpoint
  - [ ] 1.11: Add `GET /api/v1/admin/users/:user_id/sessions` endpoint
  - [ ] 1.12: Add `DELETE /api/v1/admin/users/:user_id/sessions` endpoint (revoke all)
  - [ ] 1.13: Add `DELETE /api/v1/admin/users/:user_id/sessions/:session_id` endpoint
  - [ ] 1.14: Add `GET /api/v1/admin/users/:user_id/audit-log` endpoint
  - [ ] 1.15: Add `POST /api/v1/admin/users/bulk-assign-role` endpoint
  - [ ] 1.16: Add `POST /api/v1/admin/users/bulk-deactivate` endpoint
  - [ ] 1.17: Protect all endpoints with `Permission::AdminUsers` (from Story 15.8)
  - [ ] 1.18: Add OpenAPI documentation with `utoipa`
  - [ ] 1.19: Add integration tests for all endpoints

- [ ] Task 2: Create user management repository/service layer (AC: #1, #4, #5, #6, #7)
  - [ ] 2.1: Create `crates/qa-pms-core/src/admin/users.rs` module
  - [ ] 2.2: Create `UserManagementService` struct with database pool
  - [ ] 2.3: Implement `list_users(filters, pagination)` method
  - [ ] 2.4: Implement `get_user_by_id(user_id)` method
  - [ ] 2.5: Implement `create_user(name, email, roles)` method
  - [ ] 2.6: Generate temporary password for new users (crypto-secure)
  - [ ] 2.7: Hash password with Argon2id before storage
  - [ ] 2.8: Send password reset email via email service (Story 15.5)
  - [ ] 2.9: Implement `update_user(user_id, fields)` method
  - [ ] 2.10: Implement `assign_role_to_user(user_id, role_id)` method
  - [ ] 2.11: Implement `remove_role_from_user(user_id, role_id)` method
  - [ ] 2.12: Implement `reset_user_password(user_id)` method
  - [ ] 2.13: Implement `deactivate_user(user_id)` method
  - [ ] 2.14: Implement `delete_user(user_id)` method (soft delete)
  - [ ] 2.15: Implement `get_user_sessions(user_id)` method
  - [ ] 2.16: Implement `revoke_user_sessions(user_id)` method
  - [ ] 2.17: Implement `get_user_audit_log(user_id)` method
  - [ ] 2.18: Integrate with `PermissionService` for permission cache invalidation (Story 15.8)
  - [ ] 2.19: Log all admin actions to audit trail (Story 15.10)
  - [ ] 2.20: Add unit tests for service layer

- [ ] Task 3: Create role management API endpoints (AC: #9)
  - [ ] 3.1: Create `crates/qa-pms-api/src/routes/admin/roles.rs` module
  - [ ] 3.2: Add `GET /api/v1/admin/roles` endpoint
  - [ ] 3.3: Add `GET /api/v1/admin/roles/:role_id` endpoint with permissions
  - [ ] 3.4: Add `POST /api/v1/admin/roles` endpoint for creating roles
  - [ ] 3.5: Add `PUT /api/v1/admin/roles/:role_id` endpoint for updating roles
  - [ ] 3.6: Add `DELETE /api/v1/admin/roles/:role_id` endpoint (prevent deletion of system roles)
  - [ ] 3.7: Add `POST /api/v1/admin/roles/:role_id/assign-user` endpoint
  - [ ] 3.8: Protect all endpoints with `Permission::AdminRoles` (from Story 15.8)
  - [ ] 3.9: Add OpenAPI documentation
  - [ ] 3.10: Add integration tests

- [ ] Task 4: Create User Management frontend page (AC: #1, #2)
  - [ ] 4.1: Create `frontend/src/pages/Admin/UserManagementPage.tsx`
  - [ ] 4.2: Create `frontend/src/pages/Admin/UserList.tsx` component
  - [ ] 4.3: Create `frontend/src/pages/Admin/UserFilters.tsx` component
  - [ ] 4.4: Create `frontend/src/pages/Admin/UserTable.tsx` component with columns: Name, Email, Role, Status, Last Login, Created At, Actions
  - [ ] 4.5: Implement pagination using React Query
  - [ ] 4.6: Implement filters (role, status, search) with URL persistence
  - [ ] 4.7: Implement real-time filtering (debounced search)
  - [ ] 4.8: Add loading states and skeletons
  - [ ] 4.9: Add error handling and toast notifications
  - [ ] 4.10: Integrate with Zustand store for state management (if needed)
  - [ ] 4.11: Add permission check (hide if user doesn't have `AdminUsers` permission)
  - [ ] 4.12: Add export CSV functionality

- [ ] Task 5: Create User Detail view component (AC: #8)
  - [ ] 5.1: Create `frontend/src/pages/Admin/UserDetailPage.tsx`
  - [ ] 5.2: Create `frontend/src/components/admin/UserProfile.tsx` component
  - [ ] 5.3: Create `frontend/src/components/admin/UserRoles.tsx` component (show assigned roles with permissions)
  - [ ] 5.4: Create `frontend/src/components/admin/UserSessions.tsx` component (active sessions with revoke)
  - [ ] 5.5: Create `frontend/src/components/admin/UserLoginHistory.tsx` component (last 10 logins)
  - [ ] 5.6: Create `frontend/src/components/admin/UserAuditTrail.tsx` component (all changes)
  - [ ] 5.7: Implement edit profile functionality
  - [ ] 5.8: Implement assign/remove role functionality
  - [ ] 5.9: Implement reset password functionality (with confirmation)
  - [ ] 5.10: Implement deactivate/delete functionality (with confirmation)
  - [ ] 5.11: Add loading states and error handling

- [ ] Task 6: Create User form component for create/edit (AC: #4)
  - [ ] 6.1: Create `frontend/src/components/admin/UserForm.tsx` component
  - [ ] 6.2: Add form fields: Name (required), Email (required, validated), Roles (multi-select)
  - [ ] 6.3: Implement email validation (format check)
  - [ ] 6.4: Implement role multi-select using Radix UI Select
  - [ ] 6.5: Add form validation with error messages
  - [ ] 6.6: Add submit handler for create/update
  - [ ] 6.7: Use Radix Dialog for modal form
  - [ ] 6.8: Add loading state during submission
  - [ ] 6.9: Show success toast and refresh list on success
  - [ ] 6.10: Handle API errors gracefully

- [ ] Task 7: Create Role Management UI components (AC: #9)
  - [ ] 7.1: Create `frontend/src/pages/Admin/RoleManagementPage.tsx`
  - [ ] 7.2: Create `frontend/src/components/admin/RoleList.tsx` component
  - [ ] 7.3: Create `frontend/src/components/admin/RoleForm.tsx` component
  - [ ] 7.4: Create `frontend/src/components/admin/PermissionCheckboxGroup.tsx` component
  - [ ] 7.5: Show permissions per role with hierarchical display
  - [ ] 7.6: Implement create/edit role functionality
  - [ ] 7.7: Implement assign users to role functionality
  - [ ] 7.8: Prevent deletion of system roles (show warning)
  - [ ] 7.9: Add permission check (hide if user doesn't have `AdminRoles` permission)

- [ ] Task 8: Implement bulk actions functionality (AC: #10)
  - [ ] 8.1: Add checkbox column to UserTable for selection
  - [ ] 8.2: Create `frontend/src/components/admin/BulkActions.tsx` component
  - [ ] 8.3: Implement bulk role assignment (select role, assign to selected users)
  - [ ] 8.4: Implement bulk deactivate (with confirmation dialog)
  - [ ] 8.5: Implement bulk export CSV (export selected users)
  - [ ] 8.6: Add loading state for bulk operations
  - [ ] 8.7: Show success toast with count of affected users
  - [ ] 8.8: Clear selection after bulk operation

- [ ] Task 9: Integrate with audit trail system (AC: #3)
  - [ ] 9.1: Log all user creation events to audit trail
  - [ ] 9.2: Log all role assignment/removal events
  - [ ] 9.3: Log all password reset events
  - [ ] 9.4: Log all user deactivation/deletion events
  - [ ] 9.5: Include admin user ID in audit log (who made the change)
  - [ ] 9.6: Include timestamp and change context in audit log
  - [ ] 9.7: Display audit trail in UserDetailPage
  - [ ] 9.8: Filter audit log by user, event type, date range

- [ ] Task 10: Add session management integration (AC: #8)
  - [ ] 10.1: Query user sessions from database (Story 15.6 session management)
  - [ ] 10.2: Display active sessions with metadata (IP, user agent, last activity)
  - [ ] 10.3: Implement revoke single session functionality
  - [ ] 10.4: Implement revoke all sessions functionality (with confirmation)
  - [ ] 10.5: Show session expiration time
  - [ ] 10.6: Add refresh button to update session list
  - [ ] 10.7: Integrate with session management API endpoints (Story 15.6)

- [ ] Task 11: Add routing and navigation (AC: #1, #9)
  - [ ] 11.1: Add `/admin/users` route to React Router
  - [ ] 11.2: Add `/admin/users/:userId` route for user detail
  - [ ] 11.3: Add `/admin/roles` route for role management
  - [ ] 11.4: Add navigation links in Sidebar or Header (only visible to admins)
  - [ ] 11.5: Add breadcrumbs for navigation context
  - [ ] 11.6: Add permission-based route guards (redirect if no permission)

- [ ] Task 12: Add comprehensive tests (AC: All)
  - [ ] 12.1: Add unit tests for UserManagementService
  - [ ] 12.2: Add integration tests for admin API endpoints
  - [ ] 12.3: Add React component tests for UserList, UserForm, UserDetail
  - [ ] 12.4: Add E2E tests for user management workflow (create, edit, delete)
  - [ ] 12.5: Test permission enforcement (403 Forbidden without permission)
  - [ ] 12.6: Test bulk operations
  - [ ] 12.7: Test audit trail logging
  - [ ] 12.8: Test session management integration

## Dev Notes

### Architecture Compliance

**Tech Stack:**
- **Backend:** Rust 1.80+ with Tokio async runtime, Axum 0.7+ web framework
- **Database:** PostgreSQL with SQLx 0.7
- **Frontend:** React 19 + Vite 7 + Tailwind CSS v4, Zustand for state management, Radix UI for components
- **Error Handling:** `anyhow` (internal) + `thiserror` (API boundaries)
- **Logging:** `tracing` + `tracing-subscriber` (never `println!`)

**Code Structure:**
- **API Routes:** `crates/qa-pms-api/src/routes/admin/users.rs` (new), `routes/admin/roles.rs` (new)
- **Service Layer:** `crates/qa-pms-core/src/admin/users.rs` (new)
- **Frontend Pages:** `frontend/src/pages/Admin/UserManagementPage.tsx` (new), `pages/Admin/RoleManagementPage.tsx` (new)
- **Frontend Components:** `frontend/src/components/admin/*` (new directory)
- **Database Migrations:** May need additional migrations for user management features

**Permission Integration:**
All admin endpoints must be protected with permission middleware (from Story 15.8):
- `Permission::AdminUsers` - for user management endpoints
- `Permission::AdminRoles` - for role management endpoints

Use `require_permission()` middleware factory:
```rust
router.route("/api/v1/admin/users", get(list_users))
    .route_layer(middleware::from_fn_with_state(
        state.clone(),
        require_permission(Permission::AdminUsers)
    ));
```

**Frontend Permission Checks:**
Hide admin navigation and pages if user doesn't have required permissions:
```typescript
const { user } = useAuth();
if (!user?.permissions.includes('admin.users')) {
  return <Navigate to="/" replace />;
}
```

### Previous Story Intelligence

**From Story 15.3 (RBAC):**
- Already have `RbacService` with role management
- Already have `roles` and `user_roles` tables
- Already have `Permission` enum and `PermissionService`
- This story adds **UI layer** for managing RBAC system

**From Story 15.8 (Permission System):**
- Permission system is already implemented
- Permission middleware exists for protecting endpoints
- Permission cache (Moka) for performance
- This story uses `Permission::AdminUsers` and `Permission::AdminRoles`

**Key Integration Points:**
- Reuse `RbacService` from Story 15.3 for role operations
- Use `PermissionService` from Story 15.8 for permission checks
- Integrate with audit trail from Story 15.10 (if implemented)
- Integrate with session management from Story 15.6 (if implemented)

**From Story 15.5 (Password Policies):**
- Password reset flow exists
- Password hashing with Argon2id
- Password reset email functionality
- This story reuses password reset logic for admin-initiated resets

**Code Patterns to Follow:**
```rust
// From Story 15.3 - Role assignment pattern
rbac_service.assign_role_to_user(user_id, role_id).await?;
```

### Project Structure Notes

**Alignment with unified structure:**
- ✅ Admin routes in `qa-pms-api/src/routes/admin/` (new directory)
- ✅ Service logic in `qa-pms-core/src/admin/` (new directory)
- ✅ Frontend pages in `frontend/src/pages/Admin/` (new directory)
- ✅ Frontend components in `frontend/src/components/admin/` (new directory)

**Files to Create:**
- `crates/qa-pms-api/src/routes/admin/users.rs` (new module)
- `crates/qa-pms-api/src/routes/admin/roles.rs` (new module)
- `crates/qa-pms-core/src/admin/users.rs` (new module)
- `frontend/src/pages/Admin/UserManagementPage.tsx` (new page)
- `frontend/src/pages/Admin/RoleManagementPage.tsx` (new page)
- `frontend/src/components/admin/UserList.tsx` (new component)
- `frontend/src/components/admin/UserForm.tsx` (new component)
- `frontend/src/components/admin/UserDetail.tsx` (new component)
- `frontend/src/components/admin/UserFilters.tsx` (new component)
- `frontend/src/components/admin/BulkActions.tsx` (new component)
- `frontend/src/components/admin/RoleList.tsx` (new component)
- `frontend/src/components/admin/RoleForm.tsx` (new component)
- `frontend/src/components/admin/PermissionCheckboxGroup.tsx` (new component)

**Files to Modify:**
- `crates/qa-pms-api/src/routes/mod.rs` - Add admin routes module
- `crates/qa-pms-api/src/app.rs` - Add admin routes to router, add UserManagementService to AppState
- `crates/qa-pms-core/src/lib.rs` - Export admin module
- `frontend/src/App.tsx` - Add admin routes
- `frontend/src/components/Sidebar.tsx` - Add admin navigation links (permission-based)

**Naming Conventions:**
- API routes: `/api/v1/admin/users`, `/api/v1/admin/roles`
- Components: PascalCase (UserList, UserForm, UserDetail)
- Functions: snake_case in Rust, camelCase in TypeScript
- Files: kebab-case for TypeScript/TSX, snake_case for Rust

### Frontend Patterns

**React Query Integration:**
Following existing pattern from TicketsPage:
```typescript
const { data, isLoading, error } = useQuery({
  queryKey: ['admin', 'users', filters, page],
  queryFn: () => fetchUsers({ filters, page }),
});
```

**Zustand Store (if needed):**
Following pattern from wizardStore:
```typescript
interface AdminStore {
  selectedUsers: string[];
  setSelectedUsers: (ids: string[]) => void;
  // ... other state
}
```

**Radix UI Components:**
- Use Radix Dialog for modals (UserForm, ConfirmDialog)
- Use Radix Select for role selection
- Use Radix Checkbox for bulk selection
- Use Radix Toast for notifications (already integrated)

**Table Implementation:**
Use HTML table or consider React Table library (tanstack/react-table) for sorting/filtering:
- Sortable columns
- Resizable columns (optional)
- Row selection with checkboxes
- Action buttons per row

### Testing Standards

**Unit Tests:**
- Test UserManagementService methods (create, update, assign role, etc.)
- Test validation logic (email format, required fields)
- Test permission checks (403 Forbidden without permission)
- Test audit trail logging

**Integration Tests:**
- Test all admin API endpoints
- Test pagination and filtering
- Test bulk operations
- Test permission enforcement

**Component Tests:**
- Test UserList rendering with data
- Test UserForm validation
- Test UserFilters filtering logic
- Test BulkActions selection and operations

**E2E Tests:**
- Test full user management workflow (create → edit → delete)
- Test role assignment workflow
- Test bulk operations
- Test permission-based access control

**Test Coverage Target:**
- Minimum 80% coverage for service layer
- 100% coverage for permission checks
- Integration tests for all API endpoints

### References

- **Source: `_bmad-output/planning-artifacts/epics-detailed/epic-15-authentication-authorization.md#story-15.9`** - Story requirements and acceptance criteria
- **Source: `_bmad-output/implementation-artifacts/15-3-role-based-access-control-rbac.md`** - RBAC implementation (extends this)
- **Source: `_bmad-output/implementation-artifacts/15-8-permission-system-with-granular-controls.md`** - Permission system implementation
- **Source: `qa-intelligent-pms/frontend/src/pages/Tickets/TicketsPage.tsx`** - Pagination, filtering, and table patterns
- **Source: `qa-intelligent-pms/frontend/src/pages/Setup/steps/SetupComplete.tsx`** - Form validation and submission patterns
- **Source: `qa-intelligent-pms/frontend/src/stores/wizardStore.ts`** - Zustand store patterns
- **Source: `_bmad-output/planning-artifacts/project-context.md`** - Rust patterns, error handling, logging, frontend patterns

## Dev Agent Record

### Agent Model Used

Claude Sonnet 4.5 (via Cursor)

### Debug Log References

(None yet - story not implemented)

### Completion Notes List

(None yet - story not implemented)

### File List

**Created:**
- `crates/qa-pms-api/src/routes/admin/users.rs` - User management API endpoints
- `crates/qa-pms-api/src/routes/admin/roles.rs` - Role management API endpoints
- `crates/qa-pms-core/src/admin/users.rs` - User management service layer
- `frontend/src/pages/Admin/UserManagementPage.tsx` - User management page
- `frontend/src/pages/Admin/RoleManagementPage.tsx` - Role management page
- `frontend/src/components/admin/UserList.tsx` - User list component
- `frontend/src/components/admin/UserTable.tsx` - User table component
- `frontend/src/components/admin/UserFilters.tsx` - User filters component
- `frontend/src/components/admin/UserForm.tsx` - User create/edit form
- `frontend/src/components/admin/UserDetail.tsx` - User detail view
- `frontend/src/components/admin/BulkActions.tsx` - Bulk actions component
- `frontend/src/components/admin/RoleList.tsx` - Role list component
- `frontend/src/components/admin/RoleForm.tsx` - Role create/edit form
- `frontend/src/components/admin/PermissionCheckboxGroup.tsx` - Permission selector

**Modified:**
- `crates/qa-pms-api/src/routes/mod.rs` - Add admin routes module
- `crates/qa-pms-api/src/app.rs` - Add admin routes to router, add UserManagementService to AppState
- `crates/qa-pms-core/src/lib.rs` - Export admin module
- `frontend/src/App.tsx` - Add admin routes
- `frontend/src/components/Sidebar.tsx` - Add admin navigation links (permission-based)

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete structure
- Added all required sections: Story, Metadata, Acceptance Criteria (10 ACs), Tasks (12 tasks with subtasks), Dev Notes, Dev Agent Record, File List
- Converted acceptance criteria from epic format to Given/When/Then format
- Added comprehensive dev notes with architecture patterns, frontend patterns, testing standards
- Added references to previous stories (15.3, 15.5, 15.6, 15.8, 15.10) for integration
- Added file list with all files to create and modify
