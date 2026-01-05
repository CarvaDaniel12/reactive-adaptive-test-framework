# Story 3.2: Jira Ticket Listing with Filters

Status: done

## Story

As a QA (Ana),
I want to see my Jira tickets filtered by status,
So that I can quickly find tickets ready for testing.

## Acceptance Criteria

1. **Given** user is authenticated with Jira
   **When** user views the ticket list
   **Then** tickets display showing: ticket key, title, priority (with color), status, assignee

2. **Given** ticket list is displayed
   **When** filtering is applied
   **Then** tickets are filtered by configured states (from setup wizard)

3. **Given** ticket list is displayed
   **When** user filter is applied
   **Then** tickets are filtered by user's Jira username

4. **Given** ticket list is loading
   **When** data is being fetched
   **Then** loading state shows skeleton cards

5. **Given** ticket list is loaded
   **When** no tickets match filters
   **Then** empty state shows helpful message

6. **Given** ticket list API call
   **When** performance is measured
   **Then** list loads in < 2s (NFR-PERF-01)

## Tasks / Subtasks

- [x] Task 1: Create Jira API client for tickets (AC: #1, #6)
  - [x] 1.1: Add `tickets.rs` module to `qa-pms-jira`
  - [x] 1.2: Implement `list_tickets()` with JQL query
  - [x] 1.3: Implement pagination support
  - [x] 1.4: Add retry with exponential backoff

- [x] Task 2: Create ticket list API endpoint (AC: #1, #2, #3)
  - [x] 2.1: Add `GET /api/v1/tickets` endpoint
  - [x] 2.2: Accept filter query params (status, assignee)
  - [x] 2.3: Build JQL from filters
  - [x] 2.4: Return paginated response

- [x] Task 3: Create ticket list page (AC: #1)
  - [x] 3.1: Create `TicketsPage.tsx` in `frontend/src/pages/`
  - [x] 3.2: Fetch tickets from API
  - [x] 3.3: Display ticket cards in grid/list

- [x] Task 4: Create TicketCard component (AC: #1)
  - [x] 4.1: Create `TicketCard.tsx` component
  - [x] 4.2: Display key, title, priority, status, assignee
  - [x] 4.3: Add priority color indicator
  - [x] 4.4: Add click handler for navigation

- [x] Task 5: Implement skeleton loading (AC: #4)
  - [x] 5.1: Create `TicketCardSkeleton.tsx` component
  - [x] 5.2: Show skeleton grid while loading
  - [x] 5.3: Match skeleton to card dimensions

- [x] Task 6: Implement empty state (AC: #5)
  - [x] 6.1: Create `EmptyTicketList.tsx` component
  - [x] 6.2: Show helpful message when no tickets
  - [x] 6.3: Suggest filter adjustments

- [x] Task 7: Implement filter controls (AC: #2, #3)
  - [x] 7.1: Create `TicketFilters.tsx` component
  - [x] 7.2: Add status filter dropdown
  - [x] 7.3: Load filter options from user config
  - [x] 7.4: Update URL query params on filter change

- [x] Task 8: Add performance monitoring (AC: #6)
  - [x] 8.1: Add timing metrics to API call
  - [x] 8.2: Log slow requests (> 2s)
  - [x] 8.3: Display load time in dev mode

## Dev Notes

### Architecture Alignment

This story implements **Jira Ticket Listing** per Epic 3 requirements:

- **Backend**: `crates/qa-pms-jira/src/tickets.rs`
- **API**: `GET /api/v1/tickets`
- **Frontend**: `frontend/src/pages/Tickets/`

### Technical Implementation Details

#### Jira Tickets Client

```rust
// crates/qa-pms-jira/src/tickets.rs
use crate::JiraClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraTicket {
    pub key: String,
    pub id: String,
    pub fields: TicketFields,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketFields {
    pub summary: String,
    pub description: Option<String>,
    pub status: StatusField,
    pub priority: Option<PriorityField>,
    pub assignee: Option<UserField>,
    pub reporter: Option<UserField>,
    pub created: String,
    pub updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusField {
    pub name: String,
    #[serde(rename = "statusCategory")]
    pub status_category: StatusCategory,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCategory {
    pub key: String, // "new", "indeterminate", "done"
    pub color_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriorityField {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserField {
    pub display_name: String,
    pub email_address: Option<String>,
    pub avatar_urls: Option<AvatarUrls>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarUrls {
    #[serde(rename = "24x24")]
    pub small: Option<String>,
    #[serde(rename = "48x48")]
    pub medium: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub issues: Vec<JiraTicket>,
    pub total: u32,
    pub start_at: u32,
    pub max_results: u32,
}

pub struct TicketFilters {
    pub statuses: Vec<String>,
    pub assignee: Option<String>,
    pub project: Option<String>,
}

impl JiraClient {
    /// List tickets with filters using JQL
    pub async fn list_tickets(
        &self,
        filters: &TicketFilters,
        start_at: u32,
        max_results: u32,
    ) -> Result<SearchResponse> {
        let jql = self.build_jql(filters);
        
        let url = format!(
            "{}/rest/api/3/search?jql={}&startAt={}&maxResults={}&fields=summary,status,priority,assignee,reporter,created,updated,description",
            self.base_url,
            urlencoding::encode(&jql),
            start_at,
            max_results,
        );

        let response = self
            .http_client
            .get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?
            .error_for_status()?
            .json::<SearchResponse>()
            .await?;

        Ok(response)
    }

    fn build_jql(&self, filters: &TicketFilters) -> String {
        let mut clauses = Vec::new();

        if let Some(project) = &filters.project {
            clauses.push(format!("project = {}", project));
        }

        if !filters.statuses.is_empty() {
            let statuses = filters
                .statuses
                .iter()
                .map(|s| format!("\"{}\"", s))
                .collect::<Vec<_>>()
                .join(", ");
            clauses.push(format!("status IN ({})", statuses));
        }

        if let Some(assignee) = &filters.assignee {
            clauses.push(format!("assignee = \"{}\"", assignee));
        }

        if clauses.is_empty() {
            "ORDER BY updated DESC".to_string()
        } else {
            format!("{} ORDER BY updated DESC", clauses.join(" AND "))
        }
    }
}
```

#### API Endpoint

```rust
// crates/qa-pms-api/src/routes/tickets.rs
use axum::{
    extract::{Query, State},
    Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_tickets))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTicketsQuery {
    pub status: Option<String>,      // Comma-separated
    pub assignee: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketListResponse {
    pub tickets: Vec<TicketSummary>,
    pub total: u32,
    pub page: u32,
    pub page_size: u32,
    pub has_more: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketSummary {
    pub key: String,
    pub title: String,
    pub status: String,
    pub status_color: String,
    pub priority: Option<String>,
    pub priority_color: String,
    pub assignee_name: Option<String>,
    pub assignee_avatar: Option<String>,
    pub updated_at: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/tickets",
    params(
        ("status" = Option<String>, Query, description = "Comma-separated status filters"),
        ("assignee" = Option<String>, Query, description = "Assignee email/username"),
        ("page" = Option<u32>, Query, description = "Page number (default: 1)"),
        ("page_size" = Option<u32>, Query, description = "Items per page (default: 20)"),
    ),
    responses(
        (status = 200, description = "Ticket list", body = TicketListResponse),
        (status = 401, description = "Not authenticated with Jira"),
    ),
    tag = "Tickets"
)]
async fn list_tickets(
    State(state): State<AppState>,
    Query(query): Query<ListTicketsQuery>,
) -> Result<Json<TicketListResponse>, ApiError> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let start_at = (page - 1) * page_size;

    let statuses = query
        .status
        .map(|s| s.split(',').map(String::from).collect())
        .unwrap_or_else(|| state.user_config.ticket_states.clone());

    let filters = TicketFilters {
        statuses,
        assignee: query.assignee.or_else(|| Some(state.user_config.jira_email.clone())),
        project: None,
    };

    let start = std::time::Instant::now();
    let response = state.jira_client.list_tickets(&filters, start_at, page_size).await?;
    let duration = start.elapsed();
    
    if duration.as_secs() > 2 {
        tracing::warn!("Slow ticket list query: {:?}", duration);
    }

    let tickets = response
        .issues
        .into_iter()
        .map(|t| TicketSummary {
            key: t.key,
            title: t.fields.summary,
            status: t.fields.status.name,
            status_color: t.fields.status.status_category.color_name,
            priority: t.fields.priority.map(|p| p.name),
            priority_color: get_priority_color(&t.fields.priority),
            assignee_name: t.fields.assignee.map(|a| a.display_name),
            assignee_avatar: t.fields.assignee.and_then(|a| a.avatar_urls.and_then(|av| av.small)),
            updated_at: t.fields.updated,
        })
        .collect();

    Ok(Json(TicketListResponse {
        tickets,
        total: response.total,
        page,
        page_size,
        has_more: start_at + page_size < response.total,
    }))
}

fn get_priority_color(priority: &Option<PriorityField>) -> String {
    match priority.as_ref().map(|p| p.name.as_str()) {
        Some("Highest") | Some("Blocker") => "error".to_string(),
        Some("High") => "warning".to_string(),
        Some("Medium") => "primary".to_string(),
        Some("Low") | Some("Lowest") => "neutral".to_string(),
        _ => "neutral".to_string(),
    }
}
```

#### Frontend Components

```tsx
// frontend/src/pages/Tickets/TicketsPage.tsx
import { useQuery } from "@tanstack/react-query";
import { TicketCard } from "./TicketCard";
import { TicketCardSkeleton } from "./TicketCardSkeleton";
import { TicketFilters } from "./TicketFilters";
import { EmptyTicketList } from "./EmptyTicketList";
import { useSearchParams } from "react-router-dom";

export function TicketsPage() {
  const [searchParams, setSearchParams] = useSearchParams();
  
  const status = searchParams.get("status") || undefined;
  const page = parseInt(searchParams.get("page") || "1");

  const { data, isLoading, error } = useQuery({
    queryKey: ["tickets", status, page],
    queryFn: () => fetchTickets({ status, page }),
  });

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-semibold text-neutral-900">My Tickets</h1>
        <TicketFilters 
          currentStatus={status} 
          onStatusChange={(s) => setSearchParams({ status: s, page: "1" })}
        />
      </div>

      {isLoading ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {Array.from({ length: 6 }).map((_, i) => (
            <TicketCardSkeleton key={i} />
          ))}
        </div>
      ) : error ? (
        <div className="text-center py-12 text-error-500">
          Failed to load tickets. Please try again.
        </div>
      ) : data?.tickets.length === 0 ? (
        <EmptyTicketList />
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {data?.tickets.map((ticket) => (
            <TicketCard key={ticket.key} ticket={ticket} />
          ))}
        </div>
      )}

      {/* Pagination */}
      {data && data.hasMore && (
        <div className="flex justify-center gap-2">
          <button
            disabled={page === 1}
            onClick={() => setSearchParams({ status: status || "", page: String(page - 1) })}
            className="px-4 py-2 border rounded-lg disabled:opacity-50"
          >
            Previous
          </button>
          <button
            disabled={!data.hasMore}
            onClick={() => setSearchParams({ status: status || "", page: String(page + 1) })}
            className="px-4 py-2 border rounded-lg disabled:opacity-50"
          >
            Next
          </button>
        </div>
      )}
    </div>
  );
}

async function fetchTickets(params: { status?: string; page: number }) {
  const url = new URL("/api/v1/tickets", window.location.origin);
  if (params.status) url.searchParams.set("status", params.status);
  url.searchParams.set("page", String(params.page));
  
  const res = await fetch(url);
  if (!res.ok) throw new Error("Failed to fetch tickets");
  return res.json();
}
```

```tsx
// frontend/src/pages/Tickets/TicketCard.tsx
import { Link } from "react-router-dom";

interface TicketCardProps {
  ticket: {
    key: string;
    title: string;
    status: string;
    statusColor: string;
    priority: string | null;
    priorityColor: string;
    assigneeName: string | null;
    assigneeAvatar: string | null;
    updatedAt: string;
  };
}

const PRIORITY_COLORS = {
  error: "bg-error-100 text-error-700",
  warning: "bg-warning-100 text-warning-700",
  primary: "bg-primary-100 text-primary-700",
  neutral: "bg-neutral-100 text-neutral-700",
};

export function TicketCard({ ticket }: TicketCardProps) {
  return (
    <Link
      to={`/tickets/${ticket.key}`}
      className="block p-4 bg-white border border-neutral-200 rounded-lg hover:border-primary-300 
                 hover:shadow-md transition-all"
    >
      {/* Header */}
      <div className="flex items-start justify-between mb-2">
        <span className="text-sm font-mono text-primary-600">{ticket.key}</span>
        {ticket.priority && (
          <span className={`text-xs px-2 py-0.5 rounded ${PRIORITY_COLORS[ticket.priorityColor as keyof typeof PRIORITY_COLORS]}`}>
            {ticket.priority}
          </span>
        )}
      </div>

      {/* Title */}
      <h3 className="font-medium text-neutral-900 line-clamp-2 mb-3">
        {ticket.title}
      </h3>

      {/* Footer */}
      <div className="flex items-center justify-between text-sm">
        <span className="px-2 py-1 bg-neutral-100 text-neutral-600 rounded">
          {ticket.status}
        </span>
        {ticket.assigneeName && (
          <div className="flex items-center gap-2">
            {ticket.assigneeAvatar && (
              <img 
                src={ticket.assigneeAvatar} 
                alt="" 
                className="w-5 h-5 rounded-full"
              />
            )}
            <span className="text-neutral-500">{ticket.assigneeName}</span>
          </div>
        )}
      </div>
    </Link>
  );
}
```

```tsx
// frontend/src/pages/Tickets/TicketCardSkeleton.tsx
export function TicketCardSkeleton() {
  return (
    <div className="p-4 bg-white border border-neutral-200 rounded-lg animate-pulse">
      <div className="flex items-start justify-between mb-2">
        <div className="h-4 w-20 bg-neutral-200 rounded" />
        <div className="h-4 w-12 bg-neutral-200 rounded" />
      </div>
      <div className="h-5 w-full bg-neutral-200 rounded mb-2" />
      <div className="h-5 w-3/4 bg-neutral-200 rounded mb-3" />
      <div className="flex items-center justify-between">
        <div className="h-6 w-24 bg-neutral-200 rounded" />
        <div className="h-5 w-20 bg-neutral-200 rounded" />
      </div>
    </div>
  );
}
```

```tsx
// frontend/src/pages/Tickets/EmptyTicketList.tsx
import { MagnifyingGlassIcon } from "@radix-ui/react-icons";

export function EmptyTicketList() {
  return (
    <div className="text-center py-12">
      <MagnifyingGlassIcon className="w-12 h-12 text-neutral-300 mx-auto mb-4" />
      <h3 className="text-lg font-medium text-neutral-700 mb-2">
        No tickets found
      </h3>
      <p className="text-neutral-500 max-w-md mx-auto">
        There are no tickets matching your current filters. Try adjusting your 
        filter settings or check back later for new tickets.
      </p>
    </div>
  );
}
```

### Project Structure Notes

Files to create:
```
crates/qa-pms-jira/src/
└── tickets.rs          # Jira tickets client

crates/qa-pms-api/src/routes/
└── tickets.rs          # Tickets API endpoint

frontend/src/pages/Tickets/
├── TicketsPage.tsx     # Main page
├── TicketCard.tsx      # Ticket card component
├── TicketCardSkeleton.tsx  # Loading skeleton
├── TicketFilters.tsx   # Filter controls
├── EmptyTicketList.tsx # Empty state
└── index.ts            # Barrel export
```

### Testing Notes

- Unit test JQL construction with various filters
- Unit test API response mapping
- Integration test: Fetch tickets with mock Jira API
- Frontend: Test loading, empty, and error states
- Performance test: Verify < 2s load time

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 3.2]
- [Source: Jira REST API v3 Documentation]
- [Source: _bmad-output/planning-artifacts/prd.md#NFR-PERF-01]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### Debug Log References

- cargo test: 50 tests passing (20 in qa-pms-jira, 12 in qa-pms-core, 9 in qa-pms-config, 9 in qa-pms-api)
- npm run build: Frontend build successful (142 modules, 374KB JS bundle)

### Completion Notes List

- Created `qa-pms-jira/src/tickets.rs` with JiraTicketsClient, JiraTicket types, and JQL query builder
- Added `qa-pms-api/src/routes/tickets.rs` with GET /api/v1/tickets endpoint
- Created frontend components: TicketsPage, TicketCard, TicketCardSkeleton, TicketFilters, EmptyTicketList
- Added performance monitoring with load_time_ms in API response
- Added ServiceUnavailable error type to qa-pms-core for proper 503 responses
- Integrated with wizardStore for ticket state filters from user config
- Used Radix UI DropdownMenu for filter controls

### File List

**Backend (Rust):**
- `crates/qa-pms-jira/src/tickets.rs` (NEW)
- `crates/qa-pms-jira/src/lib.rs` (MODIFIED - added tickets module)
- `crates/qa-pms-api/src/routes/tickets.rs` (NEW)
- `crates/qa-pms-api/src/routes/mod.rs` (MODIFIED - added tickets router)
- `crates/qa-pms-api/src/app.rs` (MODIFIED - added tickets route)
- `crates/qa-pms-api/src/routes/setup.rs` (MODIFIED - added cloud_id, access_token to JiraTestRequest)
- `crates/qa-pms-api/Cargo.toml` (MODIFIED - added qa-pms-jira dependency)
- `crates/qa-pms-core/src/error.rs` (MODIFIED - added ServiceUnavailable, updated Unauthorized)

**Frontend (React):**
- `frontend/src/pages/Tickets/TicketsPage.tsx` (NEW)
- `frontend/src/pages/Tickets/TicketCard.tsx` (NEW)
- `frontend/src/pages/Tickets/TicketCardSkeleton.tsx` (NEW)
- `frontend/src/pages/Tickets/TicketFilters.tsx` (NEW)
- `frontend/src/pages/Tickets/EmptyTicketList.tsx` (NEW)
- `frontend/src/pages/Tickets/index.ts` (NEW)
- `frontend/src/App.tsx` (MODIFIED - imported TicketsPage)
