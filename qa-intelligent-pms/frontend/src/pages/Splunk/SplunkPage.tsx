/**
 * Splunk Query Interface Page.
 * Epic 11: Manual query interface with templates and log display.
 */
import { useEffect, useState } from "react";
import { useQuery, useMutation } from "@tanstack/react-query";
import { useLayoutStore } from "@/stores/layoutStore";
import { useToast, type ToastVariant } from "@/hooks/useToast";

// Types
interface QueryTemplate {
  id: string;
  name: string;
  description: string | null;
  query: string;
  category: string;
  isSystem: boolean;
  placeholders: string[];
  createdAt: string;
  updatedAt: string;
}

interface PrepareQueryRequest {
  templateId?: string;
  rawQuery?: string;
  placeholders: Record<string, string>;
  timeStart?: string;
  timeEnd?: string;
  index?: string;
}

interface PrepareQueryResponse {
  query: string;
  timeStart: string;
  timeEnd: string;
  index: string | null;
  splunkUrl: string | null;
}

interface LogEntry {
  timestamp: string;
  level: string;
  message: string;
  source: string | null;
  host: string | null;
  fields: Record<string, unknown>;
}

interface ExecuteQueryResponse {
  query: string;
  entries: LogEntry[];
  totalCount: number;
  truncated: boolean;
  executionTimeMs: number;
  message: string;
}

interface PlaceholderInfo {
  key: string;
  label: string;
  description: string;
  example: string;
}

// API functions
async function fetchTemplates(): Promise<{ templates: QueryTemplate[]; total: number }> {
  const res = await fetch("/api/v1/splunk/templates");
  if (!res.ok) throw new Error("Failed to fetch templates");
  return res.json();
}

async function fetchPlaceholders(): Promise<{ placeholders: PlaceholderInfo[] }> {
  const res = await fetch("/api/v1/splunk/placeholders");
  if (!res.ok) throw new Error("Failed to fetch placeholders");
  return res.json();
}

async function prepareQuery(req: PrepareQueryRequest): Promise<PrepareQueryResponse> {
  const res = await fetch("/api/v1/splunk/query/prepare", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
  });
  if (!res.ok) {
    const err = await res.json();
    throw new Error(err.message || "Failed to prepare query");
  }
  return res.json();
}

async function executeQuery(req: {
  query: string;
  timeStart: string;
  timeEnd: string;
  index?: string;
  limit?: number;
}): Promise<ExecuteQueryResponse> {
  const res = await fetch("/api/v1/splunk/query/execute", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
  });
  if (!res.ok) throw new Error("Failed to execute query");
  return res.json();
}

export function SplunkPage() {
  const { setPageTitle } = useLayoutStore();
  const { toast } = useToast();

  // Helper to show toast
  const showToast = (title: string, variant: ToastVariant) => {
    toast({ title, variant });
  };

  // State
  const [selectedTemplate, setSelectedTemplate] = useState<QueryTemplate | null>(null);
  const [rawQuery, setRawQuery] = useState("");
  const [placeholderValues, setPlaceholderValues] = useState<Record<string, string>>({});
  const [timeRange, setTimeRange] = useState({ start: "-24h", end: "now" });
  const [indexName, setIndexName] = useState("");
  const [preparedQuery, setPreparedQuery] = useState<PrepareQueryResponse | null>(null);
  const [queryResults, setQueryResults] = useState<ExecuteQueryResponse | null>(null);
  const [activeTab, setActiveTab] = useState<"templates" | "custom">("templates");

  // Queries
  const { data: templatesData, isLoading: loadingTemplates } = useQuery({
    queryKey: ["splunk-templates"],
    queryFn: fetchTemplates,
  });

  const { data: placeholdersData } = useQuery({
    queryKey: ["splunk-placeholders"],
    queryFn: fetchPlaceholders,
  });

  // Mutations
  const prepareMutation = useMutation({
    mutationFn: prepareQuery,
    onSuccess: (data) => {
      setPreparedQuery(data);
      showToast("Query prepared successfully", "success");
    },
    onError: (err: Error) => {
      showToast(err.message, "error");
    },
  });

  const executeMutation = useMutation({
    mutationFn: executeQuery,
    onSuccess: (data) => {
      setQueryResults(data);
      showToast(`Found ${data.totalCount} results`, "success");
    },
    onError: () => {
      showToast("Failed to execute query", "error");
    },
  });

  useEffect(() => {
    setPageTitle("Splunk Logs", "Query production logs");
    return () => setPageTitle("");
  }, [setPageTitle]);

  // Get time range as ISO strings
  const getTimeRange = () => {
    const now = new Date();
    let start: Date;

    if (timeRange.start === "-1h") {
      start = new Date(now.getTime() - 60 * 60 * 1000);
    } else if (timeRange.start === "-24h") {
      start = new Date(now.getTime() - 24 * 60 * 60 * 1000);
    } else if (timeRange.start === "-7d") {
      start = new Date(now.getTime() - 7 * 24 * 60 * 60 * 1000);
    } else if (timeRange.start === "-30d") {
      start = new Date(now.getTime() - 30 * 24 * 60 * 60 * 1000);
    } else {
      start = new Date(now.getTime() - 24 * 60 * 60 * 1000);
    }

    return {
      timeStart: start.toISOString(),
      timeEnd: now.toISOString(),
    };
  };

  const handlePrepare = () => {
    const { timeStart, timeEnd } = getTimeRange();

    if (activeTab === "templates" && selectedTemplate) {
      prepareMutation.mutate({
        templateId: selectedTemplate.id,
        placeholders: placeholderValues,
        timeStart,
        timeEnd,
        index: indexName || undefined,
      });
    } else if (activeTab === "custom" && rawQuery.trim()) {
      prepareMutation.mutate({
        rawQuery: rawQuery.trim(),
        placeholders: {},
        timeStart,
        timeEnd,
        index: indexName || undefined,
      });
    } else {
      showToast("Please select a template or enter a query", "warning");
    }
  };

  const handleExecute = () => {
    if (!preparedQuery) return;

    executeMutation.mutate({
      query: preparedQuery.query,
      timeStart: preparedQuery.timeStart,
      timeEnd: preparedQuery.timeEnd,
      index: preparedQuery.index || undefined,
      limit: 100,
    });
  };

  const handleCopyQuery = () => {
    if (preparedQuery) {
      navigator.clipboard.writeText(preparedQuery.query);
      showToast("Query copied to clipboard", "success");
    }
  };

  // Group templates by category
  const templatesByCategory = templatesData?.templates.reduce(
    (acc, t) => {
      const cat = t.category;
      if (!acc[cat]) acc[cat] = [];
      acc[cat].push(t);
      return acc;
    },
    {} as Record<string, QueryTemplate[]>
  );

  return (
    <div className="p-6 space-y-6">
      {/* Tab Selector */}
      <div className="flex gap-2 p-1 bg-neutral-100 rounded-lg w-fit">
        <button
          onClick={() => setActiveTab("templates")}
          className={`px-4 py-2 text-sm rounded-md transition-colors ${
            activeTab === "templates"
              ? "bg-white text-neutral-900 shadow-sm"
              : "text-neutral-600 hover:text-neutral-900"
          }`}
        >
          Query Templates
        </button>
        <button
          onClick={() => setActiveTab("custom")}
          className={`px-4 py-2 text-sm rounded-md transition-colors ${
            activeTab === "custom"
              ? "bg-white text-neutral-900 shadow-sm"
              : "text-neutral-600 hover:text-neutral-900"
          }`}
        >
          Custom Query
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Left Panel: Templates or Custom Query */}
        <div className="lg:col-span-2 space-y-6">
          {activeTab === "templates" ? (
            <div className="bg-white rounded-xl border border-neutral-200 p-6">
              <h3 className="text-lg font-semibold text-neutral-900 mb-4 flex items-center gap-2">
                <TemplateIcon className="w-5 h-5 text-primary-600" />
                Select Template
              </h3>

              {loadingTemplates ? (
                <div className="space-y-3">
                  {[1, 2, 3].map((i) => (
                    <div key={i} className="h-16 bg-neutral-100 rounded-lg animate-pulse" />
                  ))}
                </div>
              ) : (
                <div className="space-y-4">
                  {templatesByCategory &&
                    Object.entries(templatesByCategory).map(([category, templates]) => (
                      <div key={category}>
                        <h4 className="text-xs font-medium text-neutral-500 uppercase tracking-wide mb-2">
                          {formatCategory(category)}
                        </h4>
                        <div className="space-y-2">
                          {templates.map((t) => (
                            <button
                              key={t.id}
                              onClick={() => {
                                setSelectedTemplate(t);
                                setPlaceholderValues({});
                              }}
                              className={`w-full text-left p-3 rounded-lg border transition-colors ${
                                selectedTemplate?.id === t.id
                                  ? "border-primary-500 bg-primary-50"
                                  : "border-neutral-200 hover:border-neutral-300"
                              }`}
                            >
                              <div className="flex items-center justify-between">
                                <span className="font-medium text-neutral-900">{t.name}</span>
                                {t.isSystem && (
                                  <span className="text-xs px-2 py-0.5 bg-neutral-100 text-neutral-600 rounded">
                                    System
                                  </span>
                                )}
                              </div>
                              {t.description && (
                                <p className="text-sm text-neutral-500 mt-1">{t.description}</p>
                              )}
                              {t.placeholders.length > 0 && (
                                <div className="flex gap-1 mt-2">
                                  {t.placeholders.map((p) => (
                                    <span
                                      key={p}
                                      className="text-xs px-1.5 py-0.5 bg-amber-100 text-amber-700 rounded font-mono"
                                    >
                                      {`{${p}}`}
                                    </span>
                                  ))}
                                </div>
                              )}
                            </button>
                          ))}
                        </div>
                      </div>
                    ))}
                </div>
              )}

              {/* Placeholder Inputs */}
              {selectedTemplate && selectedTemplate.placeholders.length > 0 && (
                <div className="mt-6 p-4 bg-neutral-50 rounded-lg">
                  <h4 className="text-sm font-medium text-neutral-700 mb-3">Fill Placeholders</h4>
                  <div className="space-y-3">
                    {selectedTemplate.placeholders.map((p) => {
                      const info = placeholdersData?.placeholders.find((ph) => ph.key === p);
                      return (
                        <div key={p}>
                          <label className="block text-sm text-neutral-600 mb-1">
                            {info?.label || p}
                          </label>
                          <input
                            type="text"
                            value={placeholderValues[p] || ""}
                            onChange={(e) =>
                              setPlaceholderValues((prev) => ({ ...prev, [p]: e.target.value }))
                            }
                            placeholder={info?.example || `Enter ${p}`}
                            className="w-full px-3 py-2 border border-neutral-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
                          />
                          {info?.description && (
                            <p className="text-xs text-neutral-400 mt-1">{info.description}</p>
                          )}
                        </div>
                      );
                    })}
                  </div>
                </div>
              )}
            </div>
          ) : (
            <div className="bg-white rounded-xl border border-neutral-200 p-6">
              <h3 className="text-lg font-semibold text-neutral-900 mb-4 flex items-center gap-2">
                <CodeIcon className="w-5 h-5 text-primary-600" />
                Custom SPL Query
              </h3>
              <textarea
                value={rawQuery}
                onChange={(e) => setRawQuery(e.target.value)}
                placeholder={`index=* level=ERROR earliest=-24h@h latest=now
| table _time, host, source, message
| sort -_time`}
                className="w-full h-48 px-4 py-3 font-mono text-sm border border-neutral-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
              />
              <p className="text-xs text-neutral-400 mt-2">
                Enter your SPL (Search Processing Language) query. Use placeholders like{" "}
                {`{TICKET_KEY}`} for dynamic values.
              </p>
            </div>
          )}

          {/* Query Preview */}
          {preparedQuery && (
            <div className="bg-white rounded-xl border border-neutral-200 p-6">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-lg font-semibold text-neutral-900 flex items-center gap-2">
                  <PreviewIcon className="w-5 h-5 text-emerald-600" />
                  Prepared Query
                </h3>
                <div className="flex gap-2">
                  <button
                    onClick={handleCopyQuery}
                    className="px-3 py-1.5 text-sm text-neutral-600 hover:text-neutral-900 border border-neutral-200 rounded-lg hover:bg-neutral-50 transition-colors"
                  >
                    Copy
                  </button>
                  <button
                    onClick={handleExecute}
                    disabled={executeMutation.isPending}
                    className="px-4 py-1.5 text-sm bg-emerald-600 text-white rounded-lg hover:bg-emerald-700 transition-colors disabled:opacity-50"
                  >
                    {executeMutation.isPending ? "Running..." : "Run Query"}
                  </button>
                </div>
              </div>
              <pre className="p-4 bg-neutral-900 text-neutral-100 rounded-lg overflow-x-auto text-sm font-mono">
                {preparedQuery.query}
              </pre>
              <div className="mt-3 flex gap-4 text-xs text-neutral-500">
                <span>
                  Time: {new Date(preparedQuery.timeStart).toLocaleString()} -{" "}
                  {new Date(preparedQuery.timeEnd).toLocaleString()}
                </span>
                {preparedQuery.index && <span>Index: {preparedQuery.index}</span>}
              </div>
            </div>
          )}

          {/* Query Results */}
          {queryResults && (
            <div className="bg-white rounded-xl border border-neutral-200 p-6">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-lg font-semibold text-neutral-900 flex items-center gap-2">
                  <ResultsIcon className="w-5 h-5 text-blue-600" />
                  Results ({queryResults.totalCount})
                </h3>
                <span className="text-xs text-neutral-500">
                  Executed in {queryResults.executionTimeMs}ms
                </span>
              </div>

              {/* Info banner */}
              <div className="mb-4 p-3 bg-blue-50 border-l-4 border-blue-500 text-blue-800 text-sm rounded">
                {queryResults.message}
              </div>

              {/* Log entries table */}
              {queryResults.entries.length > 0 ? (
                <div className="overflow-x-auto">
                  <table className="w-full">
                    <thead>
                      <tr className="text-left text-xs text-neutral-500 uppercase tracking-wide border-b border-neutral-100">
                        <th className="pb-3 pr-4">Time</th>
                        <th className="pb-3 pr-4">Level</th>
                        <th className="pb-3 pr-4">Host</th>
                        <th className="pb-3">Message</th>
                      </tr>
                    </thead>
                    <tbody className="divide-y divide-neutral-50">
                      {queryResults.entries.map((entry, i) => (
                        <tr key={i} className="hover:bg-neutral-50">
                          <td className="py-2 pr-4 text-xs text-neutral-500 whitespace-nowrap">
                            {new Date(entry.timestamp).toLocaleTimeString()}
                          </td>
                          <td className="py-2 pr-4">
                            <LevelBadge level={entry.level} />
                          </td>
                          <td className="py-2 pr-4 text-sm text-neutral-600 font-mono">
                            {entry.host || "—"}
                          </td>
                          <td className="py-2 text-sm text-neutral-900">{entry.message}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              ) : (
                <p className="text-center text-neutral-500 py-8">No results found</p>
              )}
            </div>
          )}
        </div>

        {/* Right Panel: Settings */}
        <div className="space-y-6">
          <div className="bg-white rounded-xl border border-neutral-200 p-6">
            <h3 className="text-lg font-semibold text-neutral-900 mb-4 flex items-center gap-2">
              <SettingsIcon className="w-5 h-5 text-neutral-600" />
              Query Settings
            </h3>

            <div className="space-y-4">
              {/* Time Range */}
              <div>
                <label className="block text-sm font-medium text-neutral-700 mb-2">
                  Time Range
                </label>
                <select
                  value={timeRange.start}
                  onChange={(e) => setTimeRange({ start: e.target.value, end: "now" })}
                  className="w-full px-3 py-2 border border-neutral-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
                >
                  <option value="-1h">Last 1 hour</option>
                  <option value="-24h">Last 24 hours</option>
                  <option value="-7d">Last 7 days</option>
                  <option value="-30d">Last 30 days</option>
                </select>
              </div>

              {/* Index */}
              <div>
                <label className="block text-sm font-medium text-neutral-700 mb-2">
                  Index (optional)
                </label>
                <input
                  type="text"
                  value={indexName}
                  onChange={(e) => setIndexName(e.target.value)}
                  placeholder="e.g., main, web_logs"
                  className="w-full px-3 py-2 border border-neutral-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
                />
              </div>

              {/* Prepare Button */}
              <button
                onClick={handlePrepare}
                disabled={prepareMutation.isPending}
                className="w-full py-2.5 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors disabled:opacity-50 font-medium"
              >
                {prepareMutation.isPending ? "Preparing..." : "Prepare Query"}
              </button>
            </div>
          </div>

          {/* Help Section */}
          <div className="bg-amber-50 rounded-xl border border-amber-200 p-6">
            <h3 className="text-sm font-semibold text-amber-900 mb-2 flex items-center gap-2">
              <InfoIcon className="w-4 h-4" />
              How to Use
            </h3>
            <ol className="text-sm text-amber-800 space-y-2 list-decimal list-inside">
              <li>Select a template or write a custom query</li>
              <li>Fill in any required placeholders</li>
              <li>Set time range and index</li>
              <li>Click "Prepare Query" to preview</li>
              <li>Copy the query to use in Splunk UI</li>
            </ol>
            <p className="mt-3 text-xs text-amber-700">
              Note: Direct Splunk API integration is not available. Use the prepared query in
              your Splunk web interface.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

// Helper functions
function formatCategory(cat: string): string {
  return cat
    .split("_")
    .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
    .join(" ");
}

// Sub-components
function LevelBadge({ level }: { level: string }) {
  const colors: Record<string, string> = {
    ERROR: "bg-red-100 text-red-700",
    WARN: "bg-amber-100 text-amber-700",
    INFO: "bg-blue-100 text-blue-700",
    DEBUG: "bg-neutral-100 text-neutral-700",
  };

  return (
    <span className={`px-2 py-0.5 text-xs font-medium rounded ${colors[level] || colors.INFO}`}>
      {level}
    </span>
  );
}

// Icons
function TemplateIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z" />
    </svg>
  );
}

function CodeIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5" />
    </svg>
  );
}

function PreviewIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M2.036 12.322a1.012 1.012 0 010-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178z" />
      <path strokeLinecap="round" strokeLinejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
    </svg>
  );
}

function ResultsIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M3.375 19.5h17.25m-17.25 0a1.125 1.125 0 01-1.125-1.125M3.375 19.5h7.5c.621 0 1.125-.504 1.125-1.125m-9.75 0V5.625m0 12.75v-1.5c0-.621.504-1.125 1.125-1.125m18.375 2.625V5.625m0 12.75c0 .621-.504 1.125-1.125 1.125m1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125m0 3.75h-7.5A1.125 1.125 0 0112 18.375m9.75-12.75c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125m19.5 0v1.5c0 .621-.504 1.125-1.125 1.125M2.25 5.625v1.5c0 .621.504 1.125 1.125 1.125m0 0h17.25m-17.25 0h7.5c.621 0 1.125.504 1.125 1.125M3.375 8.25c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125m17.25-3.75h-7.5c-.621 0-1.125.504-1.125 1.125m8.625-1.125c.621 0 1.125.504 1.125 1.125v1.5c0 .621-.504 1.125-1.125 1.125m-17.25 0h7.5m-7.5 0c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125M12 10.875v-1.5m0 1.5c0 .621-.504 1.125-1.125 1.125M12 10.875c0 .621.504 1.125 1.125 1.125m-2.25 0c.621 0 1.125.504 1.125 1.125M13.125 12h7.5m-7.5 0c-.621 0-1.125.504-1.125 1.125M20.625 12c.621 0 1.125.504 1.125 1.125v1.5c0 .621-.504 1.125-1.125 1.125m-17.25 0h7.5M12 14.625v-1.5m0 1.5c0 .621-.504 1.125-1.125 1.125M12 14.625c0 .621.504 1.125 1.125 1.125m-2.25 0c.621 0 1.125.504 1.125 1.125m0 1.5v-1.5m0 0c0-.621.504-1.125 1.125-1.125m0 0h7.5" />
    </svg>
  );
}

function SettingsIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M10.343 3.94c.09-.542.56-.94 1.11-.94h1.093c.55 0 1.02.398 1.11.94l.149.894c.07.424.384.764.78.93.398.164.855.142 1.205-.108l.737-.527a1.125 1.125 0 011.45.12l.773.774c.39.389.44 1.002.12 1.45l-.527.737c-.25.35-.272.806-.107 1.204.165.397.505.71.93.78l.893.15c.543.09.94.56.94 1.109v1.094c0 .55-.397 1.02-.94 1.11l-.893.149c-.425.07-.765.383-.93.78-.165.398-.143.854.107 1.204l.527.738c.32.447.269 1.06-.12 1.45l-.774.773a1.125 1.125 0 01-1.449.12l-.738-.527c-.35-.25-.806-.272-1.203-.107-.397.165-.71.505-.781.929l-.149.894c-.09.542-.56.94-1.11.94h-1.094c-.55 0-1.019-.398-1.11-.94l-.148-.894c-.071-.424-.384-.764-.781-.93-.398-.164-.854-.142-1.204.108l-.738.527c-.447.32-1.06.269-1.45-.12l-.773-.774a1.125 1.125 0 01-.12-1.45l.527-.737c.25-.35.273-.806.108-1.204-.165-.397-.505-.71-.93-.78l-.894-.15c-.542-.09-.94-.56-.94-1.109v-1.094c0-.55.398-1.02.94-1.11l.894-.149c.424-.07.765-.383.93-.78.165-.398.143-.854-.107-1.204l-.527-.738a1.125 1.125 0 01.12-1.45l.773-.773a1.125 1.125 0 011.45-.12l.737.527c.35.25.807.272 1.204.107.397-.165.71-.505.78-.929l.15-.894z" />
      <path strokeLinecap="round" strokeLinejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
    </svg>
  );
}

function InfoIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.5}>
      <path strokeLinecap="round" strokeLinejoin="round" d="M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z" />
    </svg>
  );
}
