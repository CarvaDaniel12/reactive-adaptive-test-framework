# Story 8.6: Dashboard Real-Time Refresh

Status: done

## Story

As a QA (Ana),
I want my dashboard to update in real-time,
So that I see current data.

## Acceptance Criteria

1. **Given** user is viewing dashboard
   **When** new data is available (workflow completed)
   **Then** KPI cards refresh

2. **Given** new data available
   **When** refresh occurs
   **Then** chart updates

3. **Given** new data available
   **When** refresh occurs
   **Then** activity list updates

4. **Given** refresh in progress
   **When** UI shows status
   **Then** refresh indicator shows when updating

5. **Given** user wants manual refresh
   **When** button available
   **Then** manual refresh button available

6. **Given** auto-refresh configuration
   **When** time passes
   **Then** auto-refresh interval is 60 seconds

7. **Given** refresh occurs
   **When** UI updates
   **Then** refresh is silent (no page flicker)

## Tasks

- [ ] Task 1: Configure React Query for auto-refresh
- [ ] Task 2: Create refresh indicator component
- [ ] Task 3: Add manual refresh button
- [ ] Task 4: Implement optimistic updates
- [ ] Task 5: Add activity stream with real-time updates
- [ ] Task 6: Smooth animation for data changes

## Dev Notes

### React Query Auto-Refresh Configuration

```tsx
// frontend/src/hooks/useDashboardData.ts
import { useQuery, useQueryClient } from "@tanstack/react-query";

const REFRESH_INTERVAL = 60 * 1000; // 60 seconds

export function useDashboardData(options: UseDashboardOptions) {
  const queryClient = useQueryClient();

  const query = useQuery({
    queryKey: ["dashboard", options.period, options.startDate, options.endDate],
    queryFn: fetchDashboardData,
    staleTime: 30 * 1000, // Consider stale after 30s
    refetchInterval: REFRESH_INTERVAL, // Auto-refresh every 60s
    refetchIntervalInBackground: false, // Only when tab is visible
    refetchOnWindowFocus: true,
    // Smooth updates - don't show loading state on refetch
    placeholderData: (previousData) => previousData,
  });

  // Manual refresh function
  const refresh = () => {
    queryClient.invalidateQueries({ queryKey: ["dashboard"] });
  };

  return { ...query, refresh };
}
```

### Refresh Indicator Component

```tsx
// frontend/src/components/dashboard/RefreshIndicator.tsx
import { useState, useEffect } from "react";
import { ReloadIcon } from "@radix-ui/react-icons";

interface RefreshIndicatorProps {
  lastUpdated: Date | null;
  isFetching: boolean;
  onRefresh: () => void;
}

export function RefreshIndicator({
  lastUpdated,
  isFetching,
  onRefresh,
}: RefreshIndicatorProps) {
  const [timeAgo, setTimeAgo] = useState<string>("");

  // Update "time ago" every 10 seconds
  useEffect(() => {
    if (!lastUpdated) return;

    const update = () => {
      const seconds = Math.floor((Date.now() - lastUpdated.getTime()) / 1000);
      if (seconds < 60) {
        setTimeAgo("just now");
      } else {
        const minutes = Math.floor(seconds / 60);
        setTimeAgo(`${minutes}m ago`);
      }
    };

    update();
    const interval = setInterval(update, 10000);
    return () => clearInterval(interval);
  }, [lastUpdated]);

  return (
    <div className="flex items-center gap-3">
      {/* Last updated */}
      <span className="text-sm text-neutral-400">
        Updated {timeAgo}
      </span>

      {/* Refresh button */}
      <button
        onClick={onRefresh}
        disabled={isFetching}
        className={cn(
          "p-2 rounded-lg transition-all",
          "text-neutral-500 hover:text-neutral-700 hover:bg-neutral-100",
          "disabled:opacity-50 disabled:cursor-not-allowed"
        )}
        title="Refresh data"
      >
        <ReloadIcon className={cn("w-5 h-5", isFetching && "animate-spin")} />
      </button>

      {/* Auto-refresh indicator */}
      {isFetching && (
        <span className="flex items-center gap-1.5 text-sm text-primary-500">
          <span className="relative flex h-2 w-2">
            <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-primary-400 opacity-75" />
            <span className="relative inline-flex rounded-full h-2 w-2 bg-primary-500" />
          </span>
          Updating...
        </span>
      )}
    </div>
  );
}
```

### Animated KPI Card Updates

```tsx
// frontend/src/components/dashboard/AnimatedValue.tsx
import { useEffect, useState, useRef } from "react";

interface AnimatedValueProps {
  value: number;
  formatter?: (value: number) => string;
  duration?: number;
}

export function AnimatedValue({
  value,
  formatter = (v) => v.toString(),
  duration = 500,
}: AnimatedValueProps) {
  const [displayValue, setDisplayValue] = useState(value);
  const previousValue = useRef(value);

  useEffect(() => {
    const start = previousValue.current;
    const end = value;
    const startTime = Date.now();

    if (start === end) return;

    const animate = () => {
      const elapsed = Date.now() - startTime;
      const progress = Math.min(elapsed / duration, 1);
      
      // Ease-out quad
      const eased = 1 - (1 - progress) * (1 - progress);
      const current = start + (end - start) * eased;
      
      setDisplayValue(current);

      if (progress < 1) {
        requestAnimationFrame(animate);
      } else {
        previousValue.current = end;
      }
    };

    requestAnimationFrame(animate);
  }, [value, duration]);

  return <span>{formatter(Math.round(displayValue))}</span>;
}

// Usage in KPI Card
<p className="text-3xl font-bold text-neutral-900">
  <AnimatedValue value={data?.ticketsCompleted || 0} />
</p>
```

### Recent Activity with Live Updates

```tsx
// frontend/src/components/dashboard/RecentActivity.tsx
import { useEffect, useRef } from "react";
import { formatDistanceToNow } from "date-fns";

interface Activity {
  id: string;
  type: "workflow_completed" | "report_generated" | "step_completed";
  ticketKey: string;
  description: string;
  timestamp: string;
}

interface RecentActivityProps {
  activities?: Activity[];
  isLoading: boolean;
}

export function RecentActivity({ activities, isLoading }: RecentActivityProps) {
  const listRef = useRef<HTMLDivElement>(null);
  const previousCount = useRef(activities?.length || 0);

  // Animate new items
  useEffect(() => {
    const currentCount = activities?.length || 0;
    if (currentCount > previousCount.current && listRef.current) {
      const firstItem = listRef.current.firstElementChild;
      if (firstItem) {
        firstItem.classList.add("animate-highlight");
        setTimeout(() => {
          firstItem.classList.remove("animate-highlight");
        }, 2000);
      }
    }
    previousCount.current = currentCount;
  }, [activities?.length]);

  if (isLoading) {
    return <ActivitySkeleton />;
  }

  return (
    <div className="bg-white rounded-xl border border-neutral-200">
      <div className="p-4 border-b border-neutral-200">
        <h3 className="font-semibold text-neutral-900">Recent Activity</h3>
      </div>

      <div ref={listRef} className="divide-y divide-neutral-100 max-h-96 overflow-y-auto">
        {activities?.length === 0 ? (
          <div className="p-8 text-center text-neutral-500">
            No recent activity
          </div>
        ) : (
          activities?.map((activity) => (
            <ActivityItem key={activity.id} activity={activity} />
          ))
        )}
      </div>
    </div>
  );
}

function ActivityItem({ activity }: { activity: Activity }) {
  const icons = {
    workflow_completed: "✅",
    report_generated: "📄",
    step_completed: "⏱️",
  };

  return (
    <div className="p-4 hover:bg-neutral-50 transition-colors">
      <div className="flex items-start gap-3">
        <span className="text-lg">{icons[activity.type]}</span>
        <div className="flex-1 min-w-0">
          <p className="text-sm text-neutral-900">
            <span className="font-medium font-mono">{activity.ticketKey}</span>
            {" "}{activity.description}
          </p>
          <p className="text-xs text-neutral-400 mt-1">
            {formatDistanceToNow(new Date(activity.timestamp), { addSuffix: true })}
          </p>
        </div>
      </div>
    </div>
  );
}
```

### CSS Animation for Highlights

```css
@keyframes highlight {
  0% { background-color: rgb(99 102 241 / 0.1); }
  100% { background-color: transparent; }
}

.animate-highlight {
  animation: highlight 2s ease-out;
}
```

### Dashboard Integration

```tsx
// frontend/src/pages/Dashboard.tsx
export function DashboardPage() {
  const { period, customRange, setPeriod } = usePeriodState();
  const { data, isLoading, isFetching, dataUpdatedAt, refresh } = useDashboardData({
    period,
    startDate: customRange?.from,
    endDate: customRange?.to,
  });

  return (
    <div className="p-6 space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">Dashboard</h1>
        
        <div className="flex items-center gap-4">
          <PeriodSelector value={period} customRange={customRange} onChange={setPeriod} />
          <RefreshIndicator
            lastUpdated={dataUpdatedAt ? new Date(dataUpdatedAt) : null}
            isFetching={isFetching}
            onRefresh={refresh}
          />
        </div>
      </div>

      {/* Rest of dashboard... */}
    </div>
  );
}
```

### References

- [Source: epics.md#Story 8.6]
- [React Query refetch](https://tanstack.com/query/v5/docs/react/guides/important-defaults)
