# Story 8.5: Dashboard Period Filters

Status: done

## Story

As a QA (Ana),
I want to filter my dashboard by time period,
So that I can analyze different timeframes.

## Acceptance Criteria

1. **Given** user views dashboard
   **When** user selects a period filter
   **Then** Last 7 days filter works

2. **Given** period filters
   **When** Last 30 days selected
   **Then** dashboard updates with 30 day data

3. **Given** period filters
   **When** Last 90 days selected
   **Then** dashboard updates with 90 day data

4. **Given** period filters
   **When** This year selected
   **Then** dashboard updates with year-to-date data

5. **Given** period filters
   **When** Custom date range selected
   **Then** date picker allows custom selection

6. **Given** filter is selected
   **When** URL is checked
   **Then** filter selection persists in URL (shareable)

7. **Given** period changes
   **When** data loads
   **Then** data refreshes without full page reload

8. **Given** data is loading
   **When** UI updates
   **Then** loading state shows during data fetch

## Tasks

- [ ] Task 1: Create PeriodSelector with all options
- [ ] Task 2: Create DateRangePicker for custom range
- [ ] Task 3: Implement URL-based state persistence
- [ ] Task 4: Add loading states to all dashboard components
- [ ] Task 5: Optimize data fetching with React Query

## Dev Notes

### Enhanced Period Selector

```tsx
// frontend/src/components/dashboard/PeriodSelector.tsx
import { useState } from "react";
import { useSearchParams } from "react-router-dom";
import * as Popover from "@radix-ui/react-popover";
import { CalendarIcon, ChevronDownIcon } from "@radix-ui/react-icons";
import { DayPicker } from "react-day-picker";
import { format, subDays, startOfYear } from "date-fns";

export type Period = "7d" | "30d" | "90d" | "1y" | "custom";

interface DateRange {
  from: Date;
  to: Date;
}

interface PeriodSelectorProps {
  value: Period;
  customRange?: DateRange;
  onChange: (period: Period, customRange?: DateRange) => void;
}

const presetPeriods = [
  { value: "7d", label: "Last 7 days" },
  { value: "30d", label: "Last 30 days" },
  { value: "90d", label: "Last 90 days" },
  { value: "1y", label: "This year" },
] as const;

export function PeriodSelector({ value, customRange, onChange }: PeriodSelectorProps) {
  const [customOpen, setCustomOpen] = useState(false);
  const [tempRange, setTempRange] = useState<DateRange | undefined>(customRange);

  const handlePresetClick = (period: Period) => {
    onChange(period);
  };

  const handleCustomApply = () => {
    if (tempRange?.from && tempRange?.to) {
      onChange("custom", tempRange);
      setCustomOpen(false);
    }
  };

  const getDisplayLabel = () => {
    if (value === "custom" && customRange) {
      return `${format(customRange.from, "MMM d")} - ${format(customRange.to, "MMM d")}`;
    }
    return presetPeriods.find(p => p.value === value)?.label || "Select period";
  };

  return (
    <div className="flex items-center gap-2">
      {/* Preset Buttons */}
      <div className="flex items-center bg-neutral-100 rounded-lg p-1">
        {presetPeriods.map((period) => (
          <button
            key={period.value}
            onClick={() => handlePresetClick(period.value as Period)}
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

      {/* Custom Date Range */}
      <Popover.Root open={customOpen} onOpenChange={setCustomOpen}>
        <Popover.Trigger asChild>
          <button
            className={cn(
              "flex items-center gap-2 px-3 py-2 text-sm font-medium rounded-lg border transition-all",
              value === "custom"
                ? "border-primary-300 bg-primary-50 text-primary-700"
                : "border-neutral-300 text-neutral-600 hover:border-neutral-400"
            )}
          >
            <CalendarIcon className="w-4 h-4" />
            {value === "custom" ? getDisplayLabel() : "Custom"}
            <ChevronDownIcon className="w-4 h-4" />
          </button>
        </Popover.Trigger>

        <Popover.Portal>
          <Popover.Content
            className="bg-white rounded-xl shadow-xl border border-neutral-200 p-4 z-50"
            sideOffset={8}
          >
            <DayPicker
              mode="range"
              selected={tempRange}
              onSelect={(range) => setTempRange(range as DateRange)}
              disabled={{ after: new Date() }}
              numberOfMonths={2}
              className="rdp-custom"
            />

            <div className="flex justify-end gap-2 mt-4 pt-4 border-t border-neutral-200">
              <button
                onClick={() => setCustomOpen(false)}
                className="px-3 py-2 text-sm text-neutral-600 hover:text-neutral-800"
              >
                Cancel
              </button>
              <button
                onClick={handleCustomApply}
                disabled={!tempRange?.from || !tempRange?.to}
                className="px-3 py-2 text-sm bg-primary-500 text-white rounded-lg hover:bg-primary-600 disabled:opacity-50"
              >
                Apply
              </button>
            </div>

            <Popover.Arrow className="fill-white" />
          </Popover.Content>
        </Popover.Portal>
      </Popover.Root>
    </div>
  );
}
```

### URL State Persistence Hook

```tsx
// frontend/src/hooks/usePeriodState.ts
import { useSearchParams } from "react-router-dom";
import { useMemo, useCallback } from "react";
import { parseISO, format } from "date-fns";

interface DateRange {
  from: Date;
  to: Date;
}

export function usePeriodState() {
  const [searchParams, setSearchParams] = useSearchParams();

  const period = useMemo(() => {
    return (searchParams.get("period") || "30d") as Period;
  }, [searchParams]);

  const customRange = useMemo((): DateRange | undefined => {
    const from = searchParams.get("from");
    const to = searchParams.get("to");
    if (from && to) {
      return {
        from: parseISO(from),
        to: parseISO(to),
      };
    }
    return undefined;
  }, [searchParams]);

  const setPeriod = useCallback((newPeriod: Period, range?: DateRange) => {
    const params = new URLSearchParams(searchParams);
    params.set("period", newPeriod);
    
    if (newPeriod === "custom" && range) {
      params.set("from", format(range.from, "yyyy-MM-dd"));
      params.set("to", format(range.to, "yyyy-MM-dd"));
    } else {
      params.delete("from");
      params.delete("to");
    }
    
    setSearchParams(params, { replace: true });
  }, [searchParams, setSearchParams]);

  return { period, customRange, setPeriod };
}
```

### Dashboard with URL State

```tsx
// frontend/src/pages/Dashboard.tsx
export function DashboardPage() {
  const { period, customRange, setPeriod } = usePeriodState();
  
  const { data, isLoading, isFetching, refetch } = useDashboardData({
    period,
    startDate: customRange?.from,
    endDate: customRange?.to,
  });

  // Show subtle loading indicator for refetches
  const showRefetching = isFetching && !isLoading;

  return (
    <div className="p-6 space-y-6">
      {/* Header with refetch indicator */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <h1 className="text-2xl font-bold">Dashboard</h1>
          {showRefetching && (
            <span className="flex items-center gap-1 text-sm text-neutral-500">
              <Spinner className="w-4 h-4" />
              Updating...
            </span>
          )}
        </div>
        
        <PeriodSelector
          value={period}
          customRange={customRange}
          onChange={setPeriod}
        />
      </div>

      {/* Content with loading states */}
      <KPICards data={data?.kpis} isLoading={isLoading} />
      
      <div className={cn(showRefetching && "opacity-70 transition-opacity")}>
        <TrendChart data={data?.trend} period={period} isLoading={isLoading} />
      </div>
    </div>
  );
}
```

### React Query Configuration

```tsx
// frontend/src/hooks/useDashboardData.ts
import { useQuery } from "@tanstack/react-query";

interface UseDashboardOptions {
  period: Period;
  startDate?: Date;
  endDate?: Date;
}

export function useDashboardData(options: UseDashboardOptions) {
  const { period, startDate, endDate } = options;

  return useQuery({
    queryKey: ["dashboard", period, startDate?.toISOString(), endDate?.toISOString()],
    queryFn: async () => {
      const params = new URLSearchParams({ period });
      if (startDate) params.set("startDate", format(startDate, "yyyy-MM-dd"));
      if (endDate) params.set("endDate", format(endDate, "yyyy-MM-dd"));

      const response = await fetch(`/api/v1/dashboard?${params}`);
      if (!response.ok) throw new Error("Failed to fetch dashboard");
      return response.json() as Promise<DashboardResponse>;
    },
    staleTime: 60 * 1000, // 1 minute
    refetchOnWindowFocus: true,
  });
}
```

### References

- [Source: epics.md#Story 8.5]
- react-day-picker: https://react-day-picker.js.org
