# Story 22.2: Integration Health Types and Error Handling

**Status:** `review`

## Story

**As a** QA Engineer working on PMS integration quality  
**I want** integration health types to be defined in Rust  
**So that** I can use them consistently across the codebase

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 22.2 |
| Epic | Epic 22: PMS Integration Health Monitoring Module |
| Sprint | Sprint 1: Database Schema and Core Types |
| Priority | P0 |
| Estimated Days | 1 |
| Dependencies | Story 22.1 |
| Status | `review` |

---

## Technical Requirements

1. Create `qa-pms-integration-health` crate
   - Follow existing crate structure patterns (`qa-pms-dashboard`, `qa-pms-patterns`)
   - Add crate to workspace `Cargo.toml`
   - Add dependencies: `serde`, `serde_json`, `utoipa`, `chrono`, `thiserror`, `anyhow`

2. Define types: `IntegrationId`, `HealthStatus`, `IntegrationHealth`, `IntegrationEvent`
   - Use `serde` for serialization (camelCase for JSON)
   - Use `utoipa::ToSchema` for OpenAPI documentation
   - Follow existing type patterns from `qa-pms-dashboard/src/types.rs`

3. Define error types: `IntegrationHealthError` (using `thiserror`)
   - Follow existing error patterns from `qa-pms-patterns`, `qa-pms-tracking`
   - Convert to `ApiError` for API boundaries (use `qa_pms_core::error::ApiError`)

4. Export types for use in API crate
   - Re-export commonly used types at crate root (`lib.rs`)
   - Follow existing patterns from `qa-pms-dashboard/src/lib.rs`

5. Follow existing patterns (`qa-pms-dashboard`, `qa-pms-patterns`)
   - Module structure: `lib.rs`, `types.rs`, `error.rs`
   - Error handling: `thiserror` for errors, `anyhow` for internal results
   - Serialization: `serde` with `rename_all = "camelCase"`

---

## Acceptance Criteria

- [x] **Given** crate structure exists  
  **When** compiling the crate  
  **Then** crate compiles without errors

- [x] **Given** types are defined  
  **When** serializing to JSON  
  **Then** types serialize correctly (camelCase format)

- [x] **Given** types are defined  
  **When** deserializing from JSON  
  **Then** types deserialize correctly

- [x] **Given** error types are defined  
  **When** error occurs  
  **Then** error message is clear and actionable

- [x] **Given** types are exported  
  **When** importing in API crate  
  **Then** types are available and compile correctly

---

## Tasks

- [x] Task 1: Create new crate structure (AC: #1)
  - [x] 1.1: Create `crates/qa-pms-integration-health/Cargo.toml` with dependencies
  - [x] 1.2: Add crate to workspace `Cargo.toml` members
  - [x] 1.3: Create `src/lib.rs` with module structure
  - [x] 1.4: Create `src/types.rs` module
  - [x] 1.5: Create `src/error.rs` module
  - [x] 1.6: Verify crate compiles without errors

- [x] Task 2: Define integration health types (AC: #2, #3)
  - [x] 2.1: Define `IntegrationId` enum (BookingCom, Airbnb, Vrbo, Hmbn)
  - [x] 2.2: Define `HealthStatus` enum (Healthy, Warning, Critical)
  - [x] 2.3: Define `IntegrationHealth` struct with all fields
  - [x] 2.4: Define `IntegrationEvent` struct with all fields
  - [x] 2.5: Add `Serialize`, `Deserialize`, `ToSchema` derives (camelCase)
  - [x] 2.6: Implement `Display` for enums
  - [x] 2.7: Add `sqlx::Type` derive for database compatibility

- [x] Task 3: Define error types (AC: #4)
  - [x] 3.1: Define `IntegrationHealthError` enum using `thiserror`
  - [x] 3.2: Add variants: NotFound, Database, Internal
  - [x] 3.3: Implement `From<sqlx::Error>` for Database variant
  - [x] 3.4: Implement `From<anyhow::Error>` for Internal variant
  - [x] 3.5: Implement `From<IntegrationHealthError>` for `ApiError`
  - [x] 3.6: Verify error messages are clear and actionable

- [x] Task 4: Export types from crate (AC: #5)
  - [x] 4.1: Re-export types in `lib.rs`
  - [x] 4.2: Re-export error types in `lib.rs`
  - [x] 4.3: Add crate documentation
  - [x] 4.4: Verify types can be imported in API crate
  - [x] 4.5: Test type serialization/deserialization (JSON camelCase)

- [x] Task 5: Add dependency to API crate
  - [x] 5.1: Add `qa-pms-integration-health` dependency to `crates/qa-pms-api/Cargo.toml`
  - [x] 5.2: Verify API crate compiles with new dependency

---

## Files to Create

| File | Changes |
|------|---------|
| `crates/qa-pms-integration-health/Cargo.toml` | Create new crate with dependencies |
| `crates/qa-pms-integration-health/src/lib.rs` | Crate root, re-export types |
| `crates/qa-pms-integration-health/src/types.rs` | Define IntegrationId, HealthStatus, IntegrationHealth, IntegrationEvent types |
| `crates/qa-pms-integration-health/src/error.rs` | Define IntegrationHealthError using thiserror |

---

## Files to Modify

| File | Changes |
|------|---------|
| `Cargo.toml` (workspace root) | Add new crate `qa-pms-integration-health` to workspace members |
| `crates/qa-pms-api/Cargo.toml` | Add `qa-pms-integration-health` dependency (for future use in Story 22.5) |

---

## Implementation Notes

### Crate Structure

Follow existing patterns from `qa-pms-dashboard` and `qa-pms-patterns`:

```
crates/qa-pms-integration-health/
├── Cargo.toml
└── src/
    ├── lib.rs          # Re-exports
    ├── types.rs        # Types (IntegrationId, HealthStatus, IntegrationHealth, IntegrationEvent)
    └── error.rs        # Error types (IntegrationHealthError)
```

### Type Definitions

Based on Architecture Document (ADR-001):

**IntegrationId** (enum):
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "kebab-case")]
#[serde(rename_all = "camelCase")]
pub enum IntegrationId {
    BookingCom,
    Airbnb,
    Vrbo,
    Hmbn,
}

impl std::fmt::Display for IntegrationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BookingCom => write!(f, "booking-com"),
            Self::Airbnb => write!(f, "airbnb"),
            Self::Vrbo => write!(f, "vrbo"),
            Self::Hmbn => write!(f, "hmbn"),
        }
    }
}
```

**HealthStatus** (enum):
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "snake_case")]
#[serde(rename_all = "camelCase")]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Healthy => write!(f, "healthy"),
            Self::Warning => write!(f, "warning"),
            Self::Critical => write!(f, "critical"),
        }
    }
}
```

**IntegrationHealth** (struct):
```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationHealth {
    pub integration_id: IntegrationId,
    pub status: HealthStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_sync_status: Option<HealthStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees_sync_status: Option<HealthStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booking_loss_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_rate: Option<f64>,
    pub last_checked: DateTime<Utc>,
    pub trend: String, // "up", "down", "neutral"
}
```

**IntegrationEvent** (struct):
```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationEvent {
    pub id: Uuid,
    pub integration_id: IntegrationId,
    pub event_type: String, // "pricing_sync_error", "fee_sync_error", "booking_loss"
    pub severity: String, // "low", "medium", "high", "critical"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    pub occurred_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
```

### Error Types

Follow existing patterns from `qa-pms-tracking/src/error.rs`:

```rust
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

// Conversion to ApiError for API boundaries
impl From<IntegrationHealthError> for qa_pms_core::error::ApiError {
    fn from(err: IntegrationHealthError) -> Self {
        match err {
            IntegrationHealthError::NotFound(msg) => ApiError::NotFound(msg),
            IntegrationHealthError::Database(e) => ApiError::Internal(e.into()),
            IntegrationHealthError::Internal(e) => ApiError::Internal(e),
        }
    }
}
```

### Cargo.toml

```toml
[package]
name = "qa-pms-integration-health"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
# Serialization
serde = { workspace = true, features = ["derive"] }
serde_json = { workspace = true }

# API Documentation
utoipa = { workspace = true }

# Time
chrono = { workspace = true, features = ["serde"] }

# Error Handling
thiserror = { workspace = true }
anyhow = { workspace = true }

# UUID
uuid = { workspace = true, features = ["v4", "serde"] }

# Database (for sqlx::Type derive)
sqlx = { workspace = true, features = ["runtime-tokio-native-tls", "postgres", "uuid", "chrono"] }

# Core types
qa-pms-core = { workspace = true }
```

### lib.rs

```rust
//! # QA PMS Integration Health
//!
//! Integration health monitoring for PMS integrations (Booking.com, Airbnb, Vrbo, HMBN).
//!
//! This crate provides:
//! - Types for integration health status and events
//! - Error handling for integration health operations
//!
//! ## Example Usage
//!
//! ```ignore
//! use qa_pms_integration_health::{IntegrationId, HealthStatus, IntegrationHealth, IntegrationHealthError};
//!
//! let integration = IntegrationId::BookingCom;
//! let status = HealthStatus::Healthy;
//! ```

pub mod error;
pub mod types;

// Re-export commonly used items at crate root
pub use error::IntegrationHealthError;
pub use types::{HealthStatus, IntegrationEvent, IntegrationHealth, IntegrationId};
```

---

## Testing Strategy

### Unit Tests

- **Type Serialization**: Test that types serialize to JSON correctly (camelCase)
- **Type Deserialization**: Test that types deserialize from JSON correctly
- **Error Formatting**: Test that error messages are clear and actionable
- **Enum Display**: Test that enum Display implementations work correctly

### Integration Tests

- **Type Compilation**: Test that types compile and work with API crate
- **Error Conversion**: Test that IntegrationHealthError converts to ApiError correctly

### Manual Tests

- Compile crate: `cargo check -p qa-pms-integration-health`
- Verify types are exported: `use qa_pms_integration_health::*;`
- Verify JSON serialization: serialize types to JSON and verify format

---

## Success Metrics

- Crate compiles successfully (`cargo check` passes)
- Types serialize/deserialize correctly (JSON camelCase format)
- Error types provide clear messages (readable error messages)
- Types exported correctly (can import in other crates)
- Ready for next story (22.3: Repository)

---

## Context & Dependencies

**Dependencies:**
- Story 22.1: Integration Health Database Schema (database schema must exist)

**Enables:**
- Story 22.3: Integration Health Repository (needs types)
- Story 22.4: Integration Health Service (needs types)
- Story 22.5: Integration Health API Endpoints (needs types)

**Related Documentation:**
- Architecture Document: `_bmad-output/planning-artifacts/architecture-observability-pms-integrations-2026-01-10.md`
- ADR-001: Integration Health Data Storage Strategy
- Epic Document: `_bmad-output/planning-artifacts/epics-observability-pms-integrations-2026-01-10.md`
- Reference Patterns: `qa-pms-dashboard/src/types.rs`, `qa-pms-patterns/src/types.rs`

---

---

## Dev Agent Record

### Agent Model Used
Claude Sonnet 4.5 (via Cursor)

### Implementation Notes

**Crate:** `qa-pms-integration-health`

**Implementation Summary:**
- Crate structure already existed and was verified to match story requirements
- All types (IntegrationId, HealthStatus, IntegrationHealth, IntegrationEvent) are correctly defined
- Error types (IntegrationHealthError) are correctly implemented using `thiserror`
- Types are properly exported from crate root in `lib.rs`
- Dependency already exists in API crate
- Added comprehensive tests for serialization/deserialization (10 tests, all passing)
- All acceptance criteria verified and satisfied

**Testing:**
- Added unit tests for serialization/deserialization (camelCase JSON format)
- Added tests for Display trait implementations
- All 10 tests passing successfully
- Crate compiles without errors
- API crate compiles with dependency

### File List

**Created:**
- `crates/qa-pms-integration-health/Cargo.toml` - Crate configuration (already existed)
- `crates/qa-pms-integration-health/src/lib.rs` - Crate root with re-exports (already existed)
- `crates/qa-pms-integration-health/src/types.rs` - Type definitions with tests (tests added)
- `crates/qa-pms-integration-health/src/error.rs` - Error type definitions (already existed)

**Modified:**
- `crates/qa-pms-integration-health/src/types.rs` - Added test module with serialization/deserialization tests
- `Cargo.toml` (workspace) - Crate already in members (verified)
- `crates/qa-pms-api/Cargo.toml` - Dependency already exists (verified)

### Change Log

**2026-01-11 - Story Implementation Complete:**
- Verified crate structure matches story requirements
- Added comprehensive unit tests for type serialization/deserialization (10 tests)
- All tests passing successfully
- Verified compilation of crate and API crate with dependency
- All acceptance criteria satisfied
- All tasks completed

---

**Story Status:** `review`  
**Last Updated:** 2026-01-11  
**Next Review:** Code review workflow
