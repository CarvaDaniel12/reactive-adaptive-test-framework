# Story 2.1: Setup Wizard UI Shell and Navigation

Status: ready-for-dev

## Story

As a new user,
I want a guided setup wizard with clear progress,
So that I know exactly where I am in the configuration process.

## Acceptance Criteria

1. **Given** a user opens the app for the first time (no config exists)
   **When** the app loads
   **Then** the setup wizard displays with a progress indicator showing steps (1 of 5, 2 of 5, etc.)

2. **Given** the wizard is displayed
   **When** user views any step
   **Then** clear step titles and descriptions are visible

3. **Given** the wizard is on any step
   **When** user looks at navigation
   **Then** Back/Next navigation buttons are available (Back disabled on step 1, Next disabled if validation fails)

4. **Given** the wizard has optional steps
   **When** user views an optional step
   **Then** a Skip option is available with a warning tooltip

5. **Given** user is mid-setup
   **When** user closes browser and reopens the app
   **Then** wizard state persists and user can resume from where they left off (FR-CFG-05)

6. **Given** the wizard UI is rendered
   **When** user interacts with any component
   **Then** all components use Radix UI primitives with Tailwind CSS v4 styling per design system

## Tasks / Subtasks

- [ ] Task 1: Create SetupWizard route and layout component (AC: #1, #6)
  - [ ] 1.1: Add `/setup` route in React Router with nested routes for each step
  - [ ] 1.2: Create `SetupWizardLayout.tsx` with Outlet for step content
  - [ ] 1.3: Implement responsive container (max-width, centered)

- [ ] Task 2: Implement WizardProgress component (AC: #1)
  - [ ] 2.1: Create `WizardProgress.tsx` with step indicators (1 of 5)
  - [ ] 2.2: Show completed steps with checkmark, current step highlighted
  - [ ] 2.3: Apply Tailwind CSS v4 styling (primary-500 for current, success-500 for completed)

- [ ] Task 3: Create WizardNavigation component (AC: #3)
  - [ ] 3.1: Create `WizardNavigation.tsx` with Back/Next buttons
  - [ ] 3.2: Disable Back on first step, disable Next when validation fails
  - [ ] 3.3: Add keyboard navigation support (Enter for Next, Escape for Back)

- [ ] Task 4: Implement step title and description display (AC: #2)
  - [ ] 4.1: Create `WizardStepHeader.tsx` with title and description props
  - [ ] 4.2: Apply typography scale (text-2xl for title, text-neutral-600 for description)

- [ ] Task 5: Implement Skip functionality for optional steps (AC: #4)
  - [ ] 5.1: Add Skip button using Radix UI Button primitive
  - [ ] 5.2: Add Radix UI Tooltip warning on hover ("You can configure this later in Settings")
  - [ ] 5.3: Track skipped steps in wizard state

- [ ] Task 6: Implement wizard state persistence (AC: #5)
  - [ ] 6.1: Create `useWizardStore.ts` Zustand store with step state
  - [ ] 6.2: Persist to localStorage using Zustand persist middleware
  - [ ] 6.3: Auto-restore state on app load, redirect to wizard if incomplete

- [ ] Task 7: Create placeholder step components (AC: #6)
  - [ ] 7.1: Create stub components for steps 1-5 (UserProfile, Jira, Postman, Testmo, Splunk)
  - [ ] 7.2: Each step shows title and placeholder content
  - [ ] 7.3: Wire up routing to navigate between steps

- [ ] Task 8: Add route guard for setup completion (AC: #5)
  - [ ] 8.1: Create `SetupGuard.tsx` that redirects to `/setup` if config incomplete
  - [ ] 8.2: Check localStorage flag `setup_complete` on app load
  - [ ] 8.3: Redirect to main app when setup is complete

## Dev Notes

### Architecture Alignment

This story implements the **Setup Wizard** UI shell per Epic 2 requirements. All code must follow patterns from `project-context.md`:

- **Frontend Location**: `frontend/src/pages/Setup/` and `frontend/src/components/wizard/`
- **State Management**: Zustand store with persist middleware
- **Styling**: Tailwind CSS v4 with OKLCH colors defined in `index.css`
- **Components**: Radix UI primitives (Dialog, Tooltip, Progress)

### Technical Implementation Details

#### React Router Setup (v6 nested routes)

```tsx
// frontend/src/App.tsx - Add setup routes
import { Outlet } from "react-router";

const router = createBrowserRouter([
  {
    path: "/setup",
    element: <SetupWizardLayout />,
    children: [
      { index: true, element: <Navigate to="profile" replace /> },
      { path: "profile", element: <UserProfileStep /> },
      { path: "jira", element: <JiraStep /> },
      { path: "postman", element: <PostmanStep /> },
      { path: "testmo", element: <TestmoStep /> },
      { path: "splunk", element: <SplunkStep /> },
      { path: "complete", element: <SetupComplete /> },
    ],
  },
  {
    path: "/",
    element: <SetupGuard><MainLayout /></SetupGuard>,
    children: [/* main app routes */],
  },
]);
```

#### SetupWizardLayout Component Pattern

```tsx
// frontend/src/pages/Setup/SetupWizardLayout.tsx
import { Outlet } from "react-router";
import { WizardProgress } from "@/components/wizard/WizardProgress";
import { WizardNavigation } from "@/components/wizard/WizardNavigation";

export function SetupWizardLayout() {
  return (
    <div className="min-h-screen bg-neutral-50 flex items-center justify-center p-4">
      <div className="w-full max-w-2xl bg-white rounded-lg shadow-lg p-8">
        <WizardProgress />
        <div className="mt-8">
          <Outlet /> {/* Step content renders here */}
        </div>
        <WizardNavigation />
      </div>
    </div>
  );
}
```

#### Zustand Wizard Store Pattern

```typescript
// frontend/src/stores/wizardStore.ts
import { create } from "zustand";
import { persist } from "zustand/middleware";

interface WizardState {
  currentStep: number;
  totalSteps: number;
  completedSteps: number[];
  skippedSteps: number[];
  formData: Record<string, unknown>;
  
  // Actions
  setCurrentStep: (step: number) => void;
  markStepComplete: (step: number) => void;
  markStepSkipped: (step: number) => void;
  setFormData: (stepId: string, data: unknown) => void;
  reset: () => void;
}

export const useWizardStore = create<WizardState>()(
  persist(
    (set, get) => ({
      currentStep: 1,
      totalSteps: 5,
      completedSteps: [],
      skippedSteps: [],
      formData: {},
      
      setCurrentStep: (step) => set({ currentStep: step }),
      markStepComplete: (step) => set((state) => ({
        completedSteps: [...new Set([...state.completedSteps, step])],
      })),
      markStepSkipped: (step) => set((state) => ({
        skippedSteps: [...new Set([...state.skippedSteps, step])],
      })),
      setFormData: (stepId, data) => set((state) => ({
        formData: { ...state.formData, [stepId]: data },
      })),
      reset: () => set({
        currentStep: 1,
        completedSteps: [],
        skippedSteps: [],
        formData: {},
      }),
    }),
    { name: "qa-pms-wizard-state" }
  )
);
```

#### WizardProgress Component Pattern

```tsx
// frontend/src/components/wizard/WizardProgress.tsx
import { useWizardStore } from "@/stores/wizardStore";
import { CheckIcon } from "@radix-ui/react-icons";

const STEPS = [
  { id: 1, title: "Profile", path: "profile" },
  { id: 2, title: "Jira", path: "jira" },
  { id: 3, title: "Postman", path: "postman" },
  { id: 4, title: "Testmo", path: "testmo" },
  { id: 5, title: "Splunk", path: "splunk", optional: true },
];

export function WizardProgress() {
  const { currentStep, completedSteps } = useWizardStore();
  
  return (
    <div className="flex items-center justify-between">
      {STEPS.map((step, index) => (
        <div key={step.id} className="flex items-center">
          <div className={`
            w-10 h-10 rounded-full flex items-center justify-center font-medium
            ${completedSteps.includes(step.id) 
              ? "bg-success-500 text-white" 
              : currentStep === step.id 
                ? "bg-primary-500 text-white" 
                : "bg-neutral-200 text-neutral-600"}
          `}>
            {completedSteps.includes(step.id) ? <CheckIcon /> : step.id}
          </div>
          {index < STEPS.length - 1 && (
            <div className={`h-1 w-16 mx-2 ${
              completedSteps.includes(step.id) ? "bg-success-500" : "bg-neutral-200"
            }`} />
          )}
        </div>
      ))}
    </div>
  );
}
```

#### Radix UI Tooltip for Skip Warning

```tsx
// frontend/src/components/wizard/SkipButton.tsx
import * as Tooltip from "@radix-ui/react-tooltip";

export function SkipButton({ onSkip }: { onSkip: () => void }) {
  return (
    <Tooltip.Provider>
      <Tooltip.Root>
        <Tooltip.Trigger asChild>
          <button
            onClick={onSkip}
            className="text-neutral-500 hover:text-neutral-700 underline text-sm"
          >
            Skip this step
          </button>
        </Tooltip.Trigger>
        <Tooltip.Portal>
          <Tooltip.Content
            className="bg-neutral-900 text-white text-sm px-3 py-2 rounded shadow-lg"
            sideOffset={5}
          >
            You can configure this later in Settings
            <Tooltip.Arrow className="fill-neutral-900" />
          </Tooltip.Content>
        </Tooltip.Portal>
      </Tooltip.Root>
    </Tooltip.Provider>
  );
}
```

### Project Structure Notes

New files to create:
```
frontend/src/
├── pages/
│   └── Setup/
│       ├── SetupWizardLayout.tsx    # Main wizard container
│       ├── SetupGuard.tsx           # Route guard component
│       └── steps/
│           ├── UserProfileStep.tsx  # Step 1 placeholder
│           ├── JiraStep.tsx         # Step 2 placeholder
│           ├── PostmanStep.tsx      # Step 3 placeholder
│           ├── TestmoStep.tsx       # Step 4 placeholder
│           ├── SplunkStep.tsx       # Step 5 placeholder
│           └── SetupComplete.tsx    # Completion screen
├── components/
│   └── wizard/
│       ├── WizardProgress.tsx       # Progress indicator
│       ├── WizardNavigation.tsx     # Back/Next buttons
│       ├── WizardStepHeader.tsx     # Step title/description
│       ├── SkipButton.tsx           # Skip with tooltip
│       └── index.ts                 # Barrel export
└── stores/
    └── wizardStore.ts               # Zustand store (add to existing stores)
```

### UX Design Requirements (from ux-design-specification.md)

- **Progress indicator**: Show steps (1 of 5, 2 of 5, etc.)
- **Navigation**: Back/Next buttons, Skip for optional steps
- **Persistence**: Wizard state persists in localStorage (FR-CFG-05)
- **Styling**: Tailwind CSS v4 with OKLCH colors
- **Colors**: Primary blue (#0ea5e9), Success green (#28c76f), Neutral grays
- **Typography**: Inter font, text-2xl for titles, text-neutral-600 for descriptions
- **Animation**: 300ms ease-in-out for transitions

### Dependencies Required

Radix UI packages (already installed in Epic 1):
- `@radix-ui/react-tooltip`
- `@radix-ui/react-progress` (optional, for progress bar variant)

React Router (already installed):
- `react-router-dom` v6+

### Testing Notes

- Unit test WizardProgress renders correct step states
- Unit test WizardNavigation disables appropriately
- Unit test useWizardStore persistence with localStorage mock
- Integration test: Navigate through all wizard steps
- Verify localStorage persistence by simulating browser close/reopen

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story 2.1]
- [Source: _bmad-output/planning-artifacts/ux-design-specification.md#Setup Wizard]
- [Source: _bmad-output/planning-artifacts/architecture.md#Frontend Architecture]
- [Source: _bmad-output/planning-artifacts/project-context.md#Critical React Patterns]

## Dev Agent Record

### Agent Model Used

{{agent_model_name_version}}

### Debug Log References

### Completion Notes List

### File List
