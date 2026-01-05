# Story 2.6: Splunk Configuration Step (Manual)

Status: ready-for-dev

## Story

As a user setting up the framework,
I want guidance on Splunk setup,
So that I can query logs manually.

## Acceptance Criteria

1. **Given** user is on Splunk configuration step
   **When** user views the step
   **Then** explanation is displayed that Splunk Cloud direct integration is not supported

2. **Given** user is on Splunk configuration step
   **When** user views the step
   **Then** instructions for manual query approach are displayed

3. **Given** user wants to save query templates
   **When** user enters query templates
   **Then** text field is available for saving frequently-used query templates

4. **Given** user wants reference to Splunk
   **When** user enters instance URL
   **Then** optional Splunk instance URL field is available for reference links

5. **Given** Splunk is optional for MVP
   **When** user views the step
   **Then** step can be skipped without configuration

6. **Given** user saves query templates
   **When** setup completes
   **Then** templates are stored in configuration for later use

## Tasks / Subtasks

- [ ] Task 1: Create SplunkStep component (AC: #1, #2, #5)
  - [ ] 1.1: Create `SplunkStep.tsx` in `frontend/src/pages/Setup/steps/`
  - [ ] 1.2: Implement form layout with WizardStepHeader
  - [ ] 1.3: Add informational section about manual integration

- [ ] Task 2: Implement informational content (AC: #1, #2)
  - [ ] 2.1: Create informational card explaining Splunk limitations
  - [ ] 2.2: Add step-by-step manual query instructions
  - [ ] 2.3: Add link to Splunk Cloud documentation

- [ ] Task 3: Implement Splunk URL field (AC: #4)
  - [ ] 3.1: Add optional Splunk instance URL input
  - [ ] 3.2: Add helper text explaining it's for reference only
  - [ ] 3.3: Validate URL format if provided

- [ ] Task 4: Implement Query Templates field (AC: #3, #6)
  - [ ] 4.1: Add textarea for query templates
  - [ ] 4.2: Add placeholder with example query
  - [ ] 4.3: Support multiple templates (newline separated)

- [ ] Task 5: Implement Skip functionality (AC: #5)
  - [ ] 5.1: Add prominent Skip button
  - [ ] 5.2: Add note that Splunk is optional
  - [ ] 5.3: Mark step as skipped in wizard store

- [ ] Task 6: Connect to wizard store (AC: #6)
  - [ ] 6.1: Read initial values from useWizardStore
  - [ ] 6.2: Save configuration to wizard formData
  - [ ] 6.3: Mark step complete or skipped

- [ ] Task 7: Implement navigation integration
  - [ ] 7.1: Allow Next regardless of completion (optional step)
  - [ ] 7.2: Navigate to `/setup/complete` on Next
  - [ ] 7.3: Navigate to `/setup/testmo` on Back

## Dev Notes

### Architecture Alignment

This story implements the **Splunk Configuration Step** per Epic 2 requirements:

- **Location**: `frontend/src/pages/Setup/steps/SplunkStep.tsx`
- **No Backend API**: This is a manual configuration step (no test connection)
- **Optional Step**: Can be skipped entirely

### Technical Implementation Details

#### SplunkStep Component Pattern

```tsx
// frontend/src/pages/Setup/steps/SplunkStep.tsx
import * as Form from "@radix-ui/react-form";
import { useWizardStore } from "@/stores/wizardStore";
import { WizardStepHeader } from "@/components/wizard/WizardStepHeader";
import { useState } from "react";
import { InfoCircledIcon } from "@radix-ui/react-icons";

interface SplunkFormData {
  instanceUrl: string;
  queryTemplates: string;
}

export function SplunkStep() {
  const { formData, setFormData, markStepComplete, markStepSkipped } = useWizardStore();
  
  const [localData, setLocalData] = useState<SplunkFormData>(() => ({
    instanceUrl: (formData.splunk as SplunkFormData)?.instanceUrl || "",
    queryTemplates: (formData.splunk as SplunkFormData)?.queryTemplates || "",
  }));

  const handleSave = () => {
    if (localData.instanceUrl || localData.queryTemplates) {
      setFormData("splunk", localData);
      markStepComplete(5);
    } else {
      markStepSkipped(5);
    }
  };

  const handleSkip = () => {
    markStepSkipped(5);
  };

  return (
    <div className="space-y-6">
      <WizardStepHeader
        title="Splunk Configuration"
        description="Set up manual log querying (optional)"
      />
      
      {/* Informational Card */}
      <div className="bg-primary-50 border border-primary-200 rounded-lg p-4">
        <div className="flex gap-3">
          <InfoCircledIcon className="w-5 h-5 text-primary-500 flex-shrink-0 mt-0.5" />
          <div className="space-y-2">
            <p className="font-medium text-primary-700">
              About Splunk Integration
            </p>
            <p className="text-sm text-primary-600">
              Due to Splunk Cloud security restrictions, direct API integration is not available. 
              This framework provides a manual query interface with pre-built templates to help 
              you quickly search logs.
            </p>
          </div>
        </div>
      </div>

      {/* Manual Query Instructions */}
      <div className="bg-neutral-50 rounded-lg p-4 space-y-3">
        <p className="font-medium text-neutral-700">How it works:</p>
        <ol className="list-decimal list-inside text-sm text-neutral-600 space-y-2">
          <li>Save your frequently-used query templates below</li>
          <li>When viewing a ticket, select a template from the Splunk panel</li>
          <li>The framework will open Splunk with your query pre-filled</li>
          <li>Review logs directly in Splunk's interface</li>
        </ol>
      </div>

      <Form.Root className="space-y-6">
        {/* Instance URL (Optional) */}
        <Form.Field name="instanceUrl" className="space-y-2">
          <Form.Label className="text-sm font-medium text-neutral-700">
            Splunk Instance URL <span className="text-neutral-400">(Optional)</span>
          </Form.Label>
          <Form.Control asChild>
            <input
              type="url"
              value={localData.instanceUrl}
              onChange={(e) => setLocalData(prev => ({ ...prev, instanceUrl: e.target.value }))}
              className="w-full px-4 py-2 border border-neutral-300 rounded-lg 
                         focus:outline-none focus:ring-2 focus:ring-primary-500"
              placeholder="https://company.splunkcloud.com"
            />
          </Form.Control>
          <p className="text-sm text-neutral-500">
            Used to generate quick links to Splunk searches
          </p>
        </Form.Field>

        {/* Query Templates */}
        <Form.Field name="queryTemplates" className="space-y-2">
          <Form.Label className="text-sm font-medium text-neutral-700">
            Query Templates <span className="text-neutral-400">(Optional)</span>
          </Form.Label>
          <Form.Control asChild>
            <textarea
              value={localData.queryTemplates}
              onChange={(e) => setLocalData(prev => ({ ...prev, queryTemplates: e.target.value }))}
              className="w-full px-4 py-3 border border-neutral-300 rounded-lg 
                         focus:outline-none focus:ring-2 focus:ring-primary-500
                         font-mono text-sm"
              rows={6}
              placeholder={`# Error logs for ticket
index=production sourcetype=app_logs level=ERROR ticket={{ticketId}}

# Request trace
index=production sourcetype=api_logs trace_id={{traceId}}`}
            />
          </Form.Control>
          <p className="text-sm text-neutral-500">
            Enter one query per block. Use <code className="bg-neutral-100 px-1 rounded">{"{{ticketId}}"}</code> as placeholder.
            Separate multiple templates with blank lines.
          </p>
        </Form.Field>

        {/* Skip Note */}
        <div className="pt-4 border-t border-neutral-200">
          <p className="text-sm text-neutral-500 mb-3">
            Splunk integration is optional. You can configure this later in Settings.
          </p>
          <button
            type="button"
            onClick={handleSkip}
            className="text-neutral-500 hover:text-neutral-700 underline text-sm"
          >
            Skip Splunk setup
          </button>
        </div>
      </Form.Root>
    </div>
  );
}
```

### Form Data Schema

```typescript
interface SplunkFormData {
  instanceUrl: string;    // Optional, Splunk instance URL for reference links
  queryTemplates: string; // Optional, newline-separated query templates
}

// Parsed templates format (for later use)
interface SplunkTemplate {
  name: string;    // Extracted from comment line (# Template Name)
  query: string;   // The actual SPL query
}
```

### Query Template Format

```
# Error logs for ticket
index=production sourcetype=app_logs level=ERROR ticket={{ticketId}}

# Request trace
index=production sourcetype=api_logs trace_id={{traceId}}

# User activity
index=production sourcetype=user_events user={{username}} | head 100
```

Template placeholders supported:
- `{{ticketId}}` - Current Jira ticket ID
- `{{traceId}}` - Trace ID from ticket metadata
- `{{username}}` - Current user's username
- `{{component}}` - Component/service name from ticket

### Project Structure Notes

Files to create:
```
frontend/src/
└── pages/
    └── Setup/
        └── steps/
            └── SplunkStep.tsx    # Main component
```

### UX Notes

- This step is intentionally minimal since Splunk is optional
- Informational card clearly explains the manual approach
- Step-by-step instructions help users understand the workflow
- Skip option is prominent since this is optional

### Testing Notes

- Unit test form renders informational content
- Unit test skip functionality marks step as skipped
- Unit test query templates are saved correctly
- Test placeholder replacement logic (for later stories)

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 2.6]
- [Source: _bmad-output/planning-artifacts/prd.md#FR-INT-11]

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
