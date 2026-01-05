# Story 3.3: Jira Ticket Detail View

Status: done

## Story

As a QA (Ana),
I want to see full ticket details,
So that I understand what needs to be tested.

## Acceptance Criteria

1. **Given** user clicks on a ticket in the list
   **When** the ticket detail loads
   **Then** full ticket key and title are displayed

2. **Given** ticket detail is displayed
   **When** description exists
   **Then** description is rendered as markdown

3. **Given** ticket detail is displayed
   **When** acceptance criteria exist
   **Then** acceptance criteria are highlighted if in Gherkin format

4. **Given** ticket detail is displayed
   **When** metadata is shown
   **Then** priority, status, assignee, reporter, created/updated dates are visible

5. **Given** ticket detail is displayed
   **When** comments exist
   **Then** latest 10 comments are shown

6. **Given** ticket detail is displayed
   **When** attachments exist
   **Then** attachments list is shown

7. **Given** ticket detail view
   **When** performance is measured
   **Then** detail view loads in < 2s

8. **Given** ticket detail is displayed
   **When** user wants to start testing
   **Then** "Start Workflow" button is prominently displayed

9. **Given** user is viewing ticket detail
   **When** user wants to return
   **Then** back navigation returns to list

## Tasks / Subtasks

- [x] Task 1: Extend Jira client for ticket details (AC: #1, #4, #5, #6)
  - [x] 1.1: Add `get_ticket()` method to JiraClient
  - [x] 1.2: Include comments in API request
  - [x] 1.3: Include attachments in API request
  - [x] 1.4: Parse all required fields

- [x] Task 2: Create ticket detail API endpoint (AC: #1, #7)
  - [x] 2.1: Add `GET /api/v1/tickets/:key` endpoint
  - [x] 2.2: Return full ticket data with comments
  - [x] 2.3: Add performance timing

- [x] Task 3: Create TicketDetailPage (AC: #1, #9)
  - [x] 3.1: Create `TicketDetailPage.tsx`
  - [x] 3.2: Fetch ticket by key from URL param
  - [x] 3.3: Add back navigation button

- [x] Task 4: Implement description rendering (AC: #2, #3)
  - [x] 4.1: Use dangerouslySetInnerHTML for ADF-converted HTML
  - [x] 4.2: Create `MarkdownRenderer.tsx` component
  - [x] 4.3: Detect and highlight Gherkin syntax
  - [x] 4.4: Style code blocks and lists

- [x] Task 5: Create ticket metadata display (AC: #4)
  - [x] 5.1: Create `TicketMetadata.tsx` component
  - [x] 5.2: Display priority with color badge
  - [x] 5.3: Display status with category color
  - [x] 5.4: Display dates in relative format

- [x] Task 6: Create comments section (AC: #5)
  - [x] 6.1: Create `TicketComments.tsx` component
  - [x] 6.2: Display latest 10 comments
  - [x] 6.3: Show author, date, and content
  - [x] 6.4: Render comment body as markdown

- [x] Task 7: Create attachments section (AC: #6)
  - [x] 7.1: Create `TicketAttachments.tsx` component
  - [x] 7.2: Display file name, size, type
  - [x] 7.3: Add download links

- [x] Task 8: Add Start Workflow button (AC: #8)
  - [x] 8.1: Add prominent CTA button
  - [x] 8.2: Navigate to workflow start page
  - [x] 8.3: Pass ticket context to workflow

## Dev Notes

### Architecture Alignment

This story implements **Jira Ticket Detail View** per Epic 3 requirements:

- **Backend**: `crates/qa-pms-jira/src/tickets.rs` (extend)
- **API**: `GET /api/v1/tickets/:key`
- **Frontend**: `frontend/src/pages/Tickets/TicketDetailPage.tsx`

### Technical Implementation Details

#### Extended Jira Client

```rust
// crates/qa-pms-jira/src/tickets.rs (extend)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketDetail {
    pub key: String,
    pub id: String,
    pub fields: TicketDetailFields,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketDetailFields {
    pub summary: String,
    pub description: Option<AtlassianDocument>,
    pub status: StatusField,
    pub priority: Option<PriorityField>,
    pub assignee: Option<UserField>,
    pub reporter: Option<UserField>,
    pub created: String,
    pub updated: String,
    pub comment: Option<CommentContainer>,
    pub attachment: Option<Vec<Attachment>>,
    pub labels: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AtlassianDocument {
    pub content: Vec<serde_json::Value>,
    // Atlassian Document Format (ADF)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentContainer {
    pub comments: Vec<Comment>,
    pub total: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub author: UserField,
    pub body: AtlassianDocument,
    pub created: String,
    pub updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub id: String,
    pub filename: String,
    pub mime_type: String,
    pub size: u64,
    pub content: String, // Download URL
    pub created: String,
}

impl JiraClient {
    /// Get ticket details by key
    pub async fn get_ticket(&self, key: &str) -> Result<TicketDetail> {
        let url = format!(
            "{}/rest/api/3/issue/{}?fields=summary,description,status,priority,assignee,reporter,created,updated,comment,attachment,labels&expand=renderedFields",
            self.base_url,
            key
        );

        let response = self
            .http_client
            .get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?
            .error_for_status()?
            .json::<TicketDetail>()
            .await?;

        Ok(response)
    }
}
```

#### API Endpoint

```rust
// crates/qa-pms-api/src/routes/tickets.rs (extend)
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketDetailResponse {
    pub key: String,
    pub title: String,
    pub description_html: Option<String>,
    pub description_raw: Option<String>,
    pub status: String,
    pub status_color: String,
    pub priority: Option<String>,
    pub priority_color: String,
    pub assignee: Option<UserInfo>,
    pub reporter: Option<UserInfo>,
    pub created_at: String,
    pub updated_at: String,
    pub comments: Vec<CommentInfo>,
    pub attachments: Vec<AttachmentInfo>,
    pub labels: Vec<String>,
    pub has_gherkin: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentInfo {
    pub id: String,
    pub author: UserInfo,
    pub body_html: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentInfo {
    pub id: String,
    pub filename: String,
    pub mime_type: String,
    pub size: u64,
    pub size_human: String,
    pub download_url: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/tickets/{key}",
    params(
        ("key" = String, Path, description = "Jira ticket key (e.g., PROJ-123)")
    ),
    responses(
        (status = 200, description = "Ticket details", body = TicketDetailResponse),
        (status = 404, description = "Ticket not found"),
    ),
    tag = "Tickets"
)]
async fn get_ticket(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<TicketDetailResponse>, ApiError> {
    let start = std::time::Instant::now();
    
    let ticket = state.jira_client.get_ticket(&key).await?;
    
    let duration = start.elapsed();
    if duration.as_secs() > 2 {
        tracing::warn!("Slow ticket detail fetch for {}: {:?}", key, duration);
    }

    let description_raw = adf_to_text(&ticket.fields.description);
    let description_html = adf_to_html(&ticket.fields.description);
    let has_gherkin = description_raw
        .as_ref()
        .map(|d| detect_gherkin(d))
        .unwrap_or(false);

    let comments = ticket
        .fields
        .comment
        .map(|c| c.comments.into_iter().take(10).map(|comment| CommentInfo {
            id: comment.id,
            author: UserInfo {
                name: comment.author.display_name,
                email: comment.author.email_address,
                avatar_url: comment.author.avatar_urls.and_then(|a| a.small),
            },
            body_html: adf_to_html(&Some(comment.body)).unwrap_or_default(),
            created_at: comment.created,
        }).collect())
        .unwrap_or_default();

    let attachments = ticket
        .fields
        .attachment
        .unwrap_or_default()
        .into_iter()
        .map(|a| AttachmentInfo {
            id: a.id,
            filename: a.filename,
            mime_type: a.mime_type,
            size: a.size,
            size_human: humanize_bytes(a.size),
            download_url: a.content,
        })
        .collect();

    Ok(Json(TicketDetailResponse {
        key: ticket.key,
        title: ticket.fields.summary,
        description_html,
        description_raw,
        status: ticket.fields.status.name,
        status_color: ticket.fields.status.status_category.color_name,
        priority: ticket.fields.priority.map(|p| p.name),
        priority_color: get_priority_color(&ticket.fields.priority),
        assignee: ticket.fields.assignee.map(|a| UserInfo {
            name: a.display_name,
            email: a.email_address,
            avatar_url: a.avatar_urls.and_then(|av| av.medium),
        }),
        reporter: ticket.fields.reporter.map(|r| UserInfo {
            name: r.display_name,
            email: r.email_address,
            avatar_url: r.avatar_urls.and_then(|av| av.medium),
        }),
        created_at: ticket.fields.created,
        updated_at: ticket.fields.updated,
        comments,
        attachments,
        labels: ticket.fields.labels,
        has_gherkin,
    }))
}

fn detect_gherkin(text: &str) -> bool {
    let keywords = ["Given", "When", "Then", "And", "But", "Scenario", "Feature"];
    keywords.iter().any(|k| text.contains(k))
}

fn humanize_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    
    if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
```

#### Frontend Components

```tsx
// frontend/src/pages/Tickets/TicketDetailPage.tsx
import { useParams, useNavigate, Link } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { ArrowLeftIcon, PlayIcon } from "@radix-ui/react-icons";
import { MarkdownRenderer } from "@/components/MarkdownRenderer";
import { TicketMetadata } from "./TicketMetadata";
import { TicketComments } from "./TicketComments";
import { TicketAttachments } from "./TicketAttachments";

export function TicketDetailPage() {
  const { key } = useParams<{ key: string }>();
  const navigate = useNavigate();

  const { data: ticket, isLoading, error } = useQuery({
    queryKey: ["ticket", key],
    queryFn: () => fetchTicket(key!),
    enabled: !!key,
  });

  if (isLoading) {
    return <TicketDetailSkeleton />;
  }

  if (error || !ticket) {
    return (
      <div className="text-center py-12">
        <p className="text-error-500">Failed to load ticket details.</p>
        <button onClick={() => navigate(-1)} className="mt-4 text-primary-500">
          Go back
        </button>
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      {/* Back Navigation */}
      <button
        onClick={() => navigate("/tickets")}
        className="flex items-center gap-2 text-neutral-600 hover:text-neutral-900"
      >
        <ArrowLeftIcon className="w-4 h-4" />
        Back to tickets
      </button>

      {/* Header */}
      <div className="flex items-start justify-between">
        <div>
          <span className="text-sm font-mono text-primary-600">{ticket.key}</span>
          <h1 className="text-2xl font-semibold text-neutral-900 mt-1">
            {ticket.title}
          </h1>
        </div>
        
        {/* Start Workflow Button */}
        <Link
          to={`/workflows/start?ticket=${ticket.key}`}
          className="flex items-center gap-2 px-6 py-3 bg-primary-500 text-white rounded-lg
                     hover:bg-primary-600 transition-colors font-medium"
        >
          <PlayIcon className="w-5 h-5" />
          Start Workflow
        </Link>
      </div>

      {/* Metadata */}
      <TicketMetadata ticket={ticket} />

      {/* Labels */}
      {ticket.labels.length > 0 && (
        <div className="flex flex-wrap gap-2">
          {ticket.labels.map((label) => (
            <span
              key={label}
              className="px-2 py-1 bg-primary-50 text-primary-700 text-sm rounded"
            >
              {label}
            </span>
          ))}
        </div>
      )}

      {/* Description */}
      {ticket.descriptionHtml && (
        <div className="bg-white border border-neutral-200 rounded-lg p-6">
          <h2 className="text-lg font-medium text-neutral-900 mb-4">Description</h2>
          <MarkdownRenderer 
            html={ticket.descriptionHtml} 
            highlightGherkin={ticket.hasGherkin}
          />
        </div>
      )}

      {/* Attachments */}
      {ticket.attachments.length > 0 && (
        <TicketAttachments attachments={ticket.attachments} />
      )}

      {/* Comments */}
      {ticket.comments.length > 0 && (
        <TicketComments comments={ticket.comments} />
      )}
    </div>
  );
}

async function fetchTicket(key: string) {
  const res = await fetch(`/api/v1/tickets/${key}`);
  if (!res.ok) throw new Error("Failed to fetch ticket");
  return res.json();
}
```

```tsx
// frontend/src/pages/Tickets/TicketMetadata.tsx
import { formatDistanceToNow } from "date-fns";

interface TicketMetadataProps {
  ticket: {
    status: string;
    statusColor: string;
    priority: string | null;
    priorityColor: string;
    assignee: { name: string; avatarUrl?: string } | null;
    reporter: { name: string; avatarUrl?: string } | null;
    createdAt: string;
    updatedAt: string;
  };
}

export function TicketMetadata({ ticket }: TicketMetadataProps) {
  return (
    <div className="grid grid-cols-2 md:grid-cols-4 gap-4 bg-neutral-50 rounded-lg p-4">
      {/* Status */}
      <div>
        <span className="text-sm text-neutral-500">Status</span>
        <p className="font-medium">{ticket.status}</p>
      </div>

      {/* Priority */}
      <div>
        <span className="text-sm text-neutral-500">Priority</span>
        <p className="font-medium">{ticket.priority || "None"}</p>
      </div>

      {/* Assignee */}
      <div>
        <span className="text-sm text-neutral-500">Assignee</span>
        {ticket.assignee ? (
          <div className="flex items-center gap-2 mt-1">
            {ticket.assignee.avatarUrl && (
              <img src={ticket.assignee.avatarUrl} alt="" className="w-6 h-6 rounded-full" />
            )}
            <span className="font-medium">{ticket.assignee.name}</span>
          </div>
        ) : (
          <p className="text-neutral-400">Unassigned</p>
        )}
      </div>

      {/* Reporter */}
      <div>
        <span className="text-sm text-neutral-500">Reporter</span>
        {ticket.reporter ? (
          <div className="flex items-center gap-2 mt-1">
            {ticket.reporter.avatarUrl && (
              <img src={ticket.reporter.avatarUrl} alt="" className="w-6 h-6 rounded-full" />
            )}
            <span className="font-medium">{ticket.reporter.name}</span>
          </div>
        ) : (
          <p className="text-neutral-400">Unknown</p>
        )}
      </div>

      {/* Created */}
      <div>
        <span className="text-sm text-neutral-500">Created</span>
        <p className="font-medium">
          {formatDistanceToNow(new Date(ticket.createdAt), { addSuffix: true })}
        </p>
      </div>

      {/* Updated */}
      <div>
        <span className="text-sm text-neutral-500">Updated</span>
        <p className="font-medium">
          {formatDistanceToNow(new Date(ticket.updatedAt), { addSuffix: true })}
        </p>
      </div>
    </div>
  );
}
```

```tsx
// frontend/src/components/MarkdownRenderer.tsx
interface MarkdownRendererProps {
  html: string;
  highlightGherkin?: boolean;
}

export function MarkdownRenderer({ html, highlightGherkin }: MarkdownRendererProps) {
  let processedHtml = html;

  // Highlight Gherkin keywords if detected
  if (highlightGherkin) {
    const keywords = ["Given", "When", "Then", "And", "But", "Scenario:", "Feature:"];
    keywords.forEach((keyword) => {
      const regex = new RegExp(`\\b(${keyword})\\b`, "g");
      processedHtml = processedHtml.replace(
        regex,
        `<span class="font-bold text-primary-600">$1</span>`
      );
    });
  }

  return (
    <div
      className="prose prose-neutral max-w-none
                 prose-headings:text-neutral-900
                 prose-p:text-neutral-700
                 prose-code:bg-neutral-100 prose-code:px-1 prose-code:rounded
                 prose-pre:bg-neutral-900 prose-pre:text-neutral-100"
      dangerouslySetInnerHTML={{ __html: processedHtml }}
    />
  );
}
```

### Project Structure Notes

Files to create/extend:
```
crates/qa-pms-jira/src/
└── tickets.rs          # Extend with get_ticket()

crates/qa-pms-api/src/routes/
└── tickets.rs          # Add GET /:key endpoint

frontend/src/
├── pages/Tickets/
│   ├── TicketDetailPage.tsx   # Main detail page
│   ├── TicketMetadata.tsx     # Metadata grid
│   ├── TicketComments.tsx     # Comments section
│   └── TicketAttachments.tsx  # Attachments list
└── components/
    └── MarkdownRenderer.tsx   # HTML/Markdown renderer
```

### Testing Notes

- Unit test ADF to HTML conversion
- Unit test Gherkin detection
- Integration test: Fetch ticket detail with mock API
- Test "Start Workflow" navigation
- Performance test: Verify < 2s load time

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 3.3]
- [Source: Jira REST API - Get Issue]
- [Source: Atlassian Document Format (ADF)]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### Debug Log References

N/A

### Completion Notes List

- Extended `JiraTicketsClient` with `get_ticket()` method for fetching full ticket details
- Added types: `TicketDetail`, `TicketDetailFields`, `Comment`, `CommentContainer`, `Attachment`
- Created `GET /api/v1/tickets/{key}` endpoint with full ADF-to-HTML conversion
- Implemented `adf_to_html()` and `adf_to_text()` functions for Atlassian Document Format conversion
- Added `detect_gherkin()` function for Gherkin syntax detection
- Added `humanize_bytes()` function for human-readable file sizes
- Created `MarkdownRenderer` component with Gherkin keyword highlighting
- Created `TicketMetadata` component showing status, priority, assignee, reporter, dates
- Created `TicketComments` component displaying latest 10 comments with author info
- Created `TicketAttachments` component with file type icons and download links
- Created `TicketDetailPage` with loading skeleton and error handling
- Added "Start Workflow" CTA button linking to `/workflows/start?ticket={key}`
- Installed `date-fns` for date formatting
- All 40 backend tests passing, frontend build successful

### File List

**Backend (Rust):**
- `crates/qa-pms-jira/src/tickets.rs` - Extended with get_ticket(), TicketDetail types
- `crates/qa-pms-jira/src/lib.rs` - Updated exports
- `crates/qa-pms-api/src/routes/tickets.rs` - Added GET /:key endpoint, ADF conversion
- `crates/qa-pms-api/src/routes/mod.rs` - Updated OpenAPI docs

**Frontend (React/TypeScript):**
- `frontend/src/pages/Tickets/types.ts` - Shared types for tickets
- `frontend/src/pages/Tickets/TicketDetailPage.tsx` - Main detail page
- `frontend/src/pages/Tickets/TicketDetailSkeleton.tsx` - Loading skeleton
- `frontend/src/pages/Tickets/TicketMetadata.tsx` - Metadata display
- `frontend/src/pages/Tickets/TicketComments.tsx` - Comments section
- `frontend/src/pages/Tickets/TicketAttachments.tsx` - Attachments section
- `frontend/src/pages/Tickets/index.ts` - Updated exports
- `frontend/src/components/MarkdownRenderer.tsx` - HTML renderer with Gherkin highlighting
- `frontend/src/App.tsx` - Added /tickets/:key route
- `frontend/src/index.css` - Added markdown and Gherkin styles
- `frontend/package.json` - Added date-fns dependency
