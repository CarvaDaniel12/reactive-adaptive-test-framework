---
stepsCompleted: [1, 2, 3, 4]
inputDocuments:
  - _bmad-output/planning-artifacts/prd.md
  - _bmad-output/planning-artifacts/architecture.md
  - _bmad-output/planning-artifacts/ux-design-specification.md
---

# QA Intelligent PMS - Companion Framework - Epic Breakdown

## Overview

This document provides the complete epic and story breakdown for QA Intelligent PMS - Companion Framework, decomposing the requirements from the PRD, UX Design, and Architecture requirements into implementable stories.

## Requirements Inventory

### Functional Requirements

**FR-INT: Integration Layer**
- FR-INT-01: Jira OAuth 2.0 + PKCE authentication for secure API access
- FR-INT-02: Jira ticket listing with filters (backlog, ready for QA, QA in progress, UAT)
- FR-INT-03: Jira ticket details retrieval (title, description, priority, acceptance criteria)
- FR-INT-04: Jira ticket status updates and comments
- FR-INT-05: Postman API key authentication
- FR-INT-06: Postman collection search by keywords and ticket context
- FR-INT-07: Postman test case retrieval with direct links
- FR-INT-08: Testmo API key authentication
- FR-INT-09: Testmo test case search and synchronization
- FR-INT-10: Testmo test run creation and result tracking
- FR-INT-11: Splunk manual query interface with pre-built templates
- FR-INT-12: Splunk log retrieval and display
- FR-INT-13: Integration health checks every 60 seconds
- FR-INT-14: Integration status dashboard (online/offline/degraded)
- FR-INT-15: Credential validation on startup

**FR-WRK: Workflow Engine**
- FR-WRK-01: Workflow templates by ticket type (bug fix, feature test, regression)
- FR-WRK-02: Step-by-step workflow guidance with visual progress
- FR-WRK-03: Workflow step completion tracking (checkboxes/progress)
- FR-WRK-04: Notes attachment per workflow step
- FR-WRK-05: Workflow pause/resume functionality
- FR-WRK-06: Automatic workflow state persistence
- FR-WRK-07: Workflow completion with summary generation

**FR-TRK: Time Tracking**
- FR-TRK-01: Automatic timer start when workflow begins
- FR-TRK-02: Timer pause/resume controls
- FR-TRK-03: Real-time timer display in header
- FR-TRK-04: Time per step tracking
- FR-TRK-05: Total time calculation per ticket
- FR-TRK-06: Time comparison: actual vs estimate (with gap alerts >20%)
- FR-TRK-07: Historical time data storage for analytics

**FR-RPT: Reporting & Documentation**
- FR-RPT-01: Automatic report generation on workflow completion
- FR-RPT-02: Report includes: steps completed, time breakdown, tests covered
- FR-RPT-03: Report includes: strategies used, gap analysis
- FR-RPT-04: Report export in Markdown/HTML formats
- FR-RPT-05: Report preview before saving
- FR-RPT-06: Historical report storage and retrieval

**FR-DSH: Dashboard System**
- FR-DSH-01: QA individual dashboard (tickets completed, time metrics, gaps)
- FR-DSH-02: PM dashboard (bugs discovered vs prevented, economy metrics)
- FR-DSH-03: Component health visualization (degraded/improved)
- FR-DSH-04: Endpoint problem tracking and display
- FR-DSH-05: Period filters (30 days, 90 days, year)
- FR-DSH-06: Trend visualization with charts
- FR-DSH-07: Dashboard data refresh in real-time
- FR-DSH-08: Dashboard export for meetings (PDF, HTML, CSV)

**FR-PTN: Pattern Detection**
- FR-PTN-01: Automatic detection of time excess patterns (>50% over estimate)
- FR-PTN-02: Consecutive ticket problem detection (3+ tickets with same issue)
- FR-PTN-03: Proactive alert generation for anomalies
- FR-PTN-04: Alert with context: affected tickets, suggested actions
- FR-PTN-05: Pattern history tracking

**FR-SRC: Search & Discovery**
- FR-SRC-01: Contextual search triggered on ticket selection
- FR-SRC-02: Keyword-based search in Postman/Testmo (basic mode)
- FR-SRC-03: Search results display with source badges and direct links
- FR-SRC-04: Search result relevance indicators

**FR-AI: AI Companion (Optional - BYOK)**
- FR-AI-01: BYOK configuration (Anthropic, OpenAI, Deepseek, z.ai, Custom)
- FR-AI-02: API key storage and validation
- FR-AI-03: Semantic search across Postman/Testmo
- FR-AI-04: Gherkin-based test case suggestions from acceptance criteria
- FR-AI-05: Mini-chatbot for framework documentation assistance
- FR-AI-06: Contextual awareness (knows current screen/workflow)
- FR-AI-07: Graceful fallback when AI unavailable

**FR-CFG: Configuration & Setup**
- FR-CFG-01: Setup wizard with step-by-step guidance
- FR-CFG-02: User profile configuration (name, Jira username)
- FR-CFG-03: Ticket state filter configuration
- FR-CFG-04: Integration credentials input with validation
- FR-CFG-05: Setup progress persistence
- FR-CFG-06: Configuration YAML generation and validation
- FR-CFG-07: Setup completion with immediate value display (first tickets)

**FR-SUP: Support & Troubleshooting**
- FR-SUP-01: Support portal with active ticket dashboard
- FR-SUP-02: Automatic error log capture with context
- FR-SUP-03: Integration diagnostic tools (health checks, ping tests)
- FR-SUP-04: Known issues knowledge base
- FR-SUP-05: Troubleshooting suggestions based on error type

### Non-Functional Requirements

**NFR-PERF: Performance**
- NFR-PERF-01: API calls < 2s for 95% of requests
- NFR-PERF-02: Dashboard loading < 5s for historical data (30/90 days)
- NFR-PERF-03: Search operations < 3s for 90% of searches

**NFR-SEC: Security**
- NFR-SEC-01: Encrypted token storage using AES-256-GCM
- NFR-SEC-02: Secure logging without sensitive data exposure
- NFR-SEC-03: HTTPS/TLS 1.2+ for all external communications
- NFR-SEC-04: OAuth 2.0 with PKCE for Jira integration

**NFR-SCAL: Scalability**
- NFR-SCAL-01: Support 100 concurrent QAs without degradation
- NFR-SCAL-02: Modular architecture (one crate per integration)
- NFR-SCAL-03: YAML config validation for up to 10,000 lines
- NFR-SCAL-04: Plugin architecture for future integrations

**NFR-REL: Reliability**
- NFR-REL-01: Uptime > 99.5% for critical components
- NFR-REL-02: Health checks every 60 seconds with 2-minute alert threshold
- NFR-REL-03: Retry with exponential backoff (1s, 2s, 4s)
- NFR-REL-04: Log retention minimum 30 days with rotation/compression

**NFR-INT: Integration Standards**
- NFR-INT-01: Stable API contracts with 7-day advance notice for breaking changes
- NFR-INT-02: Automatic credential/endpoint validation on startup
- NFR-INT-03: Real-time latency/error monitoring with dashboard alerts

### Additional Requirements

**From Architecture:**
- AR-01: Custom Cargo workspace structure with 12 crates
- AR-02: Rust 1.80+ with Tokio async runtime
- AR-03: Axum 0.7+ web framework
- AR-04: Neon PostgreSQL (cloud) with SQLx 0.7
- AR-05: React 18+ frontend with Vite 5+ build
- AR-06: Zustand state management
- AR-07: Radix UI headless components
- AR-08: tracing + tracing-subscriber for logging
- AR-09: anyhow (internal) + thiserror (API boundaries) error handling
- AR-10: aes-gcm 0.10 + secrecy 0.8 for encryption
- AR-11: utoipa 5.0 for OpenAPI documentation
- AR-12: GitHub Actions CI/CD pipeline

**From UX Design:**
- UX-01: Hybrid Adaptive layout (Workflow mode + Dashboard mode)
- UX-02: Tailwind CSS v4 with CSS-first configuration (OKLCH colors)
- UX-03: Hostfully color palette (Primary blue #0ea5e9, Success green #28c76f)
- UX-04: Sidebar collapsible (64px icons / 240px expanded)
- UX-05: Desktop-first (Chrome priority), WCAG 2.1 Level AA compliance
- UX-06: Inter font (sans) + JetBrains Mono (code)
- UX-07: Skeleton loading with contextual messages
- UX-08: Toast notifications for feedback (success/error/warning/info)
- UX-09: Keyboard shortcuts (Ctrl+K search, Ctrl+Shift+M mode toggle)
- UX-10: Mini-chatbot UI with persistent icon

### FR Coverage Map

| Requirement | Epic | Description |
|-------------|------|-------------|
| FR-INT-01 | Epic 3 | Jira OAuth 2.0 + PKCE authentication |
| FR-INT-02 | Epic 3 | Jira ticket listing with filters |
| FR-INT-03 | Epic 3 | Jira ticket details retrieval |
| FR-INT-04 | Epic 3 | Jira ticket status updates |
| FR-INT-05 | Epic 4 | Postman API key authentication |
| FR-INT-06 | Epic 4 | Postman collection search |
| FR-INT-07 | Epic 4 | Postman test case retrieval |
| FR-INT-08 | Epic 4 | Testmo API key authentication |
| FR-INT-09 | Epic 4 | Testmo test case search |
| FR-INT-10 | Epic 4 | Testmo test run tracking |
| FR-INT-11 | Epic 11 | Splunk manual query interface |
| FR-INT-12 | Epic 11 | Splunk log retrieval |
| FR-INT-13 | Epic 3 | Integration health checks |
| FR-INT-14 | Epic 3 | Integration status dashboard |
| FR-INT-15 | Epic 3 | Credential validation on startup |
| FR-WRK-01 | Epic 5 | Workflow templates by ticket type |
| FR-WRK-02 | Epic 5 | Step-by-step workflow guidance |
| FR-WRK-03 | Epic 5 | Workflow step tracking |
| FR-WRK-04 | Epic 5 | Notes per workflow step |
| FR-WRK-05 | Epic 5 | Workflow pause/resume |
| FR-WRK-06 | Epic 5 | Workflow state persistence |
| FR-WRK-07 | Epic 5 | Workflow completion summary |
| FR-TRK-01 | Epic 6 | Auto timer start |
| FR-TRK-02 | Epic 6 | Timer pause/resume |
| FR-TRK-03 | Epic 6 | Real-time timer display |
| FR-TRK-04 | Epic 6 | Time per step tracking |
| FR-TRK-05 | Epic 6 | Total time calculation |
| FR-TRK-06 | Epic 6 | Time comparison with gap alerts |
| FR-TRK-07 | Epic 6 | Historical time data |
| FR-RPT-01 | Epic 7 | Auto report generation |
| FR-RPT-02 | Epic 7 | Report content (steps, time, tests) |
| FR-RPT-03 | Epic 7 | Report strategies and gap analysis |
| FR-RPT-04 | Epic 7 | Report export (Markdown/HTML) |
| FR-RPT-05 | Epic 7 | Report preview |
| FR-RPT-06 | Epic 7 | Historical report storage |
| FR-DSH-01 | Epic 8 | QA individual dashboard |
| FR-DSH-02 | Epic 10 | PM dashboard |
| FR-DSH-03 | Epic 10 | Component health visualization |
| FR-DSH-04 | Epic 10 | Endpoint problem tracking |
| FR-DSH-05 | Epic 8 | Period filters |
| FR-DSH-06 | Epic 8 | Trend visualization |
| FR-DSH-07 | Epic 8 | Real-time refresh |
| FR-DSH-08 | Epic 10 | Dashboard export |
| FR-PTN-01 | Epic 9 | Time excess detection |
| FR-PTN-02 | Epic 9 | Consecutive problem detection |
| FR-PTN-03 | Epic 9 | Proactive alert generation |
| FR-PTN-04 | Epic 9 | Alert with context |
| FR-PTN-05 | Epic 9 | Pattern history |
| FR-SRC-01 | Epic 4 | Contextual search on ticket selection |
| FR-SRC-02 | Epic 4 | Keyword-based search |
| FR-SRC-03 | Epic 4 | Search results with badges |
| FR-SRC-04 | Epic 4 | Search relevance indicators |
| FR-AI-01 | Epic 13 | BYOK configuration |
| FR-AI-02 | Epic 13 | AI API key storage |
| FR-AI-03 | Epic 13 | Semantic search |
| FR-AI-04 | Epic 13 | Gherkin test suggestions |
| FR-AI-05 | Epic 13 | Mini-chatbot |
| FR-AI-06 | Epic 13 | Contextual awareness |
| FR-AI-07 | Epic 13 | Graceful fallback |
| FR-CFG-01 | Epic 2 | Setup wizard |
| FR-CFG-02 | Epic 2 | User profile config |
| FR-CFG-03 | Epic 2 | Ticket filter config |
| FR-CFG-04 | Epic 2 | Credential input with validation |
| FR-CFG-05 | Epic 2 | Setup persistence |
| FR-CFG-06 | Epic 2 | YAML config generation |
| FR-CFG-07 | Epic 2 | Setup completion value display |
| FR-SUP-01 | Epic 12 | Support portal |
| FR-SUP-02 | Epic 12 | Error log capture |
| FR-SUP-03 | Epic 12 | Diagnostic tools |
| FR-SUP-04 | Epic 12 | Knowledge base |
| FR-SUP-05 | Epic 12 | Troubleshooting suggestions |

## Epic List

### Epic 1: Project Foundation & Core Infrastructure
Development team has a working Rust/React foundation with database connectivity, ready to build features.

**FRs covered:** AR-01 through AR-12 (Architecture setup)
**NFRs addressed:** NFR-SCAL-02 (modular crates), NFR-SEC-01 (encryption foundation)

---

### Epic 2: User Configuration & Setup Wizard
QAs and DevOps can configure the framework with their credentials, and the system validates all connections - ready for daily use.

**FRs covered:** FR-CFG-01, FR-CFG-02, FR-CFG-03, FR-CFG-04, FR-CFG-05, FR-CFG-06, FR-CFG-07
**NFRs addressed:** NFR-INT-02 (startup validation), NFR-SEC-01 (encrypted storage)
**UX addressed:** UX-01, UX-02, UX-03, UX-04, UX-06, UX-07, UX-08

---

### Epic 3: Jira Integration & Ticket Management
Ana (QA) can see her Jira tickets directly in the framework, select tickets to work on, and view full ticket details without leaving the app.

**FRs covered:** FR-INT-01, FR-INT-02, FR-INT-03, FR-INT-04, FR-INT-13, FR-INT-14, FR-INT-15
**NFRs addressed:** NFR-SEC-04 (OAuth 2.0 + PKCE), NFR-PERF-01 (<2s API calls), NFR-REL-03 (retry)
**UX addressed:** UX-05, UX-09

---

### Epic 4: Postman & Testmo Search Integration
When Ana selects a ticket, the framework automatically searches for related tests in Postman and Testmo, showing direct links to relevant test cases.

**FRs covered:** FR-INT-05, FR-INT-06, FR-INT-07, FR-INT-08, FR-INT-09, FR-INT-10, FR-SRC-01, FR-SRC-02, FR-SRC-03, FR-SRC-04
**NFRs addressed:** NFR-PERF-03 (<3s search), NFR-REL-03 (retry)

---

### Epic 5: Workflow Engine & Guided Testing
Ana can follow step-by-step guided workflows based on ticket type, with automatic progress tracking, notes per step, and workflow state that persists even if she closes the browser.

**FRs covered:** FR-WRK-01, FR-WRK-02, FR-WRK-03, FR-WRK-04, FR-WRK-05, FR-WRK-06, FR-WRK-07
**NFRs addressed:** NFR-REL-01 (99.5% uptime), NFR-PERF-01 (<2s responses)
**UX addressed:** UX-01, UX-07

---

### Epic 6: Time Tracking & Estimation
Ana has automatic time tracking that starts when she begins a workflow, with real-time display, and sees her actual time vs estimates - enabling her to prove her capacity with data.

**FRs covered:** FR-TRK-01, FR-TRK-02, FR-TRK-03, FR-TRK-04, FR-TRK-05, FR-TRK-06, FR-TRK-07
**NFRs addressed:** NFR-PERF-01 (<2s for tracking operations)
**UX addressed:** UX-01

---

### Epic 7: Reporting & Documentation
When Ana completes a workflow, a professional report is automatically generated showing steps completed, time breakdown, tests covered, and strategies used - exportable for sharing.

**FRs covered:** FR-RPT-01, FR-RPT-02, FR-RPT-03, FR-RPT-04, FR-RPT-05, FR-RPT-06
**NFRs addressed:** NFR-PERF-02 (<5s for report generation)

---

### Epic 8: QA Individual Dashboard
Ana sees her personal performance dashboard with tickets completed, time metrics, identified gaps, and insights to improve her testing strategy.

**FRs covered:** FR-DSH-01, FR-DSH-05, FR-DSH-06, FR-DSH-07
**NFRs addressed:** NFR-PERF-02 (<5s dashboard load)
**UX addressed:** UX-01, UX-04

---

### Epic 9: Pattern Detection & Proactive Alerts
The framework automatically detects anomalies (excessive time, consecutive problems) and alerts Ana/Carlos proactively with actionable context.

**FRs covered:** FR-PTN-01, FR-PTN-02, FR-PTN-03, FR-PTN-04, FR-PTN-05
**NFRs addressed:** NFR-REL-02 (health checks), NFR-INT-03 (monitoring)

---

### Epic 10: PM/PO Observability Dashboard
Carlos (PM) sees a consolidated dashboard with bugs discovered vs prevented, economy metrics, component health, and can export reports for stakeholder meetings.

**FRs covered:** FR-DSH-02, FR-DSH-03, FR-DSH-04, FR-DSH-08
**NFRs addressed:** NFR-PERF-02 (<5s dashboard load)
**UX addressed:** UX-01

---

### Epic 11: Splunk Log Integration
Ana can query Splunk logs directly from the framework using pre-built templates, viewing production data without switching to Splunk UI.

**FRs covered:** FR-INT-11, FR-INT-12
**NFRs addressed:** NFR-PERF-01 (<2s for queries)

---

### Epic 12: Support Portal & Troubleshooting
Sofia (Support) can quickly diagnose and resolve QA issues using automatic error logs, diagnostic tools, and a knowledge base of common solutions.

**FRs covered:** FR-SUP-01, FR-SUP-02, FR-SUP-03, FR-SUP-04, FR-SUP-05
**NFRs addressed:** NFR-REL-04 (30-day log retention)

---

### Epic 13: AI Companion (Optional - BYOK)
QAs who provide their own AI API key get semantic search, Gherkin-based test suggestions, and a helpful mini-chatbot for framework assistance.

**FRs covered:** FR-AI-01, FR-AI-02, FR-AI-03, FR-AI-04, FR-AI-05, FR-AI-06, FR-AI-07
**UX addressed:** UX-10

---

## Epic 1: Project Foundation & Core Infrastructure

Development team has a working Rust/React foundation with database connectivity, ready to build features.

### Story 1.1: Initialize Cargo Workspace Structure

As a developer,
I want a properly configured Cargo workspace with all crate scaffolds,
So that I can start implementing features in a modular, organized codebase.

**Acceptance Criteria:**

**Given** a fresh repository
**When** I run `cargo build` in the workspace root
**Then** all 12 crates compile successfully (even if empty):
- `qa-pms-core` (shared types, traits)
- `qa-pms-config` (configuration management)
- `qa-pms-api` (Axum web server)
- `qa-pms-workflow` (workflow engine)
- `qa-pms-tracking` (time tracking)
- `qa-pms-dashboard` (dashboard logic)
- `qa-pms-jira` (Jira integration)
- `qa-pms-postman` (Postman integration)
- `qa-pms-testmo` (Testmo integration)
- `qa-pms-splunk` (Splunk integration)
- `qa-pms-ai` (AI companion, feature-gated)
**And** workspace `Cargo.toml` defines shared dependencies
**And** each crate has proper `Cargo.toml` with appropriate dependencies
**And** `rustfmt.toml` and `clippy.toml` are configured per architecture patterns

---

### Story 1.2: Setup Core Types and Error Handling

As a developer,
I want shared types and error handling patterns in `qa-pms-core`,
So that all crates use consistent types and error handling.

**Acceptance Criteria:**

**Given** the workspace from Story 1.1
**When** I implement `qa-pms-core`
**Then** the crate exports:
- Common types: `UserId`, `WorkflowId`, `TicketId` (UUID-based)
- Error types using `thiserror` for API boundaries
- Result type alias using `anyhow` for internal operations
- Shared traits for integrations (e.g., `Integration` trait)
**And** `#[serde(rename_all = "camelCase")]` is applied to all API types
**And** `tracing` macros are used (never `println!`)
**And** all public items have doc comments

---

### Story 1.3: Setup Configuration Management with Encryption

As a developer,
I want a configuration system that loads YAML and encrypts secrets,
So that API tokens are stored securely.

**Acceptance Criteria:**

**Given** `qa-pms-config` crate
**When** I implement configuration management
**Then** the crate:
- Parses YAML configuration files using `serde_yaml`
- Validates configuration structure on load
- Encrypts sensitive fields using `aes-gcm` (AES-256-GCM)
- Wraps secrets in `secrecy::Secret<T>` to prevent accidental logging
- Supports environment variable overrides via `dotenv`
**And** configuration validation fails gracefully with clear error messages
**And** encrypted tokens can be decrypted at runtime for API calls
**And** unit tests verify encryption/decryption round-trip

---

### Story 1.4: Setup Axum API Server with Health Endpoint

As a developer,
I want a running Axum web server with a health check endpoint,
So that I can verify the backend is operational.

**Acceptance Criteria:**

**Given** `qa-pms-api` crate
**When** I run `cargo run -p qa-pms-api`
**Then** an Axum server starts on configurable port (default 3000)
**And** `GET /api/v1/health` returns:
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "timestamp": "2026-01-03T10:00:00Z"
}
```
**And** `tower-http` middleware is configured:
- CORS (configurable origins)
- Request tracing via `TraceLayer`
- Compression
**And** graceful shutdown on SIGTERM/SIGINT
**And** structured logging via `tracing-subscriber`

---

### Story 1.5: Setup Neon PostgreSQL Database Connection

As a developer,
I want database connectivity to Neon PostgreSQL,
So that I can persist application data.

**Acceptance Criteria:**

**Given** Neon PostgreSQL database provisioned
**When** the API server starts
**Then** SQLx connection pool is established with:
- Max 10 connections
- Min 2 connections
- 30s acquire timeout
- 600s idle timeout
**And** `DATABASE_URL` is read from environment/config
**And** connection failure logs error and prevents server start
**And** health endpoint includes database status:
```json
{
  "status": "healthy",
  "database": "connected"
}
```

---

### Story 1.6: Setup Database Migrations Infrastructure

As a developer,
I want SQLx migrations to run automatically,
So that database schema stays in sync with code.

**Acceptance Criteria:**

**Given** `migrations/` directory in project root
**When** the API server starts
**Then** pending migrations run automatically via `sqlx::migrate!()`
**And** migration files follow naming: `YYYYMMDDHHMMSS_description.sql`
**And** initial migration creates `schema_version` tracking table
**And** failed migrations prevent server start with clear error

---

### Story 1.7: Setup React Frontend with Vite and Tailwind CSS v4

As a developer,
I want a React frontend scaffold with Tailwind CSS v4,
So that I can build the UI following the design system.

**Acceptance Criteria:**

**Given** `frontend/` directory
**When** I run `npm run dev`
**Then** Vite dev server starts with HMR
**And** React 18+ with TypeScript is configured
**And** Tailwind CSS v4 uses CSS-first configuration in `index.css`:
- OKLCH color tokens for primary, success, warning, error, neutral
- Hostfully palette (primary #0ea5e9, success #28c76f)
- Inter font (sans) and JetBrains Mono (mono)
**And** `npm run build` produces optimized production build
**And** ESLint and Prettier are configured

---

### Story 1.8: Setup Zustand Store Structure

As a developer,
I want Zustand stores scaffolded for state management,
So that frontend state is organized by domain.

**Acceptance Criteria:**

**Given** React frontend from Story 1.7
**When** I implement store structure
**Then** `frontend/src/stores/` contains:
- `authStore.ts` - authentication state
- `workflowStore.ts` - workflow state
- `dashboardStore.ts` - dashboard data
- `integrationStore.ts` - integration status
- `index.ts` - barrel export
**And** each store follows the Zustand pattern from architecture doc
**And** stores are typed with TypeScript interfaces
**And** stores include actions alongside state

---

### Story 1.9: Setup GitHub Actions CI Pipeline

As a developer,
I want CI/CD pipeline that validates code quality,
So that every PR is automatically checked.

**Acceptance Criteria:**

**Given** `.github/workflows/` directory
**When** a PR is opened or pushed
**Then** CI pipeline runs:
- `cargo fmt --check` - formatting
- `cargo clippy -- -D warnings` - linting (zero warnings)
- `cargo test` - all tests pass
- `cargo build --release` - builds successfully
**And** frontend checks run:
- `npm run lint` - ESLint passes
- `npm run typecheck` - TypeScript compiles
- `npm run build` - production build succeeds
**And** pipeline fails if any check fails
**And** pipeline completes in < 10 minutes

---

### Story 1.10: Setup OpenAPI Documentation with utoipa

As a developer,
I want auto-generated OpenAPI documentation,
So that API consumers have accurate, up-to-date docs.

**Acceptance Criteria:**

**Given** `qa-pms-api` with utoipa configured
**When** I access `GET /api/v1/docs`
**Then** Swagger UI displays interactive API documentation
**And** health endpoint is documented with request/response schemas
**And** OpenAPI spec is generated at compile time (not runtime)
**And** JSON schema available at `GET /api/v1/openapi.json`

---

## Epic 2: User Configuration & Setup Wizard

QAs and DevOps can configure the framework with their credentials, and the system validates all connections - ready for daily use.

### Story 2.1: Setup Wizard UI Shell and Navigation

As a new user,
I want a guided setup wizard with clear progress,
So that I know exactly where I am in the configuration process.

**Acceptance Criteria:**

**Given** a user opens the app for the first time (no config exists)
**When** the app loads
**Then** the setup wizard displays with:
- Progress indicator showing steps (1 of 5, 2 of 5, etc.)
- Clear step titles and descriptions
- Back/Next navigation buttons
- Skip option for optional steps (with warning)
**And** wizard state persists in localStorage (FR-CFG-05)
**And** user can resume wizard if they close browser mid-setup
**And** wizard uses Radix UI components with Tailwind styling

---

### Story 2.2: User Profile Configuration Step

As a QA user,
I want to configure my profile information,
So that the framework can find my Jira tickets.

**Acceptance Criteria:**

**Given** user is on Step 1 of setup wizard
**When** user enters profile information
**Then** the form captures:
- Display name (required)
- Jira username/email (required, for ticket filtering)
- Preferred ticket states to show (multi-select: backlog, ready for QA, QA in progress, UAT)
**And** form validates required fields before allowing Next
**And** profile data is stored in configuration (FR-CFG-02, FR-CFG-03)
**And** skeleton loading shows while saving

---

### Story 2.3: Jira Integration Credentials Step

As a user setting up the framework,
I want to enter my Jira credentials,
So that the framework can access my Jira instance.

**Acceptance Criteria:**

**Given** user is on Jira configuration step
**When** user enters Jira credentials
**Then** the form captures:
- Jira instance URL (e.g., https://company.atlassian.net)
- OAuth Client ID
- OAuth Client Secret
**And** "Test Connection" button validates credentials against Jira API
**And** success shows green checkmark with "Connected to Jira"
**And** failure shows red error with specific message (e.g., "Invalid credentials", "Cannot reach server")
**And** credentials are encrypted before storage (NFR-SEC-01)
**And** retry available on failure

---

### Story 2.4: Postman Integration Credentials Step

As a user setting up the framework,
I want to enter my Postman API key,
So that the framework can search my Postman collections.

**Acceptance Criteria:**

**Given** user is on Postman configuration step
**When** user enters Postman API key
**Then** the form captures:
- Postman API Key (required)
- Workspace ID (optional, for filtering)
**And** "Test Connection" validates key against Postman API
**And** success shows workspaces accessible count
**And** failure shows clear error message
**And** API key is encrypted before storage (NFR-SEC-01)

---

### Story 2.5: Testmo Integration Credentials Step

As a user setting up the framework,
I want to enter my Testmo API credentials,
So that the framework can search test cases.

**Acceptance Criteria:**

**Given** user is on Testmo configuration step
**When** user enters Testmo credentials
**Then** the form captures:
- Testmo instance URL
- API Key
**And** "Test Connection" validates credentials
**And** success shows project count accessible
**And** failure shows clear error message
**And** credentials are encrypted before storage

---

### Story 2.6: Splunk Configuration Step (Manual)

As a user setting up the framework,
I want guidance on Splunk setup,
So that I can query logs manually.

**Acceptance Criteria:**

**Given** user is on Splunk configuration step
**When** user views the step
**Then** the form displays:
- Explanation that Splunk Cloud direct integration is not supported
- Instructions for manual query approach
- Text field for saving frequently-used query templates
- Optional: Splunk instance URL for reference links
**And** step can be skipped (Splunk is optional for MVP)
**And** saved templates are stored in configuration

---

### Story 2.7: Configuration YAML Generation and Validation

As a user completing setup,
I want my configuration saved and validated,
So that I can start using the framework immediately.

**Acceptance Criteria:**

**Given** user completes all setup wizard steps
**When** user clicks "Complete Setup"
**Then** the system:
- Generates validated YAML configuration file
- Validates all required fields are present
- Verifies encrypted secrets can be decrypted
- Runs startup validation for all integrations (NFR-INT-02)
**And** configuration file follows schema from architecture doc
**And** validation errors are displayed clearly with fix suggestions
**And** success redirects to main application with first tickets loaded (FR-CFG-07)

---

### Story 2.8: Setup Wizard Backend API Endpoints

As a developer,
I want API endpoints for the setup wizard,
So that the frontend can save and validate configuration.

**Acceptance Criteria:**

**Given** `qa-pms-api` crate
**When** setup wizard endpoints are implemented
**Then** the following endpoints exist:
- `POST /api/v1/setup/profile` - save user profile
- `POST /api/v1/setup/integrations/jira/test` - test Jira connection
- `POST /api/v1/setup/integrations/postman/test` - test Postman connection
- `POST /api/v1/setup/integrations/testmo/test` - test Testmo connection
- `POST /api/v1/setup/complete` - finalize and validate full config
- `GET /api/v1/setup/status` - check if setup is complete
**And** all endpoints return standardized error responses
**And** endpoints are documented in OpenAPI spec

---

### Story 2.9: Hybrid Adaptive Layout Foundation

As a user,
I want a responsive layout that adapts to my context,
So that I have the right interface for my current task.

**Acceptance Criteria:**

**Given** user has completed setup
**When** user enters the main application
**Then** the layout includes:
- Collapsible sidebar (64px collapsed / 240px expanded)
- Header with context (current ticket/dashboard)
- Main content area that adapts to available space
**And** sidebar collapse state persists in localStorage
**And** layout uses Tailwind CSS v4 tokens from design system
**And** keyboard shortcut `Ctrl+Shift+M` toggles sidebar
**And** smooth animation (300ms ease-in-out) on sidebar toggle

---

## Epic 3: Jira Integration & Ticket Management

Ana (QA) can see her Jira tickets directly in the framework, select tickets to work on, and view full ticket details without leaving the app.

### Story 3.1: Jira OAuth 2.0 + PKCE Authentication Flow

As a user,
I want secure OAuth authentication with Jira,
So that my credentials are protected using industry standards.

**Acceptance Criteria:**

**Given** Jira credentials configured in setup
**When** the application needs to authenticate with Jira
**Then** OAuth 2.0 + PKCE flow is used:
- Generate code verifier and code challenge
- Redirect to Jira authorization endpoint
- Handle callback with authorization code
- Exchange code for access token
- Store tokens securely (encrypted)
**And** token refresh happens automatically before expiry
**And** failed auth shows clear error with re-auth option
**And** flow complies with NFR-SEC-04

---

### Story 3.2: Jira Ticket Listing with Filters

As a QA (Ana),
I want to see my Jira tickets filtered by status,
So that I can quickly find tickets ready for testing.

**Acceptance Criteria:**

**Given** user is authenticated with Jira
**When** user views the ticket list
**Then** tickets are displayed showing:
- Ticket key (e.g., JIRA-1234)
- Title
- Priority (with color indicator)
- Status
- Assignee
**And** tickets are filtered by configured states (from setup)
**And** tickets are filtered by user's Jira username
**And** list loads in < 2s (NFR-PERF-01)
**And** loading state shows skeleton cards
**And** empty state shows helpful message

---

### Story 3.3: Jira Ticket Detail View

As a QA (Ana),
I want to see full ticket details,
So that I understand what needs to be tested.

**Acceptance Criteria:**

**Given** user clicks on a ticket in the list
**When** the ticket detail loads
**Then** the view displays:
- Full ticket key and title
- Description (rendered markdown)
- Acceptance criteria (highlighted if in Gherkin format)
- Priority and status
- Assignee and reporter
- Created/updated dates
- Comments (latest 10)
- Attachments list
**And** detail view loads in < 2s
**And** "Start Workflow" button is prominently displayed
**And** back navigation returns to list

---

### Story 3.4: Jira Ticket Status Updates

As a QA (Ana),
I want to update ticket status from the framework,
So that I don't need to switch to Jira.

**Acceptance Criteria:**

**Given** user is viewing a ticket
**When** user changes the status (e.g., "QA In Progress" → "QA Complete")
**Then** status is updated in Jira via API
**And** local display updates immediately (optimistic update)
**And** success toast confirms "Status updated"
**And** failure shows error with retry option
**And** API call uses retry with exponential backoff (NFR-REL-03)

---

### Story 3.5: Integration Health Check System

As a system administrator,
I want automatic health checks for integrations,
So that I'm alerted when something is wrong.

**Acceptance Criteria:**

**Given** the application is running
**When** 60 seconds have elapsed since last check
**Then** health checks run for all configured integrations:
- Jira: ping API endpoint
- Postman: validate API key
- Testmo: validate API key
- Record response time and status
**And** health status is stored in memory
**And** if integration is down for > 2 minutes, alert is logged (NFR-REL-02)
**And** health check runs in background (doesn't block UI)

---

### Story 3.6: Integration Status Dashboard Component

As a user,
I want to see integration status at a glance,
So that I know if all my tools are connected.

**Acceptance Criteria:**

**Given** health checks are running (Story 3.5)
**When** user views the integration status (in settings or sidebar)
**Then** status displays for each integration:
- Status indicator: ✅ Online / 🟡 Degraded / 🔴 Offline
- Last successful check timestamp
- Response time (ms)
- Error message if offline
**And** clicking an integration shows detailed status
**And** "Refresh" button triggers immediate health check
**And** status updates in real-time when checks complete

---

### Story 3.7: Credential Validation on Startup

As a user,
I want credentials validated when the app starts,
So that I know immediately if there's a configuration problem.

**Acceptance Criteria:**

**Given** user opens the application
**When** the app initializes
**Then** all configured integration credentials are validated:
- Decrypt stored credentials
- Test each integration endpoint
- Record validation results
**And** if critical integration (Jira) fails, show blocking error with fix link
**And** if optional integration fails, show warning but allow continue
**And** validation runs in parallel for speed (< 5s total)
**And** validation results are shown in toast notifications

---

## Epic 4: Postman & Testmo Search Integration

When Ana selects a ticket, the framework automatically searches for related tests in Postman and Testmo, showing direct links to relevant test cases.

### Story 4.1: Postman API Client Implementation

As a developer,
I want a Postman API client in the backend,
So that I can search and retrieve test collections.

**Acceptance Criteria:**

**Given** `qa-pms-postman` crate
**When** Postman client is implemented
**Then** the client supports:
- Authentication with API key
- List workspaces
- List collections in workspace
- Search collections by name/keyword
- Get collection details with requests
**And** all API calls use retry with exponential backoff
**And** responses are typed with serde structs
**And** errors are mapped to domain error types

---

### Story 4.2: Testmo API Client Implementation

As a developer,
I want a Testmo API client in the backend,
So that I can search and retrieve test cases.

**Acceptance Criteria:**

**Given** `qa-pms-testmo` crate
**When** Testmo client is implemented
**Then** the client supports:
- Authentication with API key
- List projects
- List test suites
- Search test cases by keyword
- Get test case details
**And** all API calls use retry with exponential backoff
**And** responses are typed with serde structs
**And** errors are mapped to domain error types

---

### Story 4.3: Contextual Search on Ticket Selection

As a QA (Ana),
I want automatic search when I select a ticket,
So that related tests appear without manual searching.

**Acceptance Criteria:**

**Given** user selects a Jira ticket
**When** the ticket detail view loads
**Then** the framework automatically:
- Extracts keywords from ticket title and description
- Searches Postman collections for matches
- Searches Testmo test cases for matches
- Displays results in "Related Tests" section
**And** search runs in parallel across both systems
**And** search completes in < 3s (NFR-PERF-03)
**And** loading state shows "Searching Postman... Searching Testmo..."

---

### Story 4.4: Search Results Display with Source Badges

As a QA (Ana),
I want search results clearly labeled by source,
So that I know where each test lives.

**Acceptance Criteria:**

**Given** contextual search has completed
**When** results are displayed
**Then** each result shows:
- Source badge (Postman / Testmo)
- Test name/title
- Brief description (truncated)
- Direct link to open in source system
- Match relevance indicator (if available)
**And** results are grouped by source
**And** clicking a result opens in new tab
**And** empty results show "No tests found" with search tips

---

### Story 4.5: Testmo Test Run Creation

As a QA (Ana),
I want to create a Testmo test run from the framework,
So that I can track my test execution.

**Acceptance Criteria:**

**Given** user has found relevant Testmo test cases
**When** user clicks "Create Test Run"
**Then** the framework:
- Creates a new test run in Testmo
- Links selected test cases to the run
- Associates with current Jira ticket (in run name)
- Returns link to the created run
**And** success toast shows with link to Testmo
**And** run name follows pattern: "QA-{ticket-key}-{date}"
**And** failure shows error with retry option

---

### Story 4.6: Search API Endpoints

As a developer,
I want search endpoints in the API,
So that the frontend can trigger and display searches.

**Acceptance Criteria:**

**Given** `qa-pms-api` crate
**When** search endpoints are implemented
**Then** the following endpoints exist:
- `POST /api/v1/search/postman` - search Postman collections
- `POST /api/v1/search/testmo` - search Testmo test cases
- `POST /api/v1/search/all` - search both in parallel
- `POST /api/v1/testmo/runs` - create test run
**And** search accepts: `{ keywords: string[], ticketId?: string }`
**And** results include source, name, description, url
**And** endpoints are documented in OpenAPI spec

---

## Epic 5: Workflow Engine & Guided Testing

Ana can follow step-by-step guided workflows based on ticket type, with automatic progress tracking, notes per step, and workflow state that persists.

### Story 5.1: Workflow Templates Database Schema

As a developer,
I want database tables for workflow templates,
So that workflows can be stored and customized.

**Acceptance Criteria:**

**Given** SQLx migrations infrastructure
**When** workflow schema migration runs
**Then** the following tables are created:
- `workflow_templates` (id, name, ticket_type, steps_json, created_at, updated_at)
- `workflow_instances` (id, template_id, ticket_id, user_id, status, current_step, started_at, completed_at)
- `workflow_step_results` (id, instance_id, step_index, status, notes, started_at, completed_at)
**And** indexes exist for common queries
**And** foreign keys enforce referential integrity

---

### Story 5.2: Default Workflow Templates

As a QA (Ana),
I want pre-built workflow templates for common ticket types,
So that I can start testing with guidance immediately.

**Acceptance Criteria:**

**Given** workflow tables exist
**When** the application initializes (seeding)
**Then** default templates are created:
- **Bug Fix Template:** Reproduce → Investigate → Test Fix → Regression Check → Document
- **Feature Test Template:** Review Requirements → Exploratory Test → Happy Path → Edge Cases → Document
- **Regression Template:** Setup Environment → Run Test Suite → Analyze Failures → Report
**And** each template includes step descriptions and estimated times
**And** templates can be customized by users (future story)

---

### Story 5.3: Start Workflow from Ticket

As a QA (Ana),
I want to start a guided workflow from a ticket,
So that I can follow structured testing steps.

**Acceptance Criteria:**

**Given** user is viewing a Jira ticket
**When** user clicks "Start Workflow"
**Then** the system:
- Shows template selection (Bug Fix, Feature Test, Regression)
- Creates workflow instance linked to ticket
- Transitions UI to Workflow Mode (sidebar collapsed)
- Displays first step prominently
**And** workflow instance is persisted to database
**And** if workflow already exists for ticket, prompt to resume or restart
**And** UI shows ticket context (key, title) in header

---

### Story 5.4: Workflow Step Progress UI

As a QA (Ana),
I want to see my progress through workflow steps,
So that I know what's done and what's next.

**Acceptance Criteria:**

**Given** user is in an active workflow
**When** the workflow view renders
**Then** the UI displays:
- All steps in order (vertical stepper)
- Current step highlighted/expanded
- Completed steps with checkmark ✅
- Pending steps grayed out
- Step name and description
- Estimated time per step
**And** clicking a completed step expands to show notes
**And** progress bar shows overall completion %
**And** current step index is shown (e.g., "Step 3 of 7")

---

### Story 5.5: Complete Workflow Step with Notes

As a QA (Ana),
I want to mark steps complete and add notes,
So that I can document my testing process.

**Acceptance Criteria:**

**Given** user is on a workflow step
**When** user completes the step
**Then** they can:
- Click "Complete Step" button
- Optionally add notes (textarea, markdown supported)
- Optionally attach links (to bugs found, test results)
**And** step result is saved to database with timestamp
**And** UI advances to next step automatically
**And** completed step shows checkmark with completion time
**And** notes are displayed when step is expanded

---

### Story 5.6: Workflow Pause and Resume

As a QA (Ana),
I want to pause my workflow and resume later,
So that I can handle interruptions without losing progress.

**Acceptance Criteria:**

**Given** user is in an active workflow
**When** user clicks "Pause Workflow"
**Then** the system:
- Saves current state to database
- Shows "Workflow Paused" indicator
- Allows user to navigate away
**And** when user returns to the ticket, "Resume Workflow" button appears
**And** resuming restores exact state (current step, notes, time)
**And** paused time is tracked separately (doesn't count toward step time)

---

### Story 5.7: Workflow Completion and Summary

As a QA (Ana),
I want a summary when I complete all workflow steps,
So that I can review my testing session.

**Acceptance Criteria:**

**Given** user completes the final workflow step
**When** workflow is marked complete
**Then** the system displays summary:
- Total time spent (all steps)
- Time per step breakdown
- All notes compiled
- Links collected
- Tests covered (from search results)
**And** summary can be edited before finalizing
**And** "Generate Report" button is prominently shown
**And** workflow status is updated to "completed" in database

---

### Story 5.8: Workflow State Persistence

As a QA (Ana),
I want my workflow state to persist if I close the browser,
So that I never lose my progress.

**Acceptance Criteria:**

**Given** user has an active workflow
**When** user closes browser or navigates away
**Then** all workflow state is persisted:
- Current step index
- Step completion status
- Notes per step
- Time tracking data (see Epic 6)
**And** reopening app shows "You have an active workflow" prompt
**And** user can choose to resume or abandon
**And** abandoned workflows are marked as "cancelled" in database

---

## Epic 6: Time Tracking & Estimation

Ana has automatic time tracking that starts when she begins a workflow, with real-time display, and sees her actual time vs estimates.

### Story 6.1: Time Tracking Database Schema

As a developer,
I want database tables for time tracking,
So that time data is persisted accurately.

**Acceptance Criteria:**

**Given** SQLx migrations infrastructure
**When** time tracking schema migration runs
**Then** the following tables are created:
- `time_sessions` (id, workflow_instance_id, step_index, started_at, paused_at, resumed_at, ended_at, total_seconds, is_active)
- `time_estimates` (id, template_id, step_index, estimated_seconds)
**And** indexes exist for querying by workflow and user
**And** `total_seconds` is calculated excluding paused time

---

### Story 6.2: Automatic Timer Start on Workflow Begin

As a QA (Ana),
I want time tracking to start automatically,
So that I don't need to remember to start a timer.

**Acceptance Criteria:**

**Given** user starts a workflow (Story 5.3)
**When** the workflow begins
**Then** a time session is created automatically:
- `started_at` set to current timestamp
- `is_active` set to true
- Session linked to workflow instance and step
**And** no user action required to start timer
**And** timer session persists to database

---

### Story 6.3: Real-Time Timer Display

As a QA (Ana),
I want to see the timer counting in real-time,
So that I'm aware of time spent.

**Acceptance Criteria:**

**Given** user has an active workflow with timer running
**When** the UI renders
**Then** the header displays:
- Current step timer: "Step: 00:15:32"
- Total workflow timer: "Total: 01:23:45"
- Visual indicator that timer is running (pulsing dot)
**And** timer updates every second
**And** timer uses efficient React state (no unnecessary re-renders)
**And** timer display is always visible during workflow

---

### Story 6.4: Timer Pause and Resume Controls

As a QA (Ana),
I want to pause the timer during interruptions,
So that break time isn't counted.

**Acceptance Criteria:**

**Given** timer is running
**When** user clicks "Pause"
**Then** the system:
- Records `paused_at` timestamp
- Stops timer display from incrementing
- Shows "Paused" indicator
- Changes button to "Resume"
**And** when user clicks "Resume":
- Records `resumed_at` timestamp
- Timer continues from paused value
- Paused duration is excluded from total
**And** pause/resume events are logged in database

---

### Story 6.5: Time Per Step Tracking

As a QA (Ana),
I want time tracked per workflow step,
So that I can see where I spend the most time.

**Acceptance Criteria:**

**Given** user moves between workflow steps
**When** user completes a step
**Then** the system:
- Finalizes time session for current step
- Calculates total time (excluding pauses)
- Stores in `time_sessions` table
- Starts new session for next step
**And** step completion UI shows time spent on that step
**And** historical view shows time breakdown by step

---

### Story 6.6: Time Comparison with Gap Alerts

As a QA (Ana),
I want to see my time compared to estimates,
So that I can identify where I'm spending extra time.

**Acceptance Criteria:**

**Given** workflow has time estimates per step
**When** user completes a step
**Then** comparison is displayed:
- Actual: "45 min" | Estimated: "30 min"
- Gap indicator: 🟢 (≤100%), 🟡 (100-120%), 🔴 (>120%)
- If >20% over, alert: "This step took 50% longer than estimated"
**And** workflow summary shows total actual vs estimated
**And** gap alerts are dismissible but logged
**And** historical data updates user's personal estimates

---

### Story 6.7: Historical Time Data Storage

As a developer,
I want historical time data aggregated,
So that dashboards can show trends.

**Acceptance Criteria:**

**Given** time sessions are recorded
**When** a workflow completes
**Then** aggregated data is stored:
- Total time by ticket type
- Average time per step
- User's historical averages
- Trends over time
**And** data is queryable by date range
**And** data is used for dashboard metrics (Epic 8)
**And** old sessions are retained per NFR-REL-04 (30 days minimum)

---

## Epic 7: Reporting & Documentation

When Ana completes a workflow, a professional report is automatically generated.

### Story 7.1: Report Generation on Workflow Completion

As a QA (Ana),
I want a report generated when I complete a workflow,
So that I have documentation of my testing.

**Acceptance Criteria:**

**Given** user completes all workflow steps
**When** user clicks "Generate Report"
**Then** the system creates a report containing:
- Ticket information (key, title, description)
- Workflow template used
- All steps with completion status
- Notes per step
- Total time and time per step
- Timestamp of completion
**And** report is stored in database
**And** report generation completes in < 5s (NFR-PERF-02)

---

### Story 7.2: Report Content: Tests Covered and Strategies

As a QA (Ana),
I want my report to include tests and strategies,
So that stakeholders see the testing scope.

**Acceptance Criteria:**

**Given** report is being generated
**When** content is compiled
**Then** report includes:
- Tests covered section (from search results)
  - Postman collections used
  - Testmo test cases linked
- Strategies used section
  - Test approach notes
  - Edge cases considered
- Gap analysis
  - Time actual vs estimated
  - Steps that took longer with reasons
**And** sections are optional if no data exists
**And** format is clean and professional

---

### Story 7.3: Report Preview Before Saving

As a QA (Ana),
I want to preview my report before finalizing,
So that I can make corrections.

**Acceptance Criteria:**

**Given** report has been generated
**When** preview is displayed
**Then** user sees:
- Full report rendered (read-only)
- Edit button to modify notes/sections
- Cancel button to go back
- Save & Export buttons
**And** edits update the report in real-time
**And** preview uses same styling as final export
**And** user can add additional notes at this stage

---

### Story 7.4: Report Export to Markdown and HTML

As a QA (Ana),
I want to export my report in different formats,
So that I can share it appropriately.

**Acceptance Criteria:**

**Given** user has a finalized report
**When** user clicks "Export"
**Then** options are shown:
- Markdown (.md) - for technical sharing
- HTML (.html) - for browser viewing
- Copy to clipboard - for pasting
**And** Markdown export produces valid, formatted markdown
**And** HTML export includes professional styling
**And** exported file is downloaded to user's computer
**And** filename follows pattern: `QA-{ticket-key}-report-{date}.{ext}`

---

### Story 7.5: Report History Storage and Retrieval

As a QA (Ana),
I want to access my past reports,
So that I can reference previous testing work.

**Acceptance Criteria:**

**Given** user has completed workflows with reports
**When** user views "Report History"
**Then** list displays:
- Report date and ticket key
- Workflow type
- Total time spent
- Quick preview on hover
**And** reports are sorted by date (newest first)
**And** clicking a report opens full view
**And** search/filter by ticket key or date range
**And** reports are retained per NFR-REL-04

---

## Epic 8: QA Individual Dashboard

Ana sees her personal performance dashboard with tickets completed, time metrics, and insights.

### Story 8.1: QA Dashboard Layout and Navigation

As a QA (Ana),
I want a personal dashboard view,
So that I can see my performance metrics.

**Acceptance Criteria:**

**Given** user has completed workflows
**When** user navigates to Dashboard
**Then** the layout shows:
- KPI cards at top (tickets completed, avg time, efficiency)
- Trend chart in middle
- Recent activity list at bottom
- Period selector (7 days, 30 days, 90 days, year)
**And** dashboard uses Dashboard mode (expanded sidebar)
**And** layout is responsive for single/dual monitor
**And** navigation via sidebar "Dashboard" item

---

### Story 8.2: Tickets Completed KPI Card

As a QA (Ana),
I want to see how many tickets I've completed,
So that I can track my productivity.

**Acceptance Criteria:**

**Given** user views dashboard
**When** tickets completed card renders
**Then** it displays:
- Count of completed tickets in period
- Comparison to previous period (+/- %)
- Trend indicator (↑ green, ↓ red, → neutral)
- Breakdown by ticket type (hover for details)
**And** card loads in < 2s
**And** clicking card shows detailed list

---

### Story 8.3: Time Metrics KPI Cards

As a QA (Ana),
I want to see my time efficiency metrics,
So that I can prove my capacity.

**Acceptance Criteria:**

**Given** user views dashboard
**When** time metrics cards render
**Then** cards display:
- Average time per ticket
- Time actual vs estimated ratio (e.g., 0.92x)
- Total hours worked in period
- Efficiency trend over time
**And** ratio uses color coding: 🟢 ≤1.0, 🟡 1.0-1.2, 🔴 >1.2
**And** clicking reveals per-ticket breakdown

---

### Story 8.4: Trend Visualization Chart

As a QA (Ana),
I want to see trends over time,
So that I can identify patterns.

**Acceptance Criteria:**

**Given** user views dashboard
**When** trend chart renders
**Then** chart shows:
- X-axis: time (days/weeks based on period)
- Y-axis: tickets completed or hours
- Line graph with data points
- Hover shows exact values
**And** chart type can be toggled (tickets vs hours)
**And** chart uses design system colors
**And** chart is accessible (WCAG compliant)

---

### Story 8.5: Dashboard Period Filters

As a QA (Ana),
I want to filter my dashboard by time period,
So that I can analyze different timeframes.

**Acceptance Criteria:**

**Given** user views dashboard
**When** user selects a period filter
**Then** all dashboard components update:
- Last 7 days
- Last 30 days
- Last 90 days
- This year
- Custom date range
**And** filter selection persists in URL (shareable)
**And** data refreshes without full page reload
**And** loading state shows during data fetch

---

### Story 8.6: Dashboard Real-Time Refresh

As a QA (Ana),
I want my dashboard to update in real-time,
So that I see current data.

**Acceptance Criteria:**

**Given** user is viewing dashboard
**When** new data is available (workflow completed)
**Then** dashboard updates automatically:
- KPI cards refresh
- Chart updates
- Activity list updates
**And** refresh indicator shows when updating
**And** manual refresh button available
**And** auto-refresh interval: 60 seconds
**And** refresh is silent (no page flicker)

---

## Epic 9: Pattern Detection & Proactive Alerts

The framework automatically detects anomalies and alerts users proactively.

### Story 9.1: Time Excess Pattern Detection

As a system,
I want to detect when users consistently exceed time estimates,
So that I can alert about potential issues.

**Acceptance Criteria:**

**Given** workflow time data is being collected
**When** analysis runs (after each workflow completion)
**Then** system detects:
- Steps taking >50% longer than estimated
- Tickets taking >50% longer than similar tickets
- Trend of increasing time over last 5 tickets
**And** detection logic runs in background
**And** detected patterns are stored in database
**And** pattern includes: affected tickets, average excess %, suggested cause

---

### Story 9.2: Consecutive Problem Detection

As a system,
I want to detect consecutive tickets with same issue,
So that systemic problems are identified.

**Acceptance Criteria:**

**Given** workflow and ticket data
**When** analysis runs
**Then** system detects:
- 3+ consecutive tickets with same component affected
- 3+ consecutive tickets with similar notes/issues
- Spike in tickets for same area
**And** detection uses keyword matching on notes
**And** pattern includes: affected tickets, common factor
**And** confidence score based on match quality

---

### Story 9.3: Proactive Alert Generation

As a QA/PM,
I want to receive alerts about detected patterns,
So that I can take action before problems escalate.

**Acceptance Criteria:**

**Given** a pattern has been detected
**When** pattern meets alert threshold
**Then** alert is generated with:
- Alert type (time excess / consecutive problem / spike)
- Severity (info / warning / critical)
- Affected tickets list
- Suggested actions
- Timestamp
**And** alert is stored in database
**And** alert notification appears in UI (toast)
**And** alert badge shows on dashboard

---

### Story 9.4: Alert Display with Context

As a QA/PM,
I want to see alert details with full context,
So that I can investigate and act.

**Acceptance Criteria:**

**Given** alerts have been generated
**When** user views alerts (bell icon or alerts page)
**Then** each alert shows:
- Alert title and type
- Severity indicator
- Affected tickets (clickable links)
- Pattern description
- Suggested actions
- "Dismiss" and "Investigate" buttons
**And** alerts are sorted by severity, then date
**And** dismissed alerts are hidden but accessible
**And** unread count shows in header

---

### Story 9.5: Pattern History Tracking

As a PM (Carlos),
I want to see history of detected patterns,
So that I can identify recurring issues.

**Acceptance Criteria:**

**Given** patterns have been detected over time
**When** user views pattern history
**Then** history shows:
- List of all patterns detected
- Date range filter
- Pattern type filter
- Resolution status (addressed / ignored / recurring)
**And** clicking pattern shows full details
**And** can mark patterns as "resolved" with notes
**And** recurring patterns are highlighted

---

## Epic 10: PM/PO Observability Dashboard

Carlos (PM) sees a consolidated dashboard with bugs discovered vs prevented, economy metrics, component health.

### Story 10.1: PM Dashboard Layout

As a PM (Carlos),
I want a dashboard focused on product quality,
So that I can make data-driven decisions.

**Acceptance Criteria:**

**Given** user has PM role or access
**When** user navigates to PM Dashboard
**Then** layout shows:
- Large KPI cards (bugs discovered, bugs prevented, economy)
- Component health section
- Problematic endpoints section
- Export button prominently placed
**And** dashboard uses same design system as QA dashboard
**And** navigation via sidebar "PM Dashboard" item

---

### Story 10.2: Bugs Discovered vs Prevented Metrics

As a PM (Carlos),
I want to see bugs discovered vs prevented,
So that I can quantify QA value.

**Acceptance Criteria:**

**Given** bug data is tracked (from workflows/reports)
**When** metrics card renders
**Then** it displays:
- Bugs discovered count (from testing)
- Bugs prevented count (from proactive detection)
- Prevention rate: prevented / (discovered + prevented)
- Trend vs previous period
**And** definitions are shown on hover
**And** clicking shows detailed breakdown

---

### Story 10.3: Economy Metrics Calculation

As a PM (Carlos),
I want to see estimated savings,
So that I can demonstrate ROI.

**Acceptance Criteria:**

**Given** time and bug data exists
**When** economy card renders
**Then** it displays:
- Hours saved (time actual < estimated)
- Cost saved (hours × configurable hourly rate)
- Bug prevention value (bugs prevented × avg fix cost)
- Total economy estimate
**And** hourly rate and fix cost are configurable
**And** calculation formula is shown on hover
**And** can export economy report for stakeholders

---

### Story 10.4: Component Health Visualization

As a PM (Carlos),
I want to see which components are healthy vs degraded,
So that I can prioritize improvements.

**Acceptance Criteria:**

**Given** tickets and bugs are tagged by component
**When** component health section renders
**Then** it displays:
- List of components sorted by bug count
- Health indicator per component (🟢🟡🔴)
- Trend arrow (improving/degrading/stable)
- Bug count and ticket count
**And** clicking component shows detailed history
**And** can filter by time period
**And** components with no recent bugs show 🟢

---

### Story 10.5: Problematic Endpoints Display

As a PM (Carlos),
I want to see which endpoints have the most issues,
So that I can focus technical improvements.

**Acceptance Criteria:**

**Given** tickets reference specific endpoints
**When** endpoints section renders
**Then** it displays:
- Top 10 problematic endpoints
- Issue count per endpoint
- Common issue types
- Trend over time
**And** endpoint data is extracted from ticket descriptions/notes
**And** clicking endpoint shows related tickets
**And** can export endpoint report

---

### Story 10.6: Dashboard Export for Meetings

As a PM (Carlos),
I want to export dashboard data,
So that I can share in stakeholder meetings.

**Acceptance Criteria:**

**Given** user is viewing PM Dashboard
**When** user clicks "Export"
**Then** options are shown:
- PDF - formatted report with charts
- HTML - interactive version
- CSV - raw data for spreadsheets
**And** export includes current period's data
**And** export includes timestamp and filters applied
**And** filename: `QA-Metrics-{period}-{date}.{ext}`

---

## Epic 11: Splunk Log Integration

Ana can query Splunk logs directly from the framework using pre-built templates.

### Story 11.1: Splunk Query Interface

As a QA (Ana),
I want to query Splunk from the framework,
So that I can investigate production issues.

**Acceptance Criteria:**

**Given** Splunk is configured (manual setup)
**When** user opens Splunk panel
**Then** interface shows:
- Query input field (SPL syntax)
- Time range selector
- "Run Query" button
- Results area
**And** query templates are available as dropdown
**And** ticket context (key) can be auto-inserted in query
**And** instructions link to Splunk documentation

---

### Story 11.2: Splunk Query Templates

As a QA (Ana),
I want pre-built query templates,
So that I don't need to write SPL from scratch.

**Acceptance Criteria:**

**Given** user opens Splunk panel
**When** user views templates
**Then** available templates include:
- Error logs for date range
- Logs for specific user ID
- Logs containing ticket key
- Performance metrics
- Custom (user can save own)
**And** selecting template populates query field
**And** placeholders (e.g., {TICKET_KEY}) are auto-filled
**And** user can save custom templates

---

### Story 11.3: Splunk Log Display

As a QA (Ana),
I want to see Splunk query results,
So that I can analyze production behavior.

**Acceptance Criteria:**

**Given** user runs a Splunk query
**When** results return
**Then** display shows:
- Log entries in table format
- Timestamp, level, message columns
- Expandable row for full details
- Pagination for large results
- Export to CSV option
**And** loading state during query execution
**And** error handling for invalid queries
**And** results are not stored (privacy)

---

## Epic 12: Support Portal & Troubleshooting

Sofia (Support) can quickly diagnose and resolve QA issues.

### Story 12.1: Support Dashboard

As a support person (Sofia),
I want a dashboard of support requests,
So that I can see what needs attention.

**Acceptance Criteria:**

**Given** users have encountered errors
**When** support person views support dashboard
**Then** dashboard shows:
- List of recent errors/issues
- Error type and frequency
- Affected user
- Timestamp
- Status (new / investigating / resolved)
**And** sortable by severity, date, user
**And** search by error message
**And** accessible via admin sidebar item

---

### Story 12.2: Automatic Error Log Capture

As a system,
I want to capture errors with context automatically,
So that support can diagnose issues.

**Acceptance Criteria:**

**Given** an error occurs in the application
**When** error is caught
**Then** system logs:
- Error message and stack trace
- User ID and session info
- Current page/action
- Browser and device info
- Timestamp
**And** sensitive data is NOT logged (NFR-SEC-02)
**And** logs are retained 30 days (NFR-REL-04)
**And** logs are stored in database

---

### Story 12.3: Integration Diagnostic Tools

As a support person (Sofia),
I want to run diagnostics on integrations,
So that I can quickly identify connection issues.

**Acceptance Criteria:**

**Given** support person views an issue
**When** they click "Run Diagnostics"
**Then** system runs:
- Health check on all integrations
- Credential validation
- Latency measurement
- Recent error count per integration
**And** results display with pass/fail indicators
**And** suggestions for common fixes
**And** can be run for specific user's config

---

### Story 12.4: Knowledge Base for Common Issues

As a support person (Sofia),
I want a knowledge base of solutions,
So that I can resolve issues quickly.

**Acceptance Criteria:**

**Given** support person is investigating
**When** they access knowledge base
**Then** they see:
- Searchable list of common issues
- Each entry has: problem, cause, solution
- Related error messages
- Steps to resolve
**And** can add new entries
**And** can link issues to knowledge base entries
**And** most viewed entries shown first

---

### Story 12.5: Troubleshooting Suggestions

As a system,
I want to suggest solutions based on error type,
So that issues are resolved faster.

**Acceptance Criteria:**

**Given** an error is captured
**When** support views the error
**Then** system suggests:
- Matching knowledge base entries
- Similar past issues and resolutions
- Recommended diagnostic steps
**And** suggestions are ranked by relevance
**And** can mark suggestions as helpful/not helpful
**And** suggestion accuracy improves over time (feedback loop)

---

## Epic 13: AI Companion (Optional - BYOK)

QAs who provide their own AI API key get semantic search, Gherkin-based test suggestions, and a mini-chatbot.

### Story 13.1: AI Provider Configuration (BYOK)

As a user,
I want to configure my own AI provider,
So that I can use AI features with my API key.

**Acceptance Criteria:**

**Given** user opens AI settings
**When** user configures AI provider
**Then** options include:
- Provider selection (Anthropic, OpenAI, Deepseek, z.ai, Custom)
- API Key input (masked)
- Model selection (per provider)
- "Test Connection" button
**And** API key is encrypted before storage (NFR-SEC-01)
**And** test sends minimal request to validate key
**And** success/failure shown clearly
**And** can disable AI anytime (reverts to basic mode)

---

### Story 13.2: Semantic Search Enhancement

As a QA (Ana),
I want AI-powered semantic search,
So that I find related tests even with different wording.

**Acceptance Criteria:**

**Given** user has AI configured
**When** contextual search runs (ticket selection)
**Then** search is enhanced:
- AI analyzes ticket title, description, acceptance criteria
- Generates semantic query
- Searches Postman/Testmo with meaning-based matching
- Results ranked by semantic relevance
**And** results show relevance score (e.g., 85% match)
**And** falls back to keyword search if AI fails (FR-AI-07)
**And** search still completes in < 3s

---

### Story 13.3: Gherkin-Based Test Suggestions

As a QA (Ana),
I want AI to suggest tests from Gherkin acceptance criteria,
So that I can quickly create test cases.

**Acceptance Criteria:**

**Given** ticket has Gherkin-style acceptance criteria
**When** AI analyzes the ticket
**Then** suggestions include:
- Parsed Given/When/Then scenarios
- Suggested test steps for each scenario
- Edge cases to consider
- Potential negative test cases
**And** suggestions displayed in collapsible section
**And** user can accept, edit, or reject each suggestion
**And** accepted suggestions can be saved to notes

---

### Story 13.4: Mini-Chatbot UI

As a user,
I want a chatbot for framework questions,
So that I can get help without leaving the app.

**Acceptance Criteria:**

**Given** user has AI configured
**When** user opens chatbot (icon in corner)
**Then** chat interface shows:
- Message history (session-based)
- Text input with send button
- "Ask about this page" context button
- Close/minimize button
**And** chatbot icon is always visible (corner position)
**And** keyboard shortcut: `Ctrl+K` opens chat
**And** chat persists during session

---

### Story 13.5: Chatbot Contextual Awareness

As a user,
I want the chatbot to know where I am,
So that answers are relevant to my current context.

**Acceptance Criteria:**

**Given** chatbot is open
**When** user asks a question
**Then** AI receives context:
- Current page/view
- Current ticket (if viewing one)
- Current workflow step (if in workflow)
- User's recent actions
**And** responses reference current context
**And** can ask "What should I do next?" and get relevant answer
**And** context is not stored after session (privacy)

---

### Story 13.6: AI Graceful Fallback

As a user,
I want the app to work without AI,
So that I'm not blocked if AI is unavailable.

**Acceptance Criteria:**

**Given** AI is not configured or API fails
**When** AI features are accessed
**Then** fallback behavior:
- Semantic search → keyword search
- Test suggestions → "Enable AI for suggestions" message
- Chatbot → "AI not configured" with setup link
- No error blocking user workflow
**And** fallback is silent (no disruptive errors)
**And** features degrade gracefully
**And** user can always complete their work without AI

