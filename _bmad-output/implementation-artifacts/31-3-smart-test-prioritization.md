# Story 31.3: Smart Test Prioritization

Status: ready-for-dev

Epic: 31 - AI-Enhanced Automation
Priority: P1 (High Value)
Estimated Effort: 3 days
Sprint: 1

## Story

As a **QA Engineer**,
I want to **automatically prioritize tests based on risk and impact**,
So that **I can focus on the most critical tests first when time is limited**.

## Acceptance Criteria

1. **Given** I have 100 test cases and only 2 hours
   **When** I ask for test priority
   **Then** the system ranks tests by priority score
   **And** suggests which tests to run first
   **And** provides justification for each priority
   **And** ensures coverage of high-risk areas

2. **Given** I'm running a quick smoke test
   **When** I select "Smart Smoke Test"
   **Then** the system selects critical tests (top 10-15 tests)
   **And** ensures coverage of high-risk areas
   **And** minimizes execution time
   **And** maximizes risk coverage

3. **Given** test prioritization is applied
   **When** I execute prioritized tests
   **Then** tests run in priority order
   **And** I can see progress and results
   **And** I can stop early if critical tests pass

## Tasks / Subtasks

- [ ] Task 1: Create test prioritizer service (AC: #1, #2, #3)
  - [ ] 1.1: Create `crates/qa-pms-ai/src/test_prioritizer.rs` module
  - [ ] 1.2: Create `TestPrioritizer` struct with bug predictor dependency
  - [ ] 1.3: Define `TestPriority` struct (test_id, priority_score, risk_level, factors, estimated_time, should_run)
  - [ ] 1.4: Define `PriorityFactor` struct (factor, impact, description)
  - [ ] 1.5: Define `PrioritizationConstraints` struct (max_time, max_tests, min_coverage)

- [ ] Task 2: Implement priority calculation algorithm (AC: #1, #2)
  - [ ] 2.1: Create `prioritize_tests(tests, tickets, constraints)` method
  - [ ] 2.2: Calculate priority score for each test (weighted factors)
  - [ ] 2.3: Factor 1: Base test priority (Critical=1.0, High=0.8, Medium=0.6, Low=0.4) - weight 0.3
  - [ ] 2.4: Factor 2: Ticket risk score (from bug predictor) - weight 0.3
  - [ ] 2.5: Factor 3: Test execution frequency (frequently run = higher priority) - weight 0.2
  - [ ] 2.6: Factor 4: Test failure rate (frequently failing = higher priority) - weight 0.2
  - [ ] 2.7: Sort tests by priority score (descending)
  - [ ] 2.8: Apply constraints (time, count, coverage)

- [ ] Task 3: Implement constraint application (AC: #1, #2)
  - [ ] 3.1: Create `apply_constraints(priorities, constraints)` method
  - [ ] 3.2: Apply time constraint (select tests until time budget exhausted)
  - [ ] 3.3: Apply count constraint (select top N tests)
  - [ ] 3.4: Apply coverage constraint (ensure all high-risk areas covered)
  - [ ] 3.5: Mark tests as `should_run=true` or `should_run=false`
  - [ ] 3.6: Calculate total execution time for selected tests

- [ ] Task 4: Implement smart smoke test selection (AC: #2)
  - [ ] 4.1: Create `select_smart_smoke_tests(tests, tickets)` method
  - [ ] 4.2: Select top 10-15 tests by priority score
  - [ ] 4.3: Ensure coverage: at least 1 test per high-risk area
  - [ ] 4.4: Minimize execution time (prefer shorter tests)
  - [ ] 4.5: Maximize risk coverage (prefer high-risk tests)
  - [ ] 4.6: Return selected test list with justification

- [ ] Task 5: Create prioritization API endpoints (AC: #1, #2, #3)
  - [ ] 5.1: Create `crates/qa-pms-api/src/routes/ai/test_prioritization.rs` module
  - [ ] 5.2: Add `POST /api/v1/ai/prioritize-tests` endpoint
  - [ ] 5.3: Request body: `{ test_ids: Vec<String>, ticket_keys: Vec<String>, constraints: PrioritizationConstraints }`
  - [ ] 5.4: Return prioritized test list with scores and justifications
  - [ ] 5.5: Add `POST /api/v1/ai/smart-smoke-test` endpoint
  - [ ] 5.6: Request body: `{ ticket_keys: Vec<String> }`
  - [ ] 5.7: Return selected smoke tests with justification
  - [ ] 5.8: Add OpenAPI documentation

- [ ] Task 6: Create prioritization UI components (AC: #1, #2, #3)
  - [ ] 6.1: Create `frontend/src/components/ai/TestPrioritizer.tsx` component
  - [ ] 6.2: Add "Prioritize Tests" button to test list page
  - [ ] 6.3: Add constraints form (max time, max tests, min coverage)
  - [ ] 6.4: Display prioritized test list with scores
  - [ ] 6.5: Show priority factors for each test
  - [ ] 6.6: Add "Run Prioritized Tests" button
  - [ ] 6.7: Add "Smart Smoke Test" button
  - [ ] 6.8: Display selected smoke tests with justification

- [ ] Task 7: Integrate with test execution (AC: #3)
  - [ ] 7.1: Update test execution workflow to respect priority order
  - [ ] 7.2: Execute tests in priority order (highest first)
  - [ ] 7.3: Allow stopping early if critical tests pass
  - [ ] 7.4: Show priority in test execution UI

- [ ] Task 8: Add comprehensive tests (AC: All)
  - [ ] 8.1: Test priority calculation algorithm
  - [ ] 8.2: Test constraint application
  - [ ] 8.3: Test smart smoke test selection
  - [ ] 8.4: Test API endpoints
  - [ ] 8.5: Test prioritization UI components

## Dev Notes

### Architecture Compliance
- **Tech Stack:** Rust 1.80+ with Tokio, Axum 0.7+
- **Pattern:** Priority scoring algorithm → Constraint application → Test selection

### Previous Story Intelligence
- **From Story 31.2 (Bug Prediction):** Use `BugPredictor` for ticket risk scores
- **From Story 5.3 (Workflow Execution):** Test execution infrastructure
- **Key Integration Points:**
  - Use bug risk scores from Story 31.2
  - Integrate with test execution from Story 5.3

### References
- **Source:** `_bmad-output/planning-artifacts/epics-detailed/epic-31-ai-enhanced-automation.md#story-31.3`

## Dev Agent Record

### Agent Model Used
Claude Sonnet 4.5 (via Cursor)

### File List

**Created:**
- `crates/qa-pms-ai/src/test_prioritizer.rs` - Test prioritization service
- `crates/qa-pms-api/src/routes/ai/test_prioritization.rs` - Prioritization API endpoints
- `frontend/src/components/ai/TestPrioritizer.tsx` - Prioritization UI component

**Modified:**
- `crates/qa-pms-ai/src/lib.rs` - Export test_prioritizer module
- `crates/qa-pms-api/src/routes/ai/mod.rs` - Add test_prioritization routes

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete BMAD structure
