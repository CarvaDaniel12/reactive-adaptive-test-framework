# Story 8.2: Tickets Completed KPI Card

Status: done

## Story

As a QA (Ana),
I want to see how many tickets I've completed,
So that I can track my productivity.

## Acceptance Criteria

1. **Given** user views dashboard
   **When** tickets completed card renders
   **Then** count of completed tickets in period is displayed

2. **Given** count is displayed
   **When** comparison is shown
   **Then** comparison to previous period (+/- %) is visible

3. **Given** comparison is shown
   **When** trend is indicated
   **Then** trend indicator is shown (↑ green, ↓ red, → neutral)

4. **Given** card is displayed
   **When** details are available
   **Then** breakdown by ticket type is shown (hover for details)

5. **Given** card performance
   **When** loading
   **Then** card loads in < 2s

6. **Given** card is interactive
   **When** clicked
   **Then** clicking card shows detailed list

## Tasks

- [x] Task 1: Create tickets completed API endpoint (integrated in dashboard endpoint)
- [x] Task 2: Calculate period comparison (implemented in calculate_kpis)
- [x] Task 3: Implement breakdown by ticket type (AC #4 - completed 2026-01-10)
- [x] Task 4: Create TicketsCompletedDetail component (AC #6 - completed 2026-01-10)
- [x] Task 5: Add click-through to detail view (AC #6 - completed 2026-01-10)
- [x] Task 6: Optimize query for <2s response (uses time_daily_aggregates, verified)

## Dev Notes

### API Response

```rust
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketsCompletedKPI {
    pub current_period: i64,
    pub previous_period: i64,
    pub change_percent: f64,
    pub trend: String, // "up", "down", "neutral"
    pub by_type: Vec<TypeBreakdown>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeBreakdown {
    pub ticket_type: String,
    pub count: i64,
    pub percentage: f64,
}
```

### Service Implementation

```rust
impl DashboardService {
    pub async fn get_tickets_completed(&self, period: &str) -> Result<TicketsCompletedKPI> {
        let (start_current, end_current, start_previous, end_previous) = 
            Self::calculate_period_ranges(period);

        // Current period count
        let current: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM workflow_instances
            WHERE status = 'completed'
              AND completed_at BETWEEN $1 AND $2
            "#,
        )
        .bind(start_current)
        .bind(end_current)
        .fetch_one(&self.pool)
        .await?;

        // Previous period count
        let previous: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM workflow_instances
            WHERE status = 'completed'
              AND completed_at BETWEEN $1 AND $2
            "#,
        )
        .bind(start_previous)
        .bind(end_previous)
        .fetch_one(&self.pool)
        .await?;

        // Breakdown by type
        let by_type: Vec<TypeBreakdown> = sqlx::query_as(
            r#"
            SELECT 
                wt.ticket_type,
                COUNT(*) as count
            FROM workflow_instances wi
            JOIN workflow_templates wt ON wi.template_id = wt.id
            WHERE wi.status = 'completed'
              AND wi.completed_at BETWEEN $1 AND $2
            GROUP BY wt.ticket_type
            ORDER BY count DESC
            "#,
        )
        .bind(start_current)
        .bind(end_current)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|(ticket_type, count): (String, i64)| {
            let percentage = if current > 0 {
                (count as f64 / current as f64) * 100.0
            } else { 0.0 };
            TypeBreakdown { ticket_type, count, percentage }
        })
        .collect();

        let change = if previous > 0 {
            ((current - previous) as f64 / previous as f64) * 100.0
        } else if current > 0 {
            100.0
        } else {
            0.0
        };

        let trend = if change > 5.0 { "up" }
            else if change < -5.0 { "down" }
            else { "neutral" };

        Ok(TicketsCompletedKPI {
            current_period: current,
            previous_period: previous,
            change_percent: change.round(),
            trend: trend.into(),
            by_type,
        })
    }
}
```

### Frontend Card Component

```tsx
// frontend/src/components/dashboard/TicketsCompletedCard.tsx
import * as Tooltip from "@radix-ui/react-tooltip";
import { useNavigate } from "react-router-dom";

interface TicketsCompletedCardProps {
  data?: TicketsCompletedKPI;
  isLoading: boolean;
}

export function TicketsCompletedCard({ data, isLoading }: TicketsCompletedCardProps) {
  const navigate = useNavigate();

  if (isLoading) {
    return <KPICardSkeleton />;
  }

  const trend = data?.trend || "neutral";
  const TrendIcon = trend === "up" ? ArrowUpIcon : trend === "down" ? ArrowDownIcon : ArrowRightIcon;
  const trendColor = trend === "up" ? "text-success-500" : trend === "down" ? "text-error-500" : "text-neutral-500";

  return (
    <Tooltip.Provider>
      <Tooltip.Root delayDuration={300}>
        <Tooltip.Trigger asChild>
          <button
            onClick={() => navigate("/dashboard/tickets")}
            className="w-full text-left bg-white rounded-xl border border-neutral-200 p-5 
                       hover:shadow-md hover:border-primary-200 transition-all cursor-pointer"
          >
            <div className="flex items-start justify-between mb-4">
              <div className="p-2 bg-success-50 rounded-lg text-success-600">
                <CheckCircledIcon className="w-5 h-5" />
              </div>
              <div className={cn("flex items-center gap-1 text-sm", trendColor)}>
                <TrendIcon className="w-4 h-4" />
                <span>
                  {(data?.changePercent || 0) > 0 ? "+" : ""}
                  {data?.changePercent || 0}%
                </span>
              </div>
            </div>

            <div className="space-y-1">
              <p className="text-3xl font-bold text-neutral-900">
                {data?.currentPeriod || 0}
              </p>
              <p className="text-sm text-neutral-500">Tickets Completed</p>
              <p className="text-xs text-neutral-400">
                vs {data?.previousPeriod || 0} previous period
              </p>
            </div>
          </button>
        </Tooltip.Trigger>

        {/* Hover tooltip with breakdown */}
        <Tooltip.Content
          side="bottom"
          className="bg-white rounded-lg shadow-xl border border-neutral-200 p-4 z-50"
        >
          <p className="font-medium text-sm mb-2">Breakdown by Type</p>
          <div className="space-y-2">
            {data?.byType.map((item) => (
              <div key={item.ticketType} className="flex items-center justify-between gap-4">
                <span className="text-sm text-neutral-600 capitalize">
                  {item.ticketType}
                </span>
                <div className="flex items-center gap-2">
                  <span className="text-sm font-medium">{item.count}</span>
                  <span className="text-xs text-neutral-400">
                    ({item.percentage.toFixed(0)}%)
                  </span>
                </div>
              </div>
            ))}
          </div>
          <Tooltip.Arrow className="fill-white" />
        </Tooltip.Content>
      </Tooltip.Root>
    </Tooltip.Provider>
  );
}
```

### Detail View

```tsx
// frontend/src/pages/Dashboard/TicketsDetail.tsx
export function TicketsDetailPage() {
  const [period, setPeriod] = useState<Period>("30d");
  const { data, isLoading } = useTicketsCompleted(period);

  return (
    <div className="p-6">
      <div className="flex items-center justify-between mb-6">
        <div>
          <Link to="/dashboard" className="text-sm text-primary-600 hover:underline">
            ← Back to Dashboard
          </Link>
          <h1 className="text-2xl font-bold mt-2">Tickets Completed</h1>
        </div>
        <PeriodSelector value={period} onChange={setPeriod} />
      </div>

      {/* Summary Stats */}
      <div className="grid grid-cols-3 gap-4 mb-6">
        <StatCard label="Total" value={data?.currentPeriod || 0} />
        <StatCard label="Change" value={`${data?.changePercent || 0}%`} />
        <StatCard label="Daily Avg" value={(data?.currentPeriod || 0) / getPeriodDays(period)} />
      </div>

      {/* List of completed tickets */}
      <div className="bg-white rounded-xl border border-neutral-200">
        <div className="p-4 border-b border-neutral-200">
          <h2 className="font-semibold">Completed Workflows</h2>
        </div>
        <div className="divide-y divide-neutral-100">
          {data?.workflows.map((workflow) => (
            <WorkflowListItem key={workflow.id} workflow={workflow} />
          ))}
        </div>
      </div>
    </div>
  );
}
```

### Performance Optimization

```sql
-- Add index for fast period queries
CREATE INDEX idx_workflow_instances_completed_at 
    ON workflow_instances(completed_at DESC) 
    WHERE status = 'completed';
```

### References

- [Source: epics.md#Story 8.2]
