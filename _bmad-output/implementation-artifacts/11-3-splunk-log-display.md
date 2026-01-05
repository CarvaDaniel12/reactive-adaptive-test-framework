# Story 11.3: Splunk Log Display

Status: ready-for-dev

## Story

As a QA (Ana),
I want to see Splunk query results,
So that I can analyze production behavior.

## Acceptance Criteria

1. **Given** user runs a Splunk query
   **When** results return
   **Then** log entries are displayed in table format

2. **Given** results are displayed
   **When** columns shown
   **Then** timestamp, level, message columns are visible

3. **Given** result row exists
   **When** user interacts
   **Then** expandable row shows full details

4. **Given** large result set
   **When** displayed
   **Then** pagination handles large results

5. **Given** results are displayed
   **When** export needed
   **Then** export to CSV option is available

6. **Given** query is running
   **When** UI updates
   **Then** loading state is shown during query execution

7. **Given** query is invalid
   **When** error occurs
   **Then** error handling displays for invalid queries

8. **Given** results are returned
   **When** stored
   **Then** results are not stored (privacy)

## Tasks

- [ ] Task 1: Create SplunkResults component
- [ ] Task 2: Create log entry table with columns
- [ ] Task 3: Create expandable row component
- [ ] Task 4: Implement pagination
- [ ] Task 5: Create CSV export functionality
- [ ] Task 6: Add loading state
- [ ] Task 7: Implement error display

## Dev Notes

### Splunk Results Component

```tsx
// frontend/src/components/splunk/SplunkResults.tsx
import { useState } from "react";
import * as Collapsible from "@radix-ui/react-collapsible";

interface SplunkResultsProps {
  data: SplunkQueryResponse;
}

export function SplunkResults({ data }: SplunkResultsProps) {
  const [page, setPage] = useState(1);
  const [expandedRows, setExpandedRows] = useState<Set<number>>(new Set());
  const pageSize = 50;

  const totalPages = Math.ceil(data.results.length / pageSize);
  const paginatedResults = data.results.slice(
    (page - 1) * pageSize,
    page * pageSize
  );

  const handleExportCSV = () => {
    const headers = Object.keys(data.results[0] || {});
    const csv = [
      headers.join(","),
      ...data.results.map(row =>
        headers.map(h => JSON.stringify(row[h] ?? "")).join(",")
      ),
    ].join("\n");

    const blob = new Blob([csv], { type: "text/csv" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `splunk-results-${Date.now()}.csv`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const toggleRow = (index: number) => {
    const newExpanded = new Set(expandedRows);
    if (newExpanded.has(index)) {
      newExpanded.delete(index);
    } else {
      newExpanded.add(index);
    }
    setExpandedRows(newExpanded);
  };

  return (
    <div>
      {/* Header */}
      <div className="p-4 border-b border-neutral-200 flex items-center justify-between">
        <div className="flex items-center gap-4 text-sm text-neutral-500">
          <span>{data.totalCount.toLocaleString()} results</span>
          <span>•</span>
          <span>{data.executionTimeMs}ms</span>
        </div>
        <button
          onClick={handleExportCSV}
          className="text-sm text-primary-600 hover:text-primary-700 flex items-center gap-1"
        >
          <DownloadIcon className="w-4 h-4" />
          Export CSV
        </button>
      </div>

      {/* Table */}
      <div className="overflow-x-auto">
        <table className="w-full">
          <thead>
            <tr className="bg-neutral-50 border-b border-neutral-200">
              <th className="w-8 p-2"></th>
              <th className="text-left p-2 text-xs font-medium text-neutral-500 uppercase">
                Time
              </th>
              <th className="text-left p-2 text-xs font-medium text-neutral-500 uppercase w-20">
                Level
              </th>
              <th className="text-left p-2 text-xs font-medium text-neutral-500 uppercase">
                Message
              </th>
              <th className="text-left p-2 text-xs font-medium text-neutral-500 uppercase w-32">
                Source
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-neutral-100">
            {paginatedResults.map((row, index) => (
              <LogEntry
                key={index}
                row={row}
                index={(page - 1) * pageSize + index}
                isExpanded={expandedRows.has(index)}
                onToggle={() => toggleRow(index)}
              />
            ))}
          </tbody>
        </table>
      </div>

      {/* Pagination */}
      {totalPages > 1 && (
        <div className="p-4 border-t border-neutral-200 flex items-center justify-between">
          <span className="text-sm text-neutral-500">
            Page {page} of {totalPages}
          </span>
          <div className="flex items-center gap-2">
            <button
              onClick={() => setPage(p => Math.max(1, p - 1))}
              disabled={page === 1}
              className="px-3 py-1 text-sm border border-neutral-300 rounded hover:bg-neutral-50 disabled:opacity-50"
            >
              Previous
            </button>
            <button
              onClick={() => setPage(p => Math.min(totalPages, p + 1))}
              disabled={page === totalPages}
              className="px-3 py-1 text-sm border border-neutral-300 rounded hover:bg-neutral-50 disabled:opacity-50"
            >
              Next
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
```

### Log Entry Row

```tsx
// frontend/src/components/splunk/LogEntry.tsx
interface LogEntryProps {
  row: Record<string, any>;
  index: number;
  isExpanded: boolean;
  onToggle: () => void;
}

export function LogEntry({ row, index, isExpanded, onToggle }: LogEntryProps) {
  const level = row.level || row.severity || "INFO";
  const levelColors: Record<string, string> = {
    ERROR: "bg-error-100 text-error-700",
    FATAL: "bg-error-100 text-error-700",
    WARN: "bg-warning-100 text-warning-700",
    WARNING: "bg-warning-100 text-warning-700",
    INFO: "bg-primary-100 text-primary-700",
    DEBUG: "bg-neutral-100 text-neutral-700",
  };

  const timestamp = row._time || row.timestamp || row["@timestamp"];
  const message = row.message || row.msg || row._raw?.slice(0, 200);
  const source = row.source || row.sourcetype || row.host;

  return (
    <Collapsible.Root open={isExpanded} onOpenChange={onToggle}>
      <Collapsible.Trigger asChild>
        <tr className="hover:bg-neutral-50 cursor-pointer">
          <td className="p-2 text-center">
            <ChevronRightIcon
              className={cn(
                "w-4 h-4 text-neutral-400 transition-transform",
                isExpanded && "rotate-90"
              )}
            />
          </td>
          <td className="p-2 text-sm font-mono text-neutral-600 whitespace-nowrap">
            {formatTimestamp(timestamp)}
          </td>
          <td className="p-2">
            <span className={cn(
              "px-2 py-0.5 text-xs font-medium rounded",
              levelColors[level.toUpperCase()] || levelColors.INFO
            )}>
              {level}
            </span>
          </td>
          <td className="p-2 text-sm text-neutral-800 max-w-md truncate">
            {message}
          </td>
          <td className="p-2 text-sm text-neutral-500 truncate">
            {source}
          </td>
        </tr>
      </Collapsible.Trigger>

      <Collapsible.Content asChild>
        <tr className="bg-neutral-50">
          <td colSpan={5} className="p-4">
            <div className="text-sm">
              <h5 className="font-medium text-neutral-700 mb-2">Full Log Entry</h5>
              <pre className="p-3 bg-neutral-900 text-neutral-100 rounded-lg overflow-x-auto text-xs">
                {JSON.stringify(row, null, 2)}
              </pre>
            </div>
          </td>
        </tr>
      </Collapsible.Content>
    </Collapsible.Root>
  );
}

function formatTimestamp(ts: string): string {
  try {
    return new Date(ts).toLocaleString();
  } catch {
    return ts;
  }
}
```

### Loading State

```tsx
// frontend/src/components/splunk/SplunkLoadingState.tsx
export function SplunkLoadingState() {
  return (
    <div className="p-8 flex flex-col items-center justify-center">
      <div className="relative">
        <div className="w-12 h-12 border-4 border-primary-200 rounded-full animate-spin border-t-primary-500" />
      </div>
      <p className="mt-4 text-neutral-600">Running query...</p>
      <p className="text-sm text-neutral-400">This may take a few seconds</p>
    </div>
  );
}
```

### Error Display

```tsx
// frontend/src/components/splunk/SplunkError.tsx
interface SplunkErrorProps {
  error: Error;
  onRetry: () => void;
}

export function SplunkError({ error, onRetry }: SplunkErrorProps) {
  const isAuthError = error.message.includes("401") || error.message.includes("unauthorized");
  const isSyntaxError = error.message.includes("syntax") || error.message.includes("invalid");

  return (
    <div className="p-6 text-center">
      <div className="w-12 h-12 mx-auto mb-4 bg-error-100 rounded-full flex items-center justify-center">
        <ExclamationTriangleIcon className="w-6 h-6 text-error-500" />
      </div>
      
      <h4 className="font-medium text-neutral-900 mb-2">Query Failed</h4>
      <p className="text-sm text-neutral-600 mb-4">{error.message}</p>

      {isAuthError && (
        <p className="text-sm text-warning-600 mb-4">
          Please check your Splunk credentials in Settings.
        </p>
      )}

      {isSyntaxError && (
        <p className="text-sm text-warning-600 mb-4">
          Check your SPL syntax. <a href="https://docs.splunk.com" target="_blank" className="underline">View SPL reference</a>
        </p>
      )}

      <button
        onClick={onRetry}
        className="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600"
      >
        Try Again
      </button>
    </div>
  );
}
```

### Privacy Note

Results are only held in React state and never persisted. When the component unmounts or user navigates away, data is cleared.

### References

- [Source: epics.md#Story 11.3]
