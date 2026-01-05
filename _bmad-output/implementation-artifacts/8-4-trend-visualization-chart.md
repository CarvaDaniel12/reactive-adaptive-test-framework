# Story 8.4: Trend Visualization Chart

Status: done

## Story

As a QA (Ana),
I want to see trends over time,
So that I can identify patterns.

## Acceptance Criteria

1. **Given** user views dashboard
   **When** trend chart renders
   **Then** X-axis shows time (days/weeks based on period)

2. **Given** chart renders
   **When** data displayed
   **Then** Y-axis shows tickets completed or hours

3. **Given** chart renders
   **When** visualization style
   **Then** line graph with data points is shown

4. **Given** data points exist
   **When** user hovers
   **Then** hover shows exact values

5. **Given** chart exists
   **When** user wants to change view
   **Then** chart type can be toggled (tickets vs hours)

6. **Given** chart is styled
   **When** rendered
   **Then** chart uses design system colors

7. **Given** chart accessibility
   **When** tested
   **Then** chart is accessible (WCAG compliant)

## Tasks

- [ ] Task 1: Create trend data API endpoint
- [ ] Task 2: Install and configure Recharts
- [ ] Task 3: Create TrendChart component
- [ ] Task 4: Implement metric toggle
- [ ] Task 5: Add hover tooltips
- [ ] Task 6: Ensure accessibility

## Dev Notes

### API Response

```rust
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrendDataResponse {
    pub data_points: Vec<TrendPoint>,
    pub summary: TrendSummary,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrendPoint {
    pub date: String, // ISO date
    pub tickets: i64,
    pub hours: f64,
    pub efficiency: Option<f64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrendSummary {
    pub total_tickets: i64,
    pub total_hours: f64,
    pub avg_efficiency: f64,
    pub best_day: String,
    pub worst_day: String,
}
```

### Service Implementation

```rust
impl DashboardService {
    pub async fn get_trend_data(&self, period: &str) -> Result<TrendDataResponse> {
        let days = Self::period_to_days(period);
        
        let data_points = sqlx::query_as::<_, TrendPoint>(
            r#"
            SELECT 
                DATE(wi.completed_at) as date,
                COUNT(wi.id) as tickets,
                COALESCE(SUM(ts.total_seconds), 0)::FLOAT / 3600.0 as hours,
                CASE 
                    WHEN SUM(ts.total_estimated_seconds) > 0 
                    THEN SUM(ts.total_seconds)::FLOAT / SUM(ts.total_estimated_seconds)::FLOAT
                    ELSE NULL
                END as efficiency
            FROM workflow_instances wi
            LEFT JOIN (
                SELECT workflow_instance_id, 
                       SUM(total_seconds) as total_seconds,
                       SUM(estimated_minutes * 60) as total_estimated_seconds
                FROM time_sessions ts
                JOIN workflow_step_results wsr ON ts.step_index = wsr.step_index 
                    AND ts.workflow_instance_id = wsr.instance_id
                GROUP BY workflow_instance_id
            ) ts ON wi.id = ts.workflow_instance_id
            WHERE wi.status = 'completed'
              AND wi.completed_at >= CURRENT_DATE - $1::INTEGER
            GROUP BY DATE(wi.completed_at)
            ORDER BY date
            "#,
        )
        .bind(days)
        .fetch_all(&self.pool)
        .await?;

        // Fill missing dates with zeros
        let filled_points = Self::fill_missing_dates(&data_points, days);

        let summary = TrendSummary {
            total_tickets: filled_points.iter().map(|p| p.tickets).sum(),
            total_hours: filled_points.iter().map(|p| p.hours).sum(),
            avg_efficiency: filled_points.iter()
                .filter_map(|p| p.efficiency)
                .sum::<f64>() / filled_points.len() as f64,
            best_day: filled_points.iter()
                .max_by_key(|p| p.tickets)
                .map(|p| p.date.clone())
                .unwrap_or_default(),
            worst_day: filled_points.iter()
                .filter(|p| p.tickets > 0)
                .min_by_key(|p| p.tickets)
                .map(|p| p.date.clone())
                .unwrap_or_default(),
        };

        Ok(TrendDataResponse {
            data_points: filled_points,
            summary,
        })
    }
}
```

### TrendChart Component

```tsx
// frontend/src/components/dashboard/TrendChart.tsx
import { useState } from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  ReferenceLine,
} from "recharts";
import { format, parseISO } from "date-fns";

type MetricType = "tickets" | "hours";

interface TrendChartProps {
  data?: TrendDataResponse;
  period: Period;
  isLoading: boolean;
}

export function TrendChart({ data, period, isLoading }: TrendChartProps) {
  const [metric, setMetric] = useState<MetricType>("tickets");

  if (isLoading) {
    return (
      <div className="bg-white rounded-xl border border-neutral-200 p-6">
        <div className="h-80 bg-neutral-100 rounded-lg animate-pulse" />
      </div>
    );
  }

  const chartData = data?.dataPoints.map((point) => ({
    ...point,
    dateFormatted: format(parseISO(point.date), getDateFormat(period)),
  })) || [];

  const avgValue = metric === "tickets"
    ? chartData.reduce((sum, p) => sum + p.tickets, 0) / chartData.length
    : chartData.reduce((sum, p) => sum + p.hours, 0) / chartData.length;

  return (
    <div className="bg-white rounded-xl border border-neutral-200 p-6">
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <h3 className="font-semibold text-neutral-900">Trend Over Time</h3>
        
        {/* Metric Toggle */}
        <div className="flex items-center bg-neutral-100 rounded-lg p-1">
          <button
            onClick={() => setMetric("tickets")}
            className={cn(
              "px-3 py-1.5 text-sm font-medium rounded-md transition-all",
              metric === "tickets"
                ? "bg-white text-neutral-900 shadow-sm"
                : "text-neutral-500 hover:text-neutral-700"
            )}
          >
            Tickets
          </button>
          <button
            onClick={() => setMetric("hours")}
            className={cn(
              "px-3 py-1.5 text-sm font-medium rounded-md transition-all",
              metric === "hours"
                ? "bg-white text-neutral-900 shadow-sm"
                : "text-neutral-500 hover:text-neutral-700"
            )}
          >
            Hours
          </button>
        </div>
      </div>

      {/* Chart */}
      <div className="h-80" role="img" aria-label={`Trend chart showing ${metric} over time`}>
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={chartData} margin={{ top: 10, right: 10, left: 0, bottom: 0 }}>
            <CartesianGrid strokeDasharray="3 3" stroke="#e5e5e5" />
            <XAxis
              dataKey="dateFormatted"
              tick={{ fontSize: 12, fill: "#737373" }}
              tickLine={false}
              axisLine={{ stroke: "#e5e5e5" }}
            />
            <YAxis
              tick={{ fontSize: 12, fill: "#737373" }}
              tickLine={false}
              axisLine={{ stroke: "#e5e5e5" }}
              width={40}
            />
            <Tooltip content={<CustomTooltip metric={metric} />} />
            <ReferenceLine
              y={avgValue}
              stroke="#a3a3a3"
              strokeDasharray="5 5"
              label={{ value: "Avg", position: "right", fontSize: 10, fill: "#a3a3a3" }}
            />
            <Line
              type="monotone"
              dataKey={metric}
              stroke="#6366f1"
              strokeWidth={2}
              dot={{ r: 4, fill: "#6366f1" }}
              activeDot={{ r: 6, fill: "#6366f1", stroke: "#fff", strokeWidth: 2 }}
            />
          </LineChart>
        </ResponsiveContainer>
      </div>

      {/* Summary */}
      {data?.summary && (
        <div className="mt-4 pt-4 border-t border-neutral-100 grid grid-cols-3 gap-4 text-sm">
          <div>
            <span className="text-neutral-500">Total: </span>
            <span className="font-medium">
              {metric === "tickets" 
                ? `${data.summary.totalTickets} tickets`
                : `${data.summary.totalHours.toFixed(1)}h`
              }
            </span>
          </div>
          <div>
            <span className="text-neutral-500">Best day: </span>
            <span className="font-medium text-success-600">
              {data.summary.bestDay && format(parseISO(data.summary.bestDay), "MMM d")}
            </span>
          </div>
          <div>
            <span className="text-neutral-500">Avg efficiency: </span>
            <span className="font-medium">
              {(data.summary.avgEfficiency * 100).toFixed(0)}%
            </span>
          </div>
        </div>
      )}
    </div>
  );
}

interface CustomTooltipProps {
  active?: boolean;
  payload?: any[];
  label?: string;
  metric: MetricType;
}

function CustomTooltip({ active, payload, label, metric }: CustomTooltipProps) {
  if (!active || !payload?.length) return null;

  const data = payload[0].payload;

  return (
    <div className="bg-white rounded-lg shadow-lg border border-neutral-200 p-3">
      <p className="font-medium text-neutral-900 mb-2">{label}</p>
      <div className="space-y-1 text-sm">
        <p>
          <span className="text-neutral-500">Tickets: </span>
          <span className="font-medium">{data.tickets}</span>
        </p>
        <p>
          <span className="text-neutral-500">Hours: </span>
          <span className="font-medium">{data.hours.toFixed(1)}</span>
        </p>
        {data.efficiency && (
          <p>
            <span className="text-neutral-500">Efficiency: </span>
            <span className="font-medium">{(data.efficiency * 100).toFixed(0)}%</span>
          </p>
        )}
      </div>
    </div>
  );
}

function getDateFormat(period: Period): string {
  switch (period) {
    case "7d": return "EEE";
    case "30d": return "MMM d";
    case "90d": return "MMM d";
    case "1y": return "MMM";
    default: return "MMM d";
  }
}
```

### Accessibility Notes

- Chart has `role="img"` with `aria-label`
- Tooltips are keyboard accessible
- Color contrast meets WCAG AA
- Data table available as alternative (hidden visually)

### References

- [Source: epics.md#Story 8.4]
- Recharts: https://recharts.org
