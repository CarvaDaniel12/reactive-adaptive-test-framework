# Story 6.6: Time Comparison with Gap Alerts

Status: ready-for-dev

## Story

As a QA (Ana),
I want to see my time compared to estimates,
So that I can identify where I'm spending extra time.

## Acceptance Criteria

1. **Given** workflow has time estimates per step
   **When** user completes a step
   **Then** comparison is displayed: Actual: "45 min" | Estimated: "30 min"

2. **Given** step is completed
   **When** comparison is shown
   **Then** gap indicator uses color coding: 🟢 (≤100%), 🟡 (100-120%), 🔴 (>120%)

3. **Given** step took >20% over estimate
   **When** alert is generated
   **Then** alert shows: "This step took 50% longer than estimated"

4. **Given** workflow completes
   **When** summary is displayed
   **Then** workflow summary shows total actual vs estimated

5. **Given** gap alert is shown
   **When** user interacts
   **Then** gap alerts are dismissible but logged

6. **Given** time data exists
   **When** used for future estimates
   **Then** historical data updates user's personal estimates

## Tasks

- [ ] Task 1: Create TimeComparison component
- [ ] Task 2: Implement gap calculation logic
- [ ] Task 3: Create GapIndicator component with colors
- [ ] Task 4: Add gap alert toast notifications
- [ ] Task 5: Store gap data in database
- [ ] Task 6: Create EstimateAdjustmentService for learning

## Dev Notes

### Gap Calculation

```rust
// crates/qa-pms-workflow/src/time/gap.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GapLevel {
    OnTarget,   // ≤100%
    Warning,    // 100-120%
    Critical,   // >120%
}

impl GapLevel {
    pub fn from_ratio(ratio: f64) -> Self {
        if ratio <= 1.0 {
            GapLevel::OnTarget
        } else if ratio <= 1.2 {
            GapLevel::Warning
        } else {
            GapLevel::Critical
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeGap {
    pub actual_seconds: i32,
    pub estimated_seconds: i32,
    pub gap_seconds: i32,
    pub gap_percentage: f64,
    pub level: String, // "on_target", "warning", "critical"
    pub alert_message: Option<String>,
}

impl TimeGap {
    pub fn calculate(actual: i32, estimated: i32) -> Self {
        let ratio = actual as f64 / estimated as f64;
        let gap = actual - estimated;
        let percentage = ((ratio - 1.0) * 100.0).round();
        let level = GapLevel::from_ratio(ratio);
        
        let alert_message = if percentage > 20.0 {
            Some(format!(
                "This step took {:.0}% longer than estimated",
                percentage
            ))
        } else {
            None
        };

        Self {
            actual_seconds: actual,
            estimated_seconds: estimated,
            gap_seconds: gap,
            gap_percentage: percentage,
            level: match level {
                GapLevel::OnTarget => "on_target",
                GapLevel::Warning => "warning",
                GapLevel::Critical => "critical",
            }.to_string(),
            alert_message,
        }
    }
}
```

### Frontend - TimeComparison Component

```tsx
// frontend/src/components/workflow/TimeComparison.tsx
interface TimeComparisonProps {
  actualSeconds: number;
  estimatedSeconds: number;
  showAlert?: boolean;
}

export function TimeComparison({
  actualSeconds,
  estimatedSeconds,
  showAlert = true,
}: TimeComparisonProps) {
  const actualMinutes = Math.round(actualSeconds / 60);
  const estimatedMinutes = Math.round(estimatedSeconds / 60);
  const ratio = actualSeconds / estimatedSeconds;
  const gapPercent = Math.round((ratio - 1) * 100);

  const getLevel = (): "success" | "warning" | "error" => {
    if (ratio <= 1.0) return "success";
    if (ratio <= 1.2) return "warning";
    return "error";
  };

  const level = getLevel();

  return (
    <div className="flex items-center gap-4">
      {/* Actual vs Estimated */}
      <div className="flex items-center gap-2 text-sm">
        <span className="text-neutral-500">Actual:</span>
        <span className={cn(
          "font-medium",
          level === "success" && "text-success-600",
          level === "warning" && "text-warning-600",
          level === "error" && "text-error-600",
        )}>
          {actualMinutes} min
        </span>
        <span className="text-neutral-300">|</span>
        <span className="text-neutral-500">Estimated:</span>
        <span className="text-neutral-700">{estimatedMinutes} min</span>
      </div>

      {/* Gap Indicator */}
      <GapIndicator level={level} percent={gapPercent} />
    </div>
  );
}

interface GapIndicatorProps {
  level: "success" | "warning" | "error";
  percent: number;
}

function GapIndicator({ level, percent }: GapIndicatorProps) {
  const icons = {
    success: "🟢",
    warning: "🟡",
    error: "🔴",
  };

  const labels = {
    success: percent < 0 ? `${Math.abs(percent)}% under` : "On target",
    warning: `${percent}% over`,
    error: `${percent}% over`,
  };

  return (
    <div className={cn(
      "flex items-center gap-1 px-2 py-1 rounded text-xs font-medium",
      level === "success" && "bg-success-50 text-success-700",
      level === "warning" && "bg-warning-50 text-warning-700",
      level === "error" && "bg-error-50 text-error-700",
    )}>
      <span>{icons[level]}</span>
      <span>{labels[level]}</span>
    </div>
  );
}
```

### Gap Alert Toast

```tsx
// Show when step completes with significant gap
if (gap.gapPercentage > 20) {
  toast.warning(gap.alertMessage, {
    duration: 5000,
    action: {
      label: "Dismiss",
      onClick: () => {
        // Log dismissal
        logGapAlertDismissed(stepIndex, gap);
      },
    },
  });
}
```

### Workflow Summary Integration

```tsx
// In WorkflowSummary.tsx
<div className="grid grid-cols-2 gap-4">
  <div className="bg-neutral-50 rounded-lg p-4">
    <p className="text-sm text-neutral-500">Total Actual</p>
    <p className="text-2xl font-bold">{formatTime(totalActual)}</p>
  </div>
  <div className="bg-neutral-50 rounded-lg p-4">
    <p className="text-sm text-neutral-500">Total Estimated</p>
    <p className="text-2xl font-bold">{formatTime(totalEstimated)}</p>
  </div>
</div>

<TimeComparison
  actualSeconds={totalActual}
  estimatedSeconds={totalEstimated}
/>
```

### Historical Estimate Updates

```rust
// crates/qa-pms-workflow/src/time/estimates.rs
pub struct EstimateAdjustmentService {
    repo: TimeRepository,
}

impl EstimateAdjustmentService {
    /// Update personal estimates based on actual time
    /// Uses exponential moving average
    pub async fn update_estimate(
        &self,
        user_id: &str,
        template_id: Uuid,
        step_index: i32,
        actual_seconds: i32,
    ) -> Result<()> {
        const ALPHA: f64 = 0.3; // Weight for new data
        
        let current = self.repo
            .get_user_estimate(user_id, template_id, step_index)
            .await?;
        
        let new_estimate = match current {
            Some(est) => {
                let new = (ALPHA * actual_seconds as f64 + 
                          (1.0 - ALPHA) * est.estimated_seconds as f64) as i32;
                new
            }
            None => actual_seconds,
        };
        
        self.repo
            .upsert_user_estimate(user_id, template_id, step_index, new_estimate)
            .await
    }
}
```

### References

- [Source: epics.md#Story 6.6]
- [Dependency: Story 6.5 - Time Per Step Tracking]
