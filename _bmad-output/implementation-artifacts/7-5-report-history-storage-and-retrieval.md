# Story 7.5: Report History Storage and Retrieval

Status: ready-for-dev

## Story

As a QA (Ana),
I want to access my past reports,
So that I can reference previous testing work.

## Acceptance Criteria

1. **Given** user has completed workflows with reports
   **When** user views "Report History"
   **Then** list displays report date and ticket key

2. **Given** report list is displayed
   **When** item is shown
   **Then** workflow type is visible

3. **Given** report list is displayed
   **When** item is shown
   **Then** total time spent is visible

4. **Given** report item is hovered
   **When** preview loads
   **Then** quick preview is shown on hover

5. **Given** report list exists
   **When** sorted
   **Then** reports are sorted by date (newest first)

6. **Given** report item exists
   **When** clicked
   **Then** clicking opens full view

7. **Given** report list exists
   **When** user searches
   **Then** search/filter by ticket key or date range is available

8. **Given** report retention
   **When** time passes
   **Then** reports are retained per NFR-REL-04

## Tasks

- [ ] Task 1: Create GET /api/v1/reports endpoint with pagination
- [ ] Task 2: Add search and filter parameters
- [ ] Task 3: Create ReportHistoryPage component
- [x] Task 4: Create ReportListItem with hover preview
- [ ] Task 5: Implement date range filter
- [ ] Task 6: Create report detail page route

## Dev Notes

### API Endpoint

```rust
// GET /api/v1/reports
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListReportsQuery {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub ticket_key: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub template_name: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportListResponse {
    pub reports: Vec<ReportSummary>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ReportSummary {
    pub id: Uuid,
    pub ticket_key: String,
    pub ticket_title: Option<String>,
    pub template_name: String,
    pub total_time_seconds: Option<i32>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[utoipa::path(
    get,
    path = "/api/v1/reports",
    params(
        ("page" = Option<i32>, Query),
        ("per_page" = Option<i32>, Query),
        ("ticket_key" = Option<String>, Query),
        ("start_date" = Option<NaiveDate>, Query),
        ("end_date" = Option<NaiveDate>, Query),
    ),
    responses(
        (status = 200, description = "List of reports", body = ReportListResponse),
    ),
    tag = "reports"
)]
pub async fn list_reports(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListReportsQuery>,
) -> Result<Json<ReportListResponse>, ApiError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    // Build dynamic query
    let mut sql = String::from(
        r#"
        SELECT id, ticket_key, ticket_title, template_name, 
               total_time_seconds, status, created_at
        FROM reports
        WHERE 1=1
        "#
    );
    
    let mut count_sql = String::from("SELECT COUNT(*) FROM reports WHERE 1=1");
    
    if let Some(ref key) = query.ticket_key {
        sql.push_str(" AND ticket_key ILIKE $1");
        count_sql.push_str(" AND ticket_key ILIKE $1");
    }
    
    if query.start_date.is_some() {
        sql.push_str(" AND DATE(created_at) >= $2");
        count_sql.push_str(" AND DATE(created_at) >= $2");
    }
    
    if query.end_date.is_some() {
        sql.push_str(" AND DATE(created_at) <= $3");
        count_sql.push_str(" AND DATE(created_at) <= $3");
    }
    
    sql.push_str(" ORDER BY created_at DESC LIMIT $4 OFFSET $5");

    // Execute queries
    let reports = sqlx::query_as::<_, ReportSummary>(&sql)
        .bind(query.ticket_key.as_ref().map(|k| format!("%{}%", k)))
        .bind(query.start_date)
        .bind(query.end_date)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let total: i64 = sqlx::query_scalar(&count_sql)
        .bind(query.ticket_key.as_ref().map(|k| format!("%{}%", k)))
        .bind(query.start_date)
        .bind(query.end_date)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;

    Ok(Json(ReportListResponse {
        reports,
        total,
        page,
        per_page,
        total_pages,
    }))
}
```

### Frontend - Report History Page

```tsx
// frontend/src/pages/ReportHistory.tsx
import { useState } from "react";
import { useReports } from "@/hooks/useReports";
import { ReportListItem } from "@/components/reports/ReportListItem";
import { ReportFilters } from "@/components/reports/ReportFilters";
import { Pagination } from "@/components/ui/Pagination";

export function ReportHistoryPage() {
  const [page, setPage] = useState(1);
  const [filters, setFilters] = useState<ReportFilters>({});
  
  const { data, isLoading } = useReports({ page, ...filters });

  return (
    <div className="max-w-4xl mx-auto p-6">
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">Report History</h1>
        <span className="text-sm text-neutral-500">
          {data?.total || 0} reports
        </span>
      </div>

      {/* Filters */}
      <ReportFilters
        value={filters}
        onChange={setFilters}
        className="mb-6"
      />

      {/* List */}
      {isLoading ? (
        <div className="space-y-4">
          {[1, 2, 3].map((i) => (
            <div key={i} className="h-24 bg-neutral-100 rounded-lg animate-pulse" />
          ))}
        </div>
      ) : data?.reports.length === 0 ? (
        <div className="text-center py-12 text-neutral-500">
          <FileTextIcon className="w-12 h-12 mx-auto mb-4 opacity-50" />
          <p>No reports found</p>
        </div>
      ) : (
        <div className="space-y-3">
          {data?.reports.map((report) => (
            <ReportListItem key={report.id} report={report} />
          ))}
        </div>
      )}

      {/* Pagination */}
      {data && data.totalPages > 1 && (
        <Pagination
          currentPage={page}
          totalPages={data.totalPages}
          onPageChange={setPage}
          className="mt-6"
        />
      )}
    </div>
  );
}
```

### Report List Item with Hover Preview

```tsx
// frontend/src/components/reports/ReportListItem.tsx
import { useState } from "react";
import { Link } from "react-router-dom";
import { formatDistanceToNow, format } from "date-fns";
import * as HoverCard from "@radix-ui/react-hover-card";

interface ReportListItemProps {
  report: ReportSummary;
}

export function ReportListItem({ report }: ReportListItemProps) {
  return (
    <HoverCard.Root openDelay={300}>
      <HoverCard.Trigger asChild>
        <Link
          to={`/reports/${report.id}`}
          className="block p-4 bg-white border border-neutral-200 rounded-lg 
                     hover:border-primary-300 hover:shadow-sm transition-all"
        >
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <span className="px-2 py-1 bg-primary-50 text-primary-700 
                             font-mono text-sm rounded">
                {report.ticketKey}
              </span>
              <span className="text-neutral-700 font-medium truncate max-w-md">
                {report.ticketTitle || "Untitled"}
              </span>
            </div>

            <div className="flex items-center gap-4 text-sm text-neutral-500">
              <span>{report.templateName}</span>
              {report.totalTimeSeconds && (
                <span>{formatDuration(report.totalTimeSeconds)}</span>
              )}
              <span title={format(new Date(report.createdAt), "PPpp")}>
                {formatDistanceToNow(new Date(report.createdAt), { addSuffix: true })}
              </span>
            </div>
          </div>
        </Link>
      </HoverCard.Trigger>

      <HoverCard.Portal>
        <HoverCard.Content
          side="right"
          sideOffset={8}
          className="bg-white rounded-lg shadow-xl border border-neutral-200 
                     p-4 w-80 animate-scale-in"
        >
          <ReportQuickPreview reportId={report.id} />
          <HoverCard.Arrow className="fill-white" />
        </HoverCard.Content>
      </HoverCard.Portal>
    </HoverCard.Root>
  );
}

function ReportQuickPreview({ reportId }: { reportId: string }) {
  const { data: report, isLoading } = useReport(reportId);

  if (isLoading) {
    return <div className="animate-pulse h-32 bg-neutral-100 rounded" />;
  }

  if (!report) return null;

  const content = report.content;

  return (
    <div className="space-y-3">
      <div className="text-sm">
        <span className="font-medium">{content.steps.length} steps</span>
        <span className="text-neutral-400 mx-2">•</span>
        <span className="text-success-600">
          {content.steps.filter(s => s.status === "completed").length} completed
        </span>
      </div>

      <div className="text-sm">
        <span className="text-neutral-500">Time: </span>
        <span className="font-medium">
          {formatDuration(content.timeSummary.totalActiveSeconds)}
        </span>
        <span className="text-neutral-400"> / </span>
        <span>{formatDuration(content.timeSummary.totalEstimatedSeconds)} est.</span>
      </div>

      {content.testsCovered && (
        <div className="text-sm">
          <span className="text-neutral-500">Tests: </span>
          <span className="font-medium">{content.testsCovered.totalTests}</span>
        </div>
      )}

      <div className="pt-2 border-t border-neutral-100 text-xs text-primary-600">
        Click to view full report →
      </div>
    </div>
  );
}
```

### Report Filters Component

```tsx
// frontend/src/components/reports/ReportFilters.tsx
interface ReportFiltersProps {
  value: ReportFilters;
  onChange: (filters: ReportFilters) => void;
  className?: string;
}

export function ReportFilters({ value, onChange, className }: ReportFiltersProps) {
  return (
    <div className={cn("flex items-center gap-4", className)}>
      {/* Search */}
      <div className="relative flex-1">
        <MagnifyingGlassIcon className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-neutral-400" />
        <input
          type="text"
          value={value.ticketKey || ""}
          onChange={(e) => onChange({ ...value, ticketKey: e.target.value || undefined })}
          placeholder="Search by ticket key..."
          className="w-full pl-9 pr-4 py-2 border border-neutral-300 rounded-lg text-sm"
        />
      </div>

      {/* Date Range */}
      <DateRangePicker
        startDate={value.startDate}
        endDate={value.endDate}
        onChange={(start, end) => onChange({ ...value, startDate: start, endDate: end })}
      />

      {/* Clear Filters */}
      {(value.ticketKey || value.startDate || value.endDate) && (
        <button
          onClick={() => onChange({})}
          className="text-sm text-neutral-500 hover:text-neutral-700"
        >
          Clear
        </button>
      )}
    </div>
  );
}
```

### useReports Hook

```tsx
// frontend/src/hooks/useReports.ts
interface UseReportsOptions {
  page?: number;
  perPage?: number;
  ticketKey?: string;
  startDate?: string;
  endDate?: string;
}

export function useReports(options: UseReportsOptions = {}) {
  return useQuery({
    queryKey: ["reports", options],
    queryFn: async () => {
      const params = new URLSearchParams();
      if (options.page) params.set("page", options.page.toString());
      if (options.perPage) params.set("perPage", options.perPage.toString());
      if (options.ticketKey) params.set("ticketKey", options.ticketKey);
      if (options.startDate) params.set("startDate", options.startDate);
      if (options.endDate) params.set("endDate", options.endDate);

      const response = await fetch(`/api/v1/reports?${params}`);
      if (!response.ok) throw new Error("Failed to fetch reports");
      return response.json() as Promise<ReportListResponse>;
    },
  });
}
```

### References

- [Source: epics.md#Story 7.5]
- [NFR: NFR-REL-04 - Data retention]
