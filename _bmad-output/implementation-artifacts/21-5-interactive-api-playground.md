# Story 21.5: Interactive API Playground

Status: ready-for-dev

Epic: 21 - Developer Experience
Priority: P1 (High Value)
Estimated Effort: 2 days
Sprint: 1

## Story

As a **QA Engineer**,
I want to **explore and test the API endpoints interactively**,
So that **I can understand how to use the API and test my integrations**.

## Acceptance Criteria

1. **Given** I access the API Playground
   **When** I select an endpoint
   **Then** I see the endpoint documentation
   **And** I can fill in parameters
   **And** I can execute the request
   **And** I see the response (status, headers, body)

2. **Given** I want to save a request
   **When** I click "Save"
   **Then** it's saved to my collection
   **And** I can load it later
   **And** I can share it with team

3. **Given** I want to test an endpoint
   **When** I build a custom request
   **Then** I can set HTTP method (GET, POST, PUT, DELETE, PATCH)
   **And** I can set URL path
   **And** I can set headers
   **And** I can set request body (JSON)
   **And** I can execute the request

## Tasks / Subtasks

- [ ] Task 1: Create API Playground page and components
- [ ] Task 2: Implement endpoint explorer with OpenAPI spec parsing
- [ ] Task 3: Implement request builder with method/URL/headers/body
- [ ] Task 4: Implement request execution and response display
- [ ] Task 5: Implement request saving/loading functionality
- [ ] Task 6: Add request collection management (CRUD)
- [ ] Task 7: Add request sharing functionality
- [ ] Task 8: Add comprehensive tests

## Dev Notes

### Architecture Compliance
- **Frontend:** React 19 + Vite 7 + Tailwind CSS v4
- **API:** Axum 0.7+ with OpenAPI spec (utoipa)
- **Storage:** LocalStorage for collections, database for shared collections

### References
- **Source:** `_bmad-output/planning-artifacts/epics-detailed/epic-21-developer-experience.md#story-21.5`

## Dev Agent Record

### Agent Model Used
Claude Sonnet 4.5 (via Cursor)

### File List

**Created:**
- `frontend/src/pages/ApiPlayground/ApiPlaygroundPage.tsx`
- `frontend/src/components/api-playground/EndpointExplorer.tsx`
- `frontend/src/components/api-playground/RequestBuilder.tsx`
- `frontend/src/components/api-playground/ResponseViewer.tsx`
- `frontend/src/components/api-playground/RequestCollection.tsx`

**Modified:**
- `crates/qa-pms-api/src/app.rs` - Add OpenAPI spec endpoint (`/api/openapi.json`)

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete BMAD structure
