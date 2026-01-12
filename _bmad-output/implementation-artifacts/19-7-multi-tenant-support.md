# Story 19.7: Multi-Tenant Support

Status: ready-for-dev

## Story

**As a** System Administrator  
**I want** multi-tenant support  
**So that** I can support multiple organizations in a single deployment

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 19.7 |
| Epic | Epic 19: Advanced Features |
| Sprint | Sprint 3: Multi-Tenancy |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Epic 15 (Authentication) - Complete ✅ |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create tenant/organization structure
   - Add `tenants` table (id, name, slug, config, created_at)
   - Add `tenant_users` table (tenant_id, user_id, role)
   - Support tenant isolation (data segregation)
   - Support tenant-specific configuration

2. Implement tenant isolation
   - Filter data by tenant_id
   - Ensure tenant data isolation
   - Tenant-specific database schemas (optional, future)
   - Tenant-specific configuration

3. Create tenant management
   - Create/update/delete tenants (admin)
   - Tenant user management
   - Tenant settings
   - Tenant billing (future)

4. Implement tenant switching
   - Support multi-tenant users
   - Tenant context in API requests
   - Tenant context in frontend
   - Tenant switcher UI

---

## Acceptance Criteria

- [ ] **Given** multi-tenant support exists  
  **When** accessing data  
  **Then** only tenant-specific data is returned

- [ ] **Given** multi-tenant support exists  
  **When** creating data  
  **Then** data is associated with current tenant

- [ ] **Given** multi-tenant support exists  
  **When** switching tenant  
  **Then** tenant context is updated

- [ ] **Given** multi-tenant support exists  
  **When** managing tenant  
  **Then** tenant configuration is saved

---

## Tasks / Subtasks

- [ ] Task 1: Create tenants database schema
  - [ ] 1.1: Create migration: `YYYYMMDDHHMMSS_create_tenants_tables.sql`
  - [ ] 1.2: Define `tenants` table
  - [ ] 1.3: Define `tenant_users` table
  - [ ] 1.4: Add tenant_id to existing tables (tickets, workflows, etc.)

- [ ] Task 2: Implement tenant isolation
  - [ ] 2.1: Add tenant context middleware
  - [ ] 2.2: Filter queries by tenant_id
  - [ ] 2.3: Ensure tenant isolation in all repositories

- [ ] Task 3: Create tenant management API
  - [ ] 3.1: Create `crates/qa-pms-api/src/routes/tenants.rs`
  - [ ] 3.2: POST /api/v1/tenants - create tenant (admin)
  - [ ] 3.3: GET /api/v1/tenants - list tenants (admin)
  - [ ] 3.4: GET /api/v1/tenants/:id - get tenant
  - [ ] 3.5: POST /api/v1/tenants/switch - switch tenant context

- [ ] Task 4: Create tenant switcher UI
  - [ ] 4.1: Create `frontend/src/components/tenants/TenantSwitcher.tsx`
  - [ ] 4.2: Add tenant switcher to header
  - [ ] 4.3: Display current tenant context

---

## Files to Create

| File | Changes |
|------|---------|
| `migrations/YYYYMMDDHHMMSS_create_tenants_tables.sql` | Create tenants tables |
| `crates/qa-pms-api/src/routes/tenants.rs` | Create tenant API routes |
| `frontend/src/components/tenants/TenantSwitcher.tsx` | Create tenant switcher component |

---

## Files to Modify

| File | Changes |
|------|---------|
| All repository files | Add tenant_id filtering |
| `crates/qa-pms-api/src/app.rs` | Add tenant context middleware |
| `frontend/src/App.tsx` | Add tenant context provider |

---

## Dev Notes

### Database Schema

```sql
CREATE TABLE IF NOT EXISTS tenants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    config JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS tenant_users (
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'member',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (tenant_id, user_id)
);
```

### Tenant Isolation Pattern

**Repository Pattern:**
```rust
pub async fn get_tickets(pool: &PgPool, tenant_id: Uuid) -> Result<Vec<Ticket>> {
    sqlx::query_as!(
        r"SELECT * FROM tickets WHERE tenant_id = $1",
        tenant_id
    )
    .fetch_all(pool)
    .await
}
```

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 19, Story 19.7)
- Dependency: Epic 15 (Authentication) - must be complete
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### File List
