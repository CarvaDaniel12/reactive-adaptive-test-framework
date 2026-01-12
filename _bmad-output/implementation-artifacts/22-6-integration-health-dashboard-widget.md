# Story 22.6: Integration Health Dashboard Widget

**Status:** `review`

## Story

**As a** QA Engineer working on PMS integration quality  
**I want** to see integration health status in the dashboard  
**So that** I can quickly identify integration issues

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 22.6 |
| Epic | Epic 22: PMS Integration Health Monitoring Module |
| Sprint | Sprint 4: Dashboard Integration |
| Priority | P0 |
| Estimated Days | 2 |
| Dependencies | Story 22.5 |
| Status | `review` |

---

## Technical Requirements

1. Create `IntegrationHealthWidget` component
   - Follow existing dashboard component patterns
   - Use React Query for data fetching
   - Use existing UI components (Radix UI, Tailwind CSS)

2. Display status cards for each integration (Booking.com, Airbnb, Vrbo, HMBN)
   - Status cards with integration name and status
   - Status indicators: 🟢 OK (healthy), 🟡 Warning, 🔴 Critical
   - Sub-status: Pricing Sync, Fees Sync, Booking Loss

3. Status indicators: 🟢 OK, 🟡 Warning, 🔴 Critical
   - Color-coded status indicators
   - Accessible labels (aria-label)
   - Visual indicators (emoji or icons)

4. Sub-status: Pricing Sync, Fees Sync, Booking Loss
   - Display sub-status for each integration
   - Color-coded sub-status indicators
   - Compact display (tooltip or inline)

5. Click navigation to Integration Detail Page
   - Click on integration card navigates to detail page
   - Use React Router for navigation
   - Route: `/dashboard/integrations/:integrationId`

6. Use existing dashboard components/patterns
   - Follow patterns from `KPICards`, `TrendChart`, etc.
   - Use existing hooks patterns (`useDashboardData`)
   - Use existing styling (Tailwind CSS)

7. Integrate into existing DashboardPage
   - Add widget section to DashboardPage
   - Position widget below KPI cards
   - Maintain responsive layout

---

## Acceptance Criteria

- [x] **Given** dashboard page exists  
  **When** page is loaded  
  **Then** integration health widget is displayed

- [x] **Given** widget is displayed  
  **When** integrations have different statuses  
  **Then** status indicators are displayed correctly (🟢/🟡/🔴)

- [x] **Given** widget is displayed  
  **When** clicking on integration card  
  **Then** navigates to Integration Detail Page

- [x] **Given** widget is displayed  
  **When** data is loading  
  **Then** loading state is displayed

- [x] **Given** widget is displayed  
  **When** error occurs  
  **Then** error state is displayed

- [x] **Given** widget is displayed  
  **When** viewed on different screens  
  **Then** widget is responsive (single/dual monitor)

---

## Tasks

- [x] Task 1: Create React Query hook (AC: #1, #4, #5)
  - [x] 1.1: Create `frontend/src/hooks/useIntegrationHealth.ts`
  - [x] 1.2: Define `IntegrationHealth` and `IntegrationHealthResponse` types
  - [x] 1.3: Implement `fetchIntegrationHealth` function
  - [x] 1.4: Implement `useIntegrationHealth` hook with React Query
  - [x] 1.5: Add refetch interval (60 seconds)
  - [x] 1.6: Handle loading and error states

- [x] Task 2: Create IntegrationHealthCard component (AC: #2, #3)
  - [x] 2.1: Create `frontend/src/components/dashboard/IntegrationHealthCard.tsx`
  - [x] 2.2: Display integration name and status indicator (🟢/🟡/🔴)
  - [x] 2.3: Display sub-status (Pricing Sync, Fees Sync, Booking Loss, Error Rate)
  - [x] 2.4: Add click handler for navigation
  - [x] 2.5: Add accessibility labels (aria-label)
  - [x] 2.6: Style with Tailwind CSS (existing patterns)

- [x] Task 3: Create IntegrationHealthWidget component (AC: #1, #2, #3)
  - [x] 3.1: Create `frontend/src/components/dashboard/IntegrationHealthWidget.tsx`
  - [x] 3.2: Use `useIntegrationHealth` hook
  - [x] 3.3: Display status cards for each integration (grid layout)
  - [x] 3.4: Add loading state (skeleton component)
  - [x] 3.5: Add error state
  - [x] 3.6: Handle navigation to detail page (onClick)
  - [x] 3.7: Make widget responsive (1/2/4 columns)

- [x] Task 4: Create IntegrationHealthSkeleton component (AC: #4)
  - [x] 4.1: Create skeleton component matching card dimensions
  - [x] 4.2: Use existing skeleton patterns from dashboard
  - [x] 4.3: Display 4 skeleton cards (one per integration)

- [x] Task 5: Integrate widget into DashboardPage (AC: #1)
  - [x] 5.1: Import `IntegrationHealthWidget` in `DashboardPage.tsx`
  - [x] 5.2: Add widget section below KPI cards
  - [x] 5.3: Maintain responsive layout
  - [x] 5.4: Export widget from `components/dashboard/index.ts`

- [x] Task 6: Add type definitions
  - [x] 6.1: Types defined in `useIntegrationHealth.ts` hook
  - [x] 6.2: Define `IntegrationHealth` interface
  - [x] 6.3: Ensure types match API response (camelCase)

---

## Files to Create

| File | Changes |
|------|---------|
| `frontend/src/components/dashboard/IntegrationHealthWidget.tsx` | Create integration health widget component |
| `frontend/src/hooks/useIntegrationHealth.ts` | Create React Query hook for integration health data |

---

## Files to Modify

| File | Changes |
|------|---------|
| `frontend/src/pages/Dashboard/DashboardPage.tsx` | Add IntegrationHealthWidget section |
| `frontend/src/components/dashboard/index.ts` | Export IntegrationHealthWidget |

---

## Implementation Notes

### Component Structure

Follow existing patterns from `KPICards.tsx`:

```tsx
// frontend/src/components/dashboard/IntegrationHealthWidget.tsx
import { useNavigate } from "react-router-dom";
import { useIntegrationHealth } from "@/hooks/useIntegrationHealth";
import { IntegrationHealthCard } from "./IntegrationHealthCard";
import { IntegrationHealthSkeleton } from "./IntegrationHealthSkeleton";

export function IntegrationHealthWidget() {
  const navigate = useNavigate();
  const { data, isLoading, error } = useIntegrationHealth();

  if (isLoading) {
    return <IntegrationHealthSkeleton />;
  }

  if (error) {
    return (
      <div className="p-4 bg-error-50 border border-error-200 rounded-lg">
        <p className="text-sm text-error-600">Failed to load integration health</p>
      </div>
    );
  }

  const integrations = data?.integrations || [];

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-semibold text-neutral-900">Integration Health</h2>
      </div>
      
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {integrations.map((integration) => (
          <IntegrationHealthCard
            key={integration.integrationId}
            integration={integration}
            onClick={() => navigate(`/dashboard/integrations/${integration.integrationId}`)}
          />
        ))}
      </div>
    </div>
  );
}
```

### Integration Health Card

```tsx
// frontend/src/components/dashboard/IntegrationHealthCard.tsx
interface IntegrationHealthCardProps {
  integration: IntegrationHealth;
  onClick: () => void;
}

export function IntegrationHealthCard({ integration, onClick }: IntegrationHealthCardProps) {
  const statusColor = {
    healthy: "text-success-500",
    warning: "text-warning-500",
    critical: "text-error-500",
  }[integration.status.toLowerCase()] || "text-neutral-500";

  const statusIcon = {
    healthy: "🟢",
    warning: "🟡",
    critical: "🔴",
  }[integration.status.toLowerCase()] || "⚪";

  return (
    <button
      onClick={onClick}
      className="w-full p-4 bg-white border border-neutral-200 rounded-lg hover:shadow-md transition-shadow text-left"
      aria-label={`${integration.integrationId} integration status: ${integration.status}`}
    >
      <div className="flex items-center justify-between mb-2">
        <h3 className="text-sm font-medium text-neutral-900 capitalize">
          {integration.integrationId.replace("-", " ")}
        </h3>
        <span className="text-lg" role="img" aria-label={integration.status}>
          {statusIcon}
        </span>
      </div>
      
      <div className="space-y-1">
        {integration.pricingSyncStatus && (
          <div className="flex items-center gap-2 text-xs text-neutral-600">
            <span>Pricing:</span>
            <span className={statusColor}>
              {integration.pricingSyncStatus === "ok" ? "✅" : "⚠️"}
            </span>
          </div>
        )}
        
        {integration.feesSyncStatus && (
          <div className="flex items-center gap-2 text-xs text-neutral-600">
            <span>Fees:</span>
            <span className={statusColor}>
              {integration.feesSyncStatus === "ok" ? "✅" : "⚠️"}
            </span>
          </div>
        )}
        
        {integration.bookingLossRate !== null && (
          <div className="flex items-center gap-2 text-xs text-neutral-600">
            <span>Booking Loss:</span>
            <span className={statusColor}>
              {(integration.bookingLossRate * 100).toFixed(2)}%
            </span>
          </div>
        )}
      </div>
    </button>
  );
}
```

### React Query Hook

Follow patterns from `useDashboardData.ts`:

```tsx
// frontend/src/hooks/useIntegrationHealth.ts
import { useQuery } from "@tanstack/react-query";
import { apiClient } from "@/lib/api-client";

interface IntegrationHealth {
  integrationId: string;
  status: string;
  pricingSyncStatus?: string;
  feesSyncStatus?: string;
  bookingLossRate?: number;
  errorRate?: number;
  lastChecked: string;
  trend: string;
}

interface IntegrationHealthResponse {
  integrations: IntegrationHealth[];
}

export function useIntegrationHealth() {
  return useQuery<IntegrationHealthResponse>({
    queryKey: ["integration-health"],
    queryFn: async () => {
      const response = await apiClient.get("/api/v1/integrations/health");
      return response.data;
    },
    refetchInterval: 60_000, // Refetch every 60 seconds
  });
}
```

### Dashboard Page Integration

Add widget to `DashboardPage.tsx`:

```tsx
// In DashboardPage.tsx
import { IntegrationHealthWidget } from "@/components/dashboard/IntegrationHealthWidget";

export function DashboardPage() {
  // ... existing code ...
  
  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      {/* ... existing header ... */}
      
      {/* KPI Cards */}
      <KPICards data={data?.kpis} isLoading={isLoading} />
      
      {/* Integration Health Widget */}
      <IntegrationHealthWidget />
      
      {/* Main Content Grid */}
      {/* ... existing content ... */}
    </div>
  );
}
```

### Type Definitions

```tsx
// frontend/src/types/integration.ts
export interface IntegrationHealth {
  integrationId: "booking-com" | "airbnb" | "vrbo" | "hmbn";
  status: "healthy" | "warning" | "critical";
  pricingSyncStatus?: "ok" | "warning" | "error";
  feesSyncStatus?: "ok" | "warning" | "error";
  bookingLossRate?: number;
  errorRate?: number;
  lastChecked: string;
  trend: "up" | "down" | "neutral";
}
```

---

## Testing Strategy

### Unit Tests

- **Component Rendering**: Test component renders correctly
- **Status Indicators**: Test status indicators display correctly (🟢/🟡/🔴)
- **Navigation**: Test click navigation works correctly
- **Loading State**: Test loading state displays correctly
- **Error State**: Test error state displays correctly

### Integration Tests

- **Widget Integration**: Test widget integrates with dashboard
- **Data Fetching**: Test React Query hook fetches data correctly
- **Navigation**: Test navigation to detail page works

### Manual Tests

- Test widget displays correctly in dashboard
- Test status indicators are clear and actionable
- Test navigation works smoothly
- Test responsive layout (single/dual monitor)
- Test loading and error states

---

## Success Metrics

- Widget displays correctly (all integrations shown)
- Status indicators clear and actionable (color-coded, accessible)
- Navigation works smoothly (click navigation functional)
- Responsive layout works (single/dual monitor)
- Ready for next story (22.7: Detail Page)

---

## Context & Dependencies

**Dependencies:**
- Story 22.5: Integration Health API Endpoints (API endpoints must exist)

**Enables:**
- Story 22.7: Integration Detail Page (needs navigation route)

**Related Documentation:**
- Architecture Document: `_bmad-output/planning-artifacts/architecture-observability-pms-integrations-2026-01-10.md`
- ADR-004: Dashboard Integration Strategy
- Epic Document: `_bmad-output/planning-artifacts/epics-observability-pms-integrations-2026-01-10.md`
- Design Thinking: `_bmad-output/planning-artifacts/design-thinking-observability-2026-01-10.md` (Wireframe)
- Reference Patterns: `frontend/src/components/dashboard/KPICards.tsx`, `frontend/src/hooks/useDashboardData.ts`

---

---

## Dev Agent Record

### Agent Model Used
Claude Sonnet 4.5 (via Cursor)

### Implementation Notes

**Dashboard Widget:** `frontend/src/components/dashboard/IntegrationHealthWidget.tsx`

**Implementation Summary:**
- Dashboard widget module already existed and was verified to match story requirements
- All components implemented: `IntegrationHealthWidget`, `IntegrationHealthCard`, `IntegrationHealthSkeleton`
- React Query hook `useIntegrationHealth` implemented with refetch interval (60 seconds)
- Widget integrated into `DashboardPage` below KPI cards
- Navigation implemented to Integration Detail Page
- Status indicators with emojis (🟢/🟡/🔴) and color coding
- Sub-status display: Pricing Sync, Fees Sync, Booking Loss Rate, Error Rate
- Loading state with skeleton component (4 cards)
- Error state handling
- Responsive grid layout (1/2/4 columns)
- Accessibility labels (aria-label)
- Widget exported from `components/dashboard/index.ts`
- Types defined in `useIntegrationHealth.ts` hook

**Components:**
- `IntegrationHealthWidget`: Main widget component with data fetching and state management
- `IntegrationHealthCard`: Individual integration card with status indicators and sub-statuses
- `IntegrationHealthSkeleton`: Loading state skeleton component
- `useIntegrationHealth`: React Query hook for data fetching

### File List

**Created:**
- `frontend/src/components/dashboard/IntegrationHealthWidget.tsx` - Widget component (already existed)
- `frontend/src/components/dashboard/IntegrationHealthCard.tsx` - Card component (already existed)
- `frontend/src/components/dashboard/IntegrationHealthSkeleton.tsx` - Skeleton component (already existed)
- `frontend/src/hooks/useIntegrationHealth.ts` - React Query hook (already existed)

**Modified:**
- `frontend/src/pages/Dashboard/DashboardPage.tsx` - Widget integrated (line 119: `<IntegrationHealthWidget />`)
- `frontend/src/components/dashboard/index.ts` - Widget exported (line 6)

### Change Log

**2026-01-11 - Story Implementation Complete:**
- Verified dashboard widget matches story requirements
- All components implemented correctly
- Widget integrated into DashboardPage
- All acceptance criteria satisfied
- All tasks completed

---

**Story Status:** `review`  
**Last Updated:** 2026-01-11  
**Next Review:** Code review workflow
