/**
 * Tickets Completed Detail Page
 * 
 * Story 8.2 AC #6: Detail view showing list of completed tickets/workflows
 */
import { Link, useSearchParams } from "react-router-dom";
import { PeriodSelector, type Period } from "@/components/dashboard";
import { useDashboardData } from "@/hooks/useDashboardData";
import { formatDuration } from "@/utils/time";

const VALID_PERIODS: Period[] = ["7d", "30d", "90d", "1y"];

function isValidPeriod(value: string | null): value is Period {
  return value !== null && VALID_PERIODS.includes(value as Period);
}

function getPeriodDays(period: Period): number {
  switch (period) {
    case "7d": return 7;
    case "30d": return 30;
    case "90d": return 90;
    case "1y": return 365;
    default: return 30;
  }
}

export function TicketsCompletedDetailPage() {
  const [searchParams, setSearchParams] = useSearchParams();
  
  const urlPeriod = searchParams.get("period");
  const period: Period = isValidPeriod(urlPeriod) ? urlPeriod : "30d";
  
  const { data, isLoading } = useDashboardData(period);

  const handlePeriodChange = (newPeriod: Period) => {
    setSearchParams({ period: newPeriod });
  };

  const ticketsCompleted = data?.kpis.ticketsCompleted.value ?? 0;
  const changePercent = data?.kpis.ticketsCompleted.change ?? 0;
  const breakdown = data?.kpis.ticketsBreakdownByType || [];
  const recentActivity = data?.recentActivity || [];

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <Link
            to="/"
            className="text-sm text-indigo-600 hover:text-indigo-700 hover:underline mb-2 inline-block"
          >
            ← Back to Dashboard
          </Link>
          <h1 className="text-2xl font-bold text-neutral-900">Tickets Completed</h1>
          <p className="text-sm text-neutral-500">Detailed view of completed workflows</p>
        </div>
        <PeriodSelector value={period} onChange={handlePeriodChange} />
      </div>

      {/* Summary Stats */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="bg-white rounded-xl border border-neutral-200 p-5">
          <p className="text-sm text-neutral-500 mb-1">Total Completed</p>
          <p className="text-3xl font-bold text-neutral-900">{ticketsCompleted}</p>
        </div>
        <div className="bg-white rounded-xl border border-neutral-200 p-5">
          <p className="text-sm text-neutral-500 mb-1">Change from Previous Period</p>
          <p className={`text-3xl font-bold ${changePercent >= 0 ? "text-emerald-600" : "text-red-600"}`}>
            {changePercent > 0 ? "+" : ""}{changePercent}%
          </p>
        </div>
        <div className="bg-white rounded-xl border border-neutral-200 p-5">
          <p className="text-sm text-neutral-500 mb-1">Daily Average</p>
          <p className="text-3xl font-bold text-neutral-900">
            {(ticketsCompleted / getPeriodDays(period)).toFixed(1)}
          </p>
        </div>
      </div>

      {/* Breakdown by Type */}
      {breakdown.length > 0 && (
        <div className="bg-white rounded-xl border border-neutral-200 p-5">
          <h2 className="text-lg font-semibold text-neutral-900 mb-4">Breakdown by Ticket Type</h2>
          <div className="space-y-3">
            {breakdown.map((item) => (
              <div key={item.ticket_type} className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="w-3 h-3 rounded-full bg-indigo-500" />
                  <span className="text-sm font-medium text-neutral-700 capitalize">
                    {item.ticket_type}
                  </span>
                </div>
                <div className="flex items-center gap-4">
                  <span className="text-sm text-neutral-600">{item.count} tickets</span>
                  <span className="text-sm font-medium text-neutral-900 w-16 text-right">
                    {item.percentage.toFixed(1)}%
                  </span>
                  <div className="w-32 h-2 bg-neutral-100 rounded-full overflow-hidden">
                    <div
                      className="h-full bg-indigo-500 transition-all"
                      style={{ width: `${item.percentage}%` }}
                    />
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Recent Completed Workflows */}
      <div className="bg-white rounded-xl border border-neutral-200">
        <div className="p-5 border-b border-neutral-200">
          <h2 className="text-lg font-semibold text-neutral-900">Recent Completed Workflows</h2>
        </div>
        {isLoading ? (
          <div className="p-5">
            <div className="space-y-3">
              {[1, 2, 3, 4, 5].map((i) => (
                <div key={i} className="h-16 bg-neutral-100 rounded-lg animate-pulse" />
              ))}
            </div>
          </div>
        ) : recentActivity.length === 0 ? (
          <div className="p-10 text-center">
            <p className="text-neutral-500">No completed workflows found for this period.</p>
          </div>
        ) : (
          <div className="divide-y divide-neutral-100">
            {recentActivity.map((activity) => (
              <div
                key={activity.id}
                className="p-5 hover:bg-neutral-50 transition-colors"
              >
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <h3 className="font-medium text-neutral-900 mb-1">{activity.title}</h3>
                    {activity.ticketKey && (
                      <p className="text-sm text-neutral-500 mb-2">
                        Ticket: <span className="font-mono">{activity.ticketKey}</span>
                      </p>
                    )}
                    <div className="flex items-center gap-4 text-xs text-neutral-400">
                      <span>
                        {new Date(activity.timestamp).toLocaleDateString("en-US", {
                          month: "short",
                          day: "numeric",
                          year: "numeric",
                          hour: "2-digit",
                          minute: "2-digit",
                        })}
                      </span>
                      {activity.duration && (
                        <span>
                          Duration: {formatDuration(activity.duration)}
                        </span>
                      )}
                    </div>
                  </div>
                  {activity.ticketKey && (
                    <Link
                      to={`/tickets/${activity.ticketKey}`}
                      className="ml-4 px-3 py-1.5 text-sm text-indigo-600 hover:bg-indigo-50 rounded-lg transition-colors"
                    >
                      View Ticket
                    </Link>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
