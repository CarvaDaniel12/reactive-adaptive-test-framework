# Story 24.4: Correlation Dashboard View

Status: ready-for-dev

<!-- Note: Validation is optional. Run validate-create-story for quality check before dev-story. -->

## Story

**As a** QA Engineer working on PMS integration quality  
**I want** a dashboard view showing correlations between test results and integration health  
**So that** I can visualize patterns, prioritize test work, and identify high-correlation test cases

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 24.4 |
| Epic | Epic 24: Test-Integration Correlation Engine |
| Sprint | Sprint 2: Correlation API and Dashboard |
| Priority | P1 |
| Estimated Days | 2 |
| Dependencies | Story 24.3 (Correlation API Endpoint) |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create `CorrelationPage.tsx` component
   - Follow existing page patterns (`DashboardPage.tsx`, `PMDashboardPage.tsx` as reference)
   - Use React Router for routing
   - Support URL query parameters for filters (period, integration, threshold)

2. Create `CorrelationView.tsx` component
   - Timeline visualization showing test failures and integration failures over time
   - Correlation insights table: test case, integration, correlation score, type
   - Recommendations section: prioritized test cases based on correlation
   - Filters: period, integration, test case, correlation threshold

3. Create `useCorrelation.ts` hook
   - Use React Query for data fetching
   - Follow existing hook patterns (`useDashboardData.ts` as reference)
   - Support filtering by period, integration, threshold
   - Auto-refresh every 60 seconds

4. Use existing chart components
   - Reuse `TrendChart` component for timeline visualization (if applicable)
   - Or create new timeline chart component following existing chart patterns

5. Integration with routing
   - Add route `/dashboard/correlation/test-integration` to `App.tsx`
   - Support query parameters in URL (period, integration, threshold)

---

## Acceptance Criteria

- [ ] **Given** correlation view exists  
  **When** loading page  
  **Then** timeline visualization is displayed showing test failures and integration failures over time

- [ ] **Given** correlation view exists  
  **When** viewing insights  
  **Then** correlation insights table is displayed with test case, integration, correlation score, type

- [ ] **Given** correlation view exists  
  **When** viewing recommendations  
  **Then** recommendations section is displayed with prioritized test cases based on correlation

- [ ] **Given** correlation view exists  
  **When** filtering by period  
  **Then** data updates for selected period (7d, 30d, 90d, 1y)

- [ ] **Given** correlation view exists  
  **When** filtering by integration  
  **Then** data updates for selected integration (booking-com, airbnb, vrbo, hmbn)

- [ ] **Given** correlation view exists  
  **When** filtering by correlation threshold  
  **Then** only high correlations (score > threshold) are shown

---

## Tasks / Subtasks

- [ ] Task 1: Create correlation hook (AC: #1, #2, #3, #4, #5, #6)
  - [ ] 1.1: Create `frontend/src/hooks/useCorrelation.ts`
  - [ ] 1.2: Define `CorrelationData` interface
  - [ ] 1.3: Implement `fetchCorrelation` function with API call to `/api/v1/correlation/test-integration`
  - [ ] 1.4: Implement `useCorrelation` hook with React Query
  - [ ] 1.5: Support query parameters: period, integration, threshold
  - [ ] 1.6: Add auto-refresh every 60 seconds

- [ ] Task 2: Create CorrelationView component (AC: #1, #2, #3)
  - [ ] 2.1: Create `frontend/src/components/dashboard/CorrelationView.tsx`
  - [ ] 2.2: Implement timeline visualization showing test failures and integration failures over time
  - [ ] 2.3: Implement correlation insights table with columns: test case, integration, score, type, pattern, confidence
  - [ ] 2.4: Implement recommendations section with prioritized test cases
  - [ ] 2.5: Add filters UI: period selector, integration selector, correlation threshold slider
  - [ ] 2.6: Use existing UI components (PeriodSelector, etc.) where possible

- [ ] Task 3: Create CorrelationPage component (AC: #1, #2, #3, #4, #5, #6)
  - [ ] 3.1: Create `frontend/src/pages/Dashboard/CorrelationPage.tsx`
  - [ ] 3.2: Use `useSearchParams` for URL query parameters
  - [ ] 3.3: Use `useCorrelation` hook for data fetching
  - [ ] 3.4: Implement filter handlers that update URL query parameters
  - [ ] 3.5: Render CorrelationView component with filters
  - [ ] 3.6: Add loading and error states
  - [ ] 3.7: Use `useLayoutStore` for page title (setPageTitle)

- [ ] Task 4: Add route to App.tsx (AC: #1)
  - [ ] 4.1: Import `CorrelationPage` in `frontend/src/App.tsx`
  - [ ] 4.2: Add route `/dashboard/correlation/test-integration` to router
  - [ ] 4.3: Test routing works correctly

- [ ] Task 5: Export components (AC: #1)
  - [ ] 5.1: Add export to `frontend/src/components/dashboard/index.ts`
  - [ ] 5.2: Add export to `frontend/src/pages/Dashboard/index.ts` (if exists)

- [ ] Task 6: Add unit tests (AC: #1, #2, #3, #4, #5, #6)
  - [ ] 6.1: Test CorrelationView component rendering
  - [ ] 6.2: Test CorrelationPage component rendering
  - [ ] 6.3: Test useCorrelation hook data fetching
  - [ ] 6.4: Test filter handlers

- [ ] Task 7: Manual testing (AC: #1, #2, #3, #4, #5, #6)
  - [ ] 7.1: Verify UI looks correct
  - [ ] 7.2: Verify filters work correctly
  - [ ] 7.3: Verify timeline visualization displays correctly
  - [ ] 7.4: Verify table displays correctly
  - [ ] 7.5: Verify recommendations section displays correctly

---

## Files to Create

| File | Changes |
|------|---------|
| `frontend/src/pages/Dashboard/CorrelationPage.tsx` | Create correlation page component |
| `frontend/src/components/dashboard/CorrelationView.tsx` | Create correlation view component |
| `frontend/src/hooks/useCorrelation.ts` | Create correlation data hook |

---

## Files to Modify

| File | Changes |
|------|---------|
| `frontend/src/App.tsx` | Add route `/dashboard/correlation/test-integration` |
| `frontend/src/components/dashboard/index.ts` | Export `CorrelationView` component |

---

## Dev Notes

### Component Structure

**CorrelationPage.tsx:**
```typescript
export function CorrelationPage() {
  const [searchParams, setSearchParams] = useSearchParams();
  const { setPageTitle } = useLayoutStore();
  
  const period = searchParams.get("period") || "30d";
  const integration = searchParams.get("integration");
  const threshold = parseFloat(searchParams.get("threshold") || "0.5");
  
  const { data, isLoading, isFetching } = useCorrelation({ period, integration, threshold });
  
  useEffect(() => {
    setPageTitle("Test-Integration Correlation", "Identify patterns and prioritize tests");
    return () => setPageTitle("");
  }, [setPageTitle]);
  
  const handleFilterChange = (filters: { period?: string; integration?: string; threshold?: number }) => {
    const params = new URLSearchParams();
    if (filters.period) params.set("period", filters.period);
    if (filters.integration) params.set("integration", filters.integration);
    if (filters.threshold !== undefined) params.set("threshold", filters.threshold.toString());
    setSearchParams(params);
  };
  
  return <CorrelationView data={data} isLoading={isLoading} onFilterChange={handleFilterChange} />;
}
```

**useCorrelation.ts:**
```typescript
interface CorrelationFilters {
  period?: string; // "7d", "30d", "90d", "1y"
  integration?: string; // "booking-com", "airbnb", "vrbo", "hmbn"
  threshold?: number; // 0.0 to 1.0
}

interface CorrelationData {
  correlations: Array<{
    testCaseId: string;
    testCaseName: string;
    integrationId: string;
    correlationScore: number;
    correlationType: "high" | "medium" | "low";
    pattern: string;
    confidence: number;
    lastCorrelated: string;
  }>;
  timeline: Array<{
    date: string;
    testFailures: number;
    integrationFailures: number;
  }>;
  recommendations: Array<{
    testCaseId: string;
    testCaseName: string;
    priority: "high" | "medium" | "low";
    reason: string;
  }>;
}

export function useCorrelation(filters: CorrelationFilters) {
  return useQuery({
    queryKey: ["correlation", filters],
    queryFn: () => fetchCorrelation(filters),
    staleTime: 60_000,
    refetchInterval: 60_000,
  });
}
```

### Timeline Visualization

**Approach:**
- Show test failures and integration failures over time on same timeline
- Use area chart or line chart (similar to TrendChart)
- X-axis: time (dates)
- Y-axis: count of failures
- Two series: test failures, integration failures
- Use existing chart library (recharts or similar)

### Correlation Insights Table

**Columns:**
- Test Case (ID, Name)
- Integration (ID, Name)
- Correlation Score (0.0-1.0, color-coded: high=red, medium=yellow, low=green)
- Correlation Type (high/medium/low badge)
- Pattern (description)
- Confidence (0.0-1.0)
- Last Correlated (timestamp)

**Sorting:**
- Default: by correlation score DESC (highest first)
- Allow sorting by other columns

**Filtering:**
- By correlation threshold: only show correlations >= threshold
- By integration: filter to specific integration
- By period: filter by time range

### Recommendations Section

**Priority Calculation:**
- High: correlation score > 0.8 and confidence > 0.7
- Medium: correlation score > 0.6 and confidence > 0.5
- Low: correlation score > 0.4

**Display:**
- List of test cases with priority badges
- Reason for recommendation (e.g., "High correlation (0.85) with Airbnb integration failures")
- Link to test case details (if available)

### Project Structure Notes

**Frontend Patterns:**
- Follow existing page patterns (`DashboardPage.tsx`, `PMDashboardPage.tsx`)
- Use React Router for navigation and URL state
- Use React Query for data fetching with auto-refresh
- Use Zustand stores for global state (`useLayoutStore`)
- Use existing UI components where possible (PeriodSelector, etc.)

**Styling:**
- Use Tailwind CSS v4 (existing pattern)
- Follow existing component styling patterns
- Use Radix UI for headless components if needed

**API Integration:**
- Use API endpoint from Story 24.3: `/api/v1/correlation/test-integration`
- Support query parameters: `period`, `integration`, `threshold`
- Handle loading and error states

### Testing Standards

**Unit Tests:**
- Test component rendering
- Test hook data fetching
- Test filter handlers
- Test URL parameter parsing

**Integration Tests:**
- Test component integration with routing
- Test API integration
- Test filter interactions

**Manual Tests:**
- Verify UI looks correct
- Verify filters work correctly
- Verify timeline visualization
- Verify table displays and sorting
- Verify recommendations display

### References

- Epic Document: `_bmad-output/planning-artifacts/epics-observability-pms-integrations-2026-01-10.md` (Epic 24, Story 24.4)
- Architecture Document: `_bmad-output/planning-artifacts/architecture-observability-pms-integrations-2026-01-10.md` (Section 6.2: Frontend Architecture)
- Page Patterns: `qa-intelligent-pms/frontend/src/pages/Dashboard/DashboardPage.tsx` (reference)
- Hook Patterns: `qa-intelligent-pms/frontend/src/hooks/useDashboardData.ts` (reference)
- Dependency: Story 24.3 (Correlation API Endpoint) - must be completed first
- Routing: `qa-intelligent-pms/frontend/src/App.tsx` (for route addition)
- Project Context: `_bmad-output/planning-artifacts/project-context.md`

---

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
