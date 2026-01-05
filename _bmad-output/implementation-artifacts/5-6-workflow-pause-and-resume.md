# Story 5.6: Workflow Pause and Resume

Status: done

## Story

As a QA (Ana),
I want to pause my workflow and resume later,
So that I can handle interruptions without losing progress.

## Acceptance Criteria

1. **Given** user is in an active workflow
   **When** user clicks "Pause"
   **Then** workflow status changes to "paused"

2. **Given** workflow is paused
   **When** user returns to workflow
   **Then** "Resume" button is displayed

3. **Given** workflow is paused
   **When** user clicks "Resume"
   **Then** workflow status changes back to "active"

4. **Given** workflow is paused
   **When** user views workflow
   **Then** current step is preserved

5. **Given** workflow is paused
   **When** status changes
   **Then** paused_at timestamp is recorded

## Tasks

- [ ] Task 1: Backend pause/resume endpoints
- [ ] Task 2: Frontend hooks and UI integration
- [ ] Task 3: Build, test, and finalize

## Dev Notes

### Technical Implementation

```rust
// POST /api/v1/workflows/:id/pause
// POST /api/v1/workflows/:id/resume
```

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### File List
