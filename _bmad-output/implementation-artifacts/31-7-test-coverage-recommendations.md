# Story 31.7: Test Coverage Recommendations

Status: ready-for-dev

Epic: 31 - AI-Enhanced Automation
Priority: P1 (High Value)
Estimated Effort: 3 days
Sprint: 1

## Story

As a **QA Engineer**,
I want to **receive AI-powered recommendations for improving test coverage**,
So that **I can ensure comprehensive testing and reduce bugs**.

## Acceptance Criteria

1. **Given** I have a set of test cases
   **When** I request coverage analysis
   **Then** the system identifies gaps in coverage (missing scenarios, insufficient edge cases, untested integrations)
   **And** suggests missing test scenarios
   **And** recommends edge cases to test
   **And** highlights untested integrations

2. **Given** I'm planning a test suite
   **When** I ask for test suggestions
   **Then** the system recommends test cases
   **And** prioritizes by risk and impact
   **And** provides templates for new tests

## Tasks / Subtasks

- [ ] Task 1: Create coverage analyzer service (AC: #1, #2)
  - [ ] 1.1: Create `crates/qa-pms-ai/src/coverage_analyzer.rs` module
  - [ ] 1.2: Create `CoverageAnalyzer` struct with AI client
  - [ ] 1.3: Define `CoverageAnalysis` struct (overall_coverage, category_coverage, gaps, recommendations, priority_matrix)
  - [ ] 1.4: Define `CategoryCoverage` struct (happy_path, edge_cases, error_handling, integrations, security, performance)
  - [ ] 1.5: Define `CoverageGap` struct (area, gap_type, description, severity, suggested_tests)
  - [ ] 1.6: Define `TestRecommendation` struct (title, description, category, priority, estimated_effort, template)

- [ ] Task 2: Implement coverage gap identification (AC: #1)
  - [ ] 2.1: Create `identify_gaps(tests, tickets)` method
  - [ ] 2.2: Analyze test categories (happy path, edge cases, error handling, integrations, security, performance)
  - [ ] 2.3: Identify missing scenarios (not covered by tests)
  - [ ] 2.4: Identify insufficient edge cases (few edge case tests)
  - [ ] 2.5: Identify untested integrations (integration points without tests)
  - [ ] 2.6: Identify unhandled errors (error cases not tested)
  - [ ] 2.7: Identify security concerns (security tests missing)
  - [ ] 2.8: Identify performance gaps (performance tests missing)

- [ ] Task 3: Implement AI-powered recommendations (AC: #1, #2)
  - [ ] 3.1: Create `generate_recommendations(gaps, category_coverage)` method
  - [ ] 3.2: Use AI to generate test recommendations for each gap
  - [ ] 3.3: Prioritize recommendations by risk and impact
  - [ ] 3.4: Generate test templates for recommended tests
  - [ ] 3.5: Estimate effort for each recommended test
  - [ ] 3.6: Return list of recommendations with priorities

- [ ] Task 4: Create coverage analysis API endpoints (AC: #1, #2)
  - [ ] 4.1: Create `crates/qa-pms-api/src/routes/ai/coverage.rs` module
  - [ ] 4.2: Add `POST /api/v1/ai/analyze-coverage` endpoint
  - [ ] 4.3: Request body: `{ test_ids: Vec<String>, ticket_keys: Vec<String> }`
  - [ ] 4.4: Return `CoverageAnalysis` with gaps and recommendations
  - [ ] 4.5: Add `POST /api/v1/ai/suggest-tests` endpoint
  - [ ] 4.6: Request body: `{ ticket_keys: Vec<String>, focus_areas: Vec<String> }`
  - [ ] 4.7: Return test recommendations with templates
  - [ ] 4.8: Add OpenAPI documentation

- [ ] Task 5: Create coverage analysis UI components (AC: #1, #2)
  - [ ] 5.1: Create `frontend/src/components/ai/CoverageAnalysis.tsx` component
  - [ ] 5.2: Display overall coverage score with visual indicator
  - [ ] 5.3: Display category coverage breakdown (charts/graphs)
  - [ ] 5.4: Display coverage gaps list with severity
  - [ ] 5.5: Display test recommendations with priorities
  - [ ] 5.6: Add "Analyze Coverage" button to test list page
  - [ ] 5.7: Add "Suggest Tests" button to ticket detail page
  - [ ] 5.8: Display recommended test templates

- [ ] Task 6: Add comprehensive tests (AC: All)
  - [ ] 6.1: Test coverage gap identification
  - [ ] 6.2: Test recommendation generation
  - [ ] 6.3: Test API endpoints
  - [ ] 6.4: Test UI components

## Dev Notes

### Architecture Compliance
- **Tech Stack:** Rust 1.80+ with Tokio, Axum 0.7+, AI client from Story 13.1
- **Pattern:** Coverage analysis → Gap identification → AI recommendations → Prioritization

### Previous Story Intelligence
- **From Story 31.1 (Auto-Test Generation):** Use test generation for recommendations
- **From Story 31.2 (Bug Prediction):** Use bug risk scores for prioritization
- **Key Integration Points:**
  - Analyze existing test cases
  - Recommend new tests based on gaps
  - Prioritize recommendations by risk

### References
- **Source:** `_bmad-output/planning-artifacts/epics-detailed/epic-31-ai-enhanced-automation.md#story-31.7`

## Dev Agent Record

### Agent Model Used
Claude Sonnet 4.5 (via Cursor)

### File List

**Created:**
- `crates/qa-pms-ai/src/coverage_analyzer.rs` - Coverage analysis service
- `crates/qa-pms-api/src/routes/ai/coverage.rs` - Coverage API endpoints
- `frontend/src/components/ai/CoverageAnalysis.tsx` - Coverage analysis UI component

**Modified:**
- `crates/qa-pms-ai/src/lib.rs` - Export coverage_analyzer module
- `crates/qa-pms-api/src/routes/ai/mod.rs` - Add coverage routes

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete BMAD structure
