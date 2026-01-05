# Story 31.5: Automated Report Generation

Status: ready-for-dev

Epic: 31 - AI-Enhanced Automation
Priority: P1 (High Value)
Estimated Effort: 3 days
Sprint: 1

## Story

As a **QA Engineer**,
I want to **automatically generate comprehensive test reports**,
So that **I don't have to manually compile test results and insights**.

## Acceptance Criteria

1. **Given** a workflow execution completes
   **When** I generate the report
   **Then** the system creates a comprehensive report
   **And** includes test results summary (total, passed, failed, skipped, pass rate)
   **And** highlights critical issues
   **And** provides insights and recommendations (AI-generated)
   **And** exports to multiple formats (PDF, HTML, Markdown)

2. **Given** I need a summary for stakeholders
   **When** I generate an executive summary
   **Then** the report focuses on key metrics
   **And** includes visual charts (pass rate, failure trends, risk scores)
   **And** provides actionable insights
   **And** uses clear, non-technical language

3. **Given** report generation is configured
   **When** workflow completes
   **Then** report is generated automatically (optional)
   **And** report is saved to database or file system
   **And** report link is sent via notification (if configured)

## Tasks / Subtasks

- [ ] Task 1: Create report generator service (AC: #1, #2, #3)
  - [ ] 1.1: Create `crates/qa-pms-ai/src/report_generator.rs` module
  - [ ] 1.2: Create `ReportGenerator` struct with AI client
  - [ ] 1.3: Define `TestReport` struct (id, title, generated_at, workflow, test_results, insights, recommendations, attachments)
  - [ ] 1.4: Define `TestResultsSummary` struct (total_tests, passed, failed, skipped, pass_rate, failed_tests, execution_time)
  - [ ] 1.5: Define `ReportInsight` struct (category, title, description, severity, metrics)
  - [ ] 1.6: Define `Recommendation` struct (priority, title, description, action_items, related_tests)

- [ ] Task 2: Implement AI-powered insights generation (AC: #1, #2)
  - [ ] 2.1: Create `generate_insights(execution)` method
  - [ ] 2.2: Analyze test results for patterns (performance, quality, coverage, stability, risk)
  - [ ] 2.3: Use AI to generate insights (trends, anomalies, recommendations)
  - [ ] 2.4: Categorize insights (Performance, Quality, Coverage, Stability, Risk)
  - [ ] 2.5: Assign severity levels (Info, Warning, Critical)
  - [ ] 2.6: Include metrics and data supporting insights

- [ ] Task 3: Implement report export formats (AC: #1)
  - [ ] 3.1: Add `wkhtmltopdf` or similar for PDF generation, or use `printcss` crate
  - [ ] 3.2: Implement HTML report generation (template-based)
  - [ ] 3.3: Implement Markdown report generation
  - [ ] 3.4: Create report templates (HTML template with charts, Markdown template)
  - [ ] 3.5: Implement `export_report(report, format)` method
  - [ ] 3.6: Generate charts/graphs (pass rate, failure trends) - use chart.js or similar on frontend, or generate server-side

- [ ] Task 4: Create executive summary generation (AC: #2)
  - [ ] 4.1: Create `generate_executive_summary(report)` method
  - [ ] 4.2: Extract key metrics (pass rate, risk score, execution time)
  - [ ] 4.3: Generate visual charts (can be generated on frontend or server-side)
  - [ ] 4.4: Use AI to generate non-technical insights
  - [ ] 4.5: Focus on actionable recommendations
  - [ ] 4.6: Use clear, business-friendly language

- [ ] Task 5: Create report generation API endpoints (AC: #1, #2, #3)
  - [ ] 5.1: Create `crates/qa-pms-api/src/routes/reports.rs` module
  - [ ] 5.2: Add `POST /api/v1/reports/generate` endpoint
  - [ ] 5.3: Request body: `{ workflow_execution_id, format: "pdf"|"html"|"markdown", include_executive_summary: bool }`
  - [ ] 5.4: Generate report and save to database/file system
  - [ ] 5.5: Return report URL or file download
  - [ ] 5.6: Add `GET /api/v1/reports/:report_id` endpoint
  - [ ] 5.7: Add `GET /api/v1/reports/:report_id/download` endpoint
  - [ ] 5.8: Add OpenAPI documentation

- [ ] Task 6: Implement automatic report generation (AC: #3)
  - [ ] 6.1: Hook into workflow completion event
  - [ ] 6.2: Check if auto-report generation is enabled (settings)
  - [ ] 6.3: Generate report automatically on completion
  - [ ] 6.4: Save report to database
  - [ ] 6.5: Send notification with report link (if configured)

- [ ] Task 7: Create report UI components (AC: #1, #2, #3)
  - [ ] 7.1: Create `frontend/src/pages/Reports/ReportViewerPage.tsx`
  - [ ] 7.2: Display report content with sections (summary, results, insights, recommendations)
  - [ ] 7.3: Display charts/graphs (pass rate, failure trends, risk scores)
  - [ ] 7.4: Add "Generate Report" button to workflow execution page
  - [ ] 7.5: Add format selector (PDF, HTML, Markdown)
  - [ ] 7.6: Add "Download Report" button
  - [ ] 7.7: Display executive summary section (toggle)

- [ ] Task 8: Add comprehensive tests (AC: All)
  - [ ] 8.1: Test report generation for various scenarios
  - [ ] 8.2: Test AI insights generation
  - [ ] 8.3: Test report export formats (PDF, HTML, Markdown)
  - [ ] 8.4: Test executive summary generation
  - [ ] 8.5: Test API endpoints
  - [ ] 8.6: Test automatic report generation

## Dev Notes

### Architecture Compliance
- **Tech Stack:** Rust 1.80+ with Tokio, Axum 0.7+, AI client from Story 13.1
- **PDF Generation:** `printcss`, `wkhtmltopdf` wrapper, or similar
- **Charts:** Chart.js on frontend or server-side chart generation (plotters crate)
- **Pattern:** Report data collection → AI insights → Template rendering → Export

### Context7 Requirements (MANDATORY)
**CRITICAL:** Before implementing, use Context7 MCP to:
1. **Query Context7 for:** "How to generate PDF reports in Rust"
2. **Query Context7 for:** "Best practices for HTML report templates with charts"

### Previous Story Intelligence
- **From Story 13.1 (AI Companion):** Use `AIClient` for insights generation
- **From Story 5.3 (Workflow Execution):** Workflow execution data structure
- **Key Integration Points:**
  - Fetch workflow execution results from database
  - Use insights from other AI stories (bug prediction, root cause analysis)

### References
- **Source:** `_bmad-output/planning-artifacts/epics-detailed/epic-31-ai-enhanced-automation.md#story-31.5`

## Dev Agent Record

### Agent Model Used
Claude Sonnet 4.5 (via Cursor)

### File List

**Created:**
- `crates/qa-pms-ai/src/report_generator.rs` - Report generator service
- `crates/qa-pms-api/src/routes/reports.rs` - Report API endpoints
- `frontend/src/pages/Reports/ReportViewerPage.tsx` - Report viewer page
- `frontend/src/components/reports/ReportSummary.tsx` - Report summary component
- `templates/report.html` - HTML report template
- `templates/report.md` - Markdown report template

**Modified:**
- `crates/qa-pms-ai/src/lib.rs` - Export report_generator module
- `crates/qa-pms-api/src/routes/mod.rs` - Add reports routes
- `crates/qa-pms-core/src/workflow/execution.rs` - Add auto-report generation hook

### Change Log

**2026-01-10 - Story Created:**
- Initial story file created with complete BMAD structure
