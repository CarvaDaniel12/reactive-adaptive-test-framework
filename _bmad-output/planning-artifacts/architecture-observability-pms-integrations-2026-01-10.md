---
stepsCompleted: [1, 2, 3, 4, 5]
workflowType: 'architecture'
project_name: 'QA Framework Improvements for PMS Integration Quality - Observability'
user_name: 'Daniel'
date: '2026-01-10'
inputDocuments:
  - _bmad-output/planning-artifacts/prd-observability-pms-integrations-2026-01-10.md
  - _bmad-output/planning-artifacts/design-thinking-observability-2026-01-10.md
  - _bmad-output/planning-artifacts/research/domain-observability-pms-integration-quality-research-2026-01-10.md
  - _bmad-output/implementation-artifacts/observability-current-state-audit.md
  - qa-intelligent-pms/docs/01-architecture.md
---

# Architecture Design: QA Framework Improvements for PMS Integration Quality - Observability

**Author:** Daniel  
**Date:** 2026-01-10  
**Version:** 1.0  
**Status:** Draft for Review

---

## Executive Summary

This document defines the technical architecture for enhancing the existing QA Intelligent PMS Framework to add PMS-specific observability capabilities. The architecture builds upon the existing framework structure (Rust crates, Axum API, PostgreSQL database, React frontend) and adds three new modules: Integration Health Monitoring, Revenue Impact Calculator, and Test-Integration Correlation Engine.

**Key Architectural Principles:**
- **Enhancement, Not Replacement**: Build upon existing framework, don't break existing functionality
- **Modular Design**: New modules as separate crates, following existing patterns
- **Reusability**: Leverage existing dashboard components, types, and utilities
- **Extensibility**: Architecture should support future enhancements (automated data collection, ML-based predictions)

---

## 1. Context

### 1.1 Current System Architecture

**Framework Structure:**
```
qa-intelligent-pms/
├── crates/
│   ├── qa-pms-core/           # Shared types, traits, utilities
│   ├── qa-pms-config/         # Configuration management, encryption
│   ├── qa-pms-api/            # Axum web server (main binary)
│   ├── qa-pms-dashboard/      # Dashboard logic, KPIs, metrics
│   ├── qa-pms-jira/           # Jira integration (OAuth 2.0 + PKCE)
│   ├── qa-pms-postman/        # Postman integration
│   ├── qa-pms-testmo/         # Testmo integration
│   ├── qa-pms-splunk/         # Splunk integration
│   ├── qa-pms-workflow/       # Workflow engine
│   ├── qa-pms-time/           # Time tracking
│   ├── qa-pms-patterns/       # Pattern detection
│   ├── qa-pms-support/        # Diagnostics, knowledge base
│   └── qa-pms-ai/             # AI companion (BYOK)
├── frontend/                  # React SPA (Vite + Tailwind CSS v4)
└── migrations/                # SQLx database migrations
```

**Technology Stack:**
- **Backend**: Rust 1.80+, Tokio, Axum 0.7, SQLx 0.7
- **Database**: Neon PostgreSQL (cloud)
- **Frontend**: React 19, Vite 7, Tailwind CSS v4, Zustand
- **API Documentation**: utoipa (OpenAPI/Swagger)

**Existing Patterns:**
- **Crate Structure**: Each feature as separate crate (qa-pms-*)
- **Error Handling**: `anyhow` (internal) + `thiserror` (API boundaries)
- **Logging**: `tracing` + `tracing-subscriber`
- **Encryption**: `aes-gcm` + `secrecy` (AES-256-GCM)
- **Database**: SQLx with migrations, `PgPool` shared state
- **API Routes**: Axum routers, `State<AppState>` pattern
- **Frontend Components**: React components, Zustand stores, React Query hooks

### 1.2 Integration Requirements

**Existing Integrations:**
- **Jira**: OAuth 2.0 + PKCE, API Token, ticket sync, search
- **Splunk**: File-based CSV/JSON export processing
- **Postman**: REST API, collection sync
- **Testmo**: REST API, test case sync, run tracking

**New Integration Requirements:**
- **PMS APIs** (optional): Integration health data collection (pricing sync, fees sync, booking loss)
- **Framework Dashboard**: Integration with existing dashboard (Epic 8)

### 1.3 Constraints and Requirements

**Technical Constraints:**
- Must work with existing framework structure
- Cannot break existing functionality
- Must follow existing patterns (crate structure, error handling, logging)
- Must integrate with existing dashboard (Epic 8)
- Must use existing database (PostgreSQL, SQLx)

**Business Constraints:**
- Data collection may start manual (Phase 1), automated later (Phase 4)
- Revenue calculation must be configurable
- Integration health data must be accurate (within 5% for revenue impact)
- Performance: Dashboard < 2s, API < 500ms

**Compliance Requirements:**
- GDPR compliant (data retention, right to deletion)
- PCI DSS compliant (if payment data involved)
- Data encryption for sensitive data (revenue configuration)

---

## 2. Architecture Decisions (ADRs)

### ADR-001: Integration Health Data Storage Strategy

**Status:** Proposed  
**Date:** 2026-01-10  
**Context:** Need to store integration health data (status, events, metrics)

**Decision:**
Store integration health data in PostgreSQL database using SQLx, following existing database patterns. Use two tables:
- `integration_health`: Current health status (denormalized for fast reads)
- `integration_events`: Historical events (normalized for query flexibility)

**Rationale:**
- ✅ Consistent with existing framework (SQLx, PostgreSQL)
- ✅ Fast reads for dashboard (denormalized status table)
- ✅ Flexible queries for analysis (normalized events table)
- ✅ Supports 90-day retention easily
- ✅ Existing patterns (SQLx migrations, error handling)

**Alternatives Considered:**
- **Time-series DB (TimescaleDB)**: Overkill for current requirements, adds complexity
- **In-memory cache (Redis)**: Good for real-time, but loses historical data
- **File-based storage**: Not scalable, doesn't integrate with existing patterns

**Consequences:**
- ✅ Simple to implement (existing patterns)
- ✅ Easy to query (SQL)
- ⚠️ May need optimization for high event volume (Phase 4)
- ⚠️ 90-day retention requires cleanup job (future)

---

### ADR-002: Revenue Impact Calculation Strategy

**Status:** Proposed  
**Date:** 2026-01-10  
**Context:** Need to calculate revenue impact of integration failures

**Decision:**
Implement revenue calculation as Rust function in `qa-pms-revenue` crate, using configurable metrics (avg_booking_value, leakage_percentage) stored in YAML config (encrypted for sensitive data). Calculate on-demand (API request), not pre-calculated (allows config changes).

**Rationale:**
- ✅ Configurable (different values per integration)
- ✅ Secure (encrypted config, following existing patterns)
- ✅ Accurate (Rust type safety, calculations in Rust)
- ✅ Flexible (on-demand allows config changes)
- ✅ Extensible (easy to add new calculation methods)

**Alternatives Considered:**
- **Pre-calculated values**: Faster reads, but harder to update config
- **External service**: Overkill, adds complexity
- **Database stored procedures**: Less flexible, harder to test

**Consequences:**
- ✅ Flexible and configurable
- ✅ Easy to test (Rust functions)
- ⚠️ Calculation time < 100ms (must optimize for large datasets)
- ⚠️ Config changes require restart (acceptable trade-off)

---

### ADR-003: Test-Integration Correlation Strategy

**Status:** Proposed  
**Date:** 2026-01-10  
**Context:** Need to correlate test results (Testmo) with integration health

**Decision:**
Implement correlation calculation as Rust function in `qa-pms-correlation` crate, using time-window based correlation (default: 1 hour). Calculate on-demand (API request), store correlation scores in database for caching. Correlation algorithm: simple time-window matching (test failures within 1 hour of integration failures = correlation).

**Rationale:**
- ✅ Simple to implement (time-window matching)
- ✅ Accurate enough (> 85% target)
- ✅ Extensible (can add ML-based correlation in Phase 4)
- ✅ Cachable (store scores in database)
- ✅ Integrates with existing Testmo integration

**Alternatives Considered:**
- **ML-based correlation**: More accurate, but complex (Phase 4)
- **Event-driven correlation**: Real-time, but adds complexity
- **External correlation service**: Overkill, adds complexity

**Consequences:**
- ✅ Simple and fast to implement
- ✅ Accurate enough for MVP (> 85%)
- ⚠️ May need ML-based correlation for higher accuracy (Phase 4)
- ⚠️ Correlation calculation < 500ms (must optimize)

---

### ADR-004: Dashboard Integration Strategy

**Status:** Proposed  
**Date:** 2026-01-10  
**Context:** Need to integrate new features with existing dashboard (Epic 8)

**Decision:**
Integrate new features as additional sections in existing dashboard, reusing existing components (KPICards, TrendChart) and types (KPIMetric, TrendDataPoint). Add new components (IntegrationHealthWidget, RevenueImpactCards) following existing patterns. Use existing dashboard route (`/dashboard`), add new routes for detail pages (`/dashboard/integrations/:id`, `/dashboard/revenue/impact`, `/dashboard/correlation/test-integration`).

**Rationale:**
- ✅ Consistent with existing dashboard (Epic 8)
- ✅ Reuses existing components (KPICards, TrendChart)
- ✅ Follows existing patterns (React components, Zustand stores, React Query hooks)
- ✅ Easy to implement (existing infrastructure)
- ✅ Doesn't break existing functionality

**Alternatives Considered:**
- **Separate dashboard**: Creates fragmentation, harder to use
- **Replace existing dashboard**: Breaks existing functionality
- **External dashboard**: Doesn't integrate with framework

**Consequences:**
- ✅ Consistent user experience
- ✅ Easy to implement
- ⚠️ Dashboard may become large (need to optimize rendering)
- ⚠️ May need pagination/lazy loading for large datasets

---

### ADR-005: Data Collection Strategy

**Status:** Proposed  
**Date:** 2026-01-10  
**Context:** Need to collect integration health data (Phase 1: manual, Phase 4: automated)

**Decision:**
Phase 1: Manual data collection via API endpoint (`POST /api/v1/integrations/health`) or admin UI. Phase 4: Automated data collection via background worker (Tokio task) polling PMS APIs or processing Splunk exports. Use existing Splunk integration patterns for Phase 4.

**Rationale:**
- ✅ Phased approach (manual → automated)
- ✅ Allows development without PMS API access
- ✅ Reuses existing Splunk integration patterns
- ✅ Extensible (easy to add new data sources)
- ✅ Flexible (can mix manual and automated)

**Alternatives Considered:**
- **Automated from start**: Requires PMS API access, complex
- **Manual only**: Not scalable, doesn't solve problem
- **External data collection service**: Overkill, adds complexity

**Consequences:**
- ✅ Phased implementation (reduce risk)
- ✅ Allows development without dependencies
- ⚠️ Manual collection may be error-prone (user training needed)
- ⚠️ Automated collection requires PMS API access (Phase 4)

---

## 3. Architecture Overview

### 3.1 New Modules

**Module 1: `qa-pms-integration-health`**
- **Purpose**: Monitor integration health status (Booking.com, Airbnb, Vrbo, HMBN)
- **Responsibility**: Health status tracking, event storage, metrics calculation
- **Dependencies**: `qa-pms-core`, `qa-pms-dashboard` (types)
- **Database Tables**: `integration_health`, `integration_events`

**Module 2: `qa-pms-revenue`**
- **Purpose**: Calculate revenue impact of integration failures
- **Responsibility**: Revenue calculation, config management, metrics aggregation
- **Dependencies**: `qa-pms-core`, `qa-pms-config`, `qa-pms-integration-health`
- **Database Tables**: (none, uses integration_health data)

**Module 3: `qa-pms-correlation`**
- **Purpose**: Correlate test results with integration health
- **Responsibility**: Correlation calculation, pattern detection, recommendations
- **Dependencies**: `qa-pms-core`, `qa-pms-integration-health`, `qa-pms-testmo` (existing)
- **Database Tables**: `test_integration_correlations` (cache)

### 3.2 Updated Modules

**Module: `qa-pms-dashboard`**
- **Updates**: Add new types (`IntegrationHealth`, `RevenueImpact`, `Correlation`)
- **Reuses**: Existing types (`KPIMetric`, `TrendDataPoint`)
- **No breaking changes**: Existing functionality unchanged

**Module: `qa-pms-api`**
- **Updates**: Add new routes (`/api/v1/integrations/*`, `/api/v1/revenue/*`, `/api/v1/correlation/*`)
- **Reuses**: Existing patterns (Axum routers, `State<AppState>`)
- **No breaking changes**: Existing routes unchanged

**Module: `frontend`**
- **Updates**: Add new components (IntegrationHealthWidget, RevenueImpactCards, CorrelationView)
- **Reuses**: Existing components (KPICards, TrendChart)
- **No breaking changes**: Existing components unchanged

---

## 4. Data Models

### 4.1 Integration Health Data Model

**Table: `integration_health`**
```sql
CREATE TABLE integration_health (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    integration_id VARCHAR(50) NOT NULL, -- 'booking-com', 'airbnb', 'vrbo', 'hmbn'
    status VARCHAR(20) NOT NULL, -- 'healthy', 'warning', 'critical'
    pricing_sync_status VARCHAR(20), -- 'ok', 'warning', 'error'
    fees_sync_status VARCHAR(20), -- 'ok', 'warning', 'error'
    booking_loss_rate DECIMAL(5,4), -- 0.0000 to 1.0000
    error_rate DECIMAL(5,4), -- 0.0000 to 1.0000
    last_checked TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(integration_id, last_checked)
);

CREATE INDEX idx_integration_health_integration ON integration_health(integration_id);
CREATE INDEX idx_integration_health_last_checked ON integration_health(last_checked DESC);
```

**Table: `integration_events`**
```sql
CREATE TABLE integration_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    integration_id VARCHAR(50) NOT NULL,
    event_type VARCHAR(50) NOT NULL, -- 'pricing_sync_error', 'fee_sync_error', 'booking_loss'
    severity VARCHAR(20) NOT NULL, -- 'low', 'medium', 'high', 'critical'
    message TEXT,
    metadata JSONB,
    occurred_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_integration_events_integration ON integration_events(integration_id);
CREATE INDEX idx_integration_events_occurred_at ON integration_events(occurred_at DESC);
CREATE INDEX idx_integration_events_type ON integration_events(event_type);
```

**Rust Types:**
```rust
// qa-pms-integration-health/src/types.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum IntegrationId {
    BookingCom,
    Airbnb,
    Vrbo,
    Hmbn,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationHealth {
    pub integration_id: IntegrationId,
    pub status: HealthStatus,
    pub pricing_sync_status: Option<HealthStatus>,
    pub fees_sync_status: Option<HealthStatus>,
    pub booking_loss_rate: Option<f64>,
    pub error_rate: Option<f64>,
    pub last_checked: chrono::DateTime<chrono::Utc>,
    pub trend: String, // "up", "down", "neutral"
}
```

### 4.2 Revenue Impact Data Model

**Config (YAML):**
```yaml
revenue:
  avg_booking_value: 250.00
  leakage_percentage: 0.05  # 5% (3-7% benchmark)
  integrations:
    booking-com:
      avg_booking_value: 280.00
      leakage_percentage: 0.06
    airbnb:
      avg_booking_value: 300.00
      leakage_percentage: 0.07
    vrbo:
      avg_booking_value: 250.00
      leakage_percentage: 0.05
    hmbn:
      avg_booking_value: 220.00
      leakage_percentage: 0.04
```

**Rust Types:**
```rust
// qa-pms-revenue/src/types.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use qa_pms_dashboard::KPIMetric;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RevenueImpact {
    pub revenue_at_risk: KPIMetric,
    pub revenue_protected: KPIMetric,
    pub breakdown: Vec<RevenueBreakdown>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RevenueBreakdown {
    pub integration_id: String,
    pub integration_name: String,
    pub impact: f64,
    pub impact_type: String, // "pricing_sync_error", "fee_sync_error", "booking_loss"
    pub estimated_loss: f64,
    pub trend: String,
}
```

### 4.3 Correlation Data Model

**Table: `test_integration_correlations` (cache)**
```sql
CREATE TABLE test_integration_correlations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    test_case_id VARCHAR(100) NOT NULL, -- Testmo test case ID
    integration_id VARCHAR(50) NOT NULL,
    correlation_score DECIMAL(3,2) NOT NULL, -- 0.00 to 1.00
    correlation_type VARCHAR(20) NOT NULL, -- 'high', 'medium', 'low'
    pattern VARCHAR(50) NOT NULL, -- 'test_failure_precedes_integration_failure'
    confidence DECIMAL(3,2) NOT NULL, -- 0.00 to 1.00
    last_correlated TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(test_case_id, integration_id)
);

CREATE INDEX idx_correlation_test_case ON test_integration_correlations(test_case_id);
CREATE INDEX idx_correlation_integration ON test_integration_correlations(integration_id);
CREATE INDEX idx_correlation_score ON test_integration_correlations(correlation_score DESC);
```

**Rust Types:**
```rust
// qa-pms-correlation/src/types.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Correlation {
    pub test_case_id: String,
    pub test_case_name: String,
    pub integration_id: String,
    pub correlation_score: f64,
    pub correlation_type: String, // "high", "medium", "low"
    pub pattern: String,
    pub confidence: f64,
    pub last_correlated: chrono::DateTime<chrono::Utc>,
}
```

---

## 5. API Design

### 5.1 Integration Health API

**Base Path:** `/api/v1/integrations`

**Routes:**
- `GET /api/v1/integrations/health?period=30d` - Get integration health status
- `GET /api/v1/integrations/health/:integration_id?period=30d` - Get specific integration health
- `POST /api/v1/integrations/health` - Manual health update (Phase 1)
- `GET /api/v1/integrations/health/:integration_id/events?period=30d` - Get integration events

**Example Response:**
```json
{
  "integrations": [
    {
      "integrationId": "booking-com",
      "name": "Booking.com",
      "status": "healthy",
      "pricingSyncStatus": "ok",
      "feesSyncStatus": "ok",
      "bookingLossRate": 0.001,
      "errorRate": 0.02,
      "lastChecked": "2026-01-10T10:30:00Z",
      "trend": "neutral"
    }
  ],
  "overallHealth": "healthy"
}
```

### 5.2 Revenue Impact API

**Base Path:** `/api/v1/revenue`

**Routes:**
- `GET /api/v1/revenue/impact?period=30d` - Get revenue impact
- `GET /api/v1/revenue/config` - Get revenue configuration (admin only)
- `PUT /api/v1/revenue/config` - Update revenue configuration (admin only)

**Example Response:**
```json
{
  "revenueAtRisk": {
    "value": 575.50,
    "change": -5.2,
    "trend": "down"
  },
  "revenueProtected": {
    "value": 12450.00,
    "change": 8.3,
    "trend": "up"
  },
  "breakdown": [
    {
      "integrationId": "airbnb",
      "integrationName": "Airbnb",
      "impact": 450.00,
      "impactType": "booking_loss",
      "estimatedLoss": 450.00,
      "trend": "down"
    }
  ]
}
```

### 5.3 Correlation API

**Base Path:** `/api/v1/correlation`

**Routes:**
- `GET /api/v1/correlation/test-integration?period=30d&integration=airbnb` - Get correlations
- `GET /api/v1/correlation/test-integration/:test_case_id` - Get specific test correlation

**Example Response:**
```json
{
  "correlations": [
    {
      "testCaseId": "test-456",
      "testCaseName": "Booking Flow Test",
      "integrationId": "airbnb",
      "correlationScore": 0.91,
      "correlationType": "high",
      "pattern": "test_failure_precedes_integration_failure",
      "confidence": 0.95,
      "lastCorrelated": "2026-01-10T10:15:00Z"
    }
  ],
  "recommendations": [
    {
      "priority": "high",
      "message": "Prioritize Booking Flow Test for Airbnb - high correlation with integration failures",
      "testCaseId": "test-456",
      "integrationId": "airbnb"
    }
  ]
}
```

---

## 6. Component Architecture

### 6.1 Backend Architecture

**New Crates:**
```
qa-pms-integration-health/
├── src/
│   ├── lib.rs                 # Crate exports
│   ├── types.rs               # Integration health types
│   ├── service.rs             # Health status service
│   ├── repository.rs          # Database repository
│   └── error.rs               # Error types
└── Cargo.toml

qa-pms-revenue/
├── src/
│   ├── lib.rs                 # Crate exports
│   ├── types.rs               # Revenue types
│   ├── calculator.rs          # Revenue calculation logic
│   ├── config.rs              # Revenue configuration
│   └── error.rs               # Error types
└── Cargo.toml

qa-pms-correlation/
├── src/
│   ├── lib.rs                 # Crate exports
│   ├── types.rs               # Correlation types
│   ├── engine.rs              # Correlation engine
│   ├── repository.rs          # Database repository
│   └── error.rs               # Error types
└── Cargo.toml
```

**Updated Crates:**
```
qa-pms-api/
├── src/
│   ├── routes/
│   │   ├── dashboard.rs       # Existing (no changes)
│   │   ├── integrations.rs    # NEW: Integration health routes
│   │   ├── revenue.rs         # NEW: Revenue impact routes
│   │   └── correlation.rs     # NEW: Correlation routes
│   └── ...
└── Cargo.toml

qa-pms-dashboard/
├── src/
│   ├── types.rs               # Add new types (IntegrationHealth, RevenueImpact, Correlation)
│   └── ...
└── Cargo.toml
```

### 6.2 Frontend Architecture

**New Components:**
```
frontend/src/
├── components/
│   ├── dashboard/
│   │   ├── IntegrationHealthWidget.tsx    # NEW: Integration health widget
│   │   ├── RevenueImpactCards.tsx         # NEW: Revenue impact KPI cards
│   │   ├── CorrelationView.tsx            # NEW: Correlation dashboard view
│   │   └── ... (existing components)
│   └── ...
├── pages/
│   ├── Dashboard/
│   │   ├── IntegrationDetailPage.tsx      # NEW: Integration detail page
│   │   ├── RevenueImpactPage.tsx          # NEW: Revenue impact page
│   │   ├── CorrelationPage.tsx            # NEW: Correlation page
│   │   └── DashboardPage.tsx              # UPDATED: Add new sections
│   └── ...
├── hooks/
│   ├── useIntegrationHealth.ts            # NEW: Integration health hook
│   ├── useRevenueImpact.ts                # NEW: Revenue impact hook
│   └── useCorrelation.ts                  # NEW: Correlation hook
└── ...
```

**Reused Components:**
- `KPICards` - Reused for Revenue Impact cards
- `TrendChart` - Reused for trend visualizations
- `KPICard` - Reused for individual metrics
- Existing dashboard infrastructure (routing, stores, queries)

---

## 7. Database Schema

### 7.1 Migration Files

**Migration: `YYYYMMDDHHMMSS_create_integration_health_tables.sql`**
```sql
-- Integration Health Tables
CREATE TABLE integration_health (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    integration_id VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL,
    pricing_sync_status VARCHAR(20),
    fees_sync_status VARCHAR(20),
    booking_loss_rate DECIMAL(5,4),
    error_rate DECIMAL(5,4),
    last_checked TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(integration_id, last_checked)
);

CREATE TABLE integration_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    integration_id VARCHAR(50) NOT NULL,
    event_type VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL,
    message TEXT,
    metadata JSONB,
    occurred_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_integration_health_integration ON integration_health(integration_id);
CREATE INDEX idx_integration_health_last_checked ON integration_health(last_checked DESC);
CREATE INDEX idx_integration_events_integration ON integration_events(integration_id);
CREATE INDEX idx_integration_events_occurred_at ON integration_events(occurred_at DESC);
CREATE INDEX idx_integration_events_type ON integration_events(event_type);
```

**Migration: `YYYYMMDDHHMMSS_create_correlation_table.sql`**
```sql
-- Correlation Cache Table
CREATE TABLE test_integration_correlations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    test_case_id VARCHAR(100) NOT NULL,
    integration_id VARCHAR(50) NOT NULL,
    correlation_score DECIMAL(3,2) NOT NULL,
    correlation_type VARCHAR(20) NOT NULL,
    pattern VARCHAR(50) NOT NULL,
    confidence DECIMAL(3,2) NOT NULL,
    last_correlated TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(test_case_id, integration_id)
);

-- Indexes
CREATE INDEX idx_correlation_test_case ON test_integration_correlations(test_case_id);
CREATE INDEX idx_correlation_integration ON test_integration_correlations(integration_id);
CREATE INDEX idx_correlation_score ON test_integration_correlations(correlation_score DESC);
```

---

## 8. Integration Points

### 8.1 Existing Framework Integration

**Dashboard Integration:**
- Reuse existing dashboard route (`/dashboard`)
- Add new sections to existing dashboard page
- Reuse existing components (KPICards, TrendChart)
- Follow existing patterns (React Query hooks, Zustand stores)

**Database Integration:**
- Use existing PostgreSQL database (Neon cloud)
- Use existing SQLx patterns (migrations, queries, error handling)
- Share `PgPool` via `State<AppState>`

**API Integration:**
- Add new routes to existing Axum router
- Use existing patterns (routes, handlers, error handling)
- Share `AppState` via `State<AppState>`

**Config Integration:**
- Reuse existing config crate (`qa-pms-config`)
- Use existing encryption patterns (AES-256-GCM)
- Store revenue config in YAML (encrypted)

### 8.2 External Integrations

**Testmo Integration (Existing):**
- Reuse existing `qa-pms-testmo` crate
- Use existing test case data for correlation
- No changes to Testmo integration

**Splunk Integration (Existing):**
- Reuse existing `qa-pms-splunk` crate patterns
- Use for integration health data collection (Phase 4)
- No changes to Splunk integration (Phase 1)

**Jira Integration (Existing):**
- Reuse existing `qa-pms-jira` crate
- Link integration health issues to Jira tickets
- No changes to Jira integration

**PMS APIs (Future - Phase 4):**
- Add new integration crate `qa-pms-pms-apis` (future)
- Poll PMS APIs for integration health data
- Follow existing integration patterns

---

## 9. Error Handling

### 9.1 Error Types

**Integration Health Errors:**
```rust
// qa-pms-integration-health/src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IntegrationHealthError {
    #[error("Integration not found: {0}")]
    NotFound(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}
```

**Revenue Errors:**
```rust
// qa-pms-revenue/src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RevenueError {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Calculation error: {0}")]
    Calculation(String),
    
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}
```

**Correlation Errors:**
```rust
// qa-pms-correlation/src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CorrelationError {
    #[error("Test case not found: {0}")]
    TestCaseNotFound(String),
    
    #[error("Integration not found: {0}")]
    IntegrationNotFound(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}
```

### 9.2 Error Handling Patterns

**API Error Handling:**
- Use existing `ApiError` type from `qa-pms-core`
- Convert crate errors to `ApiError` in API routes
- Return appropriate HTTP status codes (404, 500, etc.)

**Service Error Handling:**
- Use `anyhow::Result` for internal services
- Use `thiserror` for API boundaries
- Log errors using `tracing` (existing pattern)

---

## 10. Performance Considerations

### 10.1 Database Performance

**Indexes:**
- Index on `integration_id` for fast lookups
- Index on `last_checked` / `occurred_at` for time-based queries
- Index on `correlation_score` for sorting correlations

**Query Optimization:**
- Denormalized `integration_health` table for fast reads
- Use `LIMIT` for pagination
- Use connection pooling (existing `PgPool`)

**Data Retention:**
- 90-day retention for `integration_events`
- Cleanup job for old events (future)

### 10.2 API Performance

**Caching:**
- Cache correlation scores in database
- Cache revenue calculations (in-memory, TTL 1 minute)
- Use React Query caching (existing pattern)

**Response Time:**
- Target: < 500ms for API endpoints
- Use async/await (existing Tokio runtime)
- Optimize database queries

### 10.3 Frontend Performance

**Component Optimization:**
- Lazy load detail pages (React.lazy)
- Virtualize large lists (if needed)
- Memoize expensive calculations

**Dashboard Performance:**
- Target: < 2s load time
- Use React Query for data fetching (existing pattern)
- Optimize re-renders (React.memo if needed)

---

## 11. Security Considerations

### 11.1 Data Protection

**Encryption:**
- Revenue configuration encrypted (AES-256-GCM, existing pattern)
- Sensitive data encrypted at rest (database encryption, if available)

**Authentication:**
- Use existing authentication (if implemented)
- API endpoints require authentication (future)

**Authorization:**
- Revenue config endpoints: admin only (future)
- Integration health endpoints: all authenticated users
- Correlation endpoints: all authenticated users

### 11.2 Compliance

**GDPR:**
- Data retention: 90 days for events
- Right to deletion: support data deletion (future)
- Data minimization: only collect necessary data

**PCI DSS:**
- No payment data stored (revenue calculations only)
- If payment data needed: comply with PCI DSS (future)

---

## 12. Testing Strategy

### 12.1 Unit Tests

**Backend:**
- Unit tests for revenue calculation logic
- Unit tests for correlation calculation logic
- Unit tests for integration health service

**Frontend:**
- Unit tests for components (React Testing Library)
- Unit tests for hooks
- Unit tests for utilities

### 12.2 Integration Tests

**Backend:**
- Integration tests for API endpoints
- Integration tests for database operations
- Integration tests for service interactions

**Frontend:**
- Integration tests for dashboard flows
- Integration tests for detail pages
- E2E tests for critical paths (Playwright, future)

### 12.3 Performance Tests

**Backend:**
- Load tests for API endpoints (target: < 500ms)
- Database query performance tests

**Frontend:**
- Dashboard load time tests (target: < 2s)
- Component render performance tests

---

## 13. Deployment Strategy

### 13.1 Database Migrations

**Migration Process:**
- Use existing SQLx migration patterns
- Run migrations on deployment (existing process)
- Rollback support (if needed)

**Migration Order:**
1. Create `integration_health` tables (Phase 1)
2. Create `test_integration_correlations` table (Phase 3)

### 13.2 Feature Flags

**Feature Flags (if needed):**
- `integration_health_enabled`: Enable/disable integration health features
- `revenue_impact_enabled`: Enable/disable revenue impact features
- `correlation_enabled`: Enable/disable correlation features

**Rollout Strategy:**
- Phase 1: Integration health (manual data collection)
- Phase 2: Revenue impact
- Phase 3: Correlation
- Phase 4: Automated data collection

---

## 14. Monitoring and Observability

### 14.1 Logging

**Logging Strategy:**
- Use existing `tracing` patterns
- Log API requests (existing middleware)
- Log integration health collection events
- Log revenue calculation events (debug level)

### 14.2 Metrics

**Metrics to Track:**
- API endpoint response times
- Database query performance
- Integration health collection frequency
- Revenue calculation accuracy

**Metrics Storage:**
- Use existing Prometheus metrics (if available, Phase 4)
- Log metrics for now (Phase 1)

---

## 15. Future Enhancements

### 15.1 Phase 4 Enhancements

**Automated Data Collection:**
- Background worker for PMS API polling
- Real-time event processing
- Automated health status updates

**ML-Based Correlation:**
- ML-based correlation algorithm
- Predictive failure detection
- Automated recommendations

**Advanced Features:**
- Real-time alerts (email, Slack)
- Advanced analytics (ML-based predictions)
- Automated remediation (future)

---

## 16. Open Questions

1. **Data Collection**: How do we collect integration health data? Manual input initially or automated via PMS APIs?
2. **Revenue Configuration**: Who configures revenue metrics? QAs, QA Leads, or Finance team?
3. **Correlation Algorithm**: What correlation algorithm to use? Simple time-window or ML-based?
4. **Authentication**: When to implement authentication? Phase 1 or later?
5. **Performance**: How to handle high event volume? Database partitioning or time-series DB?
6. **PMS APIs**: What PMS APIs are available for integration health data collection?

---

**Document Status:** Draft for Review  
**Last Updated:** 2026-01-10  
**Next Review Date:** TBD

---

_This architecture design is based on the PRD and follows existing framework patterns and conventions._
