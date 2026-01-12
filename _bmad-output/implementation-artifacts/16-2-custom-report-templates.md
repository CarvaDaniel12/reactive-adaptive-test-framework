# Story 16.2: Custom Report Templates

Status: ready-for-dev

## Story

**As a** QA Engineer  
**I want** to create and use custom report templates  
**So that** I can generate reports with specific formats and sections tailored to my needs

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 16.2 |
| Epic | Epic 16: Reports Enhancement |
| Sprint | Sprint 1: Report Templates |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Epic 7 (Reporting & Documentation) - Complete ✅ |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create report templates database schema
   - Create `report_templates` table
   - Columns: `id` (UUID), `name` (VARCHAR 255), `description` (TEXT), `template_config` (JSONB), `is_default` (BOOLEAN), `created_by` (UUID), `created_at`, `updated_at`
   - Support template versioning (future)

2. Create report template types
   - Define `ReportTemplate` struct
   - Define `TemplateConfig` struct with sections, formatting options
   - Support sections: steps, notes, tests_covered, strategies, time_breakdown, metrics

3. Create report template API endpoints
   - `GET /api/v1/reports/templates` - list templates
   - `GET /api/v1/reports/templates/:id` - get template
   - `POST /api/v1/reports/templates` - create template
   - `PUT /api/v1/reports/templates/:id` - update template
   - `DELETE /api/v1/reports/templates/:id` - delete template

4. Integrate templates with report generation
   - Update report generation to use template
   - Support default template fallback
   - Apply template formatting to report content

---

## Acceptance Criteria

- [ ] **Given** report templates exist  
  **When** listing templates  
  **Then** returns all available templates (default + custom)

- [ ] **Given** report template exists  
  **When** creating report with template  
  **Then** report uses template formatting and sections

- [ ] **Given** report template API exists  
  **When** creating template  
  **Then** template is saved and can be reused

- [ ] **Given** report template API exists  
  **When** updating template  
  **Then** template is updated and changes apply to future reports

---

## Tasks / Subtasks

- [ ] Task 1: Create report_templates table (AC: #1)
  - [ ] 1.1: Create migration: `YYYYMMDDHHMMSS_create_report_templates_table.sql`
  - [ ] 1.2: Define schema with all columns
  - [ ] 1.3: Add indexes on `created_by`, `is_default`

- [ ] Task 2: Create template types (AC: #2)
  - [ ] 2.1: Create `ReportTemplate` struct
  - [ ] 2.2: Create `TemplateConfig` struct (JSONB structure)
  - [ ] 2.3: Add `serde` serialization

- [ ] Task 3: Create template repository (AC: #1, #2)
  - [ ] 3.1: Create `ReportTemplateRepository` in new crate or existing
  - [ ] 3.2: Implement CRUD operations
  - [ ] 3.3: Implement list templates method

- [ ] Task 4: Create template API endpoints (AC: #3)
  - [ ] 4.1: Add routes to `routes/reports.rs`
  - [ ] 4.2: Implement GET, POST, PUT, DELETE handlers
  - [ ] 4.3: Add OpenAPI documentation

- [ ] Task 5: Integrate templates with report generation (AC: #2)
  - [ ] 5.1: Update `generate_report` to accept template_id
  - [ ] 5.2: Apply template formatting to report content
  - [ ] 5.3: Support default template fallback

---

## Files to Create

| File | Changes |
|------|---------|
| `migrations/YYYYMMDDHHMMSS_create_report_templates_table.sql` | Create report templates table |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-api/src/routes/reports.rs` | Add template endpoints and integration |
| `crates/qa-pms-api/src/routes/reports.rs` (or new service) | Add template repository/service |

---

## Dev Notes

### Database Schema

```sql
CREATE TABLE IF NOT EXISTS report_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    template_config JSONB NOT NULL, -- TemplateConfig structure
    is_default BOOLEAN NOT NULL DEFAULT false,
    created_by UUID, -- References users if exists
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_report_templates_created_by ON report_templates(created_by);
CREATE INDEX IF NOT EXISTS idx_report_templates_is_default ON report_templates(is_default);
```

### Template Config Structure

```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TemplateConfig {
    pub sections: Vec<TemplateSection>,
    pub formatting: TemplateFormatting,
}

pub enum TemplateSection {
    Steps,
    Notes,
    TestsCovered,
    Strategies,
    TimeBreakdown,
    Metrics,
}
```

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 16, Story 16.2)
- Dependency: Epic 7 (Reporting & Documentation) - must be complete
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### File List
