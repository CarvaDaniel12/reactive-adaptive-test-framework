# Story 19.1: Workflow Marketplace and Sharing

Status: ready-for-dev

## Story

**As a** QA Engineer  
**I want** a workflow marketplace and sharing system  
**So that** I can share and reuse workflows with my team

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 19.1 |
| Epic | Epic 19: Advanced Features |
| Sprint | Sprint 1: Workflow Marketplace |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Epic 5 (Workflow Engine) - Complete ✅ |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create workflow marketplace database schema
   - Add `shared` flag to `workflow_templates` table
   - Add `created_by` field to `workflow_templates` table
   - Add `workflow_ratings` table (ratings, reviews)
   - Add `workflow_downloads` table (track downloads)

2. Implement workflow sharing
   - Share workflow templates
   - Publish workflows to marketplace
   - Set visibility (private, team, public)
   - Duplicate/clone workflows

3. Create workflow marketplace UI
   - Browse shared workflows
   - Search/filter workflows
   - View workflow details and ratings
   - Import workflow to user's workspace

4. Implement workflow ratings and reviews
   - Rate workflows (1-5 stars)
   - Write reviews
   - View average ratings
   - Sort by rating/popularity

---

## Acceptance Criteria

- [ ] **Given** workflow marketplace exists  
  **When** sharing a workflow  
  **Then** workflow is published to marketplace

- [ ] **Given** workflow marketplace exists  
  **When** browsing workflows  
  **Then** shared workflows are displayed with details

- [ ] **Given** workflow marketplace exists  
  **When** searching workflows  
  **Then** matching workflows are displayed

- [ ] **Given** workflow marketplace exists  
  **When** importing workflow  
  **Then** workflow is added to user's workspace

- [ ] **Given** workflow marketplace exists  
  **When** rating workflow  
  **Then** rating is saved and average rating updates

---

## Tasks / Subtasks

- [ ] Task 1: Update workflow_templates table (AC: #1)
  - [ ] 1.1: Create migration: `YYYYMMDDHHMMSS_add_workflow_sharing_fields.sql`
  - [ ] 1.2: Add `shared BOOLEAN DEFAULT false` column
  - [ ] 1.3: Add `created_by UUID` column (nullable, references users)
  - [ ] 1.4: Add `visibility VARCHAR(20) DEFAULT 'private'` (private, team, public)
  - [ ] 1.5: Add `download_count INTEGER DEFAULT 0` column

- [ ] Task 2: Create workflow ratings table (AC: #5)
  - [ ] 2.1: Create migration: `YYYYMMDDHHMMSS_create_workflow_ratings_table.sql`
  - [ ] 2.2: Define `workflow_ratings` table (workflow_id, user_id, rating, review, created_at)
  - [ ] 2.3: Add unique constraint (workflow_id, user_id)

- [ ] Task 3: Update workflow repository (AC: #2)
  - [ ] 3.1: Update `crates/qa-pms-workflow/src/repository.rs`
  - [ ] 3.2: Add `share_workflow` method
  - [ ] 3.3: Add `get_shared_workflows` method
  - [ ] 3.4: Add `import_workflow` method
  - [ ] 3.5: Add `rate_workflow` method

- [ ] Task 4: Create workflow marketplace API (AC: #1, #2, #3, #4)
  - [ ] 4.1: Update `crates/qa-pms-api/src/routes/workflows.rs`
  - [ ] 4.2: POST /api/v1/workflows/templates/:id/share - share workflow
  - [ ] 4.3: GET /api/v1/workflows/marketplace - browse shared workflows
  - [ ] 4.4: GET /api/v1/workflows/marketplace/:id - get workflow details
  - [ ] 4.5: POST /api/v1/workflows/marketplace/:id/import - import workflow
  - [ ] 4.6: POST /api/v1/workflows/marketplace/:id/rate - rate workflow

- [ ] Task 5: Create workflow marketplace UI (AC: #3, #4)
  - [ ] 5.1: Create `frontend/src/pages/Workflows/WorkflowMarketplacePage.tsx`
  - [ ] 5.2: Create `frontend/src/components/workflow/WorkflowCard.tsx`
  - [ ] 5.3: Create `frontend/src/components/workflow/WorkflowRating.tsx`
  - [ ] 5.4: Add search/filter UI
  - [ ] 5.5: Add import workflow button

- [ ] Task 6: Integrate sharing with workflow editor (AC: #1)
  - [ ] 6.1: Add share button to workflow editor
  - [ ] 6.2: Add visibility selector (private, team, public)
  - [ ] 6.3: Add share confirmation dialog

---

## Files to Create

| File | Changes |
|------|---------|
| `migrations/YYYYMMDDHHMMSS_add_workflow_sharing_fields.sql` | Add sharing fields to workflow_templates |
| `migrations/YYYYMMDDHHMMSS_create_workflow_ratings_table.sql` | Create workflow ratings table |
| `frontend/src/pages/Workflows/WorkflowMarketplacePage.tsx` | Create workflow marketplace page |
| `frontend/src/components/workflow/WorkflowCard.tsx` | Create workflow card component |
| `frontend/src/components/workflow/WorkflowRating.tsx` | Create workflow rating component |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-workflow/src/repository.rs` | Add sharing and marketplace methods |
| `crates/qa-pms-api/src/routes/workflows.rs` | Add marketplace endpoints |
| `frontend/src/pages/Workflows/WorkflowPage.tsx` | Add share workflow functionality |

---

## Dev Notes

### Database Schema Updates

**workflow_templates table:**
```sql
ALTER TABLE workflow_templates
ADD COLUMN IF NOT EXISTS shared BOOLEAN NOT NULL DEFAULT false,
ADD COLUMN IF NOT EXISTS created_by UUID,
ADD COLUMN IF NOT EXISTS visibility VARCHAR(20) NOT NULL DEFAULT 'private',
ADD COLUMN IF NOT EXISTS download_count INTEGER NOT NULL DEFAULT 0;
```

**workflow_ratings table:**
```sql
CREATE TABLE IF NOT EXISTS workflow_ratings (
    workflow_id UUID REFERENCES workflow_templates(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    review TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (workflow_id, user_id)
);
```

### Workflow Marketplace Patterns

**API Endpoints:**
- `GET /api/v1/workflows/marketplace` - List shared workflows (with filters)
- `GET /api/v1/workflows/marketplace/:id` - Get workflow details with ratings
- `POST /api/v1/workflows/marketplace/:id/import` - Import workflow to user workspace
- `POST /api/v1/workflows/marketplace/:id/rate` - Rate workflow

**Workflow Sharing:**
- Visibility levels: `private` (only creator), `team` (team members), `public` (everyone)
- Shared workflows are read-only in marketplace
- Import creates a copy in user's workspace

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 19, Story 19.1)
- Dependency: Epic 5 (Workflow Engine) - must be complete
- Workflow Patterns: `qa-intelligent-pms/crates/qa-pms-workflow/` (reference)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### File List
