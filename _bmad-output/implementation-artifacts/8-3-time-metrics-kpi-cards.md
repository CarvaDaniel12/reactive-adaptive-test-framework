# Story 8.3: Time Metrics KPI Cards

Status: done

## Story

As a QA (Ana),
I want to see my time efficiency metrics,
So that I can prove my capacity.

## Acceptance Criteria

1. **Given** user views dashboard
   **When** time metrics cards render
   **Then** average time per ticket is displayed

2. **Given** time metrics displayed
   **When** efficiency shown
   **Then** time actual vs estimated ratio is displayed (e.g., 0.92x)

3. **Given** time metrics displayed
   **When** total hours shown
   **Then** total hours worked in period is visible

4. **Given** time metrics displayed
   **When** trend shown
   **Then** efficiency trend over time is displayed

5. **Given** efficiency ratio
   **When** color coded
   **Then** uses color coding: 🟢 ≤1.0, 🟡 1.0-1.2, 🔴 >1.2

6. **Given** card is interactive
   **When** clicked
   **Then** clicking reveals per-ticket breakdown

## Tasks

- [ ] Task 1: Create time metrics API endpoint
- [ ] Task 2: Calculate average time per ticket
- [ ] Task 3: Calculate efficiency ratio
- [ ] Task 4: Create EfficiencyCard component
- [ ] Task 5: Create TotalHoursCard component
- [ ] Task 6: Implement per-ticket breakdown view

## Dev Notes

### API Response

```rust
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeMetricsKPI {
    pub avg_time_per_ticket: AvgTimeMetric,
    pub efficiency_ratio: EfficiencyMetric,
    pub total_hours: TotalHoursMetric,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvgTimeMetric {
    pub current_seconds: i32,
    pub previous_seconds: i32,
    pub change_percent: f64,
    pub trend: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EfficiencyMetric {
    pub current_ratio: f64,
    pub previous_ratio: f64,
    pub level: String, // "good", "warning", "critical"
    pub trend: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalHoursMetric {
    pub current_hours: f64,
    pub previous_hours: f64,
    pub change_percent: f64,
    pub trend: String,
}
```

### Service Implementation

```rust
impl DashboardService {
    pub async fn get_time_metrics(&self, period: &str) -> Result<TimeMetricsKPI> {
        let (start_current, end_current, start_previous, end_previous) = 
            Self::calculate_period_ranges(period);

        // Current period aggregates
        let current = sqlx::query_as::<_, TimeAggregates>(
            r#"
            SELECT 
                COALESCE(SUM(total_active_seconds), 0) as total_active,
                COALESCE(SUM(total_estimated_seconds), 0) as total_estimated,
                COUNT(DISTINCT wi.id) as workflow_count
            FROM workflow_instances wi
            JOIN time_sessions ts ON ts.workflow_instance_id = wi.id
            WHERE wi.status = 'completed'
              AND wi.completed_at BETWEEN $1 AND $2
            "#,
        )
        .bind(start_current)
        .bind(end_current)
        .fetch_one(&self.pool)
        .await?;

        // Previous period aggregates
        let previous = sqlx::query_as::<_, TimeAggregates>(/* same query */)
            .bind(start_previous)
            .bind(end_previous)
            .fetch_one(&self.pool)
            .await?;

        // Calculate metrics
        let current_avg = if current.workflow_count > 0 {
            current.total_active / current.workflow_count as i32
        } else { 0 };

        let previous_avg = if previous.workflow_count > 0 {
            previous.total_active / previous.workflow_count as i32
        } else { 0 };

        let current_ratio = if current.total_estimated > 0 {
            current.total_active as f64 / current.total_estimated as f64
        } else { 1.0 };

        let previous_ratio = if previous.total_estimated > 0 {
            previous.total_active as f64 / previous.total_estimated as f64
        } else { 1.0 };

        let efficiency_level = if current_ratio <= 1.0 { "good" }
            else if current_ratio <= 1.2 { "warning" }
            else { "critical" };

        Ok(TimeMetricsKPI {
            avg_time_per_ticket: AvgTimeMetric {
                current_seconds: current_avg,
                previous_seconds: previous_avg,
                change_percent: Self::calc_change(current_avg, previous_avg),
                trend: Self::calc_trend(current_avg, previous_avg, true), // lower is better
            },
            efficiency_ratio: EfficiencyMetric {
                current_ratio,
                previous_ratio,
                level: efficiency_level.into(),
                trend: Self::calc_trend(current_ratio, previous_ratio, true),
            },
            total_hours: TotalHoursMetric {
                current_hours: current.total_active as f64 / 3600.0,
                previous_hours: previous.total_active as f64 / 3600.0,
                change_percent: Self::calc_change(current.total_active, previous.total_active),
                trend: Self::calc_trend(current.total_active, previous.total_active, false),
            },
        })
    }
}
```

### Efficiency Card Component

```tsx
// frontend/src/components/dashboard/EfficiencyCard.tsx
interface EfficiencyCardProps {
  data?: EfficiencyMetric;
  isLoading: boolean;
}

export function EfficiencyCard({ data, isLoading }: EfficiencyCardProps) {
  const navigate = useNavigate();

  if (isLoading) return <KPICardSkeleton />;

  const level = data?.level || "good";
  const levelColors = {
    good: { bg: "bg-success-50", text: "text-success-600", border: "border-success-200" },
    warning: { bg: "bg-warning-50", text: "text-warning-600", border: "border-warning-200" },
    critical: { bg: "bg-error-50", text: "text-error-600", border: "border-error-200" },
  };

  const colors = levelColors[level as keyof typeof levelColors];
  const ratio = data?.currentRatio || 1;

  return (
    <button
      onClick={() => navigate("/dashboard/efficiency")}
      className={cn(
        "w-full text-left rounded-xl border p-5 transition-all cursor-pointer",
        colors.border,
        "hover:shadow-md"
      )}
    >
      <div className="flex items-start justify-between mb-4">
        <div className={cn("p-2 rounded-lg", colors.bg, colors.text)}>
          <LightningBoltIcon className="w-5 h-5" />
        </div>
        <EfficiencyIndicator level={level} />
      </div>

      <div className="space-y-1">
        <p className={cn("text-3xl font-bold", colors.text)}>
          {ratio.toFixed(2)}x
        </p>
        <p className="text-sm text-neutral-500">Efficiency Ratio</p>
        <p className="text-xs text-neutral-400">
          Actual vs estimated time
        </p>
      </div>

      {/* Mini bar visualization */}
      <div className="mt-4 space-y-1">
        <div className="flex justify-between text-xs text-neutral-500">
          <span>Target: 1.0x</span>
          <span>Current: {ratio.toFixed(2)}x</span>
        </div>
        <div className="h-2 bg-neutral-100 rounded-full overflow-hidden">
          <div
            className={cn("h-full rounded-full transition-all", 
              level === "good" ? "bg-success-500" :
              level === "warning" ? "bg-warning-500" : "bg-error-500"
            )}
            style={{ width: `${Math.min(ratio / 1.5 * 100, 100)}%` }}
          />
        </div>
      </div>
    </button>
  );
}

function EfficiencyIndicator({ level }: { level: string }) {
  const icons = { good: "🟢", warning: "🟡", critical: "🔴" };
  const labels = { good: "On target", warning: "Slightly over", critical: "Over estimate" };

  return (
    <div className="flex items-center gap-1.5 text-xs">
      <span>{icons[level as keyof typeof icons]}</span>
      <span className="text-neutral-500">{labels[level as keyof typeof labels]}</span>
    </div>
  );
}
```

### Per-Ticket Breakdown View

```tsx
// frontend/src/pages/Dashboard/EfficiencyDetail.tsx
export function EfficiencyDetailPage() {
  const [period, setPeriod] = useState<Period>("30d");
  const { data, isLoading } = useEfficiencyDetail(period);

  return (
    <div className="p-6">
      <div className="flex items-center justify-between mb-6">
        <div>
          <Link to="/dashboard" className="text-sm text-primary-600">← Back</Link>
          <h1 className="text-2xl font-bold mt-2">Efficiency Breakdown</h1>
        </div>
        <PeriodSelector value={period} onChange={setPeriod} />
      </div>

      {/* Per-Ticket Table */}
      <div className="bg-white rounded-xl border">
        <table className="w-full">
          <thead className="bg-neutral-50">
            <tr>
              <th className="text-left p-4">Ticket</th>
              <th className="text-right p-4">Actual</th>
              <th className="text-right p-4">Estimated</th>
              <th className="text-right p-4">Ratio</th>
              <th className="text-center p-4">Status</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-neutral-100">
            {data?.tickets.map((ticket) => (
              <tr key={ticket.id} className="hover:bg-neutral-50">
                <td className="p-4">
                  <span className="font-mono text-sm">{ticket.ticketKey}</span>
                </td>
                <td className="p-4 text-right">
                  {formatDuration(ticket.actualSeconds)}
                </td>
                <td className="p-4 text-right text-neutral-500">
                  {formatDuration(ticket.estimatedSeconds)}
                </td>
                <td className="p-4 text-right font-medium">
                  {ticket.ratio.toFixed(2)}x
                </td>
                <td className="p-4 text-center">
                  <EfficiencyBadge ratio={ticket.ratio} />
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
```

### References

- [Source: epics.md#Story 8.3]
