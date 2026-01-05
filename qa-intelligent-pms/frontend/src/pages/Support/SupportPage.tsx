import { useLayoutStore } from "@/stores/layoutStore";
import { useEffect, useState } from "react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { toastError, toastSuccess } from "@/hooks/useToast";

// Types
interface ErrorLog {
  id: string;
  message: string;
  stackTrace?: string;
  severity: "low" | "medium" | "high" | "critical";
  source: "frontend" | "backend" | "integration" | "database" | "unknown";
  status: "new" | "investigating" | "resolved" | "dismissed";
  userId?: string;
  pageUrl?: string;
  action?: string;
  occurrenceCount: number;
  firstSeenAt: string;
  lastSeenAt: string;
  resolutionNotes?: string;
  kbEntryId?: string;
}

interface KnowledgeBaseEntry {
  id: string;
  title: string;
  problem: string;
  cause: string;
  solution: string;
  relatedErrors: string[];
  tags: string[];
  viewCount: number;
  helpfulCount: number;
  notHelpfulCount: number;
  createdAt: string;
}

interface DiagnosticResult {
  integration: string;
  passed: boolean;
  message: string;
  latencyMs?: number;
  recentErrorCount: number;
  suggestions: string[];
  checkedAt: string;
}

interface DiagnosticsReport {
  overallHealthy: boolean;
  results: DiagnosticResult[];
  summary: string;
  generatedAt: string;
}

interface DashboardSummary {
  totalErrors: number;
  newErrors: number;
  investigating: number;
  resolved: number;
  criticalCount: number;
  highCount: number;
  bySource: { source: string; count: number }[];
  topErrors: { id: string; message: string; occurrenceCount: number; severity: string }[];
}

interface TroubleshootingSuggestion {
  id: string;
  source: "knowledge_base" | "similar_issues" | "diagnostic_step";
  title: string;
  description: string;
  relevanceScore: number;
  kbEntryId?: string;
}

// API functions
const API_BASE = "/api/v1/support";

async function fetchDashboard(): Promise<DashboardSummary> {
  const res = await fetch(`${API_BASE}/dashboard`);
  if (!res.ok) throw new Error("Failed to fetch dashboard");
  return res.json();
}

async function fetchErrorLogs(params: { page?: number; status?: string; severity?: string }): Promise<{
  items: ErrorLog[];
  total: number;
  page: number;
  perPage: number;
  totalPages: number;
}> {
  const searchParams = new URLSearchParams();
  if (params.page) searchParams.set("page", String(params.page));
  if (params.status) searchParams.set("status", params.status);
  if (params.severity) searchParams.set("severity", params.severity);
  
  const res = await fetch(`${API_BASE}/errors?${searchParams}`);
  if (!res.ok) throw new Error("Failed to fetch errors");
  return res.json();
}

async function fetchDiagnostics(): Promise<DiagnosticsReport> {
  const res = await fetch(`${API_BASE}/diagnostics`);
  if (!res.ok) throw new Error("Failed to fetch diagnostics");
  return res.json();
}

async function fetchKbEntries(search?: string): Promise<{
  items: KnowledgeBaseEntry[];
  total: number;
}> {
  const params = search ? `?search=${encodeURIComponent(search)}` : "";
  const res = await fetch(`${API_BASE}/kb${params}`);
  if (!res.ok) throw new Error("Failed to fetch KB entries");
  return res.json();
}

async function fetchSuggestions(errorId: string): Promise<{ suggestions: TroubleshootingSuggestion[] }> {
  const res = await fetch(`${API_BASE}/errors/${errorId}/suggestions`);
  if (!res.ok) throw new Error("Failed to fetch suggestions");
  return res.json();
}

async function updateErrorStatus(id: string, data: { status: string; resolutionNotes?: string }): Promise<ErrorLog> {
  const res = await fetch(`${API_BASE}/errors/${id}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(data),
  });
  if (!res.ok) throw new Error("Failed to update error");
  return res.json();
}

// Components
function SeverityBadge({ severity }: { severity: string }) {
  const colors: Record<string, string> = {
    critical: "bg-red-100 text-red-800 border-red-200",
    high: "bg-orange-100 text-orange-800 border-orange-200",
    medium: "bg-yellow-100 text-yellow-800 border-yellow-200",
    low: "bg-green-100 text-green-800 border-green-200",
  };
  return (
    <span className={`px-2 py-0.5 text-xs font-medium rounded border ${colors[severity] || colors.medium}`}>
      {severity.toUpperCase()}
    </span>
  );
}

function StatusBadge({ status }: { status: string }) {
  const colors: Record<string, string> = {
    new: "bg-blue-100 text-blue-800",
    investigating: "bg-purple-100 text-purple-800",
    resolved: "bg-green-100 text-green-800",
    dismissed: "bg-neutral-100 text-neutral-600",
  };
  return (
    <span className={`px-2 py-0.5 text-xs font-medium rounded ${colors[status] || colors.new}`}>
      {status.replace("_", " ").toUpperCase()}
    </span>
  );
}

function SourceBadge({ source }: { source: string }) {
  const colors: Record<string, string> = {
    frontend: "bg-cyan-100 text-cyan-800",
    backend: "bg-indigo-100 text-indigo-800",
    integration: "bg-pink-100 text-pink-800",
    database: "bg-amber-100 text-amber-800",
    unknown: "bg-neutral-100 text-neutral-600",
  };
  return (
    <span className={`px-2 py-0.5 text-xs font-medium rounded ${colors[source] || colors.unknown}`}>
      {source}
    </span>
  );
}

function DiagnosticCard({ result }: { result: DiagnosticResult }) {
  return (
    <div className={`p-4 rounded-lg border ${result.passed ? "bg-green-50 border-green-200" : "bg-red-50 border-red-200"}`}>
      <div className="flex items-center justify-between mb-2">
        <h4 className="font-medium text-neutral-900">{result.integration}</h4>
        <span className={`w-3 h-3 rounded-full ${result.passed ? "bg-green-500" : "bg-red-500"}`} />
      </div>
      <p className="text-sm text-neutral-600 mb-2">{result.message}</p>
      {result.latencyMs !== undefined && (
        <p className="text-xs text-neutral-500">Latency: {result.latencyMs}ms</p>
      )}
      {result.recentErrorCount > 0 && (
        <p className="text-xs text-red-600">Recent errors: {result.recentErrorCount}</p>
      )}
      {result.suggestions.length > 0 && (
        <div className="mt-2 pt-2 border-t border-red-200">
          <p className="text-xs font-medium text-red-700 mb-1">Suggestions:</p>
          <ul className="text-xs text-red-600 list-disc list-inside">
            {result.suggestions.map((s, i) => (
              <li key={i}>{s}</li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}

// Main Page Component
export function SupportPage() {
  const { setPageTitle } = useLayoutStore();
  const queryClient = useQueryClient();
  const [activeTab, setActiveTab] = useState<"dashboard" | "errors" | "kb" | "diagnostics">("dashboard");
  const [selectedError, setSelectedError] = useState<ErrorLog | null>(null);
  const [statusFilter, setStatusFilter] = useState<string>("");
  const [severityFilter, setSeverityFilter] = useState<string>("");
  const [kbSearch, setKbSearch] = useState("");

  useEffect(() => {
    setPageTitle("Support Portal", "Diagnose and resolve issues");
  }, [setPageTitle]);

  // Queries
  const dashboardQuery = useQuery({
    queryKey: ["support-dashboard"],
    queryFn: fetchDashboard,
  });

  const errorsQuery = useQuery({
    queryKey: ["support-errors", statusFilter, severityFilter],
    queryFn: () => fetchErrorLogs({ status: statusFilter || undefined, severity: severityFilter || undefined }),
    enabled: activeTab === "errors" || activeTab === "dashboard",
  });

  const diagnosticsQuery = useQuery({
    queryKey: ["support-diagnostics"],
    queryFn: fetchDiagnostics,
    enabled: activeTab === "diagnostics",
  });

  const kbQuery = useQuery({
    queryKey: ["support-kb", kbSearch],
    queryFn: () => fetchKbEntries(kbSearch || undefined),
    enabled: activeTab === "kb",
  });

  const suggestionsQuery = useQuery({
    queryKey: ["support-suggestions", selectedError?.id],
    queryFn: () => selectedError ? fetchSuggestions(selectedError.id) : Promise.resolve({ suggestions: [] }),
    enabled: !!selectedError,
  });

  // Mutations
  const updateStatusMutation = useMutation({
    mutationFn: ({ id, data }: { id: string; data: { status: string; resolutionNotes?: string } }) =>
      updateErrorStatus(id, data),
    onSuccess: () => {
      toastSuccess("Status updated successfully");
      queryClient.invalidateQueries({ queryKey: ["support-errors"] });
      queryClient.invalidateQueries({ queryKey: ["support-dashboard"] });
      setSelectedError(null);
    },
    onError: () => {
      toastError("Failed to update status");
    },
  });

  const dashboard = dashboardQuery.data;

  return (
    <div className="p-6 space-y-6">
      {/* Tab Navigation */}
      <div className="flex gap-2 border-b border-neutral-200 pb-2">
        {[
          { id: "dashboard", label: "Dashboard" },
          { id: "errors", label: "Error Logs" },
          { id: "kb", label: "Knowledge Base" },
          { id: "diagnostics", label: "Diagnostics" },
        ].map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id as typeof activeTab)}
            className={`px-4 py-2 text-sm font-medium rounded-t-lg transition-colors ${
              activeTab === tab.id
                ? "bg-primary-100 text-primary-700 border-b-2 border-primary-500"
                : "text-neutral-600 hover:text-neutral-900 hover:bg-neutral-50"
            }`}
          >
            {tab.label}
          </button>
        ))}
      </div>

      {/* Dashboard Tab */}
      {activeTab === "dashboard" && dashboard && (
        <div className="space-y-6">
          {/* Summary Cards */}
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div className="bg-white p-4 rounded-lg border border-neutral-200 shadow-sm">
              <p className="text-sm text-neutral-500">Total Errors</p>
              <p className="text-2xl font-bold text-neutral-900">{dashboard.totalErrors}</p>
            </div>
            <div className="bg-blue-50 p-4 rounded-lg border border-blue-200">
              <p className="text-sm text-blue-600">New</p>
              <p className="text-2xl font-bold text-blue-700">{dashboard.newErrors}</p>
            </div>
            <div className="bg-purple-50 p-4 rounded-lg border border-purple-200">
              <p className="text-sm text-purple-600">Investigating</p>
              <p className="text-2xl font-bold text-purple-700">{dashboard.investigating}</p>
            </div>
            <div className="bg-green-50 p-4 rounded-lg border border-green-200">
              <p className="text-sm text-green-600">Resolved</p>
              <p className="text-2xl font-bold text-green-700">{dashboard.resolved}</p>
            </div>
          </div>

          {/* Critical/High Alerts */}
          {(dashboard.criticalCount > 0 || dashboard.highCount > 0) && (
            <div className="bg-red-50 border border-red-200 rounded-lg p-4">
              <h3 className="text-sm font-semibold text-red-800 mb-2">⚠️ Attention Required</h3>
              <div className="flex gap-4 text-sm">
                {dashboard.criticalCount > 0 && (
                  <span className="text-red-700">{dashboard.criticalCount} critical errors</span>
                )}
                {dashboard.highCount > 0 && (
                  <span className="text-orange-700">{dashboard.highCount} high severity errors</span>
                )}
              </div>
            </div>
          )}

          {/* Top Errors */}
          <div className="bg-white rounded-lg border border-neutral-200 shadow-sm">
            <div className="p-4 border-b border-neutral-200">
              <h3 className="font-semibold text-neutral-900">Top Recurring Errors</h3>
            </div>
            <div className="divide-y divide-neutral-100">
              {dashboard.topErrors.length === 0 ? (
                <p className="p-4 text-sm text-neutral-500">No errors recorded</p>
              ) : (
                dashboard.topErrors.map((error) => (
                  <div key={error.id} className="p-4 flex items-center justify-between">
                    <div className="flex-1 min-w-0">
                      <p className="text-sm text-neutral-900 truncate">{error.message}</p>
                    </div>
                    <div className="flex items-center gap-3 ml-4">
                      <SeverityBadge severity={error.severity} />
                      <span className="text-sm text-neutral-500">{error.occurrenceCount}x</span>
                    </div>
                  </div>
                ))
              )}
            </div>
          </div>

          {/* Errors by Source */}
          <div className="bg-white rounded-lg border border-neutral-200 shadow-sm p-4">
            <h3 className="font-semibold text-neutral-900 mb-4">Errors by Source</h3>
            <div className="flex flex-wrap gap-4">
              {dashboard.bySource.map((item) => (
                <div key={item.source} className="flex items-center gap-2">
                  <SourceBadge source={item.source} />
                  <span className="text-sm text-neutral-600">{item.count}</span>
                </div>
              ))}
            </div>
          </div>
        </div>
      )}

      {/* Errors Tab */}
      {activeTab === "errors" && (
        <div className="space-y-4">
          {/* Filters */}
          <div className="flex gap-4 items-center">
            <select
              value={statusFilter}
              onChange={(e) => setStatusFilter(e.target.value)}
              className="px-3 py-2 border border-neutral-300 rounded-lg text-sm"
            >
              <option value="">All Status</option>
              <option value="new">New</option>
              <option value="investigating">Investigating</option>
              <option value="resolved">Resolved</option>
              <option value="dismissed">Dismissed</option>
            </select>
            <select
              value={severityFilter}
              onChange={(e) => setSeverityFilter(e.target.value)}
              className="px-3 py-2 border border-neutral-300 rounded-lg text-sm"
            >
              <option value="">All Severity</option>
              <option value="critical">Critical</option>
              <option value="high">High</option>
              <option value="medium">Medium</option>
              <option value="low">Low</option>
            </select>
          </div>

          {/* Error List */}
          <div className="bg-white rounded-lg border border-neutral-200 shadow-sm divide-y divide-neutral-100">
            {errorsQuery.isLoading && (
              <div className="p-8 text-center text-neutral-500">Loading...</div>
            )}
            {errorsQuery.data?.items.length === 0 && (
              <div className="p-8 text-center text-neutral-500">No errors found</div>
            )}
            {errorsQuery.data?.items.map((error) => (
              <div
                key={error.id}
                onClick={() => setSelectedError(error)}
                className="p-4 cursor-pointer hover:bg-neutral-50 transition-colors"
              >
                <div className="flex items-start justify-between gap-4">
                  <div className="flex-1 min-w-0">
                    <p className="text-sm font-medium text-neutral-900 truncate">{error.message}</p>
                    <div className="flex items-center gap-2 mt-1">
                      <SourceBadge source={error.source} />
                      <span className="text-xs text-neutral-500">
                        {new Date(error.lastSeenAt).toLocaleString()}
                      </span>
                      {error.occurrenceCount > 1 && (
                        <span className="text-xs text-neutral-500">({error.occurrenceCount}x)</span>
                      )}
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <SeverityBadge severity={error.severity} />
                    <StatusBadge status={error.status} />
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Knowledge Base Tab */}
      {activeTab === "kb" && (
        <div className="space-y-4">
          <div className="flex gap-4">
            <input
              type="text"
              placeholder="Search knowledge base..."
              value={kbSearch}
              onChange={(e) => setKbSearch(e.target.value)}
              className="flex-1 px-4 py-2 border border-neutral-300 rounded-lg text-sm"
            />
          </div>

          <div className="grid gap-4 md:grid-cols-2">
            {kbQuery.isLoading && (
              <div className="col-span-2 p-8 text-center text-neutral-500">Loading...</div>
            )}
            {kbQuery.data?.items.length === 0 && (
              <div className="col-span-2 p-8 text-center text-neutral-500">No entries found</div>
            )}
            {kbQuery.data?.items.map((entry) => (
              <div key={entry.id} className="bg-white rounded-lg border border-neutral-200 shadow-sm p-4">
                <h4 className="font-medium text-neutral-900 mb-2">{entry.title}</h4>
                <p className="text-sm text-neutral-600 mb-3 line-clamp-2">{entry.problem}</p>
                <div className="flex items-center justify-between text-xs text-neutral-500">
                  <div className="flex gap-2">
                    {entry.tags.slice(0, 3).map((tag) => (
                      <span key={tag} className="px-2 py-0.5 bg-neutral-100 rounded">{tag}</span>
                    ))}
                  </div>
                  <div className="flex items-center gap-2">
                    <span>👁 {entry.viewCount}</span>
                    <span>👍 {entry.helpfulCount}</span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Diagnostics Tab */}
      {activeTab === "diagnostics" && (
        <div className="space-y-4">
          {diagnosticsQuery.isLoading && (
            <div className="p-8 text-center text-neutral-500">Running diagnostics...</div>
          )}
          {diagnosticsQuery.data && (
            <>
              <div className={`p-4 rounded-lg ${
                diagnosticsQuery.data.overallHealthy
                  ? "bg-green-50 border border-green-200"
                  : "bg-red-50 border border-red-200"
              }`}>
                <div className="flex items-center gap-2">
                  <span className={`text-2xl ${diagnosticsQuery.data.overallHealthy ? "" : "animate-pulse"}`}>
                    {diagnosticsQuery.data.overallHealthy ? "✅" : "⚠️"}
                  </span>
                  <div>
                    <h3 className="font-semibold text-neutral-900">{diagnosticsQuery.data.summary}</h3>
                    <p className="text-xs text-neutral-500">
                      Last checked: {new Date(diagnosticsQuery.data.generatedAt).toLocaleString()}
                    </p>
                  </div>
                </div>
              </div>

              <div className="grid gap-4 md:grid-cols-2">
                {diagnosticsQuery.data.results.map((result) => (
                  <DiagnosticCard key={result.integration} result={result} />
                ))}
              </div>

              <button
                onClick={() => diagnosticsQuery.refetch()}
                className="px-4 py-2 bg-primary-600 text-white rounded-lg text-sm hover:bg-primary-700 transition-colors"
              >
                Run Diagnostics Again
              </button>
            </>
          )}
        </div>
      )}

      {/* Error Detail Modal */}
      {selectedError && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
            <div className="p-4 border-b border-neutral-200 flex items-center justify-between">
              <h3 className="font-semibold text-neutral-900">Error Details</h3>
              <button
                onClick={() => setSelectedError(null)}
                className="text-neutral-500 hover:text-neutral-700"
              >
                ✕
              </button>
            </div>
            <div className="p-4 space-y-4">
              <div>
                <p className="text-xs text-neutral-500 mb-1">Message</p>
                <p className="text-sm text-neutral-900 font-mono bg-neutral-50 p-2 rounded">
                  {selectedError.message}
                </p>
              </div>

              <div className="grid grid-cols-3 gap-4">
                <div>
                  <p className="text-xs text-neutral-500 mb-1">Severity</p>
                  <SeverityBadge severity={selectedError.severity} />
                </div>
                <div>
                  <p className="text-xs text-neutral-500 mb-1">Source</p>
                  <SourceBadge source={selectedError.source} />
                </div>
                <div>
                  <p className="text-xs text-neutral-500 mb-1">Status</p>
                  <StatusBadge status={selectedError.status} />
                </div>
              </div>

              <div className="grid grid-cols-2 gap-4 text-sm">
                <div>
                  <p className="text-xs text-neutral-500">First Seen</p>
                  <p>{new Date(selectedError.firstSeenAt).toLocaleString()}</p>
                </div>
                <div>
                  <p className="text-xs text-neutral-500">Last Seen</p>
                  <p>{new Date(selectedError.lastSeenAt).toLocaleString()}</p>
                </div>
                <div>
                  <p className="text-xs text-neutral-500">Occurrences</p>
                  <p>{selectedError.occurrenceCount}</p>
                </div>
                {selectedError.pageUrl && (
                  <div>
                    <p className="text-xs text-neutral-500">Page</p>
                    <p className="truncate">{selectedError.pageUrl}</p>
                  </div>
                )}
              </div>

              {selectedError.stackTrace && (
                <div>
                  <p className="text-xs text-neutral-500 mb-1">Stack Trace</p>
                  <pre className="text-xs text-neutral-700 bg-neutral-50 p-2 rounded overflow-x-auto max-h-40">
                    {selectedError.stackTrace}
                  </pre>
                </div>
              )}

              {/* Suggestions */}
              <div>
                <p className="text-xs text-neutral-500 mb-2">Troubleshooting Suggestions</p>
                {suggestionsQuery.isLoading && (
                  <p className="text-sm text-neutral-500">Loading suggestions...</p>
                )}
                {suggestionsQuery.data?.suggestions.length === 0 && (
                  <p className="text-sm text-neutral-500">No suggestions available</p>
                )}
                <div className="space-y-2">
                  {suggestionsQuery.data?.suggestions.map((suggestion) => (
                    <div key={suggestion.id} className="bg-blue-50 border border-blue-200 rounded p-3">
                      <div className="flex items-center gap-2 mb-1">
                        <span className="text-xs px-2 py-0.5 bg-blue-100 text-blue-700 rounded">
                          {suggestion.source.replace("_", " ")}
                        </span>
                        <span className="text-xs text-neutral-500">
                          {suggestion.relevanceScore}% match
                        </span>
                      </div>
                      <h5 className="text-sm font-medium text-neutral-900">{suggestion.title}</h5>
                      <p className="text-xs text-neutral-600 mt-1 whitespace-pre-line">
                        {suggestion.description}
                      </p>
                    </div>
                  ))}
                </div>
              </div>

              {/* Actions */}
              <div className="flex gap-2 pt-4 border-t border-neutral-200">
                {selectedError.status === "new" && (
                  <button
                    onClick={() => updateStatusMutation.mutate({
                      id: selectedError.id,
                      data: { status: "investigating" }
                    })}
                    className="px-4 py-2 bg-purple-600 text-white rounded text-sm hover:bg-purple-700"
                  >
                    Start Investigating
                  </button>
                )}
                {selectedError.status === "investigating" && (
                  <button
                    onClick={() => updateStatusMutation.mutate({
                      id: selectedError.id,
                      data: { status: "resolved" }
                    })}
                    className="px-4 py-2 bg-green-600 text-white rounded text-sm hover:bg-green-700"
                  >
                    Mark Resolved
                  </button>
                )}
                {selectedError.status !== "dismissed" && (
                  <button
                    onClick={() => updateStatusMutation.mutate({
                      id: selectedError.id,
                      data: { status: "dismissed" }
                    })}
                    className="px-4 py-2 bg-neutral-200 text-neutral-700 rounded text-sm hover:bg-neutral-300"
                  >
                    Dismiss
                  </button>
                )}
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
