# Story 23.5: Revenue Impact Detail Page

**Status:** `ready-for-dev`

## Story

**As a** QA Engineer working on PMS integration quality  
**I want** to see detailed revenue impact breakdown with charts and trends  
**So that** I can analyze revenue impact in depth and identify patterns

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 23.5 |
| Epic | Epic 23: Revenue Impact Calculator and Dashboard |
| Sprint | Sprint 2: Revenue API and Dashboard |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | Story 23.4 (Revenue Impact KPI Cards) |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create `RevenueImpactPage` component
   - Location: `frontend/src/pages/Dashboard/RevenueImpactPage.tsx`
   - Full page component with header, content, and layout

2. Display revenue at risk and revenue protected:
   - Show summary metrics at top of page
   - Use same KPI card format as Story 23.4 (optional: reuse RevenueImpactCards component)
   - Display values in currency format

3. Breakdown table:
   - Columns: Integration, Impact, Type, Trend
   - Data source: `breakdown` array from API response
   - Integration: integration name (e.g., "Booking.com", "Airbnb")
   - Impact: revenue impact value (currency format)
   - Type: impact type (e.g., "pricing_sync_error", "fee_sync_error", "booking_loss")
   - Trend: trend indicator (up/down/neutral)

4. Trend charts:
   - Display revenue impact over time
   - Use existing chart components (e.g., `TrendChart` from dashboard)
   - Show revenue at risk trend and revenue protected trend
   - Support period selection (7d, 30d, 90d, 1y)

5. Period selector:
   - Allow user to select period (7d, 30d, 90d, 1y)
   - Default: "30d"
   - Update data when period changes
   - Use existing `PeriodSelector` component pattern if available

6. Export data (CSV, PDF) - future:
   - Note: Mark as future/optional for this story
   - Can be implemented in Phase 2 if needed

7. Add route to App:
   - Route: `/dashboard/revenue/impact`
   - Add to `App.tsx` router configuration
   - Ensure route matches navigation from Story 23.4

8. Use existing chart components:
   - Reuse `TrendChart` or similar chart component from dashboard
   - Follow existing chart patterns and styling
   - Use Recharts library (existing dependency)

---

## Acceptance Criteria

- [ ] **Given** detail page implemented  
  **When** navigating from KPI cards (Story 23.4)  
  **Then** page displays revenue impact details

- [ ] **Given** detail page displayed  
  **When** viewing breakdown  
  **Then** breakdown table shows integration, impact, type, trend columns

- [ ] **Given** detail page displayed  
  **When** viewing trends  
  **Then** trend charts show revenue impact over time

- [ ] **Given** detail page displayed  
  **When** selecting period (7d, 30d, 90d, 1y)  
  **Then** data updates for selected period

- [ ] **Given** detail page displayed  
  **When** data loading  
  **Then** loading state is displayed (skeleton or spinner)

---

## Tasks

- [ ] Task 1: Create RevenueImpactPage component (AC: #1)
  - [ ] 1.1: Create `frontend/src/pages/Dashboard/RevenueImpactPage.tsx`
  - [ ] 1.2: Create page layout (header, content sections)
  - [ ] 1.3: Add page title and description
  - [ ] 1.4: Import and use `useRevenueImpact` hook

- [ ] Task 2: Add summary metrics display (AC: #2)
  - [ ] 2.1: Display revenue at risk and revenue protected metrics
  - [ ] 2.2: Use currency formatting (reuse from Story 23.4)
  - [ ] 2.3: Optionally reuse RevenueImpactCards component
  - [ ] 2.4: Add summary section at top of page

- [ ] Task 3: Create breakdown table (AC: #3)
  - [ ] 3.1: Create table component with columns: Integration, Impact, Type, Trend
  - [ ] 3.2: Map breakdown data to table rows
  - [ ] 3.3: Format impact values as currency
  - [ ] 3.4: Display trend indicators (up/down/neutral)
  - [ ] 3.5: Style table (use existing table patterns from codebase)

- [ ] Task 4: Add trend charts (AC: #4)
  - [ ] 4.1: Import existing chart component (TrendChart or similar)
  - [ ] 4.2: Create chart for revenue at risk trend
  - [ ] 4.3: Create chart for revenue protected trend
  - [ ] 4.4: Format chart data from API response
  - [ ] 4.5: Style charts (follow existing chart patterns)

- [ ] Task 5: Add period selector (AC: #5)
  - [ ] 5.1: Create period selector component (or reuse existing)
  - [ ] 5.2: Support periods: 7d, 30d, 90d, 1y
  - [ ] 5.3: Default to "30d"
  - [ ] 5.4: Update data when period changes (update useRevenueImpact hook call)
  - [ ] 5.5: Place selector in page header or near charts

- [ ] Task 6: Add loading state (AC: #6)
  - [ ] 6.1: Show loading skeleton or spinner when data loading
  - [ ] 6.2: Use existing loading patterns from codebase
  - [ ] 6.3: Handle error state (show error message or fallback)

- [ ] Task 7: Add route to App (AC: #1)
  - [ ] 7.1: Import RevenueImpactPage in `App.tsx`
  - [ ] 7.2: Add route: `/dashboard/revenue/impact`
  - [ ] 7.3: Verify route matches navigation from Story 23.4
  - [ ] 7.4: Test navigation works correctly

- [ ] Task 8: Update useRevenueImpact hook (AC: #5)
  - [ ] 8.1: Update hook to accept period parameter (already supports this)
  - [ ] 8.2: Verify hook works with period changes
  - [ ] 8.3: Add time-series data support if needed for charts (may need API enhancement)

---

## Files to Create

| File | Changes |
|------|---------|
| `frontend/src/pages/Dashboard/RevenueImpactPage.tsx` | Create RevenueImpactPage component with breakdown table and charts |

---

## Files to Modify

| File | Changes |
|------|---------|
| `frontend/src/App.tsx` | Add route `/dashboard/revenue/impact` |
| `frontend/src/hooks/useRevenueImpact.ts` | Verify period parameter support (may need enhancement for time-series data) |

---

## Implementation Notes

### Page Structure

Follow existing page patterns from `DashboardPage.tsx`:

```typescript
// frontend/src/pages/Dashboard/RevenueImpactPage.tsx
import { useRevenueImpact } from '@/hooks/useRevenueImpact';
import { TrendChart } from '@/components/dashboard/TrendChart';
import { useState } from 'react';

export function RevenueImpactPage() {
  const [period, setPeriod] = useState('30d');
  const { data, isLoading, error } = useRevenueImpact(period);

  if (isLoading) {
    return <RevenueImpactPageSkeleton />;
  }

  if (error) {
    return <div>Error loading revenue impact data</div>;
  }

  if (!data) {
    return null;
  }

  return (
    <div className="container mx-auto px-4 py-6">
      <h1 className="text-2xl font-bold mb-6">Revenue Impact</h1>
      
      {/* Summary metrics */}
      <RevenueImpactCards data={data} />
      
      {/* Period selector */}
      <PeriodSelector value={period} onChange={setPeriod} />
      
      {/* Breakdown table */}
      <RevenueBreakdownTable breakdown={data.breakdown} />
      
      {/* Trend charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mt-6">
        <TrendChart title="Revenue At Risk" data={data.revenueAtRiskTrend} />
        <TrendChart title="Revenue Protected" data={data.revenueProtectedTrend} />
      </div>
    </div>
  );
}
```

### Breakdown Table

Create table component:

```typescript
function RevenueBreakdownTable({ breakdown }: { breakdown: RevenueBreakdown[] }) {
  return (
    <table className="w-full mt-6">
      <thead>
        <tr>
          <th>Integration</th>
          <th>Impact</th>
          <th>Type</th>
          <th>Trend</th>
        </tr>
      </thead>
      <tbody>
        {breakdown.map((item) => (
          <tr key={item.integrationId}>
            <td>{item.integrationName}</td>
            <td>{formatCurrency(item.impact)}</td>
            <td>{formatImpactType(item.impactType)}</td>
            <td><TrendIndicator trend={item.trend} /></td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}
```

### Chart Integration

Use existing `TrendChart` component:
- Pass chart data from API response
- Format data for chart (may need to transform API response)
- Follow existing chart patterns and styling

### Period Selector

Create or reuse period selector:
- Options: 7d, 30d, 90d, 1y
- Default: "30d"
- Update `useRevenueImpact` hook call when period changes
- Use React state to manage selected period

### Time-Series Data

Note: API response from Story 23.3 may not include time-series data for charts. Options:
1. Enhance API to return time-series data (if needed)
2. Use aggregated data for now, add time-series in Phase 2
3. Calculate trend from current vs previous period data

### Route Configuration

Add to `App.tsx`:

```typescript
// frontend/src/App.tsx
import { RevenueImpactPage } from './pages/Dashboard/RevenueImpactPage';

// In router configuration:
<Route path="/dashboard/revenue/impact" element={<RevenueImpactPage />} />
```

### Loading State

Use existing loading patterns:
- Skeleton loader for page structure
- Match existing page loading patterns
- Show loading state while data is fetching

### Error Handling

Handle errors gracefully:
- Show error message or fallback UI
- Use existing error handling patterns
- Consider retry logic in React Query

---

## Testing Strategy

### Unit Tests

- **Component Rendering**: Test RevenueImpactPage renders correctly
- **Breakdown Table**: Test table displays correct data
- **Charts**: Test charts render correctly (if unit testable)
- **Period Selector**: Test period selection works correctly

### Integration Tests

- **Route Integration**: Test route works correctly
- **Navigation**: Test navigation from KPI cards works
- **Data Loading**: Test data loads correctly with period changes
- **Charts**: Test charts display correctly with data

### Manual Tests

- Verify page displays correctly
- Verify breakdown table shows correct data
- Verify charts render correctly
- Verify period selector works and updates data
- Verify loading state displays correctly
- Verify error handling works
- Verify navigation from KPI cards works

---

## Success Metrics

- Page displays correctly with all sections
- Breakdown table shows accurate data
- Charts render correctly
- Period selector works and updates data
- Loading states work correctly
- Error handling works correctly
- Navigation works correctly

---

## Context & Dependencies

**Dependencies:**
- Story 23.4: Revenue Impact KPI Cards (navigation source)

**Enables:**
- Future enhancements (export functionality, advanced analytics)

**Related Documentation:**
- Architecture Document: `_bmad-output/planning-artifacts/architecture-observability-pms-integrations-2026-01-10.md`
- Epic Document: `_bmad-output/planning-artifacts/epics-observability-pms-integrations-2026-01-10.md`
- PRD: `_bmad-output/planning-artifacts/prd-observability-pms-integrations-2026-01-10.md` (FR-2.5)

**Project Context:**
- See `_bmad-output/planning-artifacts/project-context.md` for frontend patterns
- Follow existing patterns from `DashboardPage.tsx`, `PMDashboardPage.tsx`
- Use React Query for data fetching (existing pattern)
- Use React Router for routing (existing pattern)
- Use Recharts for charts (existing dependency)

---

## Dev Notes

### Key Implementation Details

1. **Page Structure**: Follow existing page patterns (DashboardPage, PMDashboardPage)
2. **Breakdown Table**: Create table component with proper styling
3. **Charts**: Reuse existing chart components (TrendChart)
4. **Period Selector**: Create or reuse period selector component
5. **Time-Series Data**: May need API enhancement for chart data (can use aggregated data for now)

### Future Enhancements

- Export data (CSV, PDF) - marked as future/optional
- Advanced filtering and sorting
- Integration-specific drill-down
- Historical comparison views

### Chart Data

API response may need enhancement for time-series chart data:
- Option 1: Enhance API to return time-series data
- Option 2: Use aggregated data for now, add time-series in Phase 2
- Option 3: Calculate trend from current vs previous period data

---

**Story Status:** `ready-for-dev`  
**Last Updated:** 2026-01-11  
**Next Review:** When moving to `in-progress`
