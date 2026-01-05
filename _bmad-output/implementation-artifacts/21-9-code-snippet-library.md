# Story 21.9: Code Snippet Library

Status: ready-for-dev

Epic: 21 - Developer Experience
Priority: P1 (High Value)
Estimated Effort: 2 days
Sprint: 1

## Story

As a **QA Engineer**,
I want to **have a library of reusable code snippets**,
So that **I can quickly implement common patterns without writing everything from scratch**.

## Acceptance Criteria

1. **Given** I need to implement a workflow step
   **When** I search the snippet library
   **Then** I find relevant snippets by category or tags
   **And** I can preview snippet code
   **And** I can insert snippet into my workflow
   **And** I can customize parameters (placeholders)

2. **Given** I create a useful snippet
   **When** I save it to the library
   **Then** it's available to my team
   **And** I can add description and tags
   **And** I can categorize it
   **And** I can mark it as favorite

3. **Given** predefined snippets exist
   **When** I access snippet library
   **Then** I see Jira Integration snippets
   **And** I see Postman Collection snippets
   **And** I see Testmo Run snippets
   **And** I see Splunk Query snippets
   **And** I see Workflow Step snippets

## Tasks / Subtasks

- [ ] Task 1: Create snippet data models and storage (AC: #1, #2, #3)
  - [ ] 1.1: Create `crates/qa-pms-core/src/snippets/mod.rs` module
  - [ ] 1.2: Define `CodeSnippet` struct (id, name, description, category, language, code, parameters, tags, created_by, created_at)
  - [ ] 1.3: Define `SnippetCategory` enum (JiraIntegration, PostmanCollection, TestmoRun, SplunkQuery, WorkflowSteps, Validation, ErrorHandling)
  - [ ] 1.4: Create database migration for `snippets` table
  - [ ] 1.5: Implement `SnippetRepository` with CRUD operations
  - [ ] 1.6: Seed predefined snippets on application startup

- [ ] Task 2: Implement snippet parameter substitution (AC: #1)
  - [ ] 2.1: Create `SnippetParameter` struct (name, type, required, default, description)
  - [ ] 2.2: Implement parameter extraction from snippet code (regex: `{{PARAM_NAME}}`)
  - [ ] 2.3: Implement parameter substitution method
  - [ ] 2.4: Validate required parameters before substitution
  - [ ] 2.5: Apply default values for optional parameters

- [ ] Task 3: Create snippet API endpoints (AC: #1, #2)
  - [ ] 3.1: Create `crates/qa-pms-api/src/routes/snippets.rs` module
  - [ ] 3.2: Add `GET /api/v1/snippets` endpoint (list with filters: category, tags, search)
  - [ ] 3.3: Add `GET /api/v1/snippets/:id` endpoint (get snippet details)
  - [ ] 3.4: Add `POST /api/v1/snippets` endpoint (create snippet)
  - [ ] 3.5: Add `PUT /api/v1/snippets/:id` endpoint (update snippet)
  - [ ] 3.6: Add `DELETE /api/v1/snippets/:id` endpoint (delete snippet)
  - [ ] 3.7: Add `POST /api/v1/snippets/:id/substitute` endpoint (substitute parameters)
  - [ ] 3.8: Add OpenAPI documentation

- [ ] Task 4: Create snippet library UI (AC: #1, #2, #3)
  - [ ] 4.1: Create `frontend/src/pages/Snippets/SnippetLibraryPage.tsx`
  - [ ] 4.2: Create `frontend/src/components/snippets/SnippetList.tsx` component
  - [ ] 4.3: Create `frontend/src/components/snippets/SnippetCard.tsx` component
  - [ ] 4.4: Add search functionality (by name, description, tags)
  - [ ] 4.5: Add filter by category
  - [ ] 4.6: Add snippet preview modal
  - [ ] 4.7: Add parameter form for snippet insertion
  - [ ] 4.8: Add "Insert Snippet" button (copies to clipboard or inserts in editor)

- [ ] Task 5: Implement snippet management UI (AC: #2)
  - [ ] 5.1: Create `frontend/src/components/snippets/SnippetForm.tsx` component
  - [ ] 5.2: Add form fields: name, description, category, language, code, tags
  - [ ] 5.3: Add parameter editor (add/remove/edit parameters)
  - [ ] 5.4: Add snippet validation (check for placeholders, syntax)
  - [ ] 5.5: Add save/update functionality
  - [ ] 5.6: Add delete confirmation dialog

- [ ] Task 6: Seed predefined snippets (AC: #3)
  - [ ] 6.1: Create `crates/qa-pms-core/src/snippets/seed.rs` module
  - [ ] 6.2: Add Jira Integration snippets (status sync, comment creation)
  - [ ] 6.3: Add Postman Collection snippets (collection run, health check)
  - [ ] 6.4: Add Testmo Run snippets (test run creation, result reporting)
  - [ ] 6.5: Add Splunk Query snippets (error search, performance query)
  - [ ] 6.6: Add Workflow Step snippets (manual step, integration step)
  - [ ] 6.7: Run seeding on application startup (idempotent)

- [ ] Task 7: Add comprehensive tests (AC: All)
  - [ ] 7.1: Test snippet CRUD operations
  - [ ] 7.2: Test parameter extraction and substitution
  - [ ] 7.3: Test snippet API endpoints
  - [ ] 7.4: Test snippet search and filtering

## Dev Notes

### Architecture Compliance
- **Tech Stack:** Rust 1.80+ with Tokio, Axum 0.7+, PostgreSQL with SQLx 0.7
- **Frontend:** React 19 + Vite 7 + Tailwind CSS v4
- **Pattern:** Snippet library as reusable component system

### Previous Story Intelligence
- From Story 21.2 (Code Generation Templates): Similar template system
- Snippets are smaller, reusable pieces; templates are complete workflows

### References
- **Source:** `_bmad-output/planning-artifacts/epics-detailed/epic-21-developer-experience.md#story-21.9`

## Dev Agent Record

### Agent Model Used
Claude Sonnet 4.5 (via Cursor)

### File List

**Created:**
- `crates/qa-pms-core/src/snippets/mod.rs` - Snippet core module
- `crates/qa-pms-core/src/snippets/seed.rs` - Predefined snippets
- `crates/qa-pms-api/src/routes/snippets.rs` - Snippet API endpoints
- `frontend/src/pages/Snippets/SnippetLibraryPage.tsx` - Snippet library page
- `frontend/src/components/snippets/SnippetList.tsx` - Snippet list component
- `frontend/src/components/snippets/SnippetForm.tsx` - Snippet form component
- `migrations/YYYYMMDDHHMMSS_create_snippets_table.sql` - Snippets table migration

**Modified:**
- `crates/qa-pms-api/src/routes/mod.rs` - Add snippets routes
- `crates/qa-pms-api/src/app.rs` - Add snippets router, seed snippets on startup

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete BMAD structure
