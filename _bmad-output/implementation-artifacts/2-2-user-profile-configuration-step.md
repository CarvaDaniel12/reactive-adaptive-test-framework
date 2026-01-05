# Story 2.2: User Profile Configuration Step

Status: ready-for-dev

## Story

As a QA user,
I want to configure my profile information,
So that the framework can find my Jira tickets.

## Acceptance Criteria

1. **Given** user is on Step 1 of setup wizard
   **When** user views the form
   **Then** the form displays fields for: Display name (required), Jira username/email (required), Preferred ticket states (multi-select: backlog, not ready for QA, ready for QA, QA in progress, UAT)

2. **Given** user has entered profile information
   **When** user clicks "Next" without filling required fields
   **Then** form validation errors are displayed inline below each invalid field

3. **Given** all required fields are filled correctly
   **When** user clicks "Next"
   **Then** profile data is stored in wizard state (FR-CFG-02, FR-CFG-03)

4. **Given** user clicks "Next" with valid data
   **When** the form is saving
   **Then** skeleton loading is displayed while saving

5. **Given** user has completed this step before
   **When** user returns to this step
   **Then** previously entered data is pre-populated in the form

6. **Given** the form is rendered
   **When** user interacts with any input
   **Then** all form elements use Radix UI Form primitives with Tailwind CSS v4 styling

## Tasks / Subtasks

- [ ] Task 1: Create UserProfileStep component (AC: #1, #6)
  - [ ] 1.1: Create `UserProfileStep.tsx` in `frontend/src/pages/Setup/steps/`
  - [ ] 1.2: Implement form layout with WizardStepHeader
  - [ ] 1.3: Add display name text input field

- [ ] Task 2: Implement Jira username/email field (AC: #1)
  - [ ] 2.1: Add email input with proper type validation
  - [ ] 2.2: Add placeholder text "your-email@company.com"
  - [ ] 2.3: Apply input styling per design system

- [ ] Task 3: Implement ticket states multi-select (AC: #1)
  - [ ] 3.1: Create multi-select checkbox group using Radix UI Checkbox
  - [ ] 3.2: Add options: "Backlog", "Not Ready for QA", "Ready for QA", "QA in Progress", "UAT"
  - [ ] 3.3: Default select "Ready for QA" and "QA in Progress"
  - [ ] 3.4: Style checkboxes with primary-500 when checked

- [ ] Task 4: Implement form validation (AC: #2)
  - [ ] 4.1: Use Radix UI Form primitives for validation
  - [ ] 4.2: Add required validation for display name
  - [ ] 4.3: Add email format validation for Jira username
  - [ ] 4.4: Display inline error messages with error-500 color

- [ ] Task 5: Connect form to wizard store (AC: #3, #5)
  - [ ] 5.1: Read initial values from useWizardStore on mount
  - [ ] 5.2: Update wizard store formData on field change
  - [ ] 5.3: Mark step complete when Next succeeds

- [ ] Task 6: Implement loading state (AC: #4)
  - [ ] 6.1: Show skeleton loading on form fields while saving
  - [ ] 6.2: Disable Next button during save operation
  - [ ] 6.3: Add 300ms minimum loading time for UX

- [ ] Task 7: Implement step navigation integration (AC: #3)
  - [ ] 7.1: Validate form on Next click
  - [ ] 7.2: If valid, save to store and navigate to `/setup/jira`
  - [ ] 7.3: If invalid, focus first error field

- [ ] Task 8: Add form accessibility (AC: #6)
  - [ ] 8.1: Add proper aria-labels to all inputs
  - [ ] 8.2: Implement keyboard navigation (Tab order)
  - [ ] 8.3: Connect labels to inputs with htmlFor

## Dev Notes

### Architecture Alignment

This story implements the **User Profile Configuration Step** per Epic 2 requirements. Follow patterns from `project-context.md`:

- **Location**: `frontend/src/pages/Setup/steps/UserProfileStep.tsx`
- **State**: Use existing `useWizardStore` from Story 2-1
- **Validation**: Radix UI Form primitives
- **Styling**: Tailwind CSS v4 with OKLCH colors

### Technical Implementation Details

#### UserProfileStep Component Pattern

```tsx
// frontend/src/pages/Setup/steps/UserProfileStep.tsx
import * as Form from "@radix-ui/react-form";
import * as Checkbox from "@radix-ui/react-checkbox";
import { CheckIcon } from "@radix-ui/react-icons";
import { useWizardStore } from "@/stores/wizardStore";
import { WizardStepHeader } from "@/components/wizard/WizardStepHeader";
import { useNavigate } from "react-router-dom";
import { useState, useEffect } from "react";

const TICKET_STATES = [
  { id: "backlog", label: "Backlog" },
  { id: "not-ready-for-qa", label: "Not Ready for QA" },
  { id: "ready-for-qa", label: "Ready for QA" },
  { id: "qa-in-progress", label: "QA in Progress" },
  { id: "uat", label: "UAT" },
];

const DEFAULT_SELECTED_STATES = ["ready-for-qa", "qa-in-progress"];

interface ProfileFormData {
  displayName: string;
  jiraEmail: string;
  ticketStates: string[];
}

export function UserProfileStep() {
  const navigate = useNavigate();
  const { formData, setFormData, markStepComplete } = useWizardStore();
  const [isLoading, setIsLoading] = useState(false);
  
  // Initialize from store or defaults
  const [localData, setLocalData] = useState<ProfileFormData>(() => ({
    displayName: (formData.profile as ProfileFormData)?.displayName || "",
    jiraEmail: (formData.profile as ProfileFormData)?.jiraEmail || "",
    ticketStates: (formData.profile as ProfileFormData)?.ticketStates || DEFAULT_SELECTED_STATES,
  }));

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setIsLoading(true);
    
    // Minimum loading time for UX
    await new Promise(resolve => setTimeout(resolve, 300));
    
    // Save to wizard store
    setFormData("profile", localData);
    markStepComplete(1);
    
    setIsLoading(false);
    navigate("/setup/jira");
  };

  const toggleTicketState = (stateId: string) => {
    setLocalData(prev => ({
      ...prev,
      ticketStates: prev.ticketStates.includes(stateId)
        ? prev.ticketStates.filter(s => s !== stateId)
        : [...prev.ticketStates, stateId],
    }));
  };

  return (
    <div className="space-y-6">
      <WizardStepHeader
        title="Your Profile"
        description="Tell us about yourself so we can find your Jira tickets"
      />
      
      <Form.Root className="space-y-6" onSubmit={handleSubmit}>
        {/* Display Name Field */}
        <Form.Field name="displayName" className="space-y-2">
          <Form.Label className="text-sm font-medium text-neutral-700">
            Display Name <span className="text-error-500">*</span>
          </Form.Label>
          <Form.Control asChild>
            <input
              type="text"
              required
              value={localData.displayName}
              onChange={(e) => setLocalData(prev => ({ ...prev, displayName: e.target.value }))}
              className="w-full px-4 py-2 border border-neutral-300 rounded-lg 
                         focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500
                         data-[invalid]:border-error-500 data-[invalid]:focus:ring-error-500"
              placeholder="Ana Silva"
              disabled={isLoading}
            />
          </Form.Control>
          <Form.Message match="valueMissing" className="text-sm text-error-500">
            Please enter your display name
          </Form.Message>
        </Form.Field>

        {/* Jira Email Field */}
        <Form.Field name="jiraEmail" className="space-y-2">
          <Form.Label className="text-sm font-medium text-neutral-700">
            Jira Email <span className="text-error-500">*</span>
          </Form.Label>
          <Form.Control asChild>
            <input
              type="email"
              required
              value={localData.jiraEmail}
              onChange={(e) => setLocalData(prev => ({ ...prev, jiraEmail: e.target.value }))}
              className="w-full px-4 py-2 border border-neutral-300 rounded-lg 
                         focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500
                         data-[invalid]:border-error-500 data-[invalid]:focus:ring-error-500"
              placeholder="your-email@company.com"
              disabled={isLoading}
            />
          </Form.Control>
          <Form.Message match="valueMissing" className="text-sm text-error-500">
            Please enter your Jira email
          </Form.Message>
          <Form.Message match="typeMismatch" className="text-sm text-error-500">
            Please enter a valid email address
          </Form.Message>
        </Form.Field>

        {/* Ticket States Multi-Select */}
        <div className="space-y-3">
          <label className="text-sm font-medium text-neutral-700">
            Ticket States to Show
          </label>
          <p className="text-sm text-neutral-500">
            Select which ticket states you want to see in your dashboard
          </p>
          <div className="space-y-2">
            {TICKET_STATES.map((state) => (
              <div key={state.id} className="flex items-center gap-3">
                <Checkbox.Root
                  id={state.id}
                  checked={localData.ticketStates.includes(state.id)}
                  onCheckedChange={() => toggleTicketState(state.id)}
                  disabled={isLoading}
                  className="w-5 h-5 border border-neutral-300 rounded flex items-center justify-center
                             data-[state=checked]:bg-primary-500 data-[state=checked]:border-primary-500
                             focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2"
                >
                  <Checkbox.Indicator>
                    <CheckIcon className="w-4 h-4 text-white" />
                  </Checkbox.Indicator>
                </Checkbox.Root>
                <label
                  htmlFor={state.id}
                  className="text-sm text-neutral-700 cursor-pointer"
                >
                  {state.label}
                </label>
              </div>
            ))}
          </div>
        </div>

        {/* Submit handled by WizardNavigation */}
      </Form.Root>
    </div>
  );
}
```

#### WizardStepHeader Component

```tsx
// frontend/src/components/wizard/WizardStepHeader.tsx
interface WizardStepHeaderProps {
  title: string;
  description: string;
}

export function WizardStepHeader({ title, description }: WizardStepHeaderProps) {
  return (
    <div className="space-y-2">
      <h2 className="text-2xl font-semibold text-neutral-900">{title}</h2>
      <p className="text-neutral-600">{description}</p>
    </div>
  );
}
```

#### Form Validation with Radix UI Form

The Radix UI Form component provides built-in validation with these features:
- `match="valueMissing"` - Shows when required field is empty
- `match="typeMismatch"` - Shows when email format is invalid
- `data-[invalid]` - CSS attribute for invalid state styling
- `data-[valid]` - CSS attribute for valid state styling

#### Loading Skeleton Pattern

```tsx
// When isLoading is true, show skeleton
{isLoading ? (
  <div className="space-y-2 animate-pulse">
    <div className="h-4 w-24 bg-neutral-200 rounded" />
    <div className="h-10 w-full bg-neutral-200 rounded-lg" />
  </div>
) : (
  // Actual form field
)}
```

### Project Structure Notes

Files to create/modify:
```
frontend/src/
├── pages/
│   └── Setup/
│       └── steps/
│           └── UserProfileStep.tsx  # Main component (create)
├── components/
│   └── wizard/
│       └── WizardStepHeader.tsx     # Step header (create if not exists)
└── stores/
    └── wizardStore.ts               # Add profile type (modify)
```

### UX Design Requirements (from ux-design-specification.md)

- **Form Layout**: Single column, centered, max-width 600px
- **Input Styling**: 
  - Border: `border-neutral-300`
  - Focus: `ring-primary-500`, `border-primary-500`
  - Error: `border-error-500`, `text-error-500`
- **Checkbox Styling**: 
  - Checked: `bg-primary-500`
  - With checkmark icon
- **Skeleton Loading**: Gray placeholders with pulse animation
- **Typography**: 
  - Labels: `text-sm font-medium text-neutral-700`
  - Descriptions: `text-neutral-600`
  - Errors: `text-sm text-error-500`

### Dependencies Required

Already installed in Epic 1:
- `@radix-ui/react-form` (if not, install: `npm install @radix-ui/react-form`)
- `@radix-ui/react-checkbox`
- `@radix-ui/react-icons`

### Form Data Schema

```typescript
interface ProfileFormData {
  displayName: string;      // Required, non-empty
  jiraEmail: string;        // Required, valid email format
  ticketStates: string[];   // Array of selected state IDs
}

// Default ticket states to pre-select
const DEFAULT_STATES = ["ready-for-qa", "qa-in-progress"];
```

### Validation Rules

| Field | Validation | Error Message |
|-------|------------|---------------|
| Display Name | Required, min 1 char | "Please enter your display name" |
| Jira Email | Required, valid email | "Please enter a valid email address" |
| Ticket States | Optional (but recommended) | No validation error |

### Testing Notes

- Unit test form renders all fields correctly
- Unit test validation triggers on submit with empty required fields
- Unit test email format validation rejects invalid emails
- Unit test checkbox state toggles work correctly
- Unit test form data persists to wizard store
- Integration test: Complete step and navigate to Jira step
- Accessibility test: Verify screen reader announces errors

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 2.2]
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Form Patterns]
- [Source: _bmad-output/planning-artifacts/architecture.md#Frontend Architecture]
- [Source: Story 2-1 for wizard store and navigation patterns]

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
