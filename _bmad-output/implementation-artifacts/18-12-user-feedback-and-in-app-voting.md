# Story 18.12: User Feedback and In-App Voting

Status: ready-for-dev

## Story

**As a** User  
**I want** in-app feedback system  
**So that** I can provide feedback easily and vote on feature requests

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 18.12 |
| Epic | Epic 18: UX Improvements |
| Sprint | Sprint 5: Feedback |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | None |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create feedback system
   - Feedback form component
   - Feedback submission API endpoint
   - Feedback storage in database
   - Feedback management UI (admin)

2. Implement feature voting
   - Vote on feature requests
   - View popular feature requests
   - Submit new feature requests
   - Vote tracking per user

3. Create feedback UI
   - Feedback button/widget
   - Feedback modal/form
   - Feedback type selection (bug, feature, suggestion)
   - Screenshot attachment (optional)

4. Store and manage feedback
   - Save feedback to database
   - Track feedback status (new, in-review, implemented, declined)
   - Feedback search and filtering
   - Feedback analytics

---

## Acceptance Criteria

- [ ] **Given** feedback system exists  
  **When** submitting feedback  
  **Then** feedback is saved and confirmation is shown

- [ ] **Given** feedback system exists  
  **When** voting on feature  
  **Then** vote is recorded and vote count updates

- [ ] **Given** feedback system exists  
  **When** viewing feedback  
  **Then** feedback list is displayed with status and votes

- [ ] **Given** feedback system exists  
  **When** searching feedback  
  **Then** filtered feedback results are displayed

---

## Tasks / Subtasks

- [ ] Task 1: Create feedback database schema
  - [ ] 1.1: Create migration: `YYYYMMDDHHMMSS_create_feedback_table.sql`
  - [ ] 1.2: Define feedback table (id, type, title, description, user_id, status, votes, created_at)
  - [ ] 1.3: Create feedback_votes table (feedback_id, user_id, created_at)

- [ ] Task 2: Create feedback API endpoints
  - [ ] 2.1: Create `crates/qa-pms-api/src/routes/feedback.rs`
  - [ ] 2.2: POST /api/v1/feedback - submit feedback
  - [ ] 2.3: GET /api/v1/feedback - list feedback
  - [ ] 2.4: POST /api/v1/feedback/:id/vote - vote on feedback
  - [ ] 2.5: GET /api/v1/feedback/popular - get popular feedback

- [ ] Task 3: Create feedback UI components
  - [ ] 3.1: Create `frontend/src/components/feedback/FeedbackButton.tsx`
  - [ ] 3.2: Create `frontend/src/components/feedback/FeedbackModal.tsx`
  - [ ] 3.3: Create `frontend/src/pages/Feedback/FeedbackPage.tsx`
  - [ ] 3.4: Create `frontend/src/components/feedback/FeedbackList.tsx`

- [ ] Task 4: Implement voting system
  - [ ] 4.1: Create vote API endpoint
  - [ ] 4.2: Create vote button component
  - [ ] 4.3: Track user votes (prevent duplicate votes)
  - [ ] 4.4: Display vote counts

- [ ] Task 5: Add unit and integration tests

---

## Files to Create

| File | Changes |
|------|---------|
| `migrations/YYYYMMDDHHMMSS_create_feedback_table.sql` | Create feedback and votes tables |
| `crates/qa-pms-api/src/routes/feedback.rs` | Create feedback API routes |
| `frontend/src/components/feedback/FeedbackButton.tsx` | Create feedback button |
| `frontend/src/components/feedback/FeedbackModal.tsx` | Create feedback modal |
| `frontend/src/pages/Feedback/FeedbackPage.tsx` | Create feedback page |

---

## Files to Modify

| File | Changes |
|------|---------|
| `frontend/src/components/layout/Header.tsx` | Add feedback button |
| `frontend/src/App.tsx` | Add feedback route |

---

## Dev Notes

### Database Schema

```sql
CREATE TABLE IF NOT EXISTS feedback (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type VARCHAR(50) NOT NULL, -- 'bug', 'feature', 'suggestion'
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    user_id UUID, -- References users if exists
    status VARCHAR(20) NOT NULL DEFAULT 'new', -- 'new', 'in-review', 'implemented', 'declined'
    votes INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS feedback_votes (
    feedback_id UUID REFERENCES feedback(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (feedback_id, user_id)
);
```

### Feedback Types

- `bug`: Bug reports
- `feature`: Feature requests
- `suggestion`: General suggestions

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 18, Story 18.12)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### File List
