# Story 21.8: Debug Tools and Profiling Helpers

Status: ready-for-dev

Epic: 21 - Developer Experience
Priority: P1 (High Value)
Estimated Effort: 2 days
Sprint: 1

## Story

As a **QA Engineer**,
I want to **have debugging tools to understand workflow execution**,
So that **I can identify bottlenecks and fix issues quickly**.

## Acceptance Criteria

1. **Given** I'm debugging a workflow
   **When** I enable debug mode
   **Then** I see step-by-step execution with pause/resume
   **And** I can pause at any step
   **And** I can inspect variables and state
   **And** I can replay steps

2. **Given** I need to analyze performance
   **When** I view profiling data
   **Then** I see time spent per step
   **And** I can identify slow operations
   **And** I can compare with historical data

3. **Given** profiling middleware is enabled
   **When** requests are processed
   **Then** profiling data is collected (request time, DB queries, external calls)
   **And** profiling data is exposed via API endpoint
   **And** profiling overhead is minimal (< 5ms)

## Tasks / Subtasks

- [ ] Task 1: Implement profiling middleware (AC: #3)
  - [ ] 1.1: Create `crates/qa-pms-api/src/middleware/profiling.rs` module
  - [ ] 1.2: Implement `profiling_middleware()` using Axum middleware pattern
  - [ ] 1.3: Record request start time
  - [ ] 1.4: Track database queries (SQLx query logging integration)
  - [ ] 1.5: Track external API calls (HTTP client integration)
  - [ ] 1.6: Calculate request duration
  - [ ] 1.7: Store profiling data in request extensions
  - [ ] 1.8: Add profiling headers to response (X-Request-ID, X-Request-Duration)

- [ ] Task 2: Implement debug mode execution (AC: #1)
  - [ ] 2.1: Create `crates/qa-pms-core/src/debug/mod.rs` module
  - [ ] 2.2: Implement `DebugMode` enum (Normal, StepByStep, Breakpoint)
  - [ ] 2.3: Create `DebugWorkflowExecutor` struct
  - [ ] 2.4: Implement breakpoint support (pause at specific step)
  - [ ] 2.5: Implement step-by-step execution (pause after each step)
  - [ ] 2.6: Implement state inspection (workflow state, step results, variables)
  - [ ] 2.7: Implement step replay functionality

- [ ] Task 3: Create profiling API endpoints (AC: #2, #3)
  - [ ] 3.1: Add `GET /api/v1/dev/profiling/request/:request_id` endpoint
  - [ ] 3.2: Add `GET /api/v1/dev/profiling/workflow/:workflow_id` endpoint
  - [ ] 3.3: Add `GET /api/v1/dev/profiling/slowest` endpoint (top N slowest requests)
  - [ ] 3.4: Add OpenAPI documentation

- [ ] Task 4: Create debug UI components (AC: #1, #2)
  - [ ] 4.1: Create `frontend/src/components/debug/DebugPanel.tsx` component
  - [ ] 4.2: Add step-by-step execution controls (play, pause, step, restart)
  - [ ] 4.3: Add state inspector (workflow state, variables, step results)
  - [ ] 4.4: Add profiling viewer (timeline, slow operations, comparison)
  - [ ] 4.5: Add breakpoint management UI

- [ ] Task 5: Add comprehensive tests (AC: All)
  - [ ] 5.1: Test profiling middleware overhead
  - [ ] 5.2: Test debug mode execution
  - [ ] 5.3: Test profiling API endpoints

## Dev Notes

### Architecture Compliance
- **Tech Stack:** Rust 1.80+ with Tokio, Axum 0.7+, `tracing` for profiling
- **Pattern:** Middleware for profiling, debug state machine for execution control

### Previous Story Intelligence
- From Story 14.3 (Prometheus Metrics): Performance metrics infrastructure
- This story adds detailed profiling and debugging capabilities

### References
- **Source:** `_bmad-output/planning-artifacts/epics-detailed/epic-21-developer-experience.md#story-21.8`

## Dev Agent Record

### Agent Model Used
Claude Sonnet 4.5 (via Cursor)

### File List

**Created:**
- `crates/qa-pms-api/src/middleware/profiling.rs` - Profiling middleware
- `crates/qa-pms-core/src/debug/mod.rs` - Debug execution module
- `frontend/src/components/debug/DebugPanel.tsx` - Debug UI component

**Modified:**
- `crates/qa-pms-api/src/app.rs` - Add profiling middleware (dev mode only)

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete BMAD structure
