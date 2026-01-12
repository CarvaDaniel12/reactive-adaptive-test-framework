# Story 18.4: Drag-and-Drop Workflow Customization

Status: ready-for-dev

## Story

**As a** QA Engineer  
**I want** drag-and-drop workflow customization  
**So that** I can visually customize workflows without editing code

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 18.4 |
| Epic | Epic 18: UX Improvements |
| Sprint | Sprint 2: Productivity |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Epic 5 (Workflow Engine) - Complete ✅ |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create drag-and-drop workflow builder
   - Visual workflow step editor
   - Drag steps to reorder
   - Add/remove steps
   - Edit step properties

2. Implement drag-and-drop functionality
   - Use HTML5 drag-and-drop API or library (react-dnd, dnd-kit)
   - Visual feedback during drag
   - Drop zones for step positioning

3. Create workflow step editor
   - Edit step name, description
   - Configure step properties
   - Preview workflow structure

4. Save custom workflows
   - Save workflow templates to database
   - Support user-created workflows
   - Share workflows (future: Story 19.1)

---

## Acceptance Criteria

- [ ] **Given** drag-and-drop editor exists  
  **When** dragging a workflow step  
  **Then** step is reordered visually

- [ ] **Given** drag-and-drop editor exists  
  **When** dropping step in valid position  
  **Then** workflow structure is updated

- [ ] **Given** workflow editor exists  
  **When** adding new step  
  **Then** step is added to workflow

- [ ] **Given** workflow editor exists  
  **When** saving custom workflow  
  **Then** workflow is saved and can be reused

---

## Tasks / Subtasks

- [ ] Task 1: Create drag-and-drop workflow builder component
  - [ ] 1.1: Create `frontend/src/components/workflow/WorkflowBuilder.tsx`
  - [ ] 1.2: Integrate drag-and-drop library (react-dnd or dnd-kit)
  - [ ] 1.3: Display workflow steps as draggable items
  - [ ] 1.4: Show drop zones between steps

- [ ] Task 2: Implement step reordering
  - [ ] 2.1: Handle drag start event
  - [ ] 2.2: Handle drag over event (visual feedback)
  - [ ] 2.3: Handle drop event (update workflow structure)
  - [ ] 2.4: Update step order in state

- [ ] Task 3: Create step editor
  - [ ] 3.1: Create `frontend/src/components/workflow/WorkflowStepEditor.tsx`
  - [ ] 3.2: Edit step properties (name, description, instructions)
  - [ ] 3.3: Add step button
  - [ ] 3.4: Remove step button

- [ ] Task 4: Save custom workflows
  - [ ] 4.1: Create API endpoint for saving custom workflows
  - [ ] 4.2: Update workflow repository to support user-created templates
  - [ ] 4.3: Integrate save functionality with builder

---

## Files to Create

| File | Changes |
|------|---------|
| `frontend/src/components/workflow/WorkflowBuilder.tsx` | Create drag-and-drop workflow builder |
| `frontend/src/components/workflow/WorkflowStepEditor.tsx` | Create workflow step editor |

---

## Files to Modify

| File | Changes |
|------|---------|
| `frontend/src/pages/Workflows/WorkflowPage.tsx` | Integrate workflow builder |
| `crates/qa-pms-api/src/routes/workflows.rs` | Add endpoint for saving custom workflows |
| `crates/qa-pms-workflow/src/repository.rs` | Support user-created workflow templates |

---

## Dev Notes

### Drag-and-Drop Library

**Options:**
- `@dnd-kit/core` - Modern, accessible drag-and-drop (recommended)
- `react-dnd` - Mature library with HTML5 backend
- Native HTML5 drag-and-drop API (custom implementation)

**Recommended: dnd-kit**
- Better accessibility
- Modern React patterns
- Smaller bundle size

### Workflow Builder Structure

```typescript
interface WorkflowBuilderProps {
  workflow: WorkflowTemplate;
  onSave: (workflow: WorkflowTemplate) => void;
}

export function WorkflowBuilder({ workflow, onSave }: WorkflowBuilderProps) {
  // Implement drag-and-drop
  // Edit workflow steps
  // Save workflow
}
```

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 18, Story 18.4)
- Dependency: Epic 5 (Workflow Engine) - must be complete
- Workflow Patterns: `qa-intelligent-pms/crates/qa-pms-workflow/` (reference)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### File List
