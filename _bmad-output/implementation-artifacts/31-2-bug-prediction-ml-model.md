# Story 31.2: Bug Prediction ML Model

Status: ready-for-dev

Epic: 31 - AI-Enhanced Automation
Priority: P0 (Critical)
Estimated Effort: 4 days
Sprint: 1

## Story

As a **QA Lead**,
I want to **predict which tickets are likely to cause bugs**,
So that **I can allocate testing resources more effectively and catch issues earlier**.

## Acceptance Criteria

1. **Given** historical data of tickets and bugs
   **When** I view a new ticket
   **Then** the system shows a bug risk score (0.0-1.0)
   **And** highlights risk factors (high story points, API integrations, etc.)
   **And** suggests areas requiring extra testing
   **And** shows confidence level

2. **Given** I'm planning a release
   **When** I analyze all tickets in the release
   **Then** I see a heatmap of high-risk tickets
   **And** can filter by risk level (Low, Medium, High, Critical)
   **And** get recommendations for test focus areas

3. **Given** the ML model is trained
   **When** I provide new training data
   **Then** the model retrains automatically
   **And** model accuracy is evaluated
   **And** training metrics are displayed

## Tasks / Subtasks

- [ ] Task 1: Create bug prediction ML model infrastructure (AC: #1, #2, #3)
  - [ ] 1.1: Create `crates/qa-pms-ai/src/bug_predictor.rs` module
  - [ ] 1.2: Define `BugPredictor` struct with model storage
  - [ ] 1.3: Define `BugRiskScore` struct (ticket_key, risk_score, risk_level, risk_factors, predicted_bugs, confidence)
  - [ ] 1.4: Define `RiskLevel` enum (Low, Medium, High, Critical)
  - [ ] 1.5: Define `RiskFactor` struct (factor, impact, description)
  - [ ] 1.6: Implement model storage (in-memory or file-based for MVP, can upgrade to ONNX/ONNX Runtime later)

- [ ] Task 2: Implement feature extraction from tickets (AC: #1)
  - [ ] 2.1: Create `extract_features(ticket)` method
  - [ ] 2.2: Extract ticket features (issue_type, priority, story_points, components, labels)
  - [ ] 2.3: Extract description features (length, keyword mentions: API, performance, security)
  - [ ] 2.4: Extract historical features (author bug rate, component bug rate, recent bug count)
  - [ ] 2.5: Normalize features (0.0-1.0 range)
  - [ ] 2.6: Return feature vector as `Vec<f32>`

- [ ] Task 3: Implement risk factor identification (AC: #1)
  - [ ] 3.1: Create `identify_risk_factors(ticket, features)` method
  - [ ] 3.2: Check for API integration mentions (high risk)
  - [ ] 3.3: Check for performance keywords (medium-high risk)
  - [ ] 3.4: Check for high story points (>8 = high complexity)
  - [ ] 3.5: Check for new components (higher risk)
  - [ ] 3.6: Check for security-related keywords (high risk)
  - [ ] 3.7: Return list of risk factors with impact scores

- [ ] Task 4: Implement bug type prediction (AC: #1)
  - [ ] 4.1: Create `predict_bug_types(ticket, features)` method
  - [ ] 4.2: Analyze description for bug patterns (race conditions, data integrity, etc.)
  - [ ] 4.3: Use keyword matching and heuristics (can be enhanced with ML later)
  - [ ] 4.4: Return list of predicted bug types

- [ ] Task 5: Implement ML model training pipeline (AC: #3)
  - [ ] 5.1: Create `train(from_date, to_date)` method
  - [ ] 5.2: Fetch historical tickets and bugs from database
  - [ ] 5.3: Prepare training data (features + labels: bug_count or bug_probability)
  - [ ] 5.4: Implement simple ML model (logistic regression or decision tree) using `linfa` crate or similar
  - [ ] 5.5: Train model with training data
  - [ ] 5.6: Evaluate model (accuracy, precision, recall, F1-score)
  - [ ] 5.7: Save trained model to disk
  - [ ] 5.8: Log training metrics

- [ ] Task 6: Implement prediction method (AC: #1)
  - [ ] 6.1: Create `predict(ticket)` method
  - [ ] 6.2: Extract features from ticket
  - [ ] 6.3: Run model prediction (risk score 0.0-1.0)
  - [ ] 6.4: Calculate risk level from score (0.0-0.25=Low, 0.25-0.5=Medium, 0.5-0.75=High, 0.75-1.0=Critical)
  - [ ] 6.5: Identify risk factors
  - [ ] 6.6: Predict bug types
  - [ ] 6.7: Calculate confidence (based on feature quality and model certainty)
  - [ ] 6.8: Return `BugRiskScore`

- [ ] Task 7: Create bug prediction API endpoints (AC: #1, #2, #3)
  - [ ] 7.1: Create `crates/qa-pms-api/src/routes/ai/bug_prediction.rs` module
  - [ ] 7.2: Add `POST /api/v1/ai/predict-bug-risk` endpoint
  - [ ] 7.3: Request body: `{ ticket_key }` or `{ ticket: Ticket }`
  - [ ] 7.4: Return `BugRiskScore` with risk score, factors, predictions
  - [ ] 7.5: Add `POST /api/v1/ai/predict-release-risk` endpoint
  - [ ] 7.6: Request body: `{ release_version, ticket_keys: Vec<String> }`
  - [ ] 7.7: Return risk assessment for entire release (heatmap data)
  - [ ] 7.8: Add `POST /api/v1/ai/train-bug-predictor` endpoint (admin only)
  - [ ] 7.9: Request body: `{ from_date, to_date }`
  - [ ] 7.10: Return training metrics
  - [ ] 7.11: Add OpenAPI documentation

- [ ] Task 8: Create bug risk UI components (AC: #1, #2)
  - [ ] 8.1: Create `frontend/src/components/ai/BugRiskScore.tsx` component
  - [ ] 8.2: Display risk score with visual indicator (color-coded)
  - [ ] 8.3: Display risk level badge (Low/Medium/High/Critical)
  - [ ] 8.4: Display risk factors list with impact indicators
  - [ ] 8.5: Display predicted bug types
  - [ ] 8.6: Display confidence level
  - [ ] 8.7: Add "Analyze Risk" button to ticket detail page
  - [ ] 8.8: Create release risk heatmap component
  - [ ] 8.9: Display tickets in heatmap (color by risk level)
  - [ ] 8.10: Add filters (risk level, component, assignee)

- [ ] Task 9: Add comprehensive tests (AC: All)
  - [ ] 9.1: Test feature extraction
  - [ ] 9.2: Test risk factor identification
  - [ ] 9.3: Test bug type prediction
  - [ ] 9.4: Test model training pipeline
  - [ ] 9.5: Test prediction method
  - [ ] 9.6: Test API endpoints
  - [ ] 9.7: Test UI components

## Dev Notes

### Architecture Compliance
- **Tech Stack:** Rust 1.80+ with Tokio, ML library (`linfa` for MVP, can upgrade to ONNX Runtime later)
- **AI/ML:** Simple ML model for MVP (logistic regression/decision tree), can enhance later
- **Pattern:** Feature extraction → Model training → Prediction → Risk analysis

### Context7 Requirements (MANDATORY)
**CRITICAL:** Before implementing, use Context7 MCP to:
1. **Resolve library ID:** Search for "rust machine learning linfa" or "rust ML crate"
2. **Query Context7 for:** "How to implement simple ML model in Rust using linfa or similar crate"
3. **Query Context7 for:** "Feature engineering best practices for bug prediction models"
4. **Verify patterns for:**
   - Feature extraction and normalization
   - Model training and evaluation
   - Model persistence (save/load)
   - Prediction with confidence scores

### Previous Story Intelligence
- **From Story 3.1 (Jira Ticket Listing):** Jira ticket structure and historical data
- **From Story 13.1 (AI Companion):** May use AI for advanced feature extraction or analysis
- **Key Integration Points:**
  - Fetch tickets from Jira integration
  - Store predictions in database (link to tickets)
  - Display risk scores in ticket UI

### Project Structure Notes
- **ML Module:** `crates/qa-pms-ai/src/bug_predictor.rs` (ML prediction logic)
- **API Routes:** `crates/qa-pms-api/src/routes/ai/bug_prediction.rs` (prediction API)
- **Frontend:** `frontend/src/components/ai/BugRiskScore.tsx` (risk display)

### References
- **Source:** `_bmad-output/planning-artifacts/epics-detailed/epic-31-ai-enhanced-automation.md#story-31.2`

## Dev Agent Record

### Agent Model Used
Claude Sonnet 4.5 (via Cursor)

### File List

**Created:**
- `crates/qa-pms-ai/src/bug_predictor.rs` - Bug prediction ML model
- `crates/qa-pms-api/src/routes/ai/bug_prediction.rs` - Bug prediction API endpoints
- `frontend/src/components/ai/BugRiskScore.tsx` - Bug risk score component
- `frontend/src/components/ai/ReleaseRiskHeatmap.tsx` - Release risk heatmap component

**Modified:**
- `crates/qa-pms-ai/src/lib.rs` - Export bug_predictor module
- `crates/qa-pms-api/src/routes/ai/mod.rs` - Add bug_prediction routes

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete BMAD structure
