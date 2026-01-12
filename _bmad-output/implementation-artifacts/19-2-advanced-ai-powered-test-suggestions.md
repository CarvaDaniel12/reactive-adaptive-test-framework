# Story 19.2: Advanced AI-Powered Test Suggestions

Status: ready-for-dev

## Story

**As a** QA Engineer  
**I want** advanced AI-powered test suggestions  
**So that** I can get better test recommendations based on ticket context

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 19.2 |
| Epic | Epic 19: Advanced Features |
| Sprint | Sprint 1: AI Enhancements |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Epic 13 (AI Companion) - Complete ✅ |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Enhance existing AI test suggestions
   - Improve semantic search with context awareness
   - Use ticket metadata (priority, type, component) for better suggestions
   - Analyze ticket history for pattern matching
   - Generate test suggestions based on similar tickets

2. Implement advanced suggestion algorithms
   - Context-aware suggestion scoring
   - Relevance ranking based on multiple factors
   - Confidence scoring for suggestions
   - Filter suggestions by confidence threshold

3. Create suggestion explanation
   - Explain why suggestions are relevant
   - Show matching criteria
   - Display confidence scores
   - Provide suggestion sources

4. Track suggestion effectiveness
   - Track which suggestions are used
   - Collect feedback on suggestion quality
   - Improve suggestions based on usage data
   - A/B testing for suggestion algorithms (future)

---

## Acceptance Criteria

- [ ] **Given** advanced AI suggestions exist  
  **When** requesting test suggestions  
  **Then** context-aware suggestions are returned

- [ ] **Given** advanced AI suggestions exist  
  **When** viewing suggestions  
  **Then** explanations and confidence scores are displayed

- [ ] **Given** advanced AI suggestions exist  
  **When** suggestions are used  
  **Then** usage is tracked for improvement

- [ ] **Given** advanced AI suggestions exist  
  **When** filtering by confidence  
  **Then** only high-confidence suggestions are shown

---

## Tasks / Subtasks

- [ ] Task 1: Enhance AI suggestion service
  - [ ] 1.1: Update `crates/qa-pms-ai/src/semantic.rs`
  - [ ] 1.2: Add context-aware suggestion logic
  - [ ] 1.3: Implement relevance scoring
  - [ ] 1.4: Add confidence calculation

- [ ] Task 2: Create suggestion explanation
  - [ ] 2.1: Create `SuggestionExplanation` type
  - [ ] 2.2: Generate explanations for suggestions
  - [ ] 2.3: Include matching criteria
  - [ ] 2.4: Include confidence scores

- [ ] Task 3: Integrate with ticket context
  - [ ] 3.1: Extract ticket metadata (priority, type, component)
  - [ ] 3.2: Use metadata in suggestion algorithm
  - [ ] 3.3: Analyze ticket history for patterns

- [ ] Task 4: Track suggestion usage
  - [ ] 4.1: Create `suggestion_usage` table (optional, future)
  - [ ] 4.2: Track suggestion clicks/usage
  - [ ] 4.3: Collect feedback on suggestions

- [ ] Task 5: Update frontend suggestion UI
  - [ ] 5.1: Update `frontend/src/components/search/RelatedTests.tsx`
  - [ ] 5.2: Display suggestion explanations
  - [ ] 5.3: Display confidence scores
  - [ ] 5.4: Add confidence filter

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-ai/src/semantic.rs` | Enhance suggestion algorithm |
| `frontend/src/components/search/RelatedTests.tsx` | Update suggestion display |

---

## Dev Notes

### Enhanced Suggestion Algorithm

**Context Factors:**
- Ticket priority (P0 > P1 > P2)
- Ticket type (bug, feature, story)
- Component/area affected
- Historical patterns (similar tickets)
- Test coverage gaps

**Scoring:**
```rust
pub struct SuggestionScore {
    pub relevance: f64,      // 0.0 to 1.0
    pub confidence: f64,     // 0.0 to 1.0
    pub explanation: String, // Why this suggestion
    pub factors: Vec<SuggestionFactor>, // Matching criteria
}
```

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 19, Story 19.2)
- Dependency: Epic 13 (AI Companion) - must be complete
- AI Patterns: `qa-intelligent-pms/crates/qa-pms-ai/` (reference)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### File List
