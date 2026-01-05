# Story 4.5: Testmo Test Run Creation

Status: done

## Story

As a QA (Ana),
I want to create a Testmo test run from the framework,
So that I can track my test execution.

## Acceptance Criteria

1. **Given** user has found relevant Testmo test cases
   **When** user clicks "Create Test Run"
   **Then** the framework creates a new test run in Testmo

2. **Given** test run is being created
   **When** test cases are specified
   **Then** selected test cases are linked to the run

3. **Given** test run is being created
   **When** ticket context is available
   **Then** run is associated with current Jira ticket (in run name)

4. **Given** test run is created successfully
   **When** response is received
   **Then** link to the created run is returned

5. **Given** test run is created successfully
   **When** user sees result
   **Then** success toast shows with link to Testmo

6. **Given** test run is being created
   **When** naming convention is applied
   **Then** run name follows pattern: "QA-{ticket-key}-{date}"

7. **Given** test run creation fails
   **When** error occurs
   **Then** failure shows error with retry option

## Tasks / Subtasks

- [ ] Task 1: Create test run API endpoint (AC: #1, #2, #4)
  - [ ] 1.1: Create `POST /api/v1/testmo/runs` endpoint
  - [ ] 1.2: Accept project_id, ticket_key, case_ids
  - [ ] 1.3: Validate case_ids exist
  - [ ] 1.4: Return created run with URL

- [ ] Task 2: Implement run naming convention (AC: #3, #6)
  - [ ] 2.1: Generate name from ticket key
  - [ ] 2.2: Add date suffix (YYYY-MM-DD)
  - [ ] 2.3: Allow optional custom name override

- [ ] Task 3: Create frontend test run button (AC: #1)
  - [ ] 3.1: Add "Create Test Run" button to results
  - [ ] 3.2: Show button only for Testmo results
  - [ ] 3.3: Disable button when no cases selected

- [ ] Task 4: Implement case selection (AC: #2)
  - [ ] 4.1: Add checkbox to Testmo results
  - [ ] 4.2: Track selected case IDs in state
  - [ ] 4.3: Show selection count

- [ ] Task 5: Create run creation dialog (AC: #1, #6)
  - [ ] 5.1: Create CreateTestRunDialog component
  - [ ] 5.2: Show selected cases summary
  - [ ] 5.3: Allow run name customization

- [ ] Task 6: Implement success feedback (AC: #4, #5)
  - [ ] 6.1: Show success toast on creation
  - [ ] 6.2: Include link to Testmo run
  - [ ] 6.3: Clear selection after success

- [ ] Task 7: Implement error handling (AC: #7)
  - [ ] 7.1: Show error toast on failure
  - [ ] 7.2: Add retry button
  - [ ] 7.3: Preserve selection on retry

## Dev Notes

### Architecture Alignment

This story implements **Testmo Test Run Creation** per Epic 4 requirements:

- **Backend**: API endpoint in `qa-pms-api`
- **Frontend**: Dialog and selection components
- **Integration**: Uses Testmo client from Story 4.2

### Technical Implementation Details

#### API Endpoint

```rust
// crates/qa-pms-api/src/routes/testmo.rs
use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use qa_pms_testmo::TestmoClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTestRunRequest {
    pub project_id: i64,
    pub ticket_key: String,
    pub case_ids: Vec<i64>,
    pub custom_name: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTestRunResponse {
    pub run_id: i64,
    pub name: String,
    pub url: String,
    pub case_count: usize,
}

/// POST /api/v1/testmo/runs
#[utoipa::path(
    post,
    path = "/api/v1/testmo/runs",
    request_body = CreateTestRunRequest,
    responses(
        (status = 201, description = "Test run created", body = CreateTestRunResponse),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "testmo"
)]
pub async fn create_test_run(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateTestRunRequest>,
) -> Result<(StatusCode, Json<CreateTestRunResponse>), ApiError> {
    let testmo_client = state.testmo_client.as_ref()
        .ok_or_else(|| ApiError::ServiceUnavailable("Testmo not configured".into()))?;

    // Validate request
    if request.case_ids.is_empty() {
        return Err(ApiError::BadRequest("At least one test case is required".into()));
    }

    // Generate run name
    let run_name = match request.custom_name {
        Some(name) if !name.is_empty() => name,
        _ => generate_run_name(&request.ticket_key),
    };

    // Create test run
    let test_run = testmo_client
        .create_test_run(request.project_id, &run_name, &request.case_ids)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to create test run: {}", e)))?;

    // Generate URL to the run
    let base_url = &state.testmo_base_url;
    let url = format!(
        "{}/projects/{}/runs/{}",
        base_url, request.project_id, test_run.id
    );

    tracing::info!(
        run_id = test_run.id,
        ticket = %request.ticket_key,
        cases = request.case_ids.len(),
        "Created Testmo test run"
    );

    Ok((
        StatusCode::CREATED,
        Json(CreateTestRunResponse {
            run_id: test_run.id,
            name: run_name,
            url,
            case_count: request.case_ids.len(),
        }),
    ))
}

fn generate_run_name(ticket_key: &str) -> String {
    let date = Utc::now().format("%Y-%m-%d");
    format!("QA-{}-{}", ticket_key, date)
}
```

#### Route Registration

```rust
// crates/qa-pms-api/src/routes/mod.rs (addition)
use axum::{routing::post, Router};

pub fn testmo_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/testmo/runs", post(testmo::create_test_run))
}
```

#### Frontend - Create Test Run Hook

```tsx
// frontend/src/hooks/useCreateTestRun.ts
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useToast } from "./useToast";

interface CreateTestRunRequest {
  projectId: number;
  ticketKey: string;
  caseIds: number[];
  customName?: string;
}

interface CreateTestRunResponse {
  runId: number;
  name: string;
  url: string;
  caseCount: number;
}

export function useCreateTestRun() {
  const queryClient = useQueryClient();
  const { toast } = useToast();

  return useMutation({
    mutationFn: async (request: CreateTestRunRequest): Promise<CreateTestRunResponse> => {
      const response = await fetch("/api/v1/testmo/runs", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          projectId: request.projectId,
          ticketKey: request.ticketKey,
          caseIds: request.caseIds,
          customName: request.customName,
        }),
      });

      if (!response.ok) {
        const error = await response.json();
        throw new Error(error.message || "Failed to create test run");
      }

      return response.json();
    },
    onSuccess: (data) => {
      toast({
        title: "Test Run Created",
        description: `Created "${data.name}" with ${data.caseCount} test cases`,
        action: {
          label: "Open in Testmo",
          onClick: () => window.open(data.url, "_blank"),
        },
      });
    },
    onError: (error: Error) => {
      toast({
        title: "Failed to Create Test Run",
        description: error.message,
        variant: "error",
      });
    },
  });
}
```

#### Frontend - Create Test Run Dialog

```tsx
// frontend/src/components/search/CreateTestRunDialog.tsx
import { useState } from "react";
import * as Dialog from "@radix-ui/react-dialog";
import { Cross2Icon, CheckIcon, RocketIcon } from "@radix-ui/react-icons";
import { useCreateTestRun } from "@/hooks/useCreateTestRun";

interface SelectedTestCase {
  id: number;
  name: string;
}

interface CreateTestRunDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  ticketKey: string;
  projectId: number;
  selectedCases: SelectedTestCase[];
  onSuccess?: () => void;
}

export function CreateTestRunDialog({
  open,
  onOpenChange,
  ticketKey,
  projectId,
  selectedCases,
  onSuccess,
}: CreateTestRunDialogProps) {
  const [customName, setCustomName] = useState("");
  const createTestRun = useCreateTestRun();

  // Generate default name
  const defaultName = `QA-${ticketKey}-${new Date().toISOString().split("T")[0]}`;

  const handleCreate = async () => {
    try {
      await createTestRun.mutateAsync({
        projectId,
        ticketKey,
        caseIds: selectedCases.map((c) => c.id),
        customName: customName || undefined,
      });

      onOpenChange(false);
      setCustomName("");
      onSuccess?.();
    } catch {
      // Error handled by mutation
    }
  };

  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-black/50 animate-fade-in" />
        <Dialog.Content
          className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
                     bg-white rounded-lg shadow-xl w-full max-w-md p-6
                     animate-scale-in focus:outline-none"
        >
          <Dialog.Title className="text-lg font-semibold text-neutral-900 mb-4">
            Create Test Run
          </Dialog.Title>

          <Dialog.Description className="text-sm text-neutral-500 mb-4">
            Create a new test run in Testmo with the selected test cases.
          </Dialog.Description>

          {/* Run Name Input */}
          <div className="mb-4">
            <label
              htmlFor="runName"
              className="block text-sm font-medium text-neutral-700 mb-1"
            >
              Run Name
            </label>
            <input
              id="runName"
              type="text"
              value={customName}
              onChange={(e) => setCustomName(e.target.value)}
              placeholder={defaultName}
              className="w-full px-3 py-2 border border-neutral-300 rounded-lg
                         focus:outline-none focus:ring-2 focus:ring-primary-500
                         focus:border-transparent"
            />
            <p className="text-xs text-neutral-400 mt-1">
              Leave empty to use default: {defaultName}
            </p>
          </div>

          {/* Selected Cases */}
          <div className="mb-6">
            <h4 className="text-sm font-medium text-neutral-700 mb-2">
              Selected Test Cases ({selectedCases.length})
            </h4>
            <div className="max-h-40 overflow-y-auto border border-neutral-200 rounded-lg">
              {selectedCases.map((testCase) => (
                <div
                  key={testCase.id}
                  className="flex items-center gap-2 px-3 py-2 border-b border-neutral-100 
                             last:border-b-0 text-sm"
                >
                  <CheckIcon className="w-4 h-4 text-success-500 flex-shrink-0" />
                  <span className="truncate">{testCase.name}</span>
                </div>
              ))}
            </div>
          </div>

          {/* Actions */}
          <div className="flex justify-end gap-3">
            <Dialog.Close asChild>
              <button
                className="px-4 py-2 text-sm font-medium text-neutral-600 
                           hover:text-neutral-800 transition-colors"
              >
                Cancel
              </button>
            </Dialog.Close>
            <button
              onClick={handleCreate}
              disabled={createTestRun.isPending || selectedCases.length === 0}
              className="flex items-center gap-2 px-4 py-2 bg-primary-500 text-white 
                         font-medium rounded-lg hover:bg-primary-600 transition-colors
                         disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {createTestRun.isPending ? (
                <>
                  <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                  Creating...
                </>
              ) : (
                <>
                  <RocketIcon className="w-4 h-4" />
                  Create Run
                </>
              )}
            </button>
          </div>

          {/* Close Button */}
          <Dialog.Close asChild>
            <button
              className="absolute top-4 right-4 p-1 text-neutral-400 
                         hover:text-neutral-600 transition-colors"
              aria-label="Close"
            >
              <Cross2Icon className="w-5 h-5" />
            </button>
          </Dialog.Close>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
```

#### Frontend - Selectable Search Results

```tsx
// frontend/src/components/search/SelectableSearchResults.tsx
import { useState, useCallback } from "react";
import * as Checkbox from "@radix-ui/react-checkbox";
import { CheckIcon, RocketIcon } from "@radix-ui/react-icons";
import { SearchResultCard } from "./SearchResultCard";
import { SourceBadge } from "./SourceBadge";
import { CreateTestRunDialog } from "./CreateTestRunDialog";

interface SearchResult {
  source: string;
  id: string;
  name: string;
  description: string | null;
  url: string;
  score: number;
  matches: string[];
}

interface SelectableSearchResultsProps {
  results: SearchResult[];
  ticketKey: string;
  testmoProjectId: number;
}

export function SelectableSearchResults({
  results,
  ticketKey,
  testmoProjectId,
}: SelectableSearchResultsProps) {
  const [selectedIds, setSelectedIds] = useState<Set<string>>(new Set());
  const [dialogOpen, setDialogOpen] = useState(false);

  // Filter Testmo results (only these are selectable)
  const testmoResults = results.filter((r) => r.source === "testmo");
  const otherResults = results.filter((r) => r.source !== "testmo");

  const toggleSelection = useCallback((id: string) => {
    setSelectedIds((prev) => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  }, []);

  const selectAll = useCallback(() => {
    setSelectedIds(new Set(testmoResults.map((r) => r.id)));
  }, [testmoResults]);

  const clearSelection = useCallback(() => {
    setSelectedIds(new Set());
  }, []);

  const selectedCases = testmoResults
    .filter((r) => selectedIds.has(r.id))
    .map((r) => ({ id: parseInt(r.id, 10), name: r.name }));

  return (
    <div className="space-y-6">
      {/* Testmo Results with Selection */}
      {testmoResults.length > 0 && (
        <div>
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-2">
              <SourceBadge source="testmo" />
              <span className="text-sm text-neutral-500">
                {testmoResults.length} result{testmoResults.length !== 1 ? "s" : ""}
              </span>
            </div>
            
            <div className="flex items-center gap-2">
              {selectedIds.size > 0 && (
                <>
                  <span className="text-sm text-primary-600">
                    {selectedIds.size} selected
                  </span>
                  <button
                    onClick={clearSelection}
                    className="text-sm text-neutral-500 hover:text-neutral-700"
                  >
                    Clear
                  </button>
                </>
              )}
              {selectedIds.size < testmoResults.length && (
                <button
                  onClick={selectAll}
                  className="text-sm text-primary-600 hover:text-primary-700"
                >
                  Select all
                </button>
              )}
            </div>
          </div>

          <div className="space-y-2">
            {testmoResults.map((result) => (
              <SelectableResultCard
                key={result.id}
                result={result}
                selected={selectedIds.has(result.id)}
                onToggle={() => toggleSelection(result.id)}
              />
            ))}
          </div>

          {/* Create Run Button */}
          <button
            onClick={() => setDialogOpen(true)}
            disabled={selectedIds.size === 0}
            className="mt-4 w-full flex items-center justify-center gap-2 px-4 py-2.5
                       bg-primary-500 text-white font-medium rounded-lg
                       hover:bg-primary-600 transition-colors
                       disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <RocketIcon className="w-4 h-4" />
            Create Test Run ({selectedIds.size} cases)
          </button>
        </div>
      )}

      {/* Other Results (not selectable) */}
      {otherResults.length > 0 && (
        <div>
          <div className="flex items-center gap-2 mb-3">
            <SourceBadge source="postman" />
            <span className="text-sm text-neutral-500">
              {otherResults.length} result{otherResults.length !== 1 ? "s" : ""}
            </span>
          </div>
          <div className="space-y-3">
            {otherResults.map((result) => (
              <SearchResultCard key={`${result.source}-${result.id}`} result={result} />
            ))}
          </div>
        </div>
      )}

      {/* Create Test Run Dialog */}
      <CreateTestRunDialog
        open={dialogOpen}
        onOpenChange={setDialogOpen}
        ticketKey={ticketKey}
        projectId={testmoProjectId}
        selectedCases={selectedCases}
        onSuccess={() => setSelectedIds(new Set())}
      />
    </div>
  );
}

interface SelectableResultCardProps {
  result: SearchResult;
  selected: boolean;
  onToggle: () => void;
}

function SelectableResultCard({ result, selected, onToggle }: SelectableResultCardProps) {
  return (
    <div
      className={`flex items-start gap-3 p-3 border rounded-lg transition-colors cursor-pointer
        ${selected 
          ? "border-primary-300 bg-primary-50" 
          : "border-neutral-200 hover:border-primary-200"
        }`}
      onClick={onToggle}
    >
      <Checkbox.Root
        checked={selected}
        onCheckedChange={onToggle}
        className="w-5 h-5 rounded border border-neutral-300 bg-white
                   flex items-center justify-center flex-shrink-0
                   data-[state=checked]:bg-primary-500 data-[state=checked]:border-primary-500"
      >
        <Checkbox.Indicator>
          <CheckIcon className="w-4 h-4 text-white" />
        </Checkbox.Indicator>
      </Checkbox.Root>

      <div className="flex-1 min-w-0">
        <h4 className="font-medium text-neutral-900 truncate">{result.name}</h4>
        {result.description && (
          <p className="text-sm text-neutral-500 line-clamp-1">{result.description}</p>
        )}
      </div>

      <a
        href={result.url}
        target="_blank"
        rel="noopener noreferrer"
        onClick={(e) => e.stopPropagation()}
        className="text-xs text-primary-600 hover:underline flex-shrink-0"
      >
        View
      </a>
    </div>
  );
}
```

### Project Structure Notes

Files to create:
```
crates/qa-pms-api/src/routes/
└── testmo.rs                 # Test run endpoint

frontend/src/
├── hooks/
│   └── useCreateTestRun.ts   # Mutation hook
└── components/search/
    ├── CreateTestRunDialog.tsx
    └── SelectableSearchResults.tsx
```

### Testing Notes

- Test run name generation with various ticket keys
- Test case selection state management
- Test dialog open/close behavior
- Test success toast with link
- Test error handling and retry
- Test API validation (empty case_ids)

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 4.5]
- [Source: Testmo API - Create Run](https://docs.testmo.com/api/runs)

## Dev Agent Record

### Agent Model Used

Claude Opus 4.5

### Debug Log References

N/A

### Completion Notes List

- Backend: POST /api/v1/testmo/runs endpoint with naming convention QA-{ticket}-{YYYY-MM-DD}
- Frontend: useCreateTestRun hook with React Query mutation and toast notifications
- Frontend: CreateTestRunDialog with Radix UI Dialog for accessibility
- Frontend: SelectableSearchResults with checkbox selection for Testmo results
- Context7 used for Axum and Radix UI documentation
- All 7 ACs verified, 31 backend tests passing, frontend build successful

### File List

- crates/qa-pms-api/src/routes/testmo.rs (new)
- crates/qa-pms-api/src/routes/mod.rs (updated)
- crates/qa-pms-api/src/app.rs (updated - added testmo_client, testmo_project_id)
- frontend/src/hooks/useCreateTestRun.ts (new)
- frontend/src/hooks/index.ts (updated)
- frontend/src/components/search/CreateTestRunDialog.tsx (new)
- frontend/src/components/search/SelectableSearchResults.tsx (new)
- frontend/src/components/search/RelatedTests.tsx (updated)
- frontend/src/components/search/index.ts (updated)
