# Story 10.2: Bugs Discovered vs Prevented Metrics

Status: ready-for-dev

## Story

As a PM (Carlos),
I want to see bugs discovered vs prevented,
So that I can quantify QA value.

## Acceptance Criteria

1. **Given** bug data is tracked (from workflows/reports)
   **When** metrics card renders
   **Then** bugs discovered count is displayed (from testing)

2. **Given** metrics displayed
   **When** prevention shown
   **Then** bugs prevented count is displayed (from proactive detection)

3. **Given** metrics displayed
   **When** rate calculated
   **Then** prevention rate is shown: prevented / (discovered + prevented)

4. **Given** metrics displayed
   **When** trend shown
   **Then** trend vs previous period is visible

5. **Given** metrics are displayed
   **When** user hovers
   **Then** definitions are shown on hover

6. **Given** card is interactive
   **When** clicked
   **Then** clicking shows detailed breakdown

## Tasks

- [ ] Task 1: Define bug tracking data model
- [ ] Task 2: Create bug metrics calculation service
- [ ] Task 3: Create BugsMetricsCard component
- [ ] Task 4: Add definitions tooltips
- [ ] Task 5: Create detailed breakdown view
- [ ] Task 6: Calculate trend vs previous period

## Dev Notes

### Bug Tracking Data Model

```rust
// Bugs are tracked from:
// 1. Links in workflow step notes (type = "bug")
// 2. Alerts that resulted in bug discovery
// 3. Manual tagging in reports

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BugMetrics {
    pub discovered: BugCount,
    pub prevented: BugCount,
    pub prevention_rate: f64,
    pub trend: TrendInfo,
    pub by_severity: Vec<SeverityBreakdown>,
    pub by_component: Vec<ComponentBreakdown>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BugCount {
    pub value: i64,
    pub previous_period: i64,
    pub change_percent: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeverityBreakdown {
    pub severity: String,
    pub discovered: i64,
    pub prevented: i64,
}
```

### Bug Metrics Service

```rust
// crates/qa-pms-analytics/src/bug_metrics.rs
impl BugMetricsService {
    pub async fn get_bug_metrics(&self, period: &str) -> Result<BugMetrics> {
        let (start_current, end_current, start_previous, end_previous) = 
            Self::calculate_period_ranges(period);

        // Count discovered bugs (links with type = "bug" in step results)
        let discovered_current = self.count_discovered_bugs(start_current, end_current).await?;
        let discovered_previous = self.count_discovered_bugs(start_previous, end_previous).await?;

        // Count prevented bugs (from proactive alerts that were acted upon)
        let prevented_current = self.count_prevented_bugs(start_current, end_current).await?;
        let prevented_previous = self.count_prevented_bugs(start_previous, end_previous).await?;

        // Calculate prevention rate
        let total = discovered_current + prevented_current;
        let prevention_rate = if total > 0 {
            prevented_current as f64 / total as f64
        } else { 0.0 };

        // Get breakdowns
        let by_severity = self.get_severity_breakdown(start_current, end_current).await?;
        let by_component = self.get_component_breakdown(start_current, end_current).await?;

        Ok(BugMetrics {
            discovered: BugCount {
                value: discovered_current,
                previous_period: discovered_previous,
                change_percent: Self::calc_change(discovered_current, discovered_previous),
            },
            prevented: BugCount {
                value: prevented_current,
                previous_period: prevented_previous,
                change_percent: Self::calc_change(prevented_current, prevented_previous),
            },
            prevention_rate,
            trend: self.calculate_trend(discovered_current, discovered_previous, 
                                        prevented_current, prevented_previous),
            by_severity,
            by_component,
        })
    }

    async fn count_discovered_bugs(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<i64> {
        sqlx::query_scalar(
            r#"
            SELECT COUNT(DISTINCT link->>'url')
            FROM workflow_step_results wsr
            CROSS JOIN LATERAL jsonb_array_elements(wsr.links) as link
            JOIN workflow_instances wi ON wsr.instance_id = wi.id
            WHERE link->>'linkType' = 'bug'
              AND wi.completed_at BETWEEN $1 AND $2
            "#,
        )
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }

    async fn count_prevented_bugs(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<i64> {
        // Prevented = alerts of type "consecutive_issues" that were resolved
        // This is a proxy for bugs caught before they reached production
        sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM detected_patterns dp
            WHERE dp.pattern_type = 'consecutive_issues'
              AND dp.resolved = true
              AND dp.created_at BETWEEN $1 AND $2
            "#,
        )
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }
}
```

### Bugs Metrics Card Component

```tsx
// frontend/src/components/pm-dashboard/BugsMetricsCard.tsx
import * as Tooltip from "@radix-ui/react-tooltip";

interface BugsMetricsCardProps {
  data?: BugMetrics;
  isLoading: boolean;
  onClick: () => void;
}

export function BugsMetricsCard({ data, isLoading, onClick }: BugsMetricsCardProps) {
  if (isLoading) {
    return <div className="h-64 bg-neutral-100 rounded-xl animate-pulse" />;
  }

  const total = (data?.discovered.value || 0) + (data?.prevented.value || 0);
  const discoveredPercent = total > 0 
    ? ((data?.discovered.value || 0) / total) * 100 
    : 50;

  return (
    <button
      onClick={onClick}
      className="w-full text-left bg-white rounded-xl border border-neutral-200 p-6 
                 hover:shadow-md hover:border-primary-200 transition-all"
    >
      <div className="flex items-center justify-between mb-6">
        <h3 className="font-semibold text-neutral-900">Bug Metrics</h3>
        <Tooltip.Provider>
          <Tooltip.Root>
            <Tooltip.Trigger asChild>
              <button className="p-1 text-neutral-400 hover:text-neutral-600">
                <InfoCircledIcon className="w-4 h-4" />
              </button>
            </Tooltip.Trigger>
            <Tooltip.Content className="bg-neutral-900 text-white text-xs rounded p-2 max-w-xs">
              <p><strong>Discovered:</strong> Bugs found during QA testing</p>
              <p><strong>Prevented:</strong> Issues caught by proactive alerts before production</p>
              <Tooltip.Arrow className="fill-neutral-900" />
            </Tooltip.Content>
          </Tooltip.Root>
        </Tooltip.Provider>
      </div>

      {/* Visual Bar */}
      <div className="mb-6">
        <div className="flex h-8 rounded-lg overflow-hidden">
          <div
            className="bg-error-500 flex items-center justify-center text-white text-sm font-medium"
            style={{ width: `${discoveredPercent}%` }}
          >
            {data?.discovered.value || 0}
          </div>
          <div
            className="bg-success-500 flex items-center justify-center text-white text-sm font-medium"
            style={{ width: `${100 - discoveredPercent}%` }}
          >
            {data?.prevented.value || 0}
          </div>
        </div>
        <div className="flex justify-between mt-2 text-xs text-neutral-500">
          <span className="flex items-center gap-1">
            <span className="w-2 h-2 rounded-full bg-error-500" />
            Discovered
          </span>
          <span className="flex items-center gap-1">
            <span className="w-2 h-2 rounded-full bg-success-500" />
            Prevented
          </span>
        </div>
      </div>

      {/* Prevention Rate */}
      <div className="text-center p-4 bg-success-50 rounded-lg">
        <p className="text-3xl font-bold text-success-600">
          {((data?.preventionRate || 0) * 100).toFixed(0)}%
        </p>
        <p className="text-sm text-success-700">Prevention Rate</p>
      </div>

      {/* Trend */}
      <div className="flex justify-center mt-4">
        <TrendIndicator 
          value={data?.trend.overallChange || 0} 
          label="vs previous period"
        />
      </div>
    </button>
  );
}
```

### Detailed Breakdown Dialog

```tsx
// frontend/src/components/pm-dashboard/BugsBreakdownDialog.tsx
export function BugsBreakdownDialog({ data, open, onOpenChange }: BugsBreakdownDialogProps) {
  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Portal>
        <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 
                                   bg-white rounded-xl p-6 w-full max-w-2xl max-h-[80vh] overflow-y-auto">
          <Dialog.Title className="text-xl font-semibold mb-6">Bug Metrics Breakdown</Dialog.Title>

          {/* By Severity */}
          <section className="mb-8">
            <h4 className="font-medium text-neutral-700 mb-4">By Severity</h4>
            <div className="space-y-3">
              {data?.bySeverity.map((item) => (
                <div key={item.severity} className="flex items-center gap-4">
                  <SeverityBadge severity={item.severity} className="w-20" />
                  <div className="flex-1">
                    <div className="flex h-6 rounded overflow-hidden">
                      <div
                        className="bg-error-400"
                        style={{ width: `${item.discovered / (item.discovered + item.prevented) * 100}%` }}
                      />
                      <div
                        className="bg-success-400"
                        style={{ width: `${item.prevented / (item.discovered + item.prevented) * 100}%` }}
                      />
                    </div>
                  </div>
                  <div className="text-sm text-neutral-500 w-24 text-right">
                    {item.discovered} / {item.prevented}
                  </div>
                </div>
              ))}
            </div>
          </section>

          {/* By Component */}
          <section>
            <h4 className="font-medium text-neutral-700 mb-4">By Component</h4>
            <div className="space-y-2">
              {data?.byComponent.map((item) => (
                <div key={item.component} className="flex items-center justify-between p-3 bg-neutral-50 rounded-lg">
                  <span className="font-medium">{item.component}</span>
                  <div className="flex items-center gap-4 text-sm">
                    <span className="text-error-600">{item.discovered} discovered</span>
                    <span className="text-success-600">{item.prevented} prevented</span>
                  </div>
                </div>
              ))}
            </div>
          </section>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
```

### References

- [Source: epics.md#Story 10.2]
