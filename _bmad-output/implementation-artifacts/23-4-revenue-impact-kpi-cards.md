# Story 23.4: Revenue Impact KPI Cards

**Status:** `ready-for-dev`

## Story

**As a** QA Engineer working on PMS integration quality  
**I want** to see revenue impact metrics as KPI cards on the dashboard  
**So that** I can quickly understand the financial impact of integration issues at a glance

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 23.4 |
| Epic | Epic 23: Revenue Impact Calculator and Dashboard |
| Sprint | Sprint 2: Revenue API and Dashboard |
| Priority | P0 |
| Estimated Days | 1 |
| Dependencies | Story 23.3 (Revenue Impact API Endpoint) |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create `RevenueImpactCards` component
   - Location: `frontend/src/components/dashboard/RevenueImpactCards.tsx`
   - Reuse existing `KPICard` component from `KPICards.tsx`

2. Revenue At Risk KPI card:
   - Shows current revenue at risk (from API response `revenue_at_risk.value`)
   - Display format: Currency format (e.g., "$1,234.56")
   - Trend indicator: up/down/neutral (from `revenue_at_risk.trend`)
   - Percentage change: from `revenue_at_risk.change`

3. Revenue Protected KPI card:
   - Shows revenue protected by QA testing (from API response `revenue_protected.value`)
   - Display format: Currency format (e.g., "$12,345.67")
   - Trend indicator: up/down/neutral (from `revenue_protected.trend`)
   - Percentage change: from `revenue_protected.change`

4. Trend indicators:
   - Use existing trend indicator patterns from `KPICard` component
   - Colors: green (up), red (down), gray (neutral)
   - Icons: arrow-up, arrow-down, dash (for neutral)

5. Percentage change display:
   - Show change percentage from previous period
   - Format: "+5.2%" or "-3.1%" (with +/- prefix)
   - Color coding: green (positive), red (negative), gray (neutral)

6. Click navigation to Revenue Detail Page:
   - Navigate to `/dashboard/revenue/impact` route (Story 23.5)
   - Use React Router `useNavigate()` hook
   - Add click handler to KPI cards

7. Create `useRevenueImpact` hook:
   - Location: `frontend/src/hooks/useRevenueImpact.ts`
   - Use React Query for data fetching
   - Call GET /api/v1/revenue/impact endpoint
   - Support period parameter (default: "30d")
   - Handle loading and error states

8. Integrate into Dashboard page:
   - Add `RevenueImpactCards` component to `DashboardPage.tsx`
   - Place cards in appropriate location (below existing KPI cards or in separate section)
   - Follow existing layout patterns (grid layout)

---

## Acceptance Criteria

- [ ] **Given** dashboard page loaded  
  **When** viewing dashboard  
  **Then** revenue impact KPI cards are displayed

- [ ] **Given** KPI cards displayed  
  **When** revenue at risk calculated  
  **Then** "Revenue At Risk" card shows correct value in currency format

- [ ] **Given** KPI cards displayed  
  **When** revenue protected calculated  
  **Then** "Revenue Protected" card shows correct value in currency format

- [ ] **Given** KPI cards displayed  
  **When** trend calculated  
  **Then** trend indicators are displayed correctly (up/down/neutral with appropriate colors)

- [ ] **Given** KPI cards displayed  
  **When** clicking on card  
  **Then** navigates to Revenue Detail Page (`/dashboard/revenue/impact`)

- [ ] **Given** KPI cards displayed  
  **When** data loading  
  **Then** loading state is displayed (skeleton or spinner)

---

## Tasks

- [ ] Task 1: Create useRevenueImpact hook (AC: #6)
  - [ ] 1.1: Create `frontend/src/hooks/useRevenueImpact.ts`
  - [ ] 1.2: Use React Query `useQuery` for data fetching
  - [ ] 1.3: Call GET /api/v1/revenue/impact endpoint
  - [ ] 1.4: Support period parameter (default: "30d")
  - [ ] 1.5: Return data, loading, error states
  - [ ] 1.6: Define TypeScript types for API response

- [ ] Task 2: Create RevenueImpactCards component (AC: #1, #2, #3)
  - [ ] 2.1: Create `frontend/src/components/dashboard/RevenueImpactCards.tsx`
  - [ ] 2.2: Import and use `useRevenueImpact` hook
  - [ ] 2.3: Create Revenue At Risk KPI card using `KPICard` component
  - [ ] 2.4: Create Revenue Protected KPI card using `KPICard` component
  - [ ] 2.5: Format currency values (use Intl.NumberFormat or utility function)
  - [ ] 2.6: Display trend indicators (use existing patterns from KPICards)

- [ ] Task 3: Add click navigation (AC: #5)
  - [ ] 3.1: Import `useNavigate` from react-router-dom
  - [ ] 3.2: Add click handler to KPI cards
  - [ ] 3.3: Navigate to `/dashboard/revenue/impact` on click
  - [ ] 3.4: Add cursor pointer style to cards

- [ ] Task 4: Add loading state (AC: #6)
  - [ ] 4.1: Show loading skeleton or spinner when data loading
  - [ ] 4.2: Use existing loading patterns from DashboardPage
  - [ ] 4.3: Handle error state (show error message or fallback)

- [ ] Task 5: Integrate into Dashboard page (AC: #1)
  - [ ] 5.1: Import `RevenueImpactCards` in `DashboardPage.tsx`
  - [ ] 5.2: Add component to dashboard layout
  - [ ] 5.3: Place cards in appropriate location (below existing KPIs or separate section)
  - [ ] 5.4: Follow existing grid layout patterns

- [ ] Task 6: Export component (AC: #1)
  - [ ] 6.1: Add export to `frontend/src/components/dashboard/index.ts`
  - [ ] 6.2: Verify component exported correctly

---

## Files to Create

| File | Changes |
|------|---------|
| `frontend/src/components/dashboard/RevenueImpactCards.tsx` | Create RevenueImpactCards component with two KPI cards |
| `frontend/src/hooks/useRevenueImpact.ts` | Create React Query hook for revenue impact data |

---

## Files to Modify

| File | Changes |
|------|---------|
| `frontend/src/pages/Dashboard/DashboardPage.tsx` | Add RevenueImpactCards component to dashboard layout |
| `frontend/src/components/dashboard/index.ts` | Export RevenueImpactCards component |

---

## Implementation Notes

### Component Structure

Follow existing patterns from `KPICards.tsx`:

```typescript
// frontend/src/components/dashboard/RevenueImpactCards.tsx
import { useRevenueImpact } from '@/hooks/useRevenueImpact';
import { KPICard } from './KPICard';
import { useNavigate } from 'react-router-dom';

export function RevenueImpactCards() {
  const { data, isLoading, error } = useRevenueImpact();
  const navigate = useNavigate();

  if (isLoading) {
    return <RevenueImpactCardsSkeleton />;
  }

  if (error) {
    return <div>Error loading revenue impact data</div>;
  }

  if (!data) {
    return null;
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
      <KPICard
        title="Revenue At Risk"
        value={formatCurrency(data.revenueAtRisk.value)}
        change={data.revenueAtRisk.change}
        trend={data.revenueAtRisk.trend}
        onClick={() => navigate('/dashboard/revenue/impact')}
      />
      <KPICard
        title="Revenue Protected"
        value={formatCurrency(data.revenueProtected.value)}
        change={data.revenueProtected.change}
        trend={data.revenueProtected.trend}
        onClick={() => navigate('/dashboard/revenue/impact')}
      />
    </div>
  );
}
```

### React Query Hook

Follow existing patterns from other hooks:

```typescript
// frontend/src/hooks/useRevenueImpact.ts
import { useQuery } from '@tanstack/react-query';
import { api } from '@/lib/api';

export function useRevenueImpact(period: string = '30d') {
  return useQuery({
    queryKey: ['revenue-impact', period],
    queryFn: async () => {
      const response = await api.get(`/api/v1/revenue/impact?period=${period}`);
      return response.data;
    },
  });
}
```

### Currency Formatting

Use `Intl.NumberFormat` for currency formatting:

```typescript
function formatCurrency(value: number): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(value);
}
```

### KPICard Component Reuse

Reuse existing `KPICard` component from `KPICards.tsx`:
- Pass title, value, change, trend as props
- Add onClick handler for navigation
- KPICard should handle trend indicators, percentage change display

### Dashboard Integration

Add to `DashboardPage.tsx`:
- Import `RevenueImpactCards` component
- Add to dashboard layout (below existing KPICards or in separate section)
- Follow existing grid layout patterns (grid-cols-1 md:grid-cols-2 lg:grid-cols-4)

### TypeScript Types

Define types for API response:

```typescript
// frontend/src/types/revenue.ts (or add to existing types file)
export interface RevenueImpact {
  revenueAtRisk: KPIMetric;
  revenueProtected: KPIMetric;
  breakdown: RevenueBreakdown[];
}

export interface KPIMetric {
  value: number;
  change: number;
  trend: 'up' | 'down' | 'neutral';
}

export interface RevenueBreakdown {
  integrationId: string;
  integrationName: string;
  impact: number;
  impactType: string;
  estimatedLoss: number;
  trend: string;
}
```

### Loading State

Use existing loading patterns:
- Skeleton loader (preferred) or spinner
- Match existing KPI card dimensions
- Show loading state while data is fetching

### Error Handling

Handle errors gracefully:
- Show error message or fallback UI
- Use existing error handling patterns from DashboardPage
- Consider retry logic in React Query (default retry behavior)

---

## Testing Strategy

### Unit Tests

- **Component Rendering**: Test RevenueImpactCards renders correctly
- **KPI Cards**: Test KPI cards display correct values, trends, changes
- **Currency Formatting**: Test currency formatting function
- **Navigation**: Test click navigation to detail page

### Integration Tests

- **Dashboard Integration**: Test RevenueImpactCards integrates with DashboardPage
- **API Integration**: Test useRevenueImpact hook fetches data correctly
- **Loading States**: Test loading state displays correctly
- **Error States**: Test error handling works correctly

### Manual Tests

- Verify KPI cards display correctly on dashboard
- Verify currency values formatted correctly
- Verify trend indicators display correctly (colors, icons)
- Verify click navigation works
- Verify loading state displays while data fetching
- Verify error handling works when API fails

---

## Success Metrics

- KPI cards display correctly with accurate values
- Trend indicators clear and accurate
- Navigation works smoothly
- Loading states work correctly
- Error handling works correctly
- Ready for Story 23.5 (Revenue Impact Detail Page)

---

## Context & Dependencies

**Dependencies:**
- Story 23.3: Revenue Impact API Endpoint (provides API endpoint)

**Enables:**
- Story 23.5: Revenue Impact Detail Page (navigation target)

**Related Documentation:**
- Architecture Document: `_bmad-output/planning-artifacts/architecture-observability-pms-integrations-2026-01-10.md`
- Epic Document: `_bmad-output/planning-artifacts/epics-observability-pms-integrations-2026-01-10.md`
- PRD: `_bmad-output/planning-artifacts/prd-observability-pms-integrations-2026-01-10.md` (FR-2.3)

**Project Context:**
- See `_bmad-output/planning-artifacts/project-context.md` for frontend patterns
- Follow existing patterns from `KPICards.tsx`, `DashboardPage.tsx`
- Use React Query for data fetching (existing pattern)
- Use React Router for navigation (existing pattern)

---

## Dev Notes

### Key Implementation Details

1. **Component Reuse**: Reuse existing `KPICard` component for consistency
2. **Currency Formatting**: Use `Intl.NumberFormat` for proper currency formatting
3. **Trend Indicators**: Use existing trend indicator patterns from KPICards
4. **Navigation**: Use React Router `useNavigate` for navigation
5. **Data Fetching**: Use React Query for data fetching (existing pattern)

### Dashboard Layout

- Place RevenueImpactCards below existing KPICards or in separate section
- Follow existing grid layout patterns
- Ensure responsive design (mobile, tablet, desktop)

### Future Enhancements

- Period selector (7d, 30d, 90d, 1y) - can be added in Phase 2
- Tooltips with additional information
- Drill-down to integration-specific breakdown

---

**Story Status:** `ready-for-dev`  
**Last Updated:** 2026-01-11  
**Next Review:** When moving to `in-progress`
