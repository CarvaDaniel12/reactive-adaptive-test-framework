# Story 21.10: Developer Portal with Interactive Docs

Status: ready-for-dev

Epic: 21 - Developer Experience
Priority: P1 (High Value)
Estimated Effort: 3 days
Sprint: 1

## Story

As a **QA Engineer**,
I want to **have interactive documentation**,
So that **I can learn the framework quickly and find answers to my questions**.

## Acceptance Criteria

1. **Given** I access the developer portal
   **When** I search for a topic
   **Then** I see relevant documentation sections
   **And** I can run code examples inline
   **And** I can copy examples to clipboard
   **And** search highlights matching terms

2. **Given** I'm learning a new feature
   **When** I read the docs
   **Then** I see interactive examples with expected outputs
   **And** I can navigate to related topics
   **And** I can see version-specific documentation
   **And** I can see changelog entries

3. **Given** documentation exists
   **When** I access the portal
   **Then** I see Getting Started section
   **And** I see API Reference section
   **And** I see Workflow Guide section
   **And** I see Integration Guides section
   **And** I see Examples section

## Tasks / Subtasks

- [ ] Task 1: Create documentation structure and storage (AC: #1, #2, #3)
  - [ ] 1.1: Create `docs/developer-portal/` directory structure
  - [ ] 1.2: Define `DocumentationSection` struct (id, title, description, order, subsections, code_examples)
  - [ ] 1.3: Store documentation in Markdown files or database
  - [ ] 1.4: Create documentation parser (Markdown to structured format)
  - [ ] 1.5: Extract code examples from documentation
  - [ ] 1.6: Create table of contents generator

- [ ] Task 2: Create documentation API endpoints (AC: #1, #2, #3)
  - [ ] 2.1: Create `crates/qa-pms-api/src/routes/docs.rs` module
  - [ ] 2.2: Add `GET /api/v1/docs` endpoint (list all sections)
  - [ ] 2.3: Add `GET /api/v1/docs/:section_id` endpoint (get section content)
  - [ ] 2.4: Add `GET /api/v1/docs/search` endpoint (search documentation)
  - [ ] 2.5: Add `POST /api/v1/docs/examples/:example_id/execute` endpoint (execute code example)
  - [ ] 2.6: Add OpenAPI documentation

- [ ] Task 3: Create interactive documentation UI (AC: #1, #2, #3)
  - [ ] 3.1: Create `frontend/src/pages/Docs/DeveloperPortalPage.tsx`
  - [ ] 3.2: Create `frontend/src/components/docs/DocumentationSidebar.tsx` component
  - [ ] 3.3: Create `frontend/src/components/docs/DocumentationContent.tsx` component
  - [ ] 3.4: Create `frontend/src/components/docs/CodeExampleViewer.tsx` component
  - [ ] 3.5: Implement Markdown rendering (react-markdown or similar)
  - [ ] 3.6: Implement syntax highlighting (prism.js or highlight.js)
  - [ ] 3.7: Add search functionality with highlighting
  - [ ] 3.8: Add copy to clipboard button for code examples
  - [ ] 3.9: Add "Run Example" button for executable examples
  - [ ] 3.10: Add table of contents sidebar

- [ ] Task 4: Implement code example execution (AC: #1, #2)
  - [ ] 4.1: Create `crates/qa-pms-api/src/routes/docs/examples.rs` module
  - [ ] 4.2: Implement example execution engine (YAML workflow examples, CLI command examples)
  - [ ] 4.3: Sandbox execution (prevent dangerous operations)
  - [ ] 4.4: Capture output (stdout, stderr)
  - [ ] 4.5: Return execution result to frontend
  - [ ] 4.6: Show execution status and output in UI

- [ ] Task 5: Create initial documentation content (AC: #3)
  - [ ] 5.1: Create Getting Started guide (installation, quick start, first workflow)
  - [ ] 5.2: Create API Reference (all endpoints with examples)
  - [ ] 5.3: Create Workflow Guide (creating workflows, templates, best practices)
  - [ ] 5.4: Create Integration Guides (Jira, Postman, Testmo, Splunk)
  - [ ] 5.5: Create Examples section (common use cases, patterns)
  - [ ] 5.6: Create FAQ section

- [ ] Task 6: Add comprehensive tests (AC: All)
  - [ ] 6.1: Test documentation API endpoints
  - [ ] 6.2: Test code example execution (sandboxed)
  - [ ] 6.3: Test search functionality
  - [ ] 6.4: Test documentation UI components

## Dev Notes

### Architecture Compliance
- **Tech Stack:** Rust 1.80+ with Tokio, Axum 0.7+, React 19 + Vite 7
- **Documentation:** Markdown files, parsed and rendered with react-markdown
- **Pattern:** Static documentation with dynamic execution for code examples

### Previous Story Intelligence
- From Story 13.1 (AI Companion): May have natural language query capabilities that can enhance documentation search

### References
- **Source:** `_bmad-output/planning-artifacts/epics-detailed/epic-21-developer-experience.md#story-21.10`

## Dev Agent Record

### Agent Model Used
Claude Sonnet 4.5 (via Cursor)

### File List

**Created:**
- `docs/developer-portal/` - Documentation content directory
- `crates/qa-pms-api/src/routes/docs.rs` - Documentation API endpoints
- `crates/qa-pms-api/src/routes/docs/examples.rs` - Example execution
- `frontend/src/pages/Docs/DeveloperPortalPage.tsx` - Documentation portal page
- `frontend/src/components/docs/DocumentationSidebar.tsx` - Sidebar navigation
- `frontend/src/components/docs/DocumentationContent.tsx` - Content renderer
- `frontend/src/components/docs/CodeExampleViewer.tsx` - Code example viewer

**Modified:**
- `crates/qa-pms-api/src/routes/mod.rs` - Add docs routes
- `crates/qa-pms-api/src/app.rs` - Add docs router

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete BMAD structure
