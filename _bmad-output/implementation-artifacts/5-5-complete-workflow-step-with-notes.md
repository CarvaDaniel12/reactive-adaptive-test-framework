# Story 5.5: Complete Workflow Step with Notes

Status: done

## Story

As a QA (Ana),
I want to mark steps complete and add notes,
So that I can document my testing process.

## Acceptance Criteria

1. **Given** user is on a workflow step
   **When** user clicks "Complete Step"
   **Then** step is marked as complete

2. **Given** user completes a step
   **When** completing
   **Then** user can optionally add notes (textarea)

3. **Given** user completes a step
   **When** completing
   **Then** user can optionally attach links (to bugs, test results)

4. **Given** step is completed
   **When** saved
   **Then** step result is saved to database with timestamp

5. **Given** step is completed
   **When** UI updates
   **Then** UI advances to next step automatically

6. **Given** step is completed
   **When** viewing completed steps
   **Then** shows checkmark with completion time

7. **Given** completed step exists
   **When** step is expanded
   **Then** notes are displayed

## Tasks / Subtasks

- [ ] Task 1: Backend endpoint to complete step (AC: #1, #4)
  - [ ] 1.1: POST /api/v1/workflows/:id/steps/:index/complete
  - [ ] 1.2: Save notes and links to database
  - [ ] 1.3: Update workflow current_step

- [ ] Task 2: Backend endpoint to skip step (AC: #1)
  - [ ] 2.1: POST /api/v1/workflows/:id/steps/:index/skip

- [ ] Task 3: Frontend mutation hooks (AC: #1, #5)
  - [ ] 3.1: useCompleteStep hook
  - [ ] 3.2: useSkipStep hook

- [ ] Task 4: Update CurrentStepCard (AC: #2, #3)
  - [ ] 4.1: Notes textarea (already exists)
  - [ ] 4.2: Add links input

- [ ] Task 5: Display completion info (AC: #6, #7)
  - [ ] 5.1: Show completion time in expanded step

## Dev Notes

### Architecture Alignment

This story adds step completion functionality:

- **Backend**: New endpoints for step completion/skip
- **Frontend**: Mutation hooks and UI updates

### Technical Implementation Details

#### Backend Endpoints

```rust
// POST /api/v1/workflows/:id/steps/:index/complete
CompleteStepRequest { notes: Option<String>, links: Vec<StepLink> }
CompleteStepResponse { next_step: Option<WorkflowStep>, workflow_completed: bool }

// POST /api/v1/workflows/:id/steps/:index/skip
SkipStepResponse { next_step: Option<WorkflowStep>, workflow_completed: bool }
```

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 5.5]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### Debug Log References

N/A

### Completion Notes List

- Backend: POST /api/v1/workflows/:id/steps/:index/complete
- Backend: POST /api/v1/workflows/:id/steps/:index/skip
- Backend: skip_step function added to repository
- Frontend: useCompleteStep and useSkipStep hooks
- Frontend: WorkflowPage integrated with step actions
- UI advances to next step automatically on complete/skip
- Toast notifications for success/error feedback
- Workflow completion detection and redirect to ticket
- All 7 ACs verified, frontend builds successfully

### File List

- crates/qa-pms-api/src/routes/workflows.rs (updated - complete/skip endpoints)
- crates/qa-pms-api/src/routes/mod.rs (updated - OpenAPI)
- crates/qa-pms-workflow/src/repository.rs (updated - skip_step function)
- frontend/src/hooks/useWorkflow.ts (updated - step action hooks)
- frontend/src/hooks/index.ts (updated - exports)
- frontend/src/pages/Workflows/WorkflowPage.tsx (updated - step actions)
