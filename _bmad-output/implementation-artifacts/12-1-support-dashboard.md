# Story 12.1: Support Dashboard

Status: ready-for-dev

## Story

As a support person (Sofia),
I want a dashboard of support requests,
So that I can see what needs attention.

## Acceptance Criteria

1. **Given** users have encountered errors
   **When** support person views support dashboard
   **Then** list of recent errors/issues is displayed

2. **Given** error list shown
   **When** details viewed
   **Then** error type and frequency are visible

3. **Given** error list shown
   **When** details viewed
   **Then** affected user is shown

4. **Given** error list shown
   **When** details viewed
   **Then** timestamp is displayed

5. **Given** error list shown
   **When** status viewed
   **Then** status is shown (new / investigating / resolved)

6. **Given** error list exists
   **When** sorted
   **Then** sortable by severity, date, user

7. **Given** error list exists
   **When** searched
   **Then** search by error message works

8. **Given** sidebar navigation
   **When** accessed
   **Then** accessible via admin sidebar item

## Tasks

- [ ] Task 1: Create SupportDashboardPage
- [ ] Task 2: Create error_logs database table
- [ ] Task 3: Create ErrorList component
- [ ] Task 4: Create ErrorListItem component
- [ ] Task 5: Implement sorting functionality
- [ ] Task 6: Implement search functionality
- [ ] Task 7: Add to admin sidebar

## Dev Notes

### Database Schema

```sql
-- migrations/20260103_create_support_tables.sql
CREATE TYPE error_severity AS ENUM ('low', 'medium', 'high', 'critical');
CREATE TYPE error_status AS ENUM ('new', 'investigating', 'resolved', 'wont_fix');

CREATE TABLE error_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    error_type VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    stack_trace TEXT,
    severity error_severity DEFAULT 'medium',
    status error_status DEFAULT 'new',
    user_id VARCHAR(255),
    session_id VARCHAR(255),
    page_url VARCHAR(500),
    browser VARCHAR(255),
    device VARCHAR(255),
    metadata JSONB,
    resolved_at TIMESTAMPTZ,
    resolved_by VARCHAR(255),
    resolution_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_error_logs_status ON error_logs(status);
CREATE INDEX idx_error_logs_severity ON error_logs(severity);
CREATE INDEX idx_error_logs_user ON error_logs(user_id);
CREATE INDEX idx_error_logs_created ON error_logs(created_at DESC);
CREATE INDEX idx_error_logs_error_type ON error_logs(error_type);
```

### Support Dashboard Page

```tsx
// frontend/src/pages/admin/SupportDashboard.tsx
import { useState } from "react";
import { useErrorLogs } from "@/hooks/useErrorLogs";
import { ErrorList } from "@/components/support/ErrorList";
import { ErrorFilters } from "@/components/support/ErrorFilters";
import { ErrorStats } from "@/components/support/ErrorStats";

export function SupportDashboardPage() {
  const [filters, setFilters] = useState<ErrorFilters>({});
  const [sortBy, setSortBy] = useState<SortOption>("created_at");
  const [sortOrder, setSortOrder] = useState<"asc" | "desc">("desc");
  const [search, setSearch] = useState("");
  const [page, setPage] = useState(1);

  const { data, isLoading } = useErrorLogs({
    ...filters,
    search,
    sortBy,
    sortOrder,
    page,
  });

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">Support Dashboard</h1>
          <p className="text-sm text-neutral-500">Monitor and resolve user issues</p>
        </div>
      </div>

      {/* Stats */}
      <ErrorStats stats={data?.stats} isLoading={isLoading} />

      {/* Filters & Search */}
      <div className="flex items-center gap-4">
        <div className="flex-1 relative">
          <MagnifyingGlassIcon className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-neutral-400" />
          <input
            type="text"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder="Search error messages..."
            className="w-full pl-10 pr-4 py-2 border border-neutral-300 rounded-lg"
          />
        </div>
        <ErrorFilters value={filters} onChange={setFilters} />
        <SortDropdown value={sortBy} order={sortOrder} onChange={(s, o) => { setSortBy(s); setSortOrder(o); }} />
      </div>

      {/* Error List */}
      <ErrorList
        errors={data?.errors}
        isLoading={isLoading}
        onStatusChange={(id, status) => updateStatus(id, status)}
      />

      {/* Pagination */}
      {data && data.totalPages > 1 && (
        <Pagination
          currentPage={page}
          totalPages={data.totalPages}
          onPageChange={setPage}
        />
      )}
    </div>
  );
}
```

### Error List Component

```tsx
// frontend/src/components/support/ErrorList.tsx
interface ErrorListProps {
  errors?: ErrorLog[];
  isLoading: boolean;
  onStatusChange: (id: string, status: ErrorStatus) => void;
}

export function ErrorList({ errors, isLoading, onStatusChange }: ErrorListProps) {
  if (isLoading) {
    return <ErrorListSkeleton />;
  }

  if (!errors?.length) {
    return (
      <div className="text-center py-12 text-neutral-500">
        <CheckCircledIcon className="w-12 h-12 mx-auto mb-4 text-success-500" />
        <p className="font-medium">All clear!</p>
        <p className="text-sm">No errors to review</p>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-xl border border-neutral-200 overflow-hidden">
      <table className="w-full">
        <thead>
          <tr className="bg-neutral-50 border-b border-neutral-200">
            <th className="text-left p-4 text-xs font-medium text-neutral-500 uppercase">Error</th>
            <th className="text-left p-4 text-xs font-medium text-neutral-500 uppercase w-24">Severity</th>
            <th className="text-left p-4 text-xs font-medium text-neutral-500 uppercase w-32">Status</th>
            <th className="text-left p-4 text-xs font-medium text-neutral-500 uppercase w-32">User</th>
            <th className="text-left p-4 text-xs font-medium text-neutral-500 uppercase w-40">Time</th>
            <th className="w-20"></th>
          </tr>
        </thead>
        <tbody className="divide-y divide-neutral-100">
          {errors.map((error) => (
            <ErrorListItem
              key={error.id}
              error={error}
              onStatusChange={(status) => onStatusChange(error.id, status)}
            />
          ))}
        </tbody>
      </table>
    </div>
  );
}
```

### Error List Item

```tsx
// frontend/src/components/support/ErrorListItem.tsx
interface ErrorListItemProps {
  error: ErrorLog;
  onStatusChange: (status: ErrorStatus) => void;
}

export function ErrorListItem({ error, onStatusChange }: ErrorListItemProps) {
  const [detailsOpen, setDetailsOpen] = useState(false);

  const severityConfig = {
    low: { bg: "bg-neutral-100", text: "text-neutral-700" },
    medium: { bg: "bg-warning-100", text: "text-warning-700" },
    high: { bg: "bg-orange-100", text: "text-orange-700" },
    critical: { bg: "bg-error-100", text: "text-error-700" },
  };

  const statusConfig = {
    new: { bg: "bg-primary-100", text: "text-primary-700" },
    investigating: { bg: "bg-warning-100", text: "text-warning-700" },
    resolved: { bg: "bg-success-100", text: "text-success-700" },
    wont_fix: { bg: "bg-neutral-100", text: "text-neutral-700" },
  };

  return (
    <>
      <tr className="hover:bg-neutral-50">
        <td className="p-4">
          <button
            onClick={() => setDetailsOpen(!detailsOpen)}
            className="text-left"
          >
            <p className="font-medium text-neutral-900">{error.errorType}</p>
            <p className="text-sm text-neutral-500 truncate max-w-md">
              {error.message}
            </p>
          </button>
        </td>
        <td className="p-4">
          <span className={cn(
            "px-2 py-1 text-xs font-medium rounded",
            severityConfig[error.severity].bg,
            severityConfig[error.severity].text
          )}>
            {error.severity}
          </span>
        </td>
        <td className="p-4">
          <StatusDropdown
            value={error.status}
            onChange={onStatusChange}
          />
        </td>
        <td className="p-4 text-sm text-neutral-600">
          {error.userId || "Anonymous"}
        </td>
        <td className="p-4 text-sm text-neutral-500">
          {formatDistanceToNow(new Date(error.createdAt), { addSuffix: true })}
        </td>
        <td className="p-4">
          <button
            onClick={() => setDetailsOpen(!detailsOpen)}
            className="p-2 text-neutral-400 hover:text-neutral-600"
          >
            <ChevronDownIcon className={cn("w-5 h-5", detailsOpen && "rotate-180")} />
          </button>
        </td>
      </tr>

      {/* Expanded Details */}
      {detailsOpen && (
        <tr className="bg-neutral-50">
          <td colSpan={6} className="p-4">
            <ErrorDetails error={error} />
          </td>
        </tr>
      )}
    </>
  );
}
```

### API Endpoints

```rust
// GET /api/v1/admin/errors
#[derive(Debug, Deserialize)]
pub struct ErrorListQuery {
    status: Option<String>,
    severity: Option<String>,
    search: Option<String>,
    sort_by: Option<String>,
    sort_order: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
}

pub async fn list_errors(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ErrorListQuery>,
) -> Result<Json<ErrorListResponse>, ApiError> {
    // Implementation
}

// PUT /api/v1/admin/errors/:id/status
pub async fn update_error_status(
    State(state): State<Arc<AppState>>,
    Path(error_id): Path<Uuid>,
    Json(request): Json<UpdateStatusRequest>,
) -> Result<Json<ErrorLog>, ApiError> {
    // Implementation
}
```

### References

- [Source: epics.md#Story 12.1]
