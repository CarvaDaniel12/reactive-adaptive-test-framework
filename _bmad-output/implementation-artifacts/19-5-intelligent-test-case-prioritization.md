# Story 19.5: Intelligent Test Case Prioritization

Status: ready-for-dev

## Story

**As a** QA Engineer  
**I want** intelligent test case prioritization  
**So that** I can run most important tests first

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 19.5 |
| Epic | Epic 19: Advanced Features |
| Sprint | Sprint 2: Predictive Analytics |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Epic 9 (Pattern Detection) - Complete ✅ |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Implement test prioritization algorithm
   - Prioritize by ticket risk score (Story 19.4)
   - Prioritize by test type (API > UI for integration tickets)
   - Prioritize by test execution history (frequently failing tests)
   - Prioritize by coverage gaps

2. Create prioritization service
   - Create or enhance `qa-pms-risk` crate
   - Calculate priority scores
   - Rank test cases by priority
   - Support custom prioritization rules

3. Integrate with test case management
   - Use existing TestCase data model (Story 31.1)
   - Update priority calculation
   - Store priority scores
   - Display prioritized test lists

4. Create prioritized test run views
   - Display tests sorted by priority
   - Group by priority level
   - Show priority factors
   - Allow manual priority adjustment

---

## Acceptance Criteria

- [ ] **Given** test prioritization exists  
  **When** calculating priority for tests  
  **Then** tests are ranked by priority score

- [ ] **Given** test prioritization exists  
  **When** viewing test list  
  **Then** tests are displayed sorted by priority

- [ ] **Given** test prioritization exists  
  **When** viewing priority details  
  **Then** priority factors are displayed

- [ ] **Given** test prioritization exists  
  **When** adjusting priority manually  
  **Then** priority is updated and respected

---

## Tasks / Subtasks

- [ ] Task 1: Create prioritization service
  - [ ] 1.1: Create `crates/qa-pms-risk/src/prioritizer.rs`
  - [ ] 1.2: Implement priority calculation algorithm
  - [ ] 1.3: Support multiple priority factors
  - [ ] 1.4: Weight priority factors

- [ ] Task 2: Implement priority factors
  - [ ] 2.1: Ticket risk score factor (Story 19.4)
  - [ ] 2.2: Test type factor (API > UI for integration)
  - [ ] 2.3: Test execution history factor (failure rate)
  - [ ] 2.4: Coverage gap factor

- [ ] Task 3: Create priority types
  - [ ] 3.1: Create `TestPriorityScore` struct
  - [ ] 3.2: Create `PriorityFactor` struct
  - [ ] 3.3: Add serde serialization

- [ ] Task 4: Integrate with test cases
  - [ ] 4.1: Calculate priority when tests are loaded
  - [ ] 4.2: Store priority scores
  - [ ] 4.3: Update priority periodically

- [ ] Task 5: Display prioritized tests
  - [ ] 5.1: Update test list components to sort by priority
  - [ ] 5.2: Display priority scores
  - [ ] 5.3: Show priority factors breakdown

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-risk/src/prioritizer.rs` | Create prioritization logic |
| `crates/qa-pms-core/src/types.rs` | Add priority score to TestCase (if needed) |

---

## Dev Notes

### Test Prioritization Algorithm

**Priority Factors:**
- Ticket risk score (from Story 19.4)
- Test type (API > Integration > UI for integration tickets)
- Test execution history (failure rate, last execution)
- Coverage gaps (untested scenarios)

**Priority Score:**
```rust
pub struct TestPriorityScore {
    pub score: f64,        // 0.0 to 100.0
    pub factors: Vec<PriorityFactor>,
}
```

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 19, Story 19.5)
- Dependency: Epic 9 (Pattern Detection) - must be complete
- Dependency: Story 19.4 (Predictive Bug Risk Scoring) - recommended
- Test Cases: Story 31.1 (Auto-Test Generation) - reference
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### File List
