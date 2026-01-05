# Story 5.3: Start Workflow from Ticket

Status: done

## Story

As a QA (Ana),
I want to start a guided workflow from a ticket,
So that I can follow structured testing steps.

## Acceptance Criteria

1. **Given** user is viewing a Jira ticket
   **When** user clicks "Start Workflow"
   **Then** template selection dialog is shown (Bug Fix, Feature Test, Regression)

2. **Given** user selects a template
   **When** user confirms selection
   **Then** workflow instance is created and linked to ticket

3. **Given** workflow is created
   **When** UI updates
   **Then** workflow mode is activated (sidebar collapsed, step view shown)

4. **Given** workflow mode is active
   **When** UI renders
   **Then** first step is displayed prominently with description

5. **Given** workflow instance is created
   **When** database is queried
   **Then** instance is persisted with ticket_id, template_id, user_id

6. **Given** user views a ticket with existing workflow
   **When** "Start Workflow" is clicked
   **Then** prompt to resume or restart is shown

7. **Given** workflow mode is active
   **When** UI renders
   **Then** ticket context (key, title) is shown in header

## Tasks / Subtasks

- [ ] Task 1: Create backend endpoints (AC: #2, #5, #6)
  - [ ] 1.1: POST /api/v1/workflows - create workflow instance
  - [ ] 1.2: GET /api/v1/workflows/active/:ticketId - check existing workflow
  - [ ] 1.3: GET /api/v1/workflows/:id - get workflow details

- [ ] Task 2: Create frontend hooks (AC: #1, #2, #6)
  - [ ] 2.1: useWorkflowTemplates - fetch available templates
  - [ ] 2.2: useCreateWorkflow - mutation to create workflow
  - [ ] 2.3: useActiveWorkflow - check for existing workflow

- [ ] Task 3: Create TemplateSelectionDialog (AC: #1)
  - [ ] 3.1: List templates with descriptions
  - [ ] 3.2: Show estimated time per template
  - [ ] 3.3: Confirm/cancel buttons

- [ ] Task 4: Create WorkflowMode component (AC: #3, #4, #7)
  - [ ] 4.1: Workflow header with ticket context
  - [ ] 4.2: Current step display
  - [ ] 4.3: Collapse sidebar when active

- [ ] Task 5: Integrate into TicketDetailPage (AC: #1, #6)
  - [ ] 5.1: Add "Start Workflow" button
  - [ ] 5.2: Handle existing workflow prompt
  - [ ] 5.3: Transition to workflow mode

## Dev Notes

### Architecture Alignment

This story implements **Start Workflow from Ticket** per Epic 5 requirements:

- **Backend**: `crates/qa-pms-api/src/routes/workflows.rs`
- **Frontend**: `frontend/src/components/workflow/`
- **State**: Zustand store for workflow mode

### Technical Implementation Details

#### Backend Endpoints

```rust
// POST /api/v1/workflows
CreateWorkflowRequest { template_id: Uuid, ticket_id: String, ticket_title: String }
CreateWorkflowResponse { id: Uuid, template_name: String, current_step: WorkflowStep }

// GET /api/v1/workflows/active/:ticketId
ActiveWorkflowResponse { exists: bool, workflow?: WorkflowSummary }

// GET /api/v1/workflows/:id
WorkflowDetailResponse { id, template, ticket_id, ticket_title, status, current_step, steps }
```

#### Frontend Components

```
frontend/src/components/workflow/
├── TemplateSelectionDialog.tsx  # Template picker modal
├── WorkflowHeader.tsx           # Ticket context + controls
├── CurrentStepCard.tsx          # Active step display
├── WorkflowMode.tsx             # Main workflow view
└── index.ts
```

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 5.3]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### Debug Log References

N/A

### Completion Notes List

- Backend: POST /api/v1/workflows creates workflow instance
- Backend: GET /api/v1/workflows/:id returns workflow details with steps
- Backend: GET /api/v1/workflows/active/:ticketId checks for existing workflow
- Frontend: useWorkflow hooks (templates, active, detail, create)
- Frontend: TemplateSelectionDialog with template list and selection
- Frontend: WorkflowHeader with progress bar and controls
- Frontend: CurrentStepCard with step details and completion
- Frontend: StepsSidebar with step status indicators
- Frontend: WorkflowPage for workflow execution
- Integrated into TicketDetailPage with Start/Resume button
- Context7 used for React documentation
- All 7 ACs verified, frontend builds successfully

### File List

- crates/qa-pms-api/src/routes/workflows.rs (updated - added instance endpoints)
- crates/qa-pms-api/src/routes/mod.rs (updated - OpenAPI schemas)
- frontend/src/hooks/useWorkflow.ts (new)
- frontend/src/hooks/index.ts (updated)
- frontend/src/components/workflow/TemplateSelectionDialog.tsx (new)
- frontend/src/components/workflow/WorkflowHeader.tsx (new)
- frontend/src/components/workflow/CurrentStepCard.tsx (new)
- frontend/src/components/workflow/StepsSidebar.tsx (new)
- frontend/src/components/workflow/index.ts (new)
- frontend/src/pages/Workflows/WorkflowPage.tsx (new)
- frontend/src/pages/Workflows/index.ts (new)
- frontend/src/pages/Tickets/TicketDetailPage.tsx (updated)
- frontend/src/App.tsx (updated - workflow route)
