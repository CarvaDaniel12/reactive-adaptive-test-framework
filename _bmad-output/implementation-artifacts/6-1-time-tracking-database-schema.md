# Story 6.1: Time Tracking Database Schema

Status: done

## Story

As a developer,
I want database tables for time tracking,
So that time data is persisted accurately.

## Acceptance Criteria

1. **Given** SQLx migrations infrastructure
   **When** time tracking schema migration runs
   **Then** `time_sessions` table is created

2. **Given** time_sessions table
   **Then** it has columns: id, workflow_instance_id, step_index, started_at, paused_at, resumed_at, ended_at, total_seconds, is_active

3. **Given** time tracking tables
   **Then** indexes exist for querying by workflow and user

## Tasks

- [ ] Task 1: Create migration file
- [ ] Task 2: Define Rust types
- [ ] Task 3: Build, test, and finalize

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### File List
