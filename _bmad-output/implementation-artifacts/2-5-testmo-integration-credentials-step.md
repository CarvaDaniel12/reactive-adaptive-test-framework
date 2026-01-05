# Story 2.5: Testmo Integration Credentials Step

Status: ready-for-dev

## Story

As a user setting up the framework,
I want to enter my Testmo API credentials,
So that the framework can search test cases.

## Acceptance Criteria

1. **Given** user is on Testmo configuration step
   **When** user views the form
   **Then** the form displays fields for: Testmo instance URL, API Key

2. **Given** user has entered Testmo credentials
   **When** user clicks "Test Connection" button
   **Then** credentials are validated against Testmo API

3. **Given** connection test succeeds
   **When** response is received
   **Then** success displays showing number of accessible projects

4. **Given** connection test fails
   **When** response is received
   **Then** clear error message is displayed

5. **Given** user completes the step
   **When** credentials are saved
   **Then** credentials are encrypted before storage (NFR-SEC-01)

6. **Given** this is an optional step
   **When** user views the step
   **Then** Skip button is available with warning tooltip

## Tasks / Subtasks

- [ ] Task 1: Create TestmoStep component (AC: #1, #6)
  - [ ] 1.1: Create `TestmoStep.tsx` in `frontend/src/pages/Setup/steps/`
  - [ ] 1.2: Implement form layout with WizardStepHeader
  - [ ] 1.3: Add Testmo instance URL input

- [ ] Task 2: Implement API Key field (AC: #1)
  - [ ] 2.1: Add API Key password input (masked)
  - [ ] 2.2: Add show/hide toggle for API key
  - [ ] 2.3: Add helper text with link to Testmo settings

- [ ] Task 3: Implement Test Connection button (AC: #2, #3, #4)
  - [ ] 3.1: Add "Test Connection" button with loading state
  - [ ] 3.2: Call backend API `/api/v1/setup/integrations/testmo/test`
  - [ ] 3.3: Display success with project count
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
  - [ ] 6.2: Navigate to `/setup/splunk` on Next
  - [ ] 6.3: Navigate to `/setup/postman` on Back

## Dev Notes

### Architecture Alignment

This story implements the **Testmo Integration Credentials Step** per Epic 2 requirements:

- **Location**: `frontend/src/pages/Setup/steps/TestmoStep.tsx`
- **API Endpoint**: `POST /api/v1/setup/integrations/testmo/test`
- **Optional Step**: Can be skipped (Testmo is not mandatory)

### Technical Implementation Details

#### TestmoStep Component Pattern

```tsx
// frontend/src/pages/Setup/steps/TestmoStep.tsx
import * as Form from "@radix-ui/react-form";
import * as Tooltip from "@radix-ui/react-tooltip";
import { useWizardStore } from "@/stores/wizardStore";
import { WizardStepHeader } from "@/components/wizard/WizardStepHeader";
import { ConnectionStatus } from "@/components/wizard/ConnectionStatus";
import { useState } from "react";
import { EyeOpenIcon, EyeClosedIcon } from "@radix-ui/react-icons";

interface TestmoFormData {
  instanceUrl: string;
  apiKey: string;
}

type ConnectionState = "idle" | "testing" | "success" | "error";

export function TestmoStep() {
  const { formData, setFormData, markStepComplete, markStepSkipped } = useWizardStore();
  const [showApiKey, setShowApiKey] = useState(false);
  const [connectionState, setConnectionState] = useState<ConnectionState>("idle");
  const [errorMessage, setErrorMessage] = useState<string>("");
  const [projectCount, setProjectCount] = useState<number>(0);
  
  const [localData, setLocalData] = useState<TestmoFormData>(() => ({
    instanceUrl: (formData.testmo as TestmoFormData)?.instanceUrl || "",
    apiKey: (formData.testmo as TestmoFormData)?.apiKey || "",
  }));

  const testConnection = async () => {
    setConnectionState("testing");
    setErrorMessage("");
    
    try {
      const response = await fetch("/api/v1/setup/integrations/testmo/test", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(localData),
      });
      
      if (response.ok) {
        const data = await response.json();
        setConnectionState("success");
        setProjectCount(data.projectCount);
        setFormData("testmo", localData);
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
    markStepSkipped(4); // Testmo is step 4
  };

  return (
    <div className="space-y-6">
      <WizardStepHeader
        title="Testmo Integration"
        description="Connect Testmo to search and sync test cases"
      />
      
      <Form.Root className="space-y-6">
        {/* Instance URL */}
        <Form.Field name="instanceUrl" className="space-y-2">
          <Form.Label className="text-sm font-medium text-neutral-700">
            Testmo Instance URL <span className="text-error-500">*</span>
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
              placeholder="https://company.testmo.net"
            />
          </Form.Control>
          <p className="text-sm text-neutral-500">
            Your Testmo instance URL
          </p>
        </Form.Field>

        {/* API Key */}
        <Form.Field name="apiKey" className="space-y-2">
          <Form.Label className="text-sm font-medium text-neutral-700">
            API Key <span className="text-error-500">*</span>
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
                placeholder="Your Testmo API Key"
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
            Find your API key in Testmo Settings → API Access
          </p>
        </Form.Field>

        {/* Test Connection Button */}
        <div className="flex items-center gap-4">
          <button
            type="button"
            onClick={testConnection}
            disabled={connectionState === "testing" || !localData.instanceUrl || !localData.apiKey}
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
          successMessage={`Connected! ${projectCount} project(s) accessible`}
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
                  Skip Testmo setup
                </button>
              </Tooltip.Trigger>
              <Tooltip.Portal>
                <Tooltip.Content
                  className="bg-neutral-900 text-white text-sm px-3 py-2 rounded shadow-lg max-w-xs"
                  sideOffset={5}
                >
                  You can configure Testmo later in Settings. Test case search will be unavailable until configured.
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
interface TestmoFormData {
  instanceUrl: string;  // Required, Testmo instance URL
  apiKey: string;       // Required, API Key (encrypted)
}
```

### API Contract

```typescript
// POST /api/v1/setup/integrations/testmo/test
// Request
{
  instanceUrl: string;
  apiKey: string;
}

// Response (Success - 200)
{
  success: true;
  projectCount: number;
  projects: Array<{ id: number; name: string }>;
}

// Response (Error - 400/401/500)
{
  success: false;
  message: string;
  code: "INVALID_API_KEY" | "INVALID_URL" | "UNREACHABLE" | "UNKNOWN";
}
```

### Validation Rules

| Field | Validation | Error Message |
|-------|------------|---------------|
| Instance URL | Required, valid HTTPS URL | "Please enter a valid Testmo URL" |
| API Key | Required, non-empty | "Please enter your API key" |
| Connection Test | Required unless skipped | None (can skip) |

### Project Structure Notes

Files to create:
```
frontend/src/
└── pages/
    └── Setup/
        └── steps/
            └── TestmoStep.tsx    # Main component
```

### Testing Notes

- Unit test form renders all fields correctly
- Unit test skip functionality marks step as skipped
- Unit test connection success shows project count
- Mock API calls for connection testing

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 2.5]
- [Source: Testmo API Documentation]

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
