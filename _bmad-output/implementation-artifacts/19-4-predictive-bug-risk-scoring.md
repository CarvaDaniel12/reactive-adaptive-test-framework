# Story 19.4: Predictive Bug Risk Scoring

Status: ready-for-dev

## Story

**As a** QA Engineer  
**I want** predictive bug risk scoring  
**So that** I can prioritize testing efforts based on bug risk

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 19.4 |
| Epic | Epic 19: Advanced Features |
| Sprint | Sprint 2: Predictive Analytics |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Epic 9 (Pattern Detection) - Complete ✅ |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create bug risk scoring algorithm
   - Analyze ticket metadata (type, priority, component)
   - Analyze ticket history (similar bugs, component failure rate)
   - Analyze integration health (integration failure rates)
   - Calculate risk score (0-100)

2. Implement risk scoring service
   - Create `qa-pms-risk` crate (or add to existing)
   - Implement risk calculation logic
   - Support multiple risk factors
   - Weight risk factors

3. Store risk scores
   - Store risk scores with tickets
   - Update risk scores when data changes
   - Historical risk score tracking (optional)

4. Display risk scores
   - Show risk score on ticket cards
   - Color-code by risk level (low, medium, high, critical)
   - Show risk factors breakdown
   - Filter/sort by risk score

---

## Acceptance Criteria

- [ ] **Given** risk scoring exists  
  **When** calculating risk for ticket  
  **Then** risk score is calculated (0-100)

- [ ] **Given** risk scoring exists  
  **When** viewing ticket  
  **Then** risk score is displayed with color coding

- [ ] **Given** risk scoring exists  
  **When** viewing risk score  
  **Then** risk factors breakdown is displayed

- [ ] **Given** risk scoring exists  
  **When** filtering by risk  
  **Then** tickets are filtered by risk score

---

## Tasks / Subtasks

- [ ] Task 1: Create risk scoring service
  - [ ] 1.1: Create `crates/qa-pms-risk/Cargo.toml` (or use existing crate)
  - [ ] 1.2: Create `crates/qa-pms-risk/src/lib.rs`
  - [ ] 1.3: Create `crates/qa-pms-risk/src/risk_calculator.rs`
  - [ ] 1.4: Implement risk calculation algorithm

- [ ] Task 2: Implement risk factors
  - [ ] 2.1: Ticket type risk factor (bug > feature)
  - [ ] 2.2: Ticket priority risk factor (P0 > P1 > P2)
  - [ ] 2.3: Component failure rate risk factor
  - [ ] 2.4: Integration health risk factor
  - [ ] 2.5: Historical bug pattern risk factor

- [ ] Task 3: Create risk score types
  - [ ] 3.1: Create `RiskScore` struct (score, factors)
  - [ ] 3.2: Create `RiskLevel` enum (low, medium, high, critical)
  - [ ] 3.3: Create `RiskFactor` struct (name, weight, value)
  - [ ] 3.4: Add serde serialization

- [ ] Task 4: Integrate with tickets
  - [ ] 4.1: Calculate risk when ticket is loaded
  - [ ] 4.2: Cache risk scores (optional)
  - [ ] 4.3: Update risk scores periodically

- [ ] Task 5: Display risk scores
  - [ ] 5.1: Update `frontend/src/pages/Tickets/TicketCard.tsx`
  - [ ] 5.2: Display risk score badge
  - [ ] 5.3: Color-code by risk level
  - [ ] 5.4: Show risk factors breakdown

---

## Files to Create

| File | Changes |
|------|---------|
| `crates/qa-pms-risk/Cargo.toml` | Create risk scoring crate |
| `crates/qa-pms-risk/src/lib.rs` | Create crate root |
| `crates/qa-pms-risk/src/risk_calculator.rs` | Create risk calculator |
| `crates/qa-pms-risk/src/types.rs` | Create risk types |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-api/src/routes/tickets.rs` | Calculate and return risk scores |
| `frontend/src/pages/Tickets/TicketCard.tsx` | Display risk scores |

---

## Dev Notes

### Risk Scoring Algorithm

**Risk Factors:**
- Ticket type: Bug (high), Feature (medium), Story (low)
- Ticket priority: P0 (high), P1 (medium), P2 (low)
- Component failure rate: Based on historical bugs
- Integration health: Based on integration health monitoring (Epic 22)
- Historical patterns: Based on pattern detection (Epic 9)

**Risk Score Calculation:**
```rust
pub struct RiskScore {
    pub score: f64,        // 0.0 to 100.0
    pub level: RiskLevel,  // low, medium, high, critical
    pub factors: Vec<RiskFactor>,
}

pub enum RiskLevel {
    Low,        // 0-25
    Medium,     // 26-50
    High,       // 51-75
    Critical,   // 76-100
}
```

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 19, Story 19.4)
- Dependency: Epic 9 (Pattern Detection) - must be complete
- Integration Health: Epic 22 (Integration Health Monitoring) - reference
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### File List
