# Story 31.6: Risk Scoring Algorithms

Status: ready-for-dev

Epic: 31 - AI-Enhanced Automation
Priority: P1 (High Value)
Estimated Effort: 3 days
Sprint: 1

## Story

As a **QA Lead**,
I want to **calculate comprehensive risk scores for releases and features**,
So that **I can make informed decisions about deployment readiness**.

## Acceptance Criteria

1. **Given** I'm preparing for a release
   **When** I request a risk assessment
   **Then** the system analyzes all changes in the release
   **And** calculates an overall risk score (0.0-1.0)
   **And** breaks down risk by category (code_quality, stability, performance, security, test_coverage)
   **And** identifies high-risk items (tickets, components, integrations)
   **And** provides mitigation strategies

2. **Given** I compare two releases
   **When** I view risk comparison
   **Then** I see side-by-side risk metrics
   **And** can identify trends
   **And** understand risk evolution over time

## Tasks / Subtasks

- [ ] Task 1: Create risk scorer service (AC: #1, #2)
  - [ ] 1.1: Create `crates/qa-pms-ai/src/risk_scorer.rs` module
  - [ ] 1.2: Create `RiskScorer` struct with configurable weights
  - [ ] 1.3: Define `RiskAssessment` struct (release_version, overall_score, risk_level, category_scores, high_risk_items, trends, mitigation_strategies)
  - [ ] 1.4: Define `CategoryScores` struct (code_quality, stability, performance, security, test_coverage)
  - [ ] 1.5: Define `RiskWeights` struct (code_quality: 0.3, stability: 0.25, performance: 0.2, security: 0.15, test_coverage: 0.1)

- [ ] Task 2: Implement category risk calculation (AC: #1)
  - [ ] 2.1: Create `calculate_code_quality_risk(tickets, test_results)` method
  - [ ] 2.2: Create `calculate_stability_risk(tickets, test_results)` method
  - [ ] 2.3: Create `calculate_performance_risk(test_results)` method
  - [ ] 2.4: Create `calculate_security_risk(tickets)` method
  - [ ] 2.5: Create `calculate_coverage_risk(test_results)` method
  - [ ] 2.6: Calculate overall score using weighted average

- [ ] Task 3: Implement high-risk item identification (AC: #1)
  - [ ] 3.1: Create `identify_high_risk_items(tickets, test_results)` method
  - [ ] 3.2: Identify high-risk tickets (bug risk score > 0.7)
  - [ ] 3.3: Identify high-risk components (frequent bugs, high complexity)
  - [ ] 3.4: Identify high-risk integrations (API changes, external dependencies)
  - [ ] 3.5: Return list of high-risk items with scores and reasons

- [ ] Task 4: Implement mitigation strategy generation (AC: #1)
  - [ ] 4.1: Create `generate_mitigation_strategies(category_scores)` method
  - [ ] 4.2: Generate strategies for each high-risk category
  - [ ] 4.3: Use AI to suggest mitigation actions (optional, can be heuristics-based)
  - [ ] 4.4: Estimate expected risk reduction for each strategy
  - [ ] 4.5: Return list of mitigation strategies with actions and expected impact

- [ ] Task 5: Implement risk trend analysis (AC: #2)
  - [ ] 5.1: Create `calculate_risk_trends(release_version, historical_data)` method
  - [ ] 5.2: Fetch historical risk assessments
  - [ ] 5.3: Calculate trends by category (improving, stable, degrading)
  - [ ] 5.4: Generate trend data points (date, score, category)
  - [ ] 5.5: Return trend data for visualization

- [ ] Task 6: Create risk assessment API endpoints (AC: #1, #2)
  - [ ] 6.1: Create `crates/qa-pms-api/src/routes/ai/risk_scoring.rs` module
  - [ ] 6.2: Add `POST /api/v1/ai/assess-release-risk` endpoint
  - [ ] 6.3: Request body: `{ release_version, ticket_keys: Vec<String> }`
  - [ ] 6.4: Return `RiskAssessment` with scores and recommendations
  - [ ] 6.5: Add `GET /api/v1/ai/compare-release-risk` endpoint
  - [ ] 6.6: Request query: `?release1=X&release2=Y`
  - [ ] 6.7: Return comparison data (side-by-side metrics, trends)
  - [ ] 6.8: Add OpenAPI documentation

- [ ] Task 7: Create risk assessment UI components (AC: #1, #2)
  - [ ] 7.1: Create `frontend/src/components/ai/RiskAssessment.tsx` component
  - [ ] 7.2: Display overall risk score with visual indicator (color-coded)
  - [ ] 7.3: Display category scores (code quality, stability, performance, security, coverage)
  - [ ] 7.4: Display high-risk items list with scores and reasons
  - [ ] 7.5: Display mitigation strategies with action items
  - [ ] 7.6: Add release risk heatmap component
  - [ ] 7.7: Add risk comparison view (side-by-side)
  - [ ] 7.8: Display risk trends over time (chart)

- [ ] Task 8: Add comprehensive tests (AC: All)
  - [ ] 8.1: Test category risk calculation
  - [ ] 8.2: Test overall risk score calculation
  - [ ] 8.3: Test high-risk item identification
  - [ ] 8.4: Test mitigation strategy generation
  - [ ] 8.5: Test risk trend analysis
  - [ ] 8.6: Test API endpoints

## Dev Notes

### Architecture Compliance
- **Tech Stack:** Rust 1.80+ with Tokio, Axum 0.7+
- **Pattern:** Risk calculation algorithm → Category scoring → Overall score → Recommendations

### Previous Story Intelligence
- **From Story 31.2 (Bug Prediction):** Use `BugPredictor` for ticket risk scores
- **From Story 31.5 (Report Generation):** May integrate with report generation
- **Key Integration Points:**
  - Use bug risk scores from Story 31.2
  - Fetch tickets and test results from database
  - Display risk scores in release planning UI

### References
- **Source:** `_bmad-output/planning-artifacts/epics-detailed/epic-31-ai-enhanced-automation.md#story-31.6`

## Dev Agent Record

### Agent Model Used
Claude Sonnet 4.5 (via Cursor)

### File List

**Created:**
- `crates/qa-pms-ai/src/risk_scorer.rs` - Risk scoring service
- `crates/qa-pms-api/src/routes/ai/risk_scoring.rs` - Risk scoring API endpoints
- `frontend/src/components/ai/RiskAssessment.tsx` - Risk assessment UI component

**Modified:**
- `crates/qa-pms-ai/src/lib.rs` - Export risk_scorer module
- `crates/qa-pms-api/src/routes/ai/mod.rs` - Add risk_scoring routes

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete BMAD structure
