# Story 24.5: Correlation Alerts (Optional)

Status: ready-for-dev

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

**As a** QA Engineer working on PMS integration quality  
**I want** alerts when test failures have high correlation with integration health issues  
**So that** I can be notified proactively when test failures indicate potential integration problems

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 24.5 |
| Epic | Epic 24: Test-Integration Correlation Engine |
| Sprint | Sprint 2: Correlation API and Dashboard |
| Priority | P2 (Optional) |
| Estimated Days | 1 |
| Dependencies | Story 24.4 (Correlation Dashboard View) |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Alert generation logic
   - Alert when test failure has high correlation (> 0.8) with integration failure
   - Generate alert from correlation data
   - Use existing alert system (Epic 9: `qa-pms-patterns`)

2. Alert content
   - Alert includes: test case, integration, correlation score, recommendation
   - Alert severity: based on correlation score (high = critical, medium = warning, low = info)
   - Alert type: "test_integration_correlation"

3. Alert display
   - Alert displayed in dashboard (AlertBell component)
   - Alert can be dismissed
   - Alert history stored (uses existing alert tables)

4. Integration with existing alert system
   - Use `qa-pms-patterns::AlertService` for alert generation
   - Use existing `alerts` table from Epic 9
   - Follow existing alert patterns

---

## Acceptance Criteria

- [ ] **Given** correlation system exists  
  **When** high correlation (> 0.8) detected  
  **Then** alert is generated and stored in alerts table

- [ ] **Given** alert exists  
  **When** alert is displayed  
  **Then** alert includes test case, integration, correlation score, recommendation

- [ ] **Given** alert exists  
  **When** alert is dismissed  
  **Then** alert is marked as dismissed (is_dismissed = true)

- [ ] **Given** alert exists  
  **When** alert is stored  
  **Then** alert history is stored in alerts table with all required fields

---

## Tasks / Subtasks

- [ ] Task 1: Add alert generation to correlation engine (AC: #1)
  - [ ] 1.1: Create `generate_correlation_alert` method in `qa-pms-correlation/src/engine.rs`
  - [ ] 1.2: Check if correlation score > 0.8 (high correlation threshold)
  - [ ] 1.3: Create `NewAlert` with correlation data:
    - `alert_type`: "test_integration_correlation"
    - `severity`: based on correlation score (high=critical, medium=warning, low=info)
    - `title`: "High correlation: {test_case_name} with {integration_id}"
    - `message`: correlation details (score, pattern, confidence)
    - `affected_tickets`: test case ID
    - `suggested_actions`: recommendations based on correlation
  - [ ] 1.4: Use `qa-pms-patterns::AlertService` to create alert

- [ ] Task 2: Integrate alert generation with correlation calculation (AC: #1)
  - [ ] 2.1: Modify correlation engine to check for high correlations after calculation
  - [ ] 2.2: Generate alerts for correlations with score > 0.8
  - [ ] 2.3: Handle alert generation errors gracefully (log warning, don't fail correlation)

- [ ] Task 3: Add correlation alert type support (AC: #1, #2)
  - [ ] 3.1: Verify `qa-pms-patterns::AlertType` supports custom alert types (or extend if needed)
  - [ ] 3.2: Define "test_integration_correlation" as alert type
  - [ ] 3.3: Update alert display logic if needed to support correlation alerts

- [ ] Task 4: Test alert generation (AC: #1, #2, #3, #4)
  - [ ] 4.1: Unit tests: Test alert generation with high correlation (> 0.8)
  - [ ] 4.2: Unit tests: Test alert generation with medium correlation (0.6-0.8)
  - [ ] 4.3: Unit tests: Test alert generation with low correlation (< 0.6) - should not generate
  - [ ] 4.4: Integration tests: Test alert creation in database
  - [ ] 4.5: Integration tests: Test alert display in AlertBell component

- [ ] Task 5: Manual testing (AC: #1, #2, #3, #4)
  - [ ] 5.1: Verify alerts are generated when high correlation detected
  - [ ] 5.2: Verify alert content is correct (test case, integration, score, recommendation)
  - [ ] 5.3: Verify alerts are displayed in AlertBell component
  - [ ] 5.4: Verify alerts can be dismissed
  - [ ] 5.5: Verify alert history is stored

---

## Files to Create

| File | Changes |
|------|---------|
| None | Use existing alert system |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-correlation/src/engine.rs` | Add alert generation method |
| `crates/qa-pms-correlation/src/lib.rs` | Export alert generation if needed |
| `crates/qa-pms-correlation/Cargo.toml` | Add `qa-pms-patterns` dependency (if not already present) |

---

## Dev Notes

### Alert Generation Logic

**Threshold:**
- High correlation: score > 0.8 → Generate critical alert
- Medium correlation: score 0.6-0.8 → Generate warning alert (optional, configurable)
- Low correlation: score < 0.6 → No alert

**Alert Content:**
```rust
let alert = NewAlert {
    pattern_id: None, // No pattern ID for correlation alerts
    alert_type: PatternType::Other, // Or custom type if supported
    severity: match correlation_score {
        score if score > 0.8 => Severity::Critical,
        score if score > 0.6 => Severity::Warning,
        _ => Severity::Info,
    },
    title: format!("High correlation: {} with {}", test_case_name, integration_id),
    message: Some(format!(
        "Test case '{}' shows {} correlation (score: {:.2}, confidence: {:.2}) with {} integration failures.\nPattern: {}",
        test_case_name,
        correlation_type,
        correlation_score,
        confidence,
        integration_id,
        pattern
    )),
    affected_tickets: vec![test_case_id.clone()],
    suggested_actions: vec![
        format!("Investigate test case {} for potential integration issues", test_case_id),
        format!("Monitor {} integration health closely", integration_id),
        format!("Consider prioritizing fixes for test case {} (high correlation)", test_case_id),
    ],
};
```

### Integration with Alert System

**Use Existing Alert Service:**
```rust
use qa_pms_patterns::{AlertService, PatternRepository};

// In correlation engine
pub async fn check_and_alert_high_correlations(
    &self,
    correlations: Vec<Correlation>,
    alert_service: &AlertService,
) -> Result<(), CorrelationError> {
    for correlation in correlations {
        if correlation.correlation_score > 0.8 {
            let alert = self.create_correlation_alert(correlation)?;
            alert_service.generate_alert_from_new_alert(alert).await
                .map_err(|e| CorrelationError::Internal(anyhow::anyhow!("Failed to create alert: {}", e)))?;
        }
    }
    Ok(())
}
```

**Note:** This assumes `AlertService` has a method to create alerts from `NewAlert`. If not, we may need to extend the alert service or create alerts directly via repository.

### Alert Display

**Existing Alert System:**
- Alerts are displayed via `AlertBell` component (frontend)
- Alerts are fetched via `/api/v1/alerts` endpoint (backend)
- Alerts use `alerts` table from Epic 9

**No Frontend Changes Needed:**
- Existing AlertBell component should display correlation alerts automatically
- Alert content (title, message, suggested_actions) will be displayed as-is
- Alert dismissal works via existing API endpoints

### Project Structure Notes

**Backend Patterns:**
- Follow existing alert generation patterns (Epic 9: `qa-pms-patterns`)
- Use `AlertService` from `qa-pms-patterns` for alert creation
- Use `PatternRepository` if direct database access needed
- Handle errors gracefully (don't fail correlation if alert generation fails)

**Dependencies:**
- `qa-pms-patterns`: Alert service and repository (Epic 9)
- `qa-pms-correlation`: Correlation engine (Story 24.1)

**Alert Type:**
- Use existing `PatternType` enum or extend if needed
- Or use `PatternType::Other` if custom types not supported
- Alert type string: "test_integration_correlation"

### Testing Standards

**Unit Tests:**
- Test alert generation logic with various correlation scores
- Test alert content generation
- Test threshold logic (> 0.8 = alert, < 0.8 = no alert)

**Integration Tests:**
- Test alert creation in database
- Test alert service integration
- Test alert display in AlertBell component

**Manual Tests:**
- Verify alerts are generated when high correlation detected
- Verify alert content is correct
- Verify alerts are displayed correctly
- Verify alerts can be dismissed

### References

- Epic Document: `_bmad-output/planning-artifacts/epics-observability-pms-integrations-2026-01-10.md` (Epic 24, Story 24.5)
- Architecture Document: `_bmad-output/planning-artifacts/architecture-observability-pms-integrations-2026-01-10.md` (Section 6.1: Backend Architecture)
- Alert System: `qa-intelligent-pms/crates/qa-pms-patterns/src/alerts.rs` (reference)
- Alert Service: `qa-intelligent-pms/crates/qa-pms-patterns/src/alerts.rs::AlertService` (reference)
- Alert API: `qa-intelligent-pms/crates/qa-pms-api/src/routes/alerts.rs` (reference)
- Dependency: Story 24.4 (Correlation Dashboard View) - must be completed first
- Dependency: Epic 9 (Pattern Detection & Alerts) - alert system must exist
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
