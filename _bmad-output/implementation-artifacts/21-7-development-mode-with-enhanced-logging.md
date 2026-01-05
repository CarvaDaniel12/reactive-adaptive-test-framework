# Story 21.7: Development Mode with Enhanced Logging

Status: ready-for-dev

Epic: 21 - Developer Experience
Priority: P1 (High Value)
Estimated Effort: 1 day
Sprint: 1

## Story

As a **QA Engineer**,
I want to **have a development mode with detailed logging**,
So that **I can debug workflows and integrations easily**.

## Acceptance Criteria

1. **Given** I start the dev server with debug mode
   **When** I execute workflows
   **Then** all API calls are logged with method, URL, status, duration
   **And** detailed integration responses are shown
   **And** timing information is displayed for each step
   **And** errors show full stack traces

2. **Given** I'm troubleshooting an issue
   **When** I need to see what happened
   **Then** I can filter logs by workflow ID or step ID
   **And** I can export logs for analysis (JSON, CSV)
   **And** I can replay failed steps

3. **Given** enhanced logging is enabled
   **When** workflow executes
   **Then** structured logs are created with trace IDs
   **And** logs include workflow context (user, ticket, environment)
   **And** logs are stored for querying

## Tasks / Subtasks

- [ ] Task 1: Enhance logging configuration for development mode (AC: #1)
  - [ ] 1.1: Update `setup_logging()` function in `main.rs` to accept debug flag
  - [ ] 1.2: Set log level to `debug` when debug mode enabled
  - [ ] 1.3: Enable file and line number in logs
  - [ ] 1.4: Enable JSON structured logging format
  - [ ] 1.5: Add trace ID to all log entries
  - [ ] 1.6: Create `WorkflowLogger` struct with structured logging methods

- [ ] Task 2: Implement structured logging for workflows (AC: #1, #3)
  - [ ] 2.1: Create `crates/qa-pms-core/src/logging/workflow.rs` module
  - [ ] 2.2: Implement `WorkflowLogger` with workflow context
  - [ ] 2.3: Add `log_step_start()` method with step details
  - [ ] 2.4: Add `log_step_complete()` method with duration
  - [ ] 2.5: Add `log_step_error()` method with full stack trace
  - [ ] 2.6: Add `log_api_call()` method for API requests
  - [ ] 2.7: Add `log_integration_response()` method for integration calls
  - [ ] 2.8: Use `tracing::instrument` macro for automatic span creation

- [ ] Task 3: Implement log filtering and export (AC: #2)
  - [ ] 3.1: Create `GET /api/v1/dev/logs` endpoint with filters (workflow_id, step_id, level, date_range)
  - [ ] 3.2: Implement log storage in database or file system
  - [ ] 3.3: Add `POST /api/v1/dev/logs/export` endpoint (JSON/CSV format)
  - [ ] 3.4: Add frontend log viewer component with filters
  - [ ] 3.5: Add export functionality to log viewer

- [ ] Task 4: Add comprehensive tests (AC: All)
  - [ ] 4.1: Test logging configuration in debug mode
  - [ ] 4.2: Test structured logging methods
  - [ ] 4.3: Test log filtering and export

## Dev Notes

### Architecture Compliance
- **Tech Stack:** Rust 1.80+ with Tokio, `tracing` + `tracing-subscriber` for structured logging
- **Pattern:** Structured logging with spans, fields, and context propagation

### Previous Story Intelligence
- From Story 14.1 (Observability): Logging infrastructure already exists
- This story enhances it for development/debugging purposes

### References
- **Source:** `_bmad-output/planning-artifacts/epics-detailed/epic-21-developer-experience.md#story-21.7`

## Dev Agent Record

### Agent Model Used
Claude Sonnet 4.5 (via Cursor)

### File List

**Created:**
- `crates/qa-pms-core/src/logging/workflow.rs` - Workflow structured logging
- `crates/qa-pms-api/src/routes/dev_tools.rs` - Log endpoints (if not exists)
- `frontend/src/components/dev/LogViewer.tsx` - Log viewer component

**Modified:**
- `crates/qa-pms-api/src/main.rs` - Enhanced logging setup

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete BMAD structure
