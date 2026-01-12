/**
 * QA Dashboard page showing personal performance metrics.
 * Displays KPIs, trend chart, and recent activity.
 * 
 * Stories implemented:
 * - 8.1: Dashboard layout and navigation
 * - 8.2: Tickets completed KPI card
 * - 8.3: Time metrics KPI cards
 * - 8.4: Trend visualization chart with toggle
 * - 8.5: Dashboard period filters with URL persistence
 * - 8.6: Dashboard real-time refresh with indicator
 */
import { useEffect } from "react";
import { useSearchParams, useLocation } from "react-router-dom";
import {
  PeriodSelector,
  KPICards,
  TrendChart,
  RecentActivity,
  IntegrationHealthWidget,
  type Period,
} from "@/components/dashboard";
import { useDashboardData } from "@/hooks/useDashboardData";
import { useLayoutStore } from "@/stores/layoutStore";

const VALID_PERIODS: Period[] = ["7d", "30d", "90d", "1y"];

function isValidPeriod(value: string | null): value is Period {
  return value !== null && VALID_PERIODS.includes(value as Period);
}

export function DashboardPage() {
  const [searchParams, setSearchParams] = useSearchParams();
  const location = useLocation();
  const { setSidebarCollapsed } = useLayoutStore();
  
  // Get period from URL or default to 30d
  const urlPeriod = searchParams.get("period");
  const period: Period = isValidPeriod(urlPeriod) ? urlPeriod : "30d";
  
  const { data, isLoading, refetch, isFetching, dataUpdatedAt } = useDashboardData(period);

  // Story 8.1 AC #5: Dashboard mode (expanded sidebar)
  // Auto-expand sidebar when on dashboard route
  useEffect(() => {
    if (location.pathname === "/" || location.pathname.startsWith("/dashboard")) {
      setSidebarCollapsed(false);
    }
  }, [location.pathname, setSidebarCollapsed]);

  // Update URL when period changes
  const handlePeriodChange = (newPeriod: Period) => {
    setSearchParams({ period: newPeriod });
  };

  // Set default period in URL if not present
  useEffect(() => {
    if (!isValidPeriod(urlPeriod)) {
      setSearchParams({ period: "30d" }, { replace: true });
    }
  }, [urlPeriod, setSearchParams]);

  // Format last updated time
  const lastUpdated = dataUpdatedAt
    ? new Date(dataUpdatedAt).toLocaleTimeString()
    : null;

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-neutral-900">Dashboard</h1>
          <p className="text-sm text-neutral-500">
            Your QA performance overview
            {lastUpdated && !isFetching && (
              <span className="ml-2 text-neutral-400">
                · Updated {lastUpdated}
              </span>
            )}
            {isFetching && !isLoading && (
              <span className="ml-2 text-indigo-500 animate-pulse">
                · Refreshing...
              </span>
            )}
          </p>
        </div>

        <div className="flex items-center gap-4">
          <PeriodSelector value={period} onChange={handlePeriodChange} />
          <button
            type="button"
            onClick={() => refetch()}
            disabled={isFetching}
            className={`
              p-2 rounded-lg transition-colors relative
              ${isFetching
                ? "text-indigo-400"
                : "text-neutral-500 hover:text-neutral-700 hover:bg-neutral-100"}
            `}
            title={isFetching ? "Refreshing..." : "Refresh dashboard"}
          >
            <RefreshIcon className={`w-5 h-5 ${isFetching ? "animate-spin" : ""}`} />
          </button>
        </div>
      </div>

      {/* Background refresh indicator bar */}
      {isFetching && !isLoading && (
        <div className="h-0.5 bg-indigo-100 rounded-full overflow-hidden">
          <div className="h-full bg-indigo-500 animate-pulse w-full" />
        </div>
      )}

      {/* KPI Cards - Stories 8.2, 8.3 */}
      <KPICards data={data?.kpis} isLoading={isLoading} />

      {/* Integration Health Widget - Story 22.6 */}
      <IntegrationHealthWidget />

      {/* Main Content Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Trend Chart - 2 columns - Story 8.4 */}
        <div className="lg:col-span-2">
          <TrendChart data={data?.trend} period={period} isLoading={isLoading} />
        </div>

        {/* Recent Activity - 1 column */}
        <div className="lg:col-span-1">
          <RecentActivity activities={data?.recentActivity} isLoading={isLoading} />
        </div>
      </div>
    </div>
  );
}

function RefreshIcon({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={2}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99"
      />
    </svg>
  );
}
