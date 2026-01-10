# Story 2.7: Configuration YAML Generation and Validation

Status: done

## Story

As a user completing setup,
I want my configuration saved and validated,
So that I can start using the framework immediately.

## Acceptance Criteria

1. **Given** user completes all setup wizard steps
   **When** user clicks "Complete Setup"
   **Then** system generates validated YAML configuration file

2. **Given** configuration is being generated
   **When** validation runs
   **Then** all required fields are validated as present

3. **Given** encrypted secrets exist
   **When** validation runs
   **Then** system verifies encrypted secrets can be decrypted

4. **Given** integrations are configured
   **When** validation runs
   **Then** startup validation runs for all integrations (NFR-INT-02)

5. **Given** configuration file is generated
   **When** file is saved
   **Then** file follows schema from architecture document

6. **Given** validation errors occur
   **When** errors are detected
   **Then** errors are displayed clearly with fix suggestions

7. **Given** setup completes successfully
   **When** validation passes
   **Then** redirect to main application with first tickets loaded (FR-CFG-07)

## Tasks / Subtasks

- [x] Task 1: Create SetupComplete component (AC: #7)
  - [x] 1.1: Create `SetupComplete.tsx` in `frontend/src/pages/Setup/steps/`
  - [x] 1.2: Implement completion summary view
  - [x] 1.3: Add "Complete Setup" button with loading state

- [x] Task 2: Implement configuration summary display (AC: #1)
  - [x] 2.1: Show configured integrations with status badges
  - [x] 2.2: Show skipped integrations with warning
  - [x] 2.3: Show user profile summary

- [x] Task 3: Implement completion API call (AC: #1, #4)
  - [x] 3.1: Call `POST /api/v1/setup/complete` with all form data
  - [x] 3.2: Handle loading state during validation
  - [x] 3.3: Handle success redirect

- [x] Task 4: Implement validation error display (AC: #6)
  - [x] 4.1: Create ValidationErrors component (inline inside `SetupComplete.tsx`)
  - [x] 4.2: Display each error with field name and message
  - [x] 4.3: Add "Fix" links that navigate to relevant step
  - [ ] 4.4: Group errors by step/category

- [x] Task 5: Implement success flow (AC: #7)
  - [x] 5.1: Show success animation/message
  - [x] 5.2: Persist completion state (Zustand persist → localStorage)
  - [x] 5.3: Redirect to main app `/` after 2 seconds

- [x] Task 6: Backend - Create configuration schema (AC: #5)
  - [x] 6.1: Define YAML schema in Rust with serde
  - [x] 6.2: Implement serialization with proper field names
  - [x] 6.3: Add schema documentation comments

- [x] Task 7: Backend - Implement validation logic (AC: #2, #3, #4)
  - [x] 7.1: Validate required fields presence
  - [x] 7.2: Test decryption of encrypted secrets
  - [x] 7.3: Run parallel integration health checks
  - [x] 7.4: Aggregate validation results

- [x] Task 8: Backend - Implement completion endpoint (AC: #1)
  - [x] 8.1: Create `POST /api/v1/setup/complete` handler
  - [x] 8.2: Generate YAML configuration file
  - [x] 8.3: Save to file system (user-scoped config path)
  - [x] 8.4: Return validation results (structured)

## Dev Notes

### Architecture Alignment

This story implements the **Configuration YAML Generation and Validation** per Epic 2 requirements:

- **Frontend**: `frontend/src/pages/Setup/steps/SetupComplete.tsx`
- **Backend**: `qa-pms-config` crate for YAML generation
- **API Endpoint**: `POST /api/v1/setup/complete`

### Technical Implementation Details

#### SetupComplete Component Pattern

```tsx
// frontend/src/pages/Setup/steps/SetupComplete.tsx
import { useWizardStore } from "@/stores/wizardStore";
import { WizardStepHeader } from "@/components/wizard/WizardStepHeader";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { CheckCircledIcon, CrossCircledIcon, ExclamationTriangleIcon } from "@radix-ui/react-icons";

type CompletionState = "idle" | "validating" | "success" | "error";

interface ValidationError {
  field: string;
  message: string;
  step: string;
  fixPath: string;
}

export function SetupComplete() {
  const navigate = useNavigate();
  const { formData, completedSteps, skippedSteps, reset } = useWizardStore();
  const [state, setState] = useState<CompletionState>("idle");
  const [errors, setErrors] = useState<ValidationError[]>([]);

  const integrations = [
    { key: "jira", name: "Jira", step: 2, required: true },
    { key: "postman", name: "Postman", step: 3, required: false },
    { key: "testmo", name: "Testmo", step: 4, required: false },
    { key: "splunk", name: "Splunk", step: 5, required: false },
  ];

  const handleComplete = async () => {
    setState("validating");
    setErrors([]);

    try {
      const response = await fetch("/api/v1/setup/complete", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(formData),
      });

      const result = await response.json();

      if (response.ok && result.success) {
        setState("success");
        localStorage.setItem("setup_complete", "true");
        
        // Redirect after success animation
        setTimeout(() => {
          reset();
          navigate("/");
        }, 2000);
      } else {
        setState("error");
        setErrors(result.errors || []);
      }
    } catch (err) {
      setState("error");
      setErrors([{
        field: "connection",
        message: "Could not connect to server",
        step: "general",
        fixPath: "",
      }]);
    }
  };

  const getIntegrationStatus = (integration: typeof integrations[0]) => {
    if (completedSteps.includes(integration.step)) {
      return "connected";
    }
    if (skippedSteps.includes(integration.step)) {
      return "skipped";
    }
    return "pending";
  };

  return (
    <div className="space-y-6">
      <WizardStepHeader
        title="Complete Setup"
        description="Review your configuration and finish setup"
      />

      {/* Configuration Summary */}
      <div className="bg-neutral-50 rounded-lg p-4 space-y-4">
        <h3 className="font-medium text-neutral-700">Configuration Summary</h3>
        
        {/* User Profile */}
        <div className="flex items-center justify-between py-2 border-b border-neutral-200">
          <span className="text-sm text-neutral-600">User Profile</span>
          <span className="text-sm font-medium">
            {(formData.profile as any)?.displayName || "Not configured"}
          </span>
        </div>

        {/* Integrations */}
        {integrations.map((integration) => {
          const status = getIntegrationStatus(integration);
          return (
            <div key={integration.key} className="flex items-center justify-between py-2 border-b border-neutral-200 last:border-0">
              <span className="text-sm text-neutral-600">
                {integration.name}
                {integration.required && <span className="text-error-500 ml-1">*</span>}
              </span>
              <span className={`text-sm font-medium flex items-center gap-1 ${
                status === "connected" ? "text-success-500" :
                status === "skipped" ? "text-warning-500" :
                "text-neutral-400"
              }`}>
                {status === "connected" && <CheckCircledIcon className="w-4 h-4" />}
                {status === "skipped" && <ExclamationTriangleIcon className="w-4 h-4" />}
                {status === "connected" ? "Connected" : status === "skipped" ? "Skipped" : "Pending"}
              </span>
            </div>
          );
        })}
      </div>

      {/* Validation Errors */}
      {state === "error" && errors.length > 0 && (
        <div className="bg-error-50 border border-error-200 rounded-lg p-4">
          <div className="flex items-center gap-2 mb-3">
            <CrossCircledIcon className="w-5 h-5 text-error-500" />
            <span className="font-medium text-error-700">Configuration Errors</span>
          </div>
          <ul className="space-y-2">
            {errors.map((error, index) => (
              <li key={index} className="text-sm text-error-600 flex items-start gap-2">
                <span className="flex-1">{error.message}</span>
                {error.fixPath && (
                  <button
                    onClick={() => navigate(error.fixPath)}
                    className="text-primary-500 hover:underline"
                  >
                    Fix
                  </button>
                )}
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Success State */}
      {state === "success" && (
        <div className="bg-success-50 border border-success-200 rounded-lg p-6 text-center">
          <CheckCircledIcon className="w-12 h-12 text-success-500 mx-auto mb-3" />
          <p className="text-lg font-medium text-success-700">Setup Complete!</p>
          <p className="text-sm text-success-600 mt-1">Redirecting to your dashboard...</p>
        </div>
      )}

      {/* Complete Button */}
      {state !== "success" && (
        <button
          onClick={handleComplete}
          disabled={state === "validating"}
          className="w-full py-3 bg-primary-500 text-white rounded-lg hover:bg-primary-600 
                     disabled:bg-neutral-300 disabled:cursor-not-allowed transition-colors
                     font-medium text-lg"
        >
          {state === "validating" ? (
            <span className="flex items-center justify-center gap-2">
              <div className="w-5 h-5 border-2 border-white border-t-transparent rounded-full animate-spin" />
              Validating Configuration...
            </span>
          ) : (
            "Complete Setup"
          )}
        </button>
      )}
    </div>
  );
}
```

#### Backend YAML Schema (Rust)

```rust
// crates/qa-pms-config/src/schema.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub version: String,
    pub profile: UserProfile,
    pub integrations: Integrations,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splunk_templates: Option<Vec<SplunkTemplate>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub display_name: String,
    pub jira_email: String,
    pub ticket_states: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Integrations {
    pub jira: JiraConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postman: Option<PostmanConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testmo: Option<TestmoConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JiraConfig {
    pub instance_url: String,
    /// Encrypted OAuth Client ID
    pub client_id_encrypted: String,
    /// Encrypted OAuth Client Secret
    pub client_secret_encrypted: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostmanConfig {
    /// Encrypted API Key
    pub api_key_encrypted: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestmoConfig {
    pub instance_url: String,
    /// Encrypted API Key
    pub api_key_encrypted: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplunkTemplate {
    pub name: String,
    pub query: String,
}
```

#### Validation Logic

```rust
// crates/qa-pms-config/src/validation.rs
use crate::schema::*;
use anyhow::Result;

#[derive(Debug)]
pub struct ValidationResult {
    pub success: bool,
    pub errors: Vec<ValidationError>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub step: String,
    pub fix_path: String,
}

pub async fn validate_config(config: &AppConfig) -> ValidationResult {
    let mut errors = Vec::new();

    // 1. Required fields validation
    if config.profile.display_name.is_empty() {
        errors.push(ValidationError {
            field: "displayName".to_string(),
            message: "Display name is required".to_string(),
            step: "profile".to_string(),
            fix_path: "/setup/profile".to_string(),
        });
    }

    // 2. Jira is required
    if config.integrations.jira.instance_url.is_empty() {
        errors.push(ValidationError {
            field: "jira.instanceUrl".to_string(),
            message: "Jira instance URL is required".to_string(),
            step: "jira".to_string(),
            fix_path: "/setup/jira".to_string(),
        });
    }

    // 3. Test decryption of secrets
    // (Implementation depends on encryption module)

    // 4. Parallel health checks (if no errors so far)
    if errors.is_empty() {
        // Run integration tests in parallel using tokio::join!
    }

    ValidationResult {
        success: errors.is_empty(),
        errors,
    }
}
```

### API Contract

```typescript
// POST /api/v1/setup/complete
// Request
{
  profile: {
    displayName: string;
    jiraEmail: string;
    ticketStates: string[];
  };
  jira: {
    instanceUrl: string;
    clientId: string;
    clientSecret: string;
  };
  postman?: {
    apiKey: string;
    workspaceId?: string;
  };
  testmo?: {
    instanceUrl: string;
    apiKey: string;
  };
  splunk?: {
    instanceUrl?: string;
    queryTemplates?: string;
  };
}

// Response (Success - 200)
{
  success: true;
  configPath: string;
  integrationsValidated: {
    jira: boolean;
    postman: boolean;
    testmo: boolean;
  };
}

// Response (Error - 400)
{
  success: false;
  errors: Array<{
    field: string;
    message: string;
    step: string;
    fixPath: string;
  }>;
}
```

### Project Structure Notes

Files to create:
```
frontend/src/
└── pages/
    └── Setup/
        └── steps/
            └── SetupComplete.tsx    # Main component

crates/qa-pms-config/src/
├── schema.rs          # YAML schema definitions
├── validation.rs      # Validation logic
└── generator.rs       # YAML file generation
```

### Testing Notes

- Unit test validation catches missing required fields
- Unit test validation catches decryption failures
- Integration test: Complete setup flow end-to-end
- Test error display and "Fix" navigation

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 2.7]
- [Source: _bmad-output/planning-artifacts/architecture.md#Configuration]
- [Source: _bmad-output/planning-artifacts/prd.md#FR-CFG-06]

## Dev Agent Record

### Agent Model Used

GPT-5.2 (Cursor)

### Debug Log References

### Completion Notes List

- Implemented structured validation errors with `fixPath` in `POST /api/v1/setup/complete`
- Added `validate_decryption()` to setup completion validation (AC3)
- Wired setup integration tests to real health checks for Postman/Testmo (AC4)
- Added startup validation via `StartupValidator` during completion (critical failures block completion; optional failures included in report)
- Frontend now displays validation errors with direct "Fix" navigation

### File List

- `qa-intelligent-pms/crates/qa-pms-api/src/routes/setup.rs`
- `qa-intelligent-pms/crates/qa-pms-api/src/app.rs`
- `qa-intelligent-pms/frontend/src/pages/Setup/steps/SetupComplete.tsx`

## Review Follow-ups (AI)

- [x] [AI-Review][HIGH] Replace stubbed setup integration tests (Postman/Testmo) with real health checks [crates/qa-pms-api/src/routes/setup.rs]
- [x] [AI-Review][MED] Run startup/integration validation in parallel during complete_setup and surface results in response [crates/qa-pms-api/src/routes/setup.rs]
- [x] [AI-Review][MED] Update this story file (tasks/status/dev record) to reflect actual implementation and reviewed file list [_bmad-output/implementation-artifacts/2-7-configuration-yaml-generation-and-validation.md]
- [ ] [AI-Review][MED] Decide Jira OAuth behavior in wizard: implement actual OAuth flow (cloud_id/access_token) or hide/disable OAuth option until supported [frontend/src/pages/Setup/steps/JiraStep.tsx, crates/qa-pms-api/src/routes/setup.rs]
- [ ] [AI-Review][LOW] Group validation errors by step/category in UI (Task 4.4) [frontend/src/pages/Setup/steps/SetupComplete.tsx]
- [ ] [AI-Review][LOW] Surface optional integration validation results (warnings) from `startupValidation` in UI [frontend/src/pages/Setup/steps/SetupComplete.tsx]
- [ ] [AI-Review][LOW] Display `configPath` to aid troubleshooting/support (optional) [frontend/src/pages/Setup/steps/SetupComplete.tsx]
