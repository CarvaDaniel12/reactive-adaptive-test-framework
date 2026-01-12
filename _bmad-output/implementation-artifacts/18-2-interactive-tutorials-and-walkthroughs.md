# Story 18.2: Interactive Tutorials and Walkthroughs

Status: ready-for-dev

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

**As a** QA Engineer  
**I want** interactive tutorials and walkthroughs for key features  
**So that** I can learn features quickly and efficiently

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 18.2 |
| Epic | Epic 18: UX Improvements |
| Sprint | Sprint 1: Onboarding |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Story 18.1 (Onboarding Wizard Enhancement) |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create interactive tutorial system
   - Reusable tutorial overlay component
   - Support step-by-step walkthroughs
   - Highlight UI elements (spotlight/pointer)
   - Display contextual tips and instructions
   - Track tutorial progress and completion

2. Implement tutorial content management
   - Define tutorial structure (steps, targets, content)
   - Support multiple tutorials per page
   - Store tutorial definitions in JSON/TypeScript
   - Support conditional steps (skip based on context)

3. Create tutorial for key features
   - Ticket Management workflow tutorial
   - Workflow Engine tutorial
   - Dashboard overview tutorial
   - Report generation tutorial

4. Add tutorial controls
   - Next/Previous navigation
   - Skip tutorial button
   - Progress indicator
   - Dismiss and "Don't show again" option

---

## Acceptance Criteria

- [ ] **Given** tutorial system exists  
  **When** user triggers tutorial  
  **Then** interactive walkthrough is displayed with step-by-step guidance

- [ ] **Given** tutorial exists  
  **When** navigating steps  
  **Then** UI elements are highlighted and contextual tips are shown

- [ ] **Given** tutorial exists  
  **When** completing tutorial  
  **Then** progress is saved and tutorial doesn't show again

- [ ] **Given** tutorial exists  
  **When** skipping tutorial  
  **Then** tutorial can be resumed later from settings

- [ ] **Given** tutorial system exists  
  **When** tutorial is active  
  **Then** user can navigate with next/previous buttons

---

## Tasks / Subtasks

- [ ] Task 1: Create tutorial overlay component (AC: #1)
  - [ ] 1.1: Create `frontend/src/components/tutorial/TutorialOverlay.tsx`
  - [ ] 1.2: Implement spotlight/highlight for target elements
  - [ ] 1.3: Display step content (title, description, tips)
  - [ ] 1.4: Add backdrop/overlay to dim non-target areas
  - [ ] 1.5: Position tooltip/instruction box near target element

- [ ] Task 2: Create tutorial step component (AC: #1, #5)
  - [ ] 2.1: Create `frontend/src/components/tutorial/TutorialStep.tsx`
  - [ ] 2.2: Display step number and total steps
  - [ ] 2.3: Add Next/Previous navigation buttons
  - [ ] 2.4: Add Skip tutorial button
  - [ ] 2.5: Add progress indicator (steps completed/total)

- [ ] Task 3: Create tutorial definition types (AC: #2)
  - [ ] 3.1: Create `frontend/src/types/tutorial.ts`
  - [ ] 3.2: Define `TutorialStep` interface (target, content, actions)
  - [ ] 3.3: Define `Tutorial` interface (id, title, steps, conditions)
  - [ ] 3.4: Define tutorial target types (selector, element, coordinates)

- [ ] Task 4: Create tutorial service/hook (AC: #3, #4)
  - [ ] 4.1: Create `frontend/src/hooks/useTutorial.ts`
  - [ ] 4.2: Implement tutorial state management
  - [ ] 4.3: Store tutorial completion in localStorage
  - [ ] 4.4: Support resume/skip functionality
  - [ ] 4.5: Check if tutorial should be shown (completion status)

- [ ] Task 5: Create tutorial definitions (AC: #3)
  - [ ] 5.1: Create `frontend/src/data/tutorials/ticketManagement.ts`
  - [ ] 5.2: Create `frontend/src/data/tutorials/workflowEngine.ts`
  - [ ] 5.3: Create `frontend/src/data/tutorials/dashboard.ts`
  - [ ] 5.4: Create `frontend/src/data/tutorials/reportGeneration.ts`

- [ ] Task 6: Integrate tutorials with pages (AC: #1)
  - [ ] 6.1: Add tutorial trigger to `TicketsPage.tsx`
  - [ ] 6.2: Add tutorial trigger to `WorkflowPage.tsx`
  - [ ] 6.3: Add tutorial trigger to `DashboardPage.tsx`
  - [ ] 6.4: Add tutorial trigger to report generation flow

- [ ] Task 7: Add tutorial settings (AC: #4)
  - [ ] 7.1: Add tutorial settings to user preferences
  - [ ] 7.2: Allow resetting tutorials (show again)
  - [ ] 7.3: Allow enabling/disabling specific tutorials

- [ ] Task 8: Add unit and integration tests (AC: #1, #2, #3, #4, #5)
  - [ ] 8.1: Test tutorial overlay component rendering
  - [ ] 8.2: Test tutorial navigation (next/previous)
  - [ ] 8.3: Test tutorial completion tracking
  - [ ] 8.4: Test tutorial skip/resume functionality

---

## Files to Create

| File | Changes |
|------|---------|
| `frontend/src/components/tutorial/TutorialOverlay.tsx` | Create tutorial overlay component |
| `frontend/src/components/tutorial/TutorialStep.tsx` | Create tutorial step component |
| `frontend/src/types/tutorial.ts` | Create tutorial type definitions |
| `frontend/src/hooks/useTutorial.ts` | Create tutorial hook |
| `frontend/src/data/tutorials/ticketManagement.ts` | Create ticket management tutorial |
| `frontend/src/data/tutorials/workflowEngine.ts` | Create workflow engine tutorial |
| `frontend/src/data/tutorials/dashboard.ts` | Create dashboard tutorial |
| `frontend/src/data/tutorials/reportGeneration.ts` | Create report generation tutorial |

---

## Files to Modify

| File | Changes |
|------|---------|
| `frontend/src/pages/Tickets/TicketsPage.tsx` | Add tutorial integration |
| `frontend/src/pages/Workflows/WorkflowPage.tsx` | Add tutorial integration |
| `frontend/src/pages/Dashboard/DashboardPage.tsx` | Add tutorial integration |

---

## Dev Notes

### Tutorial Component Structure

**TutorialOverlay:**
```typescript
interface TutorialOverlayProps {
  tutorial: Tutorial;
  onComplete: () => void;
  onSkip: () => void;
}

export function TutorialOverlay({ tutorial, onComplete, onSkip }: TutorialOverlayProps) {
  // Implement overlay with spotlight/highlight
  // Display step content
  // Handle navigation
}
```

**Tutorial Definition:**
```typescript
interface Tutorial {
  id: string;
  title: string;
  steps: TutorialStep[];
  conditions?: (context: TutorialContext) => boolean;
}

interface TutorialStep {
  target: string; // CSS selector or element ref
  title: string;
  description: string;
  tips?: string[];
  actions?: TutorialAction[];
}
```

### Project Structure Notes

**Component Patterns:**
- Follow existing component patterns (Radix UI Dialog/Popover for overlays)
- Use React Portal for overlay rendering (prevents z-index issues)
- Use CSS `position: fixed` for overlay positioning

**State Management:**
- Use localStorage for tutorial completion tracking
- Use Zustand store if multiple components need tutorial state
- Use React Query for tutorial definitions (if stored remotely, future)

**Tutorial Library Options:**
- Consider using `react-joyride` library for feature-rich tutorials
- Or build custom solution for more control and customization

### Testing Standards

**Unit Tests:**
- Test tutorial overlay rendering
- Test step navigation logic
- Test completion tracking

**Integration Tests:**
- Test tutorial flow end-to-end
- Test tutorial completion persistence
- Test tutorial skip/resume functionality

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 18, Story 18.2)
- Dependency: Story 18.1 (Onboarding Wizard Enhancement) - must be completed first
- Component Patterns: `qa-intelligent-pms/frontend/src/components/` (reference)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
