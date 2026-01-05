# Story 5.7: Workflow Completion and Summary

Status: done

## Story

As a QA (Ana),
I want a summary when I complete all workflow steps,
So that I can review my testing session.

## Acceptance Criteria

1. **Given** user completes the final workflow step
   **When** workflow is marked complete
   **Then** workflow status is updated to "completed" in database

2. **Given** workflow is completed
   **When** summary is displayed
   **Then** shows time per step breakdown

3. **Given** workflow is completed
   **When** summary is displayed
   **Then** shows all notes compiled

4. **Given** workflow is completed
   **When** summary is displayed
   **Then** shows links collected

5. **Given** workflow is completed
   **When** summary is displayed
   **Then** "Generate Report" button is shown

## Tasks

- [ ] Task 1: Backend - complete workflow endpoint
- [ ] Task 2: Backend - get workflow summary endpoint
- [ ] Task 3: Frontend - WorkflowSummary component
- [ ] Task 4: Build, test, and finalize

## Dev Notes

### Technical Implementation

```rust
// POST /api/v1/workflows/:id/complete
// GET /api/v1/workflows/:id/summary
```

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### File List
