# Story 5.2: Default Workflow Templates

Status: done

## Story

As a QA (Ana),
I want pre-built workflow templates for common ticket types,
So that I can start testing with guidance immediately.

## Acceptance Criteria

1. **Given** workflow tables exist
   **When** the application initializes (seeding)
   **Then** Bug Fix Template is created with steps: Reproduce → Investigate → Test Fix → Regression Check → Document

2. **Given** workflow tables exist
   **When** the application initializes (seeding)
   **Then** Feature Test Template is created with steps: Review Requirements → Exploratory Test → Happy Path → Edge Cases → Document

3. **Given** workflow tables exist
   **When** the application initializes (seeding)
   **Then** Regression Template is created with steps: Setup Environment → Run Test Suite → Analyze Failures → Report

4. **Given** default templates
   **When** templates are queried
   **Then** each step includes name, description, and estimated_minutes

5. **Given** default templates
   **When** templates are queried
   **Then** templates are marked as is_default = true

6. **Given** seeding runs multiple times
   **When** templates already exist
   **Then** no duplicates are created (idempotent seeding)

## Tasks / Subtasks

- [ ] Task 1: Define default templates as constants (AC: #1, #2, #3, #4)
  - [ ] 1.1: Bug Fix Template with 5 steps
  - [ ] 1.2: Feature Test Template with 5 steps
  - [ ] 1.3: Regression Template with 4 steps

- [ ] Task 2: Create seeding function (AC: #5, #6)
  - [ ] 2.1: Check if templates already exist
  - [ ] 2.2: Insert templates if missing
  - [ ] 2.3: Make seeding idempotent

- [ ] Task 3: Integrate seeding into app startup (AC: #1, #2, #3)
  - [ ] 3.1: Call seeding after migrations
  - [ ] 3.2: Log seeding results

- [ ] Task 4: Add API endpoint for templates (AC: #4)
  - [ ] 4.1: GET /api/v1/workflows/templates
  - [ ] 4.2: GET /api/v1/workflows/templates/:id

## Dev Notes

### Architecture Alignment

This story implements **Default Workflow Templates** per Epic 5 requirements:

- **Location**: `crates/qa-pms-workflow/src/seeding.rs`
- **Integration**: Called from `qa-pms-api` on startup
- **Pattern**: Idempotent seeding with existence checks

### Technical Implementation Details

#### Default Templates

```rust
// Bug Fix Template
WorkflowStep { name: "Reproduce Bug", description: "...", estimated_minutes: 15 }
WorkflowStep { name: "Investigate Root Cause", description: "...", estimated_minutes: 20 }
WorkflowStep { name: "Test Fix", description: "...", estimated_minutes: 30 }
WorkflowStep { name: "Regression Check", description: "...", estimated_minutes: 20 }
WorkflowStep { name: "Document Findings", description: "...", estimated_minutes: 10 }

// Feature Test Template
WorkflowStep { name: "Review Requirements", description: "...", estimated_minutes: 15 }
WorkflowStep { name: "Exploratory Testing", description: "...", estimated_minutes: 45 }
WorkflowStep { name: "Happy Path Testing", description: "...", estimated_minutes: 30 }
WorkflowStep { name: "Edge Case Testing", description: "...", estimated_minutes: 30 }
WorkflowStep { name: "Document Test Cases", description: "...", estimated_minutes: 15 }

// Regression Template
WorkflowStep { name: "Setup Test Environment", description: "...", estimated_minutes: 20 }
WorkflowStep { name: "Run Test Suite", description: "...", estimated_minutes: 60 }
WorkflowStep { name: "Analyze Failures", description: "...", estimated_minutes: 30 }
WorkflowStep { name: "Generate Report", description: "...", estimated_minutes: 15 }
```

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 5.2]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### Debug Log References

N/A

### Completion Notes List

- Bug Fix Template: 5 steps (95 min total) - Reproduce, Investigate, Test Fix, Regression, Document
- Feature Test Template: 5 steps (135 min total) - Review, Exploratory, Happy Path, Edge Cases, Document
- Regression Template: 4 steps (125 min total) - Setup, Run Suite, Analyze, Report
- Idempotent seeding via seed_default_templates() - checks existence before insert
- Seeding called on app startup after migrations
- API endpoints: GET /api/v1/workflows/templates, GET /api/v1/workflows/templates/:id
- Context7 used for SQLx documentation
- 9 tests passing

### File List

- crates/qa-pms-workflow/src/seeding.rs (new)
- crates/qa-pms-workflow/src/lib.rs (updated)
- crates/qa-pms-api/src/routes/workflows.rs (new)
- crates/qa-pms-api/src/routes/mod.rs (updated)
- crates/qa-pms-api/src/app.rs (updated - seeding on startup)
- crates/qa-pms-api/Cargo.toml (updated - added qa-pms-workflow)
