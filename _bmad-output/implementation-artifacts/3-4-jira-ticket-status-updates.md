# Story 3.4: Jira Ticket Status Updates

Status: done

## Story

As a QA (Ana),
I want to update ticket status from the framework,
So that I don't need to switch to Jira.

## Acceptance Criteria

1. **Given** user is viewing a ticket
   **When** user changes the status
   **Then** status is updated in Jira via API

2. **Given** status update is initiated
   **When** request is sent
   **Then** local display updates immediately (optimistic update)

3. **Given** status update succeeds
   **When** response is received
   **Then** success toast confirms "Status updated"

4. **Given** status update fails
   **When** error occurs
   **Then** error message shows with retry option

5. **Given** API call is made
   **When** network issues occur
   **Then** API call uses retry with exponential backoff (NFR-REL-03)

## Tasks / Subtasks

- [x] Task 1: Extend Jira client for status transitions (AC: #1, #5)
  - [x] 1.1: Add `get_transitions()` method to JiraClient
  - [x] 1.2: Add `transition_ticket()` method
  - [x] 1.3: Implement exponential backoff retry
  - [x] 1.4: Map transition errors to domain types

- [x] Task 2: Create status update API endpoint (AC: #1)
  - [x] 2.1: Add `POST /api/v1/tickets/:key/transition` endpoint
  - [x] 2.2: Accept transition ID in request body
  - [x] 2.3: Return updated ticket status

- [x] Task 3: Create available transitions endpoint (AC: #1)
  - [x] 3.1: Add `GET /api/v1/tickets/:key/transitions` endpoint
  - [x] 3.2: Return list of available transitions
  - [x] 3.3: Include transition names and IDs

- [x] Task 4: Create StatusSelector component (AC: #1, #2)
  - [x] 4.1: Create dropdown with available statuses
  - [x] 4.2: Show current status as selected
  - [x] 4.3: Load available transitions on mount
  - [x] 4.4: Implement optimistic UI update

- [x] Task 5: Implement toast notifications (AC: #3, #4)
  - [x] 5.1: Add success toast "Status updated"
  - [x] 5.2: Add error toast with message
  - [x] 5.3: Add retry button on error toast

- [x] Task 6: Implement optimistic update (AC: #2)
  - [x] 6.1: Update local state immediately on selection
  - [x] 6.2: Revert on API failure
  - [x] 6.3: Show loading indicator during transition

- [x] Task 7: Create retry mechanism for frontend (AC: #4)
  - [x] 7.1: Store failed transition for retry
  - [x] 7.2: Provide retry callback
  - [x] 7.3: Clear retry state on success

## Dev Notes

### Architecture Alignment

This story implements **Jira Ticket Status Updates** per Epic 3 requirements:

- **Backend**: `crates/qa-pms-jira/src/tickets.rs` (extend)
- **API**: `POST /api/v1/tickets/:key/transition`
- **Frontend**: `StatusSelector` component with optimistic updates

### Technical Implementation Details

#### Extended Jira Client

```rust
// crates/qa-pms-jira/src/tickets.rs (extend)
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transition {
    pub id: String,
    pub name: String,
    pub to: TransitionTarget,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitionTarget {
    pub id: String,
    pub name: String,
    pub status_category: StatusCategory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransitionsResponse {
    pub transitions: Vec<Transition>,
}

impl JiraClient {
    /// Get available transitions for a ticket
    pub async fn get_transitions(&self, key: &str) -> Result<Vec<Transition>> {
        let url = format!(
            "{}/rest/api/3/issue/{}/transitions",
            self.base_url, key
        );

        let response = self
            .http_client
            .get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?
            .error_for_status()?
            .json::<TransitionsResponse>()
            .await?;

        Ok(response.transitions)
    }

    /// Transition a ticket to a new status with retry
    pub async fn transition_ticket(
        &self,
        key: &str,
        transition_id: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/rest/api/3/issue/{}/transitions",
            self.base_url, key
        );

        let body = serde_json::json!({
            "transition": {
                "id": transition_id
            }
        });

        // Retry with exponential backoff: 1s, 2s, 4s
        let mut attempt = 0;
        let max_attempts = 3;
        let base_delay = Duration::from_secs(1);

        loop {
            attempt += 1;
            
            let result = self
                .http_client
                .post(&url)
                .bearer_auth(&self.access_token)
                .json(&body)
                .send()
                .await;

            match result {
                Ok(response) if response.status().is_success() => {
                    return Ok(());
                }
                Ok(response) if response.status().is_server_error() && attempt < max_attempts => {
                    let delay = base_delay * 2u32.pow(attempt - 1);
                    tracing::warn!(
                        "Jira transition failed (attempt {}), retrying in {:?}",
                        attempt, delay
                    );
                    sleep(delay).await;
                }
                Ok(response) => {
                    let error_text = response.text().await.unwrap_or_default();
                    return Err(anyhow::anyhow!("Transition failed: {}", error_text));
                }
                Err(e) if attempt < max_attempts => {
                    let delay = base_delay * 2u32.pow(attempt - 1);
                    tracing::warn!(
                        "Network error (attempt {}): {}, retrying in {:?}",
                        attempt, e, delay
                    );
                    sleep(delay).await;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }
}
```

#### API Endpoints

```rust
// crates/qa-pms-api/src/routes/tickets.rs (extend)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitionRequest {
    pub transition_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransitionInfo {
    pub id: String,
    pub name: String,
    pub to_status: String,
    pub to_status_color: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/tickets/{key}/transitions",
    params(
        ("key" = String, Path, description = "Jira ticket key")
    ),
    responses(
        (status = 200, description = "Available transitions", body = Vec<TransitionInfo>),
    ),
    tag = "Tickets"
)]
async fn get_transitions(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<Vec<TransitionInfo>>, ApiError> {
    let transitions = state.jira_client.get_transitions(&key).await?;

    let infos = transitions
        .into_iter()
        .map(|t| TransitionInfo {
            id: t.id,
            name: t.name,
            to_status: t.to.name,
            to_status_color: t.to.status_category.color_name,
        })
        .collect();

    Ok(Json(infos))
}

#[utoipa::path(
    post,
    path = "/api/v1/tickets/{key}/transition",
    params(
        ("key" = String, Path, description = "Jira ticket key")
    ),
    request_body = TransitionRequest,
    responses(
        (status = 200, description = "Transition successful"),
        (status = 400, description = "Invalid transition"),
        (status = 500, description = "Transition failed"),
    ),
    tag = "Tickets"
)]
async fn transition_ticket(
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(req): Json<TransitionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .jira_client
        .transition_ticket(&key, &req.transition_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to transition ticket {}: {}", key, e);
            ApiError::Internal(e)
        })?;

    Ok(StatusCode::OK)
}
```

#### Frontend Components

```tsx
// frontend/src/components/StatusSelector.tsx
import { useState, useEffect } from "react";
import * as Select from "@radix-ui/react-select";
import { CheckIcon, ChevronDownIcon } from "@radix-ui/react-icons";
import { useToast } from "@/hooks/useToast";

interface StatusSelectorProps {
  ticketKey: string;
  currentStatus: string;
  currentStatusColor: string;
  onStatusChange?: (newStatus: string) => void;
}

interface Transition {
  id: string;
  name: string;
  toStatus: string;
  toStatusColor: string;
}

export function StatusSelector({
  ticketKey,
  currentStatus,
  currentStatusColor,
  onStatusChange,
}: StatusSelectorProps) {
  const [transitions, setTransitions] = useState<Transition[]>([]);
  const [displayStatus, setDisplayStatus] = useState(currentStatus);
  const [isLoading, setIsLoading] = useState(false);
  const [failedTransition, setFailedTransition] = useState<Transition | null>(null);
  const { toast } = useToast();

  // Load available transitions
  useEffect(() => {
    fetch(`/api/v1/tickets/${ticketKey}/transitions`)
      .then((res) => res.json())
      .then(setTransitions)
      .catch((err) => {
        console.error("Failed to load transitions:", err);
      });
  }, [ticketKey]);

  const handleTransition = async (transition: Transition) => {
    const previousStatus = displayStatus;
    
    // Optimistic update
    setDisplayStatus(transition.toStatus);
    setIsLoading(true);
    setFailedTransition(null);

    try {
      const response = await fetch(`/api/v1/tickets/${ticketKey}/transition`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ transitionId: transition.id }),
      });

      if (!response.ok) {
        throw new Error("Transition failed");
      }

      toast({
        title: "Status updated",
        description: `Ticket moved to "${transition.toStatus}"`,
        variant: "success",
      });

      onStatusChange?.(transition.toStatus);
    } catch (error) {
      // Revert optimistic update
      setDisplayStatus(previousStatus);
      setFailedTransition(transition);

      toast({
        title: "Failed to update status",
        description: "Please try again",
        variant: "error",
        action: {
          label: "Retry",
          onClick: () => handleTransition(transition),
        },
      });
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <Select.Root
      value={displayStatus}
      onValueChange={(value) => {
        const transition = transitions.find((t) => t.toStatus === value);
        if (transition) {
          handleTransition(transition);
        }
      }}
      disabled={isLoading}
    >
      <Select.Trigger
        className={`
          inline-flex items-center gap-2 px-3 py-1.5 rounded border
          ${isLoading ? "opacity-50 cursor-wait" : "cursor-pointer hover:bg-neutral-50"}
          transition-colors
        `}
      >
        <Select.Value>{displayStatus}</Select.Value>
        <Select.Icon>
          <ChevronDownIcon className="w-4 h-4" />
        </Select.Icon>
      </Select.Trigger>

      <Select.Portal>
        <Select.Content
          className="bg-white rounded-lg shadow-lg border border-neutral-200 py-1 min-w-[160px]"
          position="popper"
          sideOffset={4}
        >
          <Select.Viewport>
            {/* Current status */}
            <Select.Item
              value={displayStatus}
              className="px-3 py-2 text-sm flex items-center gap-2 cursor-default bg-neutral-50"
            >
              <Select.ItemIndicator>
                <CheckIcon className="w-4 h-4" />
              </Select.ItemIndicator>
              <Select.ItemText>{displayStatus}</Select.ItemText>
              <span className="text-xs text-neutral-400 ml-auto">Current</span>
            </Select.Item>

            <Select.Separator className="h-px bg-neutral-200 my-1" />

            {/* Available transitions */}
            {transitions.map((transition) => (
              <Select.Item
                key={transition.id}
                value={transition.toStatus}
                className="px-3 py-2 text-sm flex items-center gap-2 cursor-pointer hover:bg-neutral-100"
              >
                <Select.ItemIndicator>
                  <CheckIcon className="w-4 h-4" />
                </Select.ItemIndicator>
                <Select.ItemText>
                  {transition.name} → {transition.toStatus}
                </Select.ItemText>
              </Select.Item>
            ))}

            {transitions.length === 0 && (
              <div className="px-3 py-2 text-sm text-neutral-500">
                No transitions available
              </div>
            )}
          </Select.Viewport>
        </Select.Content>
      </Select.Portal>
    </Select.Root>
  );
}
```

#### Toast Hook

```tsx
// frontend/src/hooks/useToast.ts
import { create } from "zustand";

interface Toast {
  id: string;
  title: string;
  description?: string;
  variant: "success" | "error" | "warning" | "info";
  action?: {
    label: string;
    onClick: () => void;
  };
}

interface ToastStore {
  toasts: Toast[];
  toast: (toast: Omit<Toast, "id">) => void;
  dismiss: (id: string) => void;
}

export const useToast = create<ToastStore>((set) => ({
  toasts: [],
  toast: (toast) => {
    const id = Math.random().toString(36).slice(2);
    set((state) => ({
      toasts: [...state.toasts, { ...toast, id }],
    }));

    // Auto-dismiss after 5 seconds
    setTimeout(() => {
      set((state) => ({
        toasts: state.toasts.filter((t) => t.id !== id),
      }));
    }, 5000);
  },
  dismiss: (id) => {
    set((state) => ({
      toasts: state.toasts.filter((t) => t.id !== id),
    }));
  },
}));
```

### Retry Logic (NFR-REL-03)

```
Attempt 1: Immediate
Attempt 2: Wait 1 second
Attempt 3: Wait 2 seconds
Attempt 4: Wait 4 seconds (max)
```

### Project Structure Notes

Files to create/extend:
```
crates/qa-pms-jira/src/
└── tickets.rs              # Extend with transitions

crates/qa-pms-api/src/routes/
└── tickets.rs              # Add transition endpoints

frontend/src/
├── components/
│   └── StatusSelector.tsx  # Status dropdown
└── hooks/
    └── useToast.ts         # Toast notifications
```

### Testing Notes

- Unit test retry logic with mocked failures
- Unit test optimistic update and revert
- Integration test: Full transition flow
- Test toast notifications appear correctly
- Test error handling and retry functionality

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 3.4]
- [Source: Jira REST API - Transitions]
- [Source: _bmad-output/planning-artifacts/prd.md#NFR-REL-03]

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### Debug Log References

N/A

### Completion Notes List

- Extended `JiraTicketsClient` with `get_transitions()` and `transition_ticket()` methods
- Added types: `Transition`, `TransitionTarget`, `TransitionsResponse`, `TransitionRequest`, `TransitionId`
- Implemented exponential backoff retry per NFR-REL-03 (1s, 2s delays, max 3 attempts)
- Created `GET /api/v1/tickets/{key}/transitions` endpoint
- Created `POST /api/v1/tickets/{key}/transition` endpoint with validation
- Created `useToast` hook with Zustand store for toast notifications
- Created `ToastProvider` component using Radix UI Toast with success/error/warning/info variants
- Created `StatusSelector` component with:
  - Radix UI Select dropdown
  - Optimistic UI updates
  - Loading spinner during transition
  - Automatic revert on failure
  - Retry action on error toast
- Integrated StatusSelector in TicketMetadata component
- Added ToastProvider wrapper in App.tsx
- Used Context7 to reference Jira REST API documentation for transitions
- All 43 backend tests passing, frontend build successful

### File List

**Backend (Rust):**
- `crates/qa-pms-jira/src/tickets.rs` - Extended with get_transitions(), transition_ticket(), Transition types
- `crates/qa-pms-jira/src/lib.rs` - Updated exports
- `crates/qa-pms-api/src/routes/tickets.rs` - Added transition endpoints
- `crates/qa-pms-api/src/routes/mod.rs` - Updated OpenAPI docs

**Frontend (React/TypeScript):**
- `frontend/src/hooks/useToast.ts` - Toast store with Zustand
- `frontend/src/hooks/index.ts` - Updated exports
- `frontend/src/components/ui/ToastProvider.tsx` - Toast UI with Radix
- `frontend/src/pages/Tickets/StatusSelector.tsx` - Status dropdown with optimistic updates
- `frontend/src/pages/Tickets/TicketMetadata.tsx` - Integrated StatusSelector
- `frontend/src/pages/Tickets/TicketDetailPage.tsx` - Added status change handler
- `frontend/src/pages/Tickets/index.ts` - Updated exports
- `frontend/src/App.tsx` - Added ToastProvider
- `frontend/src/index.css` - Added toast animations
- `frontend/package.json` - Added @radix-ui/react-select
