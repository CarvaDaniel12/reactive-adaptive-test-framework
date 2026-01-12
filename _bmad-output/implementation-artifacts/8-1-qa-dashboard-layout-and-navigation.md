# Story 8.1: QA Dashboard Layout and Navigation

Status: done

## Story

As a QA (Ana),
I want a personal dashboard view,
So that I can see my performance metrics.

## Acceptance Criteria

1. **Given** user has completed workflows
   **When** user navigates to Dashboard
   **Then** KPI cards are displayed at top (tickets completed, avg time, efficiency)

2. **Given** dashboard layout
   **When** rendered
   **Then** trend chart is displayed in middle

3. **Given** dashboard layout
   **When** rendered
   **Then** recent activity list is at bottom

4. **Given** dashboard layout
   **When** rendered
   **Then** period selector is available (7 days, 30 days, 90 days, year)

5. **Given** dashboard is active
   **When** UI renders
   **Then** dashboard uses Dashboard mode (expanded sidebar)

6. **Given** dashboard layout
   **When** viewed on different screens
   **Then** layout is responsive for single/dual monitor

7. **Given** navigation
   **When** user accesses dashboard
   **Then** navigation via sidebar "Dashboard" item

## Tasks

- [x] Task 1: Create DashboardPage component
- [x] Task 2: Create KPICard component
- [x] Task 3: Create TrendChart component (using Recharts)
- [x] Task 4: Create RecentActivityList component
- [x] Task 5: Create PeriodSelector component
- [x] Task 6: Add Dashboard to sidebar navigation
- [x] Task 7: Implement responsive grid layout
- [x] Task 8: Create useDashboardData hook with React Query
- [x] Task 9: Add Dashboard API endpoint (Rust backend)
- [x] Task 10: Implement Dashboard mode (auto-expand sidebar) - AC #5 (completed 2026-01-10)

## Dev Notes

### Dashboard Page Layout

```tsx
// frontend/src/pages/Dashboard.tsx
import { useState } from "react";
import { KPICards } from "@/components/dashboard/KPICards";
import { TrendChart } from "@/components/dashboard/TrendChart";
import { RecentActivity } from "@/components/dashboard/RecentActivity";
import { PeriodSelector, Period } from "@/components/dashboard/PeriodSelector";
import { useDashboardData } from "@/hooks/useDashboardData";

export function DashboardPage() {
  const [period, setPeriod] = useState<Period>("30d");
  const { data, isLoading, refetch } = useDashboardData(period);

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-neutral-900">Dashboard</h1>
          <p className="text-sm text-neutral-500">Your QA performance overview</p>
        </div>
        
        <div className="flex items-center gap-4">
          <PeriodSelector value={period} onChange={setPeriod} />
          <button
            onClick={() => refetch()}
            className="p-2 text-neutral-500 hover:text-neutral-700 
                       hover:bg-neutral-100 rounded-lg"
            title="Refresh"
          >
            <ReloadIcon className="w-5 h-5" />
          </button>
        </div>
      </div>

      {/* KPI Cards */}
      <KPICards data={data?.kpis} isLoading={isLoading} />

      {/* Main Content Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Trend Chart - 2 columns */}
        <div className="lg:col-span-2">
          <TrendChart data={data?.trend} period={period} isLoading={isLoading} />
        </div>

        {/* Recent Activity - 1 column */}
        <div className="lg:col-span-1">
          <RecentActivity activities={data?.recentActivity} isLoading={isLoading} />
        </div>
      </div>
    </div>
  );
}
```

### KPI Cards Component

```tsx
// frontend/src/components/dashboard/KPICards.tsx
interface KPICardsProps {
  data?: DashboardKPIs;
  isLoading: boolean;
}

interface DashboardKPIs {
  ticketsCompleted: {
    value: number;
    change: number;
    trend: "up" | "down" | "neutral";
  };
  avgTimePerTicket: {
    value: number; // seconds
    change: number;
    trend: "up" | "down" | "neutral";
  };
  efficiency: {
    value: number; // ratio
    change: number;
    trend: "up" | "down" | "neutral";
  };
  totalHours: {
    value: number;
    change: number;
    trend: "up" | "down" | "neutral";
  };
}

export function KPICards({ data, isLoading }: KPICardsProps) {
  if (isLoading) {
    return (
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {[1, 2, 3, 4].map((i) => (
          <div key={i} className="h-32 bg-neutral-100 rounded-xl animate-pulse" />
        ))}
      </div>
    );
  }

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      <KPICard
        title="Tickets Completed"
        value={data?.ticketsCompleted.value || 0}
        change={data?.ticketsCompleted.change || 0}
        trend={data?.ticketsCompleted.trend || "neutral"}
        icon={<CheckCircledIcon className="w-5 h-5" />}
      />
      <KPICard
        title="Avg. Time per Ticket"
        value={formatDuration(data?.avgTimePerTicket.value || 0)}
        change={data?.avgTimePerTicket.change || 0}
        trend={data?.avgTimePerTicket.trend || "neutral"}
        icon={<ClockIcon className="w-5 h-5" />}
        invertTrend // Lower is better
      />
      <KPICard
        title="Efficiency"
        value={`${((data?.efficiency.value || 1) * 100).toFixed(0)}%`}
        change={data?.efficiency.change || 0}
        trend={data?.efficiency.trend || "neutral"}
        icon={<RocketIcon className="w-5 h-5" />}
        description="Actual vs estimated time"
      />
      <KPICard
        title="Total Hours"
        value={(data?.totalHours.value || 0).toFixed(1)}
        change={data?.totalHours.change || 0}
        trend={data?.totalHours.trend || "neutral"}
        icon={<TimerIcon className="w-5 h-5" />}
      />
    </div>
  );
}

interface KPICardProps {
  title: string;
  value: string | number;
  change: number;
  trend: "up" | "down" | "neutral";
  icon: React.ReactNode;
  description?: string;
  invertTrend?: boolean;
}

function KPICard({
  title,
  value,
  change,
  trend,
  icon,
  description,
  invertTrend,
}: KPICardProps) {
  const trendColor = invertTrend
    ? trend === "up" ? "text-error-500" : trend === "down" ? "text-success-500" : "text-neutral-500"
    : trend === "up" ? "text-success-500" : trend === "down" ? "text-error-500" : "text-neutral-500";

  const TrendIcon = trend === "up" ? ArrowUpIcon : trend === "down" ? ArrowDownIcon : ArrowRightIcon;

  return (
    <div className="bg-white rounded-xl border border-neutral-200 p-5 hover:shadow-sm transition-shadow">
      <div className="flex items-start justify-between mb-4">
        <div className="p-2 bg-primary-50 rounded-lg text-primary-600">
          {icon}
        </div>
        <div className={cn("flex items-center gap-1 text-sm", trendColor)}>
          <TrendIcon className="w-4 h-4" />
          <span>{change > 0 ? "+" : ""}{change}%</span>
        </div>
      </div>

      <div className="space-y-1">
        <p className="text-2xl font-bold text-neutral-900">{value}</p>
        <p className="text-sm text-neutral-500">{title}</p>
        {description && (
          <p className="text-xs text-neutral-400">{description}</p>
        )}
      </div>
    </div>
  );
}
```

### Period Selector

```tsx
// frontend/src/components/dashboard/PeriodSelector.tsx
export type Period = "7d" | "30d" | "90d" | "1y" | "custom";

interface PeriodSelectorProps {
  value: Period;
  onChange: (period: Period) => void;
}

const periods: { value: Period; label: string }[] = [
  { value: "7d", label: "7 days" },
  { value: "30d", label: "30 days" },
  { value: "90d", label: "90 days" },
  { value: "1y", label: "This year" },
];

export function PeriodSelector({ value, onChange }: PeriodSelectorProps) {
  return (
    <div className="flex items-center bg-neutral-100 rounded-lg p-1">
      {periods.map((period) => (
        <button
          key={period.value}
          onClick={() => onChange(period.value)}
          className={cn(
            "px-3 py-1.5 text-sm font-medium rounded-md transition-all",
            value === period.value
              ? "bg-white text-neutral-900 shadow-sm"
              : "text-neutral-500 hover:text-neutral-700"
          )}
        >
          {period.label}
        </button>
      ))}
    </div>
  );
}
```

### Dashboard API

```rust
// GET /api/v1/dashboard
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardResponse {
    pub kpis: DashboardKPIs,
    pub trend: Vec<TrendDataPoint>,
    pub recent_activity: Vec<ActivityItem>,
}

#[utoipa::path(
    get,
    path = "/api/v1/dashboard",
    params(
        ("period" = String, Query, description = "7d, 30d, 90d, 1y")
    ),
    responses(
        (status = 200, description = "Dashboard data", body = DashboardResponse),
    ),
    tag = "dashboard"
)]
pub async fn get_dashboard(
    State(state): State<Arc<AppState>>,
    Query(query): Query<DashboardQuery>,
) -> Result<Json<DashboardResponse>, ApiError> {
    let service = DashboardService::new(state.db_pool.clone());
    let data = service.get_dashboard_data(&query.period).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;
    Ok(Json(data))
}
```

### Sidebar Navigation Update

```tsx
// Add to sidebar items
{
  label: "Dashboard",
  icon: <DashboardIcon />,
  path: "/dashboard",
  badge: null,
}
```

### Responsive Breakpoints

```css
/* Dashboard responsive grid */
.dashboard-grid {
  display: grid;
  gap: 1.5rem;
}

@media (min-width: 768px) {
  .dashboard-grid { grid-template-columns: repeat(2, 1fr); }
}

@media (min-width: 1024px) {
  .dashboard-grid { grid-template-columns: repeat(4, 1fr); }
}

@media (min-width: 1536px) {
  /* Dual monitor / ultrawide */
  .dashboard-grid { grid-template-columns: repeat(4, 1fr); }
}
```

### References

- [Source: epics.md#Story 8.1]
