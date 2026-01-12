---
stepsCompleted: [1, 2]
workflowType: 'epics-and-stories'
project_name: 'QA Framework Improvements for PMS Integration Quality - Observability'
user_name: 'Daniel'
date: '2026-01-10'
inputDocuments:
  - _bmad-output/planning-artifacts/prd-observability-pms-integrations-2026-01-10.md
  - _bmad-output/planning-artifacts/architecture-observability-pms-integrations-2026-01-10.md
  - _bmad-output/planning-artifacts/design-thinking-observability-2026-01-10.md
lastStep: 2
---

# Epics & Stories: QA Framework Improvements for PMS Integration Quality - Observability

**Author:** Daniel  
**Date:** 2026-01-10  
**Version:** 1.0  
**Status:** Draft for Review

---

## Executive Summary

This document defines **3 new epics (22-24)** to add PMS-specific observability capabilities to the existing QA Intelligent PMS Framework. These epics enhance the framework to help QA Engineers monitor integration health, quantify revenue impact, and correlate test results with integration failures.

**Total Stories:** 14 stories across 3 epics  
**Estimated Effort:** ~25 days  
**Priority:** P0 (Epic 22, 23), P1 (Epic 24)

**Implementation Phases:**
- **Phase 1** (Epic 22): Foundation - Integration Health Monitoring (P0)
- **Phase 2** (Epic 23): Revenue Impact Calculator and Dashboard (P0)
- **Phase 3** (Epic 24): Test-Integration Correlation Engine (P1)

---

## Epic 22: PMS Integration Health Monitoring Module

**Priority:** 🔴 P0 (Foundation for all other features)  
**Focus:** Monitor integration health status for PMS integrations (Booking.com, Airbnb, Vrbo, HMBN)  
**Status:** `backlog`

### Problem Statement

QA Engineers working on PMS integration quality need to monitor integration health status in the framework, but the current framework shows generic QA metrics, not PMS-integration specific metrics. Without integration health monitoring, QAs can't effectively identify integration issues (pricing sync failures, fees sync errors, booking loss) proactively.

### User Story

**As a** QA Engineer working on PMS integration quality  
**I want** to see integration health status for Booking.com, Airbnb, Vrbo, HMBN in the framework dashboard  
**So that** I can identify integration issues proactively and prevent revenue loss

### Success Criteria

- ✅ QAs can see integration health status in dashboard
- ✅ Integration health API endpoint functional
- ✅ Integration detail page shows metrics and timeline
- ✅ Manual data collection supported (Phase 1)

### Dependencies

- ✅ Epic 8 (QA Dashboard) - Complete
- ✅ Database (PostgreSQL) - Available
- ⚠️ PMS API access (optional, Phase 4)

### Stories

#### Sprint 1: Database Schema and Core Types (3 days)

**Story 22.1: Integration Health Database Schema**
- **Priority:** P0
- **Estimated Days:** 2
- **Status:** `backlog`
- **Dependencies:** None

**Description:** Create database schema for integration health data storage

**Technical Requirements:**
1. Create `integration_health` table (current status, denormalized for fast reads)
2. Create `integration_events` table (historical events, normalized for query flexibility)
3. Add indexes: `integration_id`, `last_checked`/`occurred_at`, `event_type`
4. Create SQLx migration file
5. Support data retention: 90 days for events, current status always available

**Acceptance Criteria:**
- [ ] Given database schema, When migration runs, Then `integration_health` table is created
- [ ] Given database schema, When migration runs, Then `integration_events` table is created
- [ ] Given tables exist, When querying by `integration_id`, Then query performance < 100ms
- [ ] Given tables exist, When querying by `occurred_at`, Then query performance < 100ms
- [ ] Given events table, When inserting events, Then events are stored successfully

**Files to Create:**
- `migrations/YYYYMMDDHHMMSS_create_integration_health_tables.sql`
- (Migration follows SQLx migration patterns)

**Files to Modify:**
- None (new migration)

**Testing Strategy:**
- Unit tests: Migration SQL validation
- Integration tests: Migration runs successfully, tables created with correct schema
- Manual tests: Verify indexes created, test query performance

**Success Metrics:**
- Migration runs without errors
- Tables created with correct schema
- Indexes created and functional
- Query performance meets targets (< 100ms)

---

**Story 22.2: Integration Health Types and Error Handling**
- **Priority:** P0
- **Estimated Days:** 1
- **Status:** `backlog`
- **Dependencies:** Story 22.1

**Description:** Create Rust types and error handling for integration health module

**Technical Requirements:**
1. Create `qa-pms-integration-health` crate
2. Define types: `IntegrationId`, `HealthStatus`, `IntegrationHealth`, `IntegrationEvent`
3. Define error types: `IntegrationHealthError` (using `thiserror`)
4. Export types for use in API crate
5. Follow existing patterns (`qa-pms-dashboard`, `qa-pms-patterns`)

**Acceptance Criteria:**
- [ ] Given crate structure, When compiling, Then crate compiles without errors
- [ ] Given types defined, When serializing to JSON, Then types serialize correctly (camelCase)
- [ ] Given error types, When error occurs, Then error message is clear and actionable
- [ ] Given types exported, When importing in API crate, Then types are available

**Files to Create:**
- `crates/qa-pms-integration-health/Cargo.toml`
- `crates/qa-pms-integration-health/src/lib.rs`
- `crates/qa-pms-integration-health/src/types.rs`
- `crates/qa-pms-integration-health/src/error.rs`

**Files to Modify:**
- `Cargo.toml` (workspace) - Add new crate
- `crates/qa-pms-api/Cargo.toml` - Add dependency

**Testing Strategy:**
- Unit tests: Type serialization/deserialization
- Unit tests: Error type formatting
- Integration tests: Types work with API crate

**Success Metrics:**
- Crate compiles successfully
- Types serialize/deserialize correctly
- Error types provide clear messages

---

#### Sprint 2: Repository and Service Layer (3 days)

**Story 22.3: Integration Health Repository**
- **Priority:** P0
- **Estimated Days:** 2
- **Status:** `backlog`
- **Dependencies:** Story 22.1, Story 22.2

**Description:** Implement database repository for integration health operations

**Technical Requirements:**
1. Create `repository.rs` module
2. Implement functions: `get_latest_health`, `get_health_history`, `store_health_status`, `store_event`
3. Use SQLx for database operations
4. Follow existing patterns (`qa-pms-dashboard`, `qa-pms-patterns`)
5. Use `SqlxResultExt` trait for error mapping

**Acceptance Criteria:**
- [ ] Given repository, When querying latest health, Then returns latest status for integration
- [ ] Given repository, When querying health history, Then returns health history for period
- [ ] Given repository, When storing health status, Then status is stored successfully
- [ ] Given repository, When storing event, Then event is stored successfully
- [ ] Given repository, When querying by period, Then returns data for specified period only

**Files to Create:**
- `crates/qa-pms-integration-health/src/repository.rs`

**Files to Modify:**
- `crates/qa-pms-integration-health/src/lib.rs` - Export repository

**Testing Strategy:**
- Unit tests: Repository functions (with test database)
- Integration tests: Database operations work correctly
- Manual tests: Verify data persistence

**Success Metrics:**
- Repository functions work correctly
- Database operations performant (< 100ms)
- Error handling works correctly

---

**Story 22.4: Integration Health Service**
- **Priority:** P0
- **Estimated Days:** 1
- **Status:** `backlog`
- **Dependencies:** Story 22.3

**Description:** Implement service layer for integration health business logic

**Technical Requirements:**
1. Create `service.rs` module
2. Implement functions: `get_health_status`, `get_health_history`, `update_health_status`, `add_event`
3. Business logic: status calculation (healthy/warning/critical based on error rates)
4. Use repository for data access
5. Follow existing patterns (service layer abstraction)

**Acceptance Criteria:**
- [ ] Given service, When getting health status, Then returns health status for all integrations
- [ ] Given service, When getting health history, Then returns health history with trend calculation
- [ ] Given service, When updating health status, Then status is updated correctly
- [ ] Given service, When adding event, Then event is stored and health status recalculated
- [ ] Given service, When calculating status, Then status is calculated based on error rates (healthy/warning/critical)

**Files to Create:**
- `crates/qa-pms-integration-health/src/service.rs`

**Files to Modify:**
- `crates/qa-pms-integration-health/src/lib.rs` - Export service

**Testing Strategy:**
- Unit tests: Service functions (with mock repository)
- Integration tests: Service works with real repository
- Manual tests: Status calculation logic

**Success Metrics:**
- Service functions work correctly
- Status calculation accurate
- Business logic implemented correctly

---

#### Sprint 3: API Endpoints (2 days)

**Story 22.5: Integration Health API Endpoints**
- **Priority:** P0
- **Estimated Days:** 2
- **Status:** `backlog`
- **Dependencies:** Story 22.4

**Description:** Create REST API endpoints for integration health data

**Technical Requirements:**
1. Create `routes/integrations.rs` in `qa-pms-api`
2. Endpoint: `GET /api/v1/integrations/health?period=30d`
3. Endpoint: `GET /api/v1/integrations/health/:integration_id?period=30d`
4. Endpoint: `POST /api/v1/integrations/health` (manual update, Phase 1)
5. Endpoint: `GET /api/v1/integrations/health/:integration_id/events?period=30d`
6. Use `utoipa` for OpenAPI documentation
7. Follow existing patterns (Axum routes, `State<AppState>`)

**Acceptance Criteria:**
- [ ] Given API endpoint, When GET /api/v1/integrations/health, Then returns health status for all integrations
- [ ] Given API endpoint, When GET /api/v1/integrations/health/booking-com, Then returns health status for Booking.com
- [ ] Given API endpoint, When POST /api/v1/integrations/health, Then stores health status manually
- [ ] Given API endpoint, When GET /api/v1/integrations/health/booking-com/events, Then returns events for Booking.com
- [ ] Given API endpoint, When querying with period parameter, Then returns data for specified period
- [ ] Given API endpoint, When response returned, Then response time < 500ms

**Files to Create:**
- `crates/qa-pms-api/src/routes/integrations.rs`

**Files to Modify:**
- `crates/qa-pms-api/src/app.rs` - Add routes
- `crates/qa-pms-api/src/lib.rs` - Export routes (if needed)
- `crates/qa-pms-api/Cargo.toml` - Add `qa-pms-integration-health` dependency

**Testing Strategy:**
- Unit tests: Route handlers (with mock service)
- Integration tests: API endpoints work correctly
- Manual tests: API responses correct format, performance

**Success Metrics:**
- API endpoints functional
- Response time < 500ms
- OpenAPI documentation complete

---

#### Sprint 4: Dashboard Integration (3 days)

**Story 22.6: Integration Health Dashboard Widget**
- **Priority:** P0
- **Estimated Days:** 2
- **Status:** `backlog`
- **Dependencies:** Story 22.5

**Description:** Add integration health widget to existing dashboard

**Technical Requirements:**
1. Create `IntegrationHealthWidget` component
2. Display status cards for each integration (Booking.com, Airbnb, Vrbo, HMBN)
3. Status indicators: 🟢 OK, 🟡 Warning, 🔴 Critical
4. Sub-status: Pricing Sync, Fees Sync, Booking Loss
5. Click navigation to Integration Detail Page
6. Use existing dashboard components/patterns
7. Integrate into existing DashboardPage

**Acceptance Criteria:**
- [ ] Given dashboard page, When loaded, Then integration health widget is displayed
- [ ] Given widget, When integrations have different statuses, Then status indicators are displayed correctly (🟢/🟡/🔴)
- [ ] Given widget, When clicking on integration card, Then navigates to Integration Detail Page
- [ ] Given widget, When data loading, Then loading state is displayed
- [ ] Given widget, When error occurs, Then error state is displayed
- [ ] Given widget, When viewed on different screens, Then widget is responsive

**Files to Create:**
- `frontend/src/components/dashboard/IntegrationHealthWidget.tsx`
- `frontend/src/hooks/useIntegrationHealth.ts`

**Files to Modify:**
- `frontend/src/pages/Dashboard/DashboardPage.tsx` - Add widget section
- `frontend/src/components/dashboard/index.ts` - Export widget

**Testing Strategy:**
- Unit tests: Component rendering, status indicators
- Integration tests: Widget integrates with dashboard
- Manual tests: UI looks correct, navigation works

**Success Metrics:**
- Widget displays correctly
- Status indicators clear and actionable
- Navigation works smoothly

---

**Story 22.7: Integration Detail Page**
- **Priority:** P0
- **Estimated Days:** 1
- **Status:** `backlog`
- **Dependencies:** Story 22.6

**Description:** Create detailed view page for integration health

**Technical Requirements:**
1. Create `IntegrationDetailPage` component
2. Display detailed metrics: pricing sync status, fees sync status, booking loss rate
3. Timeline visualization of integration events
4. Trend charts (pricing sync errors, fees sync errors, booking loss)
5. Links to related test cases (Testmo) - future
6. Links to related tickets (Jira) - future
7. Export data (CSV, PDF) - future
8. Use existing chart components (TrendChart)

**Acceptance Criteria:**
- [ ] Given detail page, When navigating from widget, Then page displays integration details
- [ ] Given detail page, When viewing metrics, Then pricing sync, fees sync, booking loss are displayed
- [ ] Given detail page, When viewing timeline, Then integration events are displayed in timeline
- [ ] Given detail page, When viewing trends, Then trend charts show errors over time
- [ ] Given detail page, When data loading, Then loading state is displayed
- [ ] Given detail page, When error occurs, Then error state is displayed

**Files to Create:**
- `frontend/src/pages/Dashboard/IntegrationDetailPage.tsx`

**Files to Modify:**
- `frontend/src/App.tsx` - Add route `/dashboard/integrations/:integrationId`
- `frontend/src/hooks/useIntegrationHealth.ts` - Add function for detail data

**Testing Strategy:**
- Unit tests: Component rendering, charts display
- Integration tests: Page integrates with routing
- Manual tests: UI looks correct, charts render correctly

**Success Metrics:**
- Page displays correctly
- Charts render correctly
- Navigation works smoothly

---

## Epic 23: Revenue Impact Calculator and Dashboard

**Priority:** 🔴 P0 (Critical for business value)  
**Focus:** Calculate and display revenue impact of integration failures  
**Status:** `backlog`

### Problem Statement

QA Leads need revenue-focused metrics to demonstrate ROI of QA efforts to stakeholders, but the current framework doesn't quantify revenue protection from testing. Integration failures cause 3-7% revenue leakage, and QAs need to quantify this impact to prioritize work and demonstrate value.

### User Story

**As a** QA Lead managing QA team working on PMS integrations  
**I want** to see revenue impact metrics (revenue at risk, revenue protected) in the framework dashboard  
**So that** I can demonstrate ROI of QA efforts to stakeholders and prioritize work based on revenue impact

### Success Criteria

- ✅ Revenue impact automatically calculated
- ✅ Revenue metrics visible in dashboard
- ✅ QA Leads can export revenue reports
- ✅ Revenue calculation accurate within 5%

### Dependencies

- ✅ Epic 22 (Integration Health) - Required
- ✅ Revenue configuration (config YAML)

### Stories

#### Sprint 1: Revenue Calculation Engine (3 days)

**Story 23.1: Revenue Configuration System**
- **Priority:** P0
- **Estimated Days:** 1
- **Status:** `backlog`
- **Dependencies:** None

**Description:** Create configuration system for revenue calculation metrics

**Technical Requirements:**
1. Create `qa-pms-revenue` crate
2. Define config structure: `avg_booking_value`, `leakage_percentage` (3-7%)
3. Configurable per integration (Booking.com, Airbnb, Vrbo, HMBN)
4. Default values: avg_booking_value = $250, leakage_percentage = 0.05 (5%)
5. Store in YAML config (encrypted for sensitive data)
6. Use existing config patterns (`qa-pms-config`)

**Acceptance Criteria:**
- [ ] Given config system, When loading config, Then revenue metrics are loaded correctly
- [ ] Given config system, When config missing, Then default values are used
- [ ] Given config system, When per-integration config exists, Then per-integration values are used
- [ ] Given config system, When sensitive data, Then data is encrypted (AES-256-GCM)

**Files to Create:**
- `crates/qa-pms-revenue/Cargo.toml`
- `crates/qa-pms-revenue/src/lib.rs`
- `crates/qa-pms-revenue/src/types.rs`
- `crates/qa-pms-revenue/src/config.rs`

**Files to Modify:**
- `Cargo.toml` (workspace) - Add new crate
- `crates/qa-pms-config/src/user_config.rs` - Add revenue config (if needed)
- `crates/qa-pms-api/Cargo.toml` - Add dependency

**Testing Strategy:**
- Unit tests: Config loading, default values
- Unit tests: Encryption/decryption
- Integration tests: Config works with API

**Success Metrics:**
- Config loads correctly
- Default values work
- Encryption works correctly

---

**Story 23.2: Revenue Impact Calculation Engine**
- **Priority:** P0
- **Estimated Days:** 2
- **Status:** `backlog`
- **Dependencies:** Story 23.1, Epic 22 (Story 22.4)

**Description:** Implement revenue calculation logic for integration failures

**Technical Requirements:**
1. Create `calculator.rs` module
2. Calculate revenue loss from pricing sync errors
3. Calculate revenue loss from fees sync errors
4. Calculate revenue loss from booking loss
5. Use configurable metrics (avg_booking_value, leakage_percentage)
6. Support different calculation methods per integration type
7. Calculations accurate within 5%

**Acceptance Criteria:**
- [ ] Given calculation engine, When calculating pricing sync errors, Then revenue loss is calculated correctly
- [ ] Given calculation engine, When calculating fees sync errors, Then revenue loss is calculated correctly
- [ ] Given calculation engine, When calculating booking loss, Then revenue loss is calculated correctly
- [ ] Given calculation engine, When using config values, Then calculations use config values
- [ ] Given calculation engine, When per-integration config exists, Then per-integration calculations are used
- [ ] Given calculation engine, When comparing with expected values, Then calculations accurate within 5%

**Files to Create:**
- `crates/qa-pms-revenue/src/calculator.rs`
- `crates/qa-pms-revenue/src/error.rs`

**Files to Modify:**
- `crates/qa-pms-revenue/src/lib.rs` - Export calculator

**Testing Strategy:**
- Unit tests: Calculation logic (with test data)
- Unit tests: Edge cases (zero values, negative values)
- Integration tests: Calculations work with real data
- Manual tests: Verify calculation accuracy

**Success Metrics:**
- Calculations accurate within 5%
- Calculations performant (< 100ms)
- Edge cases handled correctly

---

#### Sprint 2: Revenue API and Dashboard (3 days)

**Story 23.3: Revenue Impact API Endpoint**
- **Priority:** P0
- **Estimated Days:** 1
- **Status:** `backlog`
- **Dependencies:** Story 23.2

**Description:** Create REST API endpoint for revenue impact data

**Technical Requirements:**
1. Create `routes/revenue.rs` in `qa-pms-api`
2. Endpoint: `GET /api/v1/revenue/impact?period=30d`
3. Endpoint: `GET /api/v1/revenue/config` (admin only, future)
4. Endpoint: `PUT /api/v1/revenue/config` (admin only, future)
5. Returns revenue at risk, revenue protected, breakdown by integration
6. Use `utoipa` for OpenAPI documentation
7. Response time < 500ms

**Acceptance Criteria:**
- [ ] Given API endpoint, When GET /api/v1/revenue/impact, Then returns revenue at risk and revenue protected
- [ ] Given API endpoint, When GET /api/v1/revenue/impact?period=7d, Then returns data for 7 days
- [ ] Given API endpoint, When response returned, Then response includes breakdown by integration
- [ ] Given API endpoint, When response returned, Then response time < 500ms
- [ ] Given API endpoint, When error occurs, Then error response is clear

**Files to Create:**
- `crates/qa-pms-api/src/routes/revenue.rs`

**Files to Modify:**
- `crates/qa-pms-api/src/app.rs` - Add routes
- `crates/qa-pms-api/Cargo.toml` - Add `qa-pms-revenue` dependency

**Testing Strategy:**
- Unit tests: Route handlers
- Integration tests: API endpoints work correctly
- Manual tests: API responses correct format, performance

**Success Metrics:**
- API endpoint functional
- Response time < 500ms
- OpenAPI documentation complete

---

**Story 23.4: Revenue Impact KPI Cards**
- **Priority:** P0
- **Estimated Days:** 1
- **Status:** `backlog`
- **Dependencies:** Story 23.3

**Description:** Add revenue impact KPI cards to dashboard

**Technical Requirements:**
1. Create `RevenueImpactCards` component
2. Revenue At Risk KPI card: shows current revenue at risk
3. Revenue Protected KPI card: shows revenue protected by QA testing
4. Trend indicators (up/down/neutral)
5. Percentage change from previous period
6. Click navigation to Revenue Detail Page
7. Reuse existing `KPICard` component

**Acceptance Criteria:**
- [ ] Given dashboard page, When loaded, Then revenue impact KPI cards are displayed
- [ ] Given KPI cards, When revenue at risk calculated, Then "Revenue At Risk" card shows correct value
- [ ] Given KPI cards, When revenue protected calculated, Then "Revenue Protected" card shows correct value
- [ ] Given KPI cards, When trend calculated, Then trend indicators are displayed correctly
- [ ] Given KPI cards, When clicking on card, Then navigates to Revenue Detail Page
- [ ] Given KPI cards, When data loading, Then loading state is displayed

**Files to Create:**
- `frontend/src/components/dashboard/RevenueImpactCards.tsx`
- `frontend/src/hooks/useRevenueImpact.ts`

**Files to Modify:**
- `frontend/src/pages/Dashboard/DashboardPage.tsx` - Add KPI cards
- `frontend/src/components/dashboard/index.ts` - Export component

**Testing Strategy:**
- Unit tests: Component rendering, KPI cards display
- Integration tests: Cards integrate with dashboard
- Manual tests: UI looks correct, navigation works

**Success Metrics:**
- KPI cards display correctly
- Trend indicators clear
- Navigation works smoothly

---

**Story 23.5: Revenue Impact Detail Page**
- **Priority:** P1
- **Estimated Days:** 1
- **Status:** `backlog`
- **Dependencies:** Story 23.4

**Description:** Create detailed view page for revenue impact

**Technical Requirements:**
1. Create `RevenueImpactPage` component
2. Display revenue at risk, revenue protected
3. Breakdown table: integration, impact, type, trend
4. Trend charts (revenue impact over time)
5. Export data (CSV, PDF) - future
6. Configurable period selector
7. Use existing chart components

**Acceptance Criteria:**
- [ ] Given detail page, When navigating from KPI cards, Then page displays revenue impact details
- [ ] Given detail page, When viewing breakdown, Then breakdown table shows integration, impact, type, trend
- [ ] Given detail page, When viewing trends, Then trend charts show revenue impact over time
- [ ] Given detail page, When selecting period, Then data updates for selected period
- [ ] Given detail page, When data loading, Then loading state is displayed

**Files to Create:**
- `frontend/src/pages/Dashboard/RevenueImpactPage.tsx`

**Files to Modify:**
- `frontend/src/App.tsx` - Add route `/dashboard/revenue/impact`
- `frontend/src/hooks/useRevenueImpact.ts` - Add function for detail data

**Testing Strategy:**
- Unit tests: Component rendering, charts display
- Integration tests: Page integrates with routing
- Manual tests: UI looks correct, charts render correctly

**Success Metrics:**
- Page displays correctly
- Charts render correctly
- Period selector works

---

## Epic 24: Test-Integration Correlation Engine

**Priority:** 🟡 P1 (Important but depends on Epic 22)  
**Focus:** Correlate test results with integration health to predict and prevent failures  
**Status:** `backlog`

### Problem Statement

QA Engineers need to correlate test results with integration health status to prioritize test work and prevent integration failures, but currently test results (Testmo) and integration health are tracked separately. Without correlation, QAs can't effectively understand if test failures indicate real integration problems.

### User Story

**As a** QA Engineer working on PMS integration quality  
**I want** to see correlation between test results and integration health in the framework  
**So that** I can prioritize test work and prevent integration failures proactively

### Success Criteria

- ✅ Test results correlate with integration health
- ✅ Correlation dashboard shows insights
- ✅ QAs can prioritize work based on correlation
- ✅ Correlation accuracy > 85%

### Dependencies

- ✅ Epic 22 (Integration Health) - Required
- ✅ Testmo integration (already exists)
- ⚠️ Test execution data (from Testmo)

### Stories

#### Sprint 1: Correlation Engine (3 days)

**Story 24.1: Correlation Calculation Engine**
- **Priority:** P1
- **Estimated Days:** 2
- **Status:** `backlog`
- **Dependencies:** Epic 22 (Story 22.4), Testmo integration (exists)

**Description:** Implement correlation calculation logic between test results and integration health

**Technical Requirements:**
1. Create `qa-pms-correlation` crate
2. Correlate test results (Testmo) with integration events (integration_health, integration_events)
3. Calculate correlation score (0-1, where 1 is perfect correlation)
4. Identify patterns: test failures precede integration failures
5. Time window for correlation: configurable (default: 1 hour)
6. Correlation confidence score
7. Correlation accuracy > 85%

**Acceptance Criteria:**
- [ ] Given correlation engine, When correlating test results with integration events, Then correlation score is calculated
- [ ] Given correlation engine, When test failures precede integration failures, Then pattern is identified
- [ ] Given correlation engine, When using time window, Then correlation uses specified time window
- [ ] Given correlation engine, When calculating confidence, Then confidence score is calculated
- [ ] Given correlation engine, When comparing with expected correlations, Then correlation accuracy > 85%

**Files to Create:**
- `crates/qa-pms-correlation/Cargo.toml`
- `crates/qa-pms-correlation/src/lib.rs`
- `crates/qa-pms-correlation/src/types.rs`
- `crates/qa-pms-correlation/src/engine.rs`
- `crates/qa-pms-correlation/src/error.rs`

**Files to Modify:**
- `Cargo.toml` (workspace) - Add new crate
- `crates/qa-pms-api/Cargo.toml` - Add dependencies

**Testing Strategy:**
- Unit tests: Correlation calculation logic
- Unit tests: Pattern detection
- Integration tests: Correlation works with real data
- Manual tests: Verify correlation accuracy

**Success Metrics:**
- Correlation calculation accurate (> 85%)
- Pattern detection works correctly
- Calculations performant (< 500ms)

---

**Story 24.2: Correlation Database Schema**
- **Priority:** P1
- **Estimated Days:** 1
- **Status:** `backlog`
- **Dependencies:** Story 24.1

**Description:** Create database schema for correlation cache

**Technical Requirements:**
1. Create `test_integration_correlations` table (cache)
2. Store correlation scores, types, patterns, confidence
3. Add indexes: `test_case_id`, `integration_id`, `correlation_score`
4. Create SQLx migration file
5. Cache correlation results for performance

**Acceptance Criteria:**
- [ ] Given database schema, When migration runs, Then `test_integration_correlations` table is created
- [ ] Given table exists, When querying by `test_case_id`, Then query performance < 100ms
- [ ] Given table exists, When querying by `integration_id`, Then query performance < 100ms
- [ ] Given table exists, When querying by `correlation_score`, Then query performance < 100ms

**Files to Create:**
- `migrations/YYYYMMDDHHMMSS_create_correlation_table.sql`

**Files to Modify:**
- None (new migration)

**Testing Strategy:**
- Unit tests: Migration SQL validation
- Integration tests: Migration runs successfully, table created
- Manual tests: Verify indexes created, test query performance

**Success Metrics:**
- Migration runs without errors
- Table created with correct schema
- Indexes created and functional

---

#### Sprint 2: Correlation API and Dashboard (3 days)

**Story 24.3: Correlation API Endpoint**
- **Priority:** P1
- **Estimated Days:** 1
- **Status:** `backlog`
- **Dependencies:** Story 24.2

**Description:** Create REST API endpoint for correlation data

**Technical Requirements:**
1. Create `routes/correlation.rs` in `qa-pms-api`
2. Endpoint: `GET /api/v1/correlation/test-integration?period=30d&integration=airbnb`
3. Endpoint: `GET /api/v1/correlation/test-integration/:test_case_id`
4. Returns correlations between test cases and integrations
5. Supports filtering by test case, integration, period
6. Use `utoipa` for OpenAPI documentation
7. Response time < 500ms

**Acceptance Criteria:**
- [ ] Given API endpoint, When GET /api/v1/correlation/test-integration, Then returns correlations
- [ ] Given API endpoint, When GET /api/v1/correlation/test-integration?integration=airbnb, Then returns correlations for Airbnb
- [ ] Given API endpoint, When GET /api/v1/correlation/test-integration/:test_case_id, Then returns correlation for specific test case
- [ ] Given API endpoint, When response returned, Then response includes correlation score, type, pattern, confidence
- [ ] Given API endpoint, When response returned, Then response time < 500ms

**Files to Create:**
- `crates/qa-pms-api/src/routes/correlation.rs`

**Files to Modify:**
- `crates/qa-pms-api/src/app.rs` - Add routes
- `crates/qa-pms-api/Cargo.toml` - Add `qa-pms-correlation` dependency

**Testing Strategy:**
- Unit tests: Route handlers
- Integration tests: API endpoints work correctly
- Manual tests: API responses correct format, performance

**Success Metrics:**
- API endpoint functional
- Response time < 500ms
- OpenAPI documentation complete

---

**Story 24.4: Correlation Dashboard View**
- **Priority:** P1
- **Estimated Days:** 2
- **Status:** `backlog`
- **Dependencies:** Story 24.3

**Description:** Create dashboard view showing test-integration correlations

**Technical Requirements:**
1. Create `CorrelationView` component
2. Timeline visualization showing test failures and integration failures over time
3. Correlation insights table: test case, integration, correlation score, type
4. Recommendations section: prioritized test cases based on correlation
5. Filters: period, integration, test case, correlation threshold
6. Export data (CSV, PDF) - future
7. Use existing chart components

**Acceptance Criteria:**
- [ ] Given correlation view, When loading page, Then timeline visualization is displayed
- [ ] Given correlation view, When viewing insights, Then correlation insights table is displayed
- [ ] Given correlation view, When viewing recommendations, Then recommendations section is displayed
- [ ] Given correlation view, When filtering by period, Then data updates for selected period
- [ ] Given correlation view, When filtering by integration, Then data updates for selected integration
- [ ] Given correlation view, When filtering by correlation threshold, Then only high correlations are shown

**Files to Create:**
- `frontend/src/pages/Dashboard/CorrelationPage.tsx`
- `frontend/src/components/dashboard/CorrelationView.tsx`
- `frontend/src/hooks/useCorrelation.ts`

**Files to Modify:**
- `frontend/src/App.tsx` - Add route `/dashboard/correlation/test-integration`
- `frontend/src/components/dashboard/index.ts` - Export component

**Testing Strategy:**
- Unit tests: Component rendering, charts display
- Integration tests: View integrates with routing
- Manual tests: UI looks correct, filters work

**Success Metrics:**
- View displays correctly
- Charts render correctly
- Filters work correctly

---

**Story 24.5: Correlation Alerts (Optional)**
- **Priority:** P2
- **Estimated Days:** 1
- **Status:** `backlog`
- **Dependencies:** Story 24.4

**Description:** Alert when test failures correlate with integration health issues

**Technical Requirements:**
1. Alert when test failure has high correlation (> 0.8) with integration failure
2. Alert displayed in dashboard (AlertBell component)
3. Alert includes: test case, integration, correlation score, recommendation
4. Alert can be dismissed
5. Alert history stored
6. Use existing alert system (Epic 9)

**Acceptance Criteria:**
- [ ] Given correlation system, When high correlation detected, Then alert is displayed
- [ ] Given alert, When displayed, Then alert includes test case, integration, correlation score, recommendation
- [ ] Given alert, When dismissed, Then alert is removed
- [ ] Given alert, When alert history, Then alert history is stored

**Files to Create:**
- (Use existing alert system)

**Files to Modify:**
- `crates/qa-pms-correlation/src/engine.rs` - Add alert generation
- `frontend/src/components/AlertBell.tsx` - Add correlation alerts (if needed)

**Testing Strategy:**
- Unit tests: Alert generation logic
- Integration tests: Alerts displayed correctly
- Manual tests: Alerts work correctly

**Success Metrics:**
- Alerts displayed correctly
- Alert history stored
- Recommendations actionable

---

## Summary

### Epic Overview

| Epic | Name | Priority | Stories | Est. Days | Status |
|------|------|----------|---------|-----------|--------|
| 22 | PMS Integration Health Monitoring | 🔴 P0 | 7 | 11 | `backlog` |
| 23 | Revenue Impact Calculator | 🔴 P0 | 5 | 7 | `backlog` |
| 24 | Test-Integration Correlation | 🟡 P1 | 5 | 7 | `backlog` |
| **Total** | | | **17** | **25** | |

### Implementation Phases

**Phase 1: Foundation (Epic 22) - Sprint 1-4**
- Stories: 22.1 - 22.7
- Estimated: 11 days
- Priority: P0
- Dependencies: Epic 8 (QA Dashboard) - Complete

**Phase 2: Revenue Impact (Epic 23) - Sprint 5-6**
- Stories: 23.1 - 23.5
- Estimated: 7 days
- Priority: P0
- Dependencies: Epic 22 (required)

**Phase 3: Correlation (Epic 24) - Sprint 7-8**
- Stories: 24.1 - 24.5
- Estimated: 7 days
- Priority: P1
- Dependencies: Epic 22 (required), Testmo integration (exists)

### Sprint Breakdown

**Sprint 1-2 (Epic 22 - Foundation):**
- Sprint 1: Database Schema and Core Types (Stories 22.1, 22.2) - 3 days
- Sprint 2: Repository and Service Layer (Stories 22.3, 22.4) - 3 days

**Sprint 3-4 (Epic 22 - Foundation, continued):**
- Sprint 3: API Endpoints (Story 22.5) - 2 days
- Sprint 4: Dashboard Integration (Stories 22.6, 22.7) - 3 days

**Sprint 5-6 (Epic 23 - Revenue Impact):**
- Sprint 5: Revenue Calculation Engine (Stories 23.1, 23.2) - 3 days
- Sprint 6: Revenue API and Dashboard (Stories 23.3, 23.4, 23.5) - 4 days

**Sprint 7-8 (Epic 24 - Correlation):**
- Sprint 7: Correlation Engine (Stories 24.1, 24.2) - 3 days
- Sprint 8: Correlation API and Dashboard (Stories 24.3, 24.4, 24.5) - 4 days

**Total:** 8 sprints, ~25 days

---

## Next Steps

1. **Review and Approve** this epic plan with stakeholders
2. **Prioritize** stories based on business needs (Epic 22 and 23 are P0)
3. **Create Detailed Story Documents** for each story (following existing story template)
4. **Update Sprint Status** to include new epics
5. **Begin Sprint 1** with Epic 22 (Integration Health Foundation)

---

**Document Status:** Draft for Review  
**Last Updated:** 2026-01-10  
**Next Review Date:** TBD

---

_This epics document is based on the PRD and Architecture Design documents created in January 2026._
