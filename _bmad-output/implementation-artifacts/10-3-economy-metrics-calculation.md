# Story 10.3: Economy Metrics Calculation

Status: ready-for-dev

## Story

As a PM (Carlos),
I want to see estimated savings,
So that I can demonstrate ROI.

## Acceptance Criteria

1. **Given** time and bug data exists
   **When** economy card renders
   **Then** hours saved is displayed (time actual < estimated)

2. **Given** economy calculated
   **When** cost shown
   **Then** cost saved is displayed (hours × configurable hourly rate)

3. **Given** economy calculated
   **When** bug value shown
   **Then** bug prevention value is displayed (bugs prevented × avg fix cost)

4. **Given** economy calculated
   **When** total shown
   **Then** total economy estimate is displayed

5. **Given** configuration exists
   **When** rates used
   **Then** hourly rate and fix cost are configurable

6. **Given** calculation details
   **When** user hovers
   **Then** calculation formula is shown on hover

7. **Given** economy data
   **When** export needed
   **Then** can export economy report for stakeholders

## Tasks

- [ ] Task 1: Create economy configuration model
- [ ] Task 2: Implement economy calculation service
- [ ] Task 3: Create EconomyCard component
- [ ] Task 4: Add configuration UI for rates
- [ ] Task 5: Create formula tooltip
- [ ] Task 6: Create economy report export

## Dev Notes

### Economy Configuration

```rust
// crates/qa-pms-analytics/src/economy/config.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EconomyConfig {
    pub hourly_rate: f64,           // e.g., $75/hour
    pub avg_bug_fix_cost: f64,      // e.g., $500 per bug
    pub production_bug_cost: f64,   // e.g., $2000 per production bug
    pub currency: String,           // "USD", "BRL", etc.
}

impl Default for EconomyConfig {
    fn default() -> Self {
        Self {
            hourly_rate: 75.0,
            avg_bug_fix_cost: 500.0,
            production_bug_cost: 2000.0,
            currency: "USD".into(),
        }
    }
}
```

### Economy Metrics

```rust
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EconomyMetrics {
    pub hours_saved: HoursSaved,
    pub cost_saved: CostSaved,
    pub bug_prevention_value: BugPreventionValue,
    pub total_economy: TotalEconomy,
    pub config: EconomyConfig,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HoursSaved {
    pub value: f64,
    pub calculated_from: String, // Formula explanation
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CostSaved {
    pub value: f64,
    pub formula: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BugPreventionValue {
    pub value: f64,
    pub bugs_prevented: i64,
    pub formula: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalEconomy {
    pub value: f64,
    pub change_percent: f64,
    pub breakdown: Vec<EconomyBreakdownItem>,
}
```

### Economy Calculation Service

```rust
// crates/qa-pms-analytics/src/economy/service.rs
impl EconomyService {
    pub async fn calculate_economy(&self, period: &str) -> Result<EconomyMetrics> {
        let config = self.get_config().await?;
        let (start, end, _, _) = Self::calculate_period_ranges(period);

        // Calculate hours saved
        let time_data = self.get_time_aggregates(start, end).await?;
        let hours_saved = if time_data.total_estimated > time_data.total_actual {
            (time_data.total_estimated - time_data.total_actual) as f64 / 3600.0
        } else { 0.0 };

        // Calculate cost saved from hours
        let cost_saved = hours_saved * config.hourly_rate;

        // Calculate bug prevention value
        let bugs_prevented = self.count_prevented_bugs(start, end).await?;
        let bug_value = bugs_prevented as f64 * config.production_bug_cost;

        // Total economy
        let total = cost_saved + bug_value;

        // Previous period for comparison
        let previous_total = self.calculate_previous_period_total(period, &config).await?;
        let change = if previous_total > 0.0 {
            ((total - previous_total) / previous_total) * 100.0
        } else { 0.0 };

        Ok(EconomyMetrics {
            hours_saved: HoursSaved {
                value: hours_saved,
                calculated_from: format!(
                    "Estimated: {:.1}h - Actual: {:.1}h",
                    time_data.total_estimated as f64 / 3600.0,
                    time_data.total_actual as f64 / 3600.0
                ),
            },
            cost_saved: CostSaved {
                value: cost_saved,
                formula: format!("{:.1}h × ${:.0}/h", hours_saved, config.hourly_rate),
            },
            bug_prevention_value: BugPreventionValue {
                value: bug_value,
                bugs_prevented,
                formula: format!(
                    "{} bugs × ${:.0} (avg production bug cost)",
                    bugs_prevented, config.production_bug_cost
                ),
            },
            total_economy: TotalEconomy {
                value: total,
                change_percent: change.round(),
                breakdown: vec![
                    EconomyBreakdownItem {
                        label: "Time Efficiency".into(),
                        value: cost_saved,
                        percent: if total > 0.0 { (cost_saved / total) * 100.0 } else { 0.0 },
                    },
                    EconomyBreakdownItem {
                        label: "Bug Prevention".into(),
                        value: bug_value,
                        percent: if total > 0.0 { (bug_value / total) * 100.0 } else { 0.0 },
                    },
                ],
            },
            config,
        })
    }
}
```

### Economy Card Component

```tsx
// frontend/src/components/pm-dashboard/EconomyCard.tsx
import * as Tooltip from "@radix-ui/react-tooltip";

interface EconomyCardProps {
  data?: EconomyMetrics;
  isLoading: boolean;
  onConfigClick: () => void;
}

export function EconomyCard({ data, isLoading, onConfigClick }: EconomyCardProps) {
  if (isLoading) {
    return <div className="h-80 bg-neutral-100 rounded-xl animate-pulse" />;
  }

  const formatCurrency = (value: number) => {
    return new Intl.NumberFormat("en-US", {
      style: "currency",
      currency: data?.config.currency || "USD",
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(value);
  };

  return (
    <div className="bg-gradient-to-br from-success-50 to-success-100 rounded-xl border-2 border-success-200 p-6">
      <div className="flex items-center justify-between mb-6">
        <h3 className="font-semibold text-success-900">Estimated Savings</h3>
        <button
          onClick={onConfigClick}
          className="text-sm text-success-600 hover:text-success-700 flex items-center gap-1"
        >
          <GearIcon className="w-4 h-4" />
          Configure
        </button>
      </div>

      {/* Total */}
      <div className="text-center mb-6">
        <p className="text-4xl font-bold text-success-700">
          {formatCurrency(data?.totalEconomy.value || 0)}
        </p>
        <p className="text-sm text-success-600 mt-1">Total Economy</p>
        <TrendIndicator value={data?.totalEconomy.changePercent || 0} className="mt-2" />
      </div>

      {/* Breakdown */}
      <div className="space-y-4">
        {/* Hours Saved */}
        <Tooltip.Provider>
          <Tooltip.Root>
            <Tooltip.Trigger asChild>
              <div className="flex items-center justify-between p-3 bg-white/50 rounded-lg cursor-help">
                <div className="flex items-center gap-2">
                  <ClockIcon className="w-5 h-5 text-success-600" />
                  <span className="text-success-800">Hours Saved</span>
                </div>
                <span className="font-semibold text-success-700">
                  {data?.hoursSaved.value.toFixed(1)}h
                </span>
              </div>
            </Tooltip.Trigger>
            <Tooltip.Content className="bg-neutral-900 text-white text-xs rounded p-2 max-w-xs">
              {data?.hoursSaved.calculatedFrom}
              <Tooltip.Arrow className="fill-neutral-900" />
            </Tooltip.Content>
          </Tooltip.Root>
        </Tooltip.Provider>

        {/* Cost Saved */}
        <Tooltip.Provider>
          <Tooltip.Root>
            <Tooltip.Trigger asChild>
              <div className="flex items-center justify-between p-3 bg-white/50 rounded-lg cursor-help">
                <div className="flex items-center gap-2">
                  <CurrencyDollarIcon className="w-5 h-5 text-success-600" />
                  <span className="text-success-800">Time Efficiency</span>
                </div>
                <span className="font-semibold text-success-700">
                  {formatCurrency(data?.costSaved.value || 0)}
                </span>
              </div>
            </Tooltip.Trigger>
            <Tooltip.Content className="bg-neutral-900 text-white text-xs rounded p-2 max-w-xs">
              Formula: {data?.costSaved.formula}
              <Tooltip.Arrow className="fill-neutral-900" />
            </Tooltip.Content>
          </Tooltip.Root>
        </Tooltip.Provider>

        {/* Bug Prevention */}
        <Tooltip.Provider>
          <Tooltip.Root>
            <Tooltip.Trigger asChild>
              <div className="flex items-center justify-between p-3 bg-white/50 rounded-lg cursor-help">
                <div className="flex items-center gap-2">
                  <ShieldCheckIcon className="w-5 h-5 text-success-600" />
                  <span className="text-success-800">Bug Prevention</span>
                </div>
                <span className="font-semibold text-success-700">
                  {formatCurrency(data?.bugPreventionValue.value || 0)}
                </span>
              </div>
            </Tooltip.Trigger>
            <Tooltip.Content className="bg-neutral-900 text-white text-xs rounded p-2 max-w-xs">
              {data?.bugPreventionValue.bugs_prevented} bugs prevented
              <br />
              Formula: {data?.bugPreventionValue.formula}
              <Tooltip.Arrow className="fill-neutral-900" />
            </Tooltip.Content>
          </Tooltip.Root>
        </Tooltip.Provider>
      </div>
    </div>
  );
}
```

### Economy Configuration Dialog

```tsx
// frontend/src/components/pm-dashboard/EconomyConfigDialog.tsx
export function EconomyConfigDialog({ config, open, onOpenChange, onSave }: EconomyConfigDialogProps) {
  const [values, setValues] = useState(config);

  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Content className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-white rounded-xl p-6 w-full max-w-md">
        <Dialog.Title className="text-xl font-semibold mb-6">Economy Settings</Dialog.Title>

        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-neutral-700 mb-1">
              Hourly Rate
            </label>
            <div className="relative">
              <span className="absolute left-3 top-1/2 -translate-y-1/2 text-neutral-500">$</span>
              <input
                type="number"
                value={values.hourlyRate}
                onChange={(e) => setValues({ ...values, hourlyRate: parseFloat(e.target.value) })}
                className="w-full pl-8 pr-4 py-2 border border-neutral-300 rounded-lg"
              />
            </div>
            <p className="text-xs text-neutral-500 mt-1">Average cost per hour of QA time</p>
          </div>

          <div>
            <label className="block text-sm font-medium text-neutral-700 mb-1">
              Production Bug Cost
            </label>
            <div className="relative">
              <span className="absolute left-3 top-1/2 -translate-y-1/2 text-neutral-500">$</span>
              <input
                type="number"
                value={values.productionBugCost}
                onChange={(e) => setValues({ ...values, productionBugCost: parseFloat(e.target.value) })}
                className="w-full pl-8 pr-4 py-2 border border-neutral-300 rounded-lg"
              />
            </div>
            <p className="text-xs text-neutral-500 mt-1">Average cost to fix a bug in production</p>
          </div>
        </div>

        <div className="flex justify-end gap-2 mt-6">
          <Dialog.Close asChild>
            <button className="px-4 py-2 text-neutral-600">Cancel</button>
          </Dialog.Close>
          <button
            onClick={() => onSave(values)}
            className="px-4 py-2 bg-primary-500 text-white rounded-lg"
          >
            Save
          </button>
        </div>
      </Dialog.Content>
    </Dialog.Root>
  );
}
```

### References

- [Source: epics.md#Story 10.3]
