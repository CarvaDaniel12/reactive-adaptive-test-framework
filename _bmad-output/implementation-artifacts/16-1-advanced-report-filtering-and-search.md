# Story 16.1: Advanced Report Filtering and Search

Status: ready-for-dev

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

**As a** QA Engineer  
**I want** advanced filtering and search capabilities for reports  
**So that** I can quickly find specific reports for analysis and review

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 16.1 |
| Epic | Epic 16: Reports Enhancement |
| Sprint | Sprint 1: Report Filtering |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Epic 7 (Reporting & Documentation) - Complete ✅ |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Enhance report search endpoint
   - Update `GET /api/v1/reports` endpoint to support advanced filters
   - Add filtering by: status, date range, template name, ticket key, user (future)
   - Add search by: ticket title, content keywords (full-text search future)
   - Support pagination (page, page_size)
   - Support sorting (by generated_at DESC, ticket_key ASC, template_name ASC)

2. Enhance frontend report search
   - Update `useReports` hook to support additional filters
   - Add filter UI components (dropdowns, date pickers, search input)
   - Support URL query parameters for filters
   - Support filter persistence in URL

3. Implement filter UI
   - Add filter dropdowns: status, template name
   - Add date range picker: start date, end date
   - Add search input: ticket key, ticket title
   - Add sort selector: sort by, sort order

4. Support full-text search (future/optional)
   - Add full-text search by report content keywords
   - Use PostgreSQL full-text search (tsvector/tsquery)
   - Search in: ticket title, template name, notes, test names

---

## Acceptance Criteria

- [ ] **Given** report search endpoint exists  
  **When** filtering by status  
  **Then** returns reports matching the specified status

- [ ] **Given** report search endpoint exists  
  **When** filtering by date range  
  **Then** returns reports within the specified date range

- [ ] **Given** report search endpoint exists  
  **When** filtering by template name  
  **Then** returns reports matching the specified template name

- [ ] **Given** report search endpoint exists  
  **When** searching by ticket key  
  **Then** returns reports matching the specified ticket key

- [ ] **Given** report search endpoint exists  
  **When** filtering with multiple criteria  
  **Then** returns reports matching all criteria

- [ ] **Given** report search endpoint exists  
  **When** sorting by generated_at DESC  
  **Then** returns reports sorted by newest first

- [ ] **Given** report filter UI exists  
  **When** applying filters  
  **Then** filters are reflected in URL query parameters

- [ ] **Given** report filter UI exists  
  **When** loading page with query parameters  
  **Then** filters are restored from URL

---

## Tasks / Subtasks

- [ ] Task 1: Enhance report search endpoint (AC: #1, #2, #3, #4, #5, #6)
  - [ ] 1.1: Update `GET /api/v1/reports` endpoint in `routes/reports.rs`
  - [ ] 1.2: Create `ReportFilters` struct with filter fields:
    - `status: Option<String>` (filter by report status if exists)
    - `start_date: Option<DateTime<Utc>>` (filter by start date)
    - `end_date: Option<DateTime<Utc>>` (filter by end date)
    - `template_name: Option<String>` (filter by template name)
    - `ticket_key: Option<String>` (filter by ticket key)
    - `ticket_title_search: Option<String>` (search by ticket title, future)
  - [ ] 1.3: Create `ReportSort` enum: `GeneratedAtDesc`, `GeneratedAtAsc`, `TicketKeyAsc`, `TemplateNameAsc`
  - [ ] 1.4: Update `list_reports` handler to use filters and sort
  - [ ] 1.5: Build dynamic SQL query with filters
  - [ ] 1.6: Add pagination support (page, page_size)
  - [ ] 1.7: Add sorting support
  - [ ] 1.8: Return total count of matching reports
  - [ ] 1.9: Update `utoipa::path` macro for OpenAPI documentation

- [ ] Task 2: Create ReportFilters type (AC: #1, #2, #3, #4)
  - [ ] 2.1: Create `ReportFilters` struct in `routes/reports.rs`
  - [ ] 2.2: Add `serde` deserialization
  - [ ] 2.3: Add `utoipa::IntoParams` for query parameters

- [ ] Task 3: Update frontend useReports hook (AC: #7, #8)
  - [ ] 3.1: Update `frontend/src/hooks/useReports.ts`
  - [ ] 3.2: Add filter parameters:
    - `status?: string`
    - `startDate?: string`
    - `endDate?: string`
    - `templateName?: string`
    - `ticketKey?: string`
    - `sortBy?: string`
    - `sortOrder?: 'asc' | 'desc'`
  - [ ] 3.3: Update `reportsApi.list()` call with filters

- [ ] Task 4: Update frontend API client (AC: #7, #8)
  - [ ] 4.1: Update `frontend/src/lib/api.ts` `reportsApi.list()` method
  - [ ] 4.2: Add filter parameters to query string
  - [ ] 4.3: Add sort parameters to query string

- [ ] Task 5: Create report filter UI components (AC: #7, #8)
  - [ ] 5.1: Create `frontend/src/components/reports/ReportFilters.tsx`
  - [ ] 5.2: Add filter dropdowns: status, template name
  - [ ] 5.3: Add date range picker: start date, end date
  - [ ] 5.4: Add search input: ticket key
  - [ ] 5.5: Add sort selector: sort by, sort order
  - [ ] 5.6: Use `useSearchParams` for URL state management
  - [ ] 5.7: Update URL query parameters on filter change

- [ ] Task 6: Integrate filters with reports page (AC: #7, #8)
  - [ ] 6.1: Update reports page to use `ReportFilters` component
  - [ ] 6.2: Read filters from URL query parameters
  - [ ] 6.3: Pass filters to `useReports` hook
  - [ ] 6.4: Display filtered results

- [ ] Task 7: Add unit and integration tests (AC: #1, #2, #3, #4, #5, #6, #7, #8)
  - [ ] 7.1: Test API endpoint with various filters
  - [ ] 7.2: Test sorting functionality
  - [ ] 7.3: Test pagination with filters
  - [ ] 7.4: Test filter UI components
  - [ ] 7.5: Test URL query parameter persistence

---

## Files to Create

| File | Changes |
|------|---------|
| `frontend/src/components/reports/ReportFilters.tsx` | Create report filter UI component |

---

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qa-pms-api/src/routes/reports.rs` | Add advanced filtering and sorting to list_reports endpoint |
| `frontend/src/hooks/useReports.ts` | Add filter parameters support |
| `frontend/src/lib/api.ts` | Update reportsApi.list() with filter parameters |
| `frontend/src/pages/Reports/ReportsPage.tsx` (or similar) | Integrate ReportFilters component |

---

## Dev Notes

### API Endpoint Enhancement

**ReportFilters:**
```rust
#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ReportFilters {
    pub status: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub template_name: Option<String>,
    pub ticket_key: Option<String>,
    pub ticket_title_search: Option<String>, // Future: full-text search
}
```

**ReportSort:**
```rust
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReportSort {
    GeneratedAtDesc,
    GeneratedAtAsc,
    TicketKeyAsc,
    TemplateNameAsc,
}
```

**Dynamic Query Building:**
```rust
let mut query = sqlx::QueryBuilder::new(
    "SELECT id, workflow_instance_id, ticket_id, ticket_title, template_name, content, total_time_seconds, generated_at FROM workflow_reports WHERE 1=1"
);

if let Some(status) = filters.status {
    query.push(" AND status = ");
    query.push_bind(status);
}

if let Some(start_date) = filters.start_date {
    query.push(" AND generated_at >= ");
    query.push_bind(start_date);
}

if let Some(end_date) = filters.end_date {
    query.push(" AND generated_at <= ");
    query.push_bind(end_date);
}

if let Some(template_name) = filters.template_name {
    query.push(" AND template_name = ");
    query.push_bind(template_name);
}

if let Some(ticket_key) = filters.ticket_key {
    query.push(" AND ticket_id LIKE ");
    query.push_bind(format!("{}%", ticket_key));
}

match sort {
    ReportSort::GeneratedAtDesc => query.push(" ORDER BY generated_at DESC"),
    ReportSort::GeneratedAtAsc => query.push(" ORDER BY generated_at ASC"),
    ReportSort::TicketKeyAsc => query.push(" ORDER BY ticket_id ASC"),
    ReportSort::TemplateNameAsc => query.push(" ORDER BY template_name ASC"),
}

query.push(" LIMIT ");
query.push_bind(page_size);
query.push(" OFFSET ");
query.push_bind((page - 1) * page_size);
```

### Frontend Filter Component

**ReportFilters Component:**
```typescript
export function ReportFilters() {
  const [searchParams, setSearchParams] = useSearchParams();
  
  const status = searchParams.get("status") || "";
  const templateName = searchParams.get("templateName") || "";
  const ticketKey = searchParams.get("ticketKey") || "";
  const startDate = searchParams.get("startDate") || "";
  const endDate = searchParams.get("endDate") || "";
  const sortBy = searchParams.get("sortBy") || "generatedAt";
  const sortOrder = searchParams.get("sortOrder") || "desc";
  
  const handleFilterChange = (key: string, value: string) => {
    const params = new URLSearchParams(searchParams);
    if (value) {
      params.set(key, value);
    } else {
      params.delete(key);
    }
    params.set("page", "1"); // Reset to first page
    setSearchParams(params);
  };
  
  // Render filter UI
}
```

### Project Structure Notes

**API Patterns:**
- Follow existing route patterns (`routes/tickets.rs` for filtering reference)
- Use SQLx query builder for dynamic filtering
- Use `IntoParams` for query parameter parsing
- Support pagination and sorting

**Frontend Patterns:**
- Follow existing filter patterns (`TicketsPage.tsx` for reference)
- Use `useSearchParams` for URL state management
- Use React Query for data fetching
- Support filter persistence in URL

**Full-Text Search (Future):**
- Use PostgreSQL full-text search (tsvector/tsquery)
- Add GIN index on content column (JSONB)
- Search in: ticket_title, template_name, content JSONB fields

### Testing Standards

**Unit Tests:**
- Test filter combinations
- Test sorting functionality
- Test pagination with filters

**Integration Tests:**
- Test API endpoint with real database
- Test filter UI components
- Test URL query parameter persistence

### References

- Epic Document: `_bmad-output/planning-artifacts/epics.md` (Epic 16, Story 16.1)
- Dependency: Epic 7 (Reporting & Documentation) - must be complete
- Route Patterns: `qa-intelligent-pms/crates/qa-pms-api/src/routes/tickets.rs` (reference)
- Filter Patterns: `qa-intelligent-pms/frontend/src/pages/Tickets/TicketsPage.tsx` (reference)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
