/**
 * Recent activity list component.
 * Shows recent workflow completions and ticket updates.
 */
import { formatDistanceToNow } from "date-fns";

export interface ActivityItem {
  id: string;
  type: "workflow_completed" | "ticket_closed" | "time_logged";
  title: string;
  ticketKey?: string;
  timestamp: string;
  duration?: number; // seconds
}

interface RecentActivityProps {
  activities?: ActivityItem[];
  isLoading: boolean;
}

export function RecentActivity({ activities, isLoading }: RecentActivityProps) {
  if (isLoading) {
    return (
      <div className="bg-white rounded-xl border border-neutral-200 p-5">
        <div className="w-32 h-6 bg-neutral-100 rounded animate-pulse mb-4" />
        <div className="space-y-3">
          {[1, 2, 3, 4, 5].map((i) => (
            <div key={i} className="flex items-start gap-3">
              <div className="w-8 h-8 bg-neutral-100 rounded-full animate-pulse" />
              <div className="flex-1 space-y-2">
                <div className="w-3/4 h-4 bg-neutral-100 rounded animate-pulse" />
                <div className="w-1/2 h-3 bg-neutral-100 rounded animate-pulse" />
              </div>
            </div>
          ))}
        </div>
      </div>
    );
  }

  const items = activities ?? [];

  return (
    <div className="bg-white rounded-xl border border-neutral-200 p-5">
      <h3 className="font-semibold text-neutral-900 mb-4">Recent Activity</h3>

      {items.length === 0 ? (
        <div className="py-8 text-center text-neutral-400">
          No recent activity
        </div>
      ) : (
        <div className="space-y-3 max-h-80 overflow-y-auto">
          {items.map((item) => (
            <ActivityRow key={item.id} item={item} />
          ))}
        </div>
      )}
    </div>
  );
}

function ActivityRow({ item }: { item: ActivityItem }) {
  const icon = getActivityIcon(item.type);
  const timeAgo = formatDistanceToNow(new Date(item.timestamp), {
    addSuffix: true,
  });

  return (
    <div className="flex items-start gap-3 py-2">
      <div
        className={`
          w-8 h-8 rounded-full flex items-center justify-center flex-shrink-0
          ${getIconBackground(item.type)}
        `}
      >
        {icon}
      </div>
      <div className="flex-1 min-w-0">
        <p className="text-sm text-neutral-900 truncate">{item.title}</p>
        <div className="flex items-center gap-2 text-xs text-neutral-500">
          {item.ticketKey && (
            <>
              <span className="font-mono">{item.ticketKey}</span>
              <span>•</span>
            </>
          )}
          <span>{timeAgo}</span>
          {item.duration && (
            <>
              <span>•</span>
              <span>{formatDuration(item.duration)}</span>
            </>
          )}
        </div>
      </div>
    </div>
  );
}

function getActivityIcon(type: ActivityItem["type"]) {
  switch (type) {
    case "workflow_completed":
      return (
        <svg
          className="w-4 h-4 text-emerald-600"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth={2}
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
      );
    case "ticket_closed":
      return (
        <svg
          className="w-4 h-4 text-indigo-600"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth={2}
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M16.5 6v.75m0 3v.75m0 3v.75m0 3V18m-9-5.25h5.25M7.5 15h3M3.375 5.25c-.621 0-1.125.504-1.125 1.125v3.026a2.999 2.999 0 010 5.198v3.026c0 .621.504 1.125 1.125 1.125h17.25c.621 0 1.125-.504 1.125-1.125v-3.026a2.999 2.999 0 010-5.198V6.375c0-.621-.504-1.125-1.125-1.125H3.375z"
          />
        </svg>
      );
    case "time_logged":
      return (
        <svg
          className="w-4 h-4 text-amber-600"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth={2}
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
      );
  }
}

function getIconBackground(type: ActivityItem["type"]) {
  switch (type) {
    case "workflow_completed":
      return "bg-emerald-50";
    case "ticket_closed":
      return "bg-indigo-50";
    case "time_logged":
      return "bg-amber-50";
  }
}

function formatDuration(seconds: number): string {
  if (seconds < 60) return `${seconds}s`;
  if (seconds < 3600) return `${Math.round(seconds / 60)}m`;
  const hours = Math.floor(seconds / 3600);
  const mins = Math.round((seconds % 3600) / 60);
  return mins > 0 ? `${hours}h ${mins}m` : `${hours}h`;
}
