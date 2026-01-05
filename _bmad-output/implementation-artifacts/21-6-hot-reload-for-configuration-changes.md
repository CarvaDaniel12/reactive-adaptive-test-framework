# Story 21.6: Hot Reload for Configuration Changes

Status: ready-for-dev

Epic: 21 - Developer Experience
Priority: P1 (High Value)
Estimated Effort: 2 days
Sprint: 1

## Story

As a **QA Engineer**,
I want to **see configuration changes applied immediately without restarting the server**,
So that **I can iterate quickly on workflow definitions and integrations**.

## Acceptance Criteria

1. **Given** the dev server is running
   **When** I modify a workflow YAML file
   **Then** the server detects the change
   **And** reloads the configuration automatically
   **And** shows a notification (success or error)
   **And** updates the UI without page refresh

2. **Given** I modify integration settings
   **When** I save the configuration file
   **Then** the integration is reconfigured automatically
   **And** connection is tested automatically
   **And** any errors are shown with details

3. **Given** hot reload is enabled
   **When** I edit a workflow file
   **Then** changes are detected within 500ms
   **And** configuration is reloaded without downtime
   **And** active workflows continue running unaffected

## Tasks / Subtasks

- [ ] Task 1: Implement file watcher for configuration files (AC: #1, #3)
  - [ ] 1.1: Add `notify` crate dependency
  - [ ] 1.2: Create `crates/qa-pms-core/src/dev/hot_reload.rs` module
  - [ ] 1.3: Implement `ConfigReloader` struct with file watcher
  - [ ] 1.4: Watch workflow YAML files in workflows directory
  - [ ] 1.5: Watch integration config files (jira_config.yaml, etc.)
  - [ ] 1.6: Implement debounced file watching (200ms delay)
  - [ ] 1.7: Reload configuration on file write events
  - [ ] 1.8: Handle file deletion events (remove config)
  - [ ] 1.9: Broadcast config updates via tokio::broadcast channel

- [ ] Task 2: Implement workflow reloader (AC: #1)
  - [ ] 2.1: Create `WorkflowReloader` struct
  - [ ] 2.2: Load workflows from YAML files in workflows directory
  - [ ] 2.3: Watch workflows directory recursively
  - [ ] 2.4: Reload workflow on file change
  - [ ] 2.5: Validate workflow structure after reload
  - [ ] 2.6: Update in-memory workflow cache
  - [ ] 2.7: Broadcast workflow updates to subscribers

- [ ] Task 3: Implement integration config reloader (AC: #2)
  - [ ] 3.1: Create `IntegrationConfigReloader` struct
  - [ ] 3.2: Watch integration config files (jira_config.yaml, postman_config.yaml, etc.)
  - [ ] 3.3: Reload integration settings on file change
  - [ ] 3.4: Test integration connection after reload
  - [ ] 3.5: Update integration clients with new config
  - [ ] 3.6: Broadcast integration status updates

- [ ] Task 4: Add hot reload API endpoints (AC: #1, #2)
  - [ ] 4.1: Create `GET /api/v1/dev/hot-reload/status` endpoint
  - [ ] 4.2: Create `POST /api/v1/dev/hot-reload/reload` endpoint (manual reload)
  - [ ] 4.3: Create WebSocket endpoint `/ws/hot-reload` for real-time updates
  - [ ] 4.4: Send reload notifications to connected clients
  - [ ] 4.5: Add OpenAPI documentation

- [ ] Task 5: Add frontend hot reload integration (AC: #1)
  - [ ] 5.1: Create `frontend/src/hooks/useHotReload.ts` hook
  - [ ] 5.2: Connect to WebSocket endpoint for hot reload updates
  - [ ] 5.3: Show toast notification on config reload (success/error)
  - [ ] 5.4: Refresh workflow list on workflow reload
  - [ ] 5.5: Refresh integration status on integration reload
  - [ ] 5.6: Handle WebSocket reconnection on disconnect

- [ ] Task 6: Add comprehensive tests (AC: All)
  - [ ] 6.1: Add unit tests for file watcher
  - [ ] 6.2: Add integration tests for workflow reload
  - [ ] 6.3: Add integration tests for config reload
  - [ ] 6.4: Test WebSocket notifications

## Dev Notes

### Architecture Compliance
- **Tech Stack:** Rust 1.80+ with Tokio, `notify` crate for file watching, WebSocket for real-time updates
- **Code Structure:** `crates/qa-pms-core/src/dev/hot_reload.rs`, API routes in `qa-pms-api/src/routes/dev_tools.rs`
- **Pattern:** File watcher → Config reload → Broadcast update → Frontend refresh

### Previous Story Intelligence
- From Story 5.1 (Workflow Templates): Workflow structure for reloading
- From Story 3.1 (Jira Integration): Integration config structure

### References
- **Source:** `_bmad-output/planning-artifacts/epics-detailed/epic-21-developer-experience.md#story-21.6`

## Dev Agent Record

### Agent Model Used
Claude Sonnet 4.5 (via Cursor)

### File List

**Created:**
- `crates/qa-pms-core/src/dev/hot_reload.rs` - Hot reload implementation
- `crates/qa-pms-api/src/routes/dev_tools.rs` - Hot reload API endpoints (if not exists)
- `frontend/src/hooks/useHotReload.ts` - Frontend hot reload hook

**Modified:**
- `crates/qa-pms-api/src/app.rs` - Add hot reloader to AppState and enable in dev mode

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete BMAD structure
