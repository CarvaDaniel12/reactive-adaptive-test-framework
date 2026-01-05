/**
 * Anomaly Dashboard page showing detected anomalies.
 * Story 31.9: Displays historical anomalies with filtering, details, and trends.
 */
import { useState, useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { formatDistanceToNow } from "date-fns";
import { useLayoutStore } from "@/stores/layoutStore";
import { LineChart, Line, BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from "recharts";

interface Anomaly {
  id: string;
  detectedAt: string;
  anomalyType: string;
  severity: "info" | "warning" | "critical";
  description: string;
  metrics: {
    currentValue: number;
    baselineValue: number;
    deviation: number;
    zScore: number;
    confidence: number;
  };
  affectedEntities: string[];
  investigationSteps: string[];
}

interface AnomaliesResponse {
  anomalies: Anomaly[];
  total: number;
}

interface AnomalyTrendsResponse {
  countsByDate: Array<{ date: string; count: number }>;
  severityDistribution: Array<{ severity: string; count: number }>;
}

async function fetchAnomalies(params: {
  startDate?: string;
  endDate?: string;
  anomalyType?: string;
  severity?: string;
  limit?: number;
}): Promise<AnomaliesResponse> {
  const queryParams = new URLSearchParams();
  if (params.startDate) queryParams.set("start_date", params.startDate);
  if (params.endDate) queryParams.set("end_date", params.endDate);
  if (params.anomalyType) queryParams.set("anomaly_type", params.anomalyType);
  if (params.severity) queryParams.set("severity", params.severity);
  if (params.limit) queryParams.set("limit", params.limit.toString());

  const response = await fetch(`/api/v1/ai/anomalies?${queryParams.toString()}`);
  if (!response.ok) throw new Error("Failed to fetch anomalies");
  return response.json();
}

async function fetchAnomalyTrends(startDate: string, endDate: string): Promise<AnomalyTrendsResponse> {
  const response = await fetch(`/api/v1/ai/anomalies/trends?start_date=${startDate}&end_date=${endDate}`);
  if (!response.ok) throw new Error("Failed to fetch trends");
  return response.json();
}

export function AnomalyDashboardPage() {
  const { setPageTitle } = useLayoutStore();
  const [typeFilter, setTypeFilter] = useState<string>("all");
  const [severityFilter, setSeverityFilter] = useState<string>("all");
  const [selectedAnomaly, setSelectedAnomaly] = useState<Anomaly | null>(null);
  const [dateRange, setDateRange] = useState<"24h" | "7d" | "30d">("7d");

  // Calculate date range
  const getDateRange = () => {
    const now = new Date();
    const start = new Date();
    if (dateRange === "24h") {
      start.setHours(start.getHours() - 24);
    } else if (dateRange === "7d") {
      start.setDate(start.getDate() - 7);
    } else {
      start.setDate(start.getDate() - 30);
    }
    return {
      startDate: start.toISOString(),
      endDate: now.toISOString(),
    };
  };

  const { startDate, endDate } = getDateRange();

  const { data, isLoading, error } = useQuery({
    queryKey: ["anomalies", startDate, endDate, typeFilter, severityFilter],
    queryFn: () =>
      fetchAnomalies({
        startDate,
        endDate,
        anomalyType: typeFilter !== "all" ? typeFilter : undefined,
        severity: severityFilter !== "all" ? severityFilter : undefined,
        limit: 100,
      }),
    staleTime: 60_000,
  });

  const { data: trendsData, isLoading: trendsLoading } = useQuery({
    queryKey: ["anomaly-trends", startDate, endDate],
    queryFn: () => fetchAnomalyTrends(startDate, endDate),
    staleTime: 60_000,
  });

  useEffect(() => {
    setPageTitle("Anomaly Detection", "Detected anomalies and patterns");
    return () => {
      setPageTitle("");
    };
  }, [setPageTitle]);

  const anomalies = data?.anomalies ?? [];
  const trends = trendsData?.countsByDate ?? [];
  const severityDist = trendsData?.severityDistribution ?? [];

  const typeLabels: Record<string, string> = {
    spike_in_failures: "Spike in Failures",
    performance_degradation: "Performance Degradation",
    unusual_execution_time: "Unusual Execution Time",
    pattern_deviation: "Pattern Deviation",
    resource_exhaustion: "Resource Exhaustion",
    consecutive_failures: "Consecutive Failures",
  };

  const severityColors: Record<string, string> = {
    critical: "bg-red-100 text-red-800 border-red-200",
    warning: "bg-amber-100 text-amber-800 border-amber-200",
    info: "bg-blue-100 text-blue-800 border-blue-200",
  };

  const severityIcons: Record<string, string> = {
    critical: "🔴",
    warning: "🟡",
    info: "🔵",
  };

  if (isLoading) {
    return (
      <div className="p-6 space-y-4">
        {[1, 2, 3].map((i) => (
          <div key={i} className="animate-pulse bg-white rounded-lg p-6 border border-neutral-200">
            <div className="h-5 bg-neutral-200 rounded w-1/3 mb-3" />
            <div className="h-4 bg-neutral-100 rounded w-2/3 mb-2" />
            <div className="h-4 bg-neutral-100 rounded w-1/2" />
          </div>
        ))}
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-6">
        <div className="bg-red-50 border border-red-200 rounded-lg p-4 text-red-700">
          Failed to load anomalies. Please try again.
        </div>
      </div>
    );
  }

  return (
    <div className="p-6 space-y-6">
      {/* Header with Date Range Selector */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-neutral-900">Anomaly Detection</h1>
          <p className="text-sm text-neutral-500">Detected anomalies and patterns</p>
        </div>
        <div className="flex gap-2">
          {(["24h", "7d", "30d"] as const).map((range) => (
            <button
              key={range}
              onClick={() => setDateRange(range)}
              className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                dateRange === range
                  ? "bg-primary-500 text-white"
                  : "bg-white text-neutral-700 hover:bg-neutral-100 border border-neutral-200"
              }`}
            >
              Last {range}
            </button>
          ))}
        </div>
      </div>

      {/* Trends Charts */}
      {!trendsLoading && (trends.length > 0 || severityDist.length > 0) && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Anomaly Frequency Chart */}
          {trends.length > 0 && (
            <div className="bg-white rounded-lg border border-neutral-200 p-6">
              <h3 className="text-lg font-semibold text-neutral-900 mb-4">Anomaly Frequency</h3>
              <ResponsiveContainer width="100%" height={250}>
                <LineChart data={trends}>
                  <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
                  <XAxis
                    dataKey="date"
                    tickFormatter={(date) => new Date(date).toLocaleDateString("en-US", { month: "short", day: "numeric" })}
                    stroke="#6b7280"
                  />
                  <YAxis stroke="#6b7280" />
                  <Tooltip
                    labelFormatter={(date) => new Date(date).toLocaleDateString()}
                    formatter={(value: number) => [value, "Anomalies"]}
                  />
                  <Legend />
                  <Line
                    type="monotone"
                    dataKey="count"
                    stroke="#8b5cf6"
                    strokeWidth={2}
                    name="Anomalies"
                    dot={{ fill: "#8b5cf6", r: 4 }}
                  />
                </LineChart>
              </ResponsiveContainer>
            </div>
          )}

          {/* Severity Distribution Chart */}
          {severityDist.length > 0 && (
            <div className="bg-white rounded-lg border border-neutral-200 p-6">
              <h3 className="text-lg font-semibold text-neutral-900 mb-4">Severity Distribution</h3>
              <ResponsiveContainer width="100%" height={250}>
                <BarChart data={severityDist}>
                  <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
                  <XAxis dataKey="severity" stroke="#6b7280" />
                  <YAxis stroke="#6b7280" />
                  <Tooltip formatter={(value: number) => [value, "Count"]} />
                  <Legend />
                  <Bar dataKey="count" fill="#8b5cf6" name="Count" />
                </BarChart>
              </ResponsiveContainer>
            </div>
          )}
        </div>
      )}

      {/* Filters */}
      <div className="flex gap-4 items-center">
        <div>
          <label className="block text-xs text-neutral-500 mb-1">Type</label>
          <select
            value={typeFilter}
            onChange={(e) => setTypeFilter(e.target.value)}
            className="px-3 py-2 border border-neutral-200 rounded-lg text-sm bg-white"
          >
            <option value="all">All Types</option>
            <option value="spike_in_failures">Spike in Failures</option>
            <option value="performance_degradation">Performance Degradation</option>
            <option value="unusual_execution_time">Unusual Execution Time</option>
            <option value="pattern_deviation">Pattern Deviation</option>
            <option value="resource_exhaustion">Resource Exhaustion</option>
            <option value="consecutive_failures">Consecutive Failures</option>
          </select>
        </div>
        <div>
          <label className="block text-xs text-neutral-500 mb-1">Severity</label>
          <select
            value={severityFilter}
            onChange={(e) => setSeverityFilter(e.target.value)}
            className="px-3 py-2 border border-neutral-200 rounded-lg text-sm bg-white"
          >
            <option value="all">All Severities</option>
            <option value="critical">Critical</option>
            <option value="warning">Warning</option>
            <option value="info">Info</option>
          </select>
        </div>
        <div className="ml-auto text-sm text-neutral-500">
          {anomalies.length} anomaly{anomalies.length !== 1 ? "ies" : ""} found
        </div>
      </div>

      {/* Anomaly List */}
      {anomalies.length === 0 ? (
        <div className="bg-white rounded-lg border border-neutral-200 p-12 text-center">
          <AlertIcon className="w-12 h-12 mx-auto mb-4 text-neutral-300" />
          <h3 className="text-lg font-medium text-neutral-700 mb-2">No anomalies detected</h3>
          <p className="text-sm text-neutral-500">
            Anomalies will appear here as workflows are completed and analyzed.
          </p>
        </div>
      ) : (
        <div className="space-y-4">
          {anomalies.map((anomaly) => (
            <div
              key={anomaly.id}
              className="bg-white rounded-lg border border-neutral-200 p-6 hover:shadow-md transition-shadow cursor-pointer"
              onClick={() => setSelectedAnomaly(anomaly)}
            >
              <div className="flex items-start justify-between gap-4">
                <div className="flex-1">
                  {/* Header */}
                  <div className="flex items-center gap-3 mb-2">
                    <span
                      className={`px-2 py-1 text-xs font-medium rounded border ${severityColors[anomaly.severity]}`}
                    >
                      {severityIcons[anomaly.severity]} {anomaly.severity.toUpperCase()}
                    </span>
                    <span className="text-xs text-neutral-500 uppercase tracking-wide">
                      {typeLabels[anomaly.anomalyType] || anomaly.anomalyType}
                    </span>
                  </div>

                  {/* Description */}
                  <h3 className="text-lg font-medium text-neutral-900 mb-2">{anomaly.description}</h3>

                  {/* Metrics */}
                  <div className="flex flex-wrap gap-4 text-sm mb-3">
                    <div className="flex items-center gap-1 text-neutral-600">
                      <StatsIcon className="w-4 h-4" />
                      <span>Z-score: {anomaly.metrics.zScore.toFixed(2)}</span>
                    </div>
                    <div className="flex items-center gap-1 text-neutral-600">
                      <TargetIcon className="w-4 h-4" />
                      <span>Confidence: {(anomaly.metrics.confidence * 100).toFixed(0)}%</span>
                    </div>
                    <div className="flex items-center gap-1 text-neutral-600">
                      <TrendIcon className="w-4 h-4" />
                      <span>Deviation: {anomaly.metrics.deviation.toFixed(1)}</span>
                    </div>
                  </div>

                  {/* Affected Entities */}
                  {anomaly.affectedEntities.length > 0 && (
                    <div className="mt-3 flex flex-wrap gap-1">
                      {anomaly.affectedEntities.slice(0, 5).map((entity) => (
                        <span
                          key={entity}
                          className="px-2 py-0.5 text-xs font-mono bg-neutral-100 rounded border border-neutral-200"
                        >
                          {entity}
                        </span>
                      ))}
                      {anomaly.affectedEntities.length > 5 && (
                        <span className="text-xs text-neutral-500">
                          +{anomaly.affectedEntities.length - 5} more
                        </span>
                      )}
                    </div>
                  )}
                </div>

                {/* Timestamp */}
                <div className="text-right text-sm text-neutral-400 whitespace-nowrap">
                  {formatDistanceToNow(new Date(anomaly.detectedAt), { addSuffix: true })}
                </div>
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Anomaly Detail Modal */}
      {selectedAnomaly && (
        <div
          className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4"
          onClick={() => setSelectedAnomaly(null)}
        >
          <div
            className="bg-white rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="p-6 border-b border-neutral-200 flex items-center justify-between">
              <h2 className="text-xl font-bold text-neutral-900">Anomaly Details</h2>
              <button
                onClick={() => setSelectedAnomaly(null)}
                className="text-neutral-400 hover:text-neutral-600"
              >
                <CloseIcon className="w-6 h-6" />
              </button>
            </div>

            <div className="p-6 space-y-6">
              {/* Basic Info */}
              <div>
                <div className="flex items-center gap-3 mb-4">
                  <span
                    className={`px-3 py-1 text-sm font-medium rounded border ${severityColors[selectedAnomaly.severity]}`}
                  >
                    {severityIcons[selectedAnomaly.severity]} {selectedAnomaly.severity.toUpperCase()}
                  </span>
                  <span className="text-sm text-neutral-500 uppercase tracking-wide">
                    {typeLabels[selectedAnomaly.anomalyType] || selectedAnomaly.anomalyType}
                  </span>
                </div>
                <h3 className="text-lg font-medium text-neutral-900 mb-2">{selectedAnomaly.description}</h3>
                <p className="text-sm text-neutral-500">
                  Detected {formatDistanceToNow(new Date(selectedAnomaly.detectedAt), { addSuffix: true })}
                </p>
              </div>

              {/* Metrics */}
              <div>
                <h4 className="text-sm font-semibold text-neutral-700 mb-3">Statistical Metrics</h4>
                <div className="grid grid-cols-2 gap-4">
                  <div className="bg-neutral-50 rounded-lg p-4">
                    <div className="text-xs text-neutral-500 mb-1">Current Value</div>
                    <div className="text-lg font-semibold text-neutral-900">
                      {selectedAnomaly.metrics.currentValue.toFixed(2)}
                    </div>
                  </div>
                  <div className="bg-neutral-50 rounded-lg p-4">
                    <div className="text-xs text-neutral-500 mb-1">Baseline Value</div>
                    <div className="text-lg font-semibold text-neutral-900">
                      {selectedAnomaly.metrics.baselineValue.toFixed(2)}
                    </div>
                  </div>
                  <div className="bg-neutral-50 rounded-lg p-4">
                    <div className="text-xs text-neutral-500 mb-1">Z-Score</div>
                    <div className="text-lg font-semibold text-neutral-900">
                      {selectedAnomaly.metrics.zScore.toFixed(2)}
                    </div>
                  </div>
                  <div className="bg-neutral-50 rounded-lg p-4">
                    <div className="text-xs text-neutral-500 mb-1">Confidence</div>
                    <div className="text-lg font-semibold text-neutral-900">
                      {(selectedAnomaly.metrics.confidence * 100).toFixed(0)}%
                    </div>
                  </div>
                </div>
              </div>

              {/* Affected Entities */}
              {selectedAnomaly.affectedEntities.length > 0 && (
                <div>
                  <h4 className="text-sm font-semibold text-neutral-700 mb-3">Affected Entities</h4>
                  <div className="flex flex-wrap gap-2">
                    {selectedAnomaly.affectedEntities.map((entity) => (
                      <span
                        key={entity}
                        className="px-3 py-1 text-sm font-mono bg-neutral-100 rounded border border-neutral-200"
                      >
                        {entity}
                      </span>
                    ))}
                  </div>
                </div>
              )}

              {/* Investigation Steps */}
              {selectedAnomaly.investigationSteps.length > 0 && (
                <div>
                  <h4 className="text-sm font-semibold text-neutral-700 mb-3">Investigation Steps</h4>
                  <ol className="list-decimal list-inside space-y-2 text-sm text-neutral-600">
                    {selectedAnomaly.investigationSteps.map((step, i) => (
                      <li key={i}>{step}</li>
                    ))}
                  </ol>
                </div>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

// Icons
function AlertIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z"
      />
    </svg>
  );
}

function StatsIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z"
      />
    </svg>
  );
}

function TargetIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M7.5 14.25v2.25m3-4.5v4.5m3-6.75v6.75m3-9v9M6 20.25h12A2.25 2.25 0 0020.25 18V6A2.25 2.25 0 0018 3.75H6A2.25 2.25 0 003.75 6v12A2.25 2.25 0 006 20.25z"
      />
    </svg>
  );
}

function TrendIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M2.25 18L9 11.25l4.306 4.307a11.95 11.95 0 015.814-5.519l2.74-1.22m0 0l-5.94-2.28m5.94 2.28l-2.28 5.94"
      />
    </svg>
  );
}

function CloseIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
    </svg>
  );
}
