# Story 2.4: Postman Integration Credentials Step

Status: ready-for-dev

## Story

As a user setting up the framework,
I want to enter my Postman API key,
So that the framework can search my Postman collections.

## Acceptance Criteria

1. **Given** user is on Postman configuration step
   **When** user views the form
   **Then** the form displays fields for: Postman API Key (required), Workspace ID (optional)

2. **Given** user has entered Postman API key
   **When** user clicks "Test Connection" button
   **Then** API key is validated against Postman API

3. **Given** connection test succeeds
   **When** response is received
   **Then** success displays showing number of accessible workspaces

4. **Given** connection test fails
   **When** response is received
   **Then** clear error message is displayed

5. **Given** user completes the step
   **When** credentials are saved
   **Then** API key is encrypted before storage (NFR-SEC-01)

6. **Given** this is an optional step
   **When** user views the step
   **Then** Skip button is available with warning tooltip

## Tasks / Subtasks

- [ ] Task 1: Create PostmanStep component (AC: #1, #6)
  - [ ] 1.1: Create `PostmanStep.tsx` in `frontend/src/pages/Setup/steps/`
  - [ ] 1.2: Implement form layout with WizardStepHeader
  - [ ] 1.3: Add API Key password input (masked)

- [ ] Task 2: Implement Workspace ID field (AC: #1)
  - [ ] 2.1: Add optional Workspace ID text input
  - [ ] 2.2: Add helper text explaining workspace filtering
  - [ ] 2.3: Mark field as optional with "(Optional)" label

- [ ] Task 3: Implement Test Connection button (AC: #2, #3, #4)
  - [ ] 3.1: Add "Test Connection" button with loading state
  - [ ] 3.2: Call backend API `/api/v1/setup/integrations/postman/test`
  - [ ] 3.3: Display success with workspace count
  - [ ] 3.4: Display error with specific message

- [ ] Task 4: Implement Skip functionality (AC: #6)
  - [ ] 4.1: Add Skip button below navigation
  - [ ] 4.2: Add Tooltip warning "You can configure this later"
  - [ ] 4.3: Mark step as skipped in wizard store

- [ ] Task 5: Connect to wizard store (AC: #5)
  - [ ] 5.1: Read initial values from useWizardStore
  - [ ] 5.2: Save credentials to wizard formData
  - [ ] 5.3: Mark step complete or skipped

- [ ] Task 6: Implement navigation integration
  - [ ] 6.1: Allow Next if connection test passes OR step is skipped
  - [ ] 6.2: Navigate to `/setup/testmo` on Next
  - [ ] 6.3: Navigate to `/setup/jira` on Back

## Dev Notes

### Architecture Alignment

This story implements the **Postman Integration Credentials Step** per Epic 2 requirements:

- **Location**: `frontend/src/pages/Setup/steps/PostmanStep.tsx`
- **API Endpoint**: `POST /api/v1/setup/integrations/postman/test`
- **Optional Step**: Can be skipped (Postman is not mandatory)

### Technical Implementation Details

#### PostmanStep Component Pattern

```tsx
// frontend/src/pages/Setup/steps/PostmanStep.tsx
import * as Form from "@radix-ui/react-form";
import * as Tooltip from "@radix-ui/react-tooltip";
import { useWizardStore } from "@/stores/wizardStore";
import { WizardStepHeader } from "@/components/wizard/WizardStepHeader";
import { ConnectionStatus } from "@/components/wizard/ConnectionStatus";
import { useState } from "react";
import { EyeOpenIcon, EyeClosedIcon } from "@radix-ui/react-icons";

interface PostmanFormData {
  apiKey: string;
  workspaceId: string;
}

type ConnectionState = "idle" | "testing" | "success" | "error";

export function PostmanStep() {
  const { formData, setFormData, markStepComplete, markStepSkipped } = useWizardStore();
  const [showApiKey, setShowApiKey] = useState(false);
  const [connectionState, setConnectionState] = useState<ConnectionState>("idle");
  const [errorMessage, setErrorMessage] = useState<string>("");
  const [workspaceCount, setWorkspaceCount] = useState<number>(0);
  
  const [localData, setLocalData] = useState<PostmanFormData>(() => ({
    apiKey: (formData.postman as PostmanFormData)?.apiKey || "",
    workspaceId: (formData.postman as PostmanFormData)?.workspaceId || "",
  }));

  const testConnection = async () => {
    setConnectionState("testing");
    setErrorMessage("");
    
    try {
      const response = await fetch("/api/v1/setup/integrations/postman/test", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(localData),
      });
      
      if (response.ok) {
        const data = await response.json();
        setConnectionState("success");
        setWorkspaceCount(data.workspaceCount);
        setFormData("postman", localData);
      } else {
        const error = await response.json();
        setConnectionState("error");
        setErrorMessage(error.message || "Connection failed");
      }
    } catch (err) {
      setConnectionState("error");
      setErrorMessage("Cannot reach server. Check your network connection.");
    }
  };

  const handleSkip = () => {
    markStepSkipped(3); // Postman is step 3
  };

  return (
    <div className="space-y-6">
      <WizardStepHeader
        title="Postman Integration"
        description="Connect Postman to search your API test collections"
      />
      
      <Form.Root className="space-y-6">
        {/* API Key */}
        <Form.Field name="apiKey" className="space-y-2">
          <Form.Label className="text-sm font-medium text-neutral-700">
            Postman API Key <span className="text-error-500">*</span>
          </Form.Label>
          <div className="relative">
            <Form.Control asChild>
              <input
                type={showApiKey ? "text" : "password"}
                required
                value={localData.apiKey}
                onChange={(e) => {
                  setLocalData(prev => ({ ...prev, apiKey: e.target.value }));
                  setConnectionState("idle");
                }}
                className="w-full px-4 py-2 pr-10 border border-neutral-300 rounded-lg 
                           focus:outline-none focus:ring-2 focus:ring-primary-500"
                placeholder="PMAK-xxxxxxxx-xxxx..."
              />
            </Form.Control>
            <button
              type="button"
              onClick={() => setShowApiKey(!showApiKey)}
              className="absolute right-3 top-1/2 -translate-y-1/2 text-neutral-500 hover:text-neutral-700"
            >
              {showApiKey ? <EyeClosedIcon /> : <EyeOpenIcon />}
            </button>
          </div>
          <p className="text-sm text-neutral-500">
            Get your API key from{" "}
            <a 
              href="https://postman.com/settings/me/api-keys" 
              target="_blank" 
              rel="noopener noreferrer"
              className="text-primary-500 hover:underline"
            >
              Postman Settings
            </a>
          </p>
        </Form.Field>

        {/* Workspace ID (Optional) */}
        <Form.Field name="workspaceId" className="space-y-2">
          <Form.Label className="text-sm font-medium text-neutral-700">
            Workspace ID <span className="text-neutral-400">(Optional)</span>
          </Form.Label>
          <Form.Control asChild>
            <input
              type="text"
              value={localData.workspaceId}
              onChange={(e) => {
                setLocalData(prev => ({ ...prev, workspaceId: e.target.value }));
                setConnectionState("idle");
              }}
              className="w-full px-4 py-2 border border-neutral-300 rounded-lg 
                         focus:outline-none focus:ring-2 focus:ring-primary-500"
              placeholder="xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
            />
          </Form.Control>
          <p className="text-sm text-neutral-500">
            Limit search to a specific workspace. Leave empty to search all workspaces.
          </p>
        </Form.Field>

        {/* Test Connection Button */}
        <div className="flex items-center gap-4">
          <button
            type="button"
            onClick={testConnection}
            disabled={connectionState === "testing" || !localData.apiKey}
            className="px-6 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 
                       disabled:bg-neutral-300 disabled:cursor-not-allowed transition-colors"
          >
            {connectionState === "testing" ? "Testing..." : "Test Connection"}
          </button>
        </div>

        {/* Connection Status */}
        <ConnectionStatus 
          state={connectionState} 
          errorMessage={errorMessage}
          successMessage={`Connected! ${workspaceCount} workspace(s) accessible`}
        />

        {/* Skip Option */}
        <div className="pt-4 border-t border-neutral-200">
          <Tooltip.Provider>
            <Tooltip.Root>
              <Tooltip.Trigger asChild>
                <button
                  type="button"
                  onClick={handleSkip}
                  className="text-neutral-500 hover:text-neutral-700 underline text-sm"
                >
                  Skip Postman setup
                </button>
              </Tooltip.Trigger>
              <Tooltip.Portal>
                <Tooltip.Content
                  className="bg-neutral-900 text-white text-sm px-3 py-2 rounded shadow-lg max-w-xs"
                  sideOffset={5}
                >
                  You can configure Postman later in Settings. Test search will be unavailable until configured.
                  <Tooltip.Arrow className="fill-neutral-900" />
                </Tooltip.Content>
              </Tooltip.Portal>
            </Tooltip.Root>
          </Tooltip.Provider>
        </div>
      </Form.Root>
    </div>
  );
}
```

### Form Data Schema

```typescript
interface PostmanFormData {
  apiKey: string;      // Required, Postman API Key (encrypted)
  workspaceId: string; // Optional, specific workspace to filter
}
```

### API Contract

```typescript
// POST /api/v1/setup/integrations/postman/test
// Request
{
  apiKey: string;
  workspaceId?: string;
}

// Response (Success - 200)
{
  success: true;
  workspaceCount: number;
  workspaces: Array<{ id: string; name: string }>;
}

// Response (Error - 400/401/500)
{
  success: false;
  message: string;
  code: "INVALID_API_KEY" | "WORKSPACE_NOT_FOUND" | "RATE_LIMITED" | "UNKNOWN";
}
```

### Validation Rules

| Field | Validation | Error Message |
|-------|------------|---------------|
| API Key | Required for test, starts with PMAK- | "Please enter a valid Postman API key" |
| Workspace ID | Optional, UUID format if provided | "Invalid workspace ID format" |
| Connection Test | Required unless skipped | None (can skip) |

### Project Structure Notes

Files to modify:
```
frontend/src/
└── pages/
    └── Setup/
        └── steps/
            └── PostmanStep.tsx    # Main component (create)
```

### Testing Notes

- Unit test form renders all fields correctly
- Unit test skip functionality marks step as skipped
- Unit test connection success shows workspace count
- Mock API calls for connection testing

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 2.4]
- [Source: Postman API Documentation]

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
