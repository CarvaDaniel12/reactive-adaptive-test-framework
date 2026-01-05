# Story 9.4: Alert Display with Context

Status: done

## Story

As a QA/PM,
I want to see alert details with full context,
So that I can investigate and act.

## Acceptance Criteria

1. **Given** alerts have been generated
   **When** user views alerts (bell icon or alerts page)
   **Then** each alert shows alert title and type

2. **Given** alert is displayed
   **When** severity shown
   **Then** severity indicator is visible

3. **Given** alert is displayed
   **When** tickets shown
   **Then** affected tickets are clickable links

4. **Given** alert is displayed
   **When** pattern shown
   **Then** pattern description is included

5. **Given** alert is displayed
   **When** actions shown
   **Then** suggested actions are listed

6. **Given** alert is displayed
   **When** user interacts
   **Then** "Dismiss" and "Investigate" buttons available

7. **Given** alert list exists
   **When** sorted
   **Then** alerts are sorted by severity, then date

8. **Given** alert is dismissed
   **When** viewing alerts
   **Then** dismissed alerts are hidden but accessible

9. **Given** unread alerts exist
   **When** header displayed
   **Then** unread count shows in header

## Tasks

- [ ] Task 1: Create AlertsPanel component (slide-over)
- [ ] Task 2: Create AlertCard component
- [ ] Task 3: Implement severity indicators
- [ ] Task 4: Create ticket links in alerts
- [ ] Task 5: Add dismiss/investigate actions
- [ ] Task 6: Create dismissed alerts section
- [ ] Task 7: Update header with alert count

## Dev Notes

### Alerts Panel (Slide-over)

```tsx
// frontend/src/components/alerts/AlertsPanel.tsx
import * as Dialog from "@radix-ui/react-dialog";
import { BellIcon, Cross2Icon } from "@radix-ui/react-icons";
import { useAlerts, useDismissAlert, useMarkAlertRead } from "@/hooks/useAlerts";
import { useState } from "react";

interface AlertsPanelProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function AlertsPanel({ open, onOpenChange }: AlertsPanelProps) {
  const [showDismissed, setShowDismissed] = useState(false);
  const { data: alerts, isLoading } = useAlerts({ includeDismissed: showDismissed });

  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-black/30 z-40" />
        <Dialog.Content
          className="fixed right-0 top-0 h-full w-full max-w-md bg-white shadow-xl z-50
                     animate-slide-in-right focus:outline-none"
        >
          {/* Header */}
          <div className="flex items-center justify-between p-4 border-b border-neutral-200">
            <div className="flex items-center gap-2">
              <BellIcon className="w-5 h-5 text-neutral-500" />
              <Dialog.Title className="text-lg font-semibold">Alerts</Dialog.Title>
            </div>
            <Dialog.Close asChild>
              <button className="p-2 text-neutral-400 hover:text-neutral-600 rounded-lg">
                <Cross2Icon className="w-5 h-5" />
              </button>
            </Dialog.Close>
          </div>

          {/* Filter */}
          <div className="px-4 py-2 border-b border-neutral-100">
            <label className="flex items-center gap-2 text-sm text-neutral-500">
              <input
                type="checkbox"
                checked={showDismissed}
                onChange={(e) => setShowDismissed(e.target.checked)}
                className="rounded border-neutral-300"
              />
              Show dismissed
            </label>
          </div>

          {/* Alerts List */}
          <div className="flex-1 overflow-y-auto p-4 space-y-3">
            {isLoading ? (
              <AlertsSkeleton />
            ) : alerts?.length === 0 ? (
              <EmptyAlerts />
            ) : (
              alerts?.map((alert) => (
                <AlertCard key={alert.id} alert={alert} />
              ))
            )}
          </div>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}

function EmptyAlerts() {
  return (
    <div className="text-center py-12">
      <BellIcon className="w-12 h-12 mx-auto text-neutral-300 mb-4" />
      <p className="text-neutral-500">No alerts</p>
      <p className="text-sm text-neutral-400">You're all caught up!</p>
    </div>
  );
}
```

### Alert Card Component

```tsx
// frontend/src/components/alerts/AlertCard.tsx
import { Link } from "react-router-dom";
import { formatDistanceToNow } from "date-fns";
import { 
  ExclamationTriangleIcon, 
  InfoCircledIcon,
  Cross2Icon,
  MagnifyingGlassIcon 
} from "@radix-ui/react-icons";
import { useDismissAlert, useMarkAlertRead } from "@/hooks/useAlerts";
import { cn } from "@/lib/utils";

interface AlertCardProps {
  alert: Alert;
}

export function AlertCard({ alert }: AlertCardProps) {
  const { mutate: dismiss } = useDismissAlert();
  const { mutate: markRead } = useMarkAlertRead();

  // Mark as read when viewed
  useEffect(() => {
    if (!alert.isRead) {
      markRead(alert.id);
    }
  }, [alert.id, alert.isRead]);

  const severityConfig = {
    critical: {
      icon: ExclamationTriangleIcon,
      bg: "bg-error-50",
      border: "border-error-200",
      iconColor: "text-error-500",
      badge: "bg-error-100 text-error-700",
    },
    warning: {
      icon: ExclamationTriangleIcon,
      bg: "bg-warning-50",
      border: "border-warning-200",
      iconColor: "text-warning-500",
      badge: "bg-warning-100 text-warning-700",
    },
    info: {
      icon: InfoCircledIcon,
      bg: "bg-primary-50",
      border: "border-primary-200",
      iconColor: "text-primary-500",
      badge: "bg-primary-100 text-primary-700",
    },
  };

  const config = severityConfig[alert.severity as keyof typeof severityConfig] || severityConfig.info;
  const Icon = config.icon;

  return (
    <div
      className={cn(
        "rounded-lg border p-4 transition-all",
        config.bg,
        config.border,
        alert.isDismissed && "opacity-60"
      )}
    >
      {/* Header */}
      <div className="flex items-start justify-between gap-3 mb-3">
        <div className="flex items-start gap-3">
          <Icon className={cn("w-5 h-5 mt-0.5", config.iconColor)} />
          <div>
            <h4 className="font-medium text-neutral-900">{alert.title}</h4>
            <div className="flex items-center gap-2 mt-1">
              <span className={cn("text-xs px-2 py-0.5 rounded-full", config.badge)}>
                {alert.severity}
              </span>
              <span className="text-xs text-neutral-400">
                {formatDistanceToNow(new Date(alert.createdAt), { addSuffix: true })}
              </span>
            </div>
          </div>
        </div>

        {!alert.isDismissed && (
          <button
            onClick={() => dismiss(alert.id)}
            className="p-1 text-neutral-400 hover:text-neutral-600 rounded"
            title="Dismiss"
          >
            <Cross2Icon className="w-4 h-4" />
          </button>
        )}
      </div>

      {/* Description */}
      <p className="text-sm text-neutral-600 mb-3">{alert.description}</p>

      {/* Affected Tickets */}
      {alert.affectedTickets?.length > 0 && (
        <div className="mb-3">
          <p className="text-xs text-neutral-500 mb-1">Affected tickets:</p>
          <div className="flex flex-wrap gap-1">
            {alert.affectedTickets.map((ticket) => (
              <Link
                key={ticket}
                to={`/tickets/${ticket}`}
                className="text-xs font-mono px-2 py-1 bg-white rounded border 
                           border-neutral-200 hover:border-primary-300 hover:text-primary-600"
              >
                {ticket}
              </Link>
            ))}
          </div>
        </div>
      )}

      {/* Suggested Actions */}
      {alert.suggestedActions?.length > 0 && (
        <div className="mb-3">
          <p className="text-xs text-neutral-500 mb-1">Suggested actions:</p>
          <ul className="text-sm text-neutral-600 space-y-1">
            {alert.suggestedActions.map((action, i) => (
              <li key={i} className="flex items-start gap-2">
                <span className="text-neutral-400">•</span>
                {action}
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Actions */}
      {!alert.isDismissed && (
        <div className="flex items-center gap-2 pt-3 border-t border-neutral-200/50">
          <Link
            to={`/alerts/${alert.id}/investigate`}
            className="flex items-center gap-1.5 px-3 py-1.5 text-sm font-medium
                       text-primary-600 hover:text-primary-700 hover:bg-white rounded"
          >
            <MagnifyingGlassIcon className="w-4 h-4" />
            Investigate
          </Link>
          <button
            onClick={() => dismiss(alert.id)}
            className="px-3 py-1.5 text-sm text-neutral-500 hover:text-neutral-700"
          >
            Dismiss
          </button>
        </div>
      )}
    </div>
  );
}
```

### Header Alert Button

```tsx
// frontend/src/components/layout/Header.tsx
import { useState } from "react";
import { BellIcon } from "@radix-ui/react-icons";
import { useAlertCount } from "@/hooks/useAlerts";
import { AlertsPanel } from "@/components/alerts/AlertsPanel";

export function AlertButton() {
  const [panelOpen, setPanelOpen] = useState(false);
  const { data: count } = useAlertCount();

  return (
    <>
      <button
        onClick={() => setPanelOpen(true)}
        className="relative p-2 text-neutral-500 hover:text-neutral-700 
                   hover:bg-neutral-100 rounded-lg transition-colors"
      >
        <BellIcon className="w-6 h-6" />
        {count > 0 && (
          <span className="absolute -top-0.5 -right-0.5 flex items-center justify-center 
                           min-w-5 h-5 px-1 text-xs font-bold text-white 
                           bg-error-500 rounded-full">
            {count > 99 ? "99+" : count}
          </span>
        )}
      </button>

      <AlertsPanel open={panelOpen} onOpenChange={setPanelOpen} />
    </>
  );
}
```

### useAlerts Hooks

```tsx
// frontend/src/hooks/useAlerts.ts
export function useAlerts(options: { includeDismissed?: boolean } = {}) {
  return useQuery({
    queryKey: ["alerts", options.includeDismissed],
    queryFn: async () => {
      const params = new URLSearchParams();
      if (options.includeDismissed) params.set("includeDismissed", "true");
      const response = await fetch(`/api/v1/alerts?${params}`);
      return response.json();
    },
  });
}

export function useAlertCount() {
  return useQuery({
    queryKey: ["alerts", "count"],
    queryFn: async () => {
      const response = await fetch("/api/v1/alerts/count");
      return response.json();
    },
    refetchInterval: 30000,
  });
}

export function useDismissAlert() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (alertId: string) =>
      fetch(`/api/v1/alerts/${alertId}/dismiss`, { method: "PUT" }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["alerts"] });
    },
  });
}
```

### References

- [Source: epics.md#Story 9.4]
