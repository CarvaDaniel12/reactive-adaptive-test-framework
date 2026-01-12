# Code Review Pending List
**Generated:** 2026-01-11  
**Purpose:** Lista priorizada de stories que precisam de code review seguindo o método BMAD  
**Status Source:** `sprint-status.yaml`

---

## Priorização BMAD

Stories são priorizadas por:
1. **Priority** (P0 > P1 > P2)
2. **Epic Status** (in-progress > done > backlog)
3. **Dependencies** (foundation stories first)
4. **Story completion date** (recent first)

---

## 🔴 PRIORIDADE P0 - CRITICAL (Foundation Stories)

### Epic 22: PMS Integration Health Monitoring Module
**Status:** IN-PROGRESS  
**Priority:** P0 (Foundation for all other features)  
**Dependencies:** Epic 8 (QA Dashboard) - Complete ✅

#### Stories para Review (ordem de dependência):

1. **22-1: Integration Health Database Schema** ⭐ **FIRST**
   - Status: review
   - Story file: `22-1-integration-health-database-schema.md`
   - Completed: 2026-01-11
   - Dependencies: None
   - **Priority: CRITICAL** - Foundation story, all others depend on it
   - Evidence: Database migration with integration_health and integration_events tables

2. **22-2: Integration Health Types and Error Handling**
   - Status: review
   - Story file: `22-2-integration-health-types-and-error-handling.md`
   - Completed: 2026-01-11
   - Dependencies: Story 22.1
   - Evidence: Integration health types and error handling crate with comprehensive tests

3. **22-3: Integration Health Repository**
   - Status: review
   - Story file: `22-3-integration-health-repository.md`
   - Completed: 2026-01-11
   - Dependencies: Story 22.1, 22.2
   - Evidence: Repository module with CRUD operations for integration health data

4. **22-5: Integration Health API Endpoints**
   - Status: review
   - Story file: `22-5-integration-health-api-endpoints.md`
   - Completed: 2026-01-11
   - Dependencies: Story 22.4 (done)
   - Evidence: API endpoints for integration health (4 endpoints, OpenAPI documented)

5. **22-6: Integration Health Dashboard Widget**
   - Status: review
   - Story file: `22-6-integration-health-dashboard-widget.md`
   - Completed: 2026-01-11
   - Dependencies: Story 22.5
   - Evidence: Dashboard widget with integration health cards, React Query hook, and navigation

---

## 🟡 PRIORIDADE P1 - IN-PROGRESS EPICS

### Epic 31: AI Enhanced Automation
**Status:** IN-PROGRESS  
**Priority:** P1

6. **31-1: Auto Test Generation from Tickets**
   - Status: review
   - Story file: `31-1-auto-test-generation-from-tickets.md`
   - Started: 2026-01-10
   - **Note:** Partial completion (Task 3 completed)
   - Evidence: Test case data model in qa-pms-core, migration, repository

---

## 🟢 PRIORIDADE P2 - DONE EPICS (Needs Review)

### Epic 13: AI Companion
**Status:** DONE  
**Priority:** P1

7. **13-1: AI Provider Configuration BYOK**
   - Status: review
   - Story file: `13-1-ai-provider-configuration-byok.md`
   - Evidence: qa-pms-ai/src/provider.rs
   - **Note:** Epic 13 is marked DONE, but this story is still in review
   - **Warning:** Code review action items (sprint-status.yaml) mention issues with this story (CR-HIGH-005, CR-HIGH-006, CR-HIGH-007)

---

## 🔵 PRIORIDADE P3 - BACKLOG EPICS

### Epic 14: Rust Implementation Improvements
**Status:** BACKLOG  
**Priority:** P1

8. **14-1: Graceful Shutdown and Signal Handling**
   - Status: review
   - Story file: `14-1-graceful-shutdown-signal-handling.md`
   - Evidence: Implementation complete, 11 tests passing

9. **14-2: Request ID Middleware for Correlation**
   - Status: review
   - Story file: `14-2-request-id-middleware-for-correlation.md`
   - Evidence: Implementation complete, 10 tests passing (4 unit + 6 integration)

---

## Summary

**Total Stories Pending Review:** 9

- **P0 (Critical):** 5 stories (Epic 22)
- **P1 (In-Progress):** 1 story (Epic 31)
- **P1 (Done but review pending):** 1 story (Epic 13) ⚠️
- **P1 (Backlog):** 2 stories (Epic 14)

**Recommended Review Order:**
1. Epic 22 stories (22-1 → 22-2 → 22-3 → 22-5 → 22-6) - P0, foundation
2. Story 31-1 - P1, in-progress
3. Story 13-1 - P1, done but has known issues
4. Epic 14 stories (14-1, 14-2) - P1, backlog

---

## Execution Plan

Para cada story, seguir o workflow BMAD code-review:

1. Load story file
2. Build review attack plan (ACs, Tasks, File List)
3. Execute adversarial review (git changes vs claims)
4. Find 3-10 specific issues
5. Present findings (HIGH/MEDIUM/LOW)
6. Update story status in sprint-status.yaml

**Next Step:** Start with 22-1 (first foundation story)
