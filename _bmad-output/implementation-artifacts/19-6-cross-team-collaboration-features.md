# Story 19.6: Cross-Team Collaboration Features

Status: ready-for-dev

## Story

**As a** Team Lead  
**I want** cross-team collaboration features  
**So that** I can collaborate across teams effectively

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 19.6 |
| Epic | Epic 19: Advanced Features |
| Sprint | Sprint 2: Collaboration |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Epic 15 (Authentication) - Complete ✅ |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Implement team/organization structure
   - Add `teams` table (id, name, description, created_at)
   - Add `team_members` table (team_id, user_id, role, joined_at)
   - Support multiple teams per user
   - Support team hierarchies (future)

2. Create collaboration features
   - Share tickets with teams
   - Share workflows with teams
   - Team activity feed
   - Team notifications

3. Implement team permissions
   - Team-based access control
   - Team visibility (private, team, public)
   - Team roles (owner, admin, member)

4. Create team dashboard
   - Team activity view
   - Team metrics
   - Team member list
   - Team settings

---

## Acceptance Criteria

- [ ] **Given** team structure exists  
  **When** creating team  
  **Then** team is created with members

- [ ] **Given** collaboration features exist  
  **When** sharing ticket with team  
  **Then** team members can view ticket

- [ ] **Given** collaboration features exist  
  **When** sharing workflow with team  
  **Then** team members can use workflow

- [ ] **Given** team dashboard exists  
  **When** viewing team dashboard  
  **Then** team activity and metrics are displayed

---

## Tasks / Subtasks

- [ ] Task 1: Create teams database schema
  - [ ] 1.1: Create migration: `YYYYMMDDHHMMSS_create_teams_tables.sql`
  - [ ] 1.2: Define `teams` table
  - [ ] 1.3: Define `team_members` table
  - [ ] 1.4: Add indexes

- [ ] Task 2: Create team API endpoints
  - [ ] 2.1: Create `crates/qa-pms-api/src/routes/teams.rs`
  - [ ] 2.2: POST /api/v1/teams - create team
  - [ ] 2.3: GET /api/v1/teams - list teams
  - [ ] 2.4: POST /api/v1/teams/:id/members - add member
  - [ ] 2.5: DELETE /api/v1/teams/:id/members/:user_id - remove member

- [ ] Task 3: Implement team sharing
  - [ ] 3.1: Add team_id to tickets (optional)
  - [ ] 3.2: Add team_id to workflows (optional)
  - [ ] 3.3: Filter by team visibility

- [ ] Task 4: Create team dashboard UI
  - [ ] 4.1: Create `frontend/src/pages/Teams/TeamDashboardPage.tsx`
  - [ ] 4.2: Display team activity
  - [ ] 4.3: Display team metrics
  - [ ] 4.4: Display team members

---

## Files to Create

| File | Changes |
|------|---------|
| `migrations/YYYYMMDDHHMMSS_create_teams_tables.sql` | Create teams and team_members tables |
| `crates/qa-pms-api/src/routes/teams.rs` | Create team API routes |
| `frontend/src/pages/Teams/TeamDashboardPage.tsx` | Create team dashboard page |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-api/src/routes/tickets.rs` | Add team sharing support |
| `crates/qa-pms-api/src/routes/workflows.rs` | Add team sharing support |

---

## Dev Notes

### Database Schema

```sql
CREATE TABLE IF NOT EXISTS teams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_by UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS team_members (
    team_id UUID REFERENCES teams(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'member', -- 'owner', 'admin', 'member'
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (team_id, user_id)
);
```

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 19, Story 19.6)
- Dependency: Epic 15 (Authentication) - must be complete
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### File List
