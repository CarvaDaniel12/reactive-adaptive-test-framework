# Story 5.8: Workflow State Persistence

Status: done

## Story

As a QA (Ana),
I want my workflow state to persist if I close the browser,
So that I never lose my progress.

## Acceptance Criteria

1. **Given** user has an active workflow
   **When** user closes browser
   **Then** all workflow state is persisted in database

2. **Given** user reopens app
   **When** there's an active workflow
   **Then** "You have an active workflow" prompt is shown

3. **Given** active workflow prompt is shown
   **When** user clicks Resume
   **Then** user is taken to workflow page

4. **Given** active workflow prompt is shown
   **When** user clicks Abandon
   **Then** workflow is marked as "cancelled"

## Tasks

- [ ] Task 1: Check for active workflows on app load
- [ ] Task 2: Create ActiveWorkflowPrompt component
- [ ] Task 3: Backend - cancel workflow endpoint
- [ ] Task 4: Build, test, and finalize

## Dev Notes

State persistence is already implemented via database storage. This story adds the UX prompt.

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### File List
