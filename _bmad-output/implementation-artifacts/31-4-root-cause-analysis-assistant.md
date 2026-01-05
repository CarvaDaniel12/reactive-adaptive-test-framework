# Story 31.4: Root Cause Analysis Assistant

Status: ready-for-dev

Epic: 31 - AI-Enhanced Automation
Priority: P1 (High Value)
Estimated Effort: 3 days
Sprint: 1

## Story

As a **QA Engineer**,
I want to **get AI assistance in analyzing test failures and identifying root causes**,
So that **I can quickly understand and resolve issues**.

## Acceptance Criteria

1. **Given** a test fails with an error message
   **When** I click "Analyze Failure"
   **Then** the AI analyzes logs, error messages, and test steps
   **And** identifies likely root causes with probabilities
   **And** suggests investigation steps
   **And** recommends potential fixes

2. **Given** I have multiple related test failures
   **When** I request cluster analysis
   **Then** the AI identifies patterns
   **And** groups related failures
   **And** suggests common root causes

3. **Given** root cause analysis is complete
   **When** I review the analysis
   **Then** I can see failure summary
   **And** I can see likely causes with evidence
   **And** I can see investigation steps
   **And** I can see suggested fixes

## Tasks / Subtasks

- [ ] Task 1: Create root cause analyzer service (AC: #1, #2, #3)
  - [ ] 1.1: Create `crates/qa-pms-ai/src/root_cause_analyzer.rs` module
  - [ ] 1.2: Create `RootCauseAnalyzer` struct with AI client
  - [ ] 1.3: Define `RootCauseAnalysis` struct (test_execution_id, failure_summary, likely_causes, investigation_steps, suggested_fixes, confidence)
  - [ ] 1.4: Define `LikelyCause` struct (cause, probability, evidence, category)
  - [ ] 1.5: Define `CauseCategory` enum (CodeChange, Environment, TestData, Integration, Configuration, Timing, Other)
  - [ ] 1.6: Define `SuggestedFix` struct (fix, description, confidence)

- [ ] Task 2: Implement failure analysis prompt engineering (AC: #1)
  - [ ] 2.1: Create `build_analysis_prompt()` method
  - [ ] 2.2: Include test information (title, steps, expected, actual)
  - [ ] 2.3: Include error details (error message, type, stack trace)
  - [ ] 2.4: Include test environment (browser, environment, build version)
  - [ ] 2.5: Include related tickets (if any)
  - [ ] 2.6: Include test logs
  - [ ] 2.7: Specify JSON response format (failure_summary, likely_causes, investigation_steps, suggested_fixes)

- [ ] Task 3: Implement cluster analysis for multiple failures (AC: #2)
  - [ ] 3.1: Create `analyze_cluster(failures)` method
  - [ ] 3.2: Extract common patterns from error messages
  - [ ] 3.3: Group failures by similarity (error type, stack trace patterns, test steps)
  - [ ] 3.4: Identify common root causes across cluster
  - [ ] 3.5: Use AI to analyze cluster patterns
  - [ ] 3.6: Return cluster analysis with common causes

- [ ] Task 4: Create root cause analysis API endpoints (AC: #1, #2, #3)
  - [ ] 4.1: Create `crates/qa-pms-api/src/routes/ai/root_cause.rs` module
  - [ ] 4.2: Add `POST /api/v1/ai/analyze-failure` endpoint
  - [ ] 4.3: Request body: `{ test_execution_id, include_logs: bool }`
  - [ ] 4.4: Fetch test execution, logs, related tickets
  - [ ] 4.5: Run root cause analysis
  - [ ] 4.6: Return `RootCauseAnalysis`
  - [ ] 4.7: Add `POST /api/v1/ai/analyze-cluster` endpoint
  - [ ] 4.8: Request body: `{ test_execution_ids: Vec<String> }`
  - [ ] 4.9: Return cluster analysis
  - [ ] 4.10: Add OpenAPI documentation

- [ ] Task 5: Create root cause analysis UI components (AC: #1, #2, #3)
  - [ ] 5.1: Create `frontend/src/components/ai/RootCauseAnalysis.tsx` component
  - [ ] 5.2: Add "Analyze Failure" button to failed test details
  - [ ] 5.3: Display failure summary
  - [ ] 5.4: Display likely causes with probabilities and evidence
  - [ ] 5.5: Display investigation steps (numbered list)
  - [ ] 5.6: Display suggested fixes with confidence levels
  - [ ] 5.7: Add "Analyze Cluster" button for multiple failures
  - [ ] 5.8: Display cluster analysis with common causes

- [ ] Task 6: Add comprehensive tests (AC: All)
  - [ ] 6.1: Test failure analysis for various error types
  - [ ] 6.2: Test cluster analysis with related failures
  - [ ] 6.3: Test prompt building and parsing
  - [ ] 6.4: Test API endpoints
  - [ ] 6.5: Test UI components

## Dev Notes

### Architecture Compliance
- **Tech Stack:** Rust 1.80+ with Tokio, Axum 0.7+, AI client from Story 13.1
- **Pattern:** AI prompt engineering → Analysis → Root cause identification → Recommendations

### Context7 Requirements (MANDATORY)
**CRITICAL:** Before implementing, use Context7 MCP to:
1. **Query Context7 for:** "How to implement root cause analysis prompts for AI models"
2. **Query Context7 for:** "Best practices for failure pattern clustering and analysis"

### Previous Story Intelligence
- **From Story 13.1 (AI Companion):** Reuse `AIClient` for AI analysis
- **From Story 5.3 (Workflow Execution):** Test execution data structure
- **Key Integration Points:**
  - Fetch test execution and logs from database
  - Link to related tickets for context

### References
- **Source:** `_bmad-output/planning-artifacts/epics-detailed/epic-31-ai-enhanced-automation.md#story-31.4`

## Dev Agent Record

### Agent Model Used
Claude Sonnet 4.5 (via Cursor)

### File List

**Created:**
- `crates/qa-pms-ai/src/root_cause_analyzer.rs` - Root cause analysis service
- `crates/qa-pms-api/src/routes/ai/root_cause.rs` - Root cause API endpoints
- `frontend/src/components/ai/RootCauseAnalysis.tsx` - Root cause analysis UI component

**Modified:**
- `crates/qa-pms-ai/src/lib.rs` - Export root_cause_analyzer module
- `crates/qa-pms-api/src/routes/ai/mod.rs` - Add root_cause routes

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete BMAD structure
