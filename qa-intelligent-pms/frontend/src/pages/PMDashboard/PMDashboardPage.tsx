/**
 * PM/PO Observability Dashboard.
 * Epic 10: Dashboard for product managers with bugs, economy, and health metrics.
 */
import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { useSearchParams } from "react-router-dom";
import { useLayoutStore } from "@/stores/layoutStore";

interface PMDashboardData {
  summary: {
    totalTicketsTested: number;
    totalWorkflowsCompleted: number;
    activeQaUsers: number;
    avgTimePerTicketMinutes: number;
  };
  bugsMetrics: {
    bugsDiscovered: number;
    bugsPrevented: number;
    preventionRate: number;
    discoveredChange: number;
    preventedChange: number;
  };
  economyMetrics: {
    hoursSaved: number;
    costSaved: number;
    bugPreventionValue: number;
    totalEconomy: number;
    hourlyRate: number;
    avgBugFixCost: number;
  };
  componentHealth: Array<{
    component: string;
    bugCount: number;
    ticketCount: number;
    status: string;
    trend: string;
    lastIssueDate: string | null;
  }>;
  problematicEndpoints: Array<{
    endpoint: string;
    issueCount: number;
    commonIssues: string[];
    trend: string;
    affectedTickets: string[];
  }>;
  period: string;
  generatedAt: string;
}

async function fetchPMDashboard(period: string): Promise<PMDashboardData> {
  const response = await fetch(`/api/v1/pm-dashboard?period=${period}`);
  if (!response.ok) throw new Error("Failed to fetch PM dashboard");
  return response.json();
}

async function exportDashboard(period: string): Promise<string> {
  const response = await fetch(`/api/v1/pm-dashboard/export?period=${period}`);
  if (!response.ok) throw new Error("Failed to export dashboard");
  return response.text();
}

export function PMDashboardPage() {
  const { setPageTitle } = useLayoutStore();
  const [searchParams, setSearchParams] = useSearchParams();
  const period = searchParams.get("period") || "30d";

  const { data, isLoading, error, isFetching } = useQuery({
    queryKey: ["pm-dashboard", period],
    queryFn: () => fetchPMDashboard(period),
    staleTime: 60_000,
    refetchInterval: 60_000,
  });

  useEffect(() => {
    setPageTitle("PM Dashboard", "Observability & Quality Metrics");
    return () => setPageTitle("");
  }, [setPageTitle]);

  const handlePeriodChange = (newPeriod: string) => {
    setSearchParams({ period: newPeriod });
  };

  const handleExport = async () => {
    try {
      const csv = await exportDashboard(period);
      const blob = new Blob([csv], { type: "text/csv" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = `qa-metrics-${period}-${new Date().toISOString().split("T")[0]}.csv`;
      a.click();
      URL.revokeObjectURL(url);
    } catch (e) {
      console.error("Export failed:", e);
    }
  };

  if (isLoading) {
    return <DashboardSkeleton />;
  }

  if (error) {
    return (
      <div className="p-6">
        <div className="bg-red-50 border border-red-200 rounded-lg p-4 text-red-700">
          Failed to load PM dashboard. Please try again.
        </div>
      </div>
    );
  }

  if (!data) return null;

  return (
    <div className="p-6 space-y-6">
      {/* Header with period selector and export */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <PeriodSelector value={period} onChange={handlePeriodChange} />
          {isFetching && (
            <span className="text-xs text-neutral-400 animate-pulse">Updating...</span>
          )}
        </div>
        <button
          onClick={handleExport}
          className="flex items-center gap-2 px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors"
        >
          <DownloadIcon className="w-4 h-4" />
          Export CSV
        </button>
      </div>

      {/* Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <SummaryCard
          title="Tickets Tested"
          value={data.summary.totalTicketsTested}
          icon={<TicketIcon className="w-5 h-5" />}
        />
        <SummaryCard
          title="Workflows Completed"
          value={data.summary.totalWorkflowsCompleted}
          icon={<WorkflowIcon className="w-5 h-5" />}
        />
        <SummaryCard
          title="Active QA Users"
          value={data.summary.activeQaUsers}
          icon={<UsersIcon className="w-5 h-5" />}
        />
        <SummaryCard
          title="Avg Time/Ticket"
          value={`${data.summary.avgTimePerTicketMinutes.toFixed(0)}m`}
          icon={<ClockIcon className="w-5 h-5" />}
        />
      </div>

      {/* Bugs and Economy Section */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Bugs Metrics */}
        <div className="bg-white rounded-xl border border-neutral-200 p-6">
          <h3 className="text-lg font-semibold text-neutral-900 mb-4 flex items-center gap-2">
            <BugIcon className="w-5 h-5 text-red-500" />
            Bugs Metrics
          </h3>
          <div className="grid grid-cols-2 gap-4">
            <MetricCard
              label="Bugs Discovered"
              value={data.bugsMetrics.bugsDiscovered}
              change={data.bugsMetrics.discoveredChange}
              changeLabel="vs prev period"
            />
            <MetricCard
              label="Bugs Prevented"
              value={data.bugsMetrics.bugsPrevented}
              change={data.bugsMetrics.preventedChange}
              changeLabel="vs prev period"
              positive
            />
          </div>
          <div className="mt-4 p-4 bg-neutral-50 rounded-lg">
            <div className="flex items-center justify-between">
              <span className="text-sm text-neutral-600">Prevention Rate</span>
              <span className="text-2xl font-bold text-emerald-600">
                {(data.bugsMetrics.preventionRate * 100).toFixed(0)}%
              </span>
            </div>
            <div className="mt-2 h-2 bg-neutral-200 rounded-full overflow-hidden">
              <div
                className="h-full bg-emerald-500 rounded-full transition-all"
                style={{ width: `${data.bugsMetrics.preventionRate * 100}%` }}
              />
            </div>
          </div>
        </div>

        {/* Economy Metrics */}
        <div className="bg-white rounded-xl border border-neutral-200 p-6">
          <h3 className="text-lg font-semibold text-neutral-900 mb-4 flex items-center gap-2">
            <DollarIcon className="w-5 h-5 text-emerald-500" />
            Economy Metrics
          </h3>
          <div className="space-y-4">
            <div className="flex items-center justify-between p-3 bg-emerald-50 rounded-lg">
              <span className="text-sm text-neutral-600">Hours Saved</span>
              <span className="text-lg font-semibold text-emerald-700">
                {data.economyMetrics.hoursSaved.toFixed(1)}h
              </span>
            </div>
            <div className="flex items-center justify-between p-3 bg-emerald-50 rounded-lg">
              <span className="text-sm text-neutral-600">Cost Saved</span>
              <span className="text-lg font-semibold text-emerald-700">
                ${data.economyMetrics.costSaved.toFixed(0)}
              </span>
            </div>
            <div className="flex items-center justify-between p-3 bg-emerald-50 rounded-lg">
              <span className="text-sm text-neutral-600">Bug Prevention Value</span>
              <span className="text-lg font-semibold text-emerald-700">
                ${data.economyMetrics.bugPreventionValue.toFixed(0)}
              </span>
            </div>
            <div className="flex items-center justify-between p-4 bg-emerald-100 rounded-lg border border-emerald-200">
              <span className="text-sm font-medium text-emerald-800">Total Economy</span>
              <span className="text-2xl font-bold text-emerald-700">
                ${data.economyMetrics.totalEconomy.toFixed(0)}
              </span>
            </div>
          </div>
          <p className="mt-3 text-xs text-neutral-400">
            Based on ${data.economyMetrics.hourlyRate}/hour and ${data.economyMetrics.avgBugFixCost}/bug fix
          </p>
        </div>
      </div>

      {/* Component Health */}
      <div className="bg-white rounded-xl border border-neutral-200 p-6">
        <h3 className="text-lg font-semibold text-neutral-900 mb-4 flex items-center gap-2">
          <HeartIcon className="w-5 h-5 text-rose-500" />
          Component Health
        </h3>
        {data.componentHealth.length === 0 ? (
          <p className="text-neutral-500 text-center py-8">No component data available</p>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="text-left text-xs text-neutral-500 uppercase tracking-wide border-b border-neutral-100">
                  <th className="pb-3 pr-4">Component</th>
                  <th className="pb-3 pr-4">Status</th>
                  <th className="pb-3 pr-4">Bugs</th>
                  <th className="pb-3 pr-4">Tickets</th>
                  <th className="pb-3 pr-4">Trend</th>
                  <th className="pb-3">Last Issue</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-neutral-50">
                {data.componentHealth.map((comp) => (
                  <tr key={comp.component} className="hover:bg-neutral-50">
                    <td className="py-3 pr-4 font-medium text-neutral-900">{comp.component}</td>
                    <td className="py-3 pr-4">
                      <StatusBadge status={comp.status} />
                    </td>
                    <td className="py-3 pr-4 text-neutral-600">{comp.bugCount}</td>
                    <td className="py-3 pr-4 text-neutral-600">{comp.ticketCount}</td>
                    <td className="py-3 pr-4">
                      <TrendIndicator trend={comp.trend} />
                    </td>
                    <td className="py-3 text-sm text-neutral-500">
                      {comp.lastIssueDate || "—"}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* Problematic Endpoints */}
      <div className="bg-white rounded-xl border border-neutral-200 p-6">
        <h3 className="text-lg font-semibold text-neutral-900 mb-4 flex items-center gap-2">
          <AlertIcon className="w-5 h-5 text-amber-500" />
          Problematic Endpoints
        </h3>
        {data.problematicEndpoints.length === 0 ? (
          <p className="text-neutral-500 text-center py-8">No problematic endpoints detected</p>
        ) : (
          <div className="space-y-4">
            {data.problematicEndpoints.map((ep) => (
              <div key={ep.endpoint} className="p-4 border border-neutral-100 rounded-lg hover:border-neutral-200 transition-colors">
                <div className="flex items-center justify-between mb-2">
                  <code className="text-sm font-mono text-neutral-900 bg-neutral-100 px-2 py-1 rounded">
                    {ep.endpoint}
                  </code>
                  <span className="text-sm text-neutral-500">{ep.issueCount} issues</span>
                </div>
                {ep.affectedTickets.length > 0 && (
                  <div className="flex flex-wrap gap-1 mt-2">
                    {ep.affectedTickets.map((ticket) => (
                      <span key={ticket} className="px-2 py-0.5 text-xs font-mono bg-amber-50 text-amber-700 rounded">
                        {ticket}
                      </span>
                    ))}
                  </div>
                )}
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Footer */}
      <p className="text-xs text-neutral-400 text-center">
        Generated at {new Date(data.generatedAt).toLocaleString()}
      </p>
    </div>
  );
}

// Sub-components

function PeriodSelector({ value, onChange }: { value: string; onChange: (v: string) => void }) {
  const periods = [
    { value: "7d", label: "7 Days" },
    { value: "30d", label: "30 Days" },
    { value: "90d", label: "90 Days" },
    { value: "1y", label: "1 Year" },
  ];

  return (
    <div className="flex gap-1 p-1 bg-neutral-100 rounded-lg">
      {periods.map((p) => (
        <button
          key={p.value}
          onClick={() => onChange(p.value)}
          className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
            value === p.value
              ? "bg-white text-neutral-900 shadow-sm"
              : "text-neutral-600 hover:text-neutral-900"
          }`}
        >
          {p.label}
        </button>
      ))}
    </div>
  );
}

function SummaryCard({ title, value, icon }: { title: string; value: number | string; icon: React.ReactNode }) {
  return (
    <div className="bg-white rounded-xl border border-neutral-200 p-5">
      <div className="flex items-center gap-3">
        <div className="p-2 bg-primary-50 text-primary-600 rounded-lg">{icon}</div>
        <div>
          <p className="text-sm text-neutral-500">{title}</p>
          <p className="text-2xl font-bold text-neutral-900">{value}</p>
        </div>
      </div>
    </div>
  );
}

function MetricCard({
  label,
  value,
  change,
  changeLabel,
  positive = false,
}: {
  label: string;
  value: number;
  change: number;
  changeLabel: string;
  positive?: boolean;
}) {
  const isPositive = positive ? change >= 0 : change <= 0;
  return (
    <div className="p-4 bg-neutral-50 rounded-lg">
      <p className="text-sm text-neutral-500 mb-1">{label}</p>
      <p className="text-3xl font-bold text-neutral-900">{value}</p>
      <p className={`text-xs mt-1 ${isPositive ? "text-emerald-600" : "text-red-600"}`}>
        {change >= 0 ? "+" : ""}{change}% {changeLabel}
      </p>
    </div>
  );
}

function StatusBadge({ status }: { status: string }) {
  const colors = {
    healthy: "bg-emerald-100 text-emerald-700",
    degraded: "bg-amber-100 text-amber-700",
    critical: "bg-red-100 text-red-700",
  };
  const icons = {
    healthy: "✓",
    degraded: "⚠",
    critical: "✕",
  };
  return (
    <span className={`inline-flex items-center gap-1 px-2 py-1 text-xs font-medium rounded ${colors[status as keyof typeof colors] || colors.healthy}`}>
      {icons[status as keyof typeof icons]} {status}
    </span>
  );
}

function TrendIndicator({ trend }: { trend: string }) {
  const config = {
    improving: { icon: "↑", color: "text-emerald-600" },
    degrading: { icon: "↓", color: "text-red-600" },
    stable: { icon: "→", color: "text-neutral-500" },
    increasing: { icon: "↑", color: "text-amber-600" },
  };
  const { icon, color } = config[trend as keyof typeof config] || config.stable;
  return <span className={`font-medium ${color}`}>{icon} {trend}</span>;
}

function DashboardSkeleton() {
  return (
    <div className="p-6 space-y-6 animate-pulse">
      <div className="h-10 bg-neutral-200 rounded w-64" />
      <div className="grid grid-cols-4 gap-4">
        {[1, 2, 3, 4].map((i) => (
          <div key={i} className="h-24 bg-neutral-200 rounded-xl" />
        ))}
      </div>
      <div className="grid grid-cols-2 gap-6">
        <div className="h-64 bg-neutral-200 rounded-xl" />
        <div className="h-64 bg-neutral-200 rounded-xl" />
      </div>
      <div className="h-48 bg-neutral-200 rounded-xl" />
    </div>
  );
}

// Icons
function DownloadIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
    </svg>
  );
}

function TicketIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M16.5 6v.75m0 3v.75m0 3v.75m0 3V18m-9-5.25h5.25M7.5 15h3M3.375 5.25c-.621 0-1.125.504-1.125 1.125v3.026a2.999 2.999 0 010 5.198v3.026c0 .621.504 1.125 1.125 1.125h17.25c.621 0 1.125-.504 1.125-1.125v-3.026a2.999 2.999 0 010-5.198V6.375c0-.621-.504-1.125-1.125-1.125H3.375z" />
    </svg>
  );
}

function WorkflowIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z" />
    </svg>
  );
}

function UsersIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z" />
    </svg>
  );
}

function ClockIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z" />
    </svg>
  );
}

function BugIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M12 12.75c1.148 0 2.278.08 3.383.237 1.037.146 1.866.966 1.866 2.013 0 3.728-2.35 6.75-5.25 6.75S6.75 18.728 6.75 15c0-1.046.83-1.867 1.866-2.013A24.204 24.204 0 0112 12.75zm0 0c2.883 0 5.647.508 8.207 1.44a23.91 23.91 0 01-1.152 6.06M12 12.75c-2.883 0-5.647.508-8.208 1.44.125 2.104.52 4.136 1.153 6.06M12 12.75a2.25 2.25 0 002.248-2.354M12 12.75a2.25 2.25 0 01-2.248-2.354M12 8.25c.995 0 1.971-.08 2.922-.236.403-.066.74-.358.795-.762a3.778 3.778 0 00-.399-2.25M12 8.25c-.995 0-1.97-.08-2.922-.236-.402-.066-.74-.358-.795-.762a3.734 3.734 0 01.4-2.253M12 8.25a2.25 2.25 0 00-2.248 2.146M12 8.25a2.25 2.25 0 012.248 2.146M8.683 5a6.032 6.032 0 01-1.155-1.002c.07-.63.27-1.222.574-1.747m.581 2.749A3.75 3.75 0 0115.318 5m0 0c.427-.283.815-.62 1.155-.999a4.471 4.471 0 00-.575-1.752M4.921 6a24.048 24.048 0 00-.392 3.314c1.668.546 3.416.914 5.223 1.082M19.08 6c.205 1.08.337 2.187.392 3.314a23.882 23.882 0 01-5.223 1.082" />
    </svg>
  );
}

function DollarIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M12 6v12m-3-2.818l.879.659c1.171.879 3.07.879 4.242 0 1.172-.879 1.172-2.303 0-3.182C13.536 12.219 12.768 12 12 12c-.725 0-1.45-.22-2.003-.659-1.106-.879-1.106-2.303 0-3.182s2.9-.879 4.006 0l.415.33M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
    </svg>
  );
}

function HeartIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12z" />
    </svg>
  );
}

function AlertIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
    </svg>
  );
}
