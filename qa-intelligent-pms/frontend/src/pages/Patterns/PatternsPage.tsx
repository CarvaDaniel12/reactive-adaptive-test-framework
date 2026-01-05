/**
 * Pattern History Page.
 * Story 9.5: Displays history of detected patterns with filtering.
 */
import { useState, useEffect } from "react";
import { useQuery } from "@tanstack/react-query";
import { formatDistanceToNow } from "date-fns";
import { useLayoutStore } from "@/stores/layoutStore";

interface Pattern {
  id: string;
  patternType: "time_excess" | "consecutive_problem" | "spike";
  severity: "info" | "warning" | "critical";
  title: string;
  description: string | null;
  affectedTickets: string[];
  commonFactor: string | null;
  averageExcessPercent: number | null;
  confidenceScore: number;
  suggestedActions: string[];
  detectedAt: string;
}

interface PatternsResponse {
  patterns: Pattern[];
}

async function fetchPatterns(): Promise<PatternsResponse> {
  const response = await fetch("/api/v1/patterns");
  if (!response.ok) throw new Error("Failed to fetch patterns");
  return response.json();
}

export function PatternsPage() {
  const { setPageTitle } = useLayoutStore();
  const [typeFilter, setTypeFilter] = useState<string>("all");
  const [severityFilter, setSeverityFilter] = useState<string>("all");

  const { data, isLoading, error } = useQuery({
    queryKey: ["patterns"],
    queryFn: fetchPatterns,
    staleTime: 60_000,
  });

  useEffect(() => {
    setPageTitle("Pattern History", "Detected patterns and anomalies");
    return () => {
      setPageTitle("");
    };
  }, [setPageTitle]);

  const patterns = data?.patterns ?? [];
  
  const filteredPatterns = patterns.filter((p) => {
    if (typeFilter !== "all" && p.patternType !== typeFilter) return false;
    if (severityFilter !== "all" && p.severity !== severityFilter) return false;
    return true;
  });

  const typeLabels: Record<string, string> = {
    time_excess: "Time Excess",
    consecutive_problem: "Recurring Issue",
    spike: "Volume Spike",
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
          Failed to load patterns. Please try again.
        </div>
      </div>
    );
  }

  return (
    <div className="p-6 space-y-6">
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
            <option value="time_excess">Time Excess</option>
            <option value="consecutive_problem">Recurring Issue</option>
            <option value="spike">Volume Spike</option>
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
          {filteredPatterns.length} pattern{filteredPatterns.length !== 1 ? "s" : ""} found
        </div>
      </div>

      {/* Pattern List */}
      {filteredPatterns.length === 0 ? (
        <div className="bg-white rounded-lg border border-neutral-200 p-12 text-center">
          <ChartIcon className="w-12 h-12 mx-auto mb-4 text-neutral-300" />
          <h3 className="text-lg font-medium text-neutral-700 mb-2">No patterns detected</h3>
          <p className="text-sm text-neutral-500">
            Patterns will appear here as workflows are completed and analyzed.
          </p>
        </div>
      ) : (
        <div className="space-y-4">
          {filteredPatterns.map((pattern) => (
            <div
              key={pattern.id}
              className="bg-white rounded-lg border border-neutral-200 p-6 hover:shadow-md transition-shadow"
            >
              <div className="flex items-start justify-between gap-4">
                <div className="flex-1">
                  {/* Header */}
                  <div className="flex items-center gap-3 mb-2">
                    <span className={`px-2 py-1 text-xs font-medium rounded border ${severityColors[pattern.severity]}`}>
                      {severityIcons[pattern.severity]} {pattern.severity.toUpperCase()}
                    </span>
                    <span className="text-xs text-neutral-500 uppercase tracking-wide">
                      {typeLabels[pattern.patternType]}
                    </span>
                  </div>

                  {/* Title */}
                  <h3 className="text-lg font-medium text-neutral-900 mb-2">{pattern.title}</h3>

                  {/* Description */}
                  {pattern.description && (
                    <p className="text-sm text-neutral-600 mb-3">{pattern.description}</p>
                  )}

                  {/* Stats */}
                  <div className="flex flex-wrap gap-4 text-sm">
                    {pattern.averageExcessPercent !== null && (
                      <div className="flex items-center gap-1 text-neutral-600">
                        <ClockIcon className="w-4 h-4" />
                        <span>{pattern.averageExcessPercent.toFixed(0)}% excess</span>
                      </div>
                    )}
                    <div className="flex items-center gap-1 text-neutral-600">
                      <TargetIcon className="w-4 h-4" />
                      <span>{(pattern.confidenceScore * 100).toFixed(0)}% confidence</span>
                    </div>
                    {pattern.commonFactor && (
                      <div className="flex items-center gap-1 text-neutral-600">
                        <TagIcon className="w-4 h-4" />
                        <span>{pattern.commonFactor}</span>
                      </div>
                    )}
                  </div>

                  {/* Affected Tickets */}
                  {pattern.affectedTickets.length > 0 && (
                    <div className="mt-3 flex flex-wrap gap-1">
                      {pattern.affectedTickets.slice(0, 5).map((ticket) => (
                        <span
                          key={ticket}
                          className="px-2 py-0.5 text-xs font-mono bg-neutral-100 rounded border border-neutral-200"
                        >
                          {ticket}
                        </span>
                      ))}
                      {pattern.affectedTickets.length > 5 && (
                        <span className="text-xs text-neutral-500">
                          +{pattern.affectedTickets.length - 5} more
                        </span>
                      )}
                    </div>
                  )}

                  {/* Suggested Actions */}
                  {pattern.suggestedActions.length > 0 && (
                    <details className="mt-4">
                      <summary className="text-sm text-primary-600 cursor-pointer hover:underline">
                        Suggested actions ({pattern.suggestedActions.length})
                      </summary>
                      <ul className="mt-2 pl-4 text-sm text-neutral-600 list-disc space-y-1">
                        {pattern.suggestedActions.map((action, i) => (
                          <li key={i}>{action}</li>
                        ))}
                      </ul>
                    </details>
                  )}
                </div>

                {/* Timestamp */}
                <div className="text-right text-sm text-neutral-400 whitespace-nowrap">
                  {formatDistanceToNow(new Date(pattern.detectedAt), { addSuffix: true })}
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

// Icons
function ChartIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z" />
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

function TargetIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M7.5 14.25v2.25m3-4.5v4.5m3-6.75v6.75m3-9v9M6 20.25h12A2.25 2.25 0 0020.25 18V6A2.25 2.25 0 0018 3.75H6A2.25 2.25 0 003.75 6v12A2.25 2.25 0 006 20.25z" />
    </svg>
  );
}

function TagIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M9.568 3H5.25A2.25 2.25 0 003 5.25v4.318c0 .597.237 1.17.659 1.591l9.581 9.581c.699.699 1.78.872 2.607.33a18.095 18.095 0 005.223-5.223c.542-.827.369-1.908-.33-2.607L11.16 3.66A2.25 2.25 0 009.568 3z" />
      <path strokeLinecap="round" strokeLinejoin="round" d="M6 6h.008v.008H6V6z" />
    </svg>
  );
}
