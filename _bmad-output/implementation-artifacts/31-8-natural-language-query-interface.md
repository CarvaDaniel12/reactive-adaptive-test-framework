# Story 31.8: Natural Language Query Interface

Status: ready-for-dev

Epic: 31 - AI-Enhanced Automation
Priority: P1 (High Value)
Estimated Effort: 4 days
Sprint: 1

## Story

As a **QA Engineer**,
I want to **ask questions about test results and get natural language answers**,
So that **I can quickly get insights without writing complex queries**.

## Acceptance Criteria

1. **Given** I want to know about test failures
   **When** I ask "How many tests failed yesterday?"
   **Then** the system understands the question
   **And** queries the data (filters by date: yesterday, status: failed)
   **And** returns a natural language answer ("5 tests failed yesterday")
   **And** shows supporting data (list of failed tests)

2. **Given** I need to investigate an issue
   **When** I ask "Show me all tests related to payment processing"
   **Then** the system finds relevant tests (filters by tags/keywords: payment, processing)
   **And** presents them in a clear format
   **And** provides context (related tickets, execution history)

3. **Given** I ask a complex question
   **When** I query "What's the trend of test failures over the last month?"
   **Then** the system understands the intent (TrendAnalysis)
   **And** queries historical data
   **And** generates a natural language answer with visualization (line chart)

## Tasks / Subtasks

- [ ] Task 1: Create NL query engine service (AC: #1, #2, #3)
  - [ ] 1.1: Create `crates/qa-pms-ai/src/nl_query.rs` module
  - [ ] 1.2: Create `NLQueryEngine` struct with AI client
  - [ ] 1.3: Define `QueryResult` struct (query, answer, intent, data, visualization)
  - [ ] 1.4: Define `QueryIntent` enum (CountTests, TestFailures, TestPerformance, BugAnalysis, TrendAnalysis, Comparison, Search, Summary)
  - [ ] 1.5: Define `QueryContext` struct (start_date, end_date, environment, project, user_id)

- [ ] Task 2: Implement query intent parsing (AC: #1, #2, #3)
  - [ ] 2.1: Create `parse_intent(query, context)` method
  - [ ] 2.2: Use AI to parse natural language query and determine intent
  - [ ] 2.3: Extract entities from query (dates, test names, ticket keys, etc.)
  - [ ] 2.4: Map intent to query type (CountTests, TestFailures, etc.)
  - [ ] 2.5: Return `QueryIntent` with extracted entities

- [ ] Task 3: Implement query execution (AC: #1, #2, #3)
  - [ ] 3.1: Create `execute_query(intent, context)` method
  - [ ] 3.2: Implement `query_count_tests(context)` - count tests with filters
  - [ ] 3.3: Implement `query_test_failures(context)` - find failed tests with filters
  - [ ] 3.4: Implement `query_test_performance(context)` - analyze performance metrics
  - [ ] 3.5: Implement `query_bug_analysis(context)` - analyze bug patterns
  - [ ] 3.6: Implement `query_trend_analysis(context)` - analyze trends over time
  - [ ] 3.7: Implement `query_comparison(context)` - compare two entities
  - [ ] 3.8: Implement `query_search(context)` - search for specific items
  - [ ] 3.9: Return query results as JSON

- [ ] Task 4: Implement natural language answer generation (AC: #1, #2, #3)
  - [ ] 4.1: Create `generate_answer(query, intent, data)` method
  - [ ] 4.2: Use AI to generate natural language answer from query results
  - [ ] 4.3: Include relevant data in answer (numbers, names, dates)
  - [ ] 4.4: Provide context when available (trends, comparisons)
  - [ ] 4.5: Return clear, concise answer

- [ ] Task 5: Implement visualization suggestions (AC: #3)
  - [ ] 5.1: Create `suggest_visualization(intent, data)` method
  - [ ] 5.2: Map intent to visualization type (CountTests → BarChart, TrendAnalysis → LineChart, etc.)
  - [ ] 5.3: Format data for visualization (x-axis, y-axis, series)
  - [ ] 5.4: Return visualization configuration (type, data)

- [ ] Task 6: Create NL query API endpoints (AC: #1, #2, #3)
  - [ ] 6.1: Create `crates/qa-pms-api/src/routes/ai/nl_query.rs` module
  - [ ] 6.2: Add `POST /api/v1/ai/query` endpoint
  - [ ] 6.3: Request body: `{ query: String, context: QueryContext }`
  - [ ] 6.4: Return `QueryResult` with answer, data, and visualization
  - [ ] 6.5: Add OpenAPI documentation

- [ ] Task 7: Create NL query UI components (AC: #1, #2, #3)
  - [ ] 7.1: Create `frontend/src/components/ai/NLQueryInterface.tsx` component
  - [ ] 7.2: Add query input field (chat-like interface)
  - [ ] 7.3: Display query results with natural language answer
  - [ ] 7.4: Display supporting data (tables, lists)
  - [ ] 7.5: Display visualizations (charts/graphs) when suggested
  - [ ] 7.6: Add query history (save previous queries)
  - [ ] 7.7: Add query suggestions (common queries)
  - [ ] 7.8: Display loading state during query processing

- [ ] Task 8: Add comprehensive tests (AC: All)
  - [ ] 8.1: Test query intent parsing for various query types
  - [ ] 8.2: Test query execution for each intent type
  - [ ] 8.3: Test natural language answer generation
  - [ ] 8.4: Test visualization suggestions
  - [ ] 8.5: Test API endpoints
  - [ ] 8.6: Test UI components

## Dev Notes

### Architecture Compliance
- **Tech Stack:** Rust 1.80+ with Tokio, Axum 0.7+, AI client from Story 13.1
- **Pattern:** Natural language parsing → Intent classification → Query execution → Answer generation → Visualization

### Context7 Requirements (MANDATORY)
**CRITICAL:** Before implementing, use Context7 MCP to:
1. **Query Context7 for:** "How to implement natural language query parsing with AI models"
2. **Query Context7 for:** "Best practices for intent classification and entity extraction"

### Previous Story Intelligence
- **From Story 13.1 (AI Companion):** Reuse `AIClient` for query parsing and answer generation
- **From Story 5.3 (Workflow Execution):** Query test execution data
- **Key Integration Points:**
  - Parse natural language queries using AI
  - Execute queries against database
  - Generate natural language answers using AI

### References
- **Source:** `_bmad-output/planning-artifacts/epics-detailed/epic-31-ai-enhanced-automation.md#story-31.8`

## Dev Agent Record

### Agent Model Used
Claude Sonnet 4.5 (via Cursor)

### File List

**Created:**
- `crates/qa-pms-ai/src/nl_query.rs` - NL query engine service
- `crates/qa-pms-api/src/routes/ai/nl_query.rs` - NL query API endpoints
- `frontend/src/components/ai/NLQueryInterface.tsx` - NL query UI component

**Modified:**
- `crates/qa-pms-ai/src/lib.rs` - Export nl_query module
- `crates/qa-pms-api/src/routes/ai/mod.rs` - Add nl_query routes

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete BMAD structure
