# Story 2.3: Jira Integration Credentials Step

Status: ready-for-dev

## Story

As a user setting up the framework,
I want to enter my Jira credentials,
So that the framework can access my Jira instance.

## Acceptance Criteria

1. **Given** user is on Jira configuration step
   **When** user views the form
   **Then** the form displays fields for: Jira instance URL, OAuth Client ID, OAuth Client Secret

2. **Given** user has entered Jira credentials
   **When** user clicks "Test Connection" button
   **Then** credentials are validated against Jira API

3. **Given** connection test succeeds
   **When** response is received
   **Then** green checkmark displays with "Connected to Jira" message

4. **Given** connection test fails
   **When** response is received
   **Then** red error displays with specific message (e.g., "Invalid credentials", "Cannot reach server")

5. **Given** user completes the step
   **When** credentials are saved
   **Then** credentials are encrypted before storage (NFR-SEC-01)

6. **Given** connection test fails
   **When** user wants to try again
   **Then** retry button is available to re-test connection

## Tasks / Subtasks

- [ ] Task 1: Create JiraStep component (AC: #1, #6)
  - [ ] 1.1: Create `JiraStep.tsx` in `frontend/src/pages/Setup/steps/`
  - [ ] 1.2: Implement form layout with WizardStepHeader
  - [ ] 1.3: Add Jira instance URL input with https:// prefix helper

- [ ] Task 2: Implement OAuth credential fields (AC: #1)
  - [ ] 2.1: Add OAuth Client ID text input
  - [ ] 2.2: Add OAuth Client Secret password input (masked)
  - [ ] 2.3: Add show/hide toggle for secret field

- [ ] Task 3: Implement Test Connection button (AC: #2, #3, #4, #6)
  - [ ] 3.1: Add "Test Connection" button with loading state
  - [ ] 3.2: Call backend API `/api/v1/setup/integrations/jira/test`
  - [ ] 3.3: Display success state with green checkmark
  - [ ] 3.4: Display error state with red icon and message
  - [ ] 3.5: Add "Retry" button on failure

- [ ] Task 4: Implement connection status display (AC: #3, #4)
  - [ ] 4.1: Create ConnectionStatus component with success/error/idle states
  - [ ] 4.2: Show connected workspace info on success
  - [ ] 4.3: Show specific error message on failure
  - [ ] 4.4: Use Radix UI Alert for status messages

- [ ] Task 5: Implement form validation (AC: #1)
  - [ ] 5.1: Validate Jira URL format (must be valid URL with https)
  - [ ] 5.2: Validate Client ID is non-empty
  - [ ] 5.3: Validate Client Secret is non-empty
  - [ ] 5.4: Disable Next until connection test passes

- [ ] Task 6: Connect to wizard store (AC: #5)
  - [ ] 6.1: Read initial values from useWizardStore
  - [ ] 6.2: Save credentials to wizard formData on successful test
  - [ ] 6.3: Mark step complete when Next succeeds

- [ ] Task 7: Implement navigation integration (AC: #2)
  - [ ] 7.1: Require successful connection test before allowing Next
  - [ ] 7.2: Navigate to `/setup/postman` on Next
  - [ ] 7.3: Navigate to `/setup/profile` on Back

## Dev Notes

### Architecture Alignment

This story implements the **Jira Integration Credentials Step** per Epic 2 requirements:

- **Location**: `frontend/src/pages/Setup/steps/JiraStep.tsx`
- **API Endpoint**: `POST /api/v1/setup/integrations/jira/test`
- **Security**: Credentials encrypted via AES-256-GCM before storage

### Technical Implementation Details

#### JiraStep Component Pattern

```tsx
// frontend/src/pages/Setup/steps/JiraStep.tsx
import * as Form from "@radix-ui/react-form";
import { useWizardStore } from "@/stores/wizardStore";
import { WizardStepHeader } from "@/components/wizard/WizardStepHeader";
import { ConnectionStatus } from "@/components/wizard/ConnectionStatus";
import { useState } from "react";
import { EyeOpenIcon, EyeClosedIcon } from "@radix-ui/react-icons";

interface JiraFormData {
  instanceUrl: string;
  clientId: string;
  clientSecret: string;
}

type ConnectionState = "idle" | "testing" | "success" | "error";

export function JiraStep() {
  const { formData, setFormData, markStepComplete } = useWizardStore();
  const [showSecret, setShowSecret] = useState(false);
  const [connectionState, setConnectionState] = useState<ConnectionState>("idle");
  const [errorMessage, setErrorMessage] = useState<string>("");
  
  const [localData, setLocalData] = useState<JiraFormData>(() => ({
    instanceUrl: (formData.jira as JiraFormData)?.instanceUrl || "",
    clientId: (formData.jira as JiraFormData)?.clientId || "",
    clientSecret: (formData.jira as JiraFormData)?.clientSecret || "",
  }));

  const testConnection = async () => {
    setConnectionState("testing");
    setErrorMessage("");
    
    try {
      const response = await fetch("/api/v1/setup/integrations/jira/test", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(localData),
      });
      
      if (response.ok) {
        setConnectionState("success");
        setFormData("jira", localData);
        markStepComplete(2); // Jira is step 2
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

  const canProceed = connectionState === "success";

  return (
    <div className="space-y-6">
      <WizardStepHeader
        title="Jira Integration"
        description="Connect your Jira instance for ticket management"
      />
      
      <Form.Root className="space-y-6">
        {/* Instance URL */}
        <Form.Field name="instanceUrl" className="space-y-2">
          <Form.Label className="text-sm font-medium text-neutral-700">
            Jira Instance URL <span className="text-error-500">*</span>
          </Form.Label>
          <Form.Control asChild>
            <input
              type="url"
              required
              value={localData.instanceUrl}
              onChange={(e) => {
                setLocalData(prev => ({ ...prev, instanceUrl: e.target.value }));
                setConnectionState("idle");
              }}
              className="w-full px-4 py-2 border border-neutral-300 rounded-lg 
                         focus:outline-none focus:ring-2 focus:ring-primary-500"
              placeholder="https://company.atlassian.net"
            />
          </Form.Control>
          <p className="text-sm text-neutral-500">
            Your Atlassian Cloud or Jira Server URL
          </p>
        </Form.Field>

        {/* Client ID */}
        <Form.Field name="clientId" className="space-y-2">
          <Form.Label className="text-sm font-medium text-neutral-700">
            OAuth Client ID <span className="text-error-500">*</span>
          </Form.Label>
          <Form.Control asChild>
            <input
              type="text"
              required
              value={localData.clientId}
              onChange={(e) => {
                setLocalData(prev => ({ ...prev, clientId: e.target.value }));
                setConnectionState("idle");
              }}
              className="w-full px-4 py-2 border border-neutral-300 rounded-lg 
                         focus:outline-none focus:ring-2 focus:ring-primary-500"
              placeholder="Your OAuth 2.0 Client ID"
            />
          </Form.Control>
        </Form.Field>

        {/* Client Secret */}
        <Form.Field name="clientSecret" className="space-y-2">
          <Form.Label className="text-sm font-medium text-neutral-700">
            OAuth Client Secret <span className="text-error-500">*</span>
          </Form.Label>
          <div className="relative">
            <Form.Control asChild>
              <input
                type={showSecret ? "text" : "password"}
                required
                value={localData.clientSecret}
                onChange={(e) => {
                  setLocalData(prev => ({ ...prev, clientSecret: e.target.value }));
                  setConnectionState("idle");
                }}
                className="w-full px-4 py-2 pr-10 border border-neutral-300 rounded-lg 
                           focus:outline-none focus:ring-2 focus:ring-primary-500"
                placeholder="Your OAuth 2.0 Client Secret"
              />
            </Form.Control>
            <button
              type="button"
              onClick={() => setShowSecret(!showSecret)}
              className="absolute right-3 top-1/2 -translate-y-1/2 text-neutral-500 hover:text-neutral-700"
            >
              {showSecret ? <EyeClosedIcon /> : <EyeOpenIcon />}
            </button>
          </div>
        </Form.Field>

        {/* Test Connection Button */}
        <div className="flex items-center gap-4">
          <button
            type="button"
            onClick={testConnection}
            disabled={connectionState === "testing" || !localData.instanceUrl || !localData.clientId || !localData.clientSecret}
            className="px-6 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 
                       disabled:bg-neutral-300 disabled:cursor-not-allowed transition-colors"
          >
            {connectionState === "testing" ? "Testing..." : "Test Connection"}
          </button>
          
          {connectionState === "error" && (
            <button
              type="button"
              onClick={testConnection}
              className="text-primary-500 hover:text-primary-600 underline text-sm"
            >
              Retry
            </button>
          )}
        </div>

        {/* Connection Status */}
        <ConnectionStatus 
          state={connectionState} 
          errorMessage={errorMessage}
          successMessage="Connected to Jira"
        />
      </Form.Root>
    </div>
  );
}
```

#### ConnectionStatus Component

```tsx
// frontend/src/components/wizard/ConnectionStatus.tsx
import { CheckCircledIcon, CrossCircledIcon } from "@radix-ui/react-icons";

interface ConnectionStatusProps {
  state: "idle" | "testing" | "success" | "error";
  errorMessage?: string;
  successMessage?: string;
}

export function ConnectionStatus({ state, errorMessage, successMessage }: ConnectionStatusProps) {
  if (state === "idle") return null;
  
  if (state === "testing") {
    return (
      <div className="flex items-center gap-2 text-neutral-600">
        <div className="w-4 h-4 border-2 border-primary-500 border-t-transparent rounded-full animate-spin" />
        <span>Testing connection...</span>
      </div>
    );
  }
  
  if (state === "success") {
    return (
      <div className="flex items-center gap-2 text-success-500 bg-success-50 px-4 py-3 rounded-lg">
        <CheckCircledIcon className="w-5 h-5" />
        <span className="font-medium">{successMessage}</span>
      </div>
    );
  }
  
  if (state === "error") {
    return (
      <div className="flex items-center gap-2 text-error-500 bg-error-50 px-4 py-3 rounded-lg">
        <CrossCircledIcon className="w-5 h-5" />
        <span>{errorMessage}</span>
      </div>
    );
  }
  
  return null;
}
```

### Form Data Schema

```typescript
interface JiraFormData {
  instanceUrl: string;    // Required, valid HTTPS URL
  clientId: string;       // Required, OAuth 2.0 Client ID
  clientSecret: string;   // Required, OAuth 2.0 Client Secret (encrypted)
}
```

### API Contract

```typescript
// POST /api/v1/setup/integrations/jira/test
// Request
{
  instanceUrl: string;
  clientId: string;
  clientSecret: string;
}

// Response (Success - 200)
{
  success: true;
  workspaceName: string;
  projectCount: number;
}

// Response (Error - 400/401/500)
{
  success: false;
  message: string;
  code: "INVALID_CREDENTIALS" | "UNREACHABLE" | "INVALID_URL" | "UNKNOWN";
}
```

### Validation Rules

| Field | Validation | Error Message |
|-------|------------|---------------|
| Instance URL | Required, valid HTTPS URL | "Please enter a valid Jira URL" |
| Client ID | Required, non-empty | "Please enter your OAuth Client ID" |
| Client Secret | Required, non-empty | "Please enter your OAuth Client Secret" |
| Connection Test | Must pass before Next | "Please test connection first" |

### Security Notes

- Client Secret is masked by default (password input type)
- Show/hide toggle for secret field
- Credentials are encrypted with AES-256-GCM before storage
- Never log credentials in console or network tab

### Project Structure Notes

Files to create/modify:
```
frontend/src/
├── pages/
│   └── Setup/
│       └── steps/
│           └── JiraStep.tsx           # Main component (create)
└── components/
    └── wizard/
        └── ConnectionStatus.tsx       # Reusable status component (create)
```

### Testing Notes

- Unit test form renders all fields correctly
- Unit test connection state transitions
- Unit test error message display
- Mock API calls for connection testing
- Test show/hide toggle for secret field

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 2.3]
- [Source: _bmad-output/planning-artifacts/architecture.md#Security]
- [Source: _bmad-output/planning-artifacts/prd.md#NFR-SEC-01]

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
