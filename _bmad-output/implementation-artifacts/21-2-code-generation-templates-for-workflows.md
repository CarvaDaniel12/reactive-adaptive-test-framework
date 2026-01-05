# Story 21.2: Code Generation Templates for Workflows

Status: ready-for-dev

Epic: 21 - Developer Experience
Priority: P1 (High Value)
Estimated Effort: 2 days
Sprint: 1

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

As a **QA Engineer**,
I want to **generate workflow templates based on ticket type or testing scenario**,
So that **I don't have to manually configure every workflow and can follow best practices**.

## Acceptance Criteria

1. **Given** I select a ticket type (bug, feature, regression, etc.)
   **When** I generate a workflow template
   **Then** it includes appropriate steps for that ticket type
   **And** pre-configures integration points (Jira, Postman, Testmo, Splunk)
   **And** adds inline documentation for each step
   **And** includes validation rules

2. **Given** I want to create a custom template
   **When** I save a workflow as template
   **Then** I can name and describe it
   **And** it appears in template list
   **And** I can share it with team
   **And** template is stored for future use

3. **Given** predefined templates exist
   **When** I access template selection
   **Then** I see Bug Verification template
   **And** I see Feature Acceptance template
   **And** I see Regression Testing template
   **And** I see Smoke Testing template
   **And** I see Integration Testing template

4. **Given** I want to customize a template
   **When** I generate from template
   **Then** I can modify workflow name
   **And** I can link to specific Jira ticket
   **And** I can select which integrations to include
   **And** I can adjust estimated time multiplier

5. **Given** template supports template inheritance
   **When** I create a template that extends another
   **Then** it inherits base steps from parent template
   **And** I can override or extend steps
   **And** template versioning is supported

6. **Given** template validation exists
   **When** I create or modify a template
   **Then** system validates template structure
   **And** validates step dependencies
   **And** validates integration configurations
   **And** shows validation errors with suggestions

7. **Given** template documentation is generated
   **When** I view a template
   **Then** I see inline documentation for each step
   **And** I see integration configuration examples
   **And** I see validation rules explanation
   **And** I see estimated time breakdown

## Tasks / Subtasks

- [ ] Task 1: Extend workflow template repository with template generator (AC: #1, #3)
  - [ ] 1.1: Create `crates/qa-pms-workflow/src/templates.rs` module
  - [ ] 1.2: Create `TemplateGenerator` struct with database pool
  - [ ] 1.3: Implement `generate_from_type(ticket_type, config)` method
  - [ ] 1.4: Load predefined templates from seeding (Story 5.2)
  - [ ] 1.5: Support template types: bug, feature, regression, smoke, integration
  - [ ] 1.6: Apply customizations (name, ticket, integrations, time multiplier)
  - [ ] 1.7: Pre-configure integration points based on template type
  - [ ] 1.8: Add inline documentation to generated steps
  - [ ] 1.9: Add validation rules to steps
  - [ ] 1.10: Return generated `WorkflowTemplate` instance
  - [ ] 1.11: Add unit tests for template generation

- [ ] Task 2: Create template customization configuration (AC: #4)
  - [ ] 2.1: Create `TemplateConfig` struct in `templates.rs`
  - [ ] 2.2: Add fields: name (Option<String>), ticket (Option<String>), integrations (Option<Vec<String>>), time_multiplier (Option<f64>)
  - [ ] 2.3: Implement `TemplateConfig::from_user_input()` method
  - [ ] 2.4: Validate template config (name format, ticket format, integration names)
  - [ ] 2.5: Apply time multiplier to all step estimated times
  - [ ] 2.6: Link Jira ticket to workflow metadata
  - [ ] 2.7: Filter integrations based on config
  - [ ] 2.8: Add unit tests for config application

- [ ] Task 3: Implement template save functionality (AC: #2)
  - [ ] 3.1: Create `save_workflow_as_template(workflow_id, name, description)` method
  - [ ] 3.2: Extract workflow steps from existing workflow instance
  - [ ] 3.3: Create new template record with `is_default: false`
  - [ ] 3.4: Set template ticket_type based on workflow metadata or user selection
  - [ ] 3.5: Store template with user_id for ownership tracking
  - [ ] 3.6: Add migration to add `created_by_user_id` to `workflow_templates` table
  - [ ] 3.7: Implement template sharing (make template visible to team)
  - [ ] 3.8: Add `GET /api/v1/workflows/templates/custom` endpoint for user templates
  - [ ] 3.9: Add unit tests for template save

- [ ] Task 4: Implement template inheritance system (AC: #5)
  - [ ] 4.1: Add `parent_template_id` field to `workflow_templates` table (migration)
  - [ ] 4.2: Add `template_version` field (semantic versioning: "1.0.0")
  - [ ] 4.3: Implement `extend_template(parent_id, extensions)` method
  - [ ] 4.4: Merge parent template steps with child template steps
  - [ ] 4.5: Support step override (child step replaces parent step with same ID)
  - [ ] 4.6: Support step extension (child adds steps after parent steps)
  - [ ] 4.7: Validate inheritance chain (no circular dependencies)
  - [ ] 4.8: Add version bump functionality
  - [ ] 4.9: Add unit tests for template inheritance

- [ ] Task 5: Create template validation system (AC: #6)
  - [ ] 5.1: Create `TemplateValidator` struct in `templates.rs`
  - [ ] 5.2: Implement `validate_template(template)` method
  - [ ] 5.3: Validate required fields (name, ticket_type, steps)
  - [ ] 5.4: Validate step structure (id, name, description, estimated_minutes)
  - [ ] 5.5: Validate step dependencies (all `depends_on` steps exist)
  - [ ] 5.6: Validate no circular dependencies in steps
  - [ ] 5.7: Validate integration configurations (if integration step, config required)
  - [ ] 5.8: Return validation errors with specific field paths
  - [ ] 5.9: Add suggestions for fixing validation errors
  - [ ] 5.10: Add unit tests for template validation

- [ ] Task 6: Create template generation API endpoints (AC: #1, #3, #4)
  - [ ] 6.1: Create `crates/qa-pms-api/src/routes/templates.rs` module
  - [ ] 6.2: Add `POST /api/v1/workflows/templates/generate` endpoint
  - [ ] 6.3: Request body: `{ ticket_type, config: TemplateConfig }`
  - [ ] 6.4: Generate template using `TemplateGenerator`
  - [ ] 6.5: Return generated template as JSON
  - [ ] 6.6: Add `POST /api/v1/workflows/templates/save` endpoint
  - [ ] 6.7: Request body: `{ workflow_id, name, description, ticket_type, share: bool }`
  - [ ] 6.8: Save workflow as template using `save_workflow_as_template()`
  - [ ] 6.9: Add `GET /api/v1/workflows/templates/custom` endpoint (user templates)
  - [ ] 6.10: Add `GET /api/v1/workflows/templates/:id/extend` endpoint (template inheritance)
  - [ ] 6.11: Add OpenAPI documentation with `utoipa`
  - [ ] 6.12: Add integration tests

- [ ] Task 7: Create template selection UI component (AC: #1, #3, #4)
  - [ ] 7.1: Create `frontend/src/components/workflow/TemplateSelector.tsx` component
  - [ ] 7.2: Display predefined templates (Bug, Feature, Regression, etc.)
  - [ ] 7.3: Display custom templates (user templates)
  - [ ] 7.4: Add template preview (steps, estimated time, integrations)
  - [ ] 7.5: Add template filter by ticket type
  - [ ] 7.6: Add template search functionality
  - [ ] 7.7: Create `TemplateConfigForm` component for customization
  - [ ] 7.8: Add fields: name, ticket key, integrations (multi-select), time multiplier (slider)
  - [ ] 7.9: Add "Generate Workflow" button
  - [ ] 7.10: Show loading state during generation
  - [ ] 7.11: Navigate to workflow editor with generated template

- [ ] Task 8: Create save workflow as template UI (AC: #2)
  - [ ] 8.1: Add "Save as Template" button to workflow editor
  - [ ] 8.2: Create `SaveTemplateDialog.tsx` component (Radix Dialog)
  - [ ] 8.3: Add form fields: template name (required), description, ticket type, share checkbox
  - [ ] 8.4: Validate form (name required, description optional)
  - [ ] 8.5: Call `POST /api/v1/workflows/templates/save` endpoint
  - [ ] 8.6: Show success toast with template ID
  - [ ] 8.7: Update template list to include new template
  - [ ] 8.8: Handle errors gracefully

- [ ] Task 9: Enhance template documentation generation (AC: #7)
  - [ ] 9.1: Create `generate_template_docs(template)` function
  - [ ] 9.2: Generate markdown documentation for template
  - [ ] 9.3: Document each step: name, description, estimated time, checklist items
  - [ ] 9.4: Document integration configurations with examples
  - [ ] 9.5: Document validation rules and their purpose
  - [ ] 9.6: Calculate and document total estimated time
  - [ ] 9.7: Add template documentation to API response (optional field)
  - [ ] 9.8: Display documentation in template preview UI

- [ ] Task 10: Add template versioning support (AC: #5)
  - [ ] 10.1: Add `template_version` field to `workflow_templates` table (migration)
  - [ ] 10.2: Default version: "1.0.0" (semantic versioning)
  - [ ] 10.3: Implement `bump_version(template_id, version_type)` method
  - [ ] 10.4: Support version types: major, minor, patch
  - [ ] 10.5: Create new template record when version is bumped (preserve history)
  - [ ] 10.6: Add `GET /api/v1/workflows/templates/:id/versions` endpoint
  - [ ] 10.7: Display version history in template detail view
  - [ ] 10.8: Allow reverting to previous version
  - [ ] 10.9: Add unit tests for versioning

- [ ] Task 11: Add comprehensive tests (AC: All)
  - [ ] 11.1: Add unit tests for `TemplateGenerator::generate_from_type()`
  - [ ] 11.2: Add unit tests for template config application
  - [ ] 11.3: Add unit tests for template save functionality
  - [ ] 11.4: Add unit tests for template inheritance
  - [ ] 11.5: Add unit tests for template validation
  - [ ] 11.6: Add integration tests for template generation API
  - [ ] 11.7: Add integration tests for template save API
  - [ ] 11.8: Test template preview and selection UI components
  - [ ] 11.9: Test save workflow as template flow

## Dev Notes

### Architecture Compliance

**Tech Stack:**
- Rust 1.80+ with Tokio async runtime
- Axum 0.7+ web framework
- PostgreSQL with SQLx 0.7
- React 19 + Vite 7 + Tailwind CSS v4, Zustand for state management, Radix UI for components
- YAML for template storage (already used for workflow templates)
- Error handling: `anyhow` (internal) + `thiserror` (API boundaries)
- Logging: `tracing` + `tracing-subscriber` (never `println!`)

**Code Structure:**
- **Template Generator:** `crates/qa-pms-workflow/src/templates.rs` (new module)
- **API Routes:** `crates/qa-pms-api/src/routes/templates.rs` (new module)
- **Frontend Components:** `frontend/src/components/workflow/TemplateSelector.tsx` (new), `SaveTemplateDialog.tsx` (new)
- **Database Migrations:** New migration for template fields (`parent_template_id`, `template_version`, `created_by_user_id`)

**Template Structure:**
Reuse existing `WorkflowTemplate` and `WorkflowStep` types from Story 5.1:
```rust
// From qa-pms-workflow/src/types.rs
pub struct WorkflowTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub ticket_type: String,  // Already exists
    pub steps_json: sqlx::types::Json<Vec<WorkflowStep>>,  // Already exists
    pub is_default: bool,  // Already exists
    // New fields to add:
    pub parent_template_id: Option<Uuid>,
    pub template_version: String,  // "1.0.0"
    pub created_by_user_id: Option<Uuid>,  // For custom templates
}
```

**Integration with Existing System:**
- Reuse `WorkflowTemplateRepository` from Story 5.1 (`get_default_templates()`, `create_template()`, etc.)
- Extend `seed_default_templates()` from Story 5.2 to include new predefined templates
- Use existing workflow instance creation flow (Story 5.3) when generating from template

**Template Customization Pattern:**
```rust
pub struct TemplateConfig {
    pub name: Option<String>,
    pub ticket: Option<String>,
    pub integrations: Option<Vec<String>>,  // ["jira", "postman"]
    pub estimated_time_multiplier: Option<f64>,  // 1.5 = 50% more time
}
```

### Previous Story Intelligence

**From Story 5.1 (Workflow Templates Database Schema):**
- Already have `workflow_templates` table with basic structure
- Already have `WorkflowTemplate` and `WorkflowStep` types
- Template seeding exists in `seeding.rs`
- This story extends templates with generation, inheritance, and customization

**From Story 5.2 (Default Workflow Templates):**
- Already have 3 default templates: Bug Fix, Feature Test, Regression
- Templates are seeded on app startup
- This story adds template generation API and UI
- This story adds ability to save custom templates

**From Story 5.3 (Start Workflow from Ticket):**
- Workflow instances are created from templates
- This story enhances template selection with generation and customization
- Workflow creation flow can use generated templates

**Key Integration Points:**
- Reuse `WorkflowTemplateRepository` methods
- Extend template seeding with new predefined templates (smoke, integration)
- Integrate template generation into workflow creation flow
- Use existing workflow editor UI for generated templates

**Code Patterns to Follow:**
```rust
// From seeding.rs - Template creation pattern
create_template(
    pool,
    "Template Name",
    Some("Description"),
    "ticket_type",
    &steps,
    is_default,
).await?;
```

### Project Structure Notes

**Alignment with unified structure:**
- ✅ Template logic in `qa-pms-workflow` (workflow domain)
- ✅ API routes in `qa-pms-api/src/routes/` (API layer)
- ✅ Frontend components in `frontend/src/components/workflow/` (UI layer)
- ✅ Database migrations in `migrations/` (database layer)

**Files to Create:**
- `crates/qa-pms-workflow/src/templates.rs` - Template generator, validator, inheritance
- `crates/qa-pms-api/src/routes/templates.rs` - Template generation API endpoints
- `frontend/src/components/workflow/TemplateSelector.tsx` - Template selection UI
- `frontend/src/components/workflow/SaveTemplateDialog.tsx` - Save template dialog
- `migrations/YYYYMMDDHHMMSS_template_extensions.sql` - Template fields migration

**Files to Modify:**
- `crates/qa-pms-workflow/src/lib.rs` - Export templates module
- `crates/qa-pms-workflow/src/types.rs` - Add new fields to `WorkflowTemplate` (parent_template_id, template_version, created_by_user_id)
- `crates/qa-pms-workflow/src/seeding.rs` - Add new predefined templates (smoke, integration)
- `crates/qa-pms-api/src/routes/mod.rs` - Add templates routes
- `crates/qa-pms-api/src/app.rs` - Add templates router
- `frontend/src/pages/Workflows/WorkflowPage.tsx` - Add template generation flow

**Naming Conventions:**
- Functions: `generate_from_type()`, `save_workflow_as_template()`, `extend_template()`
- API endpoints: `/api/v1/workflows/templates/generate`, `/api/v1/workflows/templates/save`
- Components: `TemplateSelector`, `SaveTemplateDialog`, `TemplateConfigForm`

### Testing Standards

**Unit Tests:**
- Test template generation for each ticket type (bug, feature, regression, smoke, integration)
- Test template config application (name, ticket, integrations, time multiplier)
- Test template inheritance (parent-child, circular dependency prevention)
- Test template validation (required fields, step dependencies, integration configs)

**Integration Tests:**
- Test template generation API endpoint
- Test template save API endpoint
- Test template inheritance API endpoint
- Test template list endpoints (default, custom)

**Component Tests:**
- Test TemplateSelector rendering with templates
- Test template preview functionality
- Test SaveTemplateDialog form submission
- Test template generation flow UI

**Test Coverage Target:**
- Minimum 80% coverage for template generation logic
- 100% coverage for template validation
- Integration tests for all API endpoints

### References

- **Source: `_bmad-output/planning-artifacts/epics-detailed/epic-21-developer-experience.md#story-21.2`** - Story requirements and acceptance criteria
- **Source: `qa-intelligent-pms/crates/qa-pms-workflow/src/types.rs`** - WorkflowTemplate and WorkflowStep types
- **Source: `qa-intelligent-pms/crates/qa-pms-workflow/src/repository.rs`** - Template repository methods
- **Source: `qa-intelligent-pms/crates/qa-pms-workflow/src/seeding.rs`** - Default template seeding
- **Source: `_bmad-output/implementation-artifacts/5-2-default-workflow-templates.md`** - Template structure reference
- **Source: `_bmad-output/planning-artifacts/project-context.md`** - Rust patterns, error handling, logging

## Dev Agent Record

### Agent Model Used

Claude Sonnet 4.5 (via Cursor)

### Debug Log References

(None yet - story not implemented)

### Completion Notes List

(None yet - story not implemented)

### File List

**Created:**
- `crates/qa-pms-workflow/src/templates.rs` - Template generator, validator, inheritance
- `crates/qa-pms-api/src/routes/templates.rs` - Template generation API endpoints
- `frontend/src/components/workflow/TemplateSelector.tsx` - Template selection component
- `frontend/src/components/workflow/SaveTemplateDialog.tsx` - Save template dialog
- `frontend/src/components/workflow/TemplateConfigForm.tsx` - Template customization form
- `migrations/YYYYMMDDHHMMSS_template_extensions.sql` - Template fields migration

**Modified:**
- `crates/qa-pms-workflow/src/lib.rs` - Export templates module
- `crates/qa-pms-workflow/src/types.rs` - Add fields to WorkflowTemplate struct
- `crates/qa-pms-workflow/src/repository.rs` - Add template save and inheritance methods
- `crates/qa-pms-workflow/src/seeding.rs` - Add new predefined templates
- `crates/qa-pms-api/src/routes/mod.rs` - Add templates routes module
- `crates/qa-pms-api/src/app.rs` - Add templates router
- `frontend/src/pages/Workflows/WorkflowPage.tsx` - Integrate template generation

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete structure
- Added all required sections: Story, Metadata, Acceptance Criteria (7 ACs), Tasks (11 tasks with subtasks), Dev Notes, Dev Agent Record, File List
- Converted acceptance criteria from epic format to Given/When/Then format
- Added comprehensive dev notes with architecture patterns, integration with existing stories (5.1, 5.2, 5.3), testing standards
- Added file list with all files to create and modify
