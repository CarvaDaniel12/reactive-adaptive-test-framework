# Story 23.2: Revenue Impact Calculation Engine

**Status:** `ready-for-dev`

## Story

**As a** QA Engineer working on PMS integration quality  
**I want** revenue impact to be calculated from integration health data  
**So that** I can understand the financial impact of integration failures and prioritize work accordingly

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 23.2 |
| Epic | Epic 23: Revenue Impact Calculator and Dashboard |
| Sprint | Sprint 1: Revenue Calculation Engine |
| Priority | P0 |
| Estimated Days | 2 |
| Dependencies | Story 23.1 (Revenue Configuration), Epic 22 Story 22.4 (Integration Health Service) |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create `calculator.rs` module in `qa-pms-revenue` crate
   - Implement revenue calculation logic
   - Use config from Story 23.1 (`RevenueConfig`)
   - Use integration health data from Epic 22 (`qa-pms-integration-health`)

2. Calculate revenue loss from pricing sync errors:
   - Formula: `pricing_errors * avg_booking_value * leakage_percentage`
   - Use `pricing_sync_status` from integration health data
   - Count errors when status is "error" or "critical"

3. Calculate revenue loss from fees sync errors:
   - Formula: `fees_errors * avg_booking_value * leakage_percentage`
   - Use `fees_sync_status` from integration health data
   - Count errors when status is "error" or "critical"

4. Calculate revenue loss from booking loss:
   - Formula: `booking_loss_rate * avg_booking_value * total_bookings_estimate`
   - Use `booking_loss_rate` from integration health data (0.0 to 1.0)
   - Estimate total bookings from integration events or use default multiplier

5. Support different calculation methods per integration type:
   - Use per-integration config overrides from Story 23.1
   - Fall back to global defaults when per-integration config missing
   - Support custom multipliers per integration if needed

6. Calculations must be accurate within 5%:
   - Validate calculation results against expected values
   - Use precise decimal arithmetic (consider `rust_decimal` for financial calculations)
   - Round to 2 decimal places for currency values

7. Integration with integration health service:
   - Use `IntegrationHealthService` from Epic 22 Story 22.4
   - Query health data for period (7d, 30d, 90d, 1y)
   - Aggregate revenue impact across all integrations

---

## Acceptance Criteria

- [ ] **Given** calculation engine implemented  
  **When** calculating revenue loss from pricing sync errors  
  **Then** revenue loss is calculated correctly using formula: `errors * avg_booking_value * leakage_percentage`

- [ ] **Given** calculation engine implemented  
  **When** calculating revenue loss from fees sync errors  
  **Then** revenue loss is calculated correctly using formula: `errors * avg_booking_value * leakage_percentage`

- [ ] **Given** calculation engine implemented  
  **When** calculating revenue loss from booking loss  
  **Then** revenue loss is calculated correctly using formula: `booking_loss_rate * avg_booking_value * bookings_estimate`

- [ ] **Given** calculation engine implemented  
  **When** using config values from Story 23.1  
  **Then** calculations use config values (global defaults or per-integration overrides)

- [ ] **Given** calculation engine implemented  
  **When** per-integration config exists  
  **Then** per-integration calculations are used, falling back to defaults for missing integrations

- [ ] **Given** calculation engine implemented  
  **When** comparing calculated values with expected values  
  **Then** calculations are accurate within 5% tolerance

---

## Tasks

- [ ] Task 1: Create calculator module structure (AC: #1, #2, #3)
  - [ ] 1.1: Create `crates/qa-pms-revenue/src/calculator.rs`
  - [ ] 1.2: Create `crates/qa-pms-revenue/src/error.rs` for calculation errors
  - [ ] 1.3: Add dependencies to `Cargo.toml`: `qa-pms-integration-health`, `qa-pms-dashboard` (for KPIMetric)
  - [ ] 1.4: Export calculator from `lib.rs`

- [ ] Task 2: Implement pricing sync error calculation (AC: #1)
  - [ ] 2.1: Create `calculate_pricing_sync_revenue_loss()` function
  - [ ] 2.2: Query integration health data for pricing sync errors
  - [ ] 2.3: Apply formula: `errors * avg_booking_value * leakage_percentage`
  - [ ] 2.4: Use per-integration config if available, else global defaults
  - [ ] 2.5: Return revenue loss as f64 (currency value)

- [ ] Task 3: Implement fees sync error calculation (AC: #2)
  - [ ] 3.1: Create `calculate_fees_sync_revenue_loss()` function
  - [ ] 3.2: Query integration health data for fees sync errors
  - [ ] 3.3: Apply formula: `errors * avg_booking_value * leakage_percentage`
  - [ ] 3.4: Use per-integration config if available, else global defaults
  - [ ] 3.5: Return revenue loss as f64 (currency value)

- [ ] Task 4: Implement booking loss calculation (AC: #3)
  - [ ] 4.1: Create `calculate_booking_loss_revenue()` function
  - [ ] 4.2: Query integration health data for booking_loss_rate
  - [ ] 4.3: Estimate total bookings (use default multiplier or events count)
  - [ ] 4.4: Apply formula: `booking_loss_rate * avg_booking_value * bookings_estimate`
  - [ ] 4.5: Use per-integration config if available, else global defaults
  - [ ] 4.6: Return revenue loss as f64 (currency value)

- [ ] Task 5: Implement aggregate revenue calculation (AC: #4, #5)
  - [ ] 5.1: Create `calculate_revenue_impact()` function (main entry point)
  - [ ] 5.2: Aggregate revenue loss across all integration types
  - [ ] 5.3: Calculate total revenue at risk (sum of all losses)
  - [ ] 5.4: Calculate revenue protected (estimate based on QA testing effectiveness)
  - [ ] 5.5: Return `RevenueImpact` type (from architecture document)

- [ ] Task 6: Add configuration integration (AC: #4, #5)
  - [ ] 6.1: Integrate with `RevenueConfig` from Story 23.1
  - [ ] 6.2: Load config and resolve per-integration overrides
  - [ ] 6.3: Pass config values to calculation functions
  - [ ] 6.4: Handle missing config (use defaults)

- [ ] Task 7: Add integration health service integration (AC: #1, #2, #3)
  - [ ] 7.1: Integrate with `IntegrationHealthService` from Epic 22
  - [ ] 7.2: Query health data for specified period
  - [ ] 7.3: Extract error counts and booking loss rates
  - [ ] 7.4: Handle missing health data gracefully

- [ ] Task 8: Implement accuracy validation (AC: #6)
  - [ ] 8.1: Add unit tests with known input/output values
  - [ ] 8.2: Validate calculations within 5% tolerance
  - [ ] 8.3: Test edge cases (zero values, negative values, large numbers)
  - [ ] 8.4: Use precise decimal arithmetic for financial calculations

---

## Files to Create

| File | Changes |
|------|---------|
| `crates/qa-pms-revenue/src/calculator.rs` | Implement revenue calculation logic (pricing sync, fees sync, booking loss) |
| `crates/qa-pms-revenue/src/error.rs` | Define `RevenueCalculationError` using `thiserror` |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-revenue/src/lib.rs` | Export `calculator` module, `RevenueCalculationError` |
| `crates/qa-pms-revenue/Cargo.toml` | Add dependencies: `qa-pms-integration-health`, `qa-pms-dashboard` (for KPIMetric), `anyhow`, `thiserror` |

---

## Implementation Notes

### Revenue Calculation Formulas

**Pricing Sync Errors:**
```
revenue_loss = error_count * avg_booking_value * leakage_percentage
```

**Fees Sync Errors:**
```
revenue_loss = error_count * avg_booking_value * leakage_percentage
```

**Booking Loss:**
```
revenue_loss = booking_loss_rate * avg_booking_value * bookings_estimate
```

Where:
- `error_count`: Number of errors from integration health data
- `avg_booking_value`: From config (default: $250)
- `leakage_percentage`: From config (default: 0.05 = 5%)
- `booking_loss_rate`: From integration health data (0.0 to 1.0)
- `bookings_estimate`: Estimated total bookings (use default multiplier or events count)

### Integration Health Service Usage

Use `IntegrationHealthService` from Epic 22 Story 22.4:
- Query health data: `get_health_status(integration_id, period)`
- Extract error counts from `pricing_sync_status`, `fees_sync_status`
- Extract `booking_loss_rate` from health data
- Aggregate across all integrations (booking-com, airbnb, vrbo, hmbn)

### Revenue Impact Type

From architecture document, use:
```rust
// qa-pms-revenue/src/types.rs (add to existing types)
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
    pub trend: String, // "up", "down", "neutral"
}
```

### Configuration Integration

- Load `RevenueConfig` from Story 23.1
- Resolve per-integration config: `get_integration_config(config, integration_id)`
- Use per-integration `avg_booking_value` and `leakage_percentage` if available
- Fall back to global defaults if per-integration config missing

### Error Handling

Use `thiserror` for calculation errors:
```rust
#[derive(Error, Debug)]
pub enum RevenueCalculationError {
    #[error("Configuration error: {0}")]
    Config(#[from] anyhow::Error),
    
    #[error("Integration health data error: {0}")]
    IntegrationHealth(#[from] qa_pms_integration_health::IntegrationHealthError),
    
    #[error("Invalid calculation input: {0}")]
    InvalidInput(String),
}
```

### Testing Strategy

- **Unit Tests**: Calculation functions with known inputs/outputs
- **Accuracy Tests**: Validate calculations within 5% tolerance
- **Edge Cases**: Zero values, negative values, large numbers
- **Integration Tests**: End-to-end with real integration health data
- **Performance Tests**: Calculations complete in < 100ms

---

## Testing Strategy

### Unit Tests

- **Calculation Logic**: Test each calculation function with known inputs/outputs
- **Accuracy Validation**: Compare calculated values with expected values (within 5% tolerance)
- **Edge Cases**: Zero values, negative values, missing data, large numbers
- **Config Integration**: Test with global defaults and per-integration overrides
- **Error Handling**: Test error cases (missing config, missing health data)

### Integration Tests

- **End-to-End**: Calculate revenue impact with real integration health data
- **Service Integration**: Test integration with `IntegrationHealthService`
- **Config Loading**: Test loading config and resolving per-integration values

### Manual Tests

- Calculate revenue impact for sample period (7d, 30d, 90d, 1y)
- Verify calculations are reasonable (not negative, not extremely large)
- Verify accuracy by comparing with manual calculations
- Test performance with large datasets

---

## Success Metrics

- Calculations accurate within 5% tolerance
- Calculations performant (< 100ms)
- Edge cases handled correctly (zero, negative, missing data)
- Integration with config and health service works correctly
- Ready for Story 23.3 (Revenue Impact API Endpoint)

---

## Context & Dependencies

**Dependencies:**
- Story 23.1: Revenue Configuration System (provides `RevenueConfig`)
- Epic 22 Story 22.4: Integration Health Service (provides integration health data)

**Enables:**
- Story 23.3: Revenue Impact API Endpoint (needs calculation engine)

**Related Documentation:**
- Architecture Document: `_bmad-output/planning-artifacts/architecture-observability-pms-integrations-2026-01-10.md`
- ADR-002: Revenue Impact Calculation Strategy
- Epic Document: `_bmad-output/planning-artifacts/epics-observability-pms-integrations-2026-01-10.md`
- PRD: `_bmad-output/planning-artifacts/prd-observability-pms-integrations-2026-01-10.md` (FR-2.1)

**Project Context:**
- See `_bmad-output/planning-artifacts/project-context.md` for Rust patterns, error handling
- Follow existing patterns from `qa-pms-dashboard` for metric calculations
- Use `KPIMetric` from `qa-pms-dashboard` for consistent metric presentation
- Use `anyhow` for internal errors, `thiserror` for API boundaries

---

## Dev Notes

### Key Implementation Details

1. **Calculation Formulas**: Use formulas from architecture document
2. **Financial Precision**: Consider using `rust_decimal` for precise financial calculations (avoid f64 rounding errors)
3. **Config Resolution**: Always resolve per-integration config, fall back to defaults
4. **Error Aggregation**: Aggregate revenue loss across all integration types
5. **Revenue Protected**: Estimate revenue protected based on QA testing effectiveness (can use simple multiplier for now)

### Integration Points

- **Config**: Use `RevenueConfig` from Story 23.1
- **Integration Health**: Use `IntegrationHealthService` from Epic 22
- **Types**: Use `KPIMetric` from `qa-pms-dashboard` for consistency
- **Error Handling**: Use `thiserror` for structured errors

### Performance Considerations

- Calculations should complete in < 100ms
- Consider caching config values if loaded frequently
- Batch query integration health data if possible

---

**Story Status:** `ready-for-dev`  
**Last Updated:** 2026-01-11  
**Next Review:** When moving to `in-progress`
