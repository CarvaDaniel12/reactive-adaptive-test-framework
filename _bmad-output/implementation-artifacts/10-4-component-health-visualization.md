# Story 10.4: Component Health Visualization

Status: ready-for-dev

## Story

As a PM (Carlos),
I want to see which components are healthy vs degraded,
So that I can prioritize improvements.

## Acceptance Criteria

1. **Given** tickets and bugs are tagged by component
   **When** component health section renders
   **Then** list of components sorted by bug count is displayed

2. **Given** component is displayed
   **When** health shown
   **Then** health indicator per component is visible (🟢🟡🔴)

3. **Given** component is displayed
   **When** trend shown
   **Then** trend arrow is visible (improving/degrading/stable)

4. **Given** component is displayed
   **When** counts shown
   **Then** bug count and ticket count are displayed

5. **Given** component is interactive
   **When** clicked
   **Then** clicking component shows detailed history

6. **Given** component data
   **When** filter applied
   **Then** can filter by time period

7. **Given** no recent bugs
   **When** health calculated
   **Then** components with no recent bugs show 🟢

## Tasks

- [ ] Task 1: Create component extraction logic
- [ ] Task 2: Implement health calculation algorithm
- [ ] Task 3: Create ComponentHealthSection component
- [ ] Task 4: Create ComponentHealthCard component
- [ ] Task 5: Create ComponentDetailDialog
- [ ] Task 6: Implement trend calculation

## Dev Notes

### Component Health Service

```rust
// crates/qa-pms-analytics/src/component_health.rs
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentHealth {
    pub component: String,
    pub health: HealthLevel,
    pub trend: TrendDirection,
    pub bug_count: i64,
    pub ticket_count: i64,
    pub severity_breakdown: SeverityBreakdown,
    pub last_bug_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthLevel {
    Healthy,  // 🟢 0-2 bugs in period
    Warning,  // 🟡 3-5 bugs in period
    Critical, // 🔴 >5 bugs in period
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TrendDirection {
    Improving,  // ↓ Less bugs than previous period
    Degrading,  // ↑ More bugs than previous period
    Stable,     // → Same as previous period
}

impl ComponentHealthService {
    pub async fn get_component_health(&self, period: &str) -> Result<Vec<ComponentHealth>> {
        let (start_current, end_current, start_previous, end_previous) = 
            Self::calculate_period_ranges(period);

        // Get current period data
        let current_data = self.get_component_data(start_current, end_current).await?;
        
        // Get previous period for trend
        let previous_data = self.get_component_data(start_previous, end_previous).await?;

        // Calculate health and trends
        let mut components: Vec<ComponentHealth> = current_data
            .into_iter()
            .map(|(component, bug_count, ticket_count, last_bug)| {
                let previous_bugs = previous_data
                    .iter()
                    .find(|(c, _, _, _)| c == &component)
                    .map(|(_, b, _, _)| *b)
                    .unwrap_or(0);

                let health = Self::calculate_health(bug_count);
                let trend = Self::calculate_trend(bug_count, previous_bugs);

                ComponentHealth {
                    component,
                    health,
                    trend,
                    bug_count,
                    ticket_count,
                    severity_breakdown: SeverityBreakdown::default(),
                    last_bug_date: last_bug,
                }
            })
            .collect();

        // Sort by bug count descending
        components.sort_by(|a, b| b.bug_count.cmp(&a.bug_count));

        Ok(components)
    }

    async fn get_component_data(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<(String, i64, i64, Option<DateTime<Utc>>)>> {
        sqlx::query_as(
            r#"
            WITH bug_links AS (
                SELECT 
                    COALESCE(
                        wi.ticket_title SIMILAR TO '%\[(.*?)\]%',
                        'Unknown'
                    ) as component,
                    wi.id as instance_id,
                    MAX(wi.completed_at) as last_date
                FROM workflow_instances wi
                JOIN workflow_step_results wsr ON wsr.instance_id = wi.id
                WHERE EXISTS (
                    SELECT 1 FROM jsonb_array_elements(wsr.links) as link
                    WHERE link->>'linkType' = 'bug'
                )
                AND wi.completed_at BETWEEN $1 AND $2
                GROUP BY component, wi.id
            )
            SELECT 
                component,
                COUNT(DISTINCT instance_id) as bug_count,
                COUNT(*) as ticket_count,
                MAX(last_date) as last_bug_date
            FROM bug_links
            GROUP BY component
            ORDER BY bug_count DESC
            "#,
        )
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await
        .map_err(Into::into)
    }

    fn calculate_health(bug_count: i64) -> HealthLevel {
        match bug_count {
            0..=2 => HealthLevel::Healthy,
            3..=5 => HealthLevel::Warning,
            _ => HealthLevel::Critical,
        }
    }

    fn calculate_trend(current: i64, previous: i64) -> TrendDirection {
        let diff = current - previous;
        if diff < 0 { TrendDirection::Improving }
        else if diff > 0 { TrendDirection::Degrading }
        else { TrendDirection::Stable }
    }
}
```

### Component Health Section

```tsx
// frontend/src/components/pm-dashboard/ComponentHealthSection.tsx
interface ComponentHealthSectionProps {
  data?: ComponentHealth[];
  isLoading: boolean;
}

export function ComponentHealthSection({ data, isLoading }: ComponentHealthSectionProps) {
  const [selectedComponent, setSelectedComponent] = useState<string | null>(null);

  if (isLoading) {
    return (
      <div className="bg-white rounded-xl border border-neutral-200 p-6">
        <div className="h-64 bg-neutral-100 animate-pulse rounded-lg" />
      </div>
    );
  }

  return (
    <div className="bg-white rounded-xl border border-neutral-200">
      <div className="p-4 border-b border-neutral-200">
        <h3 className="font-semibold text-neutral-900">Component Health</h3>
        <p className="text-sm text-neutral-500">Sorted by bug frequency</p>
      </div>

      <div className="divide-y divide-neutral-100 max-h-96 overflow-y-auto">
        {data?.length === 0 ? (
          <div className="p-8 text-center text-neutral-500">
            No component data available
          </div>
        ) : (
          data?.map((component) => (
            <ComponentHealthCard
              key={component.component}
              data={component}
              onClick={() => setSelectedComponent(component.component)}
            />
          ))
        )}
      </div>

      {/* Detail Dialog */}
      {selectedComponent && (
        <ComponentDetailDialog
          component={selectedComponent}
          open={!!selectedComponent}
          onOpenChange={() => setSelectedComponent(null)}
        />
      )}
    </div>
  );
}
```

### Component Health Card

```tsx
// frontend/src/components/pm-dashboard/ComponentHealthCard.tsx
interface ComponentHealthCardProps {
  data: ComponentHealth;
  onClick: () => void;
}

export function ComponentHealthCard({ data, onClick }: ComponentHealthCardProps) {
  const healthConfig = {
    healthy: { icon: "🟢", color: "text-success-600", bg: "bg-success-50" },
    warning: { icon: "🟡", color: "text-warning-600", bg: "bg-warning-50" },
    critical: { icon: "🔴", color: "text-error-600", bg: "bg-error-50" },
  };

  const trendConfig = {
    improving: { icon: "↓", color: "text-success-500", label: "Improving" },
    degrading: { icon: "↑", color: "text-error-500", label: "Degrading" },
    stable: { icon: "→", color: "text-neutral-500", label: "Stable" },
  };

  const health = healthConfig[data.health];
  const trend = trendConfig[data.trend];

  return (
    <button
      onClick={onClick}
      className="w-full text-left p-4 hover:bg-neutral-50 transition-colors"
    >
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <span className={cn("w-8 h-8 rounded-lg flex items-center justify-center", health.bg)}>
            {health.icon}
          </span>
          <div>
            <p className="font-medium text-neutral-900">{data.component}</p>
            <p className="text-sm text-neutral-500">
              {data.bugCount} bugs • {data.ticketCount} tickets
            </p>
          </div>
        </div>

        <div className="flex items-center gap-3">
          <span className={cn("flex items-center gap-1 text-sm", trend.color)}>
            <span>{trend.icon}</span>
            <span>{trend.label}</span>
          </span>
          <ChevronRightIcon className="w-5 h-5 text-neutral-400" />
        </div>
      </div>

      {/* Mini timeline of last 7 days */}
      <div className="mt-3 flex items-center gap-1">
        {[...Array(7)].map((_, i) => {
          const hasIssue = Math.random() > 0.7; // Would come from real data
          return (
            <div
              key={i}
              className={cn(
                "flex-1 h-1.5 rounded-full",
                hasIssue ? "bg-error-300" : "bg-neutral-200"
              )}
            />
          );
        })}
      </div>
    </button>
  );
}
```

### Component Detail Dialog

```tsx
// frontend/src/components/pm-dashboard/ComponentDetailDialog.tsx
export function ComponentDetailDialog({ component, open, onOpenChange }: ComponentDetailDialogProps) {
  const { data, isLoading } = useComponentHistory(component);

  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 
                                 bg-white rounded-xl p-6 w-full max-w-2xl max-h-[80vh] overflow-y-auto">
        <Dialog.Title className="text-xl font-semibold mb-2">{component}</Dialog.Title>
        <Dialog.Description className="text-neutral-500 mb-6">
          Component health history
        </Dialog.Description>

        {/* Timeline Chart */}
        <div className="h-48 mb-6">
          <ComponentTimelineChart data={data?.timeline} />
        </div>

        {/* Related Tickets */}
        <section>
          <h4 className="font-medium text-neutral-700 mb-3">Related Tickets</h4>
          <div className="space-y-2">
            {data?.tickets.map((ticket) => (
              <Link
                key={ticket.key}
                to={`/tickets/${ticket.key}`}
                className="flex items-center justify-between p-3 bg-neutral-50 rounded-lg hover:bg-neutral-100"
              >
                <div>
                  <span className="font-mono text-sm">{ticket.key}</span>
                  <span className="text-neutral-500 ml-2">{ticket.title}</span>
                </div>
                <span className="text-xs text-neutral-400">
                  {formatDistanceToNow(new Date(ticket.date), { addSuffix: true })}
                </span>
              </Link>
            ))}
          </div>
        </section>
      </Dialog.Content>
    </Dialog.Root>
  );
}
```

### References

- [Source: epics.md#Story 10.4]
