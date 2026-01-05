# Story 11.1: Splunk Query Interface

Status: ready-for-dev

## Story

As a QA (Ana),
I want to query Splunk from the framework,
So that I can investigate production issues.

## Acceptance Criteria

1. **Given** Splunk is configured (manual setup)
   **When** user opens Splunk panel
   **Then** query input field is shown (SPL syntax)

2. **Given** Splunk panel is open
   **When** interface rendered
   **Then** time range selector is available

3. **Given** Splunk panel is open
   **When** interface rendered
   **Then** "Run Query" button is available

4. **Given** query is executed
   **When** results return
   **Then** results area displays data

5. **Given** query input exists
   **When** templates available
   **Then** query templates are available as dropdown

6. **Given** ticket context exists
   **When** query is prepared
   **Then** ticket context (key) can be auto-inserted in query

7. **Given** user needs help
   **When** help link shown
   **Then** instructions link to Splunk documentation

## Tasks

- [ ] Task 1: Create SplunkPanel component
- [ ] Task 2: Create SPL query input with syntax hints
- [ ] Task 3: Create TimeRangeSelector component
- [ ] Task 4: Create query execution hook
- [ ] Task 5: Create template dropdown
- [ ] Task 6: Implement ticket context injection
- [ ] Task 7: Add Splunk documentation link

## Dev Notes

### Splunk Panel Component

```tsx
// frontend/src/components/splunk/SplunkPanel.tsx
import { useState } from "react";
import { useSplunkQuery } from "@/hooks/useSplunkQuery";
import { useSplunkTemplates } from "@/hooks/useSplunkTemplates";

interface SplunkPanelProps {
  ticketKey?: string;
  ticketTitle?: string;
}

export function SplunkPanel({ ticketKey, ticketTitle }: SplunkPanelProps) {
  const [query, setQuery] = useState("");
  const [timeRange, setTimeRange] = useState<TimeRange>({
    start: "-24h",
    end: "now",
  });
  
  const { data: templates } = useSplunkTemplates();
  const { mutate: runQuery, data: results, isPending, error } = useSplunkQuery();

  const handleTemplateSelect = (template: SplunkTemplate) => {
    let q = template.query;
    // Replace placeholders
    if (ticketKey) {
      q = q.replace(/\{TICKET_KEY\}/g, ticketKey);
    }
    setQuery(q);
  };

  const handleRunQuery = () => {
    runQuery({ query, timeRange });
  };

  return (
    <div className="bg-white rounded-xl border border-neutral-200">
      {/* Header */}
      <div className="p-4 border-b border-neutral-200 flex items-center justify-between">
        <div className="flex items-center gap-2">
          <SplunkIcon className="w-5 h-5" />
          <h3 className="font-semibold">Splunk Logs</h3>
        </div>
        <a
          href="https://docs.splunk.com/Documentation/Splunk/latest/SearchReference"
          target="_blank"
          rel="noopener noreferrer"
          className="text-sm text-primary-600 hover:underline flex items-center gap-1"
        >
          <ExternalLinkIcon className="w-4 h-4" />
          SPL Reference
        </a>
      </div>

      {/* Query Area */}
      <div className="p-4 space-y-4">
        {/* Template Selector */}
        <div className="flex items-center gap-2">
          <label className="text-sm text-neutral-600">Template:</label>
          <select
            onChange={(e) => {
              const template = templates?.find(t => t.id === e.target.value);
              if (template) handleTemplateSelect(template);
            }}
            className="flex-1 px-3 py-2 border border-neutral-300 rounded-lg text-sm"
          >
            <option value="">Select a template...</option>
            {templates?.map((t) => (
              <option key={t.id} value={t.id}>{t.name}</option>
            ))}
          </select>
        </div>

        {/* Query Input */}
        <div>
          <label className="block text-sm font-medium text-neutral-700 mb-1">
            SPL Query
          </label>
          <textarea
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="index=main sourcetype=app_logs | search ..."
            rows={4}
            className="w-full px-3 py-2 font-mono text-sm border border-neutral-300 rounded-lg
                       focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
          />
          {ticketKey && (
            <p className="text-xs text-neutral-500 mt-1">
              Tip: Use {"{TICKET_KEY}"} to insert "{ticketKey}"
            </p>
          )}
        </div>

        {/* Time Range & Run */}
        <div className="flex items-end gap-4">
          <TimeRangeSelector value={timeRange} onChange={setTimeRange} />
          
          <button
            onClick={handleRunQuery}
            disabled={!query.trim() || isPending}
            className="px-4 py-2 bg-primary-500 text-white font-medium rounded-lg
                       hover:bg-primary-600 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isPending ? "Running..." : "Run Query"}
          </button>
        </div>
      </div>

      {/* Error */}
      {error && (
        <div className="mx-4 mb-4 p-3 bg-error-50 border border-error-200 rounded-lg text-sm text-error-700">
          {error.message}
        </div>
      )}

      {/* Results */}
      {results && (
        <div className="border-t border-neutral-200">
          <SplunkResults data={results} />
        </div>
      )}
    </div>
  );
}
```

### Time Range Selector

```tsx
// frontend/src/components/splunk/TimeRangeSelector.tsx
interface TimeRange {
  start: string;
  end: string;
}

interface TimeRangeSelectorProps {
  value: TimeRange;
  onChange: (range: TimeRange) => void;
}

const presets = [
  { label: "Last 15 min", start: "-15m", end: "now" },
  { label: "Last hour", start: "-1h", end: "now" },
  { label: "Last 4 hours", start: "-4h", end: "now" },
  { label: "Last 24 hours", start: "-24h", end: "now" },
  { label: "Last 7 days", start: "-7d", end: "now" },
];

export function TimeRangeSelector({ value, onChange }: TimeRangeSelectorProps) {
  return (
    <div className="flex-1">
      <label className="block text-sm font-medium text-neutral-700 mb-1">
        Time Range
      </label>
      <select
        value={`${value.start}|${value.end}`}
        onChange={(e) => {
          const [start, end] = e.target.value.split("|");
          onChange({ start, end });
        }}
        className="w-full px-3 py-2 border border-neutral-300 rounded-lg text-sm"
      >
        {presets.map((p) => (
          <option key={p.label} value={`${p.start}|${p.end}`}>
            {p.label}
          </option>
        ))}
      </select>
    </div>
  );
}
```

### Splunk Query Hook

```tsx
// frontend/src/hooks/useSplunkQuery.ts
interface SplunkQueryParams {
  query: string;
  timeRange: TimeRange;
}

export function useSplunkQuery() {
  return useMutation({
    mutationFn: async (params: SplunkQueryParams) => {
      const response = await fetch("/api/v1/splunk/query", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          query: params.query,
          earliest: params.timeRange.start,
          latest: params.timeRange.end,
        }),
      });

      if (!response.ok) {
        const error = await response.json();
        throw new Error(error.message || "Query failed");
      }

      return response.json();
    },
  });
}
```

### Backend API

```rust
// POST /api/v1/splunk/query
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplunkQueryRequest {
    pub query: String,
    pub earliest: String,
    pub latest: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SplunkQueryResponse {
    pub results: Vec<serde_json::Value>,
    pub total_count: i64,
    pub execution_time_ms: u64,
}

pub async fn execute_splunk_query(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SplunkQueryRequest>,
) -> Result<Json<SplunkQueryResponse>, ApiError> {
    let config = state.config.splunk.as_ref()
        .ok_or_else(|| ApiError::BadRequest("Splunk not configured".into()))?;

    let client = SplunkClient::new(&config.base_url, &config.token)?;
    
    let start = std::time::Instant::now();
    let results = client.search(&request.query, &request.earliest, &request.latest).await?;
    let execution_time = start.elapsed().as_millis() as u64;

    Ok(Json(SplunkQueryResponse {
        results: results.events,
        total_count: results.total_count,
        execution_time_ms: execution_time,
    }))
}
```

### References

- [Source: epics.md#Story 11.1]
