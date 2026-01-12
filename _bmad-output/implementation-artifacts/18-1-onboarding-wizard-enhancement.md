# Story 18.1: Onboarding Wizard Enhancement

Status: ready-for-dev

## Story

**As a** New User  
**I want** an enhanced onboarding wizard with interactive tutorials  
**So that** I can quickly learn how to use the framework

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 18.1 |
| Epic | Epic 18: UX Improvements |
| Sprint | Sprint 1: Onboarding |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Epic 2 (Setup Wizard) - Complete ✅, Epic 7 (Reports) - Complete ✅, Epic 15 (Authentication) - Complete ✅ |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Enhance existing setup wizard with tutorial overlay
   - Add tooltips and guided tour for first-time users
   - Show contextual help on each step
   - Track onboarding completion status

2. Create onboarding state management
   - Store onboarding progress in localStorage
   - Support skip/resume functionality
   - Mark onboarding as complete

3. Add interactive tutorials
   - Step-by-step walkthrough of key features
   - Highlight important UI elements
   - Provide next/previous navigation

---

## Acceptance Criteria

- [ ] **Given** user is new  
  **When** accessing setup wizard  
  **Then** interactive tutorial overlay is shown

- [ ] **Given** onboarding exists  
  **When** completing tutorial  
  **Then** onboarding status is saved and not shown again

- [ ] **Given** onboarding exists  
  **When** skipping tutorial  
  **Then** user can resume later from settings

---

## Tasks / Subtasks

- [ ] Task 1: Create onboarding tutorial component
- [ ] Task 2: Add tutorial overlay to setup wizard
- [ ] Task 3: Implement onboarding state management
- [ ] Task 4: Add skip/resume functionality
- [ ] Task 5: Create tutorial content for each wizard step

---

## Files to Create

| File | Changes |
|------|---------|
| `frontend/src/components/onboarding/TutorialOverlay.tsx` | Create tutorial overlay component |
| `frontend/src/components/onboarding/TutorialStep.tsx` | Create tutorial step component |
| `frontend/src/hooks/useOnboarding.ts` | Create onboarding state hook |

---

## Files to Modify

| File | Changes |
|------|---------|
| `frontend/src/pages/Setup/SetupWizard.tsx` | Add tutorial overlay integration |

---

## Dev Notes

### Tutorial Component

```typescript
export function TutorialOverlay({ steps, onComplete, onSkip }: TutorialProps) {
  // Implementation with step-by-step guided tour
}
```

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 18, Story 18.1)
- Dependency: Epic 2 (Setup Wizard), Epic 7 (Reports), Epic 15 (Authentication) - must be complete
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### File List
