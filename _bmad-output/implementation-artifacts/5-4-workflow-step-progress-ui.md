# Story 5.4: Workflow Step Progress UI

Status: done

## Story

As a QA (Ana),
I want to see my progress through workflow steps,
So that I know what's done and what's next.

## Acceptance Criteria

1. **Given** user is in an active workflow
   **When** the workflow view renders
   **Then** all steps are displayed in order (vertical stepper)

2. **Given** workflow view is rendered
   **When** user views steps
   **Then** current step is highlighted/expanded

3. **Given** workflow view is rendered
   **When** user views steps
   **Then** completed steps show checkmark ✅

4. **Given** workflow view is rendered
   **When** user views steps
   **Then** pending steps are grayed out

5. **Given** workflow view is rendered
   **When** user views steps
   **Then** each step shows name, description, and estimated time

6. **Given** a completed step exists
   **When** user clicks the completed step
   **Then** it expands to show notes

7. **Given** workflow view is rendered
   **When** user views header
   **Then** progress bar shows overall completion %

8. **Given** workflow view is rendered
   **When** user views header
   **Then** current step index is shown (e.g., "Step 3 of 7")

## Tasks / Subtasks

- [x] Task 1: Vertical stepper component (AC: #1, #2, #3, #4) - Done in 5.3
- [x] Task 2: Step details display (AC: #5) - Done in 5.3
- [ ] Task 3: Expandable completed steps with notes (AC: #6)
- [x] Task 4: Progress bar in header (AC: #7) - Done in 5.3
- [x] Task 5: Step index display (AC: #8) - Done in 5.3

## Dev Notes

### Architecture Alignment

Most of this story was implemented in Story 5.3. This story adds:
- Expandable completed steps with notes display

### Technical Implementation Details

Update StepsSidebar to be expandable and show notes when clicked.

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 5.4]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### Debug Log References

N/A

### Completion Notes List

- Most features were already implemented in Story 5.3
- Added expandable completed steps with notes display
- Added progress indicator in sidebar header (completed/total)
- Added chat bubble icon for steps with notes
- Added smooth expand/collapse animation
- All 8 ACs verified, frontend builds successfully

### File List

- frontend/src/components/workflow/StepsSidebar.tsx (updated - expandable steps)
