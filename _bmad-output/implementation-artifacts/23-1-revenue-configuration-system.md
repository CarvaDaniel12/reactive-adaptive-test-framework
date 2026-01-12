# Story 23.1: Revenue Configuration System

**Status:** `ready-for-dev`

## Story

**As a** QA Engineer working on PMS integration quality  
**I want** revenue calculation metrics to be configurable  
**So that** revenue impact calculations can be customized per integration and adjusted over time

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 23.1 |
| Epic | Epic 23: Revenue Impact Calculator and Dashboard |
| Sprint | Sprint 1: Revenue Calculation Engine |
| Priority | P0 |
| Estimated Days | 1 |
| Dependencies | None |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create `qa-pms-revenue` crate following existing crate patterns
   - Add to workspace `Cargo.toml`
   - Create crate structure: `src/lib.rs`, `src/types.rs`, `src/config.rs`
   - Follow patterns from `qa-pms-dashboard`, `qa-pms-patterns` crates

2. Define config structure for revenue metrics:
   - `avg_booking_value`: Average booking value (default: $250.00)
   - `leakage_percentage`: Revenue leakage percentage (default: 0.05 = 5%, range: 3-7%)
   - Per-integration overrides: `booking-com`, `airbnb`, `vrbo`, `hmbn`
   - Support global defaults with per-integration customization

3. Config storage:
   - Store in YAML config file (similar to `user_config.yaml` pattern)
   - Use existing config patterns from `qa-pms-config` crate
   - Config structure defined in `qa-pms-revenue` crate
   - Storage location: user config directory (same as existing user config)

4. Default values:
   - Global default: `avg_booking_value = 250.00`, `leakage_percentage = 0.05` (5%)
   - Per-integration: optional overrides, fall back to global defaults

5. Encryption (if needed for sensitive data):
   - Use existing `qa-pms-config::Encryptor` (AES-256-GCM)
   - Revenue config values are not sensitive (financial metrics, not credentials)
   - Encryption not required for this story (can be added later if needed)

6. Configuration loading:
   - Load from YAML config file
   - Support default values when config missing
   - Validate config values (leakage_percentage: 0.03-0.07 range)

---

## Acceptance Criteria

- [ ] **Given** `qa-pms-revenue` crate created  
  **When** compiling workspace  
  **Then** crate compiles without errors

- [ ] **Given** config system implemented  
  **When** loading config with valid YAML  
  **Then** revenue metrics are loaded correctly

- [ ] **Given** config system implemented  
  **When** config file missing  
  **Then** default values are used (avg_booking_value = $250, leakage_percentage = 0.05)

- [ ] **Given** config system implemented  
  **When** per-integration config exists  
  **Then** per-integration values are used, falling back to defaults for missing integrations

- [ ] **Given** config system implemented  
  **When** loading config with invalid leakage_percentage (< 0.03 or > 0.07)  
  **Then** validation error is returned with clear message

---

## Tasks

- [ ] Task 1: Create `qa-pms-revenue` crate structure (AC: #1)
  - [ ] 1.1: Create `crates/qa-pms-revenue/Cargo.toml` with dependencies
  - [ ] 1.2: Add crate to workspace `Cargo.toml`
  - [ ] 1.3: Create `crates/qa-pms-revenue/src/lib.rs` with module exports
  - [ ] 1.4: Create `crates/qa-pms-revenue/src/types.rs` placeholder
  - [ ] 1.5: Create `crates/qa-pms-revenue/src/config.rs` placeholder
  - [ ] 1.6: Verify crate compiles in workspace

- [ ] Task 2: Define revenue config types (AC: #2, #3)
  - [ ] 2.1: Create `RevenueConfig` struct with global defaults
  - [ ] 2.2: Create `IntegrationRevenueConfig` struct for per-integration overrides
  - [ ] 2.3: Add serde Serialize/Deserialize with camelCase
  - [ ] 2.4: Define default values (avg_booking_value = 250.00, leakage_percentage = 0.05)
  - [ ] 2.5: Add validation for leakage_percentage range (0.03-0.07)

- [ ] Task 3: Implement config loading (AC: #2, #3, #4)
  - [ ] 3.1: Create `load_revenue_config()` function
  - [ ] 3.2: Implement YAML file loading (use `serde_yaml`)
  - [ ] 3.3: Implement default values when config missing
  - [ ] 3.4: Implement per-integration override resolution
  - [ ] 3.5: Add config validation (leakage_percentage range)

- [ ] Task 4: Export types and functions (AC: #1)
  - [ ] 4.1: Export `RevenueConfig`, `IntegrationRevenueConfig` from `lib.rs`
  - [ ] 4.2: Export `load_revenue_config()` function
  - [ ] 4.3: Add documentation comments
  - [ ] 4.4: Verify exports available for API crate

- [ ] Task 5: Add workspace dependency (AC: #1)
  - [ ] 5.1: Add `qa-pms-revenue` to `crates/qa-pms-api/Cargo.toml` (for future use)
  - [ ] 5.2: Verify workspace compiles with new dependency

---

## Files to Create

| File | Changes |
|------|---------|
| `crates/qa-pms-revenue/Cargo.toml` | Create new crate with dependencies: `serde`, `serde_yaml`, `anyhow`, `thiserror` (for errors) |
| `crates/qa-pms-revenue/src/lib.rs` | Export modules: `types`, `config` |
| `crates/qa-pms-revenue/src/types.rs` | Define `RevenueConfig`, `IntegrationRevenueConfig` types |
| `crates/qa-pms-revenue/src/config.rs` | Implement config loading, validation, default values |

---

## Files to Modify

| File | Changes |
|------|---------|
| `Cargo.toml` (workspace root) | Add `qa-pms-revenue` to `[workspace.members]` |
| `crates/qa-pms-api/Cargo.toml` | Add `qa-pms-revenue` dependency (for future use in Story 23.2) |

---

## Implementation Notes

### Crate Structure Pattern

Follow existing crate patterns (e.g., `qa-pms-dashboard`, `qa-pms-patterns`):

```
crates/qa-pms-revenue/
├── Cargo.toml
└── src/
    ├── lib.rs          # Module exports
    ├── types.rs        # Revenue config types
    └── config.rs       # Config loading logic
```

### Config Type Structure

```rust
// qa-pms-revenue/src/types.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevenueConfig {
    /// Global default average booking value
    pub avg_booking_value: f64,
    /// Global default leakage percentage (0.03-0.07)
    pub leakage_percentage: f64,
    /// Per-integration overrides (optional)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub integrations: HashMap<String, IntegrationRevenueConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationRevenueConfig {
    /// Override average booking value for this integration
    pub avg_booking_value: Option<f64>,
    /// Override leakage percentage for this integration (0.03-0.07)
    pub leakage_percentage: Option<f64>,
}
```

### Default Values

- Global defaults: `avg_booking_value = 250.00`, `leakage_percentage = 0.05` (5%)
- Per-integration: Optional overrides, fall back to global defaults
- Validation: `leakage_percentage` must be between 0.03 (3%) and 0.07 (7%)

### Config File Location

- Store in user config directory (same location as `user_config.yaml`)
- Config file name: `revenue_config.yaml` (or integrated into user config - clarify during implementation)
- Note: Architecture says "stored in YAML config (encrypted)" but revenue metrics are not sensitive, so encryption not required for this story

### Configuration Loading Pattern

```rust
// qa-pms-revenue/src/config.rs
use anyhow::{Context, Result};
use std::path::Path;

pub fn load_revenue_config(config_path: &Path) -> Result<RevenueConfig> {
    // Load from YAML, or return defaults if file doesn't exist
    // Validate values, resolve per-integration overrides
}

pub fn get_integration_config(
    config: &RevenueConfig,
    integration_id: &str,
) -> (f64, f64) {
    // Resolve config for specific integration (override or default)
}
```

### Error Handling

- Use `anyhow::Result` for internal errors
- Use `thiserror` for config validation errors (if needed)
- Return clear error messages for invalid config values

### Dependencies

**Cargo.toml dependencies:**
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
anyhow = "1.0"
thiserror = "1.0"  # Optional, for validation errors
```

---

## Testing Strategy

### Unit Tests

- **Config Loading**: Test loading from YAML file
- **Default Values**: Test default values when config missing
- **Per-Integration Overrides**: Test override resolution
- **Validation**: Test leakage_percentage range validation (0.03-0.07)
- **Edge Cases**: Test invalid config values, missing fields

### Integration Tests

- **Workspace Compilation**: Verify crate compiles in workspace
- **Type Exports**: Verify types exported correctly
- **Config File Loading**: Test loading from actual YAML file

### Manual Tests

- Create sample `revenue_config.yaml` file
- Verify config loads correctly
- Verify default values work when config missing
- Verify per-integration overrides work
- Verify validation errors are clear

---

## Success Metrics

- Crate created and compiles successfully
- Config types defined with correct structure
- Config loading works with defaults
- Per-integration overrides work correctly
- Validation works for invalid values
- Ready for Story 23.2 (Revenue Impact Calculation Engine)

---

## Context & Dependencies

**Dependencies:**
- None (this is the first story in Epic 23)
- Uses existing patterns from `qa-pms-config`, `qa-pms-dashboard` crates

**Enables:**
- Story 23.2: Revenue Impact Calculation Engine (needs config structure)
- Story 23.3: Revenue Impact API Endpoint (needs config for calculations)

**Related Documentation:**
- Architecture Document: `_bmad-output/planning-artifacts/architecture-observability-pms-integrations-2026-01-10.md`
- ADR-002: Revenue Impact Calculation Strategy
- Epic Document: `_bmad-output/planning-artifacts/epics-observability-pms-integrations-2026-01-10.md`
- PRD: `_bmad-output/planning-artifacts/prd-observability-pms-integrations-2026-01-10.md` (FR-2.2)

**Project Context:**
- See `_bmad-output/planning-artifacts/project-context.md` for Rust patterns, error handling, crate structure
- Follow existing patterns from `qa-pms-config/src/user_config.rs` for config structure
- Use `serde` with `camelCase` for JSON serialization (API compatibility)

---

## Dev Notes

### Architecture Alignment

- **Crate Structure**: Follow existing crate patterns (`qa-pms-dashboard`, `qa-pms-patterns`)
- **Error Handling**: Use `anyhow` for internal errors, `thiserror` for validation errors (optional)
- **Config Storage**: YAML file in user config directory (similar to `user_config.yaml`)
- **Encryption**: Not required for revenue metrics (not sensitive data)

### Key Implementation Details

1. **Config Structure**: Define in `qa-pms-revenue` crate, not in `qa-pms-config` (follows architecture decision)
2. **Default Values**: Must provide sensible defaults (avg_booking_value = $250, leakage_percentage = 5%)
3. **Per-Integration Overrides**: Support optional overrides for each integration (booking-com, airbnb, vrbo, hmbn)
4. **Validation**: Validate leakage_percentage range (3-7% as per PRD requirements)

### Integration with Existing Code

- Add crate to workspace `Cargo.toml`
- Add dependency to `qa-pms-api/Cargo.toml` for future use (Story 23.2)
- Follow existing patterns from `qa-pms-config` for config loading
- Use `serde_yaml` for YAML parsing (consistent with existing codebase)

### Testing Approach

- Unit tests for config loading, defaults, overrides, validation
- Integration tests for workspace compilation, type exports
- Manual tests with sample YAML file

---

**Story Status:** `ready-for-dev`  
**Last Updated:** 2026-01-11  
**Next Review:** When moving to `in-progress`
