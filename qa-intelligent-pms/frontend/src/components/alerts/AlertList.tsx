/**
 * Alert list component.
 * Story 9.4: Displays alerts with context.
 */
import { formatDistanceToNow } from "date-fns";
import { useAlerts, useMarkAlertRead, useDismissAlert, type Alert } from "@/hooks/useAlerts";

export function AlertList() {
  const { data, isLoading } = useAlerts();
  const markRead = useMarkAlertRead();
  const dismiss = useDismissAlert();

  if (isLoading) {
    return (
      <div className="p-4 space-y-3">
        {[1, 2, 3].map((i) => (
          <div key={i} className="animate-pulse">
            <div className="h-4 bg-neutral-200 rounded w-3/4 mb-2" />
            <div className="h-3 bg-neutral-100 rounded w-1/2" />
          </div>
        ))}
      </div>
    );
  }

  const alerts = data?.alerts ?? [];

  if (alerts.length === 0) {
    return (
      <div className="p-8 text-center text-neutral-500">
        <BellOffIcon className="w-8 h-8 mx-auto mb-2 opacity-50" />
        <p>No alerts</p>
      </div>
    );
  }

  const handleMarkRead = (id: string) => {
    markRead.mutate(id);
  };

  const handleDismiss = (id: string) => {
    dismiss.mutate(id);
  };

  return (
    <div className="divide-y divide-neutral-100 max-h-96 overflow-y-auto">
      {alerts.map((alert) => (
        <AlertItem
          key={alert.id}
          alert={alert}
          onMarkRead={() => handleMarkRead(alert.id)}
          onDismiss={() => handleDismiss(alert.id)}
        />
      ))}
    </div>
  );
}

interface AlertItemProps {
  alert: Alert;
  onMarkRead: () => void;
  onDismiss: () => void;
}

function AlertItem({ alert, onMarkRead, onDismiss }: AlertItemProps) {
  const severityColors = {
    critical: "border-l-red-500 bg-red-50",
    warning: "border-l-amber-500 bg-amber-50",
    info: "border-l-blue-500 bg-blue-50",
  };

  const severityIcons = {
    critical: "🔴",
    warning: "🟡",
    info: "🔵",
  };

  const typeLabels = {
    time_excess: "Time Excess",
    consecutive_problem: "Recurring Issue",
    spike: "Volume Spike",
  };

  return (
    <div
      className={`p-4 border-l-4 ${severityColors[alert.severity]} ${!alert.isRead ? "font-medium" : "opacity-80"}`}
    >
      <div className="flex items-start justify-between gap-2">
        <div className="flex-1 min-w-0">
          <div className="flex items-center gap-2 mb-1">
            <span>{severityIcons[alert.severity]}</span>
            <span className="text-xs text-neutral-500 uppercase tracking-wide">
              {typeLabels[alert.alertType]}
            </span>
          </div>
          
          <h4 className="text-sm text-neutral-900 mb-1">{alert.title}</h4>
          
          {alert.message && (
            <p className="text-xs text-neutral-600 mb-2">{alert.message}</p>
          )}

          {alert.affectedTickets.length > 0 && (
            <div className="flex flex-wrap gap-1 mb-2">
              {alert.affectedTickets.slice(0, 3).map((ticket) => (
                <span
                  key={ticket}
                  className="px-1.5 py-0.5 text-xs font-mono bg-white rounded border border-neutral-200"
                >
                  {ticket}
                </span>
              ))}
              {alert.affectedTickets.length > 3 && (
                <span className="text-xs text-neutral-500">
                  +{alert.affectedTickets.length - 3} more
                </span>
              )}
            </div>
          )}

          <div className="flex items-center gap-3 text-xs text-neutral-400">
            <span>{formatDistanceToNow(new Date(alert.createdAt), { addSuffix: true })}</span>
          </div>
        </div>

        <div className="flex flex-col gap-1">
          {!alert.isRead && (
            <button
              onClick={onMarkRead}
              className="p-1 text-neutral-400 hover:text-neutral-600 rounded"
              title="Mark as read"
            >
              <CheckIcon className="w-4 h-4" />
            </button>
          )}
          <button
            onClick={onDismiss}
            className="p-1 text-neutral-400 hover:text-red-500 rounded"
            title="Dismiss"
          >
            <XIcon className="w-4 h-4" />
          </button>
        </div>
      </div>

      {alert.suggestedActions.length > 0 && (
        <details className="mt-2">
          <summary className="text-xs text-indigo-600 cursor-pointer hover:underline">
            Suggested actions ({alert.suggestedActions.length})
          </summary>
          <ul className="mt-1 pl-4 text-xs text-neutral-600 list-disc">
            {alert.suggestedActions.map((action, i) => (
              <li key={i}>{action}</li>
            ))}
          </ul>
        </details>
      )}
    </div>
  );
}

function BellOffIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M9.143 17.082a24.248 24.248 0 003.714.318c.61 0 1.214-.024 1.81-.072M9.143 17.082A23.848 23.848 0 013.69 15.772a8.966 8.966 0 012.31-6.022V9a6.001 6.001 0 016-6c.693 0 1.36.117 1.98.332m-5.837 12.75a3 3 0 005.714 0" />
      <path strokeLinecap="round" strokeLinejoin="round" d="M3 3l18 18" />
    </svg>
  );
}

function CheckIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M4.5 12.75l6 6 9-13.5" />
    </svg>
  );
}

function XIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
    </svg>
  );
}
