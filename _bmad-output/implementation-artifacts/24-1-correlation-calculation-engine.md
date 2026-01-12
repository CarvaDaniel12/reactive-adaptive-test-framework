# Story 24.1: Correlation Calculation Engine

Status: ready-for-dev

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

**As a** QA Engineer working on PMS integration quality  
**I want** correlation calculation logic between test results and integration health  
**So that** I can identify patterns where test failures precede integration failures and prioritize test work accordingly

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 24.1 |
| Epic | Epic 24: Test-Integration Correlation Engine |
| Sprint | Sprint 1: Correlation Engine |
| Priority | P1 |
| Estimated Days | 2 |
| Dependencies | Epic 22 (Story 22.4: Integration Health Service), Testmo integration (exists) |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create `qa-pms-correlation` crate
   - Follow existing crate patterns (separate crate in workspace)
   - Dependencies: `qa-pms-core`, `qa-pms-integration-health`, `qa-pms-testmo`
   - Structure: `lib.rs`, `types.rs`, `engine.rs`, `error.rs`

2. Implement correlation calculation logic
   - Correlate test results (Testmo) with integration events (`integration_health`, `integration_events`)
   - Calculate correlation score (0.0-1.0, where 1.0 is perfect correlation)
   - Identify patterns: test failures precede integration failures
   - Time window for correlation: configurable (default: 1 hour)
   - Correlation confidence score (0.0-1.0)

3. Correlation algorithm
   - Time-window matching: test failures within time window before integration failures
   - Pattern detection: identify if test failure precedes integration failure
   - Score calculation: based on temporal proximity and pattern matching
   - Confidence calculation: based on correlation frequency and accuracy
   - Target accuracy: > 85%

4. Use existing integrations
   - Testmo integration (`qa-pms-testmo` crate) - already exists
   - Integration Health Service (`qa-pms-integration-health` crate) - from Epic 22

---

## Acceptance Criteria

- [ ] **Given** correlation engine exists  
  **When** correlating test results with integration events  
  **Then** correlation score is calculated (0.0-1.0)

- [ ] **Given** correlation engine exists  
  **When** test failures precede integration failures within time window  
  **Then** pattern is identified and correlation score reflects the pattern

- [ ] **Given** correlation engine exists  
  **When** using configurable time window  
  **Then** correlation uses specified time window (default: 1 hour)

- [ ] **Given** correlation engine exists  
  **When** calculating confidence  
  **Then** confidence score is calculated (0.0-1.0) based on correlation frequency and accuracy

- [ ] **Given** correlation engine exists  
  **When** comparing with expected correlations  
  **Then** correlation accuracy > 85%

---

## Tasks / Subtasks

- [ ] Task 1: Create correlation crate (AC: #1)
  - [ ] 1.1: Create `crates/qa-pms-correlation/Cargo.toml`
  - [ ] 1.2: Add dependencies: `qa-pms-core`, `qa-pms-integration-health`, `qa-pms-testmo`
  - [ ] 1.3: Create `src/lib.rs` with module exports
  - [ ] 1.4: Add crate to workspace `Cargo.toml`

- [ ] Task 2: Create correlation types (AC: #1, #2)
  - [ ] 2.1: Create `src/types.rs` with `Correlation` struct
  - [ ] 2.2: Define `CorrelationType` enum ('high', 'medium', 'low')
  - [ ] 2.3: Define `CorrelationPattern` enum (e.g., 'test_failure_precedes_integration_failure')
  - [ ] 2.4: Add serde serialization with camelCase
  - [ ] 2.5: Add utoipa ToSchema for OpenAPI documentation

- [ ] Task 3: Create correlation error types (AC: #1)
  - [ ] 3.1: Create `src/error.rs` with `CorrelationError` enum
  - [ ] 3.2: Use `thiserror` for error types
  - [ ] 3.3: Implement `From<CorrelationError> for ApiError` for API boundaries

- [ ] Task 4: Implement correlation engine (AC: #1, #2, #3, #4)
  - [ ] 4.1: Create `src/engine.rs` with `CorrelationEngine` struct
  - [ ] 4.2: Implement `calculate_correlation` method
  - [ ] 4.3: Implement time-window matching logic
  - [ ] 4.4: Implement pattern detection (test failure precedes integration failure)
  - [ ] 4.5: Implement correlation score calculation (0.0-1.0)
  - [ ] 4.6: Implement confidence score calculation (0.0-1.0)
  - [ ] 4.7: Support configurable time window (default: 1 hour)

- [ ] Task 5: Integrate with Testmo and Integration Health (AC: #1, #2)
  - [ ] 5.1: Use `qa-pms-testmo` to fetch test results
  - [ ] 5.2: Use `qa-pms-integration-health` to fetch integration events
  - [ ] 5.3: Match test results with integration events by time window
  - [ ] 5.4: Handle errors from both integrations

- [ ] Task 6: Add unit tests (AC: #1, #2, #3, #4, #5)
  - [ ] 6.1: Test correlation score calculation with various inputs
  - [ ] 6.2: Test pattern detection logic
  - [ ] 6.3: Test time window matching
  - [ ] 6.4: Test confidence score calculation
  - [ ] 6.5: Test correlation accuracy (mock data with known correlations)

- [ ] Task 7: Add integration tests (AC: #1, #2, #3, #4, #5)
  - [ ] 7.1: Test correlation engine with real Testmo data
  - [ ] 7.2: Test correlation engine with real integration health data
  - [ ] 7.3: Verify correlation accuracy > 85% with test data

---

## Files to Create

| File | Changes |
|------|---------|
| `crates/qa-pms-correlation/Cargo.toml` | Create new crate with dependencies |
| `crates/qa-pms-correlation/src/lib.rs` | Create crate root with module exports |
| `crates/qa-pms-correlation/src/types.rs` | Create correlation types (Correlation, CorrelationType, CorrelationPattern) |
| `crates/qa-pms-correlation/src/engine.rs` | Create correlation engine (CorrelationEngine) |
| `crates/qa-pms-correlation/src/error.rs` | Create error types (CorrelationError) |

---

## Files to Modify

| File | Changes |
|------|---------|
| `Cargo.toml` (workspace root) | Add `qa-pms-correlation` to `members` and `workspace.dependencies` |
| `crates/qa-pms-api/Cargo.toml` | Add `qa-pms-correlation` dependency (for future API endpoints) |

---

## Dev Notes

### Correlation Algorithm

**Time-Window Matching:**
- Default time window: 1 hour (configurable)
- Match test failures with integration failures within time window
- Test failure must precede integration failure for positive correlation

**Correlation Score Calculation:**
- Score range: 0.0 (no correlation) to 1.0 (perfect correlation)
- Based on temporal proximity (closer events = higher score)
- Pattern matching bonus (test failure precedes integration failure)
- Formula: `score = temporal_proximity_weight * time_factor + pattern_weight * pattern_match`

**Confidence Score Calculation:**
- Score range: 0.0 (low confidence) to 1.0 (high confidence)
- Based on correlation frequency (more occurrences = higher confidence)
- Based on accuracy (verified correlations = higher confidence)
- Formula: `confidence = frequency_factor * frequency + accuracy_factor * accuracy`

**Pattern Detection:**
- Primary pattern: `test_failure_precedes_integration_failure`
- Identify test failures that occur before integration failures
- Time window: test failure within 1 hour before integration failure

### Project Structure Notes

**Crate Structure:**
- Follow existing crate patterns (`qa-pms-patterns`, `qa-pms-integration-health`)
- Separate crate: `qa-pms-correlation`
- Module structure: `lib.rs`, `types.rs`, `engine.rs`, `error.rs`

**Dependencies:**
- `qa-pms-core`: Shared types, error handling
- `qa-pms-integration-health`: Integration health data (Epic 22)
- `qa-pms-testmo`: Test results (existing crate)

**Integration Points:**
- Testmo: Use `qa-pms-testmo::TestmoClient` to fetch test results
- Integration Health: Use `qa-pms-integration-health::IntegrationHealthService` to fetch integration events

### Testing Standards

**Unit Tests:**
- Test correlation score calculation with mock data
- Test pattern detection logic
- Test time window matching
- Test confidence score calculation

**Integration Tests:**
- Test with real Testmo data
- Test with real integration health data
- Verify correlation accuracy > 85%

**Manual Tests:**
- Verify correlation accuracy with known test cases
- Verify correlation performance (< 500ms)

### References

- Epic Document: `_bmad-output/planning-artifacts/epics-observability-pms-integrations-2026-01-10.md` (Epic 24, Story 24.1)
- Architecture Document: `_bmad-output/planning-artifacts/architecture-observability-pms-integrations-2026-01-10.md` (Section 3.1, Module 3: qa-pms-correlation)
- Integration Health Service: Story 22.4 (required dependency)
- Testmo Integration: `qa-intelligent-pms/crates/qa-pms-testmo` (existing crate)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
